#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Control Register 0
pub mod CTRL0 {

    /// Divide value for ref_clk to generate slow_clk (used inside this IP)
    pub mod REF_CLK_DIV {
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

/// Control Register 0
pub mod CTRL0_SET {
    pub use super::CTRL0::REF_CLK_DIV;
}

/// Control Register 0
pub mod CTRL0_CLR {
    pub use super::CTRL0::REF_CLK_DIV;
}

/// Control Register 0
pub mod CTRL0_TOG {
    pub use super::CTRL0::REF_CLK_DIV;
}

/// Control Register 1
pub mod CTRL1 {

    /// Negative hysteresis value for the tuned clock
    pub mod HYST_MINUS {
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

    /// Positive hysteresis value for the tuned clock
    pub mod HYST_PLUS {
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

    /// Target count for the fast clock
    pub mod TARGET_COUNT {
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

/// Control Register 1
pub mod CTRL1_SET {
    pub use super::CTRL1::HYST_MINUS;
    pub use super::CTRL1::HYST_PLUS;
    pub use super::CTRL1::TARGET_COUNT;
}

/// Control Register 1
pub mod CTRL1_CLR {
    pub use super::CTRL1::HYST_MINUS;
    pub use super::CTRL1::HYST_PLUS;
    pub use super::CTRL1::TARGET_COUNT;
}

/// Control Register 1
pub mod CTRL1_TOG {
    pub use super::CTRL1::HYST_MINUS;
    pub use super::CTRL1::HYST_PLUS;
    pub use super::CTRL1::TARGET_COUNT;
}

/// Control Register 2
pub mod CTRL2 {

    /// Bypass the tuning logic
    pub mod TUNE_BYP {
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

            /// 0b0: Use the output of tuning logic to run the oscillator
            pub const TUNE_BYP_0: u32 = 0b0;

            /// 0b1: Bypass the tuning logic and use the programmed OSC_TUNE_VAL to run the oscillator
            pub const TUNE_BYP_1: u32 = 0b1;
        }
    }

    /// Freeze/Unfreeze the tuning value
    pub mod TUNE_EN {
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

            /// 0b0: Freezes the tuning at the current tuned value. Oscillator runs at the frozen tuning value
            pub const TUNE_EN_0: u32 = 0b0;

            /// 0b1: Unfreezes and continues the tuning operation
            pub const TUNE_EN_1: u32 = 0b1;
        }
    }

    /// Start/Stop tuning
    pub mod TUNE_START {
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

            /// 0b0: Stop tuning and reset the tuning logic. Oscillator runs using programmed OSC_TUNE_VAL
            pub const TUNE_START_0: u32 = 0b0;

            /// 0b1: Start tuning
            pub const TUNE_START_1: u32 = 0b1;
        }
    }

    /// Program the oscillator frequency
    pub mod OSC_TUNE_VAL {
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

/// Control Register 2
pub mod CTRL2_SET {

    /// Bypass the tuning logic
    pub mod TUNE_BYP {
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

    /// Freeze/Unfreeze the tuning value
    pub mod TUNE_EN {
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

    /// Start/Stop tuning
    pub mod TUNE_START {
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

    /// Program the oscillator frequency
    pub mod OSC_TUNE_VAL {
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

/// Control Register 2
pub mod CTRL2_CLR {
    pub use super::CTRL2_SET::OSC_TUNE_VAL;
    pub use super::CTRL2_SET::TUNE_BYP;
    pub use super::CTRL2_SET::TUNE_EN;
    pub use super::CTRL2_SET::TUNE_START;
}

/// Control Register 2
pub mod CTRL2_TOG {
    pub use super::CTRL2_SET::OSC_TUNE_VAL;
    pub use super::CTRL2_SET::TUNE_BYP;
    pub use super::CTRL2_SET::TUNE_EN;
    pub use super::CTRL2_SET::TUNE_START;
}

/// Control Register 3
pub mod CTRL3 {

    /// Clear the error flag CLK1M_ERR
    pub mod CLR_ERR {
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

            /// 0b0: No effect
            pub const CLR_ERR_0: u32 = 0b0;

            /// 0b1: Clears the error flag CLK1M_ERR in status register STAT0
            pub const CLR_ERR_1: u32 = 0b1;
        }
    }

    /// Enable 1MHz output Clock
    pub mod EN_1M_CLK {
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

            /// 0b0: Enable the output (clk_1m_out)
            pub const EN_1M_CLK_0: u32 = 0b0;

            /// 0b1: Disable the output (clk_1m_out)
            pub const EN_1M_CLK_1: u32 = 0b1;
        }
    }

    /// Select free/locked 1MHz output
    pub mod MUX_1M_CLK {
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

            /// 0b0: Select free-running 1MHz to be put out on clk_1m_out
            pub const MUX_1M_CLK_0: u32 = 0b0;

            /// 0b1: Select locked 1MHz to be put out on clk_1m_out
            pub const MUX_1M_CLK_1: u32 = 0b1;
        }
    }

    /// Count for the locked clk_1m_out
    pub mod COUNT_1M_CLK {
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

/// Control Register 3
pub mod CTRL3_SET {

    /// Clear the error flag CLK1M_ERR
    pub mod CLR_ERR {
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

    /// Enable 1MHz output Clock
    pub mod EN_1M_CLK {
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

    /// Select free/locked 1MHz output
    pub mod MUX_1M_CLK {
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

    /// Count for the locked clk_1m_out
    pub mod COUNT_1M_CLK {
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

/// Control Register 3
pub mod CTRL3_CLR {
    pub use super::CTRL3_SET::CLR_ERR;
    pub use super::CTRL3_SET::COUNT_1M_CLK;
    pub use super::CTRL3_SET::EN_1M_CLK;
    pub use super::CTRL3_SET::MUX_1M_CLK;
}

/// Control Register 3
pub mod CTRL3_TOG {
    pub use super::CTRL3_SET::CLR_ERR;
    pub use super::CTRL3_SET::COUNT_1M_CLK;
    pub use super::CTRL3_SET::EN_1M_CLK;
    pub use super::CTRL3_SET::MUX_1M_CLK;
}

/// Status Register 0
pub mod STAT0 {

    /// Error flag for clk_1m_locked
    pub mod CLK1M_ERR {
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

            /// 0b0: No effect
            pub const CLK1M_ERR_0: u32 = 0b0;

            /// 0b1: The count value has been reached within one divided ref_clk period
            pub const CLK1M_ERR_1: u32 = 0b1;
        }
    }
}

/// Status Register 0
pub mod STAT0_SET {

    /// Error flag for clk_1m_locked
    pub mod CLK1M_ERR {
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

/// Status Register 0
pub mod STAT0_CLR {
    pub use super::STAT0_SET::CLK1M_ERR;
}

/// Status Register 0
pub mod STAT0_TOG {
    pub use super::STAT0_SET::CLK1M_ERR;
}

/// Status Register 1
pub mod STAT1 {

    /// Current count for the fast clock
    pub mod CURR_COUNT_VAL {
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

/// Status Register 1
pub mod STAT1_SET {
    pub use super::STAT1::CURR_COUNT_VAL;
}

/// Status Register 1
pub mod STAT1_CLR {
    pub use super::STAT1::CURR_COUNT_VAL;
}

/// Status Register 1
pub mod STAT1_TOG {
    pub use super::STAT1::CURR_COUNT_VAL;
}

/// Status Register 2
pub mod STAT2 {

    /// Current tuning value used by oscillator
    pub mod CURR_OSC_TUNE_VAL {
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

/// Status Register 2
pub mod STAT2_SET {
    pub use super::STAT2::CURR_OSC_TUNE_VAL;
}

/// Status Register 2
pub mod STAT2_CLR {
    pub use super::STAT2::CURR_OSC_TUNE_VAL;
}

/// Status Register 2
pub mod STAT2_TOG {
    pub use super::STAT2::CURR_OSC_TUNE_VAL;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Control Register 0
    pub CTRL0: RWRegister<u32>,

    /// Control Register 0
    pub CTRL0_SET: RWRegister<u32>,

    /// Control Register 0
    pub CTRL0_CLR: RWRegister<u32>,

    /// Control Register 0
    pub CTRL0_TOG: RWRegister<u32>,

    /// Control Register 1
    pub CTRL1: RWRegister<u32>,

    /// Control Register 1
    pub CTRL1_SET: RWRegister<u32>,

    /// Control Register 1
    pub CTRL1_CLR: RWRegister<u32>,

    /// Control Register 1
    pub CTRL1_TOG: RWRegister<u32>,

    /// Control Register 2
    pub CTRL2: RWRegister<u32>,

    /// Control Register 2
    pub CTRL2_SET: RWRegister<u32>,

    /// Control Register 2
    pub CTRL2_CLR: RWRegister<u32>,

    /// Control Register 2
    pub CTRL2_TOG: RWRegister<u32>,

    /// Control Register 3
    pub CTRL3: RWRegister<u32>,

    /// Control Register 3
    pub CTRL3_SET: RWRegister<u32>,

    /// Control Register 3
    pub CTRL3_CLR: RWRegister<u32>,

    /// Control Register 3
    pub CTRL3_TOG: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// Status Register 0
    pub STAT0: RORegister<u32>,

    /// Status Register 0
    pub STAT0_SET: RORegister<u32>,

    /// Status Register 0
    pub STAT0_CLR: RORegister<u32>,

    /// Status Register 0
    pub STAT0_TOG: RORegister<u32>,

    /// Status Register 1
    pub STAT1: RORegister<u32>,

    /// Status Register 1
    pub STAT1_SET: RORegister<u32>,

    /// Status Register 1
    pub STAT1_CLR: RORegister<u32>,

    /// Status Register 1
    pub STAT1_TOG: RORegister<u32>,

    /// Status Register 2
    pub STAT2: RORegister<u32>,

    /// Status Register 2
    pub STAT2_SET: RORegister<u32>,

    /// Status Register 2
    pub STAT2_CLR: RORegister<u32>,

    /// Status Register 2
    pub STAT2_TOG: RORegister<u32>,
}
pub struct ResetValues {
    pub CTRL0: u32,
    pub CTRL0_SET: u32,
    pub CTRL0_CLR: u32,
    pub CTRL0_TOG: u32,
    pub CTRL1: u32,
    pub CTRL1_SET: u32,
    pub CTRL1_CLR: u32,
    pub CTRL1_TOG: u32,
    pub CTRL2: u32,
    pub CTRL2_SET: u32,
    pub CTRL2_CLR: u32,
    pub CTRL2_TOG: u32,
    pub CTRL3: u32,
    pub CTRL3_SET: u32,
    pub CTRL3_CLR: u32,
    pub CTRL3_TOG: u32,
    pub STAT0: u32,
    pub STAT0_SET: u32,
    pub STAT0_CLR: u32,
    pub STAT0_TOG: u32,
    pub STAT1: u32,
    pub STAT1_SET: u32,
    pub STAT1_CLR: u32,
    pub STAT1_TOG: u32,
    pub STAT2: u32,
    pub STAT2_SET: u32,
    pub STAT2_CLR: u32,
    pub STAT2_TOG: u32,
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
