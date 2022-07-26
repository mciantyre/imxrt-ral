#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBDCD
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Control register
pub mod CONTROL {

    /// Interrupt Acknowledge
    pub mod IACK {
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

            /// 0b0: Do not clear the interrupt.
            pub const INT_NOCLEAR: u32 = 0b0;

            /// 0b1: Clear the IF bit (interrupt flag).
            pub const INT_CLEAR: u32 = 0b1;
        }
    }

    /// Interrupt Flag
    pub mod IF {
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

            /// 0b0: No interrupt is pending.
            pub const INT_PEND: u32 = 0b0;

            /// 0b1: An interrupt is pending.
            pub const INT_NOPEND: u32 = 0b1;
        }
    }

    /// Interrupt Enable
    pub mod IE {
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

            /// 0b0: Disable interrupts to the system.
            pub const DIS_INT: u32 = 0b0;

            /// 0b1: Enable interrupts to the system.
            pub const EN_INT: u32 = 0b1;
        }
    }

    /// BC12
    pub mod BC12 {
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

            /// 0b0: Compatible with BC1.1 (default)
            pub const BC11: u32 = 0b0;

            /// 0b1: Compatible with BC1.2
            pub const BC12: u32 = 0b1;
        }
    }

    /// Start Change Detection Sequence
    pub mod START {
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

            /// 0b0: Do not start the sequence. Writes of this value have no effect.
            pub const NO_START: u32 = 0b0;

            /// 0b1: Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect.
            pub const START: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod SR {
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

            /// 0b0: Do not perform a software reset.
            pub const NO_RESET: u32 = 0b0;

            /// 0b1: Perform a software reset.
            pub const SW_RESET: u32 = 0b1;
        }
    }
}

/// Clock register
pub mod CLOCK {

    /// Unit of Measurement Encoding for Clock Speed
    pub mod CLOCK_UNIT {
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

            /// 0b0: kHz Speed (between 1 kHz and 1023 kHz)
            pub const KHZ_CLK: u32 = 0b0;

            /// 0b1: MHz Speed (between 1 MHz and 1023 MHz)
            pub const MHZ_CLK: u32 = 0b1;
        }
    }

    /// Numerical Value of Clock Speed in Binary
    pub mod CLOCK_SPEED {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (10 bits: 0x3ff << 2)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status register
pub mod STATUS {

    /// Charger Detection Sequence Results
    pub mod SEQ_RES {
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

            /// 0b00: No results to report.
            pub const NO_RESULT: u32 = 0b00;

            /// 0b01: Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected.
            pub const CONN_SDP: u32 = 0b01;

            /// 0b10: Attached to a charging port. The exact meaning depends on bit 18 (value 0: Attached to either a CDP or a DCP. The charger type detection has not completed. value 1: Attached to a CDP. The charger type detection has completed.)
            pub const CONN_CP: u32 = 0b10;

            /// 0b11: Attached to a DCP.
            pub const CONN_DCP: u32 = 0b11;
        }
    }

    /// Charger Detection Sequence Status
    pub mod SEQ_STAT {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The module is either not enabled, or the module is enabled but the data pins have not yet been detected.
            pub const NO_DATA_PIN_CONN: u32 = 0b00;

            /// 0b01: Data pin contact detection is complete.
            pub const DATA_PIN_CONN: u32 = 0b01;

            /// 0b10: Charging port detection is complete.
            pub const CP_DET_DONE: u32 = 0b10;

            /// 0b11: Charger type detection is complete.
            pub const CT_DET_DONE: u32 = 0b11;
        }
    }

    /// Error Flag
    pub mod ERR {
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

            /// 0b0: No sequence errors.
            pub const NO_SEQ_ERR: u32 = 0b0;

            /// 0b1: Error in the detection sequence. See the SEQ_STAT field to determine the phase in which the error occurred.
            pub const SEQ_ERR: u32 = 0b1;
        }
    }

    /// Timeout Flag
    pub mod TO {
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

            /// 0b0: The detection sequence has not been running for over 1s.
            pub const NO_TIMEOUT: u32 = 0b0;

            /// 0b1: It has been over 1 s since the data pin contact was detected and debounced.
            pub const TIMEOUT: u32 = 0b1;
        }
    }

    /// Active Status Indicator
    pub mod ACTIVE {
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

            /// 0b0: The sequence is not running.
            pub const SEQ_NOT_RUNNING: u32 = 0b0;

            /// 0b1: The sequence is running.
            pub const SEQ_RUNNING: u32 = 0b1;
        }
    }
}

/// Signal Override Register
pub mod SIGNAL_OVERRIDE {

    /// Phase Selection
    pub mod PS {
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

            /// 0b00: No overrides. Bit field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)
            pub const NO_OVERRIDE: u32 = 0b00;

            /// 0b10: Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin.
            pub const PRI_DET_OVERRIDE: u32 = 0b10;
        }
    }
}

/// TIMER0 register
pub mod TIMER0 {

    /// Unit Connection Timer Elapse (in ms)
    pub mod TUNITCON {
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

    /// Sequence Initiation Time
    pub mod TSEQ_INIT {
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

/// TIMER1 register
pub mod TIMER1 {

    /// Time Period Comparator Enabled
    pub mod TVDPSRC_ON {
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

    /// Time Period to Debounce D+ Signal
    pub mod TDCD_DBNC {
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

/// TIMER2_BC11 and TIMER2_BC12
/// TIMER2_BC11: TIMER2_BC11 register
/// TIMER2_BC12: TIMER2_BC12 register
pub mod TIMER2_BC1 {

    /// Time Before Check of D- Line
    pub mod CHECK_DM {
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

    /// Time Period Before Enabling D+ Pullup
    pub mod TVDPSRC_CON {
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

    /// TVDMSRC_ON
    pub mod TVDMSRC_ON {
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

    /// TWAIT_AFTER_PRD
    pub mod TWAIT_AFTER_PRD {
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
#[repr(C)]
pub struct RegisterBlock {
    /// Control register
    pub CONTROL: RWRegister<u32>,

    /// Clock register
    pub CLOCK: RWRegister<u32>,

    /// Status register
    pub STATUS: RORegister<u32>,

    /// Signal Override Register
    pub SIGNAL_OVERRIDE: RWRegister<u32>,

    /// TIMER0 register
    pub TIMER0: RWRegister<u32>,

    /// TIMER1 register
    pub TIMER1: RWRegister<u32>,

    /// TIMER2_BC11 and TIMER2_BC12
    /// TIMER2_BC11: TIMER2_BC11 register
    /// TIMER2_BC12: TIMER2_BC12 register
    pub TIMER2_BC1: RWRegister<u32>,
}
pub struct ResetValues {
    pub CONTROL: u32,
    pub CLOCK: u32,
    pub STATUS: u32,
    pub SIGNAL_OVERRIDE: u32,
    pub TIMER0: u32,
    pub TIMER1: u32,
    pub TIMER2_BC1: u32,
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
