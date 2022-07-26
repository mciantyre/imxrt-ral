#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Temperature Sensor Memory Map
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// Temperature Sensor Control Register 0
pub mod CTRL0 {

    /// Ramp slope calibration control
    pub mod SLOPE_CAL {
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

    /// Voltage Select
    pub mod V_SEL {
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

            /// 0b00: Normal temperature measuring mode
            pub const V_SEL_0: u32 = 0b00;
        }
    }

    /// Current bias trim value
    pub mod IBIAS_TRIM {
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
}

/// Temperature Sensor Control Register 0
pub mod CTRL0_SET {

    /// Ramp slope calibration control
    pub mod SLOPE_CAL {
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

    /// Voltage Select
    pub mod V_SEL {
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

    /// Current bias trim value
    pub mod IBIAS_TRIM {
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
}

/// Temperature Sensor Control Register 0
pub mod CTRL0_CLR {
    pub use super::CTRL0_SET::IBIAS_TRIM;
    pub use super::CTRL0_SET::SLOPE_CAL;
    pub use super::CTRL0_SET::V_SEL;
}

/// Temperature Sensor Control Register 0
pub mod CTRL0_TOG {
    pub use super::CTRL0_SET::IBIAS_TRIM;
    pub use super::CTRL0_SET::SLOPE_CAL;
    pub use super::CTRL0_SET::V_SEL;
}

/// Temperature Sensor Control Register 1
pub mod CTRL1 {

    /// Temperature Measurement Frequency
    pub mod FREQ {
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

            /// 0b0000000000000000: Single Reading Mode. New reading available every time CTRL1\[START\] bit is set to 1 from 0.
            pub const FREQ_0: u32 = 0b0000000000000000;

            /// 0b0000000000000001: Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete.
            pub const FREQ_1: u32 = 0b0000000000000001;

            /// 0b0000000000000010: Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete.
            pub const FREQ_2: u32 = 0b0000000000000010;

            /// 0b0000000000000011: Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete.
            pub const FREQ_3: u32 = 0b0000000000000011;

            /// 0b0000000000000100: Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete.
            pub const FREQ_4: u32 = 0b0000000000000100;

            /// 0b0000000000000101: Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete.
            pub const FREQ_5: u32 = 0b0000000000000101;

            /// 0b0000000000000110: Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete.
            pub const FREQ_6: u32 = 0b0000000000000110;

            /// 0b0000000000000111: Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete.
            pub const FREQ_7: u32 = 0b0000000000000111;

            /// 0b0000000000001000: Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete.
            pub const FREQ_8: u32 = 0b0000000000001000;

            /// 0b0000000000001001: Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete.
            pub const FREQ_9: u32 = 0b0000000000001001;
        }
    }

    /// Measurement finished interrupt enable
    pub mod FINISH_IE {
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

            /// 0b0: Interrupt is disabled
            pub const FINISH_IE_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const FINISH_IE_1: u32 = 0b1;
        }
    }

    /// Low temperature interrupt enable
    pub mod LOW_TEMP_IE {
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

            /// 0b0: Interrupt is disabled
            pub const LOW_TEMP_IE_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const LOW_TEMP_IE_1: u32 = 0b1;
        }
    }

    /// High temperature interrupt enable
    pub mod HIGH_TEMP_IE {
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

            /// 0b0: Interrupt is disabled
            pub const HIGH_TEMP_IE_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const HIGH_TEMP_IE_1: u32 = 0b1;
        }
    }

    /// Panic temperature interrupt enable
    pub mod PANIC_TEMP_IE {
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

            /// 0b0: Interrupt is disabled
            pub const PANIC_TEMP_IE_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const PANIC_TEMP_IE_1: u32 = 0b1;
        }
    }

    /// Start Temperature Measurement
    pub mod START {
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

            /// 0b0: No new temperature reading taken
            pub const START_0: u32 = 0b0;

            /// 0b1: Initiate a new temperature reading
            pub const START_1: u32 = 0b1;
        }
    }

    /// Temperature Sensor Power Down
    pub mod PWD {
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

            /// 0b0: Sensor is active
            pub const PWD_0: u32 = 0b0;

            /// 0b1: Sensor is powered down
            pub const PWD_1: u32 = 0b1;
        }
    }

    /// Read/Writeable field. Reserved for future use
    pub mod RFU {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Temperature Sensor Full Power Down
    pub mod PWD_FULL {
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

            /// 0b0: Sensor is active
            pub const PWD_FULL_0: u32 = 0b0;

            /// 0b1: Sensor is powered down
            pub const PWD_FULL_1: u32 = 0b1;
        }
    }
}

/// Temperature Sensor Control Register 1
pub mod CTRL1_SET {

    /// Temperature Measurement Frequency
    pub mod FREQ {
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

    /// Measurement finished interrupt enable
    pub mod FINISH_IE {
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

    /// Low temperature interrupt enable
    pub mod LOW_TEMP_IE {
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

    /// High temperature interrupt enable
    pub mod HIGH_TEMP_IE {
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

    /// Panic temperature interrupt enable
    pub mod PANIC_TEMP_IE {
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

    /// Start Temperature Measurement
    pub mod START {
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

    /// Temperature Sensor Power Down
    pub mod PWD {
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

    /// Read/Writeable field. Reserved for future use
    pub mod RFU {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Temperature Sensor Full Power Down
    pub mod PWD_FULL {
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

/// Temperature Sensor Control Register 1
pub mod CTRL1_CLR {
    pub use super::CTRL1_SET::FINISH_IE;
    pub use super::CTRL1_SET::FREQ;
    pub use super::CTRL1_SET::HIGH_TEMP_IE;
    pub use super::CTRL1_SET::LOW_TEMP_IE;
    pub use super::CTRL1_SET::PANIC_TEMP_IE;
    pub use super::CTRL1_SET::PWD;
    pub use super::CTRL1_SET::PWD_FULL;
    pub use super::CTRL1_SET::RFU;
    pub use super::CTRL1_SET::START;
}

/// Temperature Sensor Control Register 1
pub mod CTRL1_TOG {
    pub use super::CTRL1_SET::FINISH_IE;
    pub use super::CTRL1_SET::FREQ;
    pub use super::CTRL1_SET::HIGH_TEMP_IE;
    pub use super::CTRL1_SET::LOW_TEMP_IE;
    pub use super::CTRL1_SET::PANIC_TEMP_IE;
    pub use super::CTRL1_SET::PWD;
    pub use super::CTRL1_SET::PWD_FULL;
    pub use super::CTRL1_SET::RFU;
    pub use super::CTRL1_SET::START;
}

/// Temperature Sensor Range Register 0
pub mod RANGE0 {

    /// Low temperature threshold value
    pub mod LOW_TEMP_VAL {
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

    /// High temperature threshold value
    pub mod HIGH_TEMP_VAL {
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

/// Temperature Sensor Range Register 0
pub mod RANGE0_SET {
    pub use super::RANGE0::HIGH_TEMP_VAL;
    pub use super::RANGE0::LOW_TEMP_VAL;
}

/// Temperature Sensor Range Register 0
pub mod RANGE0_CLR {
    pub use super::RANGE0::HIGH_TEMP_VAL;
    pub use super::RANGE0::LOW_TEMP_VAL;
}

/// Temperature Sensor Range Register 0
pub mod RANGE0_TOG {
    pub use super::RANGE0::HIGH_TEMP_VAL;
    pub use super::RANGE0::LOW_TEMP_VAL;
}

/// Temperature Sensor Range Register 1
pub mod RANGE1 {

    /// Panic temperature threshold value
    pub mod PANIC_TEMP_VAL {
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
}

/// Temperature Sensor Range Register 1
pub mod RANGE1_SET {
    pub use super::RANGE1::PANIC_TEMP_VAL;
}

/// Temperature Sensor Range Register 1
pub mod RANGE1_CLR {
    pub use super::RANGE1::PANIC_TEMP_VAL;
}

/// Temperature Sensor Range Register 1
pub mod RANGE1_TOG {
    pub use super::RANGE1::PANIC_TEMP_VAL;
}

/// Temperature Sensor Status Register 0
pub mod STATUS0 {

    /// Measured temperature value
    pub mod TEMP_VAL {
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

    /// Temperature measurement complete
    pub mod FINISH {
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

            /// 0b0: Temperature sensor is busy (if CTRL1\[START\] = 1)or no new reading has been initiated (if CTRL1\[START\] = 0)
            pub const FINISH_0: u32 = 0b0;

            /// 0b1: Temperature reading is complete and new temperature value available for reading
            pub const FINISH_1: u32 = 0b1;
        }
    }

    /// Low temperature alarm bit
    pub mod LOW_TEMP {
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

            /// 0b0: No Low temperature alert
            pub const LOW_TEMP_0: u32 = 0b0;

            /// 0b1: Low temperature alert
            pub const LOW_TEMP_1: u32 = 0b1;
        }
    }

    /// High temperature alarm bit
    pub mod HIGH_TEMP {
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

            /// 0b0: No High temperature alert
            pub const HIGH_TEMP_0: u32 = 0b0;

            /// 0b1: High temperature alert
            pub const HIGH_TEMP_1: u32 = 0b1;
        }
    }

    /// Panic temperature alarm bit
    pub mod PANIC_TEMP {
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

            /// 0b0: No Panic temperature alert
            pub const PANIC_TEMP_0: u32 = 0b0;

            /// 0b1: Panic temperature alert
            pub const PANIC_TEMP_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Temperature Sensor Control Register 0
    pub CTRL0: RWRegister<u32>,

    /// Temperature Sensor Control Register 0
    pub CTRL0_SET: RWRegister<u32>,

    /// Temperature Sensor Control Register 0
    pub CTRL0_CLR: RWRegister<u32>,

    /// Temperature Sensor Control Register 0
    pub CTRL0_TOG: RWRegister<u32>,

    /// Temperature Sensor Control Register 1
    pub CTRL1: RWRegister<u32>,

    /// Temperature Sensor Control Register 1
    pub CTRL1_SET: RWRegister<u32>,

    /// Temperature Sensor Control Register 1
    pub CTRL1_CLR: RWRegister<u32>,

    /// Temperature Sensor Control Register 1
    pub CTRL1_TOG: RWRegister<u32>,

    /// Temperature Sensor Range Register 0
    pub RANGE0: RWRegister<u32>,

    /// Temperature Sensor Range Register 0
    pub RANGE0_SET: RWRegister<u32>,

    /// Temperature Sensor Range Register 0
    pub RANGE0_CLR: RWRegister<u32>,

    /// Temperature Sensor Range Register 0
    pub RANGE0_TOG: RWRegister<u32>,

    /// Temperature Sensor Range Register 1
    pub RANGE1: RWRegister<u32>,

    /// Temperature Sensor Range Register 1
    pub RANGE1_SET: RWRegister<u32>,

    /// Temperature Sensor Range Register 1
    pub RANGE1_CLR: RWRegister<u32>,

    /// Temperature Sensor Range Register 1
    pub RANGE1_TOG: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// Temperature Sensor Status Register 0
    pub STATUS0: RWRegister<u32>,
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
    pub RANGE0: u32,
    pub RANGE0_SET: u32,
    pub RANGE0_CLR: u32,
    pub RANGE0_TOG: u32,
    pub RANGE1: u32,
    pub RANGE1_SET: u32,
    pub RANGE1_CLR: u32,
    pub RANGE1_TOG: u32,
    pub STATUS0: u32,
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
