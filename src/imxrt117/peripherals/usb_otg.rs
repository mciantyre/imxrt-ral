#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Identification register
pub mod ID {

    /// ID
    pub mod ID {
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

    /// NID
    pub mod NID {
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

    /// REVISION
    pub mod REVISION {
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

/// Hardware General
pub mod HWGENERAL {

    /// PHYW
    pub mod PHYW {
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

            /// 0b00: 8 bit wide data bus (Software non-programmable)
            pub const DATA_BUS_8: u32 = 0b00;

            /// 0b01: 16 bit wide data bus (Software non-programmable)
            pub const DATA_BUS_16: u32 = 0b01;

            /// 0b10: Reset to 8 bit wide data bus (Software programmable)
            pub const SW_RST_8: u32 = 0b10;

            /// 0b11: Reset to 16 bit wide data bus (Software programmable)
            pub const SW_RST_16: u32 = 0b11;
        }
    }

    /// PHYM
    pub mod PHYM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: UTMI/UMTI+
            pub const UTMI: u32 = 0b000;

            /// 0b001: ULPI DDR
            pub const ULPI_DDR: u32 = 0b001;

            /// 0b010: ULPI
            pub const ULPI: u32 = 0b010;

            /// 0b011: Serial Only
            pub const SERIAL: u32 = 0b011;

            /// 0b100: Software programmable - reset to UTMI/UTMI+
            pub const SW_RST_UTMI: u32 = 0b100;

            /// 0b101: Software programmable - reset to ULPI DDR
            pub const SW_RST_ULPI_DDR: u32 = 0b101;

            /// 0b110: Software programmable - reset to ULPI
            pub const SW_RST_ULPI: u32 = 0b110;

            /// 0b111: Software programmable - reset to Serial
            pub const SW_RST_SERIAL: u32 = 0b111;
        }
    }

    /// SM
    pub mod SM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No Serial Engine, always use parallel signalling.
            pub const SERIAL_ENGINE_NO: u32 = 0b00;

            /// 0b01: Serial Engine present, always use serial signalling for FS/LS.
            pub const SERIAL_ENGINE_EN: u32 = 0b01;

            /// 0b10: Software programmable - Reset to use parallel signalling for FS/LS
            pub const SW_RST_PARALLEL: u32 = 0b10;

            /// 0b11: Software programmable - Reset to use serial signalling for FS/LS
            pub const SW_RST_SERIAL_ENG: u32 = 0b11;
        }
    }
}

/// Host Hardware Parameters
pub mod HWHOST {

    /// HC
    pub mod HC {
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

            /// 0b0: Not supported
            pub const HOST_OP_DIS: u32 = 0b0;

            /// 0b1: Supported
            pub const HOST_OP_EN: u32 = 0b1;
        }
    }

    /// NPORT
    pub mod NPORT {
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
}

/// Device Hardware Parameters
pub mod HWDEVICE {

    /// DC
    pub mod DC {
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

            /// 0b0: Not supported
            pub const DEVICE_OP_DIS: u32 = 0b0;

            /// 0b1: Supported
            pub const DEVICE_OP_EN: u32 = 0b1;
        }
    }

    /// DEVEP
    pub mod DEVEP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (5 bits: 0b11111 << 1)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TX Buffer Hardware Parameters
pub mod HWTXBUF {

    /// TXBURST
    pub mod TXBURST {
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

    /// TXCHANADD
    pub mod TXCHANADD {
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

/// RX Buffer Hardware Parameters
pub mod HWRXBUF {

    /// RXBURST
    pub mod RXBURST {
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

    /// RXADD
    pub mod RXADD {
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

/// General Purpose Timer #0 Load
pub mod GPTIMER0LD {

    /// GPTLD
    pub mod GPTLD {
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

/// General Purpose Timer #0 Controller
pub mod GPTIMER0CTRL {

    /// GPTCNT
    pub mod GPTCNT {
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

    /// GPTMODE
    pub mod GPTMODE {
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

            /// 0b0: One Shot Mode
            pub const ONE_SHOT: u32 = 0b0;

            /// 0b1: Repeat Mode
            pub const REPEAT: u32 = 0b1;
        }
    }

    /// GPTRST
    pub mod GPTRST {
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

            /// 0b0: No action
            pub const NO_ACTION: u32 = 0b0;

            /// 0b1: Load counter value from GPTLD bits in n_GPTIMER0LD
            pub const LOAD_CNTR: u32 = 0b1;
        }
    }

    /// GPTRUN
    pub mod GPTRUN {
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

            /// 0b0: Stop counting
            pub const STOP_CNTR: u32 = 0b0;

            /// 0b1: Run
            pub const RUN: u32 = 0b1;
        }
    }
}

/// General Purpose Timer #1 Load
pub mod GPTIMER1LD {
    pub use super::GPTIMER0LD::GPTLD;
}

/// General Purpose Timer #1 Controller
pub mod GPTIMER1CTRL {

    /// GPTCNT
    pub mod GPTCNT {
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

    /// GPTMODE
    pub mod GPTMODE {
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

            /// 0b0: One Shot Mode
            pub const ONE_SHOT: u32 = 0b0;

            /// 0b1: Repeat Mode
            pub const REPEAT: u32 = 0b1;
        }
    }

    /// GPTRST
    pub mod GPTRST {
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

            /// 0b0: No action
            pub const NO_ACTION: u32 = 0b0;

            /// 0b1: Load counter value from GPTLD bits in USB_n_GPTIMER0LD
            pub const LOAD_CNTR: u32 = 0b1;
        }
    }

    /// GPTRUN
    pub mod GPTRUN {
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

            /// 0b0: Stop counting
            pub const STOP_CNTR: u32 = 0b0;

            /// 0b1: Run
            pub const RUN: u32 = 0b1;
        }
    }
}

/// System Bus Config
pub mod SBUSCFG {

    /// AHBBRST
    pub mod AHBBRST {
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

            /// 0b000: Incremental burst of unspecified length only
            pub const INCR_BURST: u32 = 0b000;

            /// 0b001: INCR4 burst, then single transfer
            pub const INCR4_BURST: u32 = 0b001;

            /// 0b010: INCR8 burst, INCR4 burst, then single transfer
            pub const INCR8_BURST: u32 = 0b010;

            /// 0b011: INCR16 burst, INCR8 burst, INCR4 burst, then single transfer
            pub const INCR16_BURST: u32 = 0b011;

            /// 0b101: INCR4 burst, then incremental burst of unspecified length
            pub const INCR4_UNSPEC: u32 = 0b101;

            /// 0b110: INCR8 burst, INCR4 burst, then incremental burst of unspecified length
            pub const INCR8_4_UNSPEC: u32 = 0b110;

            /// 0b111: INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length
            pub const INCR16_8_4_UNSPEC: u32 = 0b111;
        }
    }
}

/// Capability Registers Length
pub mod CAPLENGTH {

    /// CAPLENGTH
    pub mod CAPLENGTH {
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

/// Host Controller Interface Version
pub mod HCIVERSION {

    /// HCIVERSION
    pub mod HCIVERSION {
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

/// Host Controller Structural Parameters
pub mod HCSPARAMS {

    /// N_PORTS
    pub mod N_PORTS {
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

    /// PPC
    pub mod PPC {
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

    /// N_PCC
    pub mod N_PCC {
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

    /// N_CC
    pub mod N_CC {
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

            /// 0b0000: There is no internal Companion Controller and port-ownership hand-off is not supported.
            pub const NO_COMP_CONTROLLER: u32 = 0b0000;

            /// 0b0001: There are internal companion controller(s) and port-ownership hand-offs is supported.
            pub const COMP_CONTROLLER: u32 = 0b0001;
        }
    }

    /// PI
    pub mod PI {
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

    /// N_PTT
    pub mod N_PTT {
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

    /// N_TT
    pub mod N_TT {
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

/// Host Controller Capability Parameters
pub mod HCCPARAMS {

    /// ADC
    pub mod ADC {
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

    /// PFL
    pub mod PFL {
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

    /// ASP
    pub mod ASP {
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

    /// IST
    pub mod IST {
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

    /// EECP
    pub mod EECP {
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

/// Device Controller Interface Version
pub mod DCIVERSION {

    /// DCIVERSION
    pub mod DCIVERSION {
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

/// Device Controller Capability Parameters
pub mod DCCPARAMS {

    /// DEN
    pub mod DEN {
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

    /// DC
    pub mod DC {
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

    /// HC
    pub mod HC {
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
}

/// USB Command Register
pub mod USBCMD {

    /// RS
    pub mod RS {
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

    /// RST
    pub mod RST {
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

    /// FS_1
    pub mod FS_1 {
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

    /// PSE
    pub mod PSE {
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

            /// 0b0: Do not process the Periodic Schedule
            pub const DONT_PROCESS_PT: u32 = 0b0;

            /// 0b1: Use the PERIODICLISTBASE register to access the Periodic Schedule.
            pub const PROCESS_PT_PERIODICLISTBASE: u32 = 0b1;
        }
    }

    /// ASE
    pub mod ASE {
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

            /// 0b0: Do not process the Asynchronous Schedule.
            pub const DONT_PROCESS_ASYNC: u32 = 0b0;

            /// 0b1: Use the ASYNCLISTADDR register to access the Asynchronous Schedule.
            pub const ACCESS_ASYNC: u32 = 0b1;
        }
    }

    /// IAA
    pub mod IAA {
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

    /// ASP
    pub mod ASP {
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

    /// ASPE
    pub mod ASPE {
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

    /// SUTW
    pub mod SUTW {
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

    /// ATDTW
    pub mod ATDTW {
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

    /// FS_2
    pub mod FS_2 {
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

    /// ITC
    pub mod ITC {
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

            /// 0b00000000: Immediate (no threshold)
            pub const IMMEDIATE: u32 = 0b00000000;

            /// 0b00000001: 1 micro-frame
            pub const MICROFRAME_1: u32 = 0b00000001;

            /// 0b00000010: 2 micro-frames
            pub const MICROFRAME_2: u32 = 0b00000010;

            /// 0b00000100: 4 micro-frames
            pub const MICROFRAME_4: u32 = 0b00000100;

            /// 0b00001000: 8 micro-frames
            pub const MICROFRAME_8: u32 = 0b00001000;

            /// 0b00010000: 16 micro-frames
            pub const MICROFRAME_16: u32 = 0b00010000;

            /// 0b00100000: 32 micro-frames
            pub const MICROFRAME_32: u32 = 0b00100000;

            /// 0b01000000: 64 micro-frames
            pub const MICROFRAME_64: u32 = 0b01000000;
        }
    }
}

/// USB Status Register
pub mod USBSTS {

    /// UI
    pub mod UI {
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

    /// UEI
    pub mod UEI {
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

    /// PCI
    pub mod PCI {
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

    /// FRI
    pub mod FRI {
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

    /// SEI
    pub mod SEI {
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

    /// AAI
    pub mod AAI {
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

    /// URI
    pub mod URI {
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

    /// SRI
    pub mod SRI {
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

    /// SLI
    pub mod SLI {
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

    /// ULPII
    pub mod ULPII {
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

    /// HCH
    pub mod HCH {
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

    /// RCL
    pub mod RCL {
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

    /// PS
    pub mod PS {
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

    /// AS
    pub mod AS {
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

    /// NAKI
    pub mod NAKI {
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

    /// TI0
    pub mod TI0 {
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

    /// TI1
    pub mod TI1 {
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

/// Interrupt Enable Register
pub mod USBINTR {

    /// UE
    pub mod UE {
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

    /// UEE
    pub mod UEE {
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

    /// PCE
    pub mod PCE {
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

    /// FRE
    pub mod FRE {
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

    /// SEE
    pub mod SEE {
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

    /// AAE
    pub mod AAE {
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

    /// URE
    pub mod URE {
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

    /// SRE
    pub mod SRE {
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

    /// SLE
    pub mod SLE {
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

    /// ULPIE
    pub mod ULPIE {
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

    /// NAKE
    pub mod NAKE {
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

    /// UAIE
    pub mod UAIE {
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

    /// UPIE
    pub mod UPIE {
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

    /// TIE0
    pub mod TIE0 {
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

    /// TIE1
    pub mod TIE1 {
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

/// USB Frame Index
pub mod FRINDEX {

    /// FRINDEX
    pub mod FRINDEX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000: (1024) 12
            pub const FRINDEX_1024: u32 = 0b00000000000000;

            /// 0b00000000000001: (512) 11
            pub const FRINDEX_512: u32 = 0b00000000000001;

            /// 0b00000000000010: (256) 10
            pub const FRINDEX_256: u32 = 0b00000000000010;

            /// 0b00000000000011: (128) 9
            pub const FRINDEX_128: u32 = 0b00000000000011;

            /// 0b00000000000100: (64) 8
            pub const FRINDEX_64: u32 = 0b00000000000100;

            /// 0b00000000000101: (32) 7
            pub const FRINDEX_32: u32 = 0b00000000000101;

            /// 0b00000000000110: (16) 6
            pub const FRINDEX_16: u32 = 0b00000000000110;

            /// 0b00000000000111: (8) 5
            pub const FRINDEX_8: u32 = 0b00000000000111;
        }
    }
}

/// DEVICEADDR and PERIODICLISTBASE
/// DEVICEADDR: Device Address
/// PERIODICLISTBASE: Frame List Base Address
pub mod DEVICEADDR {

    /// USBADRA
    pub mod USBADRA {
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

    /// USBADR
    pub mod USBADR {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (7 bits: 0x7f << 25)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BASEADR
    pub mod BASEADR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ASYNCLISTADDR and ENDPTLISTADDR
/// ASYNCLISTADDR: Next Asynch. Address
/// ENDPTLISTADDR: Endpoint List Address
pub mod ASYNCLISTADDR {

    /// ASYBASE
    pub mod ASYBASE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (27 bits: 0x7ffffff << 5)
        pub const mask: u32 = 0x7ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPBASE
    pub mod EPBASE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (21 bits: 0x1fffff << 11)
        pub const mask: u32 = 0x1fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Programmable Burst Size
pub mod BURSTSIZE {

    /// RXPBURST
    pub mod RXPBURST {
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

    /// TXPBURST
    pub mod TXPBURST {
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
}

/// TX FIFO Fill Tuning
pub mod TXFILLTUNING {

    /// TXSCHOH
    pub mod TXSCHOH {
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

    /// TXSCHHEALTH
    pub mod TXSCHHEALTH {
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

    /// TXFIFOTHRES
    pub mod TXFIFOTHRES {
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
}

/// Endpoint NAK
pub mod ENDPTNAK {

    /// EPRN
    pub mod EPRN {
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

    /// EPTN
    pub mod EPTN {
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

/// Endpoint NAK Enable
pub mod ENDPTNAKEN {

    /// EPRNE
    pub mod EPRNE {
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

    /// EPTNE
    pub mod EPTNE {
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

/// Configure Flag Register
pub mod CONFIGFLAG {

    /// CF
    pub mod CF {
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

            /// 0b0: Port routing control logic default-routes each port to an implementation dependent classic host controller.
            pub const PORT_ROUTING_CLASSIC_HOST: u32 = 0b0;

            /// 0b1: Port routing control logic default-routes all ports to this host controller.
            pub const PORT_ROUTING_HOST: u32 = 0b1;
        }
    }
}

/// Port Status & Control
pub mod PORTSC1 {

    /// CCS
    pub mod CCS {
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

    /// CSC
    pub mod CSC {
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

    /// PE
    pub mod PE {
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

    /// PEC
    pub mod PEC {
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

    /// OCA
    pub mod OCA {
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

            /// 0b0: This port does not have an over-current condition.
            pub const NO_OVERCURRENT: u32 = 0b0;

            /// 0b1: This port currently has an over-current condition
            pub const OVERCURRENT: u32 = 0b1;
        }
    }

    /// OCC
    pub mod OCC {
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

    /// FPR
    pub mod FPR {
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

    /// SUSP
    pub mod SUSP {
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

    /// PR
    pub mod PR {
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

    /// HSP
    pub mod HSP {
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

    /// LS
    pub mod LS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: SE0
            pub const SE0: u32 = 0b00;

            /// 0b01: K-state
            pub const K_STATE: u32 = 0b01;

            /// 0b10: J-state
            pub const J_STATE: u32 = 0b10;

            /// 0b11: Undefined
            pub const UNDEFINED: u32 = 0b11;
        }
    }

    /// PP
    pub mod PP {
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

    /// PO
    pub mod PO {
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

    /// PIC
    pub mod PIC {
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

            /// 0b00: Port indicators are off
            pub const PORT_INDICATOR_OFF: u32 = 0b00;

            /// 0b01: Amber
            pub const PORT_IND_AMBER: u32 = 0b01;

            /// 0b10: Green
            pub const PORT_IND_GREEN: u32 = 0b10;

            /// 0b11: Undefined
            pub const UNDEFINED: u32 = 0b11;
        }
    }

    /// PTC
    pub mod PTC {
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

            /// 0b0000: TEST_MODE_DISABLE
            pub const TST_MODE_DIS: u32 = 0b0000;

            /// 0b0001: J_STATE
            pub const J_STATE: u32 = 0b0001;

            /// 0b0010: K_STATE
            pub const K_STATE: u32 = 0b0010;

            /// 0b0011: SE0 (host) / NAK (device)
            pub const SE0: u32 = 0b0011;

            /// 0b0100: Packet
            pub const PCKT: u32 = 0b0100;

            /// 0b0101: FORCE_ENABLE_HS
            pub const HS: u32 = 0b0101;

            /// 0b0110: FORCE_ENABLE_FS
            pub const FS: u32 = 0b0110;

            /// 0b0111: FORCE_ENABLE_LS
            pub const LS: u32 = 0b0111;
        }
    }

    /// WKCN
    pub mod WKCN {
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

    /// WKDC
    pub mod WKDC {
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

    /// WKOC
    pub mod WKOC {
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

    /// PHCD
    pub mod PHCD {
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

            /// 0b0: Enable PHY clock
            pub const PHY_CLK_EN: u32 = 0b0;

            /// 0b1: Disable PHY clock
            pub const PHY_CLK_DIS: u32 = 0b1;
        }
    }

    /// PFSC
    pub mod PFSC {
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

            /// 0b0: Normal operation
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Forced to full speed
            pub const FULL_SPEED: u32 = 0b1;
        }
    }

    /// PTS_2
    pub mod PTS_2 {
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

    /// PSPD
    pub mod PSPD {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Full Speed
            pub const FS: u32 = 0b00;

            /// 0b01: Low Speed
            pub const LS: u32 = 0b01;

            /// 0b10: High Speed
            pub const HS: u32 = 0b10;

            /// 0b11: Undefined
            pub const UNDEFINED: u32 = 0b11;
        }
    }

    /// PTW
    pub mod PTW {
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

            /// 0b0: Select the 8-bit UTMI interface \[60MHz\]
            pub const UTMI_8: u32 = 0b0;

            /// 0b1: Select the 16-bit UTMI interface \[30MHz\]
            pub const UTMI_16: u32 = 0b1;
        }
    }

    /// STS
    pub mod STS {
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

    /// PTS_1
    pub mod PTS_1 {
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

/// On-The-Go Status & control
pub mod OTGSC {

    /// VD
    pub mod VD {
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

    /// VC
    pub mod VC {
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

    /// OT
    pub mod OT {
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

    /// DP
    pub mod DP {
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

    /// IDPU
    pub mod IDPU {
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

    /// ID
    pub mod ID {
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

    /// AVV
    pub mod AVV {
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

    /// ASV
    pub mod ASV {
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

    /// BSV
    pub mod BSV {
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

    /// BSE
    pub mod BSE {
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

    /// TOG_1MS
    pub mod TOG_1MS {
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

    /// DPS
    pub mod DPS {
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

    /// IDIS
    pub mod IDIS {
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

    /// AVVIS
    pub mod AVVIS {
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

    /// ASVIS
    pub mod ASVIS {
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

    /// BSVIS
    pub mod BSVIS {
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

    /// BSEIS
    pub mod BSEIS {
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

    /// STATUS_1MS
    pub mod STATUS_1MS {
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

    /// DPIS
    pub mod DPIS {
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

    /// IDIE
    pub mod IDIE {
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

    /// AVVIE
    pub mod AVVIE {
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

    /// ASVIE
    pub mod ASVIE {
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

    /// BSVIE
    pub mod BSVIE {
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

    /// BSEIE
    pub mod BSEIE {
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

    /// EN_1MS
    pub mod EN_1MS {
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

    /// DPIE
    pub mod DPIE {
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

/// USB Device Mode
pub mod USBMODE {

    /// CM
    pub mod CM {
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

            /// 0b00: Idle \[Default for combination host/device\]
            pub const IDL: u32 = 0b00;

            /// 0b10: Device Controller \[Default for device only controller\]
            pub const DEVICE_CONTR: u32 = 0b10;

            /// 0b11: Host Controller \[Default for host only controller\]
            pub const HOST_CONTR: u32 = 0b11;
        }
    }

    /// ES
    pub mod ES {
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

            /// 0b0: Little Endian \[Default\]
            pub const LITTLE_ENDIAN: u32 = 0b0;

            /// 0b1: Big Endian
            pub const BIG_ENDIAN: u32 = 0b1;
        }
    }

    /// SLOM
    pub mod SLOM {
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

            /// 0b0: Setup Lockouts On (default);
            pub const LOCKOUT_ON: u32 = 0b0;

            /// 0b1: Setup Lockouts Off
            pub const LOCKOUT_OFF: u32 = 0b1;
        }
    }

    /// SDIS
    pub mod SDIS {
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

/// Endpoint Setup Status
pub mod ENDPTSETUPSTAT {

    /// ENDPTSETUPSTAT
    pub mod ENDPTSETUPSTAT {
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

/// Endpoint Prime
pub mod ENDPTPRIME {

    /// PERB
    pub mod PERB {
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

    /// PETB
    pub mod PETB {
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

/// Endpoint Flush
pub mod ENDPTFLUSH {

    /// FERB
    pub mod FERB {
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

    /// FETB
    pub mod FETB {
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

/// Endpoint Status
pub mod ENDPTSTAT {

    /// ERBR
    pub mod ERBR {
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

    /// ETBR
    pub mod ETBR {
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

/// Endpoint Complete
pub mod ENDPTCOMPLETE {

    /// ERCE
    pub mod ERCE {
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

    /// ETCE
    pub mod ETCE {
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

/// Endpoint Control0
pub mod ENDPTCTRL0 {

    /// RXS
    pub mod RXS {
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

    /// RXT
    pub mod RXT {
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

    /// RXE
    pub mod RXE {
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

    /// TXS
    pub mod TXS {
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

    /// TXT
    pub mod TXT {
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

    /// TXE
    pub mod TXE {
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
}

/// Endpoint Control 1
pub mod ENDPTCTRL1 {

    /// RXS
    pub mod RXS {
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

    /// RXD
    pub mod RXD {
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

    /// RXT
    pub mod RXT {
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

    /// RXI
    pub mod RXI {
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

    /// RXR
    pub mod RXR {
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

    /// RXE
    pub mod RXE {
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

    /// TXS
    pub mod TXS {
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

    /// TXD
    pub mod TXD {
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

    /// TXT
    pub mod TXT {
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

    /// TXI
    pub mod TXI {
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

    /// TXR
    pub mod TXR {
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

    /// TXE
    pub mod TXE {
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
}

/// Endpoint Control 2
pub mod ENDPTCTRL2 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control 3
pub mod ENDPTCTRL3 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control 4
pub mod ENDPTCTRL4 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control 5
pub mod ENDPTCTRL5 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control 6
pub mod ENDPTCTRL6 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control 7
pub mod ENDPTCTRL7 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Identification register
    pub ID: RORegister<u32>,

    /// Hardware General
    pub HWGENERAL: RORegister<u32>,

    /// Host Hardware Parameters
    pub HWHOST: RORegister<u32>,

    /// Device Hardware Parameters
    pub HWDEVICE: RORegister<u32>,

    /// TX Buffer Hardware Parameters
    pub HWTXBUF: RORegister<u32>,

    /// RX Buffer Hardware Parameters
    pub HWRXBUF: RORegister<u32>,

    _reserved1: [u32; 26],

    /// General Purpose Timer #0 Load
    pub GPTIMER0LD: RWRegister<u32>,

    /// General Purpose Timer #0 Controller
    pub GPTIMER0CTRL: RWRegister<u32>,

    /// General Purpose Timer #1 Load
    pub GPTIMER1LD: RWRegister<u32>,

    /// General Purpose Timer #1 Controller
    pub GPTIMER1CTRL: RWRegister<u32>,

    /// System Bus Config
    pub SBUSCFG: RWRegister<u32>,

    _reserved2: [u32; 27],

    /// Capability Registers Length
    pub CAPLENGTH: RORegister<u8>,

    _reserved3: [u8; 1],

    /// Host Controller Interface Version
    pub HCIVERSION: RORegister<u16>,

    /// Host Controller Structural Parameters
    pub HCSPARAMS: RORegister<u32>,

    /// Host Controller Capability Parameters
    pub HCCPARAMS: RORegister<u32>,

    _reserved4: [u32; 5],

    /// Device Controller Interface Version
    pub DCIVERSION: RORegister<u16>,

    _reserved5: [u16; 1],

    /// Device Controller Capability Parameters
    pub DCCPARAMS: RORegister<u32>,

    _reserved6: [u32; 6],

    /// USB Command Register
    pub USBCMD: RWRegister<u32>,

    /// USB Status Register
    pub USBSTS: RWRegister<u32>,

    /// Interrupt Enable Register
    pub USBINTR: RWRegister<u32>,

    /// USB Frame Index
    pub FRINDEX: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// DEVICEADDR and PERIODICLISTBASE
    /// DEVICEADDR: Device Address
    /// PERIODICLISTBASE: Frame List Base Address
    pub DEVICEADDR: RWRegister<u32>,

    /// ASYNCLISTADDR and ENDPTLISTADDR
    /// ASYNCLISTADDR: Next Asynch. Address
    /// ENDPTLISTADDR: Endpoint List Address
    pub ASYNCLISTADDR: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// Programmable Burst Size
    pub BURSTSIZE: RWRegister<u32>,

    /// TX FIFO Fill Tuning
    pub TXFILLTUNING: RWRegister<u32>,

    _reserved9: [u32; 4],

    /// Endpoint NAK
    pub ENDPTNAK: RWRegister<u32>,

    /// Endpoint NAK Enable
    pub ENDPTNAKEN: RWRegister<u32>,

    /// Configure Flag Register
    pub CONFIGFLAG: RORegister<u32>,

    /// Port Status & Control
    pub PORTSC1: RWRegister<u32>,

    _reserved10: [u32; 7],

    /// On-The-Go Status & control
    pub OTGSC: RWRegister<u32>,

    /// USB Device Mode
    pub USBMODE: RWRegister<u32>,

    /// Endpoint Setup Status
    pub ENDPTSETUPSTAT: RWRegister<u32>,

    /// Endpoint Prime
    pub ENDPTPRIME: RWRegister<u32>,

    /// Endpoint Flush
    pub ENDPTFLUSH: RWRegister<u32>,

    /// Endpoint Status
    pub ENDPTSTAT: RORegister<u32>,

    /// Endpoint Complete
    pub ENDPTCOMPLETE: RWRegister<u32>,

    /// Endpoint Control0
    pub ENDPTCTRL0: RWRegister<u32>,

    /// Endpoint Control 1
    pub ENDPTCTRL1: RWRegister<u32>,

    /// Endpoint Control 2
    pub ENDPTCTRL2: RWRegister<u32>,

    /// Endpoint Control 3
    pub ENDPTCTRL3: RWRegister<u32>,

    /// Endpoint Control 4
    pub ENDPTCTRL4: RWRegister<u32>,

    /// Endpoint Control 5
    pub ENDPTCTRL5: RWRegister<u32>,

    /// Endpoint Control 6
    pub ENDPTCTRL6: RWRegister<u32>,

    /// Endpoint Control 7
    pub ENDPTCTRL7: RWRegister<u32>,
}
pub struct ResetValues {
    pub ID: u32,
    pub HWGENERAL: u32,
    pub HWHOST: u32,
    pub HWDEVICE: u32,
    pub HWTXBUF: u32,
    pub HWRXBUF: u32,
    pub GPTIMER0LD: u32,
    pub GPTIMER0CTRL: u32,
    pub GPTIMER1LD: u32,
    pub GPTIMER1CTRL: u32,
    pub SBUSCFG: u32,
    pub CAPLENGTH: u8,
    pub HCIVERSION: u16,
    pub HCSPARAMS: u32,
    pub HCCPARAMS: u32,
    pub DCIVERSION: u16,
    pub DCCPARAMS: u32,
    pub USBCMD: u32,
    pub USBSTS: u32,
    pub USBINTR: u32,
    pub FRINDEX: u32,
    pub DEVICEADDR: u32,
    pub ASYNCLISTADDR: u32,
    pub BURSTSIZE: u32,
    pub TXFILLTUNING: u32,
    pub ENDPTNAK: u32,
    pub ENDPTNAKEN: u32,
    pub CONFIGFLAG: u32,
    pub PORTSC1: u32,
    pub OTGSC: u32,
    pub USBMODE: u32,
    pub ENDPTSETUPSTAT: u32,
    pub ENDPTPRIME: u32,
    pub ENDPTFLUSH: u32,
    pub ENDPTSTAT: u32,
    pub ENDPTCOMPLETE: u32,
    pub ENDPTCTRL0: u32,
    pub ENDPTCTRL1: u32,
    pub ENDPTCTRL2: u32,
    pub ENDPTCTRL3: u32,
    pub ENDPTCTRL4: u32,
    pub ENDPTCTRL5: u32,
    pub ENDPTCTRL6: u32,
    pub ENDPTCTRL7: u32,
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
