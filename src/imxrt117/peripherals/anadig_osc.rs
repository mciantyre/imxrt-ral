#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// 48MHz RCOSC Control Register
pub mod OSC_48M_CTRL {

    /// 48MHz RCOSC Enable
    pub mod TEN {
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

            /// 0b0: Power down
            pub const PD: u32 = 0b0;

            /// 0b1: Power up
            pub const PU: u32 = 0b1;
        }
    }

    /// RCOSC_48M_DIV2 Enable
    pub mod RC_48M_DIV2_EN {
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

            /// 0b0: Disable
            pub const PD: u32 = 0b0;

            /// 0b1: Enable
            pub const PU: u32 = 0b1;
        }
    }

    /// RCOSC_48M_DIV2 Control Mode
    pub mod RC_48M_DIV2_CONTROL_MODE {
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

            /// 0b0: Software mode (default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC mode (Setpoint)
            pub const GPC: u32 = 0b1;
        }
    }

    /// 48MHz RCOSC Control Mode
    pub mod RC_48M_CONTROL_MODE {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RC_48M_DIV2_CONTROL_MODE::RW;
    }
}

/// 24MHz OSC Control Register
pub mod OSC_24M_CTRL {

    /// 24MHz OSC Bypass Clock
    pub mod BYPASS_CLK {
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

    /// 24MHz OSC Bypass Enable
    pub mod BYPASS_EN {
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

            /// 0b0: Disable
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// 24MHz OSC Low-Power Mode Enable
    pub mod LP_EN {
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

            /// 0b0: High Gain mode (HP)
            pub const HP: u32 = 0b0;

            /// 0b1: Low-power mode (LP)
            pub const LP: u32 = 0b1;
        }
    }

    /// 24MHz OSC Comparator Mode
    pub mod OSC_COMP_MODE {
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

            /// 0b0: Single-ended mode (default)
            pub const SINGLE: u32 = 0b0;

            /// 0b1: Differential mode (test mode)
            pub const DIFF: u32 = 0b1;
        }
    }

    /// 24MHz OSC Enable
    pub mod OSC_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN::RW;
    }

    /// 24MHz OSC Gate Control
    pub mod OSC_24M_GATE {
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

            /// 0b0: Not Gated
            pub const NG: u32 = 0b0;

            /// 0b1: Gated
            pub const GATE: u32 = 0b1;
        }
    }

    /// 24MHz OSC Stable
    pub mod OSC_24M_STABLE {
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

            /// 0b0: Not Stable
            pub const NS: u32 = 0b0;

            /// 0b1: Stable
            pub const STABLE: u32 = 0b1;
        }
    }

    /// 24MHz OSC Control Mode
    pub mod OSC_24M_CONTROL_MODE {
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

            /// 0b0: Software mode (default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC mode (Setpoint)
            pub const GPC: u32 = 0b1;
        }
    }
}

/// 400MHz RCOSC Control0 Register
pub mod OSC_400M_CTRL0 {

    /// 400MHz OSC AI BUSY
    pub mod OSC400M_AI_BUSY {
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

/// 400MHz RCOSC Control1 Register
pub mod OSC_400M_CTRL1 {

    /// Power down control for 400MHz RCOSC
    pub mod PWD {
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

            /// 0b0: No Power down
            pub const PD: u32 = 0b0;

            /// 0b1: Power down
            pub const PU: u32 = 0b1;
        }
    }

    /// Clock gate control for 400MHz RCOSC
    pub mod CLKGATE_400MEG {
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

            /// 0b0: Not Gated
            pub const NG: u32 = 0b0;

            /// 0b1: Gated
            pub const GATE: u32 = 0b1;
        }
    }

    /// 400MHz RCOSC Control mode
    pub mod RC_400M_CONTROL_MODE {
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

            /// 0b0: Software mode (default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC mode (Setpoint)
            pub const GPC: u32 = 0b1;
        }
    }
}

/// 400MHz RCOSC Control2 Register
pub mod OSC_400M_CTRL2 {

    /// Clock enable
    pub mod ENABLE_CLK {
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

            /// 0b0: Clock is disabled before entering GPC mode
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Clock is enabled before entering GPC mode
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Bypass tuning logic
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
            pub const RUN: u32 = 0b0;

            /// 0b1: Bypass the tuning logic and use the programmed OSC_TUNE_VAL to run the oscillator
            pub const BYPASS: u32 = 0b1;
        }
    }

    /// Oscillator Tune Value
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

/// 16MHz RCOSC Control Register
pub mod OSC_16M_CTRL {

    /// Enable Clock Output
    pub mod EN_IRC4M16M {
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

            /// 0b0: Disable
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Power Save Enable
    pub mod EN_POWER_SAVE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EN_IRC4M16M::RW;
    }

    /// Source select
    pub mod SOURCE_SEL_16M {
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

            /// 0b0: 16MHz Oscillator
            pub const Mhz_16: u32 = 0b0;

            /// 0b1: 24MHz Oscillator
            pub const Mhz_24: u32 = 0b1;
        }
    }

    /// Control Mode for 16MHz Oscillator
    pub mod RC_16M_CONTROL_MODE {
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

            /// 0b0: Software mode (default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC mode (Setpoint)
            pub const GPC: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 4],

    /// 48MHz RCOSC Control Register
    pub OSC_48M_CTRL: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// 24MHz OSC Control Register
    pub OSC_24M_CTRL: RWRegister<u32>,

    _reserved3: [u32; 7],

    /// 400MHz RCOSC Control0 Register
    pub OSC_400M_CTRL0: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// 400MHz RCOSC Control1 Register
    pub OSC_400M_CTRL1: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// 400MHz RCOSC Control2 Register
    pub OSC_400M_CTRL2: RWRegister<u32>,

    _reserved6: [u32; 23],

    /// 16MHz RCOSC Control Register
    pub OSC_16M_CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub OSC_48M_CTRL: u32,
    pub OSC_24M_CTRL: u32,
    pub OSC_400M_CTRL0: u32,
    pub OSC_400M_CTRL1: u32,
    pub OSC_400M_CTRL2: u32,
    pub OSC_16M_CTRL: u32,
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
