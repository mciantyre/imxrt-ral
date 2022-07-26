#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPI2C
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// Version ID
pub mod VERID {

    /// Feature Specification Number
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

            /// 0b0000000000000010: Master only, with standard feature set
            pub const MASTER_ONLY: u32 = 0b0000000000000010;

            /// 0b0000000000000011: Master and slave, with standard feature set
            pub const MASTER_AND_SLAVE: u32 = 0b0000000000000011;
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

    /// Master Transmit FIFO Size
    pub mod MTXFIFO {
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

    /// Master Receive FIFO Size
    pub mod MRXFIFO {
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

/// Master Control
pub mod MCR {

    /// Master Enable
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

            /// 0b0: Master logic is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Master logic is enabled
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

            /// 0b0: Master logic is not reset
            pub const NOT_RESET: u32 = 0b0;

            /// 0b1: Master logic is reset
            pub const RESET: u32 = 0b1;
        }
    }

    /// Doze mode enable
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

            /// 0b0: Master is enabled in Doze mode
            pub const ENABLED: u32 = 0b0;

            /// 0b1: Master is disabled in Doze mode
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

            /// 0b0: Master is disabled in debug mode
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Master is enabled in debug mode
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

            /// 0b1: Transmit FIFO is reset
            pub const RESET: u32 = 0b1;
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

            /// 0b1: Receive FIFO is reset
            pub const RESET: u32 = 0b1;
        }
    }
}

/// Master Status
pub mod MSR {

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

            /// 0b0: Transmit data is not requested
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Transmit data is requested
            pub const ENABLED: u32 = 0b1;
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
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Receive data is ready
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// End Packet Flag
    pub mod EPF {
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

            /// 0b0: Master has not generated a STOP or Repeated START condition
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Master has generated a STOP or Repeated START condition
            pub const FLAG: u32 = 0b1;
        }
    }

    /// STOP Detect Flag
    pub mod SDF {
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

            /// 0b0: Master has not generated a STOP condition
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Master has generated a STOP condition
            pub const FLAG: u32 = 0b1;
        }
    }

    /// NACK Detect Flag
    pub mod NDF {
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

            /// 0b0: Unexpected NACK was not detected
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Unexpected NACK was detected
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Arbitration Lost Flag
    pub mod ALF {
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

            /// 0b0: Master has not lost arbitration
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Master has lost arbitration
            pub const FLAG: u32 = 0b1;
        }
    }

    /// FIFO Error Flag
    pub mod FEF {
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

            /// 0b0: No error
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Master sending or receiving data without a START condition
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Pin Low Timeout Flag
    pub mod PLTF {
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

            /// 0b0: Pin low timeout has not occurred or is disabled
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Pin low timeout has occurred
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Data Match Flag
    pub mod DMF {
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

            /// 0b0: Have not received matching data
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Have received matching data
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Master Busy Flag
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

            /// 0b0: I2C Master is idle
            pub const IDLE: u32 = 0b0;

            /// 0b1: I2C Master is busy
            pub const BUSY: u32 = 0b1;
        }
    }

    /// Bus Busy Flag
    pub mod BBF {
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

            /// 0b0: I2C Bus is idle
            pub const IDLE: u32 = 0b0;

            /// 0b1: I2C Bus is busy
            pub const BUSY: u32 = 0b1;
        }
    }
}

/// Master Interrupt Enable
pub mod MIER {

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

    /// End Packet Interrupt Enable
    pub mod EPIE {
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

    /// STOP Detect Interrupt Enable
    pub mod SDIE {
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

    /// NACK Detect Interrupt Enable
    pub mod NDIE {
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

    /// Arbitration Lost Interrupt Enable
    pub mod ALIE {
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

    /// FIFO Error Interrupt Enable
    pub mod FEIE {
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

            /// 0b0: Enabled
            pub const ENABLED: u32 = 0b0;

            /// 0b1: Disabled
            pub const DISABLED: u32 = 0b1;
        }
    }

    /// Pin Low Timeout Interrupt Enable
    pub mod PLTIE {
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

    /// Data Match Interrupt Enable
    pub mod DMIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }
}

/// Master DMA Enable
pub mod MDER {

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

/// Master Configuration 0
pub mod MCFGR0 {

    /// Host Request Enable
    pub mod HREN {
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

            /// 0b0: Host request input is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Host request input is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Host Request Polarity
    pub mod HRPOL {
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

            /// 0b0: Active low
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: Active high
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// Host Request Select
    pub mod HRSEL {
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

            /// 0b0: Host request input is pin HREQ
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Host request input is input trigger
            pub const ENABLED: u32 = 0b1;
        }
    }

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

            /// 0b0: Received data is stored in the receive FIFO
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Received data is discarded unless the the Data Match Flag (MSR\[DMF\]) is set
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// Master Configuration 1
pub mod MCFGR1 {

    /// Prescaler
    pub mod PRESCALE {
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

            /// 0b000: Divide by 1
            pub const DIVIDE_BY_1: u32 = 0b000;

            /// 0b001: Divide by 2
            pub const DIVIDE_BY_2: u32 = 0b001;

            /// 0b010: Divide by 4
            pub const DIVIDE_BY_4: u32 = 0b010;

            /// 0b011: Divide by 8
            pub const DIVIDE_BY_8: u32 = 0b011;

            /// 0b100: Divide by 16
            pub const DIVIDE_BY_16: u32 = 0b100;

            /// 0b101: Divide by 32
            pub const DIVIDE_BY_32: u32 = 0b101;

            /// 0b110: Divide by 64
            pub const DIVIDE_BY_64: u32 = 0b110;

            /// 0b111: Divide by 128
            pub const DIVIDE_BY_128: u32 = 0b111;
        }
    }

    /// Automatic STOP Generation
    pub mod AUTOSTOP {
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
            pub const DISABLED: u32 = 0b0;

            /// 0b1: STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// IGNACK
    pub mod IGNACK {
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

            /// 0b0: LPI2C Master receives ACK and NACK normally
            pub const DISABLED: u32 = 0b0;

            /// 0b1: LPI2C Master treats a received NACK as if it (NACK) was an ACK
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Timeout Configuration
    pub mod TIMECFG {
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

            /// 0b0: MSR\[PLTF\] sets if SCL is low for longer than the configured timeout
            pub const IF_SCL_LOW: u32 = 0b0;

            /// 0b1: MSR\[PLTF\] sets if either SCL or SDA is low for longer than the configured timeout
            pub const IF_SCL_OR_SDA_LOW: u32 = 0b1;
        }
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

            /// 0b010: Match is enabled (1st data word equals MDMR\[MATCH0\] OR MDMR\[MATCH1\])
            pub const FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1: u32 = 0b010;

            /// 0b011: Match is enabled (any data word equals MDMR\[MATCH0\] OR MDMR\[MATCH1\])
            pub const ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1: u32 = 0b011;

            /// 0b100: Match is enabled (1st data word equals MDMR\[MATCH0\] AND 2nd data word equals MDMR\[MATCH1)
            pub const FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1: u32 = 0b100;

            /// 0b101: Match is enabled (any data word equals MDMR\[MATCH0\] AND next data word equals MDMR\[MATCH1)
            pub const ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1: u32 = 0b101;

            /// 0b110: Match is enabled (1st data word AND MDMR\[MATCH1\] equals MDMR\[MATCH0\] AND MDMR\[MATCH1\])
            pub const FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1: u32 = 0b110;

            /// 0b111: Match is enabled (any data word AND MDMR\[MATCH1\] equals MDMR\[MATCH0\] AND MDMR\[MATCH1\])
            pub const ANY_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1: u32 = 0b111;
        }
    }

    /// Pin Configuration
    pub mod PINCFG {
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

            /// 0b000: 2-pin open drain mode
            pub const OPEN_DRAIN_2_PIN: u32 = 0b000;

            /// 0b001: 2-pin output only mode (ultra-fast mode)
            pub const OUTPUT_2_PIN_ONLY: u32 = 0b001;

            /// 0b010: 2-pin push-pull mode
            pub const PUSH_PULL_2_PIN: u32 = 0b010;

            /// 0b011: 4-pin push-pull mode
            pub const PUSH_PULL_4_PIN: u32 = 0b011;

            /// 0b100: 2-pin open drain mode with separate LPI2C slave
            pub const OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE: u32 = 0b100;

            /// 0b101: 2-pin output only mode (ultra-fast mode) with separate LPI2C slave
            pub const OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE: u32 = 0b101;

            /// 0b110: 2-pin push-pull mode with separate LPI2C slave
            pub const PUSH_PULL_2_PIN_W_LPI2C_SLAVE: u32 = 0b110;

            /// 0b111: 4-pin push-pull mode (inverted outputs)
            pub const PUSH_PULL_4_PIN_W_LPI2C_SLAVE: u32 = 0b111;
        }
    }
}

/// Master Configuration 2
pub mod MCFGR2 {

    /// Bus Idle Timeout
    pub mod BUSIDLE {
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

    /// Glitch Filter SCL
    pub mod FILTSCL {
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

    /// Glitch Filter SDA
    pub mod FILTSDA {
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
}

/// Master Configuration 3
pub mod MCFGR3 {

    /// Pin Low Timeout
    pub mod PINLOW {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (12 bits: 0xfff << 8)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Master Data Match
pub mod MDMR {

    /// Match 0 Value
    pub mod MATCH0 {
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

    /// Match 1 Value
    pub mod MATCH1 {
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

/// Master Clock Configuration 0
pub mod MCCR0 {

    /// Clock Low Period
    pub mod CLKLO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock High Period
    pub mod CLKHI {
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

    /// Setup Hold Delay
    pub mod SETHOLD {
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

    /// Data Valid Delay
    pub mod DATAVD {
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
}

/// Master Clock Configuration 1
pub mod MCCR1 {
    pub use super::MCCR0::CLKHI;
    pub use super::MCCR0::CLKLO;
    pub use super::MCCR0::DATAVD;
    pub use super::MCCR0::SETHOLD;
}

/// Master FIFO Control
pub mod MFCR {

    /// Transmit FIFO Watermark
    pub mod TXWATER {
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

    /// Receive FIFO Watermark
    pub mod RXWATER {
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

/// Master FIFO Status
pub mod MFSR {

    /// Transmit FIFO Count
    pub mod TXCOUNT {
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

    /// Receive FIFO Count
    pub mod RXCOUNT {
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

/// Master Transmit Data
pub mod MTDR {

    /// Transmit Data
    pub mod DATA {
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

    /// Command Data
    pub mod CMD {
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

            /// 0b000: Transmit DATA\[7:0\]
            pub const TRANSMIT_DATA_7_THROUGH_0: u32 = 0b000;

            /// 0b001: Receive (DATA\[7:0\] + 1) bytes
            pub const RECEIVE_DATA_7_THROUGH_0_PLUS_ONE: u32 = 0b001;

            /// 0b010: Generate STOP condition
            pub const GENERATE_STOP_CONDITION: u32 = 0b010;

            /// 0b011: Receive and discard (DATA\[7:0\] + 1) bytes
            pub const RECEIVE_AND_DISCARD_DATA_7_THROUGH_0_PLUS_ONE: u32 = 0b011;

            /// 0b100: Generate (repeated) START and transmit address in DATA\[7:0\]
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0: u32 = 0b100;

            /// 0b101: Generate (repeated) START and transmit address in DATA\[7:0\]. This transfer expects a NACK to be returned.
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_EXPECT_NACK: u32 =
                0b101;

            /// 0b110: Generate (repeated) START and transmit address in DATA\[7:0\] using high speed mode
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE: u32 = 0b110;

            /// 0b111: Generate (repeated) START and transmit address in DATA\[7:0\] using high speed mode. This transfer expects a NACK to be returned.
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE_EXPECT_NACK: u32 = 0b111;
        }
    }
}

/// Master Receive Data
pub mod MRDR {

    /// Receive Data
    pub mod DATA {
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

    /// RX Empty
    pub mod RXEMPTY {
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

            /// 0b0: Receive FIFO is not empty
            pub const NOT_EMPTY: u32 = 0b0;

            /// 0b1: Receive FIFO is empty
            pub const EMPTY: u32 = 0b1;
        }
    }
}

/// Slave Control
pub mod SCR {

    /// Slave Enable
    pub mod SEN {
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

            /// 0b0: I2C Slave mode is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: I2C Slave mode is enabled
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

            /// 0b0: Slave mode logic is not reset
            pub const NOT_RESET: u32 = 0b0;

            /// 0b1: Slave mode logic is reset
            pub const RESET: u32 = 0b1;
        }
    }

    /// Filter Enable
    pub mod FILTEN {
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

            /// 0b0: Disable digital filter and output delay counter for slave mode
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable digital filter and output delay counter for slave mode
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Filter Doze Enable
    pub mod FILTDZ {
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

            /// 0b0: Filter remains enabled in Doze mode
            pub const FILTER_ENABLED: u32 = 0b0;

            /// 0b1: Filter is disabled in Doze mode
            pub const FILTER_DISABLED: u32 = 0b1;
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

            /// 0b1: Transmit Data Register is now empty
            pub const NOW_EMPTY: u32 = 0b1;
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

            /// 0b1: Receive Data Register is now empty
            pub const NOW_EMPTY: u32 = 0b1;
        }
    }
}

/// Slave Status
pub mod SSR {

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
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Transmit data is requested
            pub const FLAG: u32 = 0b1;
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

            /// 0b0: Receive data is not ready
            pub const NOT_READY: u32 = 0b0;

            /// 0b1: Receive data is ready
            pub const READY: u32 = 0b1;
        }
    }

    /// Address Valid Flag
    pub mod AVF {
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

            /// 0b0: Address Status Register is not valid
            pub const NOT_VALID: u32 = 0b0;

            /// 0b1: Address Status Register is valid
            pub const VALID: u32 = 0b1;
        }
    }

    /// Transmit ACK Flag
    pub mod TAF {
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

            /// 0b0: Transmit ACK/NACK is not required
            pub const NOT_REQUIRED: u32 = 0b0;

            /// 0b1: Transmit ACK/NACK is required
            pub const REQUIRED: u32 = 0b1;
        }
    }

    /// Repeated Start Flag
    pub mod RSF {
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

            /// 0b0: Slave has not detected a Repeated START condition
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Slave has detected a Repeated START condition
            pub const FLAG: u32 = 0b1;
        }
    }

    /// STOP Detect Flag
    pub mod SDF {
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

            /// 0b0: Slave has not detected a STOP condition
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Slave has detected a STOP condition
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Bit Error Flag
    pub mod BEF {
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

            /// 0b0: Slave has not detected a bit error
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Slave has detected a bit error
            pub const FLAG: u32 = 0b1;
        }
    }

    /// FIFO Error Flag
    pub mod FEF {
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

            /// 0b0: FIFO underflow or overflow was not detected
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: FIFO underflow or overflow was detected
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Address Match 0 Flag
    pub mod AM0F {
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

            /// 0b0: Have not received an ADDR0 matching address
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Have received an ADDR0 matching address
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Address Match 1 Flag
    pub mod AM1F {
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

            /// 0b0: Have not received an ADDR1 or ADDR0/ADDR1 range matching address
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Have received an ADDR1 or ADDR0/ADDR1 range matching address
            pub const FLAG: u32 = 0b1;
        }
    }

    /// General Call Flag
    pub mod GCF {
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

            /// 0b0: Slave has not detected the General Call Address or the General Call Address is disabled
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Slave has detected the General Call Address
            pub const FLAG: u32 = 0b1;
        }
    }

    /// SMBus Alert Response Flag
    pub mod SARF {
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

            /// 0b0: SMBus Alert Response is disabled or not detected
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: SMBus Alert Response is enabled and detected
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Slave Busy Flag
    pub mod SBF {
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

            /// 0b0: I2C Slave is idle
            pub const IDLE: u32 = 0b0;

            /// 0b1: I2C Slave is busy
            pub const BUSY: u32 = 0b1;
        }
    }

    /// Bus Busy Flag
    pub mod BBF {
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

            /// 0b0: I2C Bus is idle
            pub const IDLE: u32 = 0b0;

            /// 0b1: I2C Bus is busy
            pub const BUSY: u32 = 0b1;
        }
    }
}

/// Slave Interrupt Enable
pub mod SIER {

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

    /// Address Valid Interrupt Enable
    pub mod AVIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }

    /// Transmit ACK Interrupt Enable
    pub mod TAIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }

    /// Repeated Start Interrupt Enable
    pub mod RSIE {
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

    /// STOP Detect Interrupt Enable
    pub mod SDIE {
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

    /// Bit Error Interrupt Enable
    pub mod BEIE {
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

    /// FIFO Error Interrupt Enable
    pub mod FEIE {
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

    /// Address Match 0 Interrupt Enable
    pub mod AM0IE {
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

    /// Address Match 1 Interrupt Enable
    pub mod AM1IE {
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

    /// General Call Interrupt Enable
    pub mod GCIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }

    /// SMBus Alert Response Interrupt Enable
    pub mod SARIE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDIE::RW;
    }
}

/// Slave DMA Enable
pub mod SDER {

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

    /// Address Valid DMA Enable
    pub mod AVDE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDDE::RW;
    }
}

/// Slave Configuration 1
pub mod SCFGR1 {

    /// Address SCL Stall
    pub mod ADRSTALL {
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

            /// 0b0: Clock stretching is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Clock stretching is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// RX SCL Stall
    pub mod RXSTALL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADRSTALL::RW;
    }

    /// TX Data SCL Stall
    pub mod TXDSTALL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADRSTALL::RW;
    }

    /// ACK SCL Stall
    pub mod ACKSTALL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADRSTALL::RW;
    }

    /// General Call Enable
    pub mod GCEN {
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

            /// 0b0: General Call address is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: General Call address is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// SMBus Alert Enable
    pub mod SAEN {
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

            /// 0b0: Disables match on SMBus Alert
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enables match on SMBus Alert
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Flag Configuration
    pub mod TXCFG {
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

            /// 0b0: Transmit Data Flag only asserts during a slave-transmit transfer when the Transmit Data register is empty
            pub const ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY: u32 = 0b0;

            /// 0b1: Transmit Data Flag asserts whenever the Transmit Data register is empty
            pub const ASSERTS_WHEN_TX_DATA_EMPTY: u32 = 0b1;
        }
    }

    /// Receive Data Configuration
    pub mod RXCFG {
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

            /// 0b0: Reading the Receive Data register returns received data and clears the Receive Data flag (MSR\[RDF\]).
            pub const RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG: u32 = 0b0;

            /// 0b1: Reading the Receive Data register when the Address Valid flag (SSR\[AVF\])is set, returns the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, returns received data and clears the Receive Data flag (MSR\[RDF\]).
            pub const WHEN_ADDRESS_VALID_FLAG_SET_RETURNS_ADDRESS_STATUS_AND_CLEARS_ADDRESS_VALID_FLAG: u32 = 0b1;
        }
    }

    /// Ignore NACK
    pub mod IGNACK {
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

            /// 0b0: Slave ends transfer when NACK is detected
            pub const ENDS_TRANSFER_ON_NACK: u32 = 0b0;

            /// 0b1: Slave does not end transfer when NACK detected
            pub const DOES_NOT_END_TRANSFER_ON_NACK: u32 = 0b1;
        }
    }

    /// High Speed Mode Enable
    pub mod HSMEN {
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

            /// 0b0: Disables detection of HS-mode master code
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Enables detection of HS-mode master code
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Address Configuration
    pub mod ADDRCFG {
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

            /// 0b000: Address match 0 (7-bit)
            pub const ADDRESS_MATCH0_7_BIT: u32 = 0b000;

            /// 0b001: Address match 0 (10-bit)
            pub const ADDRESS_MATCH0_10_BIT: u32 = 0b001;

            /// 0b010: Address match 0 (7-bit) or Address match 1 (7-bit)
            pub const ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT: u32 = 0b010;

            /// 0b011: Address match 0 (10-bit) or Address match 1 (10-bit)
            pub const ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT: u32 = 0b011;

            /// 0b100: Address match 0 (7-bit) or Address match 1 (10-bit)
            pub const ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT: u32 = 0b100;

            /// 0b101: Address match 0 (10-bit) or Address match 1 (7-bit)
            pub const ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT: u32 = 0b101;

            /// 0b110: From Address match 0 (7-bit) to Address match 1 (7-bit)
            pub const FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT: u32 = 0b110;

            /// 0b111: From Address match 0 (10-bit) to Address match 1 (10-bit)
            pub const FROM_ADDRESS_MATCH0_10_BIT_TO_ADDRESS_MATCH1_10_BIT: u32 = 0b111;
        }
    }
}

/// Slave Configuration 2
pub mod SCFGR2 {

    /// Clock Hold Time
    pub mod CLKHOLD {
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

    /// Data Valid Delay
    pub mod DATAVD {
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

    /// Glitch Filter SCL
    pub mod FILTSCL {
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

    /// Glitch Filter SDA
    pub mod FILTSDA {
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
}

/// Slave Address Match
pub mod SAMR {

    /// Address 0 Value
    pub mod ADDR0 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (10 bits: 0x3ff << 1)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Address 1 Value
    pub mod ADDR1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (10 bits: 0x3ff << 17)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Slave Address Status
pub mod SASR {

    /// Received Address
    pub mod RADDR {
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

    /// Address Not Valid
    pub mod ANV {
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

            /// 0b0: Received Address (RADDR) is valid
            pub const VALID: u32 = 0b0;

            /// 0b1: Received Address (RADDR) is not valid
            pub const NOT_VALID: u32 = 0b1;
        }
    }
}

/// Slave Transmit ACK
pub mod STAR {

    /// Transmit NACK
    pub mod TXNACK {
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

            /// 0b0: Write a Transmit ACK for each received word
            pub const TRANSMIT_ACK: u32 = 0b0;

            /// 0b1: Write a Transmit NACK for each received word
            pub const TRANSMIT_NACK: u32 = 0b1;
        }
    }
}

/// Slave Transmit Data
pub mod STDR {

    /// Transmit Data
    pub mod DATA {
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

/// Slave Receive Data
pub mod SRDR {

    /// Receive Data
    pub mod DATA {
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

    /// RX Empty
    pub mod RXEMPTY {
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

            /// 0b0: The Receive Data Register is not empty
            pub const NOT_EMPTY: u32 = 0b0;

            /// 0b1: The Receive Data Register is empty
            pub const EMPTY: u32 = 0b1;
        }
    }

    /// Start Of Frame
    pub mod SOF {
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

            /// 0b0: Indicates this is not the first data word since a (repeated) START or STOP condition
            pub const NOT_FIRST_DATA_WORD: u32 = 0b0;

            /// 0b1: Indicates this is the first data word since a (repeated) START or STOP condition
            pub const FIRST_DATA_WORD: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Version ID
    pub VERID: RORegister<u32>,

    /// Parameter
    pub PARAM: RORegister<u32>,

    _reserved1: [u32; 2],

    /// Master Control
    pub MCR: RWRegister<u32>,

    /// Master Status
    pub MSR: RWRegister<u32>,

    /// Master Interrupt Enable
    pub MIER: RWRegister<u32>,

    /// Master DMA Enable
    pub MDER: RWRegister<u32>,

    /// Master Configuration 0
    pub MCFGR0: RWRegister<u32>,

    /// Master Configuration 1
    pub MCFGR1: RWRegister<u32>,

    /// Master Configuration 2
    pub MCFGR2: RWRegister<u32>,

    /// Master Configuration 3
    pub MCFGR3: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// Master Data Match
    pub MDMR: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// Master Clock Configuration 0
    pub MCCR0: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Master Clock Configuration 1
    pub MCCR1: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// Master FIFO Control
    pub MFCR: RWRegister<u32>,

    /// Master FIFO Status
    pub MFSR: RORegister<u32>,

    /// Master Transmit Data
    pub MTDR: WORegister<u32>,

    _reserved6: [u32; 3],

    /// Master Receive Data
    pub MRDR: RORegister<u32>,

    _reserved7: [u32; 39],

    /// Slave Control
    pub SCR: RWRegister<u32>,

    /// Slave Status
    pub SSR: RWRegister<u32>,

    /// Slave Interrupt Enable
    pub SIER: RWRegister<u32>,

    /// Slave DMA Enable
    pub SDER: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// Slave Configuration 1
    pub SCFGR1: RWRegister<u32>,

    /// Slave Configuration 2
    pub SCFGR2: RWRegister<u32>,

    _reserved9: [u32; 5],

    /// Slave Address Match
    pub SAMR: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// Slave Address Status
    pub SASR: RORegister<u32>,

    /// Slave Transmit ACK
    pub STAR: RWRegister<u32>,

    _reserved11: [u32; 2],

    /// Slave Transmit Data
    pub STDR: WORegister<u32>,

    _reserved12: [u32; 3],

    /// Slave Receive Data
    pub SRDR: RORegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub MCR: u32,
    pub MSR: u32,
    pub MIER: u32,
    pub MDER: u32,
    pub MCFGR0: u32,
    pub MCFGR1: u32,
    pub MCFGR2: u32,
    pub MCFGR3: u32,
    pub MDMR: u32,
    pub MCCR0: u32,
    pub MCCR1: u32,
    pub MFCR: u32,
    pub MFSR: u32,
    pub MTDR: u32,
    pub MRDR: u32,
    pub SCR: u32,
    pub SSR: u32,
    pub SIER: u32,
    pub SDER: u32,
    pub SCFGR1: u32,
    pub SCFGR2: u32,
    pub SAMR: u32,
    pub SASR: u32,
    pub STAR: u32,
    pub STDR: u32,
    pub SRDR: u32,
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
