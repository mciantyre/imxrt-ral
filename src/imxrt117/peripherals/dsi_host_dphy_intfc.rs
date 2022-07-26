#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DSI HOST DPHY INTFC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// PD_TX
pub mod PD_TX {

    /// Power Down input for D-PHY
    pub mod PD_TX {
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

            /// 0b0: Power Up
            pub const PD_TX_0: u32 = 0b0;

            /// 0b1: Power Down
            pub const PD_TX_1: u32 = 0b1;
        }
    }
}

/// M_PRG_HS_PREPARE
pub mod M_PRG_HS_PREPARE {

    /// DPHY m_PRG_HS_PREPARE input
    pub mod M_PRG_HS_PREPARE {
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
}

/// MC_PRG_HS_PREPARE
pub mod MC_PRG_HS_PREPARE {

    /// DPHY mc_PRG_HS_PREPARE input
    pub mod MC_PRG_HS_PREPARE {
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

/// M_PRG_HS_ZERO
pub mod M_PRG_HS_ZERO {

    /// DPHY m_PRG_HS_ZERO input
    pub mod M_PRG_HS_ZERO {
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

/// MC_PRG_HS_ZERO
pub mod MC_PRG_HS_ZERO {

    /// DPHY mc_PRG_HS_ZERO input
    pub mod MC_PRG_HS_ZERO {
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
}

/// M_PRG_HS_TRAIL
pub mod M_PRG_HS_TRAIL {

    /// DPHY m_PRG_HS_TRAIL input
    pub mod M_PRG_HS_TRAIL {
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

/// MC_PRG_HS_TRAIL
pub mod MC_PRG_HS_TRAIL {

    /// DPHY mc_PRG_HS_TRAIL input
    pub mod MC_PRG_HS_TRAIL {
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

/// PD_PLL
pub mod PD_PLL {

    /// Power-down signal
    pub mod PD_PLL {
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

            /// 0b0: Power up PLL
            pub const PD_PLL_0: u32 = 0b0;

            /// 0b1: Power down PLL
            pub const PD_PLL_1: u32 = 0b1;
        }
    }
}

/// TST
pub mod TST {

    /// Test
    pub mod TST {
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
}

/// CN
pub mod CN {

    /// Control N divider
    pub mod CN {
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

/// CM
pub mod CM {

    /// Control M divider
    pub mod CM {
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

/// CO
pub mod CO {

    /// Control O divider
    pub mod CO {
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

            /// 0b00: Divide by 1
            pub const CO_0: u32 = 0b00;

            /// 0b01: Divide by 2
            pub const CO_1: u32 = 0b01;

            /// 0b10: Divide by 4
            pub const CO_2: u32 = 0b10;

            /// 0b11: Divide by 8
            pub const CO_3: u32 = 0b11;
        }
    }
}

/// LOCK
pub mod LOCK {

    /// Lock Detect output
    pub mod LOCK {
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

            /// 0b0: PLL not locked
            pub const LOCK_0: u32 = 0b0;

            /// 0b1: PLL has achieved frequency lock
            pub const LOCK_1: u32 = 0b1;
        }
    }
}

/// LOCK_BYP
pub mod LOCK_BYP {

    /// DPHY LOCK_BYP input
    pub mod LOCK_BYP {
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

            /// 0b0: PLL LOCK signal will gate TxByteClkHS clock
            pub const GATE: u32 = 0b0;

            /// 0b1: PLL LOCK signal will not gate TxByteClkHS clock, CIL based counter will be used to gate the TxByteClkHS
            pub const NOGATE: u32 = 0b1;
        }
    }
}

/// TX_RCAL
pub mod TX_RCAL {

    /// On-chip termination control bits for manual calibration of HS-TX
    pub mod TX_RCAL {
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

            /// 0b00: 20% higher than mid-range. Highest impedance setting
            pub const TX_RCAL_0: u32 = 0b00;

            /// 0b01: Mid-range impedance setting (default)
            pub const TX_RCAL_1: u32 = 0b01;

            /// 0b10: 15% lower than mid-range
            pub const TX_RCAL_2: u32 = 0b10;

            /// 0b11: 25% lower than mid-range. Lowest impedance setting
            pub const TX_RCAL_3: u32 = 0b11;
        }
    }
}

/// AUTO_PD_EN
pub mod AUTO_PD_EN {

    /// DPHY AUTO_PD_EN input
    pub mod AUTO_PD_EN {
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

            /// 0b0: Inactive lanes are powered up and driving LP11
            pub const PWR_UP: u32 = 0b0;

            /// 0b1: inactive lanes are powered down
            pub const PWR_DWN: u32 = 0b1;
        }
    }
}

/// RXLPRP
pub mod RXLPRP {

    /// DPHY RXLPRP input
    pub mod RXLPRP {
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
}

/// RXCDRP
pub mod RXCDRP {

    /// DPHY RXCDRP input
    pub mod RXCDRP {
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

            /// 0b00: 344mV
            pub const RXCDRP_0: u32 = 0b00;

            /// 0b01: 325mV (Default)
            pub const RXCDRP_1: u32 = 0b01;

            /// 0b10: 307mV
            pub const RXCDRP_2: u32 = 0b10;

            /// 0b11: Invalid
            pub const RXCDRP_3: u32 = 0b11;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// PD_TX
    pub PD_TX: RWRegister<u32>,

    /// M_PRG_HS_PREPARE
    pub M_PRG_HS_PREPARE: RWRegister<u32>,

    /// MC_PRG_HS_PREPARE
    pub MC_PRG_HS_PREPARE: RWRegister<u32>,

    /// M_PRG_HS_ZERO
    pub M_PRG_HS_ZERO: RWRegister<u32>,

    /// MC_PRG_HS_ZERO
    pub MC_PRG_HS_ZERO: RWRegister<u32>,

    /// M_PRG_HS_TRAIL
    pub M_PRG_HS_TRAIL: RWRegister<u32>,

    /// MC_PRG_HS_TRAIL
    pub MC_PRG_HS_TRAIL: RWRegister<u32>,

    /// PD_PLL
    pub PD_PLL: RWRegister<u32>,

    /// TST
    pub TST: RWRegister<u32>,

    /// CN
    pub CN: RWRegister<u32>,

    /// CM
    pub CM: RWRegister<u32>,

    /// CO
    pub CO: RWRegister<u32>,

    /// LOCK
    pub LOCK: RORegister<u32>,

    /// LOCK_BYP
    pub LOCK_BYP: RWRegister<u32>,

    /// TX_RCAL
    pub TX_RCAL: RWRegister<u32>,

    /// AUTO_PD_EN
    pub AUTO_PD_EN: RWRegister<u32>,

    /// RXLPRP
    pub RXLPRP: RWRegister<u32>,

    /// RXCDRP
    pub RXCDRP: RWRegister<u32>,
}
pub struct ResetValues {
    pub PD_TX: u32,
    pub M_PRG_HS_PREPARE: u32,
    pub MC_PRG_HS_PREPARE: u32,
    pub M_PRG_HS_ZERO: u32,
    pub MC_PRG_HS_ZERO: u32,
    pub M_PRG_HS_TRAIL: u32,
    pub MC_PRG_HS_TRAIL: u32,
    pub PD_PLL: u32,
    pub TST: u32,
    pub CN: u32,
    pub CM: u32,
    pub CO: u32,
    pub LOCK: u32,
    pub LOCK_BYP: u32,
    pub TX_RCAL: u32,
    pub AUTO_PD_EN: u32,
    pub RXLPRP: u32,
    pub RXCDRP: u32,
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
