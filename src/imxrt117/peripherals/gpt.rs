#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPT
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// GPT Control Register
pub mod CR {

    /// GPT Enable
    pub mod EN {
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

            /// 0b0: Disable
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// GPT Enable Mode
    pub mod ENMOD {
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

            /// 0b0: Restart counting from their frozen values after GPT is enabled (EN=1).
            pub const RESUME_COUNT: u32 = 0b0;

            /// 0b1: Reset counting from 0 after GPT is enabled (EN=1).
            pub const ZERO_COUNT: u32 = 0b1;
        }
    }

    /// GPT Debug Mode Enable
    pub mod DBGEN {
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

            /// 0b0: Disable in Debug mode
            pub const DEBUG_DIS: u32 = 0b0;

            /// 0b1: Enable in Debug mode
            pub const DEBUG_EN: u32 = 0b1;
        }
    }

    /// GPT Wait Mode Enable
    pub mod WAITEN {
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

            /// 0b0: Disable in Wait mode
            pub const WAIT_DIS: u32 = 0b0;

            /// 0b1: Enable in Wait mode
            pub const WAIT_EN: u32 = 0b1;
        }
    }

    /// GPT Doze Mode Enable
    pub mod DOZEEN {
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

            /// 0b0: Disable in Doze mode
            pub const DOZE_DIS: u32 = 0b0;

            /// 0b1: Enable in Doze mode
            pub const DOZE_EN: u32 = 0b1;
        }
    }

    /// GPT Stop Mode Enable
    pub mod STOPEN {
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

            /// 0b0: Disable in Stop mode
            pub const STOP_DIS: u32 = 0b0;

            /// 0b1: Enable in Stop mode
            pub const STOP_EN: u32 = 0b1;
        }
    }

    /// Clock Source Select
    pub mod CLKSRC {
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

            /// 0b000: No clock
            pub const NO_CLOCK: u32 = 0b000;

            /// 0b001: Peripheral Clock (ipg_clk)
            pub const CLOCK_001: u32 = 0b001;

            /// 0b010: High Frequency Reference Clock (ipg_clk_highfreq)
            pub const CLOCK_010: u32 = 0b010;

            /// 0b011: External Clock
            pub const CLOCK_011: u32 = 0b011;

            /// 0b100: Low Frequency Reference Clock (ipg_clk_32k)
            pub const CLOCK_100: u32 = 0b100;

            /// 0b101: Oscillator as Reference Clock (ipg_clk_16M)
            pub const CLOCK_101: u32 = 0b101;
        }
    }

    /// Free-Run or Restart Mode
    pub mod FRR {
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

            /// 0b0: Restart mode. After a compare event, the counter resets to 0x0000_0000 and resumes counting.
            pub const RESTART: u32 = 0b0;

            /// 0b1: Free-Run mode. After a compare event, the counter continues counting until 0xFFFF_FFFF and then rolls over to 0.
            pub const FREE_RUN: u32 = 0b1;
        }
    }

    /// Enable Oscillator Clock Input
    pub mod EN_24M {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EN::RW;
    }

    /// Software Reset
    pub mod SWR {
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

            /// 0b0: GPT is not in software reset state
            pub const NOT_SWRESET: u32 = 0b0;

            /// 0b1: GPT is in software reset state
            pub const SWRESET: u32 = 0b1;
        }
    }

    /// Input Capture Operating Mode for Channel 1
    pub mod IM1 {
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

            /// 0b00: Capture disabled
            pub const DISABLED: u32 = 0b00;

            /// 0b01: Capture on rising edge only
            pub const RISING: u32 = 0b01;

            /// 0b10: Capture on falling edge only
            pub const FALLING: u32 = 0b10;

            /// 0b11: Capture on both edges
            pub const BOTH: u32 = 0b11;
        }
    }

    /// Input Capture Operating Mode for Channel 2
    pub mod IM2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM1::RW;
    }

    /// Output Compare Operating Mode for Channel 1
    pub mod OM1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Output disabled. No response on pin.
            pub const DISABLED: u32 = 0b000;

            /// 0b001: Toggle output pin
            pub const TOGGLE: u32 = 0b001;

            /// 0b010: Clear output pin
            pub const CLEAR: u32 = 0b010;

            /// 0b011: Set output pin
            pub const SET: u32 = 0b011;

            /// 0b000: Generate a low pulse that is one input clock cycle wide on the output pin. When OMn is first programmed as 1xx, the output pin is set to one immediately on the next input clock (if it was not one already). "Input clock" here refers to the clock selected by the CLKSRC field of this register.
            pub const PULSE: u32 = 0b000;
        }
    }

    /// Output Compare Operating Mode for Channel 2
    pub mod OM2 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (3 bits: 0b111 << 23)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OM1::RW;
    }

    /// Output Compare Operating Mode for Channel 3
    pub mod OM3 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (3 bits: 0b111 << 26)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OM1::RW;
    }

    /// Force Output Compare for Channel 1
    pub mod FO1 {
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

            /// 0b0: No effect
            pub const NO_FORCE: u32 = 0b0;

            /// 0b1: Trigger the programmed response on the pin
            pub const FORCE: u32 = 0b1;
        }
    }

    /// Force Output Compare for Channel 2
    pub mod FO2 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FO1::RW;
    }

    /// Force Output Compare for Channel 3
    pub mod FO3 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FO1::RW;
    }
}

/// GPT Prescaler Register
pub mod PR {

    /// Prescaler divide value
    pub mod PRESCALER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000000000: Divide by 1
            pub const DIV_BY_1: u32 = 0b000000000000;

            /// 0b000000000001: Divide by 2
            pub const DIV_BY_2: u32 = 0b000000000001;

            /// 0b111111111111: Divide by 4096
            pub const DIV_BY_4096: u32 = 0b111111111111;
        }
    }

    /// Prescaler divide value for the oscillator clock
    pub mod PRESCALER24M {
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

            /// 0b0000: Divide by 1
            pub const DIV_BY_1: u32 = 0b0000;

            /// 0b0001: Divide by 2
            pub const DIV_BY_2: u32 = 0b0001;

            /// 0b1111: Divide by 16
            pub const DIV_BY_16: u32 = 0b1111;
        }
    }
}

/// GPT Status Register
pub mod SR {

    /// Output Compare Flag for Channel 1
    pub mod OF1 {
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

            /// 0b0: Compare event has not occurred.
            pub const NO_EVENT: u32 = 0b0;

            /// 0b1: Compare event has occurred.
            pub const EVENT: u32 = 0b1;
        }
    }

    /// Output Compare Flag for Channel 2
    pub mod OF2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OF1::RW;
    }

    /// Output Compare Flag for Channel 3
    pub mod OF3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OF1::RW;
    }

    /// Input Capture Flag for Channel 1
    pub mod IF1 {
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

            /// 0b0: Capture event has not occurred.
            pub const NO_EVENT: u32 = 0b0;

            /// 0b1: Capture event has occurred.
            pub const EVENT: u32 = 0b1;
        }
    }

    /// Input Capture Flag for Channel 2
    pub mod IF2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IF1::RW;
    }

    /// Rollover Flag
    pub mod ROV {
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

            /// 0b0: Rollover has not occurred.
            pub const NO_ROLLOVER: u32 = 0b0;

            /// 0b1: Rollover has occurred.
            pub const ROLLOVER: u32 = 0b1;
        }
    }
}

/// GPT Interrupt Register
pub mod IR {

    /// Output Compare Flag for Channel 1 Interrupt Enable
    pub mod OF1IE {
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

            /// 0b0: Disable
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Output Compare Flag for Channel 2 Interrupt Enable
    pub mod OF2IE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OF1IE::RW;
    }

    /// Output Compare Flag for Channel 3 Interrupt Enable
    pub mod OF3IE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OF1IE::RW;
    }

    /// Input Capture Flag for Channel 1 Interrupt Enable
    pub mod IF1IE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OF1IE::RW;
    }

    /// Input Capture Flag for Channel 2 Interrupt Enable
    pub mod IF2IE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OF1IE::RW;
    }

    /// Rollover Interrupt Enable
    pub mod ROVIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OF1IE::RW;
    }
}

/// GPT Output Compare Register
pub mod OCR1 {

    /// Compare Value
    pub mod COMP {
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

/// GPT Output Compare Register
pub mod OCR2 {
    pub use super::OCR1::COMP;
}

/// GPT Output Compare Register
pub mod OCR3 {
    pub use super::OCR1::COMP;
}

/// GPT Input Capture Register
pub mod ICR1 {

    /// Capture Value
    pub mod CAPT {
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

/// GPT Input Capture Register
pub mod ICR2 {
    pub use super::ICR1::CAPT;
}

/// GPT Counter Register
pub mod CNT {

    /// Counter Value
    pub mod COUNT {
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
    /// GPT Control Register
    pub CR: RWRegister<u32>,

    /// GPT Prescaler Register
    pub PR: RWRegister<u32>,

    /// GPT Status Register
    pub SR: RWRegister<u32>,

    /// GPT Interrupt Register
    pub IR: RWRegister<u32>,

    /// GPT Output Compare Register
    pub OCR1: RWRegister<u32>,

    /// GPT Output Compare Register
    pub OCR2: RWRegister<u32>,

    /// GPT Output Compare Register
    pub OCR3: RWRegister<u32>,

    /// GPT Input Capture Register
    pub ICR1: RORegister<u32>,

    /// GPT Input Capture Register
    pub ICR2: RORegister<u32>,

    /// GPT Counter Register
    pub CNT: RORegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub PR: u32,
    pub SR: u32,
    pub IR: u32,
    pub OCR1: u32,
    pub OCR2: u32,
    pub OCR3: u32,
    pub ICR1: u32,
    pub ICR2: u32,
    pub CNT: u32,
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
