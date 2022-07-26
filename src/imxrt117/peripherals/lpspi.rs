#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPSPI
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// Version ID
pub mod VERID {

    /// Module Identification Number
    pub mod FEATURE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000100: Standard feature set supporting a 32-bit shift register.
            pub const STANDARD: u32 = 0b0000000000000100;
        }
    }

    /// Minor Version Number
    pub mod MINOR {
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

    /// Major Version Number
    pub mod MAJOR {
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

/// Parameter
pub mod PARAM {

    /// Transmit FIFO Size
    pub mod TXFIFO {
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

    /// Receive FIFO Size
    pub mod RXFIFO {
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

    /// PCS Number
    pub mod PCSNUM {
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

/// Control
pub mod CR {

    /// Module Enable
    pub mod MEN {
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

            /// 0b0: Module is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Module is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod RST {
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

            /// 0b0: Module is not reset
            pub const NOT_RESET: u32 = 0b0;

            /// 0b1: Module is reset
            pub const RESET: u32 = 0b1;
        }
    }

    /// Doze Mode Enable
    pub mod DOZEN {
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

            /// 0b0: LPSPI module is enabled in Doze mode
            pub const ENABLED: u32 = 0b0;

            /// 0b1: LPSPI module is disabled in Doze mode
            pub const DISABLED: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBGEN {
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

            /// 0b0: LPSPI module is disabled in debug mode
            pub const DISABLED: u32 = 0b0;

            /// 0b1: LPSPI module is enabled in debug mode
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Reset Transmit FIFO
    pub mod RTF {
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

            /// 0b0: No effect
            pub const NO_EFFECT: u32 = 0b0;

            /// 0b1: Reset the Transmit FIFO. The register bit always reads zero.
            pub const TXFIFO_RST: u32 = 0b1;
        }
    }

    /// Reset Receive FIFO
    pub mod RRF {
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

            /// 0b0: No effect
            pub const NO_EFFECT: u32 = 0b0;

            /// 0b1: Reset the Receive FIFO. The register bit always reads zero.
            pub const RXFIFO_RST: u32 = 0b1;
        }
    }
}

/// Status
pub mod SR {

    /// Transmit Data Flag
    pub mod TDF {
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

            /// 0b0: Transmit data not requested
            pub const TXDATA_NOT_REQST: u32 = 0b0;

            /// 0b1: Transmit data is requested
            pub const TXDATA_REQST: u32 = 0b1;
        }
    }

    /// Receive Data Flag
    pub mod RDF {
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

            /// 0b0: Receive Data is not ready
            pub const NOTREADY: u32 = 0b0;

            /// 0b1: Receive data is ready
            pub const READY: u32 = 0b1;
        }
    }

    /// Word Complete Flag
    pub mod WCF {
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

            /// 0b0: Transfer of a received word has not yet completed
            pub const NOT_COMPLETED: u32 = 0b0;

            /// 0b1: Transfer of a received word has completed
            pub const COMPLETED: u32 = 0b1;
        }
    }

    /// Frame Complete Flag
    pub mod FCF {
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

            /// 0b0: Frame transfer has not completed
            pub const NOT_COMPLETED: u32 = 0b0;

            /// 0b1: Frame transfer has completed
            pub const COMPLETED: u32 = 0b1;
        }
    }

    /// Transfer Complete Flag
    pub mod TCF {
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

            /// 0b0: All transfers have not completed
            pub const NOT_COMPLETED: u32 = 0b0;

            /// 0b1: All transfers have completed
            pub const COMPLETED: u32 = 0b1;
        }
    }

    /// Transmit Error Flag
    pub mod TEF {
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

            /// 0b0: Transmit FIFO underrun has not occurred
            pub const NO_UNDERRUN: u32 = 0b0;

            /// 0b1: Transmit FIFO underrun has occurred
            pub const UNDERRUN: u32 = 0b1;
        }
    }

    /// Receive Error Flag
    pub mod REF {
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

            /// 0b0: Receive FIFO has not overflowed
            pub const NOT_OVERFLOWED: u32 = 0b0;

            /// 0b1: Receive FIFO has overflowed
            pub const OVERFLOWED: u32 = 0b1;
        }
    }

    /// Data Match Flag
    pub mod DMF {
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

            /// 0b0: Have not received matching data
            pub const NO_MATCH: u32 = 0b0;

            /// 0b1: Have received matching data
            pub const MATCH: u32 = 0b1;
        }
    }

    /// Module Busy Flag
    pub mod MBF {
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

            /// 0b0: LPSPI is idle
            pub const IDLE: u32 = 0b0;

            /// 0b1: LPSPI is busy
            pub const BUSY: u32 = 0b1;
        }
    }
}

/// Interrupt Enable
pub mod IER {

    /// Transmit Data Interrupt Enable
    pub mod TDIE {
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

            /// 0b0: Disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Receive Data Interrupt Enable
    pub mod RDIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }

    /// Word Complete Interrupt Enable
    pub mod WCIE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }

    /// Frame Complete Interrupt Enable
    pub mod FCIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }

    /// Transfer Complete Interrupt Enable
    pub mod TCIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }

    /// Transmit Error Interrupt Enable
    pub mod TEIE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }

    /// Receive Error Interrupt Enable
    pub mod REIE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }

    /// Data Match Interrupt Enable
    pub mod DMIE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }
}

/// DMA Enable
pub mod DER {

    /// Transmit Data DMA Enable
    pub mod TDDE {
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

            /// 0b0: DMA request is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: DMA request is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Receive Data DMA Enable
    pub mod RDDE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDDE::RW;
    }
}

/// Configuration 0
pub mod CFGR0 {

    /// Circular FIFO Enable
    pub mod CIRFIFO {
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

            /// 0b0: Circular FIFO is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Circular FIFO is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Receive Data Match Only
    pub mod RDMO {
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

            /// 0b0: Received data is stored in the receive FIFO as in normal operations
            pub const STORED: u32 = 0b0;

            /// 0b1: Received data is discarded unless the SR\[DMF\] = 1
            pub const DISCARDED: u32 = 0b1;
        }
    }
}

/// Configuration 1
pub mod CFGR1 {

    /// Master Mode
    pub mod MASTER {
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

            /// 0b0: Slave mode
            pub const SLAVE_MODE: u32 = 0b0;

            /// 0b1: Master mode
            pub const MASTER_MODE: u32 = 0b1;
        }
    }

    /// Sample Point
    pub mod SAMPLE {
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

            /// 0b0: Input data is sampled on SCK edge
            pub const ON_SCK_EDGE: u32 = 0b0;

            /// 0b1: Input data is sampled on delayed SCK edge
            pub const ON_DELAYED_SCK_EDGE: u32 = 0b1;
        }
    }

    /// Automatic PCS
    pub mod AUTOPCS {
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

            /// 0b0: Automatic PCS generation is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Automatic PCS generation is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// No Stall
    pub mod NOSTALL {
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

            /// 0b0: Transfers stall when the transmit FIFO is empty
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Transfers do not stall, allowing transmit FIFO underruns to occur
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Peripheral Chip Select Polarity
    pub mod PCSPOL {
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

    /// Match Configuration
    pub mod MATCFG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Match is disabled
            pub const DISABLED: u32 = 0b000;

            /// 0b010: Match is enabled is 1st data word is MATCH0 or MATCH1
            pub const ENABLED_FIRSTDATAMATCH: u32 = 0b010;

            /// 0b011: Match is enabled on any data word equal MATCH0 or MATCH1
            pub const ENABLED_ANYDATAMATCH: u32 = 0b011;

            /// 0b100: Match is enabled on data match sequence
            pub const ENABLED_DATAMATCH_100: u32 = 0b100;

            /// 0b101: Match is enabled on data match sequence
            pub const ENABLED_DATAMATCH_101: u32 = 0b101;

            /// 0b110: Match is enabled
            pub const ENABLED_DATAMATCH_110: u32 = 0b110;

            /// 0b111: Match is enabled
            pub const ENABLED_DATAMATCH_111: u32 = 0b111;
        }
    }

    /// Pin Configuration
    pub mod PINCFG {
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

            /// 0b00: SIN is used for input data and SOUT is used for output data
            pub const SIN_IN_SOUT_OUT: u32 = 0b00;

            /// 0b01: SIN is used for both input and output data, only half-duplex serial transfers are supported
            pub const SIN_BOTH_IN_OUT: u32 = 0b01;

            /// 0b10: SOUT is used for both input and output data, only half-duplex serial transfers are supported
            pub const SOUT_BOTH_IN_OUT: u32 = 0b10;

            /// 0b11: SOUT is used for input data and SIN is used for output data
            pub const SOUT_IN_SIN_OUT: u32 = 0b11;
        }
    }

    /// Output Configuration
    pub mod OUTCFG {
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

            /// 0b0: Output data retains last value when chip select is negated
            pub const RETAIN_LASTVALUE: u32 = 0b0;

            /// 0b1: Output data is tristated when chip select is negated
            pub const TRISTATED: u32 = 0b1;
        }
    }

    /// Peripheral Chip Select Configuration
    pub mod PCSCFG {
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

            /// 0b0: PCS\[3:2\] are configured for chip select function
            pub const CHIP_SELECT: u32 = 0b0;

            /// 0b1: PCS\[3:2\] are configured for half-duplex 4-bit transfers (PCS\[3:2\] = DATA\[3:2\])
            pub const HALFDUPLEX4BIT: u32 = 0b1;
        }
    }
}

/// Data Match 0
pub mod DMR0 {

    /// Match 0 Value
    pub mod MATCH0 {
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

/// Data Match 1
pub mod DMR1 {

    /// Match 1 Value
    pub mod MATCH1 {
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

/// Clock Configuration
pub mod CCR {

    /// SCK Divider
    pub mod SCKDIV {
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

    /// Delay Between Transfers
    pub mod DBT {
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

    /// PCS-to-SCK Delay
    pub mod PCSSCK {
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

    /// SCK-to-PCS Delay
    pub mod SCKPCS {
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

/// FIFO Control
pub mod FCR {

    /// Transmit FIFO Watermark
    pub mod TXWATER {
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

    /// Receive FIFO Watermark
    pub mod RXWATER {
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

/// FIFO Status
pub mod FSR {

    /// Transmit FIFO Count
    pub mod TXCOUNT {
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

    /// Receive FIFO Count
    pub mod RXCOUNT {
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

/// Transmit Command
pub mod TCR {

    /// Frame Size
    pub mod FRAMESZ {
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

    /// Transfer Width
    pub mod WIDTH {
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

            /// 0b00: 1 bit transfer
            pub const ONEBIT: u32 = 0b00;

            /// 0b01: 2 bit transfer
            pub const TWOBIT: u32 = 0b01;

            /// 0b10: 4 bit transfer
            pub const FOURBIT: u32 = 0b10;
        }
    }

    /// Transmit Data Mask
    pub mod TXMSK {
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

            /// 0b0: Normal transfer
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Mask transmit data
            pub const MASK: u32 = 0b1;
        }
    }

    /// Receive Data Mask
    pub mod RXMSK {
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

            /// 0b0: Normal transfer
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Receive data is masked
            pub const MASK: u32 = 0b1;
        }
    }

    /// Continuing Command
    pub mod CONTC {
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

            /// 0b0: Command word for start of new transfer
            pub const START: u32 = 0b0;

            /// 0b1: Command word for continuing transfer
            pub const CONTINUE: u32 = 0b1;
        }
    }

    /// Continuous Transfer
    pub mod CONT {
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

            /// 0b0: Continuous transfer is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Continuous transfer is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Byte Swap
    pub mod BYSW {
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

            /// 0b0: Byte swap is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Byte swap is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// LSB First
    pub mod LSBF {
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

            /// 0b0: Data is transferred MSB first
            pub const MSB_FIRST: u32 = 0b0;

            /// 0b1: Data is transferred LSB first
            pub const LSB_FIRST: u32 = 0b1;
        }
    }

    /// Peripheral Chip Select
    pub mod PCS {
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

            /// 0b00: Transfer using PCS\[0\]
            pub const TX_PCS0: u32 = 0b00;

            /// 0b01: Transfer using PCS\[1\]
            pub const TX_PCS1: u32 = 0b01;

            /// 0b10: Transfer using PCS\[2\]
            pub const TX_PCS2: u32 = 0b10;

            /// 0b11: Transfer using PCS\[3\]
            pub const TX_PCS3: u32 = 0b11;
        }
    }

    /// Prescaler Value
    pub mod PRESCALE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (3 bits: 0b111 << 27)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Divide by 1
            pub const DIVIDEBY1: u32 = 0b000;

            /// 0b001: Divide by 2
            pub const DIVIDEBY2: u32 = 0b001;

            /// 0b010: Divide by 4
            pub const DIVIDEBY4: u32 = 0b010;

            /// 0b011: Divide by 8
            pub const DIVIDEBY8: u32 = 0b011;

            /// 0b100: Divide by 16
            pub const DIVIDEBY16: u32 = 0b100;

            /// 0b101: Divide by 32
            pub const DIVIDEBY32: u32 = 0b101;

            /// 0b110: Divide by 64
            pub const DIVIDEBY64: u32 = 0b110;

            /// 0b111: Divide by 128
            pub const DIVIDEBY128: u32 = 0b111;
        }
    }

    /// Clock Phase
    pub mod CPHA {
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

            /// 0b0: Captured
            pub const CAPTURED: u32 = 0b0;

            /// 0b1: Changed
            pub const CHANGED: u32 = 0b1;
        }
    }

    /// Clock Polarity
    pub mod CPOL {
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

            /// 0b0: The inactive state value of SCK is low
            pub const INACTIVE_LOW: u32 = 0b0;

            /// 0b1: The inactive state value of SCK is high
            pub const INACTIVE_HIGH: u32 = 0b1;
        }
    }
}

/// Transmit Data
pub mod TDR {

    /// Transmit Data
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

/// Receive Status
pub mod RSR {

    /// Start Of Frame
    pub mod SOF {
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

            /// 0b0: Subsequent data word received after PCS assertion
            pub const NEXT_DATAWORD: u32 = 0b0;

            /// 0b1: First data word received after PCS assertion
            pub const FIRST_DATAWORD: u32 = 0b1;
        }
    }

    /// RX FIFO Empty
    pub mod RXEMPTY {
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

            /// 0b0: RX FIFO is not empty
            pub const NOT_EMPTY: u32 = 0b0;

            /// 0b1: RX FIFO is empty
            pub const EMPTY: u32 = 0b1;
        }
    }
}

/// Receive Data
pub mod RDR {

    /// Receive Data
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
#[repr(C)]
pub struct RegisterBlock {
    /// Version ID
    pub VERID: RORegister<u32>,

    /// Parameter
    pub PARAM: RORegister<u32>,

    _reserved1: [u32; 2],

    /// Control
    pub CR: RWRegister<u32>,

    /// Status
    pub SR: RWRegister<u32>,

    /// Interrupt Enable
    pub IER: RWRegister<u32>,

    /// DMA Enable
    pub DER: RWRegister<u32>,

    /// Configuration 0
    pub CFGR0: RWRegister<u32>,

    /// Configuration 1
    pub CFGR1: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// Data Match 0
    pub DMR0: RWRegister<u32>,

    /// Data Match 1
    pub DMR1: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// Clock Configuration
    pub CCR: RWRegister<u32>,

    _reserved4: [u32; 5],

    /// FIFO Control
    pub FCR: RWRegister<u32>,

    /// FIFO Status
    pub FSR: RORegister<u32>,

    /// Transmit Command
    pub TCR: RWRegister<u32>,

    /// Transmit Data
    pub TDR: WORegister<u32>,

    _reserved5: [u32; 2],

    /// Receive Status
    pub RSR: RORegister<u32>,

    /// Receive Data
    pub RDR: RORegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub CR: u32,
    pub SR: u32,
    pub IER: u32,
    pub DER: u32,
    pub CFGR0: u32,
    pub CFGR1: u32,
    pub DMR0: u32,
    pub DMR1: u32,
    pub CCR: u32,
    pub FCR: u32,
    pub FSR: u32,
    pub TCR: u32,
    pub TDR: u32,
    pub RSR: u32,
    pub RDR: u32,
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
