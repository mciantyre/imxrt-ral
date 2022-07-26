#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! QDC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Control Register
pub mod CTRL {

    /// Compare Interrupt Enable
    pub mod CMPIE {
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

            /// 0b0: Disabled
            pub const CMPIE_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const CMPIE_1: u16 = 0b1;
        }
    }

    /// Compare Interrupt Request
    pub mod CMPIRQ {
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

            /// 0b0: No match has occurred (the counter does not match the COMP value)
            pub const CMPIRQ_0: u16 = 0b0;

            /// 0b1: COMP match has occurred (the counter matches the COMP value)
            pub const CMPIRQ_1: u16 = 0b1;
        }
    }

    /// Watchdog Enable
    pub mod WDE {
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

            /// 0b0: Disabled
            pub const WDE_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const WDE_1: u16 = 0b1;
        }
    }

    /// Watchdog Timeout Interrupt Enable
    pub mod DIE {
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

            /// 0b0: Disabled
            pub const DIE_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const DIE_1: u16 = 0b1;
        }
    }

    /// Watchdog Timeout Interrupt Request
    pub mod DIRQ {
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

            /// 0b0: No Watchdog timeout interrupt has occurred
            pub const DIRQ_0: u16 = 0b0;

            /// 0b1: Watchdog timeout interrupt has occurred
            pub const DIRQ_1: u16 = 0b1;
        }
    }

    /// Use Negative Edge of INDEX Pulse
    pub mod XNE {
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

            /// 0b0: Use positive edge of INDEX pulse
            pub const XNE_0: u16 = 0b0;

            /// 0b1: Use negative edge of INDEX pulse
            pub const XNE_1: u16 = 0b1;
        }
    }

    /// INDEX Triggered Initialization of Position Counters UPOS and LPOS
    pub mod XIP {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: INDEX pulse does not initialize the position counter
            pub const XIP_0: u16 = 0b0;

            /// 0b1: INDEX pulse initializes the position counter
            pub const XIP_1: u16 = 0b1;
        }
    }

    /// INDEX Pulse Interrupt Enable
    pub mod XIE {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const XIE_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const XIE_1: u16 = 0b1;
        }
    }

    /// INDEX Pulse Interrupt Request
    pub mod XIRQ {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: INDEX pulse has not occurred
            pub const XIRQ_0: u16 = 0b0;

            /// 0b1: INDEX pulse has occurred
            pub const XIRQ_1: u16 = 0b1;
        }
    }

    /// Enable Signal Phase Count Mode
    pub mod PH1 {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Use the standard quadrature decoder, where PHASEA and PHASEB represent a two-phase quadrature signal.
            pub const PH1_0: u16 = 0b0;

            /// 0b1: Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction: If CTRL\[REV\] = 0, PHASEB = 0, then count up If CTRL\[REV\] = 1, PHASEB = 1, then count up If CTRL\[REV\] = 0, PHASEB = 1, then count down If CTRL\[REV\] = 1, PHASEB = 0, then count down
            pub const PH1_1: u16 = 0b1;
        }
    }

    /// Enable Reverse Direction Counting
    pub mod REV {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Count normally
            pub const REV_0: u16 = 0b0;

            /// 0b1: Count in the reverse direction
            pub const REV_1: u16 = 0b1;
        }
    }

    /// Software-Triggered Initialization of Position Counters UPOS and LPOS
    pub mod SWIP {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No action
            pub const SWIP_0: u16 = 0b0;

            /// 0b1: Initialize position counter (using upper and lower initialization registers, UINIT and LINIT)
            pub const SWIP_1: u16 = 0b1;
        }
    }

    /// Use Negative Edge of HOME Input
    pub mod HNE {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Use positive-going edge-to-trigger initialization of position counters UPOS and LPOS
            pub const HNE_0: u16 = 0b0;

            /// 0b1: Use negative-going edge-to-trigger initialization of position counters UPOS and LPOS
            pub const HNE_1: u16 = 0b1;
        }
    }

    /// Enable HOME to Initialize Position Counters UPOS and LPOS
    pub mod HIP {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No action
            pub const HIP_0: u16 = 0b0;

            /// 0b1: HOME signal initializes the position counter
            pub const HIP_1: u16 = 0b1;
        }
    }

    /// HOME Interrupt Enable
    pub mod HIE {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const HIE_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const HIE_1: u16 = 0b1;
        }
    }

    /// HOME Signal Transition Interrupt Request
    pub mod HIRQ {
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

            /// 0b0: No transition on the HOME signal has occurred
            pub const HIRQ_0: u16 = 0b0;

            /// 0b1: A transition on the HOME signal has occurred
            pub const HIRQ_1: u16 = 0b1;
        }
    }
}

/// Input Filter Register
pub mod FILT {

    /// Input Filter Sample Period
    pub mod FILT_PER {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input Filter Sample Count
    pub mod FILT_CNT {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// prescaler divide IPbus clock to FILT clk
    pub mod FILT_PRSC {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Watchdog Timeout Register
pub mod WTR {

    /// WDOG
    pub mod WDOG {
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

/// Position Difference Counter Register
pub mod POSD {

    /// POSD
    pub mod POSD {
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

/// Position Difference Hold Register
pub mod POSDH {

    /// POSDH
    pub mod POSDH {
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

/// Revolution Counter Register
pub mod REV {

    /// REV
    pub mod REV {
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

/// Revolution Hold Register
pub mod REVH {

    /// REVH
    pub mod REVH {
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

/// Upper Position Counter Register
pub mod UPOS {

    /// POS
    pub mod POS {
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

/// Lower Position Counter Register
pub mod LPOS {
    pub use super::UPOS::POS;
}

/// Upper Position Hold Register
pub mod UPOSH {

    /// POSH
    pub mod POSH {
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

/// Lower Position Hold Register
pub mod LPOSH {
    pub use super::UPOSH::POSH;
}

/// Upper Initialization Register
pub mod UINIT {

    /// INIT
    pub mod INIT {
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

/// Lower Initialization Register
pub mod LINIT {
    pub use super::UINIT::INIT;
}

/// Input Monitor Register
pub mod IMR {

    /// HOME
    pub mod HOME {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INDEX
    pub mod INDEX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PHB
    pub mod PHB {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PHA
    pub mod PHA {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FHOM
    pub mod FHOM {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIND
    pub mod FIND {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FPHB
    pub mod FPHB {
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

    /// FPHA
    pub mod FPHA {
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
}

/// Test Register
pub mod TST {

    /// TEST_COUNT
    pub mod TEST_COUNT {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TEST_PERIOD
    pub mod TEST_PERIOD {
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

    /// Quadrature Decoder Negative Signal
    pub mod QDN {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Generates a positive quadrature decoder signal
            pub const QDN_0: u16 = 0b0;

            /// 0b1: Generates a negative quadrature decoder signal
            pub const QDN_1: u16 = 0b1;
        }
    }

    /// Test Counter Enable
    pub mod TCE {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const TCE_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const TCE_1: u16 = 0b1;
        }
    }

    /// Test Mode Enable
    pub mod TEN {
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

            /// 0b0: Disabled
            pub const TEN_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const TEN_1: u16 = 0b1;
        }
    }
}

/// Control 2 Register
pub mod CTRL2 {

    /// Update Hold Registers
    pub mod UPDHLD {
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

            /// 0b0: Disable updates of hold registers on the rising edge of TRIGGER input signal
            pub const UPDHLD_0: u16 = 0b0;

            /// 0b1: Enable updates of hold registers on the rising edge of TRIGGER input signal
            pub const UPDHLD_1: u16 = 0b1;
        }
    }

    /// Update Position Registers
    pub mod UPDPOS {
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

            /// 0b0: No action for POSD, REV, UPOS and LPOS registers on rising edge of TRIGGER
            pub const UPDPOS_0: u16 = 0b0;

            /// 0b1: Clear POSD, REV, UPOS and LPOS registers on rising edge of TRIGGER
            pub const UPDPOS_1: u16 = 0b1;
        }
    }

    /// Enable Modulo Counting
    pub mod MOD {
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

            /// 0b0: Disable modulo counting
            pub const MOD_0: u16 = 0b0;

            /// 0b1: Enable modulo counting
            pub const MOD_1: u16 = 0b1;
        }
    }

    /// Count Direction Flag
    pub mod DIR {
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

            /// 0b0: Last count was in the down direction
            pub const DIR_0: u16 = 0b0;

            /// 0b1: Last count was in the up direction
            pub const DIR_1: u16 = 0b1;
        }
    }

    /// Roll-under Interrupt Enable
    pub mod RUIE {
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

            /// 0b0: Disabled
            pub const RUIE_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const RUIE_1: u16 = 0b1;
        }
    }

    /// Roll-under Interrupt Request
    pub mod RUIRQ {
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

            /// 0b0: No roll-under has occurred
            pub const RUIRQ_0: u16 = 0b0;

            /// 0b1: Roll-under has occurred
            pub const RUIRQ_1: u16 = 0b1;
        }
    }

    /// Roll-over Interrupt Enable
    pub mod ROIE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const ROIE_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const ROIE_1: u16 = 0b1;
        }
    }

    /// Roll-over Interrupt Request
    pub mod ROIRQ {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No roll-over has occurred
            pub const ROIRQ_0: u16 = 0b0;

            /// 0b1: Roll-over has occurred
            pub const ROIRQ_1: u16 = 0b1;
        }
    }

    /// Revolution Counter Modulus Enable
    pub mod REVMOD {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Use INDEX pulse to increment/decrement revolution counter (REV)
            pub const REVMOD_0: u16 = 0b0;

            /// 0b1: Use modulus counting roll-over/under to increment/decrement revolution counter (REV)
            pub const REVMOD_1: u16 = 0b1;
        }
    }

    /// Output Control
    pub mod OUTCTL {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: POSMATCH pulses when a match occurs between the position counters (POS) and the corresponding compare value (COMP )
            pub const OUTCTL_0: u16 = 0b0;

            /// 0b1: POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read
            pub const OUTCTL_1: u16 = 0b1;
        }
    }

    /// Simultaneous PHASEA and PHASEB Change Interrupt Enable
    pub mod SABIE {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const SABIE_0: u16 = 0b0;

            /// 0b1: Enabled
            pub const SABIE_1: u16 = 0b1;
        }
    }

    /// Simultaneous PHASEA and PHASEB Change Interrupt Request
    pub mod SABIRQ {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No simultaneous change of PHASEA and PHASEB has occurred
            pub const SABIRQ_0: u16 = 0b0;

            /// 0b1: A simultaneous change of PHASEA and PHASEB has occurred
            pub const SABIRQ_1: u16 = 0b1;
        }
    }
}

/// Upper Modulus Register
pub mod UMOD {

    /// MOD
    pub mod MOD {
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

/// Lower Modulus Register
pub mod LMOD {
    pub use super::UMOD::MOD;
}

/// Upper Position Compare Register
pub mod UCOMP {

    /// COMP
    pub mod COMP {
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

/// Lower Position Compare Register
pub mod LCOMP {
    pub use super::UCOMP::COMP;
}

/// Last Edge Time Register
pub mod LASTEDGE {

    /// Last Edge Time Counter
    pub mod LASTEDGE {
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

/// Last Edge Time Hold Register
pub mod LASTEDGEH {

    /// Last Edge Time Hold
    pub mod LASTEDGEH {
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

/// Position Difference Period Counter Register
pub mod POSDPER {

    /// Position difference period
    pub mod POSDPER {
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

/// Position Difference Period Buffer Register
pub mod POSDPERBFR {

    /// Position difference period buffer
    pub mod POSDPERBFR {
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

/// Position Difference Period Hold Register
pub mod POSDPERH {

    /// Position difference period hold
    pub mod POSDPERH {
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

/// Control 3 Register
pub mod CTRL3 {

    /// Period measurement function enable
    pub mod PMEN {
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

            /// 0b0: Period measurement functions are not used. POSD is loaded to POSDH and then cleared whenever POSD, UPOS, LPOS, or REV is read.
            pub const PMEN_0: u16 = 0b0;

            /// 0b1: Period measurement functions are used. POSD is loaded to POSDH and then cleared only when POSD is read.
            pub const PMEN_1: u16 = 0b1;
        }
    }

    /// Prescaler
    pub mod PRSC {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u16 = 0b1111 << offset;
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
    /// Control Register
    pub CTRL: RWRegister<u16>,

    /// Input Filter Register
    pub FILT: RWRegister<u16>,

    /// Watchdog Timeout Register
    pub WTR: RWRegister<u16>,

    /// Position Difference Counter Register
    pub POSD: RWRegister<u16>,

    /// Position Difference Hold Register
    pub POSDH: RORegister<u16>,

    /// Revolution Counter Register
    pub REV: RWRegister<u16>,

    /// Revolution Hold Register
    pub REVH: RORegister<u16>,

    /// Upper Position Counter Register
    pub UPOS: RWRegister<u16>,

    /// Lower Position Counter Register
    pub LPOS: RWRegister<u16>,

    /// Upper Position Hold Register
    pub UPOSH: RORegister<u16>,

    /// Lower Position Hold Register
    pub LPOSH: RORegister<u16>,

    /// Upper Initialization Register
    pub UINIT: RWRegister<u16>,

    /// Lower Initialization Register
    pub LINIT: RWRegister<u16>,

    /// Input Monitor Register
    pub IMR: RORegister<u16>,

    /// Test Register
    pub TST: RWRegister<u16>,

    /// Control 2 Register
    pub CTRL2: RWRegister<u16>,

    /// Upper Modulus Register
    pub UMOD: RWRegister<u16>,

    /// Lower Modulus Register
    pub LMOD: RWRegister<u16>,

    /// Upper Position Compare Register
    pub UCOMP: RWRegister<u16>,

    /// Lower Position Compare Register
    pub LCOMP: RWRegister<u16>,

    /// Last Edge Time Register
    pub LASTEDGE: RORegister<u16>,

    /// Last Edge Time Hold Register
    pub LASTEDGEH: RORegister<u16>,

    /// Position Difference Period Counter Register
    pub POSDPER: RORegister<u16>,

    /// Position Difference Period Buffer Register
    pub POSDPERBFR: RORegister<u16>,

    /// Position Difference Period Hold Register
    pub POSDPERH: RORegister<u16>,

    /// Control 3 Register
    pub CTRL3: RWRegister<u16>,
}
pub struct ResetValues {
    pub CTRL: u16,
    pub FILT: u16,
    pub WTR: u16,
    pub POSD: u16,
    pub POSDH: u16,
    pub REV: u16,
    pub REVH: u16,
    pub UPOS: u16,
    pub LPOS: u16,
    pub UPOSH: u16,
    pub LPOSH: u16,
    pub UINIT: u16,
    pub LINIT: u16,
    pub IMR: u16,
    pub TST: u16,
    pub CTRL2: u16,
    pub UMOD: u16,
    pub LMOD: u16,
    pub UCOMP: u16,
    pub LCOMP: u16,
    pub LASTEDGE: u16,
    pub LASTEDGEH: u16,
    pub POSDPER: u16,
    pub POSDPERBFR: u16,
    pub POSDPERH: u16,
    pub CTRL3: u16,
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
