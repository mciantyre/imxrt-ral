#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CAAM
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// Master Configuration Register
pub mod MCFGR {

    /// Normal Burst Size
    pub mod NORMAL_BURST {
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

            /// 0b0: Aligned 32 byte burst size target
            pub const ALIGNED_32B_TARGET: u32 = 0b0;

            /// 0b1: Aligned 64 byte burst size target
            pub const ALIGNED_64B_TARGET: u32 = 0b1;
        }
    }

    /// Enable Large Bursts
    pub mod LARGE_BURST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Pipeline Depth
    pub mod AXIPIPE {
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

    /// AXI Write Transaction Attributes
    pub mod AWCACHE {
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

    /// AXI Read Transaction Attributes
    pub mod ARCACHE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Pointer Size. This bit determines the size of address pointers. (see Address pointers).
    pub mod PS {
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

            /// 0b0: Pointers fit in one 32-bit word (pointers are 32-bit addresses).
            pub const SML_32Bit_PTRS: u32 = 0b0;

            /// 0b1: Pointers require two 32-bit words (pointers are 36-bit addresses).
            pub const LRG_64BIT_PTRS: u32 = 0b1;
        }
    }

    /// Double Word Transpose
    pub mod DWT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Write Handoff Disable
    pub mod WRHD {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Reset
    pub mod DMA_RST {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Watchdog Fast
    pub mod WDF {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECO Watchdog Enable
    pub mod WDE {
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

    /// Software Reset
    pub mod SWRST {
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

/// Page 0 SDID Register
pub mod PAGE0_SDID {

    /// Security Domain Identifier
    pub mod SDID {
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
}

/// Security Configuration Register
pub mod SCFGR {

    /// Private Blob
    pub mod PRIBLOB {
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

            /// 0b00: Private secure boot software blobs
            pub const PRIV_SEC_BOOT_SW: u32 = 0b00;

            /// 0b01: Private provisioning type 1 blobs
            pub const PRIV_PROV_TYPE_1: u32 = 0b01;

            /// 0b10: Private provisioning type 2 blobs
            pub const PRIV_PROV_TYPE_2: u32 = 0b10;

            /// 0b11: Normal operation blobs
            pub const NORMAL_OPERATION: u32 = 0b11;
        }
    }

    /// Random Number Generator State Handle 0.
    pub mod RNGSH0 {
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

            /// 0b0: When RNGSH0 is 0, RNG DRNG State Handle 0 can be instantiated in any mode. RNGSH0 is set to 0 only for testing.
            pub const SH0_ANY_MODE: u32 = 0b0;

            /// 0b1: When RNGSH0 is 1, RNG DRNG State Handle 0 cannot be instantiated in deterministic (test) mode. RNGSHO should be set to 1 before the RNG is instantiated. If it is currently instantiated in a deterministic mode, it will be un-instantiated. Once this bit has been written to a 1, it cannot be changed to a 0 until the next power on reset.
            pub const SH0_RANDOM_MODE: u32 = 0b1;
        }
    }

    /// Lock TRNG Program Mode
    pub mod LCK_TRNG {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Virtualization enable
    pub mod VIRT_EN {
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

            /// 0b0: Disable job ring virtualization
            pub const DISABLE_JR_VIRT: u32 = 0b0;

            /// 0b1: Enable job ring virtualization
            pub const ENABLE_JR_VIRT: u32 = 0b1;
        }
    }

    /// Manufacturing Protection Message Register Lock
    pub mod MPMRL {
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

    /// Manufacturing Protection Private Key Register Clear
    pub mod MPPKRC {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Manufacturing Protection Curve
    pub mod MPCURVE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Debug Control Register
pub mod DEBUGCTL {

    /// STOP is written to 1 to request that CAAM stop processing jobs
    pub mod STOP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STOP_ACK will assert when the job queue controller acknowledges that it is stopped.
    pub mod STOP_ACK {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Job Ring Start Register
pub mod JRSTARTR {

    /// Start Job Ring 0
    pub mod Start_JR0 {
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

            /// 0b0: Stop Mode. The JR0DID register and the SMVBA register for Job Ring 0 can be written but the IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR for Job Ring 0 are NOT accessible. If Job Ring 0 is allocated to TrustZone SecureWorld (JR0DID\[TZ\]=1), the JR0DID and SMVBA register can be written only via a bus transaction that has ns=0.
            pub const JR0_STOP_MODE: u32 = 0b0;

            /// 0b1: Start Mode. The JR0DID register and the SMVBA register for Job Ring 0 CANNOT be written but the IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR for Job Ring 0 ARE accessible. If Job Ring 0 is allocated to TrustZone SecureWorld (JR0DID\[TZ\]=1), then the SMVBA, IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR registers for Job Ring 0 can be written only via a bus transaction that has ns=0.
            pub const JR0_START_MODE: u32 = 0b1;
        }
    }

    /// Start Job Ring 1
    pub mod Start_JR1 {
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

            /// 0b0: Stop Mode. The JR1DID register and the SMVBA register for Job Ring 1 can be written but the IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR for Job Ring 1 are NOT accessible. If Job Ring 1 is allocated to TrustZone SecureWorld (JR1DID\[TZ\]=1), the JR1DID and SMVBA register can be written only via a bus transaction that has ns=0.
            pub const JR1_STOP_MODE: u32 = 0b0;

            /// 0b1: Start Mode. The JR1DID register and the SMVBA register for Job Ring 1 CANNOT be written but the IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR for Job Ring 1 ARE accessible. If Job Ring 1 is allocated to TrustZone SecureWorld (JR1DID\[TZ\]=1), then the SMVBA, IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR registers for Job Ring 1 can be written only via a bus transaction that has ns=0.
            pub const JR1_START_MODE: u32 = 0b1;
        }
    }

    /// Start Job Ring 2
    pub mod Start_JR2 {
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

            /// 0b0: Stop Mode. The JR2DID register and the SMVBA register for Job Ring 2 can be written but the IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR for Job Ring 2 are NOT accessible. If Job Ring 2 is allocated to TrustZone SecureWorld (JR2DID\[TZ\]=1), the JR2DID and SMVBA register can be written only via a bus transaction that has ns=0.
            pub const JR2_STOP_MODE: u32 = 0b0;

            /// 0b1: Start Mode. The JR2DID register and the SMVBA register for Job Ring 2 CANNOT be written but the IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR for Job Ring 2 ARE accessible. If Job Ring 2 is allocated to TrustZone SecureWorld (JR2DID\[TZ\]=1), then the SMVBA, IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR registers for Job Ring 2 can be written only via a bus transaction that has ns=0.
            pub const JR2_START_MODE: u32 = 0b1;
        }
    }

    /// Start Job Ring 3
    pub mod Start_JR3 {
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

            /// 0b0: Stop Mode. The JR3DID register and the SMVBA register for Job Ring 3 can be written but the IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR for Job Ring 3 are NOT accessible. If Job Ring 3 is allocated to TrustZone SecureWorld (JR3DID\[TZ\]=1), the JR3DID and SMVBA register can be written only via a bus transaction that has ns=0.
            pub const JR3_STOP_MODE: u32 = 0b0;

            /// 0b1: Start Mode. The JR3DID register and the SMVBA register for Job Ring 3 CANNOT be written but the IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR for Job Ring 3 ARE accessible. If Job Ring 3 is allocated to TrustZone SecureWorld (JR3DID\[TZ\]=1), then the SMVBA, IRBAR, IRSR, IRSAR, IRJAR, ORBAR, ORSR, ORJRR, ORSFR and JRSTAR registers for Job Ring 3 can be written only via a bus transaction that has ns=0.
            pub const JR3_START_MODE: u32 = 0b1;
        }
    }
}

/// RTIC OWN Register
pub mod RTIC_OWN {

    /// RTIC Owner's DID
    pub mod ROWN_DID {
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

    /// RTIC Owner's TZ
    pub mod ROWN_TZ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTIC OWN Lock
    pub mod LCK {
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

/// DECO Request Source Register
pub mod DECORSR {

    /// Job Ring number
    pub mod JR {
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

    /// Valid
    pub mod VALID {
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

/// DECO Request Register
pub mod DECORR {

    /// This bit is set by software to request direct access to DECO 0/CCB 0
    pub mod RQD0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The job queue controller asserts this bit when permission is granted for the software to directly access DECO 0/CCB 0
    pub mod DEN0 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO0 DID Register - most significant half
pub mod DECO0DID_MS {

    /// DECO Owner
    pub mod DPRIM_DID {
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

    /// DECO Owner's ns
    pub mod D_NS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock
    pub mod LCK {
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

/// DECO0 DID Register - least significant half
pub mod DECO0DID_LS {

    /// DECO SEQ DID
    pub mod DSEQ_DID {
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

    /// DECO SEQ NonSecure
    pub mod DSEQ_NS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECO Non-SEQ DID
    pub mod DNSEQ_DID {
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

    /// DECO NONSEQ NonSecure
    pub mod DNONSEQ_NS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO Availability Register
pub mod DAR {

    /// This bit is set by software to start polling for the availability of DECO 0
    pub mod NYA0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO Reset Register
pub mod DRR {

    /// Software writes a 1 to this bit to initiate a soft reset of DECO 0
    pub mod RST0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Peak Bandwidth Smoothing Limit Register
pub mod PBSL {

    /// Whenever the number of outstanding AXI read bursts exceeds the value programmed in this field, the Job Rings will be prevented from issuing additional AXI reads
    pub mod PBSL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA0_AIDL_MAP_MS
pub mod DMA0_AIDL_MAP_MS {

    /// This field shows the CAAM Block ID that uses AXI ID 4.
    pub mod AID4_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 5.
    pub mod AID5_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 6.
    pub mod AID6_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 7.
    pub mod AID7_BID {
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

/// DMA0_AIDL_MAP_LS
pub mod DMA0_AIDL_MAP_LS {

    /// This field shows the CAAM Block ID that uses AXI ID 0.
    pub mod AID0_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 1.
    pub mod AID1_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 2.
    pub mod AID2_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 3.
    pub mod AID3_BID {
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

/// DMA0_AIDM_MAP_MS
pub mod DMA0_AIDM_MAP_MS {

    /// This field shows the CAAM Block ID that uses AXI ID 12.
    pub mod AID12_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 13.
    pub mod AID13_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 14.
    pub mod AID14_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 15.
    pub mod AID15_BID {
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

/// DMA0_AIDM_MAP_LS
pub mod DMA0_AIDM_MAP_LS {

    /// This field shows the CAAM Block ID that uses AXI ID 8.
    pub mod AID8_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 9.
    pub mod AID9_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 10.
    pub mod AID10_BID {
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

    /// This field shows the CAAM Block ID that uses AXI ID 11.
    pub mod AID11_BID {
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

/// DMA0 AXI ID Enable Register
pub mod DMA0_AID_ENB {

    /// If AID0E=1 then AXI ID 0 is enabled for this DMA engine.
    pub mod AID0E {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID1E=1 then AXI ID 1 is enabled for this DMA engine.
    pub mod AID1E {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID2E=1 then AXI ID 2 is enabled for this DMA engine.
    pub mod AID2E {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID3E=1 then AXI ID 3 is enabled for this DMA engine.
    pub mod AID3E {
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

    /// If AID4E=1 then AXI ID 4 is enabled for this DMA engine.
    pub mod AID4E {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID5E=1 then AXI ID 5 is enabled for this DMA engine.
    pub mod AID5E {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID6E=1 then AXI ID 6 is enabled for this DMA engine.
    pub mod AID6E {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID7E=1 then AXI ID 7 is enabled for this DMA engine.
    pub mod AID7E {
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

    /// If AID8E=1 then AXI ID 8 is enabled for this DMA engine.
    pub mod AID8E {
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

    /// If AID9E=1 then AXI ID 9 is enabled for this DMA engine.
    pub mod AID9E {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID10E=1 then AXI ID 10 is enabled for this DMA engine.
    pub mod AID10E {
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

    /// If AID11E=1 then AXI ID 11 is enabled for this DMA engine.
    pub mod AID11E {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID12E=1 then AXI ID 12 is enabled for this DMA engine.
    pub mod AID12E {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID13E=1 then AXI ID 13 is enabled for this DMA engine.
    pub mod AID13E {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID14E=1 then AXI ID 14 is enabled for this DMA engine.
    pub mod AID14E {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If AID15E=1 then AXI ID 15 is enabled for this DMA engine.
    pub mod AID15E {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA0 AXI Read Timing Check Register
pub mod DMA0_ARD_TC {

    /// AXI Read Sample Count
    pub mod ARSC {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u64 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Read Late Count
    pub mod ARLC {
        /// Offset (24 bits)
        pub const offset: u64 = 24;
        /// Mask (20 bits: 0xfffff << 24)
        pub const mask: u64 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Read Limit
    pub mod ARL {
        /// Offset (48 bits)
        pub const offset: u64 = 48;
        /// Mask (12 bits: 0xfff << 48)
        pub const mask: u64 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Read Timer Last
    pub mod ARTL {
        /// Offset (60 bits)
        pub const offset: u64 = 60;
        /// Mask (1 bit: 1 << 60)
        pub const mask: u64 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Read Timer Test
    pub mod ARTT {
        /// Offset (61 bits)
        pub const offset: u64 = 61;
        /// Mask (1 bit: 1 << 61)
        pub const mask: u64 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Read Counter Test
    pub mod ARCT {
        /// Offset (62 bits)
        pub const offset: u64 = 62;
        /// Mask (1 bit: 1 << 62)
        pub const mask: u64 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Read Timing Check Enable
    pub mod ARTCE {
        /// Offset (63 bits)
        pub const offset: u64 = 63;
        /// Mask (1 bit: 1 << 63)
        pub const mask: u64 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA0 Read Timing Check Latency Register
pub mod DMA0_ARD_LAT {

    /// Sum of the AXI Read Latencies
    pub mod SARL {
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

/// DMA0 AXI Write Timing Check Register
pub mod DMA0_AWR_TC {

    /// AXI Write Sample Count
    pub mod AWSC {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u64 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Write Late Count
    pub mod AWLC {
        /// Offset (24 bits)
        pub const offset: u64 = 24;
        /// Mask (20 bits: 0xfffff << 24)
        pub const mask: u64 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Write Limit
    pub mod AWL {
        /// Offset (48 bits)
        pub const offset: u64 = 48;
        /// Mask (12 bits: 0xfff << 48)
        pub const mask: u64 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Write Timer Test
    pub mod AWTT {
        /// Offset (61 bits)
        pub const offset: u64 = 61;
        /// Mask (1 bit: 1 << 61)
        pub const mask: u64 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Write Counter Test
    pub mod AWCT {
        /// Offset (62 bits)
        pub const offset: u64 = 62;
        /// Mask (1 bit: 1 << 62)
        pub const mask: u64 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Write Timing Check Enable
    pub mod AWTCE {
        /// Offset (63 bits)
        pub const offset: u64 = 63;
        /// Mask (1 bit: 1 << 63)
        pub const mask: u64 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA0 Write Timing Check Latency Register
pub mod DMA0_AWR_LAT {

    /// Sum of the AXI Write Latencies
    pub mod SAWL {
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

/// Manufacturing Protection Private Key Register
pub mod MPPKR0 {

    /// MPPrivK. The 512-bit Manufacturing Protection Private Key.
    pub mod MPPrivK {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR1 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR2 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR3 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR4 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR5 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR6 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR7 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR8 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR9 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR10 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR11 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR12 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR13 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR14 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR15 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR16 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR17 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR18 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR19 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR20 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR21 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR22 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR23 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR24 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR25 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR26 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR27 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR28 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR29 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR30 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR31 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR32 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR33 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR34 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR35 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR36 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR37 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR38 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR39 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR40 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR41 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR42 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR43 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR44 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR45 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR46 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR47 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR48 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR49 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR50 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR51 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR52 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR53 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR54 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR55 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR56 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR57 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR58 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR59 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR60 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR61 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR62 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Private Key Register
pub mod MPPKR63 {
    pub use super::MPPKR0::MPPrivK;
}

/// Manufacturing Protection Message Register
pub mod MPMR0 {

    /// Holds 256 bits of message data that will be prepended to the input data to the MPSIGN operation
    pub mod MPMSG {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Manufacturing Protection Message Register
pub mod MPMR1 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR2 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR3 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR4 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR5 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR6 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR7 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR8 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR9 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR10 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR11 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR12 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR13 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR14 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR15 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR16 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR17 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR18 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR19 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR20 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR21 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR22 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR23 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR24 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR25 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR26 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR27 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR28 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR29 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR30 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Message Register
pub mod MPMR31 {
    pub use super::MPMR0::MPMSG;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR0 {

    /// TEST_VALUE
    pub mod TEST_VALUE {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Manufacturing Protection Test Register
pub mod MPTESTR1 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR2 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR3 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR4 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR5 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR6 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR7 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR8 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR9 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR10 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR11 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR12 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR13 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR14 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR15 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR16 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR17 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR18 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR19 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR20 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR21 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR22 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR23 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR24 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR25 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR26 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR27 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR28 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR29 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR30 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection Test Register
pub mod MPTESTR31 {
    pub use super::MPTESTR0::TEST_VALUE;
}

/// Manufacturing Protection ECC Register
pub mod MPECC {

    /// This is the syndrome produced by the ECC check on the Manufacturing Protection Key that is programmed in the Security Fuse Processor
    pub mod MP_SYNDROME {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (9 bits: 0x1ff << 16)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000000: The MP Key in the SFP passes the ECC check.
            pub const KEYOK: u32 = 0b000000000;
        }
    }

    /// This bit indicates if the Manufacturing Protection Key that is programmed in the Security Fuse Processor has an all-zero value
    pub mod MP_ZERO {
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

            /// 0b0: The MP Key in the SFP has a non-zero value.
            pub const NONZERO: u32 = 0b0;

            /// 0b1: The MP Key in the SFP is all zeros (unprogrammed).
            pub const ALLZERO: u32 = 0b1;
        }
    }
}

/// Job Descriptor Key Encryption Key Register
pub mod JDKEKR0 {

    /// The 256-bit Job Descriptor Key Encryption Key used to encrypt and decrypt Black Keys.
    pub mod JDKEK {
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

/// Job Descriptor Key Encryption Key Register
pub mod JDKEKR1 {
    pub use super::JDKEKR0::JDKEK;
}

/// Job Descriptor Key Encryption Key Register
pub mod JDKEKR2 {
    pub use super::JDKEKR0::JDKEK;
}

/// Job Descriptor Key Encryption Key Register
pub mod JDKEKR3 {
    pub use super::JDKEKR0::JDKEK;
}

/// Job Descriptor Key Encryption Key Register
pub mod JDKEKR4 {
    pub use super::JDKEKR0::JDKEK;
}

/// Job Descriptor Key Encryption Key Register
pub mod JDKEKR5 {
    pub use super::JDKEKR0::JDKEK;
}

/// Job Descriptor Key Encryption Key Register
pub mod JDKEKR6 {
    pub use super::JDKEKR0::JDKEK;
}

/// Job Descriptor Key Encryption Key Register
pub mod JDKEKR7 {
    pub use super::JDKEKR0::JDKEK;
}

/// Trusted Descriptor Key Encryption Key Register
pub mod TDKEKR0 {

    /// The 256-bit Trusted Descriptor Key Encryption Key used to encrypt and decrypt Black Keys.
    pub mod TDKEK {
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

/// Trusted Descriptor Key Encryption Key Register
pub mod TDKEKR1 {
    pub use super::TDKEKR0::TDKEK;
}

/// Trusted Descriptor Key Encryption Key Register
pub mod TDKEKR2 {
    pub use super::TDKEKR0::TDKEK;
}

/// Trusted Descriptor Key Encryption Key Register
pub mod TDKEKR3 {
    pub use super::TDKEKR0::TDKEK;
}

/// Trusted Descriptor Key Encryption Key Register
pub mod TDKEKR4 {
    pub use super::TDKEKR0::TDKEK;
}

/// Trusted Descriptor Key Encryption Key Register
pub mod TDKEKR5 {
    pub use super::TDKEKR0::TDKEK;
}

/// Trusted Descriptor Key Encryption Key Register
pub mod TDKEKR6 {
    pub use super::TDKEKR0::TDKEK;
}

/// Trusted Descriptor Key Encryption Key Register
pub mod TDKEKR7 {
    pub use super::TDKEKR0::TDKEK;
}

/// Trusted Descriptor Signing Key Register
pub mod TDSKR0 {

    /// The 256-bit Trusted Descriptor Signing Key used to sign and verify Trusted Descriptors.
    pub mod TDSK {
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

/// Trusted Descriptor Signing Key Register
pub mod TDSKR1 {
    pub use super::TDSKR0::TDSK;
}

/// Trusted Descriptor Signing Key Register
pub mod TDSKR2 {
    pub use super::TDSKR0::TDSK;
}

/// Trusted Descriptor Signing Key Register
pub mod TDSKR3 {
    pub use super::TDSKR0::TDSK;
}

/// Trusted Descriptor Signing Key Register
pub mod TDSKR4 {
    pub use super::TDSKR0::TDSK;
}

/// Trusted Descriptor Signing Key Register
pub mod TDSKR5 {
    pub use super::TDSKR0::TDSK;
}

/// Trusted Descriptor Signing Key Register
pub mod TDSKR6 {
    pub use super::TDSKR0::TDSK;
}

/// Trusted Descriptor Signing Key Register
pub mod TDSKR7 {
    pub use super::TDSKR0::TDSK;
}

/// Secure Key Nonce Register
pub mod SKNR {

    /// Secure Key Nonce - Least Significant Bits
    pub mod SK_NONCE_LS {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u64 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Secure Key Nonce - Most Significant Bits
    pub mod SK_NONCE_MS {
        /// Offset (32 bits)
        pub const offset: u64 = 32;
        /// Mask (15 bits: 0x7fff << 32)
        pub const mask: u64 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA Status Register
pub mod DMA_STA {

    /// DMA0 External Transactions in Flight
    pub mod DMA0_ETIF {
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

    /// DMA0 Internal Transactions in Flight
    pub mod DMA0_ITIF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA0 is idle. DMA0's command queue is empty.
    pub mod DMA0_IDLE {
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
}

/// DMA_X_AID_7_4_MAP
pub mod DMA_X_AID_7_4_MAP {
    pub use super::DMA0_AIDL_MAP_MS::AID4_BID;
    pub use super::DMA0_AIDL_MAP_MS::AID5_BID;
    pub use super::DMA0_AIDL_MAP_MS::AID6_BID;
    pub use super::DMA0_AIDL_MAP_MS::AID7_BID;
}

/// DMA_X_AID_3_0_MAP
pub mod DMA_X_AID_3_0_MAP {
    pub use super::DMA0_AIDL_MAP_LS::AID0_BID;
    pub use super::DMA0_AIDL_MAP_LS::AID1_BID;
    pub use super::DMA0_AIDL_MAP_LS::AID2_BID;
    pub use super::DMA0_AIDL_MAP_LS::AID3_BID;
}

/// DMA_X_AID_15_12_MAP
pub mod DMA_X_AID_15_12_MAP {
    pub use super::DMA0_AIDM_MAP_MS::AID12_BID;
    pub use super::DMA0_AIDM_MAP_MS::AID13_BID;
    pub use super::DMA0_AIDM_MAP_MS::AID14_BID;
    pub use super::DMA0_AIDM_MAP_MS::AID15_BID;
}

/// DMA_X_AID_11_8_MAP
pub mod DMA_X_AID_11_8_MAP {
    pub use super::DMA0_AIDM_MAP_LS::AID10_BID;
    pub use super::DMA0_AIDM_MAP_LS::AID11_BID;
    pub use super::DMA0_AIDM_MAP_LS::AID8_BID;
    pub use super::DMA0_AIDM_MAP_LS::AID9_BID;
}

/// DMA_X AXI ID Map Enable Register
pub mod DMA_X_AID_15_0_EN {
    pub use super::DMA0_AID_ENB::AID0E;
    pub use super::DMA0_AID_ENB::AID10E;
    pub use super::DMA0_AID_ENB::AID11E;
    pub use super::DMA0_AID_ENB::AID12E;
    pub use super::DMA0_AID_ENB::AID13E;
    pub use super::DMA0_AID_ENB::AID14E;
    pub use super::DMA0_AID_ENB::AID15E;
    pub use super::DMA0_AID_ENB::AID1E;
    pub use super::DMA0_AID_ENB::AID2E;
    pub use super::DMA0_AID_ENB::AID3E;
    pub use super::DMA0_AID_ENB::AID4E;
    pub use super::DMA0_AID_ENB::AID5E;
    pub use super::DMA0_AID_ENB::AID6E;
    pub use super::DMA0_AID_ENB::AID7E;
    pub use super::DMA0_AID_ENB::AID8E;
    pub use super::DMA0_AID_ENB::AID9E;
}

/// DMA_X AXI Read Timing Check Control Register
pub mod DMA_X_ARTC_CTL {

    /// AXI Read Timer
    pub mod ART {
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

    /// AXI Read Limit
    pub mod ARL {
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

    /// AXI Read Timer Last
    pub mod ARTL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Read Timer Test
    pub mod ARTT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Read Counter Test
    pub mod ARCT {
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

    /// AXI Read Timing Check Enable
    pub mod ARTCE {
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

/// DMA_X AXI Read Timing Check Late Count Register
pub mod DMA_X_ARTC_LC {

    /// AXI Read Late Count
    pub mod ARLC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA_X AXI Read Timing Check Sample Count Register
pub mod DMA_X_ARTC_SC {

    /// AXI Read Sample Count
    pub mod ARSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA_X Read Timing Check Latency Register
pub mod DMA_X_ARTC_LAT {
    pub use super::DMA0_ARD_LAT::SARL;
}

/// DMA_X AXI Write Timing Check Control Register
pub mod DMA_X_AWTC_CTL {

    /// AXI Write Timer
    pub mod AWT {
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

    /// AXI Write Limit
    pub mod AWL {
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

    /// AXI Write Timer Test
    pub mod AWTT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Write Counter Test
    pub mod AWCT {
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

    /// AXI Write Timing Check Enable
    pub mod AWTCE {
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

/// DMA_X AXI Write Timing Check Late Count Register
pub mod DMA_X_AWTC_LC {

    /// AXI Write Late Count
    pub mod AWLC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA_X AXI Write Timing Check Sample Count Register
pub mod DMA_X_AWTC_SC {

    /// AXI Write Sample Count
    pub mod AWSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA_X Write Timing Check Latency Register
pub mod DMA_X_AWTC_LAT {
    pub use super::DMA0_AWR_LAT::SAWL;
}

/// RNG TRNG Miscellaneous Control Register
pub mod RTMCTL {

    /// Sample Mode
    pub mod SAMP_MODE {
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

            /// 0b00: use Von Neumann data into both Entropy shifter and Statistical Checker
            pub const VON_DATA_4_SHIFTER_N_CHECKER: u32 = 0b00;

            /// 0b01: use raw data into both Entropy shifter and Statistical Checker
            pub const RAW_DATA_4_SHIFTER_N_CHECKER: u32 = 0b01;

            /// 0b10: use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker
            pub const VON_4_SHIFTER_N_RAW_4_CHECKER: u32 = 0b10;
        }
    }

    /// Oscillator Divide
    pub mod OSC_DIV {
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

            /// 0b00: use ring oscillator with no divide
            pub const OSC_NOT_DIVIDED: u32 = 0b00;

            /// 0b01: use ring oscillator divided-by-2
            pub const OSC_DIVIDE_BY_2: u32 = 0b01;

            /// 0b10: use ring oscillator divided-by-4
            pub const OSC_DIVIDE_BY_4: u32 = 0b10;

            /// 0b11: use ring oscillator divided-by-8
            pub const OSC_DIVIDE_BY_8: u32 = 0b11;
        }
    }

    /// Clock Output Enable
    pub mod CLK_OUT_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRNG Access Mode
    pub mod TRNG_ACC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset Defaults
    pub mod RST_DEF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Force System Clock
    pub mod FORCE_SYSCLK {
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

    /// Read only: Frequency Count Fail
    pub mod FCT_FAIL {
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

    /// Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from RTFRQCNT.
    pub mod FCT_VAL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read only: Entropy Valid
    pub mod ENT_VAL {
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

    /// Read only: Test point inside ring oscillator.
    pub mod TST_OUT {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read: Error status
    pub mod ERR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRNG_OK_TO_STOP
    pub mod TSTOP_OK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming Mode Select
    pub mod PRGM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RNG TRNG Statistical Check Miscellaneous Register
pub mod RTSCMISC {

    /// LONG RUN MAX LIMIT
    pub mod LRUN_MAX {
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

    /// RETRY COUNT
    pub mod RTY_CNT {
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

/// RNG TRNG Poker Range Register
pub mod RTPKRRNG {

    /// Poker Range
    pub mod PKR_RNG {
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

/// RTPKRMAX and RTPKRSQ
/// RTPKRMAX: RNG TRNG Poker Maximum Limit Register
/// RTPKRSQ: RNG TRNG Poker Square Calculation Result Register
pub mod RTPKR {

    /// Poker Maximum Limit
    pub mod PKR_MAX {
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

    /// Poker Square Calculation Result
    pub mod PKR_SQ {
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

/// RNG TRNG Seed Control Register
pub mod RTSDCTL {

    /// Sample Size
    pub mod SAMP_SIZE {
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

    /// Entropy Delay
    pub mod ENT_DLY {
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

/// RTSBLIM and RTTOTSAM
/// RTSBLIM: RNG TRNG Sparse Bit Limit Register
/// RTTOTSAM: RNG TRNG Total Samples Register
pub mod RT {

    /// Sparse Bit Limit
    pub mod SB_LIM {
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

    /// Total Samples
    pub mod TOT_SAM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RNG TRNG Frequency Count Minimum Limit Register
pub mod RTFRQMIN {

    /// Frequency Count Minimum Limit
    pub mod FRQ_MIN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTFRQCNT and RTFRQMAX
/// RTFRQCNT: RNG TRNG Frequency Count Register
/// RTFRQMAX: RNG TRNG Frequency Count Maximum Limit Register
pub mod RTFRQ {

    /// Frequency Count
    pub mod FRQ_CNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Frequency Counter Maximum Limit
    pub mod FRQ_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTSCMC and RTSCML
/// RTSCMC: RNG TRNG Statistical Check Monobit Count Register
/// RTSCML: RNG TRNG Statistical Check Monobit Limit Register
pub mod RTSCM {

    /// Monobit Count
    pub mod MONO_CNT {
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

    /// Monobit Maximum Limit
    pub mod MONO_MAX {
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

    /// Monobit Range
    pub mod MONO_RNG {
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

/// RTSCR1C and RTSCR1L
/// RTSCR1C: RNG TRNG Statistical Check Run Length 1 Count Register
/// RTSCR1L: RNG TRNG Statistical Check Run Length 1 Limit Register
pub mod RTSCR1 {

    /// Runs of Zero, Length 1 Count
    pub mod R1_0_COUNT {
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

    /// Runs of One, Length 1 Count
    pub mod R1_1_COUNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (15 bits: 0x7fff << 16)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 1 Maximum Limit
    pub mod RUN1_MAX {
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

    /// Run Length 1 Range
    pub mod RUN1_RNG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (15 bits: 0x7fff << 16)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTSCR2C and RTSCR2L
/// RTSCR2C: RNG TRNG Statistical Check Run Length 2 Count Register
/// RTSCR2L: RNG TRNG Statistical Check Run Length 2 Limit Register
pub mod RTSCR2 {

    /// Runs of Zero, Length 2 Count
    pub mod R2_0_COUNT {
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

    /// Runs of One, Length 2 Count
    pub mod R2_1_COUNT {
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

    /// Run Length 2 Maximum Limit
    pub mod RUN2_MAX {
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

    /// Run Length 2 Range
    pub mod RUN2_RNG {
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

/// RTSCR3C and RTSCR3L
/// RTSCR3C: RNG TRNG Statistical Check Run Length 3 Count Register
/// RTSCR3L: RNG TRNG Statistical Check Run Length 3 Limit Register
pub mod RTSCR3 {

    /// Runs of Zeroes, Length 3 Count
    pub mod R3_0_COUNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Runs of Ones, Length 3 Count
    pub mod R3_1_COUNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (13 bits: 0x1fff << 16)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 3 Maximum Limit
    pub mod RUN3_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 3 Range
    pub mod RUN3_RNG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (13 bits: 0x1fff << 16)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTSCR4C and RTSCR4L
/// RTSCR4C: RNG TRNG Statistical Check Run Length 4 Count Register
/// RTSCR4L: RNG TRNG Statistical Check Run Length 4 Limit Register
pub mod RTSCR4 {

    /// Runs of Zero, Length 4 Count
    pub mod R4_0_COUNT {
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

    /// Runs of One, Length 4 Count
    pub mod R4_1_COUNT {
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

    /// Run Length 4 Maximum Limit
    pub mod RUN4_MAX {
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

    /// Run Length 4 Range
    pub mod RUN4_RNG {
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

/// RTSCR5C and RTSCR5L
/// RTSCR5C: RNG TRNG Statistical Check Run Length 5 Count Register
/// RTSCR5L: RNG TRNG Statistical Check Run Length 5 Limit Register
pub mod RTSCR5 {

    /// Runs of Zero, Length 5 Count
    pub mod R5_0_COUNT {
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

    /// Runs of One, Length 5 Count
    pub mod R5_1_COUNT {
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

    /// Run Length 5 Maximum Limit
    pub mod RUN5_MAX {
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

    /// Run Length 5 Range
    pub mod RUN5_RNG {
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
}

/// RTSCR6PC and RTSCR6PL
/// RTSCR6PC: RNG TRNG Statistical Check Run Length 6+ Count Register
/// RTSCR6PL: RNG TRNG Statistical Check Run Length 6+ Limit Register
pub mod RTSCR6P {

    /// Runs of Zero, Length 6+ Count
    pub mod R6P_0_COUNT {
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

    /// Runs of One, Length 6+ Count
    pub mod R6P_1_COUNT {
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

    /// Run Length 6+ Maximum Limit
    pub mod RUN6P_MAX {
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

    /// Run Length 6+ Range
    pub mod RUN6P_RNG {
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
}

/// RNG TRNG Status Register
pub mod RTSTATUS {

    /// 1-Bit Run, Sampling 0s, Test Fail. If 1BR0TF=1, the 1-Bit Run, Sampling 0s Test has failed.
    pub mod F1BR0TF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1-Bit Run, Sampling 1s, Test Fail. If 1BR1TF=1, the 1-Bit Run, Sampling 1s Test has failed.
    pub mod F1BR1TF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 2-Bit Run, Sampling 0s, Test Fail. If 2BR0TF=1, the 2-Bit Run, Sampling 0s Test has failed.
    pub mod F2BR0TF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 2-Bit Run, Sampling 1s, Test Fail. If 2BR1TF=1, the 2-Bit Run, Sampling 1s Test has failed.
    pub mod F2BR1TF {
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

    /// 3-Bit Run, Sampling 0s, Test Fail. If 3BR0TF=1, the 3-Bit Run, Sampling 0s Test has failed.
    pub mod F3BR01TF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 3-Bit Run, Sampling 1s, Test Fail. If 3BR1TF=1, the 3-Bit Run, Sampling 1s Test has failed.
    pub mod F3BR1TF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 4-Bit Run, Sampling 0s, Test Fail. If 4BR0TF=1, the 4-Bit Run, Sampling 0s Test has failed.
    pub mod F4BR0TF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 4-Bit Run, Sampling 1s, Test Fail. If 4BR1TF=1, the 4-Bit Run, Sampling 1s Test has failed.
    pub mod F4BR1TF {
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

    /// 5-Bit Run, Sampling 0s, Test Fail. If 5BR0TF=1, the 5-Bit Run, Sampling 0s Test has failed.
    pub mod F5BR0TF {
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

    /// 5-Bit Run, Sampling 1s, Test Fail. If 5BR1TF=1, the 5-Bit Run, Sampling 1s Test has failed.
    pub mod F5BR1TF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 6 Plus Bit Run, Sampling 0s, Test Fail
    pub mod F6PBR0TF {
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

    /// 6 Plus Bit Run, Sampling 1s, Test Fail
    pub mod F6PBR1TF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sparse Bit Test Fail. If SBTF=1, the Sparse Bit Test has failed.
    pub mod FSBTF {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Long Run Test Fail. If LRTF=1, the Long Run Test has failed.
    pub mod FLRTF {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker Test Fail. If PTF=1, the Poker Test has failed.
    pub mod FPTF {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mono Bit Test Fail. If MBTF=1, the Mono Bit Test has failed.
    pub mod FMBTF {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RETRY COUNT
    pub mod RETRY_COUNT {
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

/// RNG TRNG Entropy Read Register
pub mod RTENT0 {

    /// Entropy Value
    pub mod ENT {
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

/// RNG TRNG Entropy Read Register
pub mod RTENT1 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT2 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT3 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT4 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT5 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT6 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT7 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT8 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT9 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT10 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT11 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT12 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT13 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT14 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Entropy Read Register
pub mod RTENT15 {
    pub use super::RTENT0::ENT;
}

/// RNG TRNG Statistical Check Poker Count 1 and 0 Register
pub mod RTPKRCNT10 {

    /// Poker 0h Count
    pub mod PKR_0_CNT {
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

    /// Poker 1h Count
    pub mod PKR_1_CNT {
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

/// RNG TRNG Statistical Check Poker Count 3 and 2 Register
pub mod RTPKRCNT32 {

    /// Poker 2h Count
    pub mod PKR_2_CNT {
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

    /// Poker 3h Count
    pub mod PKR_3_CNT {
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

/// RNG TRNG Statistical Check Poker Count 5 and 4 Register
pub mod RTPKRCNT54 {

    /// Poker 4h Count
    pub mod PKR_4_CNT {
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

    /// Poker 5h Count
    pub mod PKR_5_CNT {
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

/// RNG TRNG Statistical Check Poker Count 7 and 6 Register
pub mod RTPKRCNT76 {

    /// Poker 6h Count
    pub mod PKR_6_CNT {
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

    /// Poker 7h Count
    pub mod PKR_7_CNT {
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

/// RNG TRNG Statistical Check Poker Count 9 and 8 Register
pub mod RTPKRCNT98 {

    /// Poker 8h Count
    pub mod PKR_8_CNT {
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

    /// Poker 9h Count
    pub mod PKR_9_CNT {
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

/// RNG TRNG Statistical Check Poker Count B and A Register
pub mod RTPKRCNTBA {

    /// Poker Ah Count
    pub mod PKR_A_CNT {
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

    /// Poker Bh Count
    pub mod PKR_B_CNT {
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

/// RNG TRNG Statistical Check Poker Count D and C Register
pub mod RTPKRCNTDC {

    /// Poker Ch Count
    pub mod PKR_C_CNT {
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

    /// Poker Dh Count
    pub mod PKR_D_CNT {
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

/// RNG TRNG Statistical Check Poker Count F and E Register
pub mod RTPKRCNTFE {

    /// Poker Eh Count
    pub mod PKR_E_CNT {
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

    /// Poker Fh Count
    pub mod PKR_F_CNT {
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

/// RNG DRNG Status Register
pub mod RDSTA {

    /// Instantiated Flag State Handle 0. State Handle 0 has been instantiated.
    pub mod IF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Instantiated Flag State Handle 1. State Handle 1 has been instantiated.
    pub mod IF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Prediction Resistance Flag State Handle 0
    pub mod PR0 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Prediction Resistance Flag State Handle 1
    pub mod PR1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Test Flag State Handle 0. State handle 0 has been instantiated as a test (deterministic) instance.
    pub mod TF0 {
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

    /// Test Flag State Handle 1. State handle 1 has been instantiated as a test (deterministic) instance.
    pub mod TF1 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Error Code. These bits represent the current error in the RNG.
    pub mod ERRCODE {
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

    /// Catastrophic Error
    pub mod CE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Secure Key Valid Non-Test
    pub mod SKVN {
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

    /// Secure Key Valid Test
    pub mod SKVT {
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

/// RNG DRNG State Handle 0 Reseed Interval Register
pub mod RDINT0 {

    /// RESINT0. This read-only register holds the Reseed Interval for State Handle 0.
    pub mod RESINT0 {
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

/// RNG DRNG State Handle 1 Reseed Interval Register
pub mod RDINT1 {

    /// RESINT1. This read-only register holds the Reseed Interval for State Handle 1.
    pub mod RESINT1 {
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

/// RNG DRNG Hash Control Register
pub mod RDHCNTL {

    /// Hashing Done. This bit asserts when the hashing engine is done.
    pub mod HD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Hashing Begin. Writing this bit will causing the Hashing Engine to begin hashing.
    pub mod HB {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Hashing Initialize. Writing to this bit will initialize the Hashing Engine.
    pub mod HI {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Hashing Test Mode. Writing this bit will put RNG in Hashing Test Mode.
    pub mod HTM {
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

    /// Hashing Test Mode Clear. Writing this bit will take the RNG out of hashing test mode.
    pub mod HTC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RNG DRNG Hash Digest Register
pub mod RDHDIG {

    /// HASHMD
    pub mod HASHMD {
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

/// RNG DRNG Hash Buffer Register
pub mod RDHBUF {

    /// HASHBUF
    pub mod HASHBUF {
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

/// Partition 0 SDID register
pub mod P0SDID_PG0 {

    /// Security Domain Identifier
    pub mod SDID {
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

/// Secure Memory Access Permissions register
pub mod P0SMAPR_PG0 {

    /// Access Group 1 Read
    pub mod G1_READ {
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

            /// 0b0: Instruction fetches and reads are prohibited (except that Trusted Descriptor reads (if G1_TDO=1) and key-reads are always allowed, and exporting Secure Memory Blobs is allowed if G1_SMBLOB=1 or if done by a Trusted Descriptor and G1_TDO=1).
            pub const G1_READ_PROHIB: u32 = 0b0;

            /// 0b1: Instruction fetches and reads are allowed (but exporting a Secure Memory Blob is prohibited if G1_SMBLOB=0 and the descriptor is not a Trusted Descriptor or if G1_TDO=0).
            pub const G1_READ_ALLOWED: u32 = 0b1;
        }
    }

    /// Access Group 1 Write
    pub mod G1_WRITE {
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

            /// 0b0: Writes are prohibited (except that Trusted Descriptor writes are allowed, and importing Secure Memory Blobs is allowed if G1_SMBLOB=1 or if done by a Trusted Descriptor and G1_TDO=1).
            pub const G1_WRITE_PROHIB: u32 = 0b0;

            /// 0b1: Writes are allowed (but importing a Secure Memory Blob is prohibited if G1_SMBLOB=0 and the descriptor is not a Trusted Descriptor or if G1_TDO=0).
            pub const G1_WRITE_ALLOWED: u32 = 0b1;
        }
    }

    /// Access Group 1 Trusted Descriptor Override
    pub mod G1_TDO {
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

            /// 0b0: Trusted Descriptors have the same access privileges as Job Descriptors
            pub const G1_TD_PRIVILEGE_AS_JD: u32 = 0b0;

            /// 0b1: Trusted Descriptors are allowed to override the other access permissions, i.e. they can export blobs from or import blobs to the partition and read from and write to the partition regardless of the G1_SMBLOB, G1_WRITE and G1_READ settings.
            pub const G1_TD_OVERIDE_ALLOWED: u32 = 0b1;
        }
    }

    /// Access Group 1 Secure Memory Blobs
    pub mod G1_SMBLOB {
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

            /// 0b0: Exporting or importing Secure Memory Blobs is prohibited, unless done via a Trusted Descriptor and G1_TDO=1.
            pub const G1_SM_BLOB_EXPORT_IMPORT_PROHIB: u32 = 0b0;

            /// 0b1: Exporting or importing Secure Memory Blobs is allowed, regardless of the G1_READ and G1_WRITE settings.
            pub const G1_SM_B_EXPORT_IMPORT_ALLOWED: u32 = 0b1;
        }
    }

    /// Access Group 2 Read
    pub mod G2_READ {
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

            /// 0b0: Instruction fetches and reads are prohibited (except that Trusted Descriptor reads (if G2_TDO=1) and key-reads are always allowed, and exporting Secure Memory Blobs is allowed if G2_SMBLOB=1 or if done by a Trusted Descriptor and G2_TDO=1).
            pub const G2_READ_PROHIB: u32 = 0b0;

            /// 0b1: Instruction fetches and reads are allowed (but exporting a Secure Memory Blob is prohibited if G2_SMBLOB=0 and the descriptor is not a Trusted Descriptor or if G2_TDO=0).
            pub const G2_READ_ALLOWED: u32 = 0b1;
        }
    }

    /// Access Group 2 Write
    pub mod G2_WRITE {
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

            /// 0b0: Writes are prohibited (except that Trusted Descriptor writes are allowed, and importing Secure Memory Blobs is allowed if G2_SMBLOB=1 or if done by a Trusted Descriptor and G2_TDO=1).
            pub const G2_WRITE_PROHIB: u32 = 0b0;

            /// 0b1: Writes are allowed (but importing a Secure Memory Blob is prohibited if G2_SMBLOB=0 and the descriptor is not a Trusted Descriptor or if G2_TDO=0).
            pub const G2_WRITE_ALLOWED: u32 = 0b1;
        }
    }

    /// Access Group 2 Trusted Descriptor Override
    pub mod G2_TDO {
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

            /// 0b0: Trusted Descriptors have the same access privileges as Job Descriptors
            pub const G2_TD_PRIVILEGE_AS_JD: u32 = 0b0;

            /// 0b1: Trusted Descriptors are allowed to override the other access permissions, i.e. they can export blobs from or import blobs to the partition and read from and write to the partition regardless of the G2_SMBLOB, G2_WRITE and G2_READ settings.
            pub const G2_TD_OVERIDE_ALLOWED: u32 = 0b1;
        }
    }

    /// Access Group 2 Secure Memory Blobs
    pub mod G2_SMBLOB {
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

            /// 0b0: Exporting or importing Secure Memory Blobs is prohibited, unless done via a Trusted Descriptor and G2_TDO=1.
            pub const G2_SM_BLOB_EXPORT_IMPORT_PROHIB: u32 = 0b0;

            /// 0b1: Exporting or importing Secure Memory Blobs is allowed, regardless of the G2_READ and G2_WRITE settings.
            pub const G2_SM_BLOB_EXPORT_IMPORT_ALLOWED: u32 = 0b1;
        }
    }

    /// SMAG LOCK bit
    pub mod SMAG_LCK {
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

            /// 0b0: The SMAG2JR register and SMAG1JR register are unlocked. The partition owner can change any writable bits of these registers.
            pub const SMAG_UNLOCKED: u32 = 0b0;

            /// 0b1: The SMAG2JR register and SMAG1JR register are locked. The SMAG2JR and SMAG1JR registers cannot be changed until the partition is de-allocated or a POR occurs.
            pub const SMAG_LOCKED: u32 = 0b1;
        }
    }

    /// SMAP LOCK bit
    pub mod SMAP_LCK {
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

            /// 0b0: The SMAP register is unlocked. The partition owner can change any writable bits of the SMAP register.
            pub const SMAP_UNLOCKED: u32 = 0b0;

            /// 0b1: The SMAP register is locked. The SMAP_LCK, CSP and PSP bits and G1 and G2 permission bits of the SMAP register cannot be changed until the partition is de-allocated or a POR occurs. The PARTITION_KMOD value can still be changed. The SMAG_LCK bit can be changed to a 1, but cannot be changed to a 0.
            pub const SMAP_LOCKED: u32 = 0b1;
        }
    }

    /// Public Security Parameters
    pub mod PSP {
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

            /// 0b0: The partition and any of the pages allocated to the partition can be de-allocated.
            pub const NOT_DEALLOCATABLE: u32 = 0b0;

            /// 0b1: The partition cannot be de-allocated and the pages allocated to the partition cannot be de-allocated.
            pub const DEALLOCATABLE: u32 = 0b1;
        }
    }

    /// Critical Security Parameters
    pub mod CSP {
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

            /// 0b0: The pages allocated to the partition will not be zeroized when they are de-allocated or the partition is released or a security alarm occurs.
            pub const NOT_ZEROIZED: u32 = 0b0;

            /// 0b1: The pages allocated to the partition will be zeroized when they are individually de-allocated or the partition is released or a security alarm occurs.
            pub const ZEROIZED: u32 = 0b1;
        }
    }

    /// The value in this field is used to modify the Blob Key Encryption Key when exporting cryptographic Blobs from, or importing cryptographic Blobs to, this partition
    pub mod PARTITION_KMOD {
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

/// Secure Memory Access Group Registers
pub mod P0SMAG2_PG0 {

    /// Bit set to 1 indicates SecureWorld DID 00 is a member of Access Group x.
    pub mod Gx_ID00 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 01 is a member of Access Group x.
    pub mod Gx_ID01 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 02 is a member of Access Group x (1 or 2).
    pub mod Gx_ID02 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 03 is a member of Access Group x (1 or 2).
    pub mod Gx_ID03 {
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

    /// Bit set to 1 indicates SecureWorld DID 04 is a member of Access Group x (1 or 2).
    pub mod Gx_ID04 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 05 is a member of Access Group x (1 or 2).
    pub mod Gx_ID05 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 06 is a member of Access Group x (1 or 2).
    pub mod Gx_ID06 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 07 is a member of Access Group x (1 or 2).
    pub mod Gx_ID07 {
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

    /// Bit set to 1 indicates SecureWorld DID 08 is a member of Access Group x (1 or 2).
    pub mod Gx_ID08 {
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

    /// Bit set to 1 indicates SecureWorld DID 09 is a member of Access Group x (1 or 2).
    pub mod Gx_ID09 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 10 is a member of Access Group x (1 or 2).
    pub mod Gx_ID10 {
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

    /// Bit set to 1 indicates SecureWorld DID 11 is a member of Access Group x (1 or 2).
    pub mod Gx_ID11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 12 is a member of Access Group x (1 or 2).
    pub mod Gx_ID12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 13 is a member of Access Group x (1 or 2).
    pub mod Gx_ID13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 14 is a member of Access Group x (1 or 2).
    pub mod Gx_ID14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates SecureWorld DID 15 is a member of Access Group x (1 or 2).
    pub mod Gx_ID15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 00 is a member of Access Group x (1 or 2).
    pub mod Gx_ID16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 01 is a member of Access Group x (1 or 2).
    pub mod Gx_ID17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 02 is a member of Access Group x (1 or 2).
    pub mod Gx_ID18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 03 is a member of Access Group x (1 or 2).
    pub mod Gx_ID19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 04 is a member of Access Group x (1 or 2).
    pub mod Gx_ID20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 05 is a member of Access Group x (1 or 2).
    pub mod Gx_ID21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 06 is a member of Access Group x (1 or 2).
    pub mod Gx_ID22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 07 is a member of Access Group x (1 or 2).
    pub mod Gx_ID23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 08 is a member of Access Group x (1 or 2).
    pub mod Gx_ID24 {
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

    /// Bit set to 1 indicates NonSecureWorld DID 09 is a member of Access Group x (1 or 2).
    pub mod Gx_ID25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 10 is a member of Access Group x (1 or 2).
    pub mod Gx_ID26 {
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

    /// Bit set to 1 indicates NonSecureWorld DID 11 is a member of Access Group x (1 or 2).
    pub mod Gx_ID27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 12 is a member of Access Group x (1 or 2).
    pub mod Gx_ID28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 13 is a member of Access Group x (1 or 2).
    pub mod Gx_ID29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bit set to 1 indicates NonSecureWorld DID 14 is a member of Access Group x (1 or 2).
    pub mod Gx_ID30 {
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

    /// Bit set to 1 indicates NonSecureWorld DID 15 is a member of Access Group x (1 or 2).
    pub mod Gx_ID31 {
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

/// Secure Memory Access Group Registers
pub mod P0SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 1 SDID register
pub mod P1SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P1SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 2 SDID register
pub mod P2SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P2SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 3 SDID register
pub mod P3SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P3SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 4 SDID register
pub mod P4SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P4SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 5 SDID register
pub mod P5SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P5SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 6 SDID register
pub mod P6SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P6SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 7 SDID register
pub mod P7SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P7SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 8 SDID register
pub mod P8SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P8SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 9 SDID register
pub mod P9SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P9SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 10 SDID register
pub mod P10SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P10SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 11 SDID register
pub mod P11SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P11SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 12 SDID register
pub mod P12SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P12SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 13 SDID register
pub mod P13SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P13SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 14 SDID register
pub mod P14SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P14SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 15 SDID register
pub mod P15SDID_PG0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P15SMAPR_PG0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG2_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG1_PG0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Recoverable Error Interrupt Status
pub mod REIS {

    /// The CAAM watchdog timer expired.
    pub mod CWDE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A bus transaction initiated by CAAM RTIC resulted in a bus access error.
    pub mod RBAE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A job descriptor executed from Job Ring 0 caused a bus access error.
    pub mod JBAE0 {
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

    /// A job descriptor executed from Job Ring 1 caused a bus access error.
    pub mod JBAE1 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A job descriptor executed from Job Ring 2 caused a bus access error.
    pub mod JBAE2 {
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

    /// A job descriptor executed from Job Ring 3 caused a bus access error.
    pub mod JBAE3 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Recoverable Error Interrupt Enable
pub mod REIE {
    pub use super::REIS::CWDE;
    pub use super::REIS::JBAE0;
    pub use super::REIS::JBAE1;
    pub use super::REIS::JBAE2;
    pub use super::REIS::JBAE3;
    pub use super::REIS::RBAE;
}

/// Recoverable Error Interrupt Force
pub mod REIF {

    /// CAAM watchdog timer expired
    pub mod CWDE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTIC-initiated job execution caused bus access error.
    pub mod RBAE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// JR0-initiated job execution caused bus access error
    pub mod JBAE0 {
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

    /// JR1-initiated job execution caused bus access error
    pub mod JBAE1 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// JR2-initiated job execution caused bus access error
    pub mod JBAE2 {
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

    /// JR3-initiated job execution caused bus access error
    pub mod JBAE3 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Recoverable Error Interrupt Halt
pub mod REIH {

    /// Halt CAAM if CAAM watchdog timer expires.
    pub mod CWDE {
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

            /// 0b0: Don't halt CAAM if CAAM watchdog expired.
            pub const DONT_HALT: u32 = 0b0;

            /// 0b1: Halt CAAM if CAAM watchdog expired..
            pub const HALT: u32 = 0b1;
        }
    }

    /// Halt CAAM if RTIC-initiated job execution caused bus access error.
    pub mod RBAE {
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

            /// 0b0: Don't halt CAAM if RTIC-initiated job execution caused bus access error.
            pub const DONT_HALT: u32 = 0b0;

            /// 0b1: Halt CAAM if RTIC-initiated job execution caused bus access error.
            pub const HALT: u32 = 0b1;
        }
    }

    /// Halt CAAM if JR0-initiated job execution caused bus access error.
    pub mod JBAE0 {
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

            /// 0b0: Don't halt CAAM if JR0-initiated job execution caused bus access error.
            pub const DONT_HALT: u32 = 0b0;

            /// 0b1: Halt CAAM if JR0-initiated job execution caused bus access error.
            pub const HALT: u32 = 0b1;
        }
    }

    /// Halt CAAM if JR1-initiated job execution caused bus access error.
    pub mod JBAE1 {
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

            /// 0b0: Don't halt CAAM if JR1-initiated job execution caused bus access error.
            pub const DONT_HALT: u32 = 0b0;

            /// 0b1: Halt CAAM if JR1-initiated job execution caused bus access error.
            pub const HALT: u32 = 0b1;
        }
    }

    /// Halt CAAM if JR2-initiated job execution caused bus access error.
    pub mod JBAE2 {
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

            /// 0b0: Don't halt CAAM if JR2-initiated job execution caused bus access error.
            pub const DONT_HALT: u32 = 0b0;

            /// 0b1: Halt CAAM if JR2-initiated job execution caused bus access error.
            pub const HALT: u32 = 0b1;
        }
    }

    /// Halt CAAM if JR3-initiated job execution caused bus access error.
    pub mod JBAE3 {
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

            /// 0b0: Don't halt CAAM if JR3-initiated job execution caused bus access error.
            pub const DONT_HALT: u32 = 0b0;

            /// 0b1: Halt CAAM if JR3-initiated job execution caused bus access error.
            pub const HALT: u32 = 0b1;
        }
    }
}

/// Secure Memory Write Protect Job Ring Register
pub mod SMWPJRR0 {

    /// Secure Memory Registers Write Protect
    pub mod SMR_WP_JRa {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Secure Memory Write Protect Job Ring Register
pub mod SMWPJRR1 {
    pub use super::SMWPJRR0::SMR_WP_JRa;
}

/// Secure Memory Write Protect Job Ring Register
pub mod SMWPJRR2 {
    pub use super::SMWPJRR0::SMR_WP_JRa;
}

/// Secure Memory Write Protect Job Ring Register
pub mod SMWPJRR3 {
    pub use super::SMWPJRR0::SMR_WP_JRa;
}

/// Secure Memory Command Register
pub mod SMCR_PG0 {

    /// Command: 1h: Allocate Page - This command allocates the page specified in the PAGE field to the partition specified in the PRTN field
    pub mod CMD {
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

    /// Partition: When an Allocate Page or De-allocate Partition command is issued, the action is performed on the partition indicated in the PRTN field
    pub mod PRTN {
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

    /// This is the number of the page to be referenced in field CMD
    pub mod PAGE {
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

/// Secure Memory Command Status Register
pub mod SMCSR_PG0 {

    /// Following a Page Inquiry Command, if the PO field is 10 or 11, this field indicates the partition to which the page specified in the PAGE field is allocated
    pub mod PRTN {
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

    /// Page Owner: Following a Page Inquiry Command, this field indicates if the Page is owned by the entity that issued the inquiry, owned by another entity, or unowned
    pub mod PO {
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

            /// 0b00: Available; Unowned: The entity that issued the inquiry may allocate this page to a partition. No zeroization is needed since it has already been cleared, therefore no interrupt should be expected.
            pub const AVAILABLE: u32 = 0b00;

            /// 0b01: Page does not exist in this version or is not initialized yet.
            pub const NOT_PRESENT_OR_INITIALIZED: u32 = 0b01;

            /// 0b10: Another entity owns the page. This page is unavailable to the issuer of the inquiry.
            pub const UNAVAILABLE: u32 = 0b10;

            /// 0b11: Owned by the entity making the inquiry. The owner may de-allocate this page if its partition is not marked PSP. If the partition to which the page is allocated is designated as CSP, the page will be zeroized upon de-allocation.
            pub const OWNED: u32 = 0b11;
        }
    }

    /// Allocation Error
    pub mod AERR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command Error
    pub mod CERR {
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

            /// 0b00: No Error.
            pub const NO_ERROR: u32 = 0b00;

            /// 0b01: Command has not yet completed.
            pub const CMD_INCOMPLETE_ERROR: u32 = 0b01;

            /// 0b10: A security failure occurred.
            pub const SECURITY_FAILURE: u32 = 0b10;

            /// 0b11: Command Overflow. Another command was issued by the same Job Ring owner before the owner's previous command completed. The additional command was ignored.
            pub const CMD_OVERFLOW_ERROR: u32 = 0b11;
        }
    }

    /// Page
    pub mod PAGE {
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

/// Holding Tank 0 Job Descriptor Address
pub mod HT0_JD_ADDR {

    /// Job Descriptor Address.
    pub mod JD_ADDR {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (36 bits: 0xfffffffff << 0)
        pub const mask: u64 = 0xfffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Holding Tank 0 Shared Descriptor Address
pub mod HT0_SD_ADDR {

    /// Shared Descriptor Address.
    pub mod SD_ADDR {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (36 bits: 0xfffffffff << 0)
        pub const mask: u64 = 0xfffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Holding Tank 0 Job Queue Control, most-significant half
pub mod HT0_JQ_CTRL_MS {

    /// Job ID
    pub mod ID {
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

    /// Job Source
    pub mod SRC {
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

            /// 0b000: Job Ring 0
            pub const JR0: u32 = 0b000;

            /// 0b001: Job Ring 1
            pub const JR1: u32 = 0b001;

            /// 0b010: Job Ring 2
            pub const JR2: u32 = 0b010;

            /// 0b011: Job Ring 3
            pub const JR3: u32 = 0b011;

            /// 0b100: RTIC
            pub const RTIC: u32 = 0b100;
        }
    }

    /// Job Descriptor DID Select
    pub mod JDDS {
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

            /// 0b0: Non-SEQ DID
            pub const NON_SEQ_DID: u32 = 0b0;

            /// 0b1: SEQ DID
            pub const SEQ_DID: u32 = 0b1;
        }
    }

    /// Allow Make Trusted Descriptor
    pub mod AMTD {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Shared or Burst
    pub mod SOB {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Holding Tank Error. (This field is implemented only in versions of CAAM that support prefetching.)
    pub mod HT_ERROR {
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

            /// 0b00: No error
            pub const NO_ERROR: u32 = 0b00;

            /// 0b01: Job Descriptor or Shared Descriptor length error
            pub const JD_OR_SD_LENGTH_ERROR: u32 = 0b01;

            /// 0b10: AXI_error while reading a Job Ring Shared Descriptor or the remainder of a Job Ring Job Descriptor
            pub const JD_OR_SD_READ_ERROR: u32 = 0b10;
        }
    }

    /// Double Word Swap.
    pub mod DWORD_SWAP {
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

            /// 0b0: DWords are in the order most-significant word, least-significant word.
            pub const SRC_IN_MS_LS_ORDER: u32 = 0b0;

            /// 0b1: DWords are in the order least-significant word, most-significant word.
            pub const SRC_IN_LS_MS_ORDER: u32 = 0b1;
        }
    }

    /// Share From
    pub mod SHR_FROM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (5 bits: 0b11111 << 22)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Immediate Little Endian
    pub mod ILE {
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

            /// 0b0: No byte-swapping is performed for immediate data transferred to or from the Descriptor Buffer.
            pub const NO_BYTE_SWAP: u32 = 0b0;

            /// 0b1: Byte-swapping is performed for immediate data transferred to or from the Descriptor Buffer.
            pub const BYTE_SWAP: u32 = 0b1;
        }
    }

    /// Four Words. Job Queue Controller will pass at least 4 words of the descriptor to DECO.
    pub mod FOUR {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Whole Descriptor
    pub mod WHL {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Holding Tank 0 Job Queue Control, least-significant half
pub mod HT0_JQ_CTRL_LS {

    /// Primary DID
    pub mod PRIM_DID {
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

    /// Primary TZ
    pub mod PRIM_TZ {
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

            /// 0b0: TrustZone NonSecureWorld
            pub const NONSECUREWORLD: u32 = 0b0;

            /// 0b1: TrustZone SecureWorld
            pub const SECUREWORLD: u32 = 0b1;
        }
    }

    /// Primary ICID
    pub mod PRIM_ICID {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (11 bits: 0x7ff << 5)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output DID
    pub mod OUT_DID {
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

    /// Output ICID
    pub mod OUT_ICID {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (11 bits: 0x7ff << 21)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Holding Tank Status
pub mod HT0_STATUS {

    /// Pending for DECO 0
    pub mod PEND_0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// In Use
    pub mod IN_USE {
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

    /// Been Changed
    pub mod BC {
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

/// Job Queue Debug Select Register
pub mod JQ_DEBUG_SEL {

    /// Holding Tank Select
    pub mod HT_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Job ID
    pub mod JOB_ID {
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

/// Job Ring Job IDs in Use Register, least-significant half
pub mod JRJIDU_LS {

    /// Job ID 00
    pub mod JID00 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Job ID 01
    pub mod JID01 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Job ID 02
    pub mod JID02 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Job ID 03
    pub mod JID03 {
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
}

/// Job Ring Job-Done Job ID FIFO BC
pub mod JRJDJIFBC {

    /// Been changed
    pub mod BC {
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

/// Job Ring Job-Done Job ID FIFO
pub mod JRJDJIF {

    /// Job ID entry
    pub mod JOB_ID_ENTRY {
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
}

/// Job Ring Job-Done Source 1
pub mod JRJDS1 {

    /// Source
    pub mod SRC {
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

    /// Valid
    pub mod VALID {
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

/// Job Ring Job-Done Descriptor Address 0 Register
pub mod JRJDDA {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// CHA Revision Number Register, most-significant half
pub mod CRNR_MS {

    /// CRC Hardware Accelerator Revision Number
    pub mod CRCRN {
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

    /// SNOW-f9 Hardware Accelerator Revision Number
    pub mod SNW9RN {
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

    /// ZUC Encryption Hardware Accelerator Revision Number
    pub mod ZERN {
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

    /// ZUC Authentication Hardware Accelerator Revision Number
    pub mod ZARN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECO Revision Number
    pub mod DECORN {
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

    /// Job Ring Revision Number
    pub mod JRRN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CHA Revision Number Register, least-significant half
pub mod CRNR_LS {

    /// AES Accelerator Revision Number
    pub mod AESRN {
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

    /// DES Accelerator Revision Number.
    pub mod DESRN {
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

    /// Message Digest Hardware Accelerator module Revision Number.
    pub mod MDRN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Random Number Generator Revision Number.
    pub mod RNGRN {
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

    /// SNOW-f8 Hardware Accelerator Revision Number
    pub mod SNW8RN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Kasumi f8/f9 Hardware Accelerator Revision Number
    pub mod KASRN {
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

    /// Public Key Hardware Accelerator Revision Number For PKHA-XT, PKRN=1. For PKHA-SD, see below.
    pub mod PKRN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PKHA-SDv1
            pub const PKHA_SDV0: u32 = 0b0000;

            /// 0b0001: PKHA-SDv2
            pub const PKHA_SDV1: u32 = 0b0001;

            /// 0b0010: PKHA-SDv3
            pub const PKHA_SDV2: u32 = 0b0010;

            /// 0b0011: PKHA-SDv4
            pub const PKHA_SDV3: u32 = 0b0011;
        }
    }
}

/// Compile Time Parameters Register, most-significant half
pub mod CTPR_MS {

    /// Job Ring Virtualization programmable
    pub mod VIRT_EN_INCL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Job Ring Virtualization POR state
    pub mod VIRT_EN_POR_VALUE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CAAM register page size. 0: CAAM uses 4Kbyte register pages. 1: CAAM uses 64Kbyte register pages.
    pub mod REG_PG_SIZE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RNG Instantiations
    pub mod RNG_I {
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

    /// AIOP interface implemented
    pub mod AI_INCL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ICIDs with AMQs supported
    pub mod DPAA2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IP Bus Slave Clock
    pub mod IP_CLK {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Burst Configurability If MCFG_BURST is 0, the normal burst size is limited to 32 bytes and large bursts cannot be enabled
    pub mod MCFG_BURST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Pointer Size field implemented 0: The Master Configuration Register does not contain a Pointer Size field
    pub mod MCFG_PS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Eight Scatter-Gather Tables implemented 0: CAAM implements one Scatter-Gather Table register
    pub mod SG8 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Performance Monitor Event Bus implemented 0: CAAM does not implement a Performance Monitor Event Bus
    pub mod PM_EVT_BUS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECO Watchdog Counter implemented 0: CAAM does not implement a DECO Watchdog Counter
    pub mod DECO_WD {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Performance Counter registers implemented 0: CAAM does not implement Performance Counter registers
    pub mod PC {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Separate C1 and C2 registers 0: In this implementation of CAAM the Class 2 Key and Context registers are shared with the Class 1 Key and Context registers
    pub mod C1C2 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System/user partition-based CAAM IP Bus register access control 0: CAAM does not implement partition-based access control for its IP Bus registers, i
    pub mod ACC_CTL {
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

    /// Queue Manager interface (QI) implemented
    pub mod QI {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Master Priority implemented
    pub mod AXI_PRI {
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

    /// LIODN logic included
    pub mod AXI_LIODN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Pipeline Depth
    pub mod AXI_PIPE_DEPTH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Compile Time Parameters Register, least-significant half
pub mod CTPR_LS {

    /// PK generation and digital signature protcols
    pub mod KG_DS {
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

            /// 0b0: CAAM does not implement specialized support for Public Key Generation and Digital Signatures.
            pub const KG_DS_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for Public Key Generation and Digital Signatures.
            pub const KG_DS_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// Blob protocol
    pub mod BLOB {
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

            /// 0b0: CAAM does not implement specialized support for encapsulating and decapsulating cryptographic blobs.
            pub const BLOB_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for encapsulating and decapsulating cryptographic blobs.
            pub const BLOB_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// WiFi protocol
    pub mod WIFI {
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

            /// 0b0: CAAM does not implement specialized support for the WIFI protocol.
            pub const WIFI_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for the WIFI protocol.
            pub const WIFI_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// WiMax protocol
    pub mod WIMAX {
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

            /// 0b0: CAAM does not implement specialized support for the WIMAX protocol.
            pub const WIMAX_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for the WIMAX protocol.
            pub const WIMAX_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// SRTP protocol
    pub mod SRTP {
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

            /// 0b0: CAAM does not implement specialized support for the SRTP protocol.
            pub const SRTP_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for the SRTP protocol.
            pub const SRTP_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// IPSEC protocols
    pub mod IPSEC {
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

            /// 0b0: CAAM does not implement specialized support for the IPSEC protocol.
            pub const IPSEC_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for the IPSEC protocol.
            pub const IPSEC_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// IKE protocols
    pub mod IKE {
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

            /// 0b0: CAAM does not implement specialized support for the IKE protocol.
            pub const IKE_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for the IKE protocol.
            pub const IKE_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// SSL/TLS protocol
    pub mod SSL_TLS {
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

            /// 0b0: CAAM does not implement specialized support for the SSL and TLS protocols.
            pub const SSL_TLS_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for the SSL and TLS protocols.
            pub const SSL_TLS_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// TLS PRF protocol
    pub mod TLS_PRF {
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

            /// 0b0: CAAM does not implement specialized support for the TLS protocol pseudo-random function.
            pub const TLS_PRF_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for the TLS protocol pseudo-random function.
            pub const TLS_PRF_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// MACSEC protocol
    pub mod MACSEC {
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

            /// 0b0: CAAM does not implement specialized support for the MACSEC protocol.
            pub const MACSEC_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for the MACSEC protocol.
            pub const MACSEC_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// RSA protocol
    pub mod RSA {
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

            /// 0b0: CAAM does not implement specialized support for RSA encrypt and decrypt operations.
            pub const RSA_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for RSA encrypt and decrypt operations.
            pub const RSA_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// 3GPP/LTE protocol
    pub mod P3G_LTE {
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

            /// 0b0: CAAM does not implement specialized support for 3G and LTE protocols.
            pub const P3G_LTE_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for 3G and LTE protocols.
            pub const P3G_LTE_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// Double CRC protocol
    pub mod DBL_CRC {
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

            /// 0b0: CAAM does not implement specialized support for Double CRC.
            pub const DBL_CRC_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements specialized support for Double CRC.
            pub const DBL_CRC_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// Manufacturing Protection protocol
    pub mod MAN_PROT {
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

            /// 0b0: CAAM does not implement Manufacturing Protection functions.
            pub const MAN_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements Manufacturing Protection functions.
            pub const MAN_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }

    /// Derived Key Protocol
    pub mod DKP {
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

            /// 0b0: CAAM does not implement the Derived Key Protocol.
            pub const DERIVED_KEY_PROTOCOL_ABSENT: u32 = 0b0;

            /// 0b1: CAAM implements the Derived Key Protocol.
            pub const DERIVED_KEY_PROTOCOL_EXISTS: u32 = 0b1;
        }
    }
}

/// Secure Memory Status Register
pub mod SMSTA {

    /// Current State. This field represents the current state of the Secure Memory Controller.
    pub mod STATE {
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

            /// 0b0000: Reset State
            pub const RESET: u32 = 0b0000;

            /// 0b0001: Initialize State
            pub const INIT: u32 = 0b0001;

            /// 0b0010: Normal State
            pub const NORMAL: u32 = 0b0010;

            /// 0b0011: Fail State
            pub const FAIL: u32 = 0b0011;
        }
    }

    /// Access Error
    pub mod ACCERR {
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

            /// 0b0000: No error occurred
            pub const NO_ERROR: u32 = 0b0000;

            /// 0b0001: A bus transaction attempted to access a page in Secure Memory, but the page was not allocated to any partition.
            pub const PAGE_NOT_ALLOCATED_ACCESS_ERROR: u32 = 0b0001;

            /// 0b0010: A bus transaction attempted to access a partition, but the transaction's TrustZone World, DID was not granted access to the partition in the partition's SMAG2/1JR registers.
            pub const PARTITION_NOT_GRANTED_ACCESS_ERROR: u32 = 0b0010;

            /// 0b0011: A bus transaction attempted to read, but reads from this partition are not allowed.
            pub const READ_NOT_ALLOWED_ACCESS_ERROR: u32 = 0b0011;

            /// 0b0100: A bus transaction attempted to write, but writes to this partition are not allowed.
            pub const WRITE_NOT_ALLOWED_ACCESS_ERROR: u32 = 0b0100;

            /// 0b0110: A bus transaction attempted a non-key read, but the only reads permitted from this partition are key reads.
            pub const NON_KEY_READ_NOT_ALLOWED_ACCESS_ERROR: u32 = 0b0110;

            /// 0b1001: Secure Memory Blob import or export was attempted, but Secure Memory Blob access is not allowed for this partition.
            pub const BLOB_IMPORT_EXPORT_ACCESS_ERROR: u32 = 0b1001;

            /// 0b1010: A Descriptor attempted a Secure Memory Blob import or export, but not all of the pages referenced were from the same partition.
            pub const MULTI_PARTITION_BLOB_IMPORT_EXPORT_ERROR: u32 = 0b1010;

            /// 0b1011: A memory access was directed to Secure Memory, but the specified address is not implemented in Secure Memory. The address was either outside the address range occupied by Secure Memory, or was within an unimplemented portion of the 4kbyte address block occupied by a 1Kbyte or 2Kbyte Secure Memory page.
            pub const INVALID_SECURE_MEMORY_ADDRESS_ERROR: u32 = 0b1011;

            /// 0b1100: A bus transaction was attempted, but the burst would have crossed a page boundary.
            pub const PAGE_BOUNDARY_CROSSING_ERROR: u32 = 0b1100;

            /// 0b1101: An attempt was made to access a page while it was still being initialized.
            pub const UNINITIALIZED_PAGE_ACCESS_ERROR: u32 = 0b1101;
        }
    }

    /// The DID of the bus master whose access to Secure Memory was denied.
    pub mod DID {
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

    /// The TrustZone nonsecure bit of the bus master whose access to Secure Memory was denied
    pub mod NS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Secure Memory Registers Write Protected
    pub mod SMR_WP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Page
    pub mod PAGE {
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

    /// Partition
    pub mod PART {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Secure Memory Partition Owners Register
pub mod SMPO {

    /// Partition Owner for partition 0: When read by a Job Ring owner, this field indicates if partition 0 is owned by that Job Ring, another Job Ring, Unowned, or Unimplemented
    pub mod PO0 {
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

            /// 0b00: Available; Unowned. A Job Ring owner may claim partition 0 by writing to the appropriate SMAPJR register address alias. Note that the entire register will return all 0s if read by a entity that does not own the Job Ring associated with the SMPO address alias that was read.
            pub const AVAILABLE: u32 = 0b00;

            /// 0b01: Partition 0 does not exist in this version
            pub const NOT_PRESENT: u32 = 0b01;

            /// 0b10: Another entity owns partition 0. Partition 0 is unavailable to the reader. If the reader attempts to de-allocate partition 0 or write to the SMAPJR register or SMAGJR register for partition 0 or allocate a page to or de-allocate a page from partition 0 the command will be ignored. (Note that if a CSP partition is de-allocated, all entities (including the owner that de-allocated the partition) will see a 0b10 value for that partition until all its pages have been zeroized.)
            pub const UNAVAILABLE: u32 = 0b10;

            /// 0b11: The entity that read the SMPO register owns partition 0. Ownership is claimed when the access permissions register (SMAPJR) of an available partition is first written.
            pub const OWNED: u32 = 0b11;
        }
    }

    /// Partition Owner for partition 1. See PO0.
    pub mod PO1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 2. See PO0.
    pub mod PO2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 3. See PO0.
    pub mod PO3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 4. See PO0.
    pub mod PO4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 5. See PO0.
    pub mod PO5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 6. See PO0.
    pub mod PO6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 7. See PO0.
    pub mod PO7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 8. See PO0.
    pub mod PO8 {
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

    /// Partition Owner for partition 9. See PO0.
    pub mod PO9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 10. See PO0.
    pub mod PO10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 11. See PO0.
    pub mod PO11 {
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

    /// Partition Owner for partition 12. See PO0.
    pub mod PO12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 13. See PO0.
    pub mod PO13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 14. See PO0.
    pub mod PO14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Partition Owner for partition 15. See PO0.
    pub mod PO15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fault Address Register
pub mod FAR {

    /// Fault Address
    pub mod FAR {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (36 bits: 0xfffffffff << 0)
        pub const mask: u64 = 0xfffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fault Address DID Register
pub mod FADID {

    /// DMA transaction DID. This was the DID associated with the DMA transaction that failed.
    pub mod FDID {
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

    /// DMA transaction ns
    pub mod FNS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA transaction ICID. This was the ICID value associated with the DMA transaction that failed.
    pub mod FICID {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (11 bits: 0x7ff << 5)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fault Address Detail Register
pub mod FADR {

    /// AXI Transaction Transfer Size
    pub mod FSZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Transaction Type
    pub mod TYP {
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

            /// 0b0: Read.
            pub const READ: u32 = 0b0;

            /// 0b1: Write.
            pub const WRITE: u32 = 0b1;
        }
    }

    /// Block ID
    pub mod BLKID {
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

            /// 0b0100: job queue controller Burst Buffer
            pub const JQ: u32 = 0b0100;

            /// 0b0101: One of the Job Rings (see JSRC field)
            pub const JRN: u32 = 0b0101;

            /// 0b1000: DECO0
            pub const DECO0: u32 = 0b1000;
        }
    }

    /// Job Source. The source of the job whose AXI transfer ended with an error:
    pub mod JSRC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Job Ring 0
            pub const JR0: u32 = 0b000;

            /// 0b001: Job Ring 1
            pub const JR1: u32 = 0b001;

            /// 0b010: Job Ring 2
            pub const JR2: u32 = 0b010;

            /// 0b011: Job Ring 3
            pub const JR3: u32 = 0b011;

            /// 0b100: RTIC
            pub const RTIC: u32 = 0b100;
        }
    }

    /// Data Type. The type of data being processed when the AXI transfer error occurred.
    pub mod DTYP {
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

            /// 0b0: message data
            pub const MSG_DATA: u32 = 0b0;

            /// 0b1: control data
            pub const CTL_DATA: u32 = 0b1;
        }
    }

    /// AXI Transaction Transfer Size - extended
    pub mod FSZ_EXT {
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

    /// Key Modifier Read
    pub mod FKMOD {
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

            /// 0b0: CAAM DMA was not attempting to read the key modifier from Secure Memory at the time that the DMA error occurred.
            pub const NOT: u32 = 0b0;

            /// 0b1: CAAM DMA was attempting to read the key modifier from Secure Memory at the time that the DMA error occurred.
            pub const YES: u32 = 0b1;
        }
    }

    /// Key Access Read
    pub mod FKEY {
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

            /// 0b0: CAAM DMA was not attempting to perform a key read from Secure Memory at the time of the DMA error.
            pub const NOT: u32 = 0b0;

            /// 0b1: CAAM DMA was attempting to perform a key read from Secure Memory at the time of the DMA error.
            pub const YES: u32 = 0b1;
        }
    }

    /// Trusted Descriptor access to Secure Memory
    pub mod FTDSC {
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

            /// 0b0: CAAM DMA was not executing a Trusted Descriptor at the time of the DMA error.
            pub const NOT: u32 = 0b0;

            /// 0b1: CAAM DMA was executing a Trusted Descriptor at the time of the DMA error.
            pub const YES: u32 = 0b1;
        }
    }

    /// Access permission binding access to Secure Memory.
    pub mod FBNDG {
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

            /// 0b0: CAAM DMA was not reading access permissions from a Secure Memory partition at the time of the DMA error.
            pub const NOT: u32 = 0b0;

            /// 0b1: CAAM DMA was reading access permissions from a Secure Memory partition at the time of the DMA error.
            pub const YES: u32 = 0b1;
        }
    }

    /// Non-secure (AXI bus ns bit) access to Secure Memory.
    pub mod FNS {
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

            /// 0b0: CAAM DMA was asserting ns=0 at the time of the DMA error.
            pub const YES: u32 = 0b0;

            /// 0b1: CAAM DMA was asserting ns=1 at the time of the DMA error.
            pub const NOT: u32 = 0b1;
        }
    }

    /// Fault Error Code. This is the AXI Error Response Code.
    pub mod FERR {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: OKAY - Normal Access
            pub const OKAY: u32 = 0b00;

            /// 0b10: SLVERR - Slave Error
            pub const SLVERR: u32 = 0b10;

            /// 0b11: DECERR - Decode Error
            pub const DECERR: u32 = 0b11;
        }
    }
}

/// CAAM Status Register
pub mod CSTA {

    /// CAAM Busy
    pub mod BSY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CAAM Idle
    pub mod IDLE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRNG Idle
    pub mod TRNG_IDLE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mode of Operation
    pub mod MOO {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Non-Secure
            pub const NON_SECURE: u32 = 0b00;

            /// 0b01: Secure
            pub const SECURE: u32 = 0b01;

            /// 0b10: Trusted
            pub const TRUSTED: u32 = 0b10;

            /// 0b11: Fail
            pub const FAIL: u32 = 0b11;
        }
    }

    /// Platform Endianness
    pub mod PLEND {
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

            /// 0b0: Platform default is Little Endian
            pub const LITTLE: u32 = 0b0;

            /// 0b1: Platform default is Big Endian
            pub const BIG: u32 = 0b1;
        }
    }
}

/// Secure Memory Version ID Register, most-significant half
pub mod SMVID_MS {

    /// This is the highest numbered page of Secure Memory
    pub mod NPAG {
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

    /// This is the highest numbered Secure Memory partition, so there can be 1 to 16 partitions.
    pub mod NPRT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Maximum allowable value for NPAG
    pub mod MAX_NPAG {
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

/// Secure Memory Version ID Register, least-significant half
pub mod SMVID_LS {

    /// Secure Memory Minor Version ID.
    pub mod SMNV {
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

    /// Secure Memory Major Version ID
    pub mod SMJV {
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

    /// Page Size
    pub mod PSIZ {
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

/// RTIC Version ID Register
pub mod RVID {

    /// RTIC Minor Version
    pub mod RMNV {
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

    /// RTIC Major Version
    pub mod RMJV {
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

    /// SHA-256.
    pub mod SHA_256 {
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

            /// 0b0: RTIC cannot use the SHA-256 hashing algorithm.
            pub const SHA256_ABSENT: u32 = 0b0;

            /// 0b1: RTIC can use the SHA-256 hashing algorithm.
            pub const SHA256_EXISTS: u32 = 0b1;
        }
    }

    /// SHA-512.
    pub mod SHA_512 {
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

            /// 0b0: RTIC cannot use the SHA-512 hashing algorithm.
            pub const SHA512_ABSENT: u32 = 0b0;

            /// 0b1: RTIC can use the SHA-512 hashing algorithm.
            pub const SHA512_EXISTS: u32 = 0b1;
        }
    }

    /// Memory Block A Available
    pub mod MA {
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

    /// Memory Block B Available
    pub mod MB {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Block C Available
    pub mod MC {
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

    /// Memory Block D Available
    pub mod MD {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CHA Cluster Block Version ID Register
pub mod CCBVID {

    /// Accelerator Minor Revision Number
    pub mod AMNV {
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

    /// Accelerator Major Revision Number
    pub mod AMJV {
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

    /// CAAM Era. This version of CAAM is based on Era 9 RTL. A value of 0 implies CAAM Era 5 or earlier.
    pub mod CAAM_ERA {
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

/// CHA Version ID Register, most-significant half
pub mod CHAVID_MS {

    /// CRC Hardware Accelerator Version ID
    pub mod CRCVID {
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

    /// SNOW-f9 Hardware Accelerator Version ID
    pub mod SNW9VID {
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

    /// ZUC Encryption Hardware Accelerator Version ID
    pub mod ZEVID {
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

    /// ZUC Authentication Hardware Accelerator Version ID
    pub mod ZAVID {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECO Version ID
    pub mod DECOVID {
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

    /// Job Ring Version ID
    pub mod JRVID {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CHA Version ID Register, least-significant half
pub mod CHAVID_LS {

    /// AES Accelerator Version ID.
    pub mod AESVID {
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

            /// 0b0011: Low-power AESA, implementing ECB, CBC, CBC-CS2, CFB128, OFB, CTR, CCM, CMAC, XCBC-MAC, and GCM modes
            pub const AESA_LP: u32 = 0b0011;

            /// 0b0100: High-performance AESA, implementing ECB, CBC, CBC-CS2, CFB128, OFB, CTR, CCM, CMAC, XCBC-MAC, CBCXCBC, CTRXCBC, XTS, and GCM modes
            pub const AESA_HP: u32 = 0b0100;
        }
    }

    /// DES Accelerator Version ID.
    pub mod DESVID {
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

    /// Message Digest Hardware Accelerator Version ID.
    pub mod MDVID {
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

            /// 0b0000: Low-power MDHA, with SHA-1, SHA-256, SHA 224, MD5 and HMAC
            pub const MDHA_LP0: u32 = 0b0000;

            /// 0b0001: Low-power MDHA, with SHA-1, SHA-256, SHA 224, SHA-512, SHA-512/224, SHA-512/256, SHA-384, MD5 and HMAC
            pub const MDHA_LP1: u32 = 0b0001;

            /// 0b0010: Medium-performance MDHA, with SHA-1, SHA-256, SHA 224, SHA-512, SHA-512/224, SHA-512/256, SHA-384, MD5, HMAC & SMAC
            pub const MDHA_MP: u32 = 0b0010;

            /// 0b0011: High-performance MDHA, with SHA-1, SHA-256, SHA 224, SHA-512, SHA-512/224, SHA-512/256, SHA-384, MD5, HMAC & SMAC
            pub const MDHA_HP: u32 = 0b0011;
        }
    }

    /// Random Number Generator Version ID.
    pub mod RNGVID {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0010: RNGB
            pub const RNGB: u32 = 0b0010;

            /// 0b0100: RNG4
            pub const RNG4: u32 = 0b0100;
        }
    }

    /// SNOW-f8 Hardware Accelerator Version ID.
    pub mod SNW8VID {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Kasumi f8/f9 Hardware Accelerator Version ID.
    pub mod KASVID {
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

    /// Public Key Hardware Accelerator Version ID
    pub mod PKVID {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PKHA-XT (32-bit); minimum modulus five bytes
            pub const PKHA_XT: u32 = 0b0000;

            /// 0b0001: PKHA-SD (32-bit)
            pub const PKHA_SD32: u32 = 0b0001;

            /// 0b0010: PKHA-SD (64-bit)
            pub const PKHA_SD64: u32 = 0b0010;

            /// 0b0011: PKHA-SD (128-bit)
            pub const PKHA_SD128: u32 = 0b0011;
        }
    }
}

/// CHA Number Register, most-significant half
pub mod CHANUM_MS {

    /// The number of copies of the CRC module that are implemented in this version of CAAM
    pub mod CRCNUM {
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

    /// The number of copies of the SNOW-f9 module that are implemented in this version of CAAM
    pub mod SNW9NUM {
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

    /// The number of copies of ZUCE that are implemented in this version of CAAM
    pub mod ZENUM {
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

    /// The number of copies of ZUCA that are implemented in this version of CAAM
    pub mod ZANUM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The number of copies of the DECO that are implemented in this version of CAAM
    pub mod DECONUM {
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

    /// The number of copies of the Job Ring that are implemented in this version of CAAM
    pub mod JRNUM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CHA Number Register, least-significant half
pub mod CHANUM_LS {

    /// The number of copies of the AES module that are implemented in this version of CAAM.
    pub mod AESNUM {
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

    /// The number of copies of the DES module that are implemented in this version of CAAM.
    pub mod DESNUM {
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

    /// The number of copies of the ARC4 module that are implemented in this version of CAAM.
    pub mod ARC4NUM {
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

    /// The number of copies of the MDHA (Hashing module) that are implemented in this version of CAAM.
    pub mod MDNUM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The number of copies of the Random Number Generator that are implemented in this version of CAAM.
    pub mod RNGNUM {
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

    /// The number of copies of the SNOW-f8 module that are implemented in this version of CAAM
    pub mod SNW8NUM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The number of copies of the Kasumi module that are implemented in this version of CAAM
    pub mod KASNUM {
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

    /// The number of copies of the Public Key module that are implemented in this version of CAAM
    pub mod PKNUM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CAAM Version ID Register, most-significant half
pub mod CAAMVID_MS {

    /// Minor revision number for CAAM.
    pub mod MIN_REV {
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

    /// Major revision number for CAAM.
    pub mod MAJ_REV {
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

    /// ID for CAAM.
    pub mod IP_ID {
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

/// CAAM Version ID Register, least-significant half
pub mod CAAMVID_LS {

    /// Configuration options for CAAM.
    pub mod CONFIG_OPT {
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

    /// ECO revision for CAAM.
    pub mod ECO_REV {
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

    /// Integration options for CAAM.
    pub mod INTG_OPT {
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

    /// Compile options for CAAM.
    pub mod COMPILE_OPT {
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

/// Input Ring Base Address Register for Job Ring 0
pub mod IRBAR_JR0 {

    /// Input Ring Base Address.
    pub mod IRBA {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (36 bits: 0xfffffffff << 0)
        pub const mask: u64 = 0xfffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Input Ring Size Register for Job Ring 0
pub mod IRSR_JR0 {

    /// Input Ring Size. (measured in number of entries)
    pub mod IRS {
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

/// Input Ring Slots Available Register for Job Ring 0
pub mod IRSAR_JR0 {

    /// Input Ring Slots Available. (measured in number of available job slots)
    pub mod IRSA {
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

/// Input Ring Jobs Added Register for Job Ring0
pub mod IRJAR_JR0 {

    /// Input Ring Jobs Added. (measured in number of entries)
    pub mod IRJA {
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

/// Output Ring Base Address Register for Job Ring 0
pub mod ORBAR_JR0 {

    /// Output Ring Base Address.
    pub mod ORBA {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (36 bits: 0xfffffffff << 0)
        pub const mask: u64 = 0xfffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Output Ring Size Register for Job Ring 0
pub mod ORSR_JR0 {

    /// Output Ring Size. (measured in number of entries)
    pub mod ORS {
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

/// Output Ring Jobs Removed Register for Job Ring 0
pub mod ORJRR_JR0 {

    /// Output Ring Jobs Removed. (measured in number of entries)
    pub mod ORJR {
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

/// Output Ring Slots Full Register for Job Ring 0
pub mod ORSFR_JR0 {

    /// Output Ring Slots Full. (measured in number of entries)
    pub mod ORSF {
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

/// Job Ring Output Status Register for Job Ring 0
pub mod JRSTAR_JR0 {

    /// Source-specific error details
    pub mod SSED {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (28 bits: 0xfffffff << 0)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status source. These bits define which source is reporting the status. All other values - reserved
    pub mod SSRC {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No Status Source (No Error or Status Reported)
            pub const NO_STATUS: u32 = 0b0000;

            /// 0b0010: CCB Status Source (CCB Error Reported)
            pub const CCB_STATUS: u32 = 0b0010;

            /// 0b0011: Jump Halt User Status Source (User-Provided Status Reported)
            pub const JMP_USR_STATUS: u32 = 0b0011;

            /// 0b0100: DECO Status Source (DECO Error Reported)
            pub const DECO_STATUS: u32 = 0b0100;

            /// 0b0110: Job Ring Status Source (Job Ring Error Reported)
            pub const JR_STATUS: u32 = 0b0110;

            /// 0b0111: Jump Halt Condition Codes (Condition Code Status Reported)
            pub const JMP_COND_STATUS: u32 = 0b0111;
        }
    }
}

/// Job Ring Interrupt Status Register for Job Ring 0
pub mod JRINTR_JR0 {

    /// Job Ring Interrupt
    pub mod JRI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Job Ring Error
    pub mod JRE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Halt the Job Ring
    pub mod HALT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enter SNVS Fail State
    pub mod ENTER_FAIL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Exit SNVS Fail State
    pub mod EXIT_FAIL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Error type
    pub mod ERR_TYPE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00001: Error writing status to Output Ring
            pub const OR_WR_ERR: u32 = 0b00001;

            /// 0b00011: Bad input ring base address (not on a 4-byte boundary).
            pub const BAD_IR_ADDR_ERR: u32 = 0b00011;

            /// 0b00100: Bad output ring base address (not on a 4-byte boundary).
            pub const BAD_OR_ADDR_ERR: u32 = 0b00100;

            /// 0b00101: Invalid write to Input Ring Base Address Register or Input Ring Size Register. Can be written when there are no jobs in the input ring or when the Job Ring is halted. These are fatal and will likely result in not being able to get all jobs out into the output ring for processing by software. Resetting the job ring will almost certainly be necessary.
            pub const INV_IR_REG_WR_ERR: u32 = 0b00101;

            /// 0b00110: Invalid write to Output Ring Base Address Register or Output Ring Size Register. Can be written when there are no jobs in the output ring and no jobs from this queue are already processing in CAAM (in the holding tanks or DECOs), or when the Job Ring is halted.
            pub const INV_OR_REG_WR_ERR: u32 = 0b00110;

            /// 0b00111: Job Ring reset released before Job Ring is halted.
            pub const INV_JR_RESET_ERR: u32 = 0b00111;

            /// 0b01000: Removed too many jobs (ORJRR larger than ORSFR).
            pub const ORJRR_GT_ORSFR_ERR: u32 = 0b01000;

            /// 0b01001: Added too many jobs (IRJAR larger than IRSAR).
            pub const IRJAR_GT_IRSAR_ERR: u32 = 0b01001;

            /// 0b01010: Writing ORSF > ORS In these error cases the write is ignored, the interrupt is asserted (unless masked) and the error bit and error_type fields are set in the Job Ring Interrupt Status Register.
            pub const ORSF_GT_ORS_ERR: u32 = 0b01010;

            /// 0b01011: Writing IRSA > IRS
            pub const IRSA_GT_IRS_ERR: u32 = 0b01011;

            /// 0b01100: Writing ORWI > ORS in bytes
            pub const ORWI_GT_ORS_ERR: u32 = 0b01100;

            /// 0b01101: Writing IRRI > IRS in bytes
            pub const IRRI_GT_IRS_ERR: u32 = 0b01101;

            /// 0b01110: Writing IRSA when ring is active
            pub const INV_IRSA_WR_ERR: u32 = 0b01110;

            /// 0b01111: Writing IRRI when ring is active
            pub const INV_IRRI_WR_ERR: u32 = 0b01111;

            /// 0b10000: Writing ORSF when ring is active
            pub const INV_ORSF_WR_ERR: u32 = 0b10000;

            /// 0b10001: Writing ORWI when ring is active
            pub const INV_ORWI_WR_ERR: u32 = 0b10001;
        }
    }

    /// Output ring write index with error
    pub mod ERR_ORWI {
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

/// Job Ring Configuration Register for Job Ring 0, most-significant half
pub mod JRCFGR_JR0_MS {

    /// To assist with mixed Endianness platforms, this bit configures a byte swap of message data read by CAAM DMA
    pub mod MBSI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// To assist with mixed Endianness platforms, this bit configures a halfword swap of message data read by CAAM DMA
    pub mod MHWSI {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// To assist with mixed Endianness platforms, this bit configures a fullword swap of message data read by CAAM DMA
    pub mod MWSI {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// To assist with mixed Endianness platforms, this bit configures a byte swap of control data read by CAAM DMA
    pub mod CBSI {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// To assist with mixed Endianness platforms, this bit configures a halfword swap of control data read by CAAM DMA
    pub mod CHWSI {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// To assist with mixed Endianness platforms, this bit configures a fullword swap of control data read by CAAM DMA
    pub mod CWSI {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// To assist with mixed Endianness platforms, this bit configures a byte swap of message data written by CAAM DMA
    pub mod MBSO {
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

    /// To assist with mixed Endianness platforms, this bit configures a halfword swap of message data written by CAAM DMA
    pub mod MHWSO {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// To assist with mixed Endianness platforms, this bit configures a fullword swap of message data written by CAAM DMA
    pub mod MWSO {
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

    /// To assist with mixed Endianness platforms, this bit configures a byte swap of control data written by CAAM DMA
    pub mod CBSO {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// To assist with mixed Endianness platforms, this bit configures a halfword swap of control data written by CAAM DMA
    pub mod CHWSO {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// To assist with mixed Endianness platforms, this bit configures a fullword swap of control data written by CAAM DMA
    pub mod CWSO {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Descriptor Message Data Byte Swap (this applies only to internal message data transfers to/from DECO Descriptor Buffers)
    pub mod DMBS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Platform Endian Override - The bit is XORed with the PLEND bit in the CaCSTA Register and the other "swap" bits in the Job Ring Configuration Register to determine the AXI Master's view of memory endianness when executing Job Descriptors from this Job Ring
    pub mod PEO {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Double Word Swap Override
    pub mod DWSO {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fail mode control
    pub mod FAIL_MODE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Include Sequence Out Length
    pub mod INCL_SEQ_OUT {
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

/// Job Ring Configuration Register for Job Ring 0, least-significant half
pub mod JRCFGR_JR0_LS {

    /// Interrupt Mask. Mask the interrupt that is associated with the particular processor.
    pub mod IMSK {
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

            /// 0b0: Interrupt enabled.
            pub const INTR_ENABLED: u32 = 0b0;

            /// 0b1: Interrupt masked.
            pub const INTR_MASKED: u32 = 0b1;
        }
    }

    /// Interrupt Coalescing Enable.
    pub mod ICEN {
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

            /// 0b0: Interrupt coalescing is disabled. If the IMSK bit is cleared, an interrupt is asserted whenever a job is written to the output ring. ICDCT is ignored. Note that if software removes one or more jobs and clears the interrupt but the output rings slots full is still greater than 0 (ORSF > 0), then the interrupt will clear but reassert on the next clock cycle.
            pub const INTR_COAL_DISABLED: u32 = 0b0;

            /// 0b1: Interrupt coalescing is enabled. If the IMSK bit is cleared, an interrupt is asserted whenever the threshold number of frames is reached (ICDCT) or when the threshold timer expires (ICTT). Note that if software removes one or more jobs and clears the interrupt but the interrupt coalescing threshold is still met (ORSF >= ICDCT), then the interrupt will clear but reassert on the next clock cycle.
            pub const INTR_COAL_ENABLED: u32 = 0b1;
        }
    }

    /// Interrupt Coalescing Descriptor Count Threshold
    pub mod ICDCT {
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

    /// Interrupt Coalescing Timer Threshold
    pub mod ICTT {
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

/// Input Ring Read Index Register for Job Ring 0
pub mod IRRIR_JR0 {

    /// Input Ring Read Index.
    pub mod IRRI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Output Ring Write Index Register for Job Ring 0
pub mod ORWIR_JR0 {

    /// Output Ring Write Index. The pointer to the next entry in the output ring.
    pub mod ORWI {
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

/// Job Ring Command Register for Job Ring 0
pub mod JRCR_JR0 {

    /// Reset - When RESET is 0, software writes a 1 to RESET to request a flush of the Job Ring
    pub mod RESET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Park - When PARK is 0, software writes a 1 to PARK to request that the Job Ring be "parked", i
    pub mod PARK {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Job Ring 0 Address-Array Valid Register
pub mod JR0AAV {

    /// Valid 0
    pub mod V0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Valid 1
    pub mod V1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Valid 2
    pub mod V2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Valid 3
    pub mod V3 {
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

    /// Been Changed
    pub mod BC {
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

/// Job Ring 0 Address-Array Address 0 Register
pub mod JR0AAA0 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 0 Address-Array Address 1 Register
pub mod JR0AAA1 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 0 Address-Array Address 2 Register
pub mod JR0AAA2 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 0 Address-Array Address 3 Register
pub mod JR0AAA3 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Partition 0 SDID register
pub mod P0SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P0SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P0SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P0SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 1 SDID register
pub mod P1SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P1SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 2 SDID register
pub mod P2SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P2SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 3 SDID register
pub mod P3SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P3SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 4 SDID register
pub mod P4SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P4SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 5 SDID register
pub mod P5SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P5SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 6 SDID register
pub mod P6SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P6SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 7 SDID register
pub mod P7SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P7SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 8 SDID register
pub mod P8SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P8SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 9 SDID register
pub mod P9SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P9SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 10 SDID register
pub mod P10SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P10SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 11 SDID register
pub mod P11SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P11SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 12 SDID register
pub mod P12SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P12SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 13 SDID register
pub mod P13SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P13SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 14 SDID register
pub mod P14SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P14SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 15 SDID register
pub mod P15SDID_JR0 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P15SMAPR_JR0 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG2_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG1_JR0 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Command Register
pub mod SMCR_JR0 {
    pub use super::SMCR_PG0::CMD;
    pub use super::SMCR_PG0::PAGE;
    pub use super::SMCR_PG0::PRTN;
}

/// Secure Memory Command Status Register
pub mod SMCSR_JR0 {
    pub use super::SMCSR_PG0::AERR;
    pub use super::SMCSR_PG0::CERR;
    pub use super::SMCSR_PG0::PAGE;
    pub use super::SMCSR_PG0::PO;
    pub use super::SMCSR_PG0::PRTN;
}

/// Recoverable Error Interrupt Record 0 for Job Ring 0
pub mod REIR0JR0 {

    /// This field indicates the type of the recoverable error
    pub mod TYPE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If MISS=1, a second recoverable error associated with JR occurred before REIR0JR was written following a previous JR recoverable error
    pub mod MISS {
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

/// Recoverable Error Interrupt Record 2 for Job Ring 0
pub mod REIR2JR0 {

    /// Address associated with the recoverable JR error.
    pub mod ADDR {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (36 bits: 0xfffffffff << 0)
        pub const mask: u64 = 0xfffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Recoverable Error Interrupt Record 4 for Job Ring 0
pub mod REIR4JR0 {

    /// This field holds the ICID associated with the recoverable error.
    pub mod ICID {
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

    /// This field holds the DID associated with the recoverable error.
    pub mod DID {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (4 bits: 0b1111 << 11)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field holds the AXI cache control transaction attribute used for the memory access.
    pub mod AXCACHE {
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

    /// This field holds the AXI protection transaction attribute used for the memory access.
    pub mod AXPROT {
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

    /// This field specifies whether the memory access was a read or write.
    pub mod RWB {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field holds the AXI error response associated with the recoverable error.
    pub mod ERR {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field holds the memory interface index associated with the recoverable error.
    pub mod MIX {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Recoverable Error Interrupt Record 5 for Job Ring 0
pub mod REIR5JR0 {

    /// This field holds the block identifier (see Internal Block ID) of the source of the AXI transaction associated with the recoverable error
    pub mod BID {
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

    /// This field indicates whether the bus transaction associated with the recoverable error was initiating an CAAM Secure Memory blob operation
    pub mod BNDG {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field indicates whether the bus transaction associated with the recoverable error was an attempt to assert trusted descriptor privilege when accessing an CAAM Secure Memory partition
    pub mod TDSC {
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

    /// This field indicates whether the bus transaction associated with the recoverable error was an attempt to read the key modifier associated with an CAAM Secure Memory partition
    pub mod KMOD {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field indicates whether the bus transaction associated with the recoverable error was an attempted key access to CAAM Secure Memory
    pub mod KEY {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field indicates whether the bus transaction associated with the recoverable error was an attempted access to CAAM Secure Memory
    pub mod SMA {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Input Ring Base Address Register for Job Ring 1
pub mod IRBAR_JR1 {
    pub use super::IRBAR_JR0::IRBA;
}

/// Input Ring Size Register for Job Ring 1
pub mod IRSR_JR1 {
    pub use super::IRSR_JR0::IRS;
}

/// Input Ring Slots Available Register for Job Ring 1
pub mod IRSAR_JR1 {
    pub use super::IRSAR_JR0::IRSA;
}

/// Input Ring Jobs Added Register for Job Ring1
pub mod IRJAR_JR1 {
    pub use super::IRJAR_JR0::IRJA;
}

/// Output Ring Base Address Register for Job Ring 1
pub mod ORBAR_JR1 {
    pub use super::ORBAR_JR0::ORBA;
}

/// Output Ring Size Register for Job Ring 1
pub mod ORSR_JR1 {
    pub use super::ORSR_JR0::ORS;
}

/// Output Ring Jobs Removed Register for Job Ring 1
pub mod ORJRR_JR1 {
    pub use super::ORJRR_JR0::ORJR;
}

/// Output Ring Slots Full Register for Job Ring 1
pub mod ORSFR_JR1 {
    pub use super::ORSFR_JR0::ORSF;
}

/// Job Ring Output Status Register for Job Ring 1
pub mod JRSTAR_JR1 {
    pub use super::JRSTAR_JR0::SSED;
    pub use super::JRSTAR_JR0::SSRC;
}

/// Job Ring Interrupt Status Register for Job Ring 1
pub mod JRINTR_JR1 {
    pub use super::JRINTR_JR0::ENTER_FAIL;
    pub use super::JRINTR_JR0::ERR_ORWI;
    pub use super::JRINTR_JR0::ERR_TYPE;
    pub use super::JRINTR_JR0::EXIT_FAIL;
    pub use super::JRINTR_JR0::HALT;
    pub use super::JRINTR_JR0::JRE;
    pub use super::JRINTR_JR0::JRI;
}

/// Job Ring Configuration Register for Job Ring 1, most-significant half
pub mod JRCFGR_JR1_MS {
    pub use super::JRCFGR_JR0_MS::CBSI;
    pub use super::JRCFGR_JR0_MS::CBSO;
    pub use super::JRCFGR_JR0_MS::CHWSI;
    pub use super::JRCFGR_JR0_MS::CHWSO;
    pub use super::JRCFGR_JR0_MS::CWSI;
    pub use super::JRCFGR_JR0_MS::CWSO;
    pub use super::JRCFGR_JR0_MS::DMBS;
    pub use super::JRCFGR_JR0_MS::DWSO;
    pub use super::JRCFGR_JR0_MS::FAIL_MODE;
    pub use super::JRCFGR_JR0_MS::INCL_SEQ_OUT;
    pub use super::JRCFGR_JR0_MS::MBSI;
    pub use super::JRCFGR_JR0_MS::MBSO;
    pub use super::JRCFGR_JR0_MS::MHWSI;
    pub use super::JRCFGR_JR0_MS::MHWSO;
    pub use super::JRCFGR_JR0_MS::MWSI;
    pub use super::JRCFGR_JR0_MS::MWSO;
    pub use super::JRCFGR_JR0_MS::PEO;
}

/// Job Ring Configuration Register for Job Ring 1, least-significant half
pub mod JRCFGR_JR1_LS {
    pub use super::JRCFGR_JR0_LS::ICDCT;
    pub use super::JRCFGR_JR0_LS::ICEN;
    pub use super::JRCFGR_JR0_LS::ICTT;
    pub use super::JRCFGR_JR0_LS::IMSK;
}

/// Input Ring Read Index Register for Job Ring 1
pub mod IRRIR_JR1 {
    pub use super::IRRIR_JR0::IRRI;
}

/// Output Ring Write Index Register for Job Ring 1
pub mod ORWIR_JR1 {
    pub use super::ORWIR_JR0::ORWI;
}

/// Job Ring Command Register for Job Ring 1
pub mod JRCR_JR1 {
    pub use super::JRCR_JR0::PARK;
    pub use super::JRCR_JR0::RESET;
}

/// Job Ring 1 Address-Array Valid Register
pub mod JR1AAV {
    pub use super::JR0AAV::BC;
    pub use super::JR0AAV::V0;
    pub use super::JR0AAV::V1;
    pub use super::JR0AAV::V2;
    pub use super::JR0AAV::V3;
}

/// Job Ring 1 Address-Array Address 0 Register
pub mod JR1AAA0 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 1 Address-Array Address 1 Register
pub mod JR1AAA1 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 1 Address-Array Address 2 Register
pub mod JR1AAA2 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 1 Address-Array Address 3 Register
pub mod JR1AAA3 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Partition 0 SDID register
pub mod P0SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P0SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P0SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P0SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 1 SDID register
pub mod P1SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P1SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 2 SDID register
pub mod P2SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P2SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 3 SDID register
pub mod P3SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P3SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 4 SDID register
pub mod P4SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P4SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 5 SDID register
pub mod P5SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P5SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 6 SDID register
pub mod P6SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P6SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 7 SDID register
pub mod P7SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P7SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 8 SDID register
pub mod P8SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P8SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 9 SDID register
pub mod P9SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P9SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 10 SDID register
pub mod P10SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P10SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 11 SDID register
pub mod P11SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P11SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 12 SDID register
pub mod P12SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P12SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 13 SDID register
pub mod P13SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P13SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 14 SDID register
pub mod P14SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P14SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 15 SDID register
pub mod P15SDID_JR1 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P15SMAPR_JR1 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG2_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG1_JR1 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Command Register
pub mod SMCR_JR1 {
    pub use super::SMCR_PG0::CMD;
    pub use super::SMCR_PG0::PAGE;
    pub use super::SMCR_PG0::PRTN;
}

/// Secure Memory Command Status Register
pub mod SMCSR_JR1 {
    pub use super::SMCSR_PG0::AERR;
    pub use super::SMCSR_PG0::CERR;
    pub use super::SMCSR_PG0::PAGE;
    pub use super::SMCSR_PG0::PO;
    pub use super::SMCSR_PG0::PRTN;
}

/// Recoverable Error Interrupt Record 0 for Job Ring 1
pub mod REIR0JR1 {
    pub use super::REIR0JR0::MISS;
    pub use super::REIR0JR0::TYPE;
}

/// Recoverable Error Interrupt Record 2 for Job Ring 1
pub mod REIR2JR1 {
    pub use super::REIR2JR0::ADDR;
}

/// Recoverable Error Interrupt Record 4 for Job Ring 1
pub mod REIR4JR1 {
    pub use super::REIR4JR0::AXCACHE;
    pub use super::REIR4JR0::AXPROT;
    pub use super::REIR4JR0::DID;
    pub use super::REIR4JR0::ERR;
    pub use super::REIR4JR0::ICID;
    pub use super::REIR4JR0::MIX;
    pub use super::REIR4JR0::RWB;
}

/// Recoverable Error Interrupt Record 5 for Job Ring 1
pub mod REIR5JR1 {
    pub use super::REIR5JR0::BID;
    pub use super::REIR5JR0::BNDG;
    pub use super::REIR5JR0::KEY;
    pub use super::REIR5JR0::KMOD;
    pub use super::REIR5JR0::SMA;
    pub use super::REIR5JR0::TDSC;
}

/// Input Ring Base Address Register for Job Ring 2
pub mod IRBAR_JR2 {
    pub use super::IRBAR_JR0::IRBA;
}

/// Input Ring Size Register for Job Ring 2
pub mod IRSR_JR2 {
    pub use super::IRSR_JR0::IRS;
}

/// Input Ring Slots Available Register for Job Ring 2
pub mod IRSAR_JR2 {
    pub use super::IRSAR_JR0::IRSA;
}

/// Input Ring Jobs Added Register for Job Ring2
pub mod IRJAR_JR2 {
    pub use super::IRJAR_JR0::IRJA;
}

/// Output Ring Base Address Register for Job Ring 2
pub mod ORBAR_JR2 {
    pub use super::ORBAR_JR0::ORBA;
}

/// Output Ring Size Register for Job Ring 2
pub mod ORSR_JR2 {
    pub use super::ORSR_JR0::ORS;
}

/// Output Ring Jobs Removed Register for Job Ring 2
pub mod ORJRR_JR2 {
    pub use super::ORJRR_JR0::ORJR;
}

/// Output Ring Slots Full Register for Job Ring 2
pub mod ORSFR_JR2 {
    pub use super::ORSFR_JR0::ORSF;
}

/// Job Ring Output Status Register for Job Ring 2
pub mod JRSTAR_JR2 {
    pub use super::JRSTAR_JR0::SSED;
    pub use super::JRSTAR_JR0::SSRC;
}

/// Job Ring Interrupt Status Register for Job Ring 2
pub mod JRINTR_JR2 {
    pub use super::JRINTR_JR0::ENTER_FAIL;
    pub use super::JRINTR_JR0::ERR_ORWI;
    pub use super::JRINTR_JR0::ERR_TYPE;
    pub use super::JRINTR_JR0::EXIT_FAIL;
    pub use super::JRINTR_JR0::HALT;
    pub use super::JRINTR_JR0::JRE;
    pub use super::JRINTR_JR0::JRI;
}

/// Job Ring Configuration Register for Job Ring 2, most-significant half
pub mod JRCFGR_JR2_MS {
    pub use super::JRCFGR_JR0_MS::CBSI;
    pub use super::JRCFGR_JR0_MS::CBSO;
    pub use super::JRCFGR_JR0_MS::CHWSI;
    pub use super::JRCFGR_JR0_MS::CHWSO;
    pub use super::JRCFGR_JR0_MS::CWSI;
    pub use super::JRCFGR_JR0_MS::CWSO;
    pub use super::JRCFGR_JR0_MS::DMBS;
    pub use super::JRCFGR_JR0_MS::DWSO;
    pub use super::JRCFGR_JR0_MS::FAIL_MODE;
    pub use super::JRCFGR_JR0_MS::INCL_SEQ_OUT;
    pub use super::JRCFGR_JR0_MS::MBSI;
    pub use super::JRCFGR_JR0_MS::MBSO;
    pub use super::JRCFGR_JR0_MS::MHWSI;
    pub use super::JRCFGR_JR0_MS::MHWSO;
    pub use super::JRCFGR_JR0_MS::MWSI;
    pub use super::JRCFGR_JR0_MS::MWSO;
    pub use super::JRCFGR_JR0_MS::PEO;
}

/// Job Ring Configuration Register for Job Ring 2, least-significant half
pub mod JRCFGR_JR2_LS {
    pub use super::JRCFGR_JR0_LS::ICDCT;
    pub use super::JRCFGR_JR0_LS::ICEN;
    pub use super::JRCFGR_JR0_LS::ICTT;
    pub use super::JRCFGR_JR0_LS::IMSK;
}

/// Input Ring Read Index Register for Job Ring 2
pub mod IRRIR_JR2 {
    pub use super::IRRIR_JR0::IRRI;
}

/// Output Ring Write Index Register for Job Ring 2
pub mod ORWIR_JR2 {
    pub use super::ORWIR_JR0::ORWI;
}

/// Job Ring Command Register for Job Ring 2
pub mod JRCR_JR2 {
    pub use super::JRCR_JR0::PARK;
    pub use super::JRCR_JR0::RESET;
}

/// Job Ring 2 Address-Array Valid Register
pub mod JR2AAV {
    pub use super::JR0AAV::BC;
    pub use super::JR0AAV::V0;
    pub use super::JR0AAV::V1;
    pub use super::JR0AAV::V2;
    pub use super::JR0AAV::V3;
}

/// Job Ring 2 Address-Array Address 0 Register
pub mod JR2AAA0 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 2 Address-Array Address 1 Register
pub mod JR2AAA1 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 2 Address-Array Address 2 Register
pub mod JR2AAA2 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 2 Address-Array Address 3 Register
pub mod JR2AAA3 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Partition 0 SDID register
pub mod P0SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P0SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P0SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P0SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 1 SDID register
pub mod P1SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P1SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 2 SDID register
pub mod P2SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P2SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 3 SDID register
pub mod P3SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P3SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 4 SDID register
pub mod P4SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P4SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 5 SDID register
pub mod P5SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P5SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 6 SDID register
pub mod P6SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P6SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 7 SDID register
pub mod P7SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P7SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 8 SDID register
pub mod P8SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P8SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 9 SDID register
pub mod P9SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P9SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 10 SDID register
pub mod P10SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P10SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 11 SDID register
pub mod P11SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P11SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 12 SDID register
pub mod P12SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P12SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 13 SDID register
pub mod P13SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P13SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 14 SDID register
pub mod P14SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P14SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 15 SDID register
pub mod P15SDID_JR2 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P15SMAPR_JR2 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG2_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG1_JR2 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Command Register
pub mod SMCR_JR2 {
    pub use super::SMCR_PG0::CMD;
    pub use super::SMCR_PG0::PAGE;
    pub use super::SMCR_PG0::PRTN;
}

/// Secure Memory Command Status Register
pub mod SMCSR_JR2 {
    pub use super::SMCSR_PG0::AERR;
    pub use super::SMCSR_PG0::CERR;
    pub use super::SMCSR_PG0::PAGE;
    pub use super::SMCSR_PG0::PO;
    pub use super::SMCSR_PG0::PRTN;
}

/// Recoverable Error Interrupt Record 0 for Job Ring 2
pub mod REIR0JR2 {
    pub use super::REIR0JR0::MISS;
    pub use super::REIR0JR0::TYPE;
}

/// Recoverable Error Interrupt Record 2 for Job Ring 2
pub mod REIR2JR2 {
    pub use super::REIR2JR0::ADDR;
}

/// Recoverable Error Interrupt Record 4 for Job Ring 2
pub mod REIR4JR2 {
    pub use super::REIR4JR0::AXCACHE;
    pub use super::REIR4JR0::AXPROT;
    pub use super::REIR4JR0::DID;
    pub use super::REIR4JR0::ERR;
    pub use super::REIR4JR0::ICID;
    pub use super::REIR4JR0::MIX;
    pub use super::REIR4JR0::RWB;
}

/// Recoverable Error Interrupt Record 5 for Job Ring 2
pub mod REIR5JR2 {
    pub use super::REIR5JR0::BID;
    pub use super::REIR5JR0::BNDG;
    pub use super::REIR5JR0::KEY;
    pub use super::REIR5JR0::KMOD;
    pub use super::REIR5JR0::SMA;
    pub use super::REIR5JR0::TDSC;
}

/// Input Ring Base Address Register for Job Ring 3
pub mod IRBAR_JR3 {
    pub use super::IRBAR_JR0::IRBA;
}

/// Input Ring Size Register for Job Ring 3
pub mod IRSR_JR3 {
    pub use super::IRSR_JR0::IRS;
}

/// Input Ring Slots Available Register for Job Ring 3
pub mod IRSAR_JR3 {
    pub use super::IRSAR_JR0::IRSA;
}

/// Input Ring Jobs Added Register for Job Ring3
pub mod IRJAR_JR3 {
    pub use super::IRJAR_JR0::IRJA;
}

/// Output Ring Base Address Register for Job Ring 3
pub mod ORBAR_JR3 {
    pub use super::ORBAR_JR0::ORBA;
}

/// Output Ring Size Register for Job Ring 3
pub mod ORSR_JR3 {
    pub use super::ORSR_JR0::ORS;
}

/// Output Ring Jobs Removed Register for Job Ring 3
pub mod ORJRR_JR3 {
    pub use super::ORJRR_JR0::ORJR;
}

/// Output Ring Slots Full Register for Job Ring 3
pub mod ORSFR_JR3 {
    pub use super::ORSFR_JR0::ORSF;
}

/// Job Ring Output Status Register for Job Ring 3
pub mod JRSTAR_JR3 {
    pub use super::JRSTAR_JR0::SSED;
    pub use super::JRSTAR_JR0::SSRC;
}

/// Job Ring Interrupt Status Register for Job Ring 3
pub mod JRINTR_JR3 {
    pub use super::JRINTR_JR0::ENTER_FAIL;
    pub use super::JRINTR_JR0::ERR_ORWI;
    pub use super::JRINTR_JR0::ERR_TYPE;
    pub use super::JRINTR_JR0::EXIT_FAIL;
    pub use super::JRINTR_JR0::HALT;
    pub use super::JRINTR_JR0::JRE;
    pub use super::JRINTR_JR0::JRI;
}

/// Job Ring Configuration Register for Job Ring 3, most-significant half
pub mod JRCFGR_JR3_MS {
    pub use super::JRCFGR_JR0_MS::CBSI;
    pub use super::JRCFGR_JR0_MS::CBSO;
    pub use super::JRCFGR_JR0_MS::CHWSI;
    pub use super::JRCFGR_JR0_MS::CHWSO;
    pub use super::JRCFGR_JR0_MS::CWSI;
    pub use super::JRCFGR_JR0_MS::CWSO;
    pub use super::JRCFGR_JR0_MS::DMBS;
    pub use super::JRCFGR_JR0_MS::DWSO;
    pub use super::JRCFGR_JR0_MS::FAIL_MODE;
    pub use super::JRCFGR_JR0_MS::INCL_SEQ_OUT;
    pub use super::JRCFGR_JR0_MS::MBSI;
    pub use super::JRCFGR_JR0_MS::MBSO;
    pub use super::JRCFGR_JR0_MS::MHWSI;
    pub use super::JRCFGR_JR0_MS::MHWSO;
    pub use super::JRCFGR_JR0_MS::MWSI;
    pub use super::JRCFGR_JR0_MS::MWSO;
    pub use super::JRCFGR_JR0_MS::PEO;
}

/// Job Ring Configuration Register for Job Ring 3, least-significant half
pub mod JRCFGR_JR3_LS {
    pub use super::JRCFGR_JR0_LS::ICDCT;
    pub use super::JRCFGR_JR0_LS::ICEN;
    pub use super::JRCFGR_JR0_LS::ICTT;
    pub use super::JRCFGR_JR0_LS::IMSK;
}

/// Input Ring Read Index Register for Job Ring 3
pub mod IRRIR_JR3 {
    pub use super::IRRIR_JR0::IRRI;
}

/// Output Ring Write Index Register for Job Ring 3
pub mod ORWIR_JR3 {
    pub use super::ORWIR_JR0::ORWI;
}

/// Job Ring Command Register for Job Ring 3
pub mod JRCR_JR3 {
    pub use super::JRCR_JR0::PARK;
    pub use super::JRCR_JR0::RESET;
}

/// Job Ring 3 Address-Array Valid Register
pub mod JR3AAV {
    pub use super::JR0AAV::BC;
    pub use super::JR0AAV::V0;
    pub use super::JR0AAV::V1;
    pub use super::JR0AAV::V2;
    pub use super::JR0AAV::V3;
}

/// Job Ring 3 Address-Array Address 0 Register
pub mod JR3AAA0 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 3 Address-Array Address 1 Register
pub mod JR3AAA1 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 3 Address-Array Address 2 Register
pub mod JR3AAA2 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Job Ring 3 Address-Array Address 3 Register
pub mod JR3AAA3 {
    pub use super::HT0_JD_ADDR::JD_ADDR;
}

/// Partition 0 SDID register
pub mod P0SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P0SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P0SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P0SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 1 SDID register
pub mod P1SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P1SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P1SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 2 SDID register
pub mod P2SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P2SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P2SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 3 SDID register
pub mod P3SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P3SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P3SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 4 SDID register
pub mod P4SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P4SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P4SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 5 SDID register
pub mod P5SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P5SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P5SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 6 SDID register
pub mod P6SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P6SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P6SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 7 SDID register
pub mod P7SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P7SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P7SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 8 SDID register
pub mod P8SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P8SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P8SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 9 SDID register
pub mod P9SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P9SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P9SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 10 SDID register
pub mod P10SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P10SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P10SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 11 SDID register
pub mod P11SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P11SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P11SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 12 SDID register
pub mod P12SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P12SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P12SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 13 SDID register
pub mod P13SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P13SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P13SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 14 SDID register
pub mod P14SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P14SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P14SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Partition 15 SDID register
pub mod P15SDID_JR3 {
    pub use super::P0SDID_PG0::SDID;
}

/// Secure Memory Access Permissions register
pub mod P15SMAPR_JR3 {
    pub use super::P0SMAPR_PG0::CSP;
    pub use super::P0SMAPR_PG0::G1_READ;
    pub use super::P0SMAPR_PG0::G1_SMBLOB;
    pub use super::P0SMAPR_PG0::G1_TDO;
    pub use super::P0SMAPR_PG0::G1_WRITE;
    pub use super::P0SMAPR_PG0::G2_READ;
    pub use super::P0SMAPR_PG0::G2_SMBLOB;
    pub use super::P0SMAPR_PG0::G2_TDO;
    pub use super::P0SMAPR_PG0::G2_WRITE;
    pub use super::P0SMAPR_PG0::PARTITION_KMOD;
    pub use super::P0SMAPR_PG0::PSP;
    pub use super::P0SMAPR_PG0::SMAG_LCK;
    pub use super::P0SMAPR_PG0::SMAP_LCK;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG2_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Access Group Registers
pub mod P15SMAG1_JR3 {
    pub use super::P0SMAG2_PG0::Gx_ID00;
    pub use super::P0SMAG2_PG0::Gx_ID01;
    pub use super::P0SMAG2_PG0::Gx_ID02;
    pub use super::P0SMAG2_PG0::Gx_ID03;
    pub use super::P0SMAG2_PG0::Gx_ID04;
    pub use super::P0SMAG2_PG0::Gx_ID05;
    pub use super::P0SMAG2_PG0::Gx_ID06;
    pub use super::P0SMAG2_PG0::Gx_ID07;
    pub use super::P0SMAG2_PG0::Gx_ID08;
    pub use super::P0SMAG2_PG0::Gx_ID09;
    pub use super::P0SMAG2_PG0::Gx_ID10;
    pub use super::P0SMAG2_PG0::Gx_ID11;
    pub use super::P0SMAG2_PG0::Gx_ID12;
    pub use super::P0SMAG2_PG0::Gx_ID13;
    pub use super::P0SMAG2_PG0::Gx_ID14;
    pub use super::P0SMAG2_PG0::Gx_ID15;
    pub use super::P0SMAG2_PG0::Gx_ID16;
    pub use super::P0SMAG2_PG0::Gx_ID17;
    pub use super::P0SMAG2_PG0::Gx_ID18;
    pub use super::P0SMAG2_PG0::Gx_ID19;
    pub use super::P0SMAG2_PG0::Gx_ID20;
    pub use super::P0SMAG2_PG0::Gx_ID21;
    pub use super::P0SMAG2_PG0::Gx_ID22;
    pub use super::P0SMAG2_PG0::Gx_ID23;
    pub use super::P0SMAG2_PG0::Gx_ID24;
    pub use super::P0SMAG2_PG0::Gx_ID25;
    pub use super::P0SMAG2_PG0::Gx_ID26;
    pub use super::P0SMAG2_PG0::Gx_ID27;
    pub use super::P0SMAG2_PG0::Gx_ID28;
    pub use super::P0SMAG2_PG0::Gx_ID29;
    pub use super::P0SMAG2_PG0::Gx_ID30;
    pub use super::P0SMAG2_PG0::Gx_ID31;
}

/// Secure Memory Command Register
pub mod SMCR_JR3 {
    pub use super::SMCR_PG0::CMD;
    pub use super::SMCR_PG0::PAGE;
    pub use super::SMCR_PG0::PRTN;
}

/// Secure Memory Command Status Register
pub mod SMCSR_JR3 {
    pub use super::SMCSR_PG0::AERR;
    pub use super::SMCSR_PG0::CERR;
    pub use super::SMCSR_PG0::PAGE;
    pub use super::SMCSR_PG0::PO;
    pub use super::SMCSR_PG0::PRTN;
}

/// Recoverable Error Interrupt Record 0 for Job Ring 3
pub mod REIR0JR3 {
    pub use super::REIR0JR0::MISS;
    pub use super::REIR0JR0::TYPE;
}

/// Recoverable Error Interrupt Record 2 for Job Ring 3
pub mod REIR2JR3 {
    pub use super::REIR2JR0::ADDR;
}

/// Recoverable Error Interrupt Record 4 for Job Ring 3
pub mod REIR4JR3 {
    pub use super::REIR4JR0::AXCACHE;
    pub use super::REIR4JR0::AXPROT;
    pub use super::REIR4JR0::DID;
    pub use super::REIR4JR0::ERR;
    pub use super::REIR4JR0::ICID;
    pub use super::REIR4JR0::MIX;
    pub use super::REIR4JR0::RWB;
}

/// Recoverable Error Interrupt Record 5 for Job Ring 3
pub mod REIR5JR3 {
    pub use super::REIR5JR0::BID;
    pub use super::REIR5JR0::BNDG;
    pub use super::REIR5JR0::KEY;
    pub use super::REIR5JR0::KMOD;
    pub use super::REIR5JR0::SMA;
    pub use super::REIR5JR0::TDSC;
}

/// RTIC Status Register
pub mod RSTA {

    /// RTIC Idle/Busy Status. When busy, the RTIC cannot be written to.
    pub mod BSY {
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

            /// 0b0: RTIC Idle.
            pub const IDLE: u32 = 0b0;

            /// 0b1: RTIC Busy.
            pub const BUSY: u32 = 0b1;
        }
    }

    /// Hash Once Operation Completed (Hash Done)
    pub mod HD {
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

            /// 0b0: Boot authentication disabled
            pub const BOOT_AUTH_DISABLED: u32 = 0b0;

            /// 0b1: Authenticate code/generate reference hash value. This bit cannot be modified during run-time checking mode.
            pub const BOOT_AUTH_ENABLED: u32 = 0b1;
        }
    }

    /// Security Violation
    pub mod SV {
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

            /// 0b0: Memory block contents authenticated.
            pub const BLOCKS_AUTHENTICATED: u32 = 0b0;

            /// 0b1: Memory block hash doesn't match reference value.
            pub const BLOCKS_HASH_MISMATCH: u32 = 0b1;
        }
    }

    /// Hashing Error
    pub mod HE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SV::RW;
    }

    /// Memory Integrity Status
    pub mod MIS {
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

            /// 0b0000: Memory Block X is valid or state unknown
            pub const VALID_OR_UNKNOWN: u32 = 0b0000;

            /// 0b0001: Memory Block X has been corrupted
            pub const CORRUPTED: u32 = 0b0001;
        }
    }

    /// Address Error
    pub mod AE {
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

            /// 0b0000: All reads by RTIC were valid.
            pub const ADDR_VALID: u32 = 0b0000;

            /// 0b0001: An illegal address was accessed by the RTIC
            pub const ADDR_ERROR: u32 = 0b0001;
        }
    }

    /// RTIC Watchdog Error
    pub mod WE {
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

            /// 0b0: No RTIC Watchdog timer error has occurred.
            pub const WATCHDOG_GOING: u32 = 0b0;

            /// 0b1: RTIC Watchdog timer has expired prior to completing a round of hashing.
            pub const WATCHDOG_ERROR: u32 = 0b1;
        }
    }

    /// All Blocks Hashed
    pub mod ABH {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Hash Once Blocks Disabled
    pub mod HOD {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Time Blocks Disabled
    pub mod RTD {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTIC Current State. Indicates the current state of the RTIC controller.
    pub mod CS {
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

            /// 0b00: Idle State
            pub const IDLE: u32 = 0b00;

            /// 0b01: Single Hash State
            pub const SINGLE_HASH: u32 = 0b01;

            /// 0b10: Run-time State
            pub const RUN_TIME: u32 = 0b10;

            /// 0b11: Error State
            pub const ERROR: u32 = 0b11;
        }
    }
}

/// RTIC Command Register
pub mod RCMD {

    /// Clear Interrupt
    pub mod CINT {
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

            /// 0b0: Do not clear interrupt
            pub const KEEP_INTR: u32 = 0b0;

            /// 0b1: Clear interrupt. This bit cannot be modified during run-time checking mode
            pub const CLEAR_INTR: u32 = 0b1;
        }
    }

    /// Hash once
    pub mod HO {
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

            /// 0b0: Boot authentication disabled
            pub const BOOT_AUTH_DISABLED: u32 = 0b0;

            /// 0b1: Authenticate code/generate reference hash value. This bit cannot be modified during run-time checking mode.
            pub const BOOT_AUTH_ENABLED: u32 = 0b1;
        }
    }

    /// Run time check
    pub mod RTC {
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

            /// 0b0: Run-time checking disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Verify run-time memory blocks continually
            pub const CONTINOUS: u32 = 0b1;
        }
    }

    /// Run Time Disable
    pub mod RTD {
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

            /// 0b0: Allow Run Time Mode
            pub const ENABLED: u32 = 0b0;

            /// 0b1: Prevent Run Time Mode
            pub const DISABLED: u32 = 0b1;
        }
    }
}

/// RTIC Control Register
pub mod RCTL {

    /// Interrupt Enable
    pub mod IE {
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

            /// 0b0: Interrupts disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Interrupts enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// RTIC Request Size
    pub mod RREQS {
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

    /// Hash Once Memory Enable
    pub mod HOME {
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

    /// Run Time Memory Enable
    pub mod RTME {
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

    /// Run Time Memory Unlock
    pub mod RTMU {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTIC Algorithm Select
    pub mod RALG {
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

    /// RTIC setting for the IPG_IDLE signal
    pub mod RIDLE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTIC Throttle Register
pub mod RTHR {

    /// Run Time Mode DMA Throttle
    pub mod RTHR {
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

/// RTIC Watchdog Timer
pub mod RWDOG {

    /// Run Time Watchdog Time-Out value
    pub mod RWDOG {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u64 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTIC Endian Register
pub mod REND {

    /// RTIC Endian Platform Override
    pub mod REPO {
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

            /// 0b0000: Byte Swap Memory Block A
            pub const BYTE_SWAP_BLOCK_A: u32 = 0b0000;
        }
    }

    /// RTIC Byte Swap
    pub mod RBS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::REPO::RW;
    }

    /// RTIC Half-Word Swap
    pub mod RHWS {
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

            /// 0b0000: Half-Word Swap Memory Block A
            pub const HALFWORD_SWAP_BLOCK_A: u32 = 0b0000;
        }
    }

    /// RTIC Word Swap
    pub mod RWS {
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

            /// 0b0000: Word Swap Memory Block A
            pub const WORD_SWAP_BLOCK_A: u32 = 0b0000;
        }
    }
}

/// Recoverable Error Interrupt Record 0 for RTIC
pub mod REIR0RTIC {
    pub use super::REIR0JR0::MISS;
    pub use super::REIR0JR0::TYPE;
}

/// Recoverable Error Interrupt Record 2 for RTIC
pub mod REIR2RTIC {

    /// This register holds the address associated with the recoverable RTIC error.
    pub mod ADDR {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (64 bits: 0xffffffffffffffff << 0)
        pub const mask: u64 = 0xffffffffffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Recoverable Error Interrupt Record 4 for RTIC
pub mod REIR4RTIC {
    pub use super::REIR4JR0::AXCACHE;
    pub use super::REIR4JR0::AXPROT;
    pub use super::REIR4JR0::DID;
    pub use super::REIR4JR0::ERR;
    pub use super::REIR4JR0::ICID;
    pub use super::REIR4JR0::MIX;
    pub use super::REIR4JR0::RWB;
}

/// Recoverable Error Interrupt Record 5 for RTIC
pub mod REIR5RTIC {

    /// This field holds the block identifier (see Internal Block ID) of the source of the AXI transaction associated with the recoverable error
    pub mod BID {
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

    /// SAFE indicates whether the AXI transaction associated with the recoverable error was a "safe" transaction
    pub mod SAFE {
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

    /// This field indicates whether the bus transaction associated with the recoverable error was an attempted access to CAAM Secure Memory
    pub mod SMA {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// C0C1MR and C0C1MR_RNG
/// C0C1MR: C0C1MR and C0C1MR_PK
/// C0C1MR: CCB 0 Class 1 Mode Register Format for Non-Public Key Algorithms
/// C0C1MR_PK: CCB 0 Class 1 Mode Register Format for Public Key Algorithms
/// C0C1MR_RNG: CCB 0 Class 1 Mode Register Format for RNG4
pub mod C0C1MR {

    /// Encrypt/Decrypt
    pub mod ENC {
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

            /// 0b0: Decrypt.
            pub const DECRYPT: u32 = 0b0;

            /// 0b1: Encrypt.
            pub const ENCRYPT: u32 = 0b1;
        }
    }

    /// ICV Checking / Test AESA fault detection
    pub mod ICV_TEST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Algorithm State
    pub mod AS {
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

            /// 0b00: Update
            pub const UPDATE: u32 = 0b00;

            /// 0b01: Initialize
            pub const INITIALIZE: u32 = 0b01;

            /// 0b10: Finalize
            pub const FINALIZE: u32 = 0b10;

            /// 0b11: Initialize/Finalize
            pub const INIT_FINAL: u32 = 0b11;
        }
    }

    /// Additional Algorithm information
    pub mod AAI {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (9 bits: 0x1ff << 4)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Algorithm. This field specifies which algorithm is being selected.
    pub mod ALG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00010000: AES
            pub const AES: u32 = 0b00010000;

            /// 0b00100000: DES
            pub const DES: u32 = 0b00100000;

            /// 0b00100001: 3DES
            pub const DES_3: u32 = 0b00100001;

            /// 0b01010000: RNG
            pub const RNG: u32 = 0b01010000;
        }
    }

    /// PKHA_MODE least significant 12 bits
    pub mod PKHA_MODE_LS {
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

    /// PKHA_MODE most-significant 4 bits
    pub mod PKHA_MODE_MS {
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

    /// Test Mode Request
    pub mod TST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Prediction Resistance
    pub mod PR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// State Handle
    pub mod SH {
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

            /// 0b00: State Handle 0
            pub const SH0: u32 = 0b00;

            /// 0b01: State Handle 1
            pub const SH1: u32 = 0b01;
        }
    }

    /// NonZero bytes
    pub mod NZB {
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

            /// 0b0: Generate random data with all-zero bytes permitted.
            pub const ZERO_BYTES_OK: u32 = 0b0;

            /// 0b1: Generate random data without any all-zero bytes.
            pub const NO_ZERO_BYTES: u32 = 0b1;
        }
    }

    /// Odd Byte Parity
    pub mod OBP {
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

            /// 0b0: No odd byte parity.
            pub const NOT_ODD_BYTE_PARITY: u32 = 0b0;

            /// 0b1: Generate random data with odd byte parity.
            pub const ODD_BYTE_PARITY: u32 = 0b1;
        }
    }

    /// Personalization String Included
    pub mod PS {
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

            /// 0b0: No personalization string is included.
            pub const NO_PS_STRING: u32 = 0b0;

            /// 0b1: A personalization string is included.
            pub const PS_STRING_INCL: u32 = 0b1;
        }
    }

    /// Additional Input Included
    pub mod AI {
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

            /// 0b0: No additional entropy input has been provided.
            pub const NO_ADDL_INPUT: u32 = 0b0;

            /// 0b1: Additional entropy input has been provided.
            pub const ADDL_INPUT: u32 = 0b1;
        }
    }

    /// Secure Key
    pub mod SK {
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

            /// 0b0: The destination for the RNG data is specified by the FIFO STORE command.
            pub const RNG_DEST_SPECD_BY_FIFO_STORE: u32 = 0b0;

            /// 0b1: The RNG data will go to the JDKEKR, TDKEKR and DSKR.
            pub const RNG_DATA_TO_KEKR: u32 = 0b1;
        }
    }
}

/// CCB 0 Class 1 Key Size Register
pub mod C0C1KSR {

    /// Class 1 Key Size. This is the size of a Class 1 Key measured in bytes
    pub mod C1KS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CCB 0 Class 1 Data Size Register
pub mod C0C1DSR {

    /// Class 1 Data Size
    pub mod C1DS {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u64 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Class 1 Data Size Carry
    pub mod C1CY {
        /// Offset (32 bits)
        pub const offset: u64 = 32;
        /// Mask (1 bit: 1 << 32)
        pub const mask: u64 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No carry out of the C1 Data Size Reg.
            pub const NO_C1DS_CARRY: u64 = 0b0;

            /// 0b1: There was a carry out of the C1 Data Size Reg.
            pub const C1DS_CARRY: u64 = 0b1;
        }
    }

    /// Class 1 Data Size Number of bits
    pub mod NUMBITS {
        /// Offset (61 bits)
        pub const offset: u64 = 61;
        /// Mask (3 bits: 0b111 << 61)
        pub const mask: u64 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CCB 0 Class 1 ICV Size Register
pub mod C0C1ICVSR {

    /// Class 1 ICV Size, in Bytes.
    pub mod C1ICVS {
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

/// CCB 0 CHA Control Register
pub mod C0CCTRL {

    /// Reset CCB
    pub mod CCB {
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

            /// 0b0: Do Not Reset
            pub const DO_NOT_CCB: u32 = 0b0;

            /// 0b1: Reset CCB
            pub const RESET_CCB: u32 = 0b1;
        }
    }

    /// Reset AESA. Writing a 1 to this bit resets the AES Accelerator.
    pub mod AES {
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

            /// 0b0: Do Not Reset
            pub const DO_NOT_RESET_AESA: u32 = 0b0;

            /// 0b1: Reset AES Accelerator
            pub const RESET_AESA: u32 = 0b1;
        }
    }

    /// Reset DESA. Writing a 1 to this bit resets the DES Accelerator.
    pub mod DES {
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

            /// 0b0: Do Not Reset
            pub const DO_NOT_RESET_DESA: u32 = 0b0;

            /// 0b1: Reset DES Accelerator
            pub const RESET_DESA: u32 = 0b1;
        }
    }

    /// Reset PKHA. Writing a 1 to this bit resets the Public Key Hardware Accelerator.
    pub mod PK {
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

            /// 0b0: Do Not Reset
            pub const DO_NOT_RESET_PKHA: u32 = 0b0;

            /// 0b1: Reset Public Key Hardware Accelerator
            pub const RESET_PKHA: u32 = 0b1;
        }
    }

    /// Reset MDHA. Writing a 1 to this bit resets the Message Digest Hardware Accelerator.
    pub mod MD {
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

            /// 0b0: Do Not Reset
            pub const DO_NOT_RESET_MDHA: u32 = 0b0;

            /// 0b1: Reset Message Digest Hardware Accelerator
            pub const RESET_MDHA: u32 = 0b1;
        }
    }

    /// Reset CRCA. Writing a 1 to this bit resets the CRC Accelerator.
    pub mod CRC {
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

            /// 0b0: Do Not Reset
            pub const DO_NOT_RESET_CRCA: u32 = 0b0;

            /// 0b1: Reset CRC Accelerator
            pub const RESET_CRCA: u32 = 0b1;
        }
    }

    /// Reset Random Number Generator. Writing a 1 to this bit resets the Random Number Generator.
    pub mod RNG {
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

            /// 0b0: Do Not Reset
            pub const DO_NOT_RESET_RNG: u32 = 0b0;

            /// 0b1: Reset Random Number Generator Block.
            pub const RESET_RNG: u32 = 0b1;
        }
    }

    /// Unload the PKHA A0 Memory
    pub mod UA0 {
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

            /// 0b0: Don't unload the PKHA A0 Memory.
            pub const DONT_UNLOAD_PKHA_A0: u32 = 0b0;

            /// 0b1: Unload the PKHA A0 Memory into OFIFO.
            pub const UNLOAD_PKHA_A0: u32 = 0b1;
        }
    }

    /// Unload the PKHA A1 Memory
    pub mod UA1 {
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

            /// 0b0: Don't unload the PKHA A1 Memory.
            pub const DONT_UNLOAD_PKHA_A1: u32 = 0b0;

            /// 0b1: Unload the PKHA A1 Memory into OFIFO.
            pub const UNLOAD_PKHA_A1: u32 = 0b1;
        }
    }

    /// Unload the PKHA A2 Memory
    pub mod UA2 {
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

            /// 0b0: Don't unload the PKHA A2 Memory.
            pub const DONT_UNLOAD_PKHA_A2: u32 = 0b0;

            /// 0b1: Unload the PKHA A2 Memory into OFIFO.
            pub const UNLOAD_PKHA_A2: u32 = 0b1;
        }
    }

    /// Unload the PKHA A3 Memory
    pub mod UA3 {
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

            /// 0b0: Don't unload the PKHA A3 Memory.
            pub const DONT_UNLOAD_PKHA_A3: u32 = 0b0;

            /// 0b1: Unload the PKHA A3 Memory into OFIFO.
            pub const UNLOAD_PKHA_A3: u32 = 0b1;
        }
    }

    /// Unload the PKHA B0 Memory
    pub mod UB0 {
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

            /// 0b0: Don't unload the PKHA B0 Memory.
            pub const DONT_UNLOAD_PKHA_B0: u32 = 0b0;

            /// 0b1: Unload the PKHA B0 Memory into OFIFO.
            pub const UNLOAD_PKHA_B0: u32 = 0b1;
        }
    }

    /// Unload the PKHA B1 Memory
    pub mod UB1 {
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

            /// 0b0: Don't unload the PKHA B1 Memory.
            pub const DONT_UNLOAD_PKHA_B1: u32 = 0b0;

            /// 0b1: Unload the PKHA B1 Memory into OFIFO.
            pub const UNLOAD_PKHA_B1: u32 = 0b1;
        }
    }

    /// Unload the PKHA B2 Memory
    pub mod UB2 {
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

            /// 0b0: Don't unload the PKHA B2 Memory.
            pub const DONT_UNLOAD_PKHA_B2: u32 = 0b0;

            /// 0b1: Unload the PKHA B2 Memory into OFIFO.
            pub const UNLOAD_PKHA_B2: u32 = 0b1;
        }
    }

    /// Unload the PKHA B3 Memory
    pub mod UB3 {
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

            /// 0b0: Don't unload the PKHA B3 Memory.
            pub const DONT_UNLOAD_PKHA_B3: u32 = 0b0;

            /// 0b1: Unload the PKHA B3 Memory into OFIFO.
            pub const UNLOAD_PKHA_B3: u32 = 0b1;
        }
    }

    /// Unload the PKHA N Memory
    pub mod UN {
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

            /// 0b0: Don't unload the PKHA N Memory.
            pub const DONT_UNLOAD_PKHA_N: u32 = 0b0;

            /// 0b1: Unload the PKHA N Memory into OFIFO.
            pub const UNLOAD_PKHA_N: u32 = 0b1;
        }
    }

    /// Unload the PKHA A Memory
    pub mod UA {
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

            /// 0b0: Don't unload the PKHA A Memory.
            pub const DONT_UNLOAD_PKHA_A: u32 = 0b0;

            /// 0b1: Unload the PKHA A Memory into OFIFO.
            pub const UNLOAD_PKHA_A: u32 = 0b1;
        }
    }

    /// Unload the PKHA B Memory
    pub mod UB {
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

            /// 0b0: Don't unload the PKHA B Memory.
            pub const DONT_UNLOAD_PKHA_B: u32 = 0b0;

            /// 0b1: Unload the PKHA B Memory into OFIFO.
            pub const UNLOAD_PKHA_B: u32 = 0b1;
        }
    }
}

/// CCB 0 Interrupt Control Register
pub mod C0ICTL {

    /// AESA done interrupt
    pub mod ADI {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DESA done interrupt
    pub mod DDI {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKHA (Public Key) done interrupt
    pub mod PDI {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MDHA (hashing) done interrupt
    pub mod MDI {
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

    /// CRCA done interrupt
    pub mod CDI {
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

    /// RNG done interrupt
    pub mod RNDI {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AESA Error Interrupt asserted.
    pub mod AEI {
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

            /// 0b0: No AESA error detected
            pub const NO_AESA_ERROR: u32 = 0b0;

            /// 0b1: AESA error detected
            pub const AESA_ERROR: u32 = 0b1;
        }
    }

    /// DESA Error Interrupt asserted.
    pub mod DEI {
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

            /// 0b0: No DESA error detected
            pub const NO_DESA_ERROR: u32 = 0b0;

            /// 0b1: DESA error detected
            pub const DESA_ERROR: u32 = 0b1;
        }
    }

    /// PKHA (Public Key) Error Interrupt asserted.
    pub mod PEI {
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

            /// 0b0: No PKHA error detected
            pub const NO_PKHA_ERROR: u32 = 0b0;

            /// 0b1: PKHA error detected
            pub const PKHA_ERROR: u32 = 0b1;
        }
    }

    /// MDHA (hashing) Error Interrupt asserted.
    pub mod MEI {
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

            /// 0b0: No MDHA error detected
            pub const NO_MDHA_ERROR: u32 = 0b0;

            /// 0b1: MDHA error detected
            pub const MDHA_ERROR: u32 = 0b1;
        }
    }

    /// CRCA Error Interrupt asserted.
    pub mod CEI {
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

            /// 0b0: No CRCA error detected
            pub const NO_CRCA_ERROR: u32 = 0b0;

            /// 0b1: CRCA error detected
            pub const CRCA_ERROR: u32 = 0b1;
        }
    }

    /// RNG Error Interrupt asserted.
    pub mod RNEI {
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

            /// 0b0: No RNG error detected
            pub const NO_RNG_ERROR: u32 = 0b0;

            /// 0b1: RNG error detected
            pub const RNG_ERROR: u32 = 0b1;
        }
    }
}

/// CCB 0 Clear Written Register
pub mod C0CWR {

    /// Clear the Class 1 Mode Register
    pub mod C1M {
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

            /// 0b0: Don't clear the Class 1 Mode Register.
            pub const DONT_CLEAR_C1_MODE: u32 = 0b0;

            /// 0b1: Clear the Class 1 Mode Register.
            pub const CLEAR_C1_MODE: u32 = 0b1;
        }
    }

    /// Clear the Class 1 Data Size Register
    pub mod C1DS {
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

            /// 0b0: Don't clear the Class 1 Data Size Register.
            pub const DONT_CLEAR_C1_DATA_SIZE: u32 = 0b0;

            /// 0b1: Clear the Class 1 Data Size Register.
            pub const CLEAR_C1_DATA_SIZE: u32 = 0b1;
        }
    }

    /// Clear the Class 1 ICV Size Register
    pub mod C1ICV {
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

            /// 0b0: Don't clear the Class 1 ICV Size Register.
            pub const DONT_CLEAR_C1_ICV_SIZE: u32 = 0b0;

            /// 0b1: Clear the Class 1 ICV Size Register.
            pub const CLEAR_C1_ICV_SIZE: u32 = 0b1;
        }
    }

    /// Clear the Class 1 Context Register
    pub mod C1C {
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

            /// 0b0: Don't clear the Class 1 Context Register.
            pub const DONT_CLEAR_C1_CTXR: u32 = 0b0;

            /// 0b1: Clear the Class 1 Context Register.
            pub const CLEAR_C1_CTXR: u32 = 0b1;
        }
    }

    /// Clear the Class 1 Key Register
    pub mod C1K {
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

            /// 0b0: Don't clear the Class 1 Key Register.
            pub const DONT_CLEAR_C1_KEYR: u32 = 0b0;

            /// 0b1: Clear the Class 1 Key Register.
            pub const CLEAR_C1_KEYR: u32 = 0b1;
        }
    }

    /// Clear the PKHA A Size Register
    pub mod CPKA {
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

            /// 0b0: Don't clear the PKHA A Size Register.
            pub const DONT_CLEAR_PKHA_A_SIZE: u32 = 0b0;

            /// 0b1: Clear the PKHA A Size Register.
            pub const CLEAR_PKHA_A_SIZE: u32 = 0b1;
        }
    }

    /// Clear the PKHA B Size Register
    pub mod CPKB {
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

            /// 0b0: Don't clear the PKHA B Size Register.
            pub const DONT_CLEAR_PKHA_B_SIZE: u32 = 0b0;

            /// 0b1: Clear the PKHA B Size Register.
            pub const CLEAR_PKHA_B_SIZE: u32 = 0b1;
        }
    }

    /// Clear the PKHA N Size Register
    pub mod CPKN {
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

            /// 0b0: Don't clear the PKHA N Size Register.
            pub const DONT_CLEAR_PKHA_N_SIZE: u32 = 0b0;

            /// 0b1: Clear the PKHA N Size Register.
            pub const CLEAR_PKHA_N_SIZE: u32 = 0b1;
        }
    }

    /// Clear the PKHA E Size Register
    pub mod CPKE {
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

            /// 0b0: Don't clear the PKHA E Size Register..
            pub const DONT_CLEAR_PKHA_E_SIZE: u32 = 0b0;

            /// 0b1: Clear the PKHA E Size Register.
            pub const CLEAR_PKHA_E_SIZE: u32 = 0b1;
        }
    }

    /// Clear the Class 2 Mode Register
    pub mod C2M {
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

            /// 0b0: Don't clear the Class 2 Mode Register.
            pub const DONT_CLEAR_C2_MODE: u32 = 0b0;

            /// 0b1: Clear the Class 2 Mode Register.
            pub const CLEAR_C2_MODE: u32 = 0b1;
        }
    }

    /// Clear the Class 2 Data Size Registers
    pub mod C2DS {
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

            /// 0b0: Don't clear the Class 2 Data Size Register.
            pub const DONT_CLEAR_C2_DATA_SIZE: u32 = 0b0;

            /// 0b1: Clear the Class 2 Data Size Register.
            pub const CLEAR_C2_DATA_SIZE: u32 = 0b1;
        }
    }

    /// Clear the Class 2 Context Register
    pub mod C2C {
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

            /// 0b0: Don't clear the Class 2 Context Register.
            pub const DONT_CLEAR_C2_CTXR: u32 = 0b0;

            /// 0b1: Clear the Class 2 Context Register.
            pub const CLEAR_C2_CTXR: u32 = 0b1;
        }
    }

    /// Clear the Class 2 Key Register
    pub mod C2K {
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

            /// 0b0: Don't clear the Class 2 Key Register.
            pub const DONT_CLEAR_C2_KEYR: u32 = 0b0;

            /// 0b1: Clear the Class 2 Key Register.
            pub const CLEAR_C2_KEYR: u32 = 0b1;
        }
    }

    /// Clear Descriptor Sharing signal
    pub mod CDS {
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

            /// 0b0: Don't clear the shared descriptor signal.
            pub const DONT_CLEAR_SD_SIGNAL: u32 = 0b0;

            /// 0b1: Clear the shared descriptor signal.
            pub const CLEAR_SD_SIGNAL: u32 = 0b1;
        }
    }

    /// Clear Class 2 Done Interrupt. Writing a 1 to this bit clears the Class 2 done interrupt.
    pub mod C2D {
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

            /// 0b0: Don't clear the Class 2 done interrrupt.
            pub const DONT_CLEAR_C2_DONE_INT: u32 = 0b0;

            /// 0b1: Clear the Class 2 done interrrupt.
            pub const CLEAR_C2_DONE_INT: u32 = 0b1;
        }
    }

    /// Clear Class 1 Done Interrupt. Writing a 1 to this bit clears the Class 1 done interrupt.
    pub mod C1D {
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

            /// 0b0: Don't clear the Class 1 done interrrupt.
            pub const DONT_CLEAR_C1_DONE_INT: u32 = 0b0;

            /// 0b1: Clear the Class 1 done interrrupt.
            pub const CLEAR_C1_DONE_INT: u32 = 0b1;
        }
    }

    /// Reset Class 2 CHA
    pub mod C2RST {
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

            /// 0b0: Don't reset the Class 2 CHA.
            pub const DONT_RESET_C2_CHA: u32 = 0b0;

            /// 0b1: Reset the Class 2 CHA.
            pub const RESET_C2_CHA: u32 = 0b1;
        }
    }

    /// Reset Class 1 CHA
    pub mod C1RST {
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

            /// 0b0: Don't reset the Class 1 CHA.
            pub const DONT_RESET_C1_CHA: u32 = 0b0;

            /// 0b1: Reset the Class 1 CHA.
            pub const RESET_C1_CHA: u32 = 0b1;
        }
    }

    /// Clear Output FIFO. Writing a 1 to this bit causes the Output FIFO to be cleared.
    pub mod COF {
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

            /// 0b0: Don't clear the OFIFO.
            pub const DONT_CLEAR_OFIFO: u32 = 0b0;

            /// 0b1: Clear the OFIFO.
            pub const CLEAR_OFIFO: u32 = 0b1;
        }
    }

    /// Clear Input FIFO (and NFIFO)
    pub mod CIF {
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

            /// 0b0: Don't clear the IFIFO.
            pub const DONT_CLEAR_IFIFO: u32 = 0b0;

            /// 0b1: Clear the IFIFO.
            pub const CLEAR_IFIFO: u32 = 0b1;
        }
    }
}

/// CCB 0 Status and Error Register, most-significant half
pub mod C0CSTA_MS {

    /// Error ID 1
    pub mod ERRID1 {
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

            /// 0b0001: Mode Error
            pub const MODE_ERROR: u32 = 0b0001;

            /// 0b0010: Data Size Error, including PKHA N Memory Size Error
            pub const DATA_SIZE_ERROR: u32 = 0b0010;

            /// 0b0011: Key Size Error, including PKHA E Memory Size Error
            pub const KEY_SIZE_ERROR: u32 = 0b0011;

            /// 0b0100: PKHA A Memory Size Error
            pub const PKHA_A_SIZE_ERROR: u32 = 0b0100;

            /// 0b0101: PKHA B Memory Size Error
            pub const PKHA_B_SIZE_ERROR: u32 = 0b0101;

            /// 0b0110: Data Arrived out of Sequence Error
            pub const OUT_OF_SEQ_ERROR: u32 = 0b0110;

            /// 0b0111: PKHA Divide by Zero Error
            pub const DIVIDE_BY_0_ERROR: u32 = 0b0111;

            /// 0b1000: PKHA Modulus Even Error
            pub const MOD_EVEN_ERROR: u32 = 0b1000;

            /// 0b1001: DES Key Parity Error
            pub const DES_KEY_PARITY_ERROR: u32 = 0b1001;

            /// 0b1010: ICV Check Failed
            pub const ICV_CHECK_FAILED_ERROR: u32 = 0b1010;

            /// 0b1011: Internal Hardware Failure
            pub const INTERNAL_HW_FAIL: u32 = 0b1011;

            /// 0b1100: CCM AAD Size Error (either 1. AAD flag in B0 =1 and no AAD type provided, 2. AAD flag in B0 = 0 and AAD provided, or 3. AAD flag in B0 =1 and not enough AAD provided - expecting more based on AAD size.)
            pub const CCM_AAD_SISE_ERROR: u32 = 0b1100;

            /// 0b1101: Class 1 CHA is not reset
            pub const C1_CHA_NOT_RESET_ERROR: u32 = 0b1101;

            /// 0b1110: Invalid CHA combination was selected
            pub const INVALID_CHA_COMBO_ERROR: u32 = 0b1110;

            /// 0b1111: Invalid CHA Selected
            pub const INVALID_CHA_SELECTED_ERROR: u32 = 0b1111;
        }
    }

    /// Class 1 algorithms
    pub mod CL1 {
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

            /// 0b0001: AES
            pub const AES: u32 = 0b0001;

            /// 0b0010: DES
            pub const DES: u32 = 0b0010;

            /// 0b0101: RNG
            pub const RNG: u32 = 0b0101;

            /// 0b1000: Public Key
            pub const PK: u32 = 0b1000;
        }
    }

    /// Error ID 2
    pub mod ERRID2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0001: Mode Error
            pub const MODE_ERROR: u32 = 0b0001;

            /// 0b0010: Data Size Error
            pub const DATA_SIZE_ERROR: u32 = 0b0010;

            /// 0b0011: Key Size Error
            pub const KEY_SIZE_ERROR: u32 = 0b0011;

            /// 0b0110: Data Arrived out of Sequence Error
            pub const DATA_OUT_OF_SEQ_ERROR: u32 = 0b0110;

            /// 0b1010: ICV Check Failed
            pub const ICV_CHECK_ERROR: u32 = 0b1010;

            /// 0b1011: Internal Hardware Failure
            pub const INTERNAL_HW_ERROR: u32 = 0b1011;

            /// 0b1110: Invalid CHA combination was selected.
            pub const INVALID_CHA_COMBO_ERROR: u32 = 0b1110;

            /// 0b1111: Invalid CHA Selected
            pub const INVALID_CHA_SELECT_ERROR: u32 = 0b1111;
        }
    }

    /// Class 2 Algorithms
    pub mod CL2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0100: MD5, SHA-1, SHA-224, SHA-256, SHA-384, SHA-512 and SHA-512/224, SHA-512/256
            pub const MD: u32 = 0b0100;

            /// 0b1001: CRC
            pub const CRC: u32 = 0b1001;
        }
    }
}

/// CCB 0 Status and Error Register, least-significant half
pub mod C0CSTA_LS {

    /// AESA Busy
    pub mod AB {
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

            /// 0b0: AESA Idle
            pub const AESA_IDLE: u32 = 0b0;

            /// 0b1: AESA Busy
            pub const AESA_BUSY: u32 = 0b1;
        }
    }

    /// DESA Busy
    pub mod DB {
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

            /// 0b0: DESA Idle
            pub const DESA_IDLE: u32 = 0b0;

            /// 0b1: DESA Busy
            pub const DESA_BUSY: u32 = 0b1;
        }
    }

    /// PKHA Busy
    pub mod PB {
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

            /// 0b0: PKHA Idle
            pub const PKHA_IDLE: u32 = 0b0;

            /// 0b1: PKHA Busy
            pub const PKHA_BUSY: u32 = 0b1;
        }
    }

    /// MDHA Busy
    pub mod MB {
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

            /// 0b0: MDHA Idle
            pub const MDHA_IDLE: u32 = 0b0;

            /// 0b1: MDHA Busy
            pub const MDHA_BUSY: u32 = 0b1;
        }
    }

    /// CRC Block Busy
    pub mod CB {
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

            /// 0b0: CRCA Idle
            pub const CRCA_IDLE: u32 = 0b0;

            /// 0b1: CRCA Busy
            pub const CRCA_BUSY: u32 = 0b1;
        }
    }

    /// RNG Block Busy
    pub mod RNB {
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

            /// 0b0: RNG Idle
            pub const RNG_IDLE: u32 = 0b0;

            /// 0b1: RNG Busy
            pub const RNG_BUSY: u32 = 0b1;
        }
    }

    /// Class 1 Done Interrupt. The Class 1 Done Interrupt has been asserted.
    pub mod PDI {
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

            /// 0b0: Not Done
            pub const C1_NOT_DONE: u32 = 0b0;

            /// 0b1: Done Interrupt
            pub const C1_DONE_INT: u32 = 0b1;
        }
    }

    /// Class 2 Done Interrupt. The Class 2 Done Interrupt has been asserted.
    pub mod SDI {
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

            /// 0b0: Not Done
            pub const C2_NOT_DONE: u32 = 0b0;

            /// 0b1: Done Interrupt
            pub const C2_DONE_INT: u32 = 0b1;
        }
    }

    /// Class 1 Error Interrupt. The Class 1 Error Interrupt has been asserted.
    pub mod PEI {
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

            /// 0b0: No Error
            pub const NO_C1_ERROR: u32 = 0b0;

            /// 0b1: Error Interrupt
            pub const C1_ERROR_INT: u32 = 0b1;
        }
    }

    /// Class 2 Error Interrupt. The Class 2 Error Interrupt has been asserted.
    pub mod SEI {
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

            /// 0b0: No Error
            pub const NO_C2_ERROR: u32 = 0b0;

            /// 0b1: Error Interrupt
            pub const C2_ERROR_INT: u32 = 0b1;
        }
    }

    /// Public Key is Prime
    pub mod PRM {
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

            /// 0b0: The given number is NOT prime.
            pub const NOT_PRIME: u32 = 0b0;

            /// 0b1: The given number is probably prime.
            pub const PROBABLY_PRIME: u32 = 0b1;
        }
    }

    /// GCD is One
    pub mod GCD {
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

            /// 0b0: The greatest common divisor of two numbers is NOT one.
            pub const GCD_IS_NOT_1: u32 = 0b0;

            /// 0b1: The greatest common divisor of two numbers is one.
            pub const GCD_IS_1: u32 = 0b1;
        }
    }

    /// Public Key Operation is Zero
    pub mod PIZ {
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

            /// 0b0: The result of a Public Key operation is not zero.
            pub const PK_RESULT_NONZERO: u32 = 0b0;

            /// 0b1: The result of a Public Key operation is zero.
            pub const PK_RESULT_ZERO: u32 = 0b1;
        }
    }
}

/// CCB 0 Class 1 AAD Size Register
pub mod C0C1AADSZR {

    /// AAD size in Bytes, mod 16.
    pub mod AASZ {
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

/// CCB 0 Class 1 IV Size Register
pub mod C0C1IVSZR {

    /// IV size in bytes, mod 16.
    pub mod IVSZ {
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

/// PKHA A Size Register
pub mod C0PKASZR {

    /// PKHA A Memory key size in bytes.
    pub mod PKASZ {
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

/// PKHA B Size Register
pub mod C0PKBSZR {

    /// PKHA B Memory key size in bytes.
    pub mod PKBSZ {
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

/// PKHA N Size Register
pub mod C0PKNSZR {

    /// PKHA N Memory key size in bytes.
    pub mod PKNSZ {
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

/// PKHA E Size Register
pub mod C0PKESZR {

    /// PKHA E Memory key size in bytes.
    pub mod PKESZ {
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

/// CCB 0 Class 1 Context Register Word 0
pub mod C0C1CTXR0 {

    /// Class 1 Context.
    pub mod C1CTX {
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

/// CCB 0 Class 1 Context Register Word 1
pub mod C0C1CTXR1 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 2
pub mod C0C1CTXR2 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 3
pub mod C0C1CTXR3 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 4
pub mod C0C1CTXR4 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 5
pub mod C0C1CTXR5 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 6
pub mod C0C1CTXR6 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 7
pub mod C0C1CTXR7 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 8
pub mod C0C1CTXR8 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 9
pub mod C0C1CTXR9 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 10
pub mod C0C1CTXR10 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 11
pub mod C0C1CTXR11 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 12
pub mod C0C1CTXR12 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 13
pub mod C0C1CTXR13 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 14
pub mod C0C1CTXR14 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Context Register Word 15
pub mod C0C1CTXR15 {
    pub use super::C0C1CTXR0::C1CTX;
}

/// CCB 0 Class 1 Key Registers Word 0
pub mod C0C1KR0 {

    /// Class 1 Key.
    pub mod C1KEY {
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

/// CCB 0 Class 1 Key Registers Word 1
pub mod C0C1KR1 {
    pub use super::C0C1KR0::C1KEY;
}

/// CCB 0 Class 1 Key Registers Word 2
pub mod C0C1KR2 {
    pub use super::C0C1KR0::C1KEY;
}

/// CCB 0 Class 1 Key Registers Word 3
pub mod C0C1KR3 {
    pub use super::C0C1KR0::C1KEY;
}

/// CCB 0 Class 1 Key Registers Word 4
pub mod C0C1KR4 {
    pub use super::C0C1KR0::C1KEY;
}

/// CCB 0 Class 1 Key Registers Word 5
pub mod C0C1KR5 {
    pub use super::C0C1KR0::C1KEY;
}

/// CCB 0 Class 1 Key Registers Word 6
pub mod C0C1KR6 {
    pub use super::C0C1KR0::C1KEY;
}

/// CCB 0 Class 1 Key Registers Word 7
pub mod C0C1KR7 {
    pub use super::C0C1KR0::C1KEY;
}

/// CCB 0 Class 2 Mode Register
pub mod C0C2MR {

    /// Authenticate / Protect
    pub mod AP {
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

            /// 0b0: Authenticate
            pub const AUTHENTICATE: u32 = 0b0;

            /// 0b1: Protect
            pub const PROTECT: u32 = 0b1;
        }
    }

    /// ICV Checking
    pub mod ICV {
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

            /// 0b0: Don't compare the calculated ICV against a received ICV.
            pub const NO_COMPARISON: u32 = 0b0;

            /// 0b1: Compare the calculated ICV against a received ICV.
            pub const COMPARISON: u32 = 0b1;
        }
    }

    /// Algorithm State
    pub mod AS {
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

            /// 0b00: Update.
            pub const UPDATE: u32 = 0b00;

            /// 0b01: Initialize.
            pub const INITIALIZE: u32 = 0b01;

            /// 0b10: Finalize.
            pub const FINALIZE: u32 = 0b10;

            /// 0b11: Initialize/Finalize.
            pub const INITIALIZE_FINALIZE: u32 = 0b11;
        }
    }

    /// Additional Algorithm information
    pub mod AAI {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (9 bits: 0x1ff << 4)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Algorithm. This field specifies which algorithm has been requested for an OPERATION command.
    pub mod ALG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b01000000: MD5
            pub const MD5: u32 = 0b01000000;

            /// 0b01000001: SHA-1
            pub const SHA_1: u32 = 0b01000001;

            /// 0b01000010: SHA-224
            pub const SHA_224: u32 = 0b01000010;

            /// 0b01000011: SHA-256
            pub const SHA_256: u32 = 0b01000011;

            /// 0b01000100: SHA-384
            pub const SHA_384: u32 = 0b01000100;

            /// 0b01000101: SHA-512
            pub const SHA_512: u32 = 0b01000101;

            /// 0b01000110: SHA-512/224
            pub const SHA_512_224: u32 = 0b01000110;

            /// 0b01000111: SHA-512/256
            pub const SHA_512_256: u32 = 0b01000111;

            /// 0b10010000: CRC
            pub const CRC: u32 = 0b10010000;
        }
    }
}

/// CCB 0 Class 2 Key Size Register
pub mod C0C2KSR {

    /// Class 2 key size in bytes.
    pub mod C2KS {
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

/// CCB 0 Class 2 Data Size Register
pub mod C0C2DSR {

    /// Class 2 Data Size in Bytes
    pub mod C2DS {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u64 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Class 2 Data Size Carry
    pub mod C2CY {
        /// Offset (32 bits)
        pub const offset: u64 = 32;
        /// Mask (1 bit: 1 << 32)
        pub const mask: u64 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A write to the Class 2 Data Size Register did not cause a carry.
            pub const NO_C2DS_CARRY: u64 = 0b0;

            /// 0b1: A write to the Class 2 Data Size Register caused a carry.
            pub const C2DS_CARRY: u64 = 0b1;
        }
    }

    /// Class 2 Data Size Number of bits
    pub mod NUMBITS {
        /// Offset (61 bits)
        pub const offset: u64 = 61;
        /// Mask (3 bits: 0b111 << 61)
        pub const mask: u64 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CCB 0 Class 2 ICV Size Register
pub mod C0C2ICVSZR {

    /// Class 2 ICV size (mod 8) in bytes
    pub mod ICVSZ {
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

/// CCB 0 Class 2 Context Register Word 0
pub mod C0C2CTXR0 {

    /// Class 2 Context.
    pub mod C2CTXR {
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

/// CCB 0 Class 2 Context Register Word 1
pub mod C0C2CTXR1 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 2
pub mod C0C2CTXR2 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 3
pub mod C0C2CTXR3 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 4
pub mod C0C2CTXR4 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 5
pub mod C0C2CTXR5 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 6
pub mod C0C2CTXR6 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 7
pub mod C0C2CTXR7 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 8
pub mod C0C2CTXR8 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 9
pub mod C0C2CTXR9 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 10
pub mod C0C2CTXR10 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 11
pub mod C0C2CTXR11 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 12
pub mod C0C2CTXR12 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 13
pub mod C0C2CTXR13 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 14
pub mod C0C2CTXR14 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 15
pub mod C0C2CTXR15 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 16
pub mod C0C2CTXR16 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Context Register Word 17
pub mod C0C2CTXR17 {
    pub use super::C0C2CTXR0::C2CTXR;
}

/// CCB 0 Class 2 Key Register Word 0
pub mod C0C2KEYR0 {

    /// Class 2 Key.
    pub mod C2KEY {
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

/// CCB 0 Class 2 Key Register Word 1
pub mod C0C2KEYR1 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 2
pub mod C0C2KEYR2 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 3
pub mod C0C2KEYR3 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 4
pub mod C0C2KEYR4 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 5
pub mod C0C2KEYR5 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 6
pub mod C0C2KEYR6 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 7
pub mod C0C2KEYR7 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 8
pub mod C0C2KEYR8 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 9
pub mod C0C2KEYR9 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 10
pub mod C0C2KEYR10 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 11
pub mod C0C2KEYR11 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 12
pub mod C0C2KEYR12 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 13
pub mod C0C2KEYR13 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 14
pub mod C0C2KEYR14 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 15
pub mod C0C2KEYR15 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 16
pub mod C0C2KEYR16 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 17
pub mod C0C2KEYR17 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 18
pub mod C0C2KEYR18 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 19
pub mod C0C2KEYR19 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 20
pub mod C0C2KEYR20 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 21
pub mod C0C2KEYR21 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 22
pub mod C0C2KEYR22 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 23
pub mod C0C2KEYR23 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 24
pub mod C0C2KEYR24 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 25
pub mod C0C2KEYR25 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 26
pub mod C0C2KEYR26 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 27
pub mod C0C2KEYR27 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 28
pub mod C0C2KEYR28 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 29
pub mod C0C2KEYR29 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 30
pub mod C0C2KEYR30 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 Class 2 Key Register Word 31
pub mod C0C2KEYR31 {
    pub use super::C0C2KEYR0::C2KEY;
}

/// CCB 0 FIFO Status Register
pub mod C0FIFOSTA {

    /// This is the current head of the DECO Alignment Block queue located within the Output Data FIFO
    pub mod DECOOQHEAD {
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

    /// This is the current head of the DMA queue located within the Output Data FIFO
    pub mod DMAOQHEAD {
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

    /// This is the current head of the Class 2 Alignment Block queue located within the Input Data FIFO
    pub mod C2IQHEAD {
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

    /// This is the current head of the Class 1 Alignment Block queue located within the Input Data FIFO
    pub mod C1IQHEAD {
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

/// C0NFIFO and C0NFIFO_2
/// C0NFIFO: CCB 0 iNformation FIFO When STYPE != 10b
/// C0NFIFO_2: CCB 0 iNformation FIFO When STYPE == 10b
pub mod C0NFIFO {

    /// Data Length
    pub mod DL {
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

    /// Additional Source Types
    pub mod AST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OFIFO Continuation - This bit causes the final word to not be popped from the Output Data FIFO.
    pub mod OC {
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

            /// 0b0: Allow the final word to be popped from the Output Data FIFO.
            pub const POP_OFIFO_FINAL: u32 = 0b0;

            /// 0b1: Don't pop the final word from the Output Data FIFO.
            pub const DONT_POP_OFIFO_FINAL: u32 = 0b1;
        }
    }

    /// Pad Type
    pub mod PTYPE {
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

    /// Boundary padding
    pub mod BND {
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

            /// 0b0: Don't pad.
            pub const DONT_PAD: u32 = 0b0;

            /// 0b1: Pad to the next 16-byte boundary.
            pub const PAD_TO_BOUNDARY: u32 = 0b1;
        }
    }

    /// Data Type
    pub mod DTYPE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Source Type
    pub mod STYPE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Flush Class 1. Flush the remainder of the data out of the Class 1 alignment block.
    pub mod FC1 {
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

            /// 0b0: Don't flush Class 1 data.
            pub const DONT_FLUSH_C1: u32 = 0b0;

            /// 0b1: Flush Class 1 data.
            pub const FLUSH_C1: u32 = 0b1;
        }
    }

    /// Flush Class 2
    pub mod FC2 {
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

            /// 0b0: Don't flush Class 2 data.
            pub const DONT_FLUSH_C2: u32 = 0b0;

            /// 0b1: Flush Class 2 data.
            pub const FLUSH_C2: u32 = 0b1;
        }
    }

    /// Last Class 1
    pub mod LC1 {
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

            /// 0b0: This is not the last Class 1 data.
            pub const NOT_LAST_C1: u32 = 0b0;

            /// 0b1: This is the last Class 1 data.
            pub const LAST_C1: u32 = 0b1;
        }
    }

    /// Last Class 2
    pub mod LC2 {
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

            /// 0b0: This is not the last Class 2 data.
            pub const NOT_LAST_C2: u32 = 0b0;

            /// 0b1: This is the last Class 2 data.
            pub const LAST_C2: u32 = 0b1;
        }
    }

    /// Destination
    pub mod DEST {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: DECO Alignment Block. If DTYPE == Eh, data sent to the DECO Alignment Block is dropped. This is used to skip over input data. An error is generated if a DTYPE other than Eh (drop) or Fh (message) is used with the DECO Alignment Block destination.
            pub const DECO: u32 = 0b00;

            /// 0b01: Class 1.
            pub const CLASS_1: u32 = 0b01;

            /// 0b10: Class 2.
            pub const CLASS_2: u32 = 0b10;

            /// 0b11: Both Class 1 and Class 2.
            pub const CLASS_1_2: u32 = 0b11;
        }
    }

    /// Pad Length
    pub mod PL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Pad Snoop
    pub mod PS {
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

            /// 0b0: C2 CHA snoops pad data from padding block.
            pub const FROM_PAD_BLOCK: u32 = 0b0;

            /// 0b1: C2 CHA snoops pad data from OFIFO.
            pub const FROM_OFIFO: u32 = 0b1;
        }
    }

    /// Boundary Minus 1
    pub mod BM {
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

            /// 0b0: When padding, pad to power-of-2 boundary.
            pub const BOUND_PAD_NOT_MINUS_1: u32 = 0b0;

            /// 0b1: When padding, pad to power-of-2 boundary minus 1 byte.
            pub const BOUND_PAD_MINUS_1: u32 = 0b1;
        }
    }

    /// Prediction Resistance - If PTYPE specifies random data, setting PR=1 causes the RNG to supply random data for prediction resistance (i
    pub mod PR {
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

            /// 0b0: No prediction resistance.
            pub const NO_PRED_RES: u32 = 0b0;

            /// 0b1: Prediction resistance.
            pub const PRED_RES: u32 = 0b1;
        }
    }
}

/// CCB 0 Input Data FIFO
pub mod C0IFIFO {

    /// Input Data FIFO.
    pub mod IFIFO {
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

/// CCB 0 Output Data FIFO
pub mod C0OFIFO {

    /// Output FIFO
    pub mod OFIFO {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (64 bits: 0xffffffffffffffff << 0)
        pub const mask: u64 = 0xffffffffffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO0 Job Queue Control Register, most-significant half
pub mod D0JQCR_MS {

    /// Job ID
    pub mod ID {
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

    /// Job Source
    pub mod SRC {
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

            /// 0b000: Job Ring 0
            pub const JR0: u32 = 0b000;

            /// 0b001: Job Ring 1
            pub const JR1: u32 = 0b001;

            /// 0b010: Job Ring 2
            pub const JR2: u32 = 0b010;

            /// 0b011: Job Ring 3
            pub const JR3: u32 = 0b011;

            /// 0b100: RTIC
            pub const RTIC: u32 = 0b100;
        }
    }

    /// Allow Make Trusted Descriptor
    pub mod AMTD {
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

            /// 0b0: The Allowed Make Trusted Descriptor bit was NOT set.
            pub const AMTD_NOT_SET: u32 = 0b0;

            /// 0b1: The Allowed Make Trusted Descriptor bit was set.
            pub const AMTD_SET: u32 = 0b1;
        }
    }

    /// Shared Descriptor burst
    pub mod SOB {
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

            /// 0b0: Shared Descriptor has NOT been loaded.
            pub const SD_NOT_LOADED: u32 = 0b0;

            /// 0b1: Shared Descriptor HAS been loaded.
            pub const SD_LOADED: u32 = 0b1;
        }
    }

    /// Double word swap. Causes/allows dword swapping of addresses, and MOVE and MATH immediate values.
    pub mod DWS {
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

            /// 0b0: Double Word Swap is NOT set.
            pub const NO_DWS: u32 = 0b0;

            /// 0b1: Double Word Swap is set.
            pub const DWS: u32 = 0b1;
        }
    }

    /// Share From
    pub mod SHR_FROM {
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

    /// Immediate Little Endian
    pub mod ILE {
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

            /// 0b0: No byte-swapping is performed for immediate data transferred to or from the Descriptor Buffer.
            pub const NO_BYTE_SWAP: u32 = 0b0;

            /// 0b1: Byte-swapping is performed for immediate data transferred to or from the Descriptor Buffer.
            pub const BYTE_SWAP: u32 = 0b1;
        }
    }

    /// Four Words. job queue controller is passing at least 4 words of the Descriptor to DECO.
    pub mod FOUR {
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

            /// 0b0: DECO has not been given at least four words of the descriptor.
            pub const NOT_FOUR_WORDS: u32 = 0b0;

            /// 0b1: DECO has been given at least four words of the descriptor.
            pub const FOUR_WORDS: u32 = 0b1;
        }
    }

    /// Whole Descriptor
    pub mod WHL {
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

            /// 0b0: DECO has not been given the whole descriptor.
            pub const NOT_WHOLE_DESC: u32 = 0b0;

            /// 0b1: DECO has been given the whole descriptor.
            pub const WHOLE_DESC: u32 = 0b1;
        }
    }

    /// Single Step Mode
    pub mod SING {
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

            /// 0b0: Do not tell DECO to execute the descriptor in single-step mode.
            pub const NOT_SINGLE_STEP_MODE: u32 = 0b0;

            /// 0b1: Tell DECO to execute the descriptor in single-step mode.
            pub const SINGLE_STEP_MODE: u32 = 0b1;
        }
    }

    /// Step
    pub mod STEP {
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

            /// 0b0: DECO has not been told to execute the next command in the descriptor.
            pub const DONT_STEP: u32 = 0b0;

            /// 0b1: DECO has been told to execute the next command in the descriptor.
            pub const STEP: u32 = 0b1;
        }
    }
}

/// DECO0 Job Queue Control Register, least-significant half
pub mod D0JQCR_LS {

    /// Command
    pub mod CMD {
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

/// DECO0 Descriptor Address Register
pub mod D0DAR {

    /// Descriptor Pointer. Memory address of the Descriptor. Needed for write-back purposes.
    pub mod DPTR {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (36 bits: 0xfffffffff << 0)
        pub const mask: u64 = 0xfffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO0 Operation Status Register, most-significant half
pub mod D0OPSTA_MS {

    /// If ERRTYP indicates no error, this field contains PKHA/Math Status, as defined below
    pub mod STATUS {
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

    /// Command index: A pointer to a 32-bit word within the descriptor
    pub mod COMMAND_INDEX {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Non-Local Jump
    pub mod NLJ {
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

            /// 0b0: The original job descriptor running in this DECO has not caused another job descriptor to be executed.
            pub const ORIGINAL_DESC: u32 = 0b0;

            /// 0b1: The original job descriptor running in this DECO has caused another job descriptor to be executed.
            pub const NON_LOCAL_DECR: u32 = 0b1;
        }
    }

    /// Status Type
    pub mod STATUS_TYPE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: no error
            pub const NOT_AN_ERROR: u32 = 0b0000;

            /// 0b0001: DMA error
            pub const DMA_ERROR: u32 = 0b0001;

            /// 0b0010: CCB error
            pub const CCB_ERROR: u32 = 0b0010;

            /// 0b0011: Jump Halt User Status
            pub const JHALT_STATUS: u32 = 0b0011;

            /// 0b0100: DECO error
            pub const DECO_ERROR: u32 = 0b0100;

            /// 0b0111: Jump Halt Condition Code
            pub const JHALT_COND: u32 = 0b0111;
        }
    }
}

/// DECO0 Operation Status Register, least-significant half
pub mod D0OPSTA_LS {

    /// Output Count. Number of bytes written to sequential out pointer.
    pub mod OUT_CT {
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

/// DECO0 Primary DID Status Register
pub mod D0PDIDSR {

    /// DECO Primary DID
    pub mod PRIM_DID {
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

    /// DECO Primary ICID
    pub mod PRIM_ICID {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (11 bits: 0x7ff << 19)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO0 Output DID Status Register
pub mod D0ODIDSR {

    /// DECO Output DID
    pub mod OUT_DID {
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

    /// DECO Output ICID
    pub mod OUT_ICID {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (11 bits: 0x7ff << 19)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO0 Math Register 0_MS
pub mod D0MTH0_MS {

    /// MATH register, most-significant 32 bits.
    pub mod MATH_MS {
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

/// DECO0 Math Register 0_LS
pub mod D0MTH0_LS {

    /// MATH register, least-significant 32 bits.
    pub mod MATH_LS {
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

/// DECO0 Math Register 1_MS
pub mod D0MTH1_MS {
    pub use super::D0MTH0_MS::MATH_MS;
}

/// DECO0 Math Register 1_LS
pub mod D0MTH1_LS {
    pub use super::D0MTH0_LS::MATH_LS;
}

/// DECO0 Math Register 2_MS
pub mod D0MTH2_MS {
    pub use super::D0MTH0_MS::MATH_MS;
}

/// DECO0 Math Register 2_LS
pub mod D0MTH2_LS {
    pub use super::D0MTH0_LS::MATH_LS;
}

/// DECO0 Math Register 3_MS
pub mod D0MTH3_MS {
    pub use super::D0MTH0_MS::MATH_MS;
}

/// DECO0 Math Register 3_LS
pub mod D0MTH3_LS {
    pub use super::D0MTH0_LS::MATH_LS;
}

/// DECO0 Gather Table Register 0 Word 0
pub mod D0GTR0_0 {

    /// most-significant bits of memory address pointed to by table entry
    pub mod ADDRESS_POINTER {
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

/// DECO0 Gather Table Register 0 Word 1
pub mod D0GTR0_1 {

    /// This field holds the least-significant 32 bits of the memory address to which this table entry points
    pub mod ADDRESS_POINTER {
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

/// DECO0 Gather Table Register 0 Word 2
pub mod D0GTR0_2 {

    /// This field specifies how many bytes of data (for Gather Tables) or available space (for Scatter Tables) are located at the address pointed to by the Address Pointer
    pub mod Length {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (30 bits: 0x3fffffff << 0)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Final Bit. If set, this is the last entry of this Scatter/Gather Table.
    pub mod F {
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

            /// 0b0: This is not the last entry of the SGT.
            pub const NOT_LAST: u32 = 0b0;

            /// 0b1: This is the last entry of the SGT.
            pub const LAST: u32 = 0b1;
        }
    }

    /// Extension bit
    pub mod E {
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

            /// 0b0: Address Pointer points to a memory buffer.
            pub const MEM_BUFFER: u32 = 0b0;

            /// 0b1: Address Pointer points to a Scatter/Gather Table Entry.
            pub const SGTE: u32 = 0b1;
        }
    }
}

/// DECO0 Gather Table Register 0 Word 3
pub mod D0GTR0_3 {

    /// Offset (measured in bytes) into memory where significant data is to be found
    pub mod Offset {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO0 Scatter Table Register 0 Word 0
pub mod D0STR0_0 {
    pub use super::D0GTR0_0::ADDRESS_POINTER;
}

/// DECO0 Scatter Table Register 0 Word 1
pub mod D0STR0_1 {
    pub use super::D0GTR0_1::ADDRESS_POINTER;
}

/// DECO0 Scatter Table Register 0 Word 2
pub mod D0STR0_2 {
    pub use super::D0GTR0_2::Length;
    pub use super::D0GTR0_2::E;
    pub use super::D0GTR0_2::F;
}

/// DECO0 Scatter Table Register 0 Word 3
pub mod D0STR0_3 {
    pub use super::D0GTR0_3::Offset;
}

/// DECO0 Descriptor Buffer Word 0
pub mod D0DESB0 {

    /// Descriptor Buffer Word
    pub mod DESBW {
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

/// DECO0 Descriptor Buffer Word 1
pub mod D0DESB1 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 2
pub mod D0DESB2 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 3
pub mod D0DESB3 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 4
pub mod D0DESB4 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 5
pub mod D0DESB5 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 6
pub mod D0DESB6 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 7
pub mod D0DESB7 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 8
pub mod D0DESB8 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 9
pub mod D0DESB9 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 10
pub mod D0DESB10 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 11
pub mod D0DESB11 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 12
pub mod D0DESB12 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 13
pub mod D0DESB13 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 14
pub mod D0DESB14 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 15
pub mod D0DESB15 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 16
pub mod D0DESB16 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 17
pub mod D0DESB17 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 18
pub mod D0DESB18 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 19
pub mod D0DESB19 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 20
pub mod D0DESB20 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 21
pub mod D0DESB21 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 22
pub mod D0DESB22 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 23
pub mod D0DESB23 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 24
pub mod D0DESB24 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 25
pub mod D0DESB25 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 26
pub mod D0DESB26 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 27
pub mod D0DESB27 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 28
pub mod D0DESB28 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 29
pub mod D0DESB29 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 30
pub mod D0DESB30 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 31
pub mod D0DESB31 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 32
pub mod D0DESB32 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 33
pub mod D0DESB33 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 34
pub mod D0DESB34 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 35
pub mod D0DESB35 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 36
pub mod D0DESB36 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 37
pub mod D0DESB37 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 38
pub mod D0DESB38 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 39
pub mod D0DESB39 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 40
pub mod D0DESB40 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 41
pub mod D0DESB41 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 42
pub mod D0DESB42 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 43
pub mod D0DESB43 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 44
pub mod D0DESB44 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 45
pub mod D0DESB45 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 46
pub mod D0DESB46 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 47
pub mod D0DESB47 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 48
pub mod D0DESB48 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 49
pub mod D0DESB49 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 50
pub mod D0DESB50 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 51
pub mod D0DESB51 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 52
pub mod D0DESB52 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 53
pub mod D0DESB53 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 54
pub mod D0DESB54 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 55
pub mod D0DESB55 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 56
pub mod D0DESB56 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 57
pub mod D0DESB57 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 58
pub mod D0DESB58 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 59
pub mod D0DESB59 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 60
pub mod D0DESB60 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 61
pub mod D0DESB61 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 62
pub mod D0DESB62 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Descriptor Buffer Word 63
pub mod D0DESB63 {
    pub use super::D0DESB0::DESBW;
}

/// DECO0 Debug Job Register
pub mod D0DJR {

    /// Job ID
    pub mod ID {
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

    /// Job Source
    pub mod SRC {
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

            /// 0b000: Job Ring 0
            pub const JR0: u32 = 0b000;

            /// 0b001: Job Ring 1
            pub const JR1: u32 = 0b001;

            /// 0b010: Job Ring 2
            pub const JR2: u32 = 0b010;

            /// 0b011: Job Ring 3
            pub const JR3: u32 = 0b011;

            /// 0b100: RTIC
            pub const RTIC: u32 = 0b100;
        }
    }

    /// Job Descriptor DID Select
    pub mod JDDS {
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

            /// 0b0: Non-SEQ DID
            pub const NON_SEQ_DID: u32 = 0b0;

            /// 0b1: SEQ DID
            pub const SEQ_DID: u32 = 0b1;
        }
    }

    /// Allow Make Trusted Descriptor
    pub mod AMTD {
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

            /// 0b0: The Allowed Make Trusted Descriptor bit was NOT set.
            pub const AMTD_NOT_SET: u32 = 0b0;

            /// 0b1: The Allowed Make Trusted Descriptor bit was set.
            pub const AMTD_SET: u32 = 0b1;
        }
    }

    /// Got Shared Descriptor
    pub mod GSD {
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

            /// 0b0: Shared Descriptor was NOT obtained from another DECO.
            pub const DID_NOT_GET_SD: u32 = 0b0;

            /// 0b1: Shared Descriptor was obtained from another DECO.
            pub const GOT_SD: u32 = 0b1;
        }
    }

    /// Double Word Swap. Double word swapping was set.
    pub mod DWS {
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

            /// 0b0: Double Word Swap is NOT set.
            pub const NO_DWS: u32 = 0b0;

            /// 0b1: Double Word Swap is set.
            pub const DWS: u32 = 0b1;
        }
    }

    /// Share From
    pub mod SHR_FROM {
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

    /// Immediate Little Endian
    pub mod ILE {
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

            /// 0b0: No byte-swapping is performed for immediate data transferred to or from the Descriptor Buffer.
            pub const NO_BYTE_SWAP: u32 = 0b0;

            /// 0b1: Byte-swapping is performed for immediate data transferred to or from the Descriptor Buffer.
            pub const BYTE_SWAP: u32 = 0b1;
        }
    }

    /// Four Words
    pub mod FOUR {
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

            /// 0b0: DECO has not been given at least four words of the descriptor.
            pub const NOT_FOUR_WORDS: u32 = 0b0;

            /// 0b1: DECO has been given at least four words of the descriptor.
            pub const FOUR_WORDS: u32 = 0b1;
        }
    }

    /// Whole Descriptor
    pub mod WHL {
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

            /// 0b0: DECO has not been given the whole descriptor.
            pub const NOT_WHOLE_DESC: u32 = 0b0;

            /// 0b1: DECO has been given the whole descriptor.
            pub const WHOLE_DESC: u32 = 0b1;
        }
    }

    /// Single Step Mode
    pub mod SING {
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

            /// 0b0: DECO has not been told to execute the descriptor in single-step mode.
            pub const NOT_SINGLE_STEP_MODE: u32 = 0b0;

            /// 0b1: DECO has been told to execute the descriptor in single-step mode.
            pub const SINGLE_STEP_MODE: u32 = 0b1;
        }
    }

    /// Step
    pub mod STEP {
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

            /// 0b0: DECO has not been told to execute the next command in the descriptor.
            pub const DONT_STEP: u32 = 0b0;

            /// 0b1: DECO has been told to execute the next command in the descriptor.
            pub const STEP: u32 = 0b1;
        }
    }
}

/// DECO0 Debug DECO Register
pub mod D0DDR {

    /// Checking Trusted
    pub mod CT {
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

            /// 0b0: This DECO is NOTcurrently generating the signature of a Trusted Descriptor.
            pub const NOT_CHECKING: u32 = 0b0;

            /// 0b1: This DECO is currently generating the signature of a Trusted Descriptor.
            pub const CHECKING: u32 = 0b1;
        }
    }

    /// Burster Read Busy
    pub mod BRB {
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

            /// 0b0: The READ machine in the Burster is not busy.
            pub const NOT_BUSY: u32 = 0b0;

            /// 0b1: The READ machine in the Burster is busy.
            pub const BUSY: u32 = 0b1;
        }
    }

    /// Burster Write Busy
    pub mod BWB {
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

            /// 0b0: The WRITE machine in the Burster is not busy.
            pub const NOT_BUSY: u32 = 0b0;

            /// 0b1: The WRITE machine in the Burster is busy.
            pub const BUSY: u32 = 0b1;
        }
    }

    /// No Command
    pub mod NC {
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

            /// 0b0: This DECO is currently executing a command.
            pub const CMD_EXEC: u32 = 0b0;

            /// 0b1: This DECO is not currently executing a command.
            pub const NO_CMD_EXEC: u32 = 0b1;
        }
    }

    /// Command Stage Aux
    pub mod CSA {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command Stage
    pub mod CMD_STAGE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command Index
    pub mod CMD_INDEX {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Took Non-local JUMP
    pub mod NLJ {
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

            /// 0b0: The original job descriptor running in this DECO has not caused another job descriptor to be executed.
            pub const ORIGINAL_DESC: u32 = 0b0;

            /// 0b1: The original job descriptor running in this DECO has caused another job descriptor to be executed.
            pub const NON_LOCAL_DECR: u32 = 0b1;
        }
    }

    /// Protocol running. PTCL_RUN=1 indicates that a protocol is running in this DECO.
    pub mod PTCL_RUN {
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

            /// 0b0: No protocol is running in this DECO.
            pub const NOT_RUNNING: u32 = 0b0;

            /// 0b1: A protocol is running in this DECO.
            pub const RUNNING: u32 = 0b1;
        }
    }

    /// PDB Stall State
    pub mod PDB_STALL {
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

    /// PDB Writeback State. Lower two bits of the state machine that tracks the state of PDB writebacks.
    pub mod PDB_WB_ST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECO State. The current state of DECO's main state machine.
    pub mod DECO_STATE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Non-SEQ DID Select. This indicates which type of DID is being used for Non-SEQ commands:
    pub mod NSEQLSEL {
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

            /// 0b01: SEQ DID
            pub const SEQ_DID: u32 = 0b01;

            /// 0b10: Non-SEQ DID
            pub const NONSEQ_DID: u32 = 0b10;

            /// 0b11: Trusted DID
            pub const TRUSTED_DID: u32 = 0b11;
        }
    }

    /// SEQ DID Select. This indicates which type of DID is being used for SEQ commands:
    pub mod SEQLSEL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::NSEQLSEL::RW;
    }

    /// DMA Transaction Count
    pub mod TRCT {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Shared Descriptor
    pub mod SD {
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

            /// 0b0: This DECO has not received a shared descriptor from another DECO.
            pub const NO_SD_RCVD: u32 = 0b0;

            /// 0b1: This DECO has received a shared descriptor from another DECO.
            pub const SD_RCVD: u32 = 0b1;
        }
    }

    /// Valid
    pub mod VALID {
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

            /// 0b0: No descriptor is currently running in this DECO.
            pub const NO_DESC_RUNNING: u32 = 0b0;

            /// 0b1: There is currently a descriptor running in this DECO.
            pub const DESC_RUNNING: u32 = 0b1;
        }
    }
}

/// DECO0 Debug Job Pointer
pub mod D0DJP {

    /// Job Descriptor Pointer.
    pub mod JDPTR {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (36 bits: 0xfffffffff << 0)
        pub const mask: u64 = 0xfffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO0 Debug Shared Pointer
pub mod D0SDP {

    /// Shared Descriptor Pointer.
    pub mod SDPTR {
        /// Offset (0 bits)
        pub const offset: u64 = 0;
        /// Mask (36 bits: 0xfffffffff << 0)
        pub const mask: u64 = 0xfffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DECO0 Debug DID, most-significant half
pub mod D0DDR_MS {
    pub use super::HT0_JQ_CTRL_LS::OUT_DID;
    pub use super::HT0_JQ_CTRL_LS::OUT_ICID;
    pub use super::HT0_JQ_CTRL_LS::PRIM_DID;
    pub use super::HT0_JQ_CTRL_LS::PRIM_ICID;
    pub use super::HT0_JQ_CTRL_LS::PRIM_TZ;
}

/// DECO0 Debug DID, least-significant half
pub mod D0DDR_LS {
    pub use super::D0ODIDSR::OUT_DID;
    pub use super::D0ODIDSR::OUT_ICID;
}

/// Sequence Output Length Register
pub mod SOL0 {

    /// Output Sequence Length
    pub mod SOL {
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

/// Variable Sequence Output Length Register
pub mod VSOL0 {

    /// This value is used in variable-length output data sequences
    pub mod VSOL {
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

/// Sequence Input Length Register
pub mod SIL0 {

    /// This value is used in input data sequences
    pub mod SIL {
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

/// Variable Sequence Input Length Register
pub mod VSIL0 {

    /// This value is used in variable-length input data sequences
    pub mod VSIL {
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

/// Protocol Override Register
pub mod D0POVRD {

    /// DPOVRD can be used as a general purpose math register.
    pub mod DPOVRD {
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

/// Variable Sequence Output Length Register; Upper 32 bits
pub mod UVSOL0 {

    /// This value is used in variable-length output data sequences
    pub mod UVSOL {
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

/// Variable Sequence Input Length Register; Upper 32 bits
pub mod UVSIL0 {

    /// This value is used in variable-length input data sequences
    pub mod UVSIL {
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

/// Job Ring Job_Ring DID Register - most significant half
pub mod JRDID_MS_0 {

    /// Job Ring Owner's DID
    pub mod PRIM_DID {
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

    /// Primary TZ
    pub mod PRIM_TZ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Security Domain Identifier most significant bits
    pub mod SDID_MS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (10 bits: 0x3ff << 5)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TrustZone SecureWorld
    pub mod TZ_OWN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Allow Make Trusted Descriptor
    pub mod AMTD {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock AMTD
    pub mod LAMTD {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Primary ICID
    pub mod PRIM_ICID {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (11 bits: 0x7ff << 19)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USE_OUT
    pub mod USE_OUT {
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

    /// Lock DIDs
    pub mod LDID {
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

/// Job Ring Job_Ring DID Register - least significant half
pub mod JRDID_LS_0 {

    /// Output DID
    pub mod OUT_DID {
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

    /// Job Ring Output ICID
    pub mod OUT_ICID {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (11 bits: 0x7ff << 19)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Job Ring Job_Ring DID Register - most significant half
pub mod JRDID_MS_1 {
    pub use super::JRDID_MS_0::AMTD;
    pub use super::JRDID_MS_0::LAMTD;
    pub use super::JRDID_MS_0::LDID;
    pub use super::JRDID_MS_0::PRIM_DID;
    pub use super::JRDID_MS_0::PRIM_ICID;
    pub use super::JRDID_MS_0::PRIM_TZ;
    pub use super::JRDID_MS_0::SDID_MS;
    pub use super::JRDID_MS_0::TZ_OWN;
    pub use super::JRDID_MS_0::USE_OUT;
}

/// Job Ring Job_Ring DID Register - least significant half
pub mod JRDID_LS_1 {
    pub use super::JRDID_LS_0::OUT_DID;
    pub use super::JRDID_LS_0::OUT_ICID;
}

/// Job Ring Job_Ring DID Register - most significant half
pub mod JRDID_MS_2 {
    pub use super::JRDID_MS_0::AMTD;
    pub use super::JRDID_MS_0::LAMTD;
    pub use super::JRDID_MS_0::LDID;
    pub use super::JRDID_MS_0::PRIM_DID;
    pub use super::JRDID_MS_0::PRIM_ICID;
    pub use super::JRDID_MS_0::PRIM_TZ;
    pub use super::JRDID_MS_0::SDID_MS;
    pub use super::JRDID_MS_0::TZ_OWN;
    pub use super::JRDID_MS_0::USE_OUT;
}

/// Job Ring Job_Ring DID Register - least significant half
pub mod JRDID_LS_2 {
    pub use super::JRDID_LS_0::OUT_DID;
    pub use super::JRDID_LS_0::OUT_ICID;
}

/// Job Ring Job_Ring DID Register - most significant half
pub mod JRDID_MS_3 {
    pub use super::JRDID_MS_0::AMTD;
    pub use super::JRDID_MS_0::LAMTD;
    pub use super::JRDID_MS_0::LDID;
    pub use super::JRDID_MS_0::PRIM_DID;
    pub use super::JRDID_MS_0::PRIM_ICID;
    pub use super::JRDID_MS_0::PRIM_TZ;
    pub use super::JRDID_MS_0::SDID_MS;
    pub use super::JRDID_MS_0::TZ_OWN;
    pub use super::JRDID_MS_0::USE_OUT;
}

/// Job Ring Job_Ring DID Register - least significant half
pub mod JRDID_LS_3 {
    pub use super::JRDID_LS_0::OUT_DID;
    pub use super::JRDID_LS_0::OUT_ICID;
}

/// RTIC DID Register for Block RTIC_hash_block
pub mod RTIC_DID_0 {

    /// RTIC DID
    pub mod RTIC_DID {
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

    /// RTIC_TZ
    pub mod RTIC_TZ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTIC ICID
    pub mod RTIC_ICID {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (11 bits: 0x7ff << 19)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTIC DID Register for Block RTIC_hash_block
pub mod RTIC_DID_1 {
    pub use super::RTIC_DID_0::RTIC_DID;
    pub use super::RTIC_DID_0::RTIC_ICID;
    pub use super::RTIC_DID_0::RTIC_TZ;
}

/// RTIC DID Register for Block RTIC_hash_block
pub mod RTIC_DID_2 {
    pub use super::RTIC_DID_0::RTIC_DID;
    pub use super::RTIC_DID_0::RTIC_ICID;
    pub use super::RTIC_DID_0::RTIC_TZ;
}

/// RTIC DID Register for Block RTIC_hash_block
pub mod RTIC_DID_3 {
    pub use super::RTIC_DID_0::RTIC_DID;
    pub use super::RTIC_DID_0::RTIC_ICID;
    pub use super::RTIC_DID_0::RTIC_TZ;
}

/// Job Ring Job_Ring Secure Memory Virtual Base Address Register
pub mod JRSMVBAR_0 {

    /// Secure Memory Virtual Base Address
    pub mod SMVBA {
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

/// Job Ring Job_Ring Secure Memory Virtual Base Address Register
pub mod JRSMVBAR_1 {
    pub use super::JRSMVBAR_0::SMVBA;
}

/// Job Ring Job_Ring Secure Memory Virtual Base Address Register
pub mod JRSMVBAR_2 {
    pub use super::JRSMVBAR_0::SMVBA;
}

/// Job Ring Job_Ring Secure Memory Virtual Base Address Register
pub mod JRSMVBAR_3 {
    pub use super::JRSMVBAR_0::SMVBA;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 1],

    /// Master Configuration Register
    pub MCFGR: RWRegister<u32>,

    /// Page 0 SDID Register
    pub PAGE0_SDID: RWRegister<u32>,

    /// Security Configuration Register
    pub SCFGR: RWRegister<u32>,

    /// Job Ring Job_Ring DID Register - most significant half
    pub JRDID_MS_0: RWRegister<u32>,

    /// Job Ring Job_Ring DID Register - least significant half
    pub JRDID_LS_0: RWRegister<u32>,

    /// Job Ring Job_Ring DID Register - most significant half
    pub JRDID_MS_1: RWRegister<u32>,

    /// Job Ring Job_Ring DID Register - least significant half
    pub JRDID_LS_1: RWRegister<u32>,

    /// Job Ring Job_Ring DID Register - most significant half
    pub JRDID_MS_2: RWRegister<u32>,

    /// Job Ring Job_Ring DID Register - least significant half
    pub JRDID_LS_2: RWRegister<u32>,

    /// Job Ring Job_Ring DID Register - most significant half
    pub JRDID_MS_3: RWRegister<u32>,

    /// Job Ring Job_Ring DID Register - least significant half
    pub JRDID_LS_3: RWRegister<u32>,

    _reserved2: [u32; 10],

    /// Debug Control Register
    pub DEBUGCTL: RWRegister<u32>,

    /// Job Ring Start Register
    pub JRSTARTR: RWRegister<u32>,

    /// RTIC OWN Register
    pub RTIC_OWN: RWRegister<u32>,

    /// RTIC DID Register for Block RTIC_hash_block
    pub RTIC_DID_0: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// RTIC DID Register for Block RTIC_hash_block
    pub RTIC_DID_1: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// RTIC DID Register for Block RTIC_hash_block
    pub RTIC_DID_2: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// RTIC DID Register for Block RTIC_hash_block
    pub RTIC_DID_3: RWRegister<u32>,

    _reserved6: [u32; 5],

    /// DECO Request Source Register
    pub DECORSR: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// DECO Request Register
    pub DECORR: RWRegister<u32>,

    /// DECO0 DID Register - most significant half
    pub DECO0DID_MS: RWRegister<u32>,

    /// DECO0 DID Register - least significant half
    pub DECO0DID_LS: RWRegister<u32>,

    _reserved8: [u32; 30],

    /// DECO Availability Register
    pub DAR: RWRegister<u32>,

    /// DECO Reset Register
    pub DRR: WORegister<u32>,

    _reserved9: [u32; 23],

    /// Job Ring Job_Ring Secure Memory Virtual Base Address Register
    pub JRSMVBAR_0: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// Job Ring Job_Ring Secure Memory Virtual Base Address Register
    pub JRSMVBAR_1: RWRegister<u32>,

    _reserved11: [u32; 1],

    /// Job Ring Job_Ring Secure Memory Virtual Base Address Register
    pub JRSMVBAR_2: RWRegister<u32>,

    _reserved12: [u32; 1],

    /// Job Ring Job_Ring Secure Memory Virtual Base Address Register
    pub JRSMVBAR_3: RWRegister<u32>,

    _reserved13: [u32; 32],

    /// Peak Bandwidth Smoothing Limit Register
    pub PBSL: RWRegister<u32>,

    _reserved14: [u32; 7],

    /// DMA0_AIDL_MAP_MS
    pub DMA0_AIDL_MAP_MS: RORegister<u32>,

    /// DMA0_AIDL_MAP_LS
    pub DMA0_AIDL_MAP_LS: RORegister<u32>,

    /// DMA0_AIDM_MAP_MS
    pub DMA0_AIDM_MAP_MS: RORegister<u32>,

    /// DMA0_AIDM_MAP_LS
    pub DMA0_AIDM_MAP_LS: RORegister<u32>,

    /// DMA0 AXI ID Enable Register
    pub DMA0_AID_ENB: RORegister<u32>,

    _reserved15: [u32; 3],

    /// DMA0 AXI Read Timing Check Register
    pub DMA0_ARD_TC: RWRegister<u64>,

    _reserved16: [u32; 1],

    /// DMA0 Read Timing Check Latency Register
    pub DMA0_ARD_LAT: RWRegister<u32>,

    /// DMA0 AXI Write Timing Check Register
    pub DMA0_AWR_TC: RWRegister<u64>,

    _reserved17: [u32; 1],

    /// DMA0 Write Timing Check Latency Register
    pub DMA0_AWR_LAT: RWRegister<u32>,

    _reserved18: [u32; 32],

    /// Manufacturing Protection Private Key Register
    pub MPPKR0: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR1: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR2: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR3: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR4: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR5: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR6: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR7: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR8: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR9: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR10: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR11: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR12: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR13: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR14: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR15: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR16: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR17: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR18: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR19: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR20: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR21: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR22: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR23: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR24: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR25: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR26: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR27: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR28: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR29: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR30: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR31: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR32: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR33: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR34: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR35: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR36: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR37: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR38: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR39: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR40: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR41: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR42: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR43: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR44: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR45: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR46: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR47: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR48: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR49: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR50: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR51: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR52: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR53: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR54: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR55: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR56: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR57: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR58: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR59: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR60: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR61: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR62: RWRegister<u8>,

    /// Manufacturing Protection Private Key Register
    pub MPPKR63: RWRegister<u8>,

    _reserved19: [u32; 16],

    /// Manufacturing Protection Message Register
    pub MPMR0: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR1: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR2: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR3: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR4: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR5: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR6: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR7: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR8: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR9: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR10: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR11: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR12: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR13: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR14: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR15: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR16: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR17: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR18: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR19: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR20: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR21: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR22: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR23: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR24: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR25: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR26: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR27: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR28: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR29: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR30: RWRegister<u8>,

    /// Manufacturing Protection Message Register
    pub MPMR31: RWRegister<u8>,

    _reserved20: [u32; 8],

    /// Manufacturing Protection Test Register
    pub MPTESTR0: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR1: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR2: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR3: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR4: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR5: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR6: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR7: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR8: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR9: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR10: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR11: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR12: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR13: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR14: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR15: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR16: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR17: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR18: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR19: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR20: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR21: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR22: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR23: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR24: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR25: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR26: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR27: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR28: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR29: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR30: RORegister<u8>,

    /// Manufacturing Protection Test Register
    pub MPTESTR31: RORegister<u8>,

    _reserved21: [u32; 6],

    /// Manufacturing Protection ECC Register
    pub MPECC: RORegister<u32>,

    _reserved22: [u32; 1],

    /// Job Descriptor Key Encryption Key Register
    pub JDKEKR0: RWRegister<u32>,

    /// Job Descriptor Key Encryption Key Register
    pub JDKEKR1: RWRegister<u32>,

    /// Job Descriptor Key Encryption Key Register
    pub JDKEKR2: RWRegister<u32>,

    /// Job Descriptor Key Encryption Key Register
    pub JDKEKR3: RWRegister<u32>,

    /// Job Descriptor Key Encryption Key Register
    pub JDKEKR4: RWRegister<u32>,

    /// Job Descriptor Key Encryption Key Register
    pub JDKEKR5: RWRegister<u32>,

    /// Job Descriptor Key Encryption Key Register
    pub JDKEKR6: RWRegister<u32>,

    /// Job Descriptor Key Encryption Key Register
    pub JDKEKR7: RWRegister<u32>,

    /// Trusted Descriptor Key Encryption Key Register
    pub TDKEKR0: RWRegister<u32>,

    /// Trusted Descriptor Key Encryption Key Register
    pub TDKEKR1: RWRegister<u32>,

    /// Trusted Descriptor Key Encryption Key Register
    pub TDKEKR2: RWRegister<u32>,

    /// Trusted Descriptor Key Encryption Key Register
    pub TDKEKR3: RWRegister<u32>,

    /// Trusted Descriptor Key Encryption Key Register
    pub TDKEKR4: RWRegister<u32>,

    /// Trusted Descriptor Key Encryption Key Register
    pub TDKEKR5: RWRegister<u32>,

    /// Trusted Descriptor Key Encryption Key Register
    pub TDKEKR6: RWRegister<u32>,

    /// Trusted Descriptor Key Encryption Key Register
    pub TDKEKR7: RWRegister<u32>,

    /// Trusted Descriptor Signing Key Register
    pub TDSKR0: RWRegister<u32>,

    /// Trusted Descriptor Signing Key Register
    pub TDSKR1: RWRegister<u32>,

    /// Trusted Descriptor Signing Key Register
    pub TDSKR2: RWRegister<u32>,

    /// Trusted Descriptor Signing Key Register
    pub TDSKR3: RWRegister<u32>,

    /// Trusted Descriptor Signing Key Register
    pub TDSKR4: RWRegister<u32>,

    /// Trusted Descriptor Signing Key Register
    pub TDSKR5: RWRegister<u32>,

    /// Trusted Descriptor Signing Key Register
    pub TDSKR6: RWRegister<u32>,

    /// Trusted Descriptor Signing Key Register
    pub TDSKR7: RWRegister<u32>,

    _reserved23: [u32; 32],

    /// Secure Key Nonce Register
    pub SKNR: RWRegister<u64>,

    _reserved24: [u32; 9],

    /// DMA Status Register
    pub DMA_STA: RORegister<u32>,

    /// DMA_X_AID_7_4_MAP
    pub DMA_X_AID_7_4_MAP: RORegister<u32>,

    /// DMA_X_AID_3_0_MAP
    pub DMA_X_AID_3_0_MAP: RORegister<u32>,

    /// DMA_X_AID_15_12_MAP
    pub DMA_X_AID_15_12_MAP: RORegister<u32>,

    /// DMA_X_AID_11_8_MAP
    pub DMA_X_AID_11_8_MAP: RORegister<u32>,

    _reserved25: [u32; 1],

    /// DMA_X AXI ID Map Enable Register
    pub DMA_X_AID_15_0_EN: RORegister<u32>,

    _reserved26: [u32; 2],

    /// DMA_X AXI Read Timing Check Control Register
    pub DMA_X_ARTC_CTL: RWRegister<u32>,

    /// DMA_X AXI Read Timing Check Late Count Register
    pub DMA_X_ARTC_LC: RWRegister<u32>,

    /// DMA_X AXI Read Timing Check Sample Count Register
    pub DMA_X_ARTC_SC: RWRegister<u32>,

    /// DMA_X Read Timing Check Latency Register
    pub DMA_X_ARTC_LAT: RWRegister<u32>,

    /// DMA_X AXI Write Timing Check Control Register
    pub DMA_X_AWTC_CTL: RWRegister<u32>,

    /// DMA_X AXI Write Timing Check Late Count Register
    pub DMA_X_AWTC_LC: RWRegister<u32>,

    /// DMA_X AXI Write Timing Check Sample Count Register
    pub DMA_X_AWTC_SC: RWRegister<u32>,

    /// DMA_X Write Timing Check Latency Register
    pub DMA_X_AWTC_LAT: RWRegister<u32>,

    _reserved27: [u32; 44],

    /// RNG TRNG Miscellaneous Control Register
    pub RTMCTL: RWRegister<u32>,

    /// RNG TRNG Statistical Check Miscellaneous Register
    pub RTSCMISC: RWRegister<u32>,

    /// RNG TRNG Poker Range Register
    pub RTPKRRNG: RWRegister<u32>,

    /// RTPKRMAX and RTPKRSQ
    /// RTPKRMAX: RNG TRNG Poker Maximum Limit Register
    /// RTPKRSQ: RNG TRNG Poker Square Calculation Result Register
    pub RTPKR: RWRegister<u32>,

    /// RNG TRNG Seed Control Register
    pub RTSDCTL: RWRegister<u32>,

    /// RTSBLIM and RTTOTSAM
    /// RTSBLIM: RNG TRNG Sparse Bit Limit Register
    /// RTTOTSAM: RNG TRNG Total Samples Register
    pub RT: RWRegister<u32>,

    /// RNG TRNG Frequency Count Minimum Limit Register
    pub RTFRQMIN: RWRegister<u32>,

    /// RTFRQCNT and RTFRQMAX
    /// RTFRQCNT: RNG TRNG Frequency Count Register
    /// RTFRQMAX: RNG TRNG Frequency Count Maximum Limit Register
    pub RTFRQ: RWRegister<u32>,

    /// RTSCMC and RTSCML
    /// RTSCMC: RNG TRNG Statistical Check Monobit Count Register
    /// RTSCML: RNG TRNG Statistical Check Monobit Limit Register
    pub RTSCM: RWRegister<u32>,

    /// RTSCR1C and RTSCR1L
    /// RTSCR1C: RNG TRNG Statistical Check Run Length 1 Count Register
    /// RTSCR1L: RNG TRNG Statistical Check Run Length 1 Limit Register
    pub RTSCR1: RWRegister<u32>,

    /// RTSCR2C and RTSCR2L
    /// RTSCR2C: RNG TRNG Statistical Check Run Length 2 Count Register
    /// RTSCR2L: RNG TRNG Statistical Check Run Length 2 Limit Register
    pub RTSCR2: RWRegister<u32>,

    /// RTSCR3C and RTSCR3L
    /// RTSCR3C: RNG TRNG Statistical Check Run Length 3 Count Register
    /// RTSCR3L: RNG TRNG Statistical Check Run Length 3 Limit Register
    pub RTSCR3: RWRegister<u32>,

    /// RTSCR4C and RTSCR4L
    /// RTSCR4C: RNG TRNG Statistical Check Run Length 4 Count Register
    /// RTSCR4L: RNG TRNG Statistical Check Run Length 4 Limit Register
    pub RTSCR4: RWRegister<u32>,

    /// RTSCR5C and RTSCR5L
    /// RTSCR5C: RNG TRNG Statistical Check Run Length 5 Count Register
    /// RTSCR5L: RNG TRNG Statistical Check Run Length 5 Limit Register
    pub RTSCR5: RWRegister<u32>,

    /// RTSCR6PC and RTSCR6PL
    /// RTSCR6PC: RNG TRNG Statistical Check Run Length 6+ Count Register
    /// RTSCR6PL: RNG TRNG Statistical Check Run Length 6+ Limit Register
    pub RTSCR6P: RWRegister<u32>,

    /// RNG TRNG Status Register
    pub RTSTATUS: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT0: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT1: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT2: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT3: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT4: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT5: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT6: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT7: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT8: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT9: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT10: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT11: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT12: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT13: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT14: RORegister<u32>,

    /// RNG TRNG Entropy Read Register
    pub RTENT15: RORegister<u32>,

    /// RNG TRNG Statistical Check Poker Count 1 and 0 Register
    pub RTPKRCNT10: RORegister<u32>,

    /// RNG TRNG Statistical Check Poker Count 3 and 2 Register
    pub RTPKRCNT32: RORegister<u32>,

    /// RNG TRNG Statistical Check Poker Count 5 and 4 Register
    pub RTPKRCNT54: RORegister<u32>,

    /// RNG TRNG Statistical Check Poker Count 7 and 6 Register
    pub RTPKRCNT76: RORegister<u32>,

    /// RNG TRNG Statistical Check Poker Count 9 and 8 Register
    pub RTPKRCNT98: RORegister<u32>,

    /// RNG TRNG Statistical Check Poker Count B and A Register
    pub RTPKRCNTBA: RORegister<u32>,

    /// RNG TRNG Statistical Check Poker Count D and C Register
    pub RTPKRCNTDC: RORegister<u32>,

    /// RNG TRNG Statistical Check Poker Count F and E Register
    pub RTPKRCNTFE: RORegister<u32>,

    _reserved28: [u32; 8],

    /// RNG DRNG Status Register
    pub RDSTA: RORegister<u32>,

    _reserved29: [u32; 3],

    /// RNG DRNG State Handle 0 Reseed Interval Register
    pub RDINT0: RORegister<u32>,

    /// RNG DRNG State Handle 1 Reseed Interval Register
    pub RDINT1: RORegister<u32>,

    _reserved30: [u32; 2],

    /// RNG DRNG Hash Control Register
    pub RDHCNTL: RWRegister<u32>,

    /// RNG DRNG Hash Digest Register
    pub RDHDIG: RORegister<u32>,

    /// RNG DRNG Hash Buffer Register
    pub RDHBUF: WORegister<u32>,

    _reserved31: [u32; 197],

    /// Partition 0 SDID register
    pub P0SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P0SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG1_PG0: RWRegister<u32>,

    /// Partition 1 SDID register
    pub P1SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P1SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG1_PG0: RWRegister<u32>,

    /// Partition 2 SDID register
    pub P2SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P2SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG1_PG0: RWRegister<u32>,

    /// Partition 3 SDID register
    pub P3SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P3SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG1_PG0: RWRegister<u32>,

    /// Partition 4 SDID register
    pub P4SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P4SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG1_PG0: RWRegister<u32>,

    /// Partition 5 SDID register
    pub P5SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P5SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG1_PG0: RWRegister<u32>,

    /// Partition 6 SDID register
    pub P6SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P6SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG1_PG0: RWRegister<u32>,

    /// Partition 7 SDID register
    pub P7SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P7SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG1_PG0: RWRegister<u32>,

    /// Partition 8 SDID register
    pub P8SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P8SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG1_PG0: RWRegister<u32>,

    /// Partition 9 SDID register
    pub P9SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P9SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG1_PG0: RWRegister<u32>,

    /// Partition 10 SDID register
    pub P10SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P10SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG1_PG0: RWRegister<u32>,

    /// Partition 11 SDID register
    pub P11SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P11SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG1_PG0: RWRegister<u32>,

    /// Partition 12 SDID register
    pub P12SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P12SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG1_PG0: RWRegister<u32>,

    /// Partition 13 SDID register
    pub P13SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P13SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG1_PG0: RWRegister<u32>,

    /// Partition 14 SDID register
    pub P14SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P14SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG1_PG0: RWRegister<u32>,

    /// Partition 15 SDID register
    pub P15SDID_PG0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P15SMAPR_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG2_PG0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG1_PG0: RWRegister<u32>,

    /// Recoverable Error Interrupt Status
    pub REIS: RWRegister<u32>,

    /// Recoverable Error Interrupt Enable
    pub REIE: RWRegister<u32>,

    /// Recoverable Error Interrupt Force
    pub REIF: RORegister<u32>,

    /// Recoverable Error Interrupt Halt
    pub REIH: RWRegister<u32>,

    _reserved32: [u32; 48],

    /// Secure Memory Write Protect Job Ring Register
    pub SMWPJRR0: RWRegister<u32>,

    /// Secure Memory Write Protect Job Ring Register
    pub SMWPJRR1: RWRegister<u32>,

    /// Secure Memory Write Protect Job Ring Register
    pub SMWPJRR2: RWRegister<u32>,

    /// Secure Memory Write Protect Job Ring Register
    pub SMWPJRR3: RWRegister<u32>,

    _reserved33: [u32; 1],

    /// Secure Memory Command Register
    pub SMCR_PG0: WORegister<u32>,

    _reserved34: [u32; 1],

    /// Secure Memory Command Status Register
    pub SMCSR_PG0: RORegister<u32>,

    _reserved35: [u32; 4],

    /// Holding Tank 0 Job Descriptor Address
    pub HT0_JD_ADDR: RORegister<u64>,

    /// Holding Tank 0 Shared Descriptor Address
    pub HT0_SD_ADDR: RORegister<u64>,

    /// Holding Tank 0 Job Queue Control, most-significant half
    pub HT0_JQ_CTRL_MS: RORegister<u32>,

    /// Holding Tank 0 Job Queue Control, least-significant half
    pub HT0_JQ_CTRL_LS: RORegister<u32>,

    _reserved36: [u32; 1],

    /// Holding Tank Status
    pub HT0_STATUS: RORegister<u32>,

    _reserved37: [u32; 1],

    /// Job Queue Debug Select Register
    pub JQ_DEBUG_SEL: RWRegister<u32>,

    _reserved38: [u32; 101],

    /// Job Ring Job IDs in Use Register, least-significant half
    pub JRJIDU_LS: RORegister<u32>,

    /// Job Ring Job-Done Job ID FIFO BC
    pub JRJDJIFBC: RORegister<u32>,

    /// Job Ring Job-Done Job ID FIFO
    pub JRJDJIF: RORegister<u32>,

    _reserved39: [u32; 7],

    /// Job Ring Job-Done Source 1
    pub JRJDS1: RORegister<u32>,

    _reserved40: [u32; 6],

    /// Job Ring Job-Done Descriptor Address 0 Register
    pub JRJDDA: RORegister<u64>,

    _reserved41: [u32; 102],

    /// CHA Revision Number Register, most-significant half
    pub CRNR_MS: RORegister<u32>,

    /// CHA Revision Number Register, least-significant half
    pub CRNR_LS: RORegister<u32>,

    /// Compile Time Parameters Register, most-significant half
    pub CTPR_MS: RORegister<u32>,

    /// Compile Time Parameters Register, least-significant half
    pub CTPR_LS: RORegister<u32>,

    _reserved42: [u32; 1],

    /// Secure Memory Status Register
    pub SMSTA: RORegister<u32>,

    _reserved43: [u32; 1],

    /// Secure Memory Partition Owners Register
    pub SMPO: RORegister<u32>,

    /// Fault Address Register
    pub FAR: RORegister<u64>,

    /// Fault Address DID Register
    pub FADID: RORegister<u32>,

    /// Fault Address Detail Register
    pub FADR: RORegister<u32>,

    _reserved44: [u32; 1],

    /// CAAM Status Register
    pub CSTA: RORegister<u32>,

    /// Secure Memory Version ID Register, most-significant half
    pub SMVID_MS: RORegister<u32>,

    /// Secure Memory Version ID Register, least-significant half
    pub SMVID_LS: RORegister<u32>,

    /// RTIC Version ID Register
    pub RVID: RORegister<u32>,

    /// CHA Cluster Block Version ID Register
    pub CCBVID: RORegister<u32>,

    /// CHA Version ID Register, most-significant half
    pub CHAVID_MS: RORegister<u32>,

    /// CHA Version ID Register, least-significant half
    pub CHAVID_LS: RORegister<u32>,

    /// CHA Number Register, most-significant half
    pub CHANUM_MS: RORegister<u32>,

    /// CHA Number Register, least-significant half
    pub CHANUM_LS: RORegister<u32>,

    /// CAAM Version ID Register, most-significant half
    pub CAAMVID_MS: RORegister<u32>,

    /// CAAM Version ID Register, least-significant half
    pub CAAMVID_LS: RORegister<u32>,

    _reserved45: [u32; 15360],

    /// Input Ring Base Address Register for Job Ring 0
    pub IRBAR_JR0: RWRegister<u64>,

    _reserved46: [u32; 1],

    /// Input Ring Size Register for Job Ring 0
    pub IRSR_JR0: RWRegister<u32>,

    _reserved47: [u32; 1],

    /// Input Ring Slots Available Register for Job Ring 0
    pub IRSAR_JR0: RWRegister<u32>,

    _reserved48: [u32; 1],

    /// Input Ring Jobs Added Register for Job Ring0
    pub IRJAR_JR0: RWRegister<u32>,

    /// Output Ring Base Address Register for Job Ring 0
    pub ORBAR_JR0: RWRegister<u64>,

    _reserved49: [u32; 1],

    /// Output Ring Size Register for Job Ring 0
    pub ORSR_JR0: RWRegister<u32>,

    _reserved50: [u32; 1],

    /// Output Ring Jobs Removed Register for Job Ring 0
    pub ORJRR_JR0: RWRegister<u32>,

    _reserved51: [u32; 1],

    /// Output Ring Slots Full Register for Job Ring 0
    pub ORSFR_JR0: RWRegister<u32>,

    _reserved52: [u32; 1],

    /// Job Ring Output Status Register for Job Ring 0
    pub JRSTAR_JR0: RORegister<u32>,

    _reserved53: [u32; 1],

    /// Job Ring Interrupt Status Register for Job Ring 0
    pub JRINTR_JR0: RWRegister<u32>,

    /// Job Ring Configuration Register for Job Ring 0, most-significant half
    pub JRCFGR_JR0_MS: RWRegister<u32>,

    /// Job Ring Configuration Register for Job Ring 0, least-significant half
    pub JRCFGR_JR0_LS: RWRegister<u32>,

    _reserved54: [u32; 1],

    /// Input Ring Read Index Register for Job Ring 0
    pub IRRIR_JR0: RWRegister<u32>,

    _reserved55: [u32; 1],

    /// Output Ring Write Index Register for Job Ring 0
    pub ORWIR_JR0: RWRegister<u32>,

    _reserved56: [u32; 1],

    /// Job Ring Command Register for Job Ring 0
    pub JRCR_JR0: WORegister<u32>,

    _reserved57: [u32; 421],

    /// Job Ring 0 Address-Array Valid Register
    pub JR0AAV: RORegister<u32>,

    _reserved58: [u32; 62],

    /// Job Ring 0 Address-Array Address 0 Register
    pub JR0AAA0: RORegister<u64>,

    /// Job Ring 0 Address-Array Address 1 Register
    pub JR0AAA1: RORegister<u64>,

    /// Job Ring 0 Address-Array Address 2 Register
    pub JR0AAA2: RORegister<u64>,

    /// Job Ring 0 Address-Array Address 3 Register
    pub JR0AAA3: RORegister<u64>,

    _reserved59: [u32; 120],

    /// Partition 0 SDID register
    pub P0SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P0SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG1_JR0: RWRegister<u32>,

    /// Partition 1 SDID register
    pub P1SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P1SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG1_JR0: RWRegister<u32>,

    /// Partition 2 SDID register
    pub P2SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P2SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG1_JR0: RWRegister<u32>,

    /// Partition 3 SDID register
    pub P3SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P3SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG1_JR0: RWRegister<u32>,

    /// Partition 4 SDID register
    pub P4SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P4SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG1_JR0: RWRegister<u32>,

    /// Partition 5 SDID register
    pub P5SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P5SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG1_JR0: RWRegister<u32>,

    /// Partition 6 SDID register
    pub P6SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P6SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG1_JR0: RWRegister<u32>,

    /// Partition 7 SDID register
    pub P7SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P7SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG1_JR0: RWRegister<u32>,

    /// Partition 8 SDID register
    pub P8SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P8SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG1_JR0: RWRegister<u32>,

    /// Partition 9 SDID register
    pub P9SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P9SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG1_JR0: RWRegister<u32>,

    /// Partition 10 SDID register
    pub P10SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P10SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG1_JR0: RWRegister<u32>,

    /// Partition 11 SDID register
    pub P11SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P11SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG1_JR0: RWRegister<u32>,

    /// Partition 12 SDID register
    pub P12SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P12SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG1_JR0: RWRegister<u32>,

    /// Partition 13 SDID register
    pub P13SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P13SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG1_JR0: RWRegister<u32>,

    /// Partition 14 SDID register
    pub P14SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P14SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG1_JR0: RWRegister<u32>,

    /// Partition 15 SDID register
    pub P15SDID_JR0: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P15SMAPR_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG2_JR0: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG1_JR0: RWRegister<u32>,

    _reserved60: [u32; 57],

    /// Secure Memory Command Register
    pub SMCR_JR0: WORegister<u32>,

    _reserved61: [u32; 1],

    /// Secure Memory Command Status Register
    pub SMCSR_JR0: RORegister<u32>,

    _reserved62: [u32; 132],

    /// Recoverable Error Interrupt Record 0 for Job Ring 0
    pub REIR0JR0: RORegister<u32>,

    _reserved63: [u32; 1],

    /// Recoverable Error Interrupt Record 2 for Job Ring 0
    pub REIR2JR0: RORegister<u64>,

    /// Recoverable Error Interrupt Record 4 for Job Ring 0
    pub REIR4JR0: RORegister<u32>,

    /// Recoverable Error Interrupt Record 5 for Job Ring 0
    pub REIR5JR0: RORegister<u32>,

    _reserved64: [u32; 15482],

    /// Input Ring Base Address Register for Job Ring 1
    pub IRBAR_JR1: RWRegister<u64>,

    _reserved65: [u32; 1],

    /// Input Ring Size Register for Job Ring 1
    pub IRSR_JR1: RWRegister<u32>,

    _reserved66: [u32; 1],

    /// Input Ring Slots Available Register for Job Ring 1
    pub IRSAR_JR1: RWRegister<u32>,

    _reserved67: [u32; 1],

    /// Input Ring Jobs Added Register for Job Ring1
    pub IRJAR_JR1: RWRegister<u32>,

    /// Output Ring Base Address Register for Job Ring 1
    pub ORBAR_JR1: RWRegister<u64>,

    _reserved68: [u32; 1],

    /// Output Ring Size Register for Job Ring 1
    pub ORSR_JR1: RWRegister<u32>,

    _reserved69: [u32; 1],

    /// Output Ring Jobs Removed Register for Job Ring 1
    pub ORJRR_JR1: RWRegister<u32>,

    _reserved70: [u32; 1],

    /// Output Ring Slots Full Register for Job Ring 1
    pub ORSFR_JR1: RWRegister<u32>,

    _reserved71: [u32; 1],

    /// Job Ring Output Status Register for Job Ring 1
    pub JRSTAR_JR1: RORegister<u32>,

    _reserved72: [u32; 1],

    /// Job Ring Interrupt Status Register for Job Ring 1
    pub JRINTR_JR1: RWRegister<u32>,

    /// Job Ring Configuration Register for Job Ring 1, most-significant half
    pub JRCFGR_JR1_MS: RWRegister<u32>,

    /// Job Ring Configuration Register for Job Ring 1, least-significant half
    pub JRCFGR_JR1_LS: RWRegister<u32>,

    _reserved73: [u32; 1],

    /// Input Ring Read Index Register for Job Ring 1
    pub IRRIR_JR1: RWRegister<u32>,

    _reserved74: [u32; 1],

    /// Output Ring Write Index Register for Job Ring 1
    pub ORWIR_JR1: RWRegister<u32>,

    _reserved75: [u32; 1],

    /// Job Ring Command Register for Job Ring 1
    pub JRCR_JR1: WORegister<u32>,

    _reserved76: [u32; 421],

    /// Job Ring 1 Address-Array Valid Register
    pub JR1AAV: RORegister<u32>,

    _reserved77: [u32; 62],

    /// Job Ring 1 Address-Array Address 0 Register
    pub JR1AAA0: RORegister<u64>,

    /// Job Ring 1 Address-Array Address 1 Register
    pub JR1AAA1: RORegister<u64>,

    /// Job Ring 1 Address-Array Address 2 Register
    pub JR1AAA2: RORegister<u64>,

    /// Job Ring 1 Address-Array Address 3 Register
    pub JR1AAA3: RORegister<u64>,

    _reserved78: [u32; 120],

    /// Partition 0 SDID register
    pub P0SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P0SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG1_JR1: RWRegister<u32>,

    /// Partition 1 SDID register
    pub P1SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P1SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG1_JR1: RWRegister<u32>,

    /// Partition 2 SDID register
    pub P2SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P2SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG1_JR1: RWRegister<u32>,

    /// Partition 3 SDID register
    pub P3SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P3SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG1_JR1: RWRegister<u32>,

    /// Partition 4 SDID register
    pub P4SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P4SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG1_JR1: RWRegister<u32>,

    /// Partition 5 SDID register
    pub P5SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P5SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG1_JR1: RWRegister<u32>,

    /// Partition 6 SDID register
    pub P6SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P6SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG1_JR1: RWRegister<u32>,

    /// Partition 7 SDID register
    pub P7SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P7SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG1_JR1: RWRegister<u32>,

    /// Partition 8 SDID register
    pub P8SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P8SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG1_JR1: RWRegister<u32>,

    /// Partition 9 SDID register
    pub P9SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P9SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG1_JR1: RWRegister<u32>,

    /// Partition 10 SDID register
    pub P10SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P10SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG1_JR1: RWRegister<u32>,

    /// Partition 11 SDID register
    pub P11SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P11SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG1_JR1: RWRegister<u32>,

    /// Partition 12 SDID register
    pub P12SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P12SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG1_JR1: RWRegister<u32>,

    /// Partition 13 SDID register
    pub P13SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P13SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG1_JR1: RWRegister<u32>,

    /// Partition 14 SDID register
    pub P14SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P14SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG1_JR1: RWRegister<u32>,

    /// Partition 15 SDID register
    pub P15SDID_JR1: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P15SMAPR_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG2_JR1: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG1_JR1: RWRegister<u32>,

    _reserved79: [u32; 57],

    /// Secure Memory Command Register
    pub SMCR_JR1: WORegister<u32>,

    _reserved80: [u32; 1],

    /// Secure Memory Command Status Register
    pub SMCSR_JR1: RORegister<u32>,

    _reserved81: [u32; 132],

    /// Recoverable Error Interrupt Record 0 for Job Ring 1
    pub REIR0JR1: RORegister<u32>,

    _reserved82: [u32; 1],

    /// Recoverable Error Interrupt Record 2 for Job Ring 1
    pub REIR2JR1: RORegister<u64>,

    /// Recoverable Error Interrupt Record 4 for Job Ring 1
    pub REIR4JR1: RORegister<u32>,

    /// Recoverable Error Interrupt Record 5 for Job Ring 1
    pub REIR5JR1: RORegister<u32>,

    _reserved83: [u32; 15482],

    /// Input Ring Base Address Register for Job Ring 2
    pub IRBAR_JR2: RWRegister<u64>,

    _reserved84: [u32; 1],

    /// Input Ring Size Register for Job Ring 2
    pub IRSR_JR2: RWRegister<u32>,

    _reserved85: [u32; 1],

    /// Input Ring Slots Available Register for Job Ring 2
    pub IRSAR_JR2: RWRegister<u32>,

    _reserved86: [u32; 1],

    /// Input Ring Jobs Added Register for Job Ring2
    pub IRJAR_JR2: RWRegister<u32>,

    /// Output Ring Base Address Register for Job Ring 2
    pub ORBAR_JR2: RWRegister<u64>,

    _reserved87: [u32; 1],

    /// Output Ring Size Register for Job Ring 2
    pub ORSR_JR2: RWRegister<u32>,

    _reserved88: [u32; 1],

    /// Output Ring Jobs Removed Register for Job Ring 2
    pub ORJRR_JR2: RWRegister<u32>,

    _reserved89: [u32; 1],

    /// Output Ring Slots Full Register for Job Ring 2
    pub ORSFR_JR2: RWRegister<u32>,

    _reserved90: [u32; 1],

    /// Job Ring Output Status Register for Job Ring 2
    pub JRSTAR_JR2: RORegister<u32>,

    _reserved91: [u32; 1],

    /// Job Ring Interrupt Status Register for Job Ring 2
    pub JRINTR_JR2: RWRegister<u32>,

    /// Job Ring Configuration Register for Job Ring 2, most-significant half
    pub JRCFGR_JR2_MS: RWRegister<u32>,

    /// Job Ring Configuration Register for Job Ring 2, least-significant half
    pub JRCFGR_JR2_LS: RWRegister<u32>,

    _reserved92: [u32; 1],

    /// Input Ring Read Index Register for Job Ring 2
    pub IRRIR_JR2: RWRegister<u32>,

    _reserved93: [u32; 1],

    /// Output Ring Write Index Register for Job Ring 2
    pub ORWIR_JR2: RWRegister<u32>,

    _reserved94: [u32; 1],

    /// Job Ring Command Register for Job Ring 2
    pub JRCR_JR2: WORegister<u32>,

    _reserved95: [u32; 421],

    /// Job Ring 2 Address-Array Valid Register
    pub JR2AAV: RORegister<u32>,

    _reserved96: [u32; 62],

    /// Job Ring 2 Address-Array Address 0 Register
    pub JR2AAA0: RORegister<u64>,

    /// Job Ring 2 Address-Array Address 1 Register
    pub JR2AAA1: RORegister<u64>,

    /// Job Ring 2 Address-Array Address 2 Register
    pub JR2AAA2: RORegister<u64>,

    /// Job Ring 2 Address-Array Address 3 Register
    pub JR2AAA3: RORegister<u64>,

    _reserved97: [u32; 120],

    /// Partition 0 SDID register
    pub P0SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P0SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG1_JR2: RWRegister<u32>,

    /// Partition 1 SDID register
    pub P1SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P1SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG1_JR2: RWRegister<u32>,

    /// Partition 2 SDID register
    pub P2SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P2SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG1_JR2: RWRegister<u32>,

    /// Partition 3 SDID register
    pub P3SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P3SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG1_JR2: RWRegister<u32>,

    /// Partition 4 SDID register
    pub P4SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P4SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG1_JR2: RWRegister<u32>,

    /// Partition 5 SDID register
    pub P5SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P5SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG1_JR2: RWRegister<u32>,

    /// Partition 6 SDID register
    pub P6SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P6SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG1_JR2: RWRegister<u32>,

    /// Partition 7 SDID register
    pub P7SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P7SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG1_JR2: RWRegister<u32>,

    /// Partition 8 SDID register
    pub P8SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P8SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG1_JR2: RWRegister<u32>,

    /// Partition 9 SDID register
    pub P9SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P9SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG1_JR2: RWRegister<u32>,

    /// Partition 10 SDID register
    pub P10SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P10SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG1_JR2: RWRegister<u32>,

    /// Partition 11 SDID register
    pub P11SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P11SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG1_JR2: RWRegister<u32>,

    /// Partition 12 SDID register
    pub P12SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P12SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG1_JR2: RWRegister<u32>,

    /// Partition 13 SDID register
    pub P13SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P13SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG1_JR2: RWRegister<u32>,

    /// Partition 14 SDID register
    pub P14SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P14SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG1_JR2: RWRegister<u32>,

    /// Partition 15 SDID register
    pub P15SDID_JR2: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P15SMAPR_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG2_JR2: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG1_JR2: RWRegister<u32>,

    _reserved98: [u32; 57],

    /// Secure Memory Command Register
    pub SMCR_JR2: WORegister<u32>,

    _reserved99: [u32; 1],

    /// Secure Memory Command Status Register
    pub SMCSR_JR2: RORegister<u32>,

    _reserved100: [u32; 132],

    /// Recoverable Error Interrupt Record 0 for Job Ring 2
    pub REIR0JR2: RORegister<u32>,

    _reserved101: [u32; 1],

    /// Recoverable Error Interrupt Record 2 for Job Ring 2
    pub REIR2JR2: RORegister<u64>,

    /// Recoverable Error Interrupt Record 4 for Job Ring 2
    pub REIR4JR2: RORegister<u32>,

    /// Recoverable Error Interrupt Record 5 for Job Ring 2
    pub REIR5JR2: RORegister<u32>,

    _reserved102: [u32; 15482],

    /// Input Ring Base Address Register for Job Ring 3
    pub IRBAR_JR3: RWRegister<u64>,

    _reserved103: [u32; 1],

    /// Input Ring Size Register for Job Ring 3
    pub IRSR_JR3: RWRegister<u32>,

    _reserved104: [u32; 1],

    /// Input Ring Slots Available Register for Job Ring 3
    pub IRSAR_JR3: RWRegister<u32>,

    _reserved105: [u32; 1],

    /// Input Ring Jobs Added Register for Job Ring3
    pub IRJAR_JR3: RWRegister<u32>,

    /// Output Ring Base Address Register for Job Ring 3
    pub ORBAR_JR3: RWRegister<u64>,

    _reserved106: [u32; 1],

    /// Output Ring Size Register for Job Ring 3
    pub ORSR_JR3: RWRegister<u32>,

    _reserved107: [u32; 1],

    /// Output Ring Jobs Removed Register for Job Ring 3
    pub ORJRR_JR3: RWRegister<u32>,

    _reserved108: [u32; 1],

    /// Output Ring Slots Full Register for Job Ring 3
    pub ORSFR_JR3: RWRegister<u32>,

    _reserved109: [u32; 1],

    /// Job Ring Output Status Register for Job Ring 3
    pub JRSTAR_JR3: RORegister<u32>,

    _reserved110: [u32; 1],

    /// Job Ring Interrupt Status Register for Job Ring 3
    pub JRINTR_JR3: RWRegister<u32>,

    /// Job Ring Configuration Register for Job Ring 3, most-significant half
    pub JRCFGR_JR3_MS: RWRegister<u32>,

    /// Job Ring Configuration Register for Job Ring 3, least-significant half
    pub JRCFGR_JR3_LS: RWRegister<u32>,

    _reserved111: [u32; 1],

    /// Input Ring Read Index Register for Job Ring 3
    pub IRRIR_JR3: RWRegister<u32>,

    _reserved112: [u32; 1],

    /// Output Ring Write Index Register for Job Ring 3
    pub ORWIR_JR3: RWRegister<u32>,

    _reserved113: [u32; 1],

    /// Job Ring Command Register for Job Ring 3
    pub JRCR_JR3: WORegister<u32>,

    _reserved114: [u32; 421],

    /// Job Ring 3 Address-Array Valid Register
    pub JR3AAV: RORegister<u32>,

    _reserved115: [u32; 62],

    /// Job Ring 3 Address-Array Address 0 Register
    pub JR3AAA0: RORegister<u64>,

    /// Job Ring 3 Address-Array Address 1 Register
    pub JR3AAA1: RORegister<u64>,

    /// Job Ring 3 Address-Array Address 2 Register
    pub JR3AAA2: RORegister<u64>,

    /// Job Ring 3 Address-Array Address 3 Register
    pub JR3AAA3: RORegister<u64>,

    _reserved116: [u32; 120],

    /// Partition 0 SDID register
    pub P0SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P0SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P0SMAG1_JR3: RWRegister<u32>,

    /// Partition 1 SDID register
    pub P1SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P1SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P1SMAG1_JR3: RWRegister<u32>,

    /// Partition 2 SDID register
    pub P2SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P2SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P2SMAG1_JR3: RWRegister<u32>,

    /// Partition 3 SDID register
    pub P3SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P3SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P3SMAG1_JR3: RWRegister<u32>,

    /// Partition 4 SDID register
    pub P4SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P4SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P4SMAG1_JR3: RWRegister<u32>,

    /// Partition 5 SDID register
    pub P5SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P5SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P5SMAG1_JR3: RWRegister<u32>,

    /// Partition 6 SDID register
    pub P6SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P6SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P6SMAG1_JR3: RWRegister<u32>,

    /// Partition 7 SDID register
    pub P7SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P7SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P7SMAG1_JR3: RWRegister<u32>,

    /// Partition 8 SDID register
    pub P8SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P8SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P8SMAG1_JR3: RWRegister<u32>,

    /// Partition 9 SDID register
    pub P9SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P9SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P9SMAG1_JR3: RWRegister<u32>,

    /// Partition 10 SDID register
    pub P10SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P10SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P10SMAG1_JR3: RWRegister<u32>,

    /// Partition 11 SDID register
    pub P11SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P11SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P11SMAG1_JR3: RWRegister<u32>,

    /// Partition 12 SDID register
    pub P12SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P12SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P12SMAG1_JR3: RWRegister<u32>,

    /// Partition 13 SDID register
    pub P13SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P13SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P13SMAG1_JR3: RWRegister<u32>,

    /// Partition 14 SDID register
    pub P14SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P14SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P14SMAG1_JR3: RWRegister<u32>,

    /// Partition 15 SDID register
    pub P15SDID_JR3: RORegister<u32>,

    /// Secure Memory Access Permissions register
    pub P15SMAPR_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG2_JR3: RWRegister<u32>,

    /// Secure Memory Access Group Registers
    pub P15SMAG1_JR3: RWRegister<u32>,

    _reserved117: [u32; 57],

    /// Secure Memory Command Register
    pub SMCR_JR3: WORegister<u32>,

    _reserved118: [u32; 1],

    /// Secure Memory Command Status Register
    pub SMCSR_JR3: RORegister<u32>,

    _reserved119: [u32; 132],

    /// Recoverable Error Interrupt Record 0 for Job Ring 3
    pub REIR0JR3: RORegister<u32>,

    _reserved120: [u32; 1],

    /// Recoverable Error Interrupt Record 2 for Job Ring 3
    pub REIR2JR3: RORegister<u64>,

    /// Recoverable Error Interrupt Record 4 for Job Ring 3
    pub REIR4JR3: RORegister<u32>,

    /// Recoverable Error Interrupt Record 5 for Job Ring 3
    pub REIR5JR3: RORegister<u32>,

    _reserved121: [u32; 31867],

    /// RTIC Status Register
    pub RSTA: RORegister<u32>,

    _reserved122: [u32; 1],

    /// RTIC Command Register
    pub RCMD: RWRegister<u32>,

    _reserved123: [u32; 1],

    /// RTIC Control Register
    pub RCTL: RWRegister<u32>,

    _reserved124: [u32; 1],

    /// RTIC Throttle Register
    pub RTHR: RWRegister<u32>,

    _reserved125: [u32; 2],

    /// RTIC Watchdog Timer
    pub RWDOG: RWRegister<u64>,

    _reserved126: [u32; 1],

    /// RTIC Endian Register
    pub REND: RWRegister<u32>,

    _reserved127: [u32; 882],

    /// Recoverable Error Interrupt Record 0 for RTIC
    pub REIR0RTIC: RORegister<u32>,

    _reserved128: [u32; 1],

    /// Recoverable Error Interrupt Record 2 for RTIC
    pub REIR2RTIC: RORegister<u64>,

    /// Recoverable Error Interrupt Record 4 for RTIC
    pub REIR4RTIC: RORegister<u32>,

    /// Recoverable Error Interrupt Record 5 for RTIC
    pub REIR5RTIC: RORegister<u32>,

    _reserved129: [u32; 31867],

    /// C0C1MR and C0C1MR_RNG
    /// C0C1MR: C0C1MR and C0C1MR_PK
    /// C0C1MR: CCB 0 Class 1 Mode Register Format for Non-Public Key Algorithms
    /// C0C1MR_PK: CCB 0 Class 1 Mode Register Format for Public Key Algorithms
    /// C0C1MR_RNG: CCB 0 Class 1 Mode Register Format for RNG4
    pub C0C1MR: RWRegister<u32>,

    _reserved130: [u32; 1],

    /// CCB 0 Class 1 Key Size Register
    pub C0C1KSR: RWRegister<u32>,

    /// CCB 0 Class 1 Data Size Register
    pub C0C1DSR: RWRegister<u64>,

    _reserved131: [u32; 1],

    /// CCB 0 Class 1 ICV Size Register
    pub C0C1ICVSR: RWRegister<u32>,

    _reserved132: [u32; 5],

    /// CCB 0 CHA Control Register
    pub C0CCTRL: WORegister<u32>,

    _reserved133: [u32; 1],

    /// CCB 0 Interrupt Control Register
    pub C0ICTL: RWRegister<u32>,

    _reserved134: [u32; 1],

    /// CCB 0 Clear Written Register
    pub C0CWR: WORegister<u32>,

    /// CCB 0 Status and Error Register, most-significant half
    pub C0CSTA_MS: RORegister<u32>,

    /// CCB 0 Status and Error Register, least-significant half
    pub C0CSTA_LS: RORegister<u32>,

    _reserved135: [u32; 3],

    /// CCB 0 Class 1 AAD Size Register
    pub C0C1AADSZR: RWRegister<u32>,

    _reserved136: [u32; 1],

    /// CCB 0 Class 1 IV Size Register
    pub C0C1IVSZR: RWRegister<u32>,

    _reserved137: [u32; 7],

    /// PKHA A Size Register
    pub C0PKASZR: RWRegister<u32>,

    _reserved138: [u32; 1],

    /// PKHA B Size Register
    pub C0PKBSZR: RWRegister<u32>,

    _reserved139: [u32; 1],

    /// PKHA N Size Register
    pub C0PKNSZR: RWRegister<u32>,

    _reserved140: [u32; 1],

    /// PKHA E Size Register
    pub C0PKESZR: RWRegister<u32>,

    _reserved141: [u32; 24],

    /// CCB 0 Class 1 Context Register Word 0
    pub C0C1CTXR0: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 1
    pub C0C1CTXR1: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 2
    pub C0C1CTXR2: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 3
    pub C0C1CTXR3: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 4
    pub C0C1CTXR4: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 5
    pub C0C1CTXR5: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 6
    pub C0C1CTXR6: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 7
    pub C0C1CTXR7: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 8
    pub C0C1CTXR8: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 9
    pub C0C1CTXR9: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 10
    pub C0C1CTXR10: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 11
    pub C0C1CTXR11: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 12
    pub C0C1CTXR12: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 13
    pub C0C1CTXR13: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 14
    pub C0C1CTXR14: RWRegister<u32>,

    /// CCB 0 Class 1 Context Register Word 15
    pub C0C1CTXR15: RWRegister<u32>,

    _reserved142: [u32; 48],

    /// CCB 0 Class 1 Key Registers Word 0
    pub C0C1KR0: RWRegister<u32>,

    /// CCB 0 Class 1 Key Registers Word 1
    pub C0C1KR1: RWRegister<u32>,

    /// CCB 0 Class 1 Key Registers Word 2
    pub C0C1KR2: RWRegister<u32>,

    /// CCB 0 Class 1 Key Registers Word 3
    pub C0C1KR3: RWRegister<u32>,

    /// CCB 0 Class 1 Key Registers Word 4
    pub C0C1KR4: RWRegister<u32>,

    /// CCB 0 Class 1 Key Registers Word 5
    pub C0C1KR5: RWRegister<u32>,

    /// CCB 0 Class 1 Key Registers Word 6
    pub C0C1KR6: RWRegister<u32>,

    /// CCB 0 Class 1 Key Registers Word 7
    pub C0C1KR7: RWRegister<u32>,

    _reserved143: [u32; 121],

    /// CCB 0 Class 2 Mode Register
    pub C0C2MR: RWRegister<u32>,

    _reserved144: [u32; 1],

    /// CCB 0 Class 2 Key Size Register
    pub C0C2KSR: RWRegister<u32>,

    /// CCB 0 Class 2 Data Size Register
    pub C0C2DSR: RWRegister<u64>,

    _reserved145: [u32; 1],

    /// CCB 0 Class 2 ICV Size Register
    pub C0C2ICVSZR: RWRegister<u32>,

    _reserved146: [u32; 56],

    /// CCB 0 Class 2 Context Register Word 0
    pub C0C2CTXR0: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 1
    pub C0C2CTXR1: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 2
    pub C0C2CTXR2: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 3
    pub C0C2CTXR3: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 4
    pub C0C2CTXR4: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 5
    pub C0C2CTXR5: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 6
    pub C0C2CTXR6: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 7
    pub C0C2CTXR7: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 8
    pub C0C2CTXR8: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 9
    pub C0C2CTXR9: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 10
    pub C0C2CTXR10: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 11
    pub C0C2CTXR11: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 12
    pub C0C2CTXR12: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 13
    pub C0C2CTXR13: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 14
    pub C0C2CTXR14: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 15
    pub C0C2CTXR15: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 16
    pub C0C2CTXR16: RWRegister<u32>,

    /// CCB 0 Class 2 Context Register Word 17
    pub C0C2CTXR17: RWRegister<u32>,

    _reserved147: [u32; 46],

    /// CCB 0 Class 2 Key Register Word 0
    pub C0C2KEYR0: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 1
    pub C0C2KEYR1: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 2
    pub C0C2KEYR2: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 3
    pub C0C2KEYR3: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 4
    pub C0C2KEYR4: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 5
    pub C0C2KEYR5: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 6
    pub C0C2KEYR6: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 7
    pub C0C2KEYR7: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 8
    pub C0C2KEYR8: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 9
    pub C0C2KEYR9: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 10
    pub C0C2KEYR10: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 11
    pub C0C2KEYR11: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 12
    pub C0C2KEYR12: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 13
    pub C0C2KEYR13: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 14
    pub C0C2KEYR14: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 15
    pub C0C2KEYR15: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 16
    pub C0C2KEYR16: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 17
    pub C0C2KEYR17: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 18
    pub C0C2KEYR18: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 19
    pub C0C2KEYR19: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 20
    pub C0C2KEYR20: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 21
    pub C0C2KEYR21: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 22
    pub C0C2KEYR22: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 23
    pub C0C2KEYR23: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 24
    pub C0C2KEYR24: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 25
    pub C0C2KEYR25: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 26
    pub C0C2KEYR26: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 27
    pub C0C2KEYR27: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 28
    pub C0C2KEYR28: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 29
    pub C0C2KEYR29: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 30
    pub C0C2KEYR30: RWRegister<u32>,

    /// CCB 0 Class 2 Key Register Word 31
    pub C0C2KEYR31: RWRegister<u32>,

    _reserved148: [u32; 80],

    /// CCB 0 FIFO Status Register
    pub C0FIFOSTA: RORegister<u32>,

    _reserved149: [u32; 3],

    /// C0NFIFO and C0NFIFO_2
    /// C0NFIFO: CCB 0 iNformation FIFO When STYPE != 10b
    /// C0NFIFO_2: CCB 0 iNformation FIFO When STYPE == 10b
    pub C0NFIFO: RWRegister<u32>,

    _reserved150: [u32; 3],

    /// CCB 0 Input Data FIFO
    pub C0IFIFO: WORegister<u32>,

    _reserved151: [u32; 3],

    /// CCB 0 Output Data FIFO
    pub C0OFIFO: RORegister<u64>,

    _reserved152: [u32; 2],

    /// DECO0 Job Queue Control Register, most-significant half
    pub D0JQCR_MS: RWRegister<u32>,

    /// DECO0 Job Queue Control Register, least-significant half
    pub D0JQCR_LS: RORegister<u32>,

    /// DECO0 Descriptor Address Register
    pub D0DAR: RORegister<u64>,

    /// DECO0 Operation Status Register, most-significant half
    pub D0OPSTA_MS: RORegister<u32>,

    /// DECO0 Operation Status Register, least-significant half
    pub D0OPSTA_LS: RORegister<u32>,

    _reserved153: [u32; 2],

    /// DECO0 Primary DID Status Register
    pub D0PDIDSR: RORegister<u32>,

    /// DECO0 Output DID Status Register
    pub D0ODIDSR: RORegister<u32>,

    _reserved154: [u32; 6],

    /// DECO0 Math Register 0_MS
    pub D0MTH0_MS: RWRegister<u32>,

    /// DECO0 Math Register 0_LS
    pub D0MTH0_LS: RWRegister<u32>,

    /// DECO0 Math Register 1_MS
    pub D0MTH1_MS: RWRegister<u32>,

    /// DECO0 Math Register 1_LS
    pub D0MTH1_LS: RWRegister<u32>,

    /// DECO0 Math Register 2_MS
    pub D0MTH2_MS: RWRegister<u32>,

    /// DECO0 Math Register 2_LS
    pub D0MTH2_LS: RWRegister<u32>,

    /// DECO0 Math Register 3_MS
    pub D0MTH3_MS: RWRegister<u32>,

    /// DECO0 Math Register 3_LS
    pub D0MTH3_LS: RWRegister<u32>,

    _reserved155: [u32; 8],

    /// DECO0 Gather Table Register 0 Word 0
    pub D0GTR0_0: RWRegister<u32>,

    /// DECO0 Gather Table Register 0 Word 1
    pub D0GTR0_1: RWRegister<u32>,

    /// DECO0 Gather Table Register 0 Word 2
    pub D0GTR0_2: RWRegister<u32>,

    /// DECO0 Gather Table Register 0 Word 3
    pub D0GTR0_3: RWRegister<u32>,

    _reserved156: [u32; 28],

    /// DECO0 Scatter Table Register 0 Word 0
    pub D0STR0_0: RWRegister<u32>,

    /// DECO0 Scatter Table Register 0 Word 1
    pub D0STR0_1: RWRegister<u32>,

    /// DECO0 Scatter Table Register 0 Word 2
    pub D0STR0_2: RWRegister<u32>,

    /// DECO0 Scatter Table Register 0 Word 3
    pub D0STR0_3: RWRegister<u32>,

    _reserved157: [u32; 60],

    /// DECO0 Descriptor Buffer Word 0
    pub D0DESB0: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 1
    pub D0DESB1: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 2
    pub D0DESB2: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 3
    pub D0DESB3: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 4
    pub D0DESB4: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 5
    pub D0DESB5: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 6
    pub D0DESB6: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 7
    pub D0DESB7: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 8
    pub D0DESB8: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 9
    pub D0DESB9: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 10
    pub D0DESB10: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 11
    pub D0DESB11: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 12
    pub D0DESB12: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 13
    pub D0DESB13: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 14
    pub D0DESB14: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 15
    pub D0DESB15: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 16
    pub D0DESB16: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 17
    pub D0DESB17: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 18
    pub D0DESB18: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 19
    pub D0DESB19: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 20
    pub D0DESB20: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 21
    pub D0DESB21: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 22
    pub D0DESB22: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 23
    pub D0DESB23: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 24
    pub D0DESB24: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 25
    pub D0DESB25: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 26
    pub D0DESB26: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 27
    pub D0DESB27: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 28
    pub D0DESB28: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 29
    pub D0DESB29: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 30
    pub D0DESB30: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 31
    pub D0DESB31: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 32
    pub D0DESB32: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 33
    pub D0DESB33: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 34
    pub D0DESB34: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 35
    pub D0DESB35: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 36
    pub D0DESB36: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 37
    pub D0DESB37: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 38
    pub D0DESB38: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 39
    pub D0DESB39: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 40
    pub D0DESB40: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 41
    pub D0DESB41: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 42
    pub D0DESB42: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 43
    pub D0DESB43: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 44
    pub D0DESB44: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 45
    pub D0DESB45: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 46
    pub D0DESB46: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 47
    pub D0DESB47: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 48
    pub D0DESB48: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 49
    pub D0DESB49: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 50
    pub D0DESB50: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 51
    pub D0DESB51: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 52
    pub D0DESB52: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 53
    pub D0DESB53: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 54
    pub D0DESB54: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 55
    pub D0DESB55: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 56
    pub D0DESB56: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 57
    pub D0DESB57: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 58
    pub D0DESB58: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 59
    pub D0DESB59: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 60
    pub D0DESB60: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 61
    pub D0DESB61: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 62
    pub D0DESB62: RWRegister<u32>,

    /// DECO0 Descriptor Buffer Word 63
    pub D0DESB63: RWRegister<u32>,

    _reserved158: [u32; 192],

    /// DECO0 Debug Job Register
    pub D0DJR: RORegister<u32>,

    /// DECO0 Debug DECO Register
    pub D0DDR: RORegister<u32>,

    /// DECO0 Debug Job Pointer
    pub D0DJP: RORegister<u64>,

    /// DECO0 Debug Shared Pointer
    pub D0SDP: RORegister<u64>,

    /// DECO0 Debug DID, most-significant half
    pub D0DDR_MS: RORegister<u32>,

    /// DECO0 Debug DID, least-significant half
    pub D0DDR_LS: RORegister<u32>,

    /// Sequence Output Length Register
    pub SOL0: RWRegister<u32>,

    /// Variable Sequence Output Length Register
    pub VSOL0: RWRegister<u32>,

    /// Sequence Input Length Register
    pub SIL0: RWRegister<u32>,

    /// Variable Sequence Input Length Register
    pub VSIL0: RWRegister<u32>,

    /// Protocol Override Register
    pub D0POVRD: RWRegister<u32>,

    /// Variable Sequence Output Length Register; Upper 32 bits
    pub UVSOL0: RWRegister<u32>,

    /// Variable Sequence Input Length Register; Upper 32 bits
    pub UVSIL0: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCFGR: u32,
    pub PAGE0_SDID: u32,
    pub SCFGR: u32,
    pub JRDID_MS_0: u32,
    pub JRDID_LS_0: u32,
    pub JRDID_MS_1: u32,
    pub JRDID_LS_1: u32,
    pub JRDID_MS_2: u32,
    pub JRDID_LS_2: u32,
    pub JRDID_MS_3: u32,
    pub JRDID_LS_3: u32,
    pub DEBUGCTL: u32,
    pub JRSTARTR: u32,
    pub RTIC_OWN: u32,
    pub RTIC_DID_0: u32,
    pub RTIC_DID_1: u32,
    pub RTIC_DID_2: u32,
    pub RTIC_DID_3: u32,
    pub DECORSR: u32,
    pub DECORR: u32,
    pub DECO0DID_MS: u32,
    pub DECO0DID_LS: u32,
    pub DAR: u32,
    pub DRR: u32,
    pub JRSMVBAR_0: u32,
    pub JRSMVBAR_1: u32,
    pub JRSMVBAR_2: u32,
    pub JRSMVBAR_3: u32,
    pub PBSL: u32,
    pub DMA0_AIDL_MAP_MS: u32,
    pub DMA0_AIDL_MAP_LS: u32,
    pub DMA0_AIDM_MAP_MS: u32,
    pub DMA0_AIDM_MAP_LS: u32,
    pub DMA0_AID_ENB: u32,
    pub DMA0_ARD_TC: u64,
    pub DMA0_ARD_LAT: u32,
    pub DMA0_AWR_TC: u64,
    pub DMA0_AWR_LAT: u32,
    pub MPPKR0: u8,
    pub MPPKR1: u8,
    pub MPPKR2: u8,
    pub MPPKR3: u8,
    pub MPPKR4: u8,
    pub MPPKR5: u8,
    pub MPPKR6: u8,
    pub MPPKR7: u8,
    pub MPPKR8: u8,
    pub MPPKR9: u8,
    pub MPPKR10: u8,
    pub MPPKR11: u8,
    pub MPPKR12: u8,
    pub MPPKR13: u8,
    pub MPPKR14: u8,
    pub MPPKR15: u8,
    pub MPPKR16: u8,
    pub MPPKR17: u8,
    pub MPPKR18: u8,
    pub MPPKR19: u8,
    pub MPPKR20: u8,
    pub MPPKR21: u8,
    pub MPPKR22: u8,
    pub MPPKR23: u8,
    pub MPPKR24: u8,
    pub MPPKR25: u8,
    pub MPPKR26: u8,
    pub MPPKR27: u8,
    pub MPPKR28: u8,
    pub MPPKR29: u8,
    pub MPPKR30: u8,
    pub MPPKR31: u8,
    pub MPPKR32: u8,
    pub MPPKR33: u8,
    pub MPPKR34: u8,
    pub MPPKR35: u8,
    pub MPPKR36: u8,
    pub MPPKR37: u8,
    pub MPPKR38: u8,
    pub MPPKR39: u8,
    pub MPPKR40: u8,
    pub MPPKR41: u8,
    pub MPPKR42: u8,
    pub MPPKR43: u8,
    pub MPPKR44: u8,
    pub MPPKR45: u8,
    pub MPPKR46: u8,
    pub MPPKR47: u8,
    pub MPPKR48: u8,
    pub MPPKR49: u8,
    pub MPPKR50: u8,
    pub MPPKR51: u8,
    pub MPPKR52: u8,
    pub MPPKR53: u8,
    pub MPPKR54: u8,
    pub MPPKR55: u8,
    pub MPPKR56: u8,
    pub MPPKR57: u8,
    pub MPPKR58: u8,
    pub MPPKR59: u8,
    pub MPPKR60: u8,
    pub MPPKR61: u8,
    pub MPPKR62: u8,
    pub MPPKR63: u8,
    pub MPMR0: u8,
    pub MPMR1: u8,
    pub MPMR2: u8,
    pub MPMR3: u8,
    pub MPMR4: u8,
    pub MPMR5: u8,
    pub MPMR6: u8,
    pub MPMR7: u8,
    pub MPMR8: u8,
    pub MPMR9: u8,
    pub MPMR10: u8,
    pub MPMR11: u8,
    pub MPMR12: u8,
    pub MPMR13: u8,
    pub MPMR14: u8,
    pub MPMR15: u8,
    pub MPMR16: u8,
    pub MPMR17: u8,
    pub MPMR18: u8,
    pub MPMR19: u8,
    pub MPMR20: u8,
    pub MPMR21: u8,
    pub MPMR22: u8,
    pub MPMR23: u8,
    pub MPMR24: u8,
    pub MPMR25: u8,
    pub MPMR26: u8,
    pub MPMR27: u8,
    pub MPMR28: u8,
    pub MPMR29: u8,
    pub MPMR30: u8,
    pub MPMR31: u8,
    pub MPTESTR0: u8,
    pub MPTESTR1: u8,
    pub MPTESTR2: u8,
    pub MPTESTR3: u8,
    pub MPTESTR4: u8,
    pub MPTESTR5: u8,
    pub MPTESTR6: u8,
    pub MPTESTR7: u8,
    pub MPTESTR8: u8,
    pub MPTESTR9: u8,
    pub MPTESTR10: u8,
    pub MPTESTR11: u8,
    pub MPTESTR12: u8,
    pub MPTESTR13: u8,
    pub MPTESTR14: u8,
    pub MPTESTR15: u8,
    pub MPTESTR16: u8,
    pub MPTESTR17: u8,
    pub MPTESTR18: u8,
    pub MPTESTR19: u8,
    pub MPTESTR20: u8,
    pub MPTESTR21: u8,
    pub MPTESTR22: u8,
    pub MPTESTR23: u8,
    pub MPTESTR24: u8,
    pub MPTESTR25: u8,
    pub MPTESTR26: u8,
    pub MPTESTR27: u8,
    pub MPTESTR28: u8,
    pub MPTESTR29: u8,
    pub MPTESTR30: u8,
    pub MPTESTR31: u8,
    pub MPECC: u32,
    pub JDKEKR0: u32,
    pub JDKEKR1: u32,
    pub JDKEKR2: u32,
    pub JDKEKR3: u32,
    pub JDKEKR4: u32,
    pub JDKEKR5: u32,
    pub JDKEKR6: u32,
    pub JDKEKR7: u32,
    pub TDKEKR0: u32,
    pub TDKEKR1: u32,
    pub TDKEKR2: u32,
    pub TDKEKR3: u32,
    pub TDKEKR4: u32,
    pub TDKEKR5: u32,
    pub TDKEKR6: u32,
    pub TDKEKR7: u32,
    pub TDSKR0: u32,
    pub TDSKR1: u32,
    pub TDSKR2: u32,
    pub TDSKR3: u32,
    pub TDSKR4: u32,
    pub TDSKR5: u32,
    pub TDSKR6: u32,
    pub TDSKR7: u32,
    pub SKNR: u64,
    pub DMA_STA: u32,
    pub DMA_X_AID_7_4_MAP: u32,
    pub DMA_X_AID_3_0_MAP: u32,
    pub DMA_X_AID_15_12_MAP: u32,
    pub DMA_X_AID_11_8_MAP: u32,
    pub DMA_X_AID_15_0_EN: u32,
    pub DMA_X_ARTC_CTL: u32,
    pub DMA_X_ARTC_LC: u32,
    pub DMA_X_ARTC_SC: u32,
    pub DMA_X_ARTC_LAT: u32,
    pub DMA_X_AWTC_CTL: u32,
    pub DMA_X_AWTC_LC: u32,
    pub DMA_X_AWTC_SC: u32,
    pub DMA_X_AWTC_LAT: u32,
    pub RTMCTL: u32,
    pub RTSCMISC: u32,
    pub RTPKRRNG: u32,
    pub RTPKR: u32,
    pub RTSDCTL: u32,
    pub RT: u32,
    pub RTFRQMIN: u32,
    pub RTFRQ: u32,
    pub RTSCM: u32,
    pub RTSCR1: u32,
    pub RTSCR2: u32,
    pub RTSCR3: u32,
    pub RTSCR4: u32,
    pub RTSCR5: u32,
    pub RTSCR6P: u32,
    pub RTSTATUS: u32,
    pub RTENT0: u32,
    pub RTENT1: u32,
    pub RTENT2: u32,
    pub RTENT3: u32,
    pub RTENT4: u32,
    pub RTENT5: u32,
    pub RTENT6: u32,
    pub RTENT7: u32,
    pub RTENT8: u32,
    pub RTENT9: u32,
    pub RTENT10: u32,
    pub RTENT11: u32,
    pub RTENT12: u32,
    pub RTENT13: u32,
    pub RTENT14: u32,
    pub RTENT15: u32,
    pub RTPKRCNT10: u32,
    pub RTPKRCNT32: u32,
    pub RTPKRCNT54: u32,
    pub RTPKRCNT76: u32,
    pub RTPKRCNT98: u32,
    pub RTPKRCNTBA: u32,
    pub RTPKRCNTDC: u32,
    pub RTPKRCNTFE: u32,
    pub RDSTA: u32,
    pub RDINT0: u32,
    pub RDINT1: u32,
    pub RDHCNTL: u32,
    pub RDHDIG: u32,
    pub RDHBUF: u32,
    pub P0SDID_PG0: u32,
    pub P0SMAPR_PG0: u32,
    pub P0SMAG2_PG0: u32,
    pub P0SMAG1_PG0: u32,
    pub P1SDID_PG0: u32,
    pub P1SMAPR_PG0: u32,
    pub P1SMAG2_PG0: u32,
    pub P1SMAG1_PG0: u32,
    pub P2SDID_PG0: u32,
    pub P2SMAPR_PG0: u32,
    pub P2SMAG2_PG0: u32,
    pub P2SMAG1_PG0: u32,
    pub P3SDID_PG0: u32,
    pub P3SMAPR_PG0: u32,
    pub P3SMAG2_PG0: u32,
    pub P3SMAG1_PG0: u32,
    pub P4SDID_PG0: u32,
    pub P4SMAPR_PG0: u32,
    pub P4SMAG2_PG0: u32,
    pub P4SMAG1_PG0: u32,
    pub P5SDID_PG0: u32,
    pub P5SMAPR_PG0: u32,
    pub P5SMAG2_PG0: u32,
    pub P5SMAG1_PG0: u32,
    pub P6SDID_PG0: u32,
    pub P6SMAPR_PG0: u32,
    pub P6SMAG2_PG0: u32,
    pub P6SMAG1_PG0: u32,
    pub P7SDID_PG0: u32,
    pub P7SMAPR_PG0: u32,
    pub P7SMAG2_PG0: u32,
    pub P7SMAG1_PG0: u32,
    pub P8SDID_PG0: u32,
    pub P8SMAPR_PG0: u32,
    pub P8SMAG2_PG0: u32,
    pub P8SMAG1_PG0: u32,
    pub P9SDID_PG0: u32,
    pub P9SMAPR_PG0: u32,
    pub P9SMAG2_PG0: u32,
    pub P9SMAG1_PG0: u32,
    pub P10SDID_PG0: u32,
    pub P10SMAPR_PG0: u32,
    pub P10SMAG2_PG0: u32,
    pub P10SMAG1_PG0: u32,
    pub P11SDID_PG0: u32,
    pub P11SMAPR_PG0: u32,
    pub P11SMAG2_PG0: u32,
    pub P11SMAG1_PG0: u32,
    pub P12SDID_PG0: u32,
    pub P12SMAPR_PG0: u32,
    pub P12SMAG2_PG0: u32,
    pub P12SMAG1_PG0: u32,
    pub P13SDID_PG0: u32,
    pub P13SMAPR_PG0: u32,
    pub P13SMAG2_PG0: u32,
    pub P13SMAG1_PG0: u32,
    pub P14SDID_PG0: u32,
    pub P14SMAPR_PG0: u32,
    pub P14SMAG2_PG0: u32,
    pub P14SMAG1_PG0: u32,
    pub P15SDID_PG0: u32,
    pub P15SMAPR_PG0: u32,
    pub P15SMAG2_PG0: u32,
    pub P15SMAG1_PG0: u32,
    pub REIS: u32,
    pub REIE: u32,
    pub REIF: u32,
    pub REIH: u32,
    pub SMWPJRR0: u32,
    pub SMWPJRR1: u32,
    pub SMWPJRR2: u32,
    pub SMWPJRR3: u32,
    pub SMCR_PG0: u32,
    pub SMCSR_PG0: u32,
    pub HT0_JD_ADDR: u64,
    pub HT0_SD_ADDR: u64,
    pub HT0_JQ_CTRL_MS: u32,
    pub HT0_JQ_CTRL_LS: u32,
    pub HT0_STATUS: u32,
    pub JQ_DEBUG_SEL: u32,
    pub JRJIDU_LS: u32,
    pub JRJDJIFBC: u32,
    pub JRJDJIF: u32,
    pub JRJDS1: u32,
    pub JRJDDA: u64,
    pub CRNR_MS: u32,
    pub CRNR_LS: u32,
    pub CTPR_MS: u32,
    pub CTPR_LS: u32,
    pub SMSTA: u32,
    pub SMPO: u32,
    pub FAR: u64,
    pub FADID: u32,
    pub FADR: u32,
    pub CSTA: u32,
    pub SMVID_MS: u32,
    pub SMVID_LS: u32,
    pub RVID: u32,
    pub CCBVID: u32,
    pub CHAVID_MS: u32,
    pub CHAVID_LS: u32,
    pub CHANUM_MS: u32,
    pub CHANUM_LS: u32,
    pub CAAMVID_MS: u32,
    pub CAAMVID_LS: u32,
    pub IRBAR_JR0: u64,
    pub IRSR_JR0: u32,
    pub IRSAR_JR0: u32,
    pub IRJAR_JR0: u32,
    pub ORBAR_JR0: u64,
    pub ORSR_JR0: u32,
    pub ORJRR_JR0: u32,
    pub ORSFR_JR0: u32,
    pub JRSTAR_JR0: u32,
    pub JRINTR_JR0: u32,
    pub JRCFGR_JR0_MS: u32,
    pub JRCFGR_JR0_LS: u32,
    pub IRRIR_JR0: u32,
    pub ORWIR_JR0: u32,
    pub JRCR_JR0: u32,
    pub JR0AAV: u32,
    pub JR0AAA0: u64,
    pub JR0AAA1: u64,
    pub JR0AAA2: u64,
    pub JR0AAA3: u64,
    pub P0SDID_JR0: u32,
    pub P0SMAPR_JR0: u32,
    pub P0SMAG2_JR0: u32,
    pub P0SMAG1_JR0: u32,
    pub P1SDID_JR0: u32,
    pub P1SMAPR_JR0: u32,
    pub P1SMAG2_JR0: u32,
    pub P1SMAG1_JR0: u32,
    pub P2SDID_JR0: u32,
    pub P2SMAPR_JR0: u32,
    pub P2SMAG2_JR0: u32,
    pub P2SMAG1_JR0: u32,
    pub P3SDID_JR0: u32,
    pub P3SMAPR_JR0: u32,
    pub P3SMAG2_JR0: u32,
    pub P3SMAG1_JR0: u32,
    pub P4SDID_JR0: u32,
    pub P4SMAPR_JR0: u32,
    pub P4SMAG2_JR0: u32,
    pub P4SMAG1_JR0: u32,
    pub P5SDID_JR0: u32,
    pub P5SMAPR_JR0: u32,
    pub P5SMAG2_JR0: u32,
    pub P5SMAG1_JR0: u32,
    pub P6SDID_JR0: u32,
    pub P6SMAPR_JR0: u32,
    pub P6SMAG2_JR0: u32,
    pub P6SMAG1_JR0: u32,
    pub P7SDID_JR0: u32,
    pub P7SMAPR_JR0: u32,
    pub P7SMAG2_JR0: u32,
    pub P7SMAG1_JR0: u32,
    pub P8SDID_JR0: u32,
    pub P8SMAPR_JR0: u32,
    pub P8SMAG2_JR0: u32,
    pub P8SMAG1_JR0: u32,
    pub P9SDID_JR0: u32,
    pub P9SMAPR_JR0: u32,
    pub P9SMAG2_JR0: u32,
    pub P9SMAG1_JR0: u32,
    pub P10SDID_JR0: u32,
    pub P10SMAPR_JR0: u32,
    pub P10SMAG2_JR0: u32,
    pub P10SMAG1_JR0: u32,
    pub P11SDID_JR0: u32,
    pub P11SMAPR_JR0: u32,
    pub P11SMAG2_JR0: u32,
    pub P11SMAG1_JR0: u32,
    pub P12SDID_JR0: u32,
    pub P12SMAPR_JR0: u32,
    pub P12SMAG2_JR0: u32,
    pub P12SMAG1_JR0: u32,
    pub P13SDID_JR0: u32,
    pub P13SMAPR_JR0: u32,
    pub P13SMAG2_JR0: u32,
    pub P13SMAG1_JR0: u32,
    pub P14SDID_JR0: u32,
    pub P14SMAPR_JR0: u32,
    pub P14SMAG2_JR0: u32,
    pub P14SMAG1_JR0: u32,
    pub P15SDID_JR0: u32,
    pub P15SMAPR_JR0: u32,
    pub P15SMAG2_JR0: u32,
    pub P15SMAG1_JR0: u32,
    pub SMCR_JR0: u32,
    pub SMCSR_JR0: u32,
    pub REIR0JR0: u32,
    pub REIR2JR0: u64,
    pub REIR4JR0: u32,
    pub REIR5JR0: u32,
    pub IRBAR_JR1: u64,
    pub IRSR_JR1: u32,
    pub IRSAR_JR1: u32,
    pub IRJAR_JR1: u32,
    pub ORBAR_JR1: u64,
    pub ORSR_JR1: u32,
    pub ORJRR_JR1: u32,
    pub ORSFR_JR1: u32,
    pub JRSTAR_JR1: u32,
    pub JRINTR_JR1: u32,
    pub JRCFGR_JR1_MS: u32,
    pub JRCFGR_JR1_LS: u32,
    pub IRRIR_JR1: u32,
    pub ORWIR_JR1: u32,
    pub JRCR_JR1: u32,
    pub JR1AAV: u32,
    pub JR1AAA0: u64,
    pub JR1AAA1: u64,
    pub JR1AAA2: u64,
    pub JR1AAA3: u64,
    pub P0SDID_JR1: u32,
    pub P0SMAPR_JR1: u32,
    pub P0SMAG2_JR1: u32,
    pub P0SMAG1_JR1: u32,
    pub P1SDID_JR1: u32,
    pub P1SMAPR_JR1: u32,
    pub P1SMAG2_JR1: u32,
    pub P1SMAG1_JR1: u32,
    pub P2SDID_JR1: u32,
    pub P2SMAPR_JR1: u32,
    pub P2SMAG2_JR1: u32,
    pub P2SMAG1_JR1: u32,
    pub P3SDID_JR1: u32,
    pub P3SMAPR_JR1: u32,
    pub P3SMAG2_JR1: u32,
    pub P3SMAG1_JR1: u32,
    pub P4SDID_JR1: u32,
    pub P4SMAPR_JR1: u32,
    pub P4SMAG2_JR1: u32,
    pub P4SMAG1_JR1: u32,
    pub P5SDID_JR1: u32,
    pub P5SMAPR_JR1: u32,
    pub P5SMAG2_JR1: u32,
    pub P5SMAG1_JR1: u32,
    pub P6SDID_JR1: u32,
    pub P6SMAPR_JR1: u32,
    pub P6SMAG2_JR1: u32,
    pub P6SMAG1_JR1: u32,
    pub P7SDID_JR1: u32,
    pub P7SMAPR_JR1: u32,
    pub P7SMAG2_JR1: u32,
    pub P7SMAG1_JR1: u32,
    pub P8SDID_JR1: u32,
    pub P8SMAPR_JR1: u32,
    pub P8SMAG2_JR1: u32,
    pub P8SMAG1_JR1: u32,
    pub P9SDID_JR1: u32,
    pub P9SMAPR_JR1: u32,
    pub P9SMAG2_JR1: u32,
    pub P9SMAG1_JR1: u32,
    pub P10SDID_JR1: u32,
    pub P10SMAPR_JR1: u32,
    pub P10SMAG2_JR1: u32,
    pub P10SMAG1_JR1: u32,
    pub P11SDID_JR1: u32,
    pub P11SMAPR_JR1: u32,
    pub P11SMAG2_JR1: u32,
    pub P11SMAG1_JR1: u32,
    pub P12SDID_JR1: u32,
    pub P12SMAPR_JR1: u32,
    pub P12SMAG2_JR1: u32,
    pub P12SMAG1_JR1: u32,
    pub P13SDID_JR1: u32,
    pub P13SMAPR_JR1: u32,
    pub P13SMAG2_JR1: u32,
    pub P13SMAG1_JR1: u32,
    pub P14SDID_JR1: u32,
    pub P14SMAPR_JR1: u32,
    pub P14SMAG2_JR1: u32,
    pub P14SMAG1_JR1: u32,
    pub P15SDID_JR1: u32,
    pub P15SMAPR_JR1: u32,
    pub P15SMAG2_JR1: u32,
    pub P15SMAG1_JR1: u32,
    pub SMCR_JR1: u32,
    pub SMCSR_JR1: u32,
    pub REIR0JR1: u32,
    pub REIR2JR1: u64,
    pub REIR4JR1: u32,
    pub REIR5JR1: u32,
    pub IRBAR_JR2: u64,
    pub IRSR_JR2: u32,
    pub IRSAR_JR2: u32,
    pub IRJAR_JR2: u32,
    pub ORBAR_JR2: u64,
    pub ORSR_JR2: u32,
    pub ORJRR_JR2: u32,
    pub ORSFR_JR2: u32,
    pub JRSTAR_JR2: u32,
    pub JRINTR_JR2: u32,
    pub JRCFGR_JR2_MS: u32,
    pub JRCFGR_JR2_LS: u32,
    pub IRRIR_JR2: u32,
    pub ORWIR_JR2: u32,
    pub JRCR_JR2: u32,
    pub JR2AAV: u32,
    pub JR2AAA0: u64,
    pub JR2AAA1: u64,
    pub JR2AAA2: u64,
    pub JR2AAA3: u64,
    pub P0SDID_JR2: u32,
    pub P0SMAPR_JR2: u32,
    pub P0SMAG2_JR2: u32,
    pub P0SMAG1_JR2: u32,
    pub P1SDID_JR2: u32,
    pub P1SMAPR_JR2: u32,
    pub P1SMAG2_JR2: u32,
    pub P1SMAG1_JR2: u32,
    pub P2SDID_JR2: u32,
    pub P2SMAPR_JR2: u32,
    pub P2SMAG2_JR2: u32,
    pub P2SMAG1_JR2: u32,
    pub P3SDID_JR2: u32,
    pub P3SMAPR_JR2: u32,
    pub P3SMAG2_JR2: u32,
    pub P3SMAG1_JR2: u32,
    pub P4SDID_JR2: u32,
    pub P4SMAPR_JR2: u32,
    pub P4SMAG2_JR2: u32,
    pub P4SMAG1_JR2: u32,
    pub P5SDID_JR2: u32,
    pub P5SMAPR_JR2: u32,
    pub P5SMAG2_JR2: u32,
    pub P5SMAG1_JR2: u32,
    pub P6SDID_JR2: u32,
    pub P6SMAPR_JR2: u32,
    pub P6SMAG2_JR2: u32,
    pub P6SMAG1_JR2: u32,
    pub P7SDID_JR2: u32,
    pub P7SMAPR_JR2: u32,
    pub P7SMAG2_JR2: u32,
    pub P7SMAG1_JR2: u32,
    pub P8SDID_JR2: u32,
    pub P8SMAPR_JR2: u32,
    pub P8SMAG2_JR2: u32,
    pub P8SMAG1_JR2: u32,
    pub P9SDID_JR2: u32,
    pub P9SMAPR_JR2: u32,
    pub P9SMAG2_JR2: u32,
    pub P9SMAG1_JR2: u32,
    pub P10SDID_JR2: u32,
    pub P10SMAPR_JR2: u32,
    pub P10SMAG2_JR2: u32,
    pub P10SMAG1_JR2: u32,
    pub P11SDID_JR2: u32,
    pub P11SMAPR_JR2: u32,
    pub P11SMAG2_JR2: u32,
    pub P11SMAG1_JR2: u32,
    pub P12SDID_JR2: u32,
    pub P12SMAPR_JR2: u32,
    pub P12SMAG2_JR2: u32,
    pub P12SMAG1_JR2: u32,
    pub P13SDID_JR2: u32,
    pub P13SMAPR_JR2: u32,
    pub P13SMAG2_JR2: u32,
    pub P13SMAG1_JR2: u32,
    pub P14SDID_JR2: u32,
    pub P14SMAPR_JR2: u32,
    pub P14SMAG2_JR2: u32,
    pub P14SMAG1_JR2: u32,
    pub P15SDID_JR2: u32,
    pub P15SMAPR_JR2: u32,
    pub P15SMAG2_JR2: u32,
    pub P15SMAG1_JR2: u32,
    pub SMCR_JR2: u32,
    pub SMCSR_JR2: u32,
    pub REIR0JR2: u32,
    pub REIR2JR2: u64,
    pub REIR4JR2: u32,
    pub REIR5JR2: u32,
    pub IRBAR_JR3: u64,
    pub IRSR_JR3: u32,
    pub IRSAR_JR3: u32,
    pub IRJAR_JR3: u32,
    pub ORBAR_JR3: u64,
    pub ORSR_JR3: u32,
    pub ORJRR_JR3: u32,
    pub ORSFR_JR3: u32,
    pub JRSTAR_JR3: u32,
    pub JRINTR_JR3: u32,
    pub JRCFGR_JR3_MS: u32,
    pub JRCFGR_JR3_LS: u32,
    pub IRRIR_JR3: u32,
    pub ORWIR_JR3: u32,
    pub JRCR_JR3: u32,
    pub JR3AAV: u32,
    pub JR3AAA0: u64,
    pub JR3AAA1: u64,
    pub JR3AAA2: u64,
    pub JR3AAA3: u64,
    pub P0SDID_JR3: u32,
    pub P0SMAPR_JR3: u32,
    pub P0SMAG2_JR3: u32,
    pub P0SMAG1_JR3: u32,
    pub P1SDID_JR3: u32,
    pub P1SMAPR_JR3: u32,
    pub P1SMAG2_JR3: u32,
    pub P1SMAG1_JR3: u32,
    pub P2SDID_JR3: u32,
    pub P2SMAPR_JR3: u32,
    pub P2SMAG2_JR3: u32,
    pub P2SMAG1_JR3: u32,
    pub P3SDID_JR3: u32,
    pub P3SMAPR_JR3: u32,
    pub P3SMAG2_JR3: u32,
    pub P3SMAG1_JR3: u32,
    pub P4SDID_JR3: u32,
    pub P4SMAPR_JR3: u32,
    pub P4SMAG2_JR3: u32,
    pub P4SMAG1_JR3: u32,
    pub P5SDID_JR3: u32,
    pub P5SMAPR_JR3: u32,
    pub P5SMAG2_JR3: u32,
    pub P5SMAG1_JR3: u32,
    pub P6SDID_JR3: u32,
    pub P6SMAPR_JR3: u32,
    pub P6SMAG2_JR3: u32,
    pub P6SMAG1_JR3: u32,
    pub P7SDID_JR3: u32,
    pub P7SMAPR_JR3: u32,
    pub P7SMAG2_JR3: u32,
    pub P7SMAG1_JR3: u32,
    pub P8SDID_JR3: u32,
    pub P8SMAPR_JR3: u32,
    pub P8SMAG2_JR3: u32,
    pub P8SMAG1_JR3: u32,
    pub P9SDID_JR3: u32,
    pub P9SMAPR_JR3: u32,
    pub P9SMAG2_JR3: u32,
    pub P9SMAG1_JR3: u32,
    pub P10SDID_JR3: u32,
    pub P10SMAPR_JR3: u32,
    pub P10SMAG2_JR3: u32,
    pub P10SMAG1_JR3: u32,
    pub P11SDID_JR3: u32,
    pub P11SMAPR_JR3: u32,
    pub P11SMAG2_JR3: u32,
    pub P11SMAG1_JR3: u32,
    pub P12SDID_JR3: u32,
    pub P12SMAPR_JR3: u32,
    pub P12SMAG2_JR3: u32,
    pub P12SMAG1_JR3: u32,
    pub P13SDID_JR3: u32,
    pub P13SMAPR_JR3: u32,
    pub P13SMAG2_JR3: u32,
    pub P13SMAG1_JR3: u32,
    pub P14SDID_JR3: u32,
    pub P14SMAPR_JR3: u32,
    pub P14SMAG2_JR3: u32,
    pub P14SMAG1_JR3: u32,
    pub P15SDID_JR3: u32,
    pub P15SMAPR_JR3: u32,
    pub P15SMAG2_JR3: u32,
    pub P15SMAG1_JR3: u32,
    pub SMCR_JR3: u32,
    pub SMCSR_JR3: u32,
    pub REIR0JR3: u32,
    pub REIR2JR3: u64,
    pub REIR4JR3: u32,
    pub REIR5JR3: u32,
    pub RSTA: u32,
    pub RCMD: u32,
    pub RCTL: u32,
    pub RTHR: u32,
    pub RWDOG: u64,
    pub REND: u32,
    pub REIR0RTIC: u32,
    pub REIR2RTIC: u64,
    pub REIR4RTIC: u32,
    pub REIR5RTIC: u32,
    pub C0C1MR: u32,
    pub C0C1KSR: u32,
    pub C0C1DSR: u64,
    pub C0C1ICVSR: u32,
    pub C0CCTRL: u32,
    pub C0ICTL: u32,
    pub C0CWR: u32,
    pub C0CSTA_MS: u32,
    pub C0CSTA_LS: u32,
    pub C0C1AADSZR: u32,
    pub C0C1IVSZR: u32,
    pub C0PKASZR: u32,
    pub C0PKBSZR: u32,
    pub C0PKNSZR: u32,
    pub C0PKESZR: u32,
    pub C0C1CTXR0: u32,
    pub C0C1CTXR1: u32,
    pub C0C1CTXR2: u32,
    pub C0C1CTXR3: u32,
    pub C0C1CTXR4: u32,
    pub C0C1CTXR5: u32,
    pub C0C1CTXR6: u32,
    pub C0C1CTXR7: u32,
    pub C0C1CTXR8: u32,
    pub C0C1CTXR9: u32,
    pub C0C1CTXR10: u32,
    pub C0C1CTXR11: u32,
    pub C0C1CTXR12: u32,
    pub C0C1CTXR13: u32,
    pub C0C1CTXR14: u32,
    pub C0C1CTXR15: u32,
    pub C0C1KR0: u32,
    pub C0C1KR1: u32,
    pub C0C1KR2: u32,
    pub C0C1KR3: u32,
    pub C0C1KR4: u32,
    pub C0C1KR5: u32,
    pub C0C1KR6: u32,
    pub C0C1KR7: u32,
    pub C0C2MR: u32,
    pub C0C2KSR: u32,
    pub C0C2DSR: u64,
    pub C0C2ICVSZR: u32,
    pub C0C2CTXR0: u32,
    pub C0C2CTXR1: u32,
    pub C0C2CTXR2: u32,
    pub C0C2CTXR3: u32,
    pub C0C2CTXR4: u32,
    pub C0C2CTXR5: u32,
    pub C0C2CTXR6: u32,
    pub C0C2CTXR7: u32,
    pub C0C2CTXR8: u32,
    pub C0C2CTXR9: u32,
    pub C0C2CTXR10: u32,
    pub C0C2CTXR11: u32,
    pub C0C2CTXR12: u32,
    pub C0C2CTXR13: u32,
    pub C0C2CTXR14: u32,
    pub C0C2CTXR15: u32,
    pub C0C2CTXR16: u32,
    pub C0C2CTXR17: u32,
    pub C0C2KEYR0: u32,
    pub C0C2KEYR1: u32,
    pub C0C2KEYR2: u32,
    pub C0C2KEYR3: u32,
    pub C0C2KEYR4: u32,
    pub C0C2KEYR5: u32,
    pub C0C2KEYR6: u32,
    pub C0C2KEYR7: u32,
    pub C0C2KEYR8: u32,
    pub C0C2KEYR9: u32,
    pub C0C2KEYR10: u32,
    pub C0C2KEYR11: u32,
    pub C0C2KEYR12: u32,
    pub C0C2KEYR13: u32,
    pub C0C2KEYR14: u32,
    pub C0C2KEYR15: u32,
    pub C0C2KEYR16: u32,
    pub C0C2KEYR17: u32,
    pub C0C2KEYR18: u32,
    pub C0C2KEYR19: u32,
    pub C0C2KEYR20: u32,
    pub C0C2KEYR21: u32,
    pub C0C2KEYR22: u32,
    pub C0C2KEYR23: u32,
    pub C0C2KEYR24: u32,
    pub C0C2KEYR25: u32,
    pub C0C2KEYR26: u32,
    pub C0C2KEYR27: u32,
    pub C0C2KEYR28: u32,
    pub C0C2KEYR29: u32,
    pub C0C2KEYR30: u32,
    pub C0C2KEYR31: u32,
    pub C0FIFOSTA: u32,
    pub C0NFIFO: u32,
    pub C0IFIFO: u32,
    pub C0OFIFO: u64,
    pub D0JQCR_MS: u32,
    pub D0JQCR_LS: u32,
    pub D0DAR: u64,
    pub D0OPSTA_MS: u32,
    pub D0OPSTA_LS: u32,
    pub D0PDIDSR: u32,
    pub D0ODIDSR: u32,
    pub D0MTH0_MS: u32,
    pub D0MTH0_LS: u32,
    pub D0MTH1_MS: u32,
    pub D0MTH1_LS: u32,
    pub D0MTH2_MS: u32,
    pub D0MTH2_LS: u32,
    pub D0MTH3_MS: u32,
    pub D0MTH3_LS: u32,
    pub D0GTR0_0: u32,
    pub D0GTR0_1: u32,
    pub D0GTR0_2: u32,
    pub D0GTR0_3: u32,
    pub D0STR0_0: u32,
    pub D0STR0_1: u32,
    pub D0STR0_2: u32,
    pub D0STR0_3: u32,
    pub D0DESB0: u32,
    pub D0DESB1: u32,
    pub D0DESB2: u32,
    pub D0DESB3: u32,
    pub D0DESB4: u32,
    pub D0DESB5: u32,
    pub D0DESB6: u32,
    pub D0DESB7: u32,
    pub D0DESB8: u32,
    pub D0DESB9: u32,
    pub D0DESB10: u32,
    pub D0DESB11: u32,
    pub D0DESB12: u32,
    pub D0DESB13: u32,
    pub D0DESB14: u32,
    pub D0DESB15: u32,
    pub D0DESB16: u32,
    pub D0DESB17: u32,
    pub D0DESB18: u32,
    pub D0DESB19: u32,
    pub D0DESB20: u32,
    pub D0DESB21: u32,
    pub D0DESB22: u32,
    pub D0DESB23: u32,
    pub D0DESB24: u32,
    pub D0DESB25: u32,
    pub D0DESB26: u32,
    pub D0DESB27: u32,
    pub D0DESB28: u32,
    pub D0DESB29: u32,
    pub D0DESB30: u32,
    pub D0DESB31: u32,
    pub D0DESB32: u32,
    pub D0DESB33: u32,
    pub D0DESB34: u32,
    pub D0DESB35: u32,
    pub D0DESB36: u32,
    pub D0DESB37: u32,
    pub D0DESB38: u32,
    pub D0DESB39: u32,
    pub D0DESB40: u32,
    pub D0DESB41: u32,
    pub D0DESB42: u32,
    pub D0DESB43: u32,
    pub D0DESB44: u32,
    pub D0DESB45: u32,
    pub D0DESB46: u32,
    pub D0DESB47: u32,
    pub D0DESB48: u32,
    pub D0DESB49: u32,
    pub D0DESB50: u32,
    pub D0DESB51: u32,
    pub D0DESB52: u32,
    pub D0DESB53: u32,
    pub D0DESB54: u32,
    pub D0DESB55: u32,
    pub D0DESB56: u32,
    pub D0DESB57: u32,
    pub D0DESB58: u32,
    pub D0DESB59: u32,
    pub D0DESB60: u32,
    pub D0DESB61: u32,
    pub D0DESB62: u32,
    pub D0DESB63: u32,
    pub D0DJR: u32,
    pub D0DDR: u32,
    pub D0DJP: u64,
    pub D0SDP: u64,
    pub D0DDR_MS: u32,
    pub D0DDR_LS: u32,
    pub SOL0: u32,
    pub VSOL0: u32,
    pub SIL0: u32,
    pub VSIL0: u32,
    pub D0POVRD: u32,
    pub UVSOL0: u32,
    pub UVSIL0: u32,
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
