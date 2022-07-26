#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DAC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// Version Identifier Register
pub mod VERID {

    /// Feature Identification Number
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

            /// 0b0000000000000000: Standard feature set
            pub const FEATURE_0: u32 = 0b0000000000000000;

            /// 0b0000000000000001: C40 feature set
            pub const FEATURE_1: u32 = 0b0000000000000001;

            /// 0b0000000000000010: 5V DAC feature set
            pub const FEATURE_2: u32 = 0b0000000000000010;

            /// 0b0000000000000100: ADC BIST feature set
            pub const FEATURE_4: u32 = 0b0000000000000100;
        }
    }

    /// Minor version number
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

    /// Major version number
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

/// Parameter Register
pub mod PARAM {

    /// FIFO size
    pub mod FIFOSZ {
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

            /// 0b000: FIFO depth is 2
            pub const FIFOSZ_0: u32 = 0b000;

            /// 0b001: FIFO depth is 4
            pub const FIFOSZ_1: u32 = 0b001;

            /// 0b010: FIFO depth is 8
            pub const FIFOSZ_2: u32 = 0b010;

            /// 0b011: FIFO depth is 16
            pub const FIFOSZ_3: u32 = 0b011;

            /// 0b100: FIFO depth is 32
            pub const FIFOSZ_4: u32 = 0b100;

            /// 0b101: FIFO depth is 64
            pub const FIFOSZ_5: u32 = 0b101;

            /// 0b110: FIFO depth is 128
            pub const FIFOSZ_6: u32 = 0b110;

            /// 0b111: FIFO depth is 256
            pub const FIFOSZ_7: u32 = 0b111;
        }
    }
}

/// DAC Data Register
pub mod DATA {

    /// FIFO DATA0
    pub mod DATA0 {
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

/// DAC Status and Control Register
pub mod CR {

    /// Full Flag
    pub mod FULLF {
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

            /// 0b0: FIFO is not full.
            pub const FULLF_0: u32 = 0b0;

            /// 0b1: FIFO is full.
            pub const FULLF_1: u32 = 0b1;
        }
    }

    /// Nearly Empty Flag
    pub mod NEMPTF {
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

            /// 0b0: More than one data is available in the FIFO.
            pub const NEMPTF_0: u32 = 0b0;

            /// 0b1: One data is available in the FIFO.
            pub const NEMPTF_1: u32 = 0b1;
        }
    }

    /// FIFO Watermark Status Flag
    pub mod WMF {
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

            /// 0b0: The DAC buffer read pointer has not reached the watermark level.
            pub const WMF_0: u32 = 0b0;

            /// 0b1: The DAC buffer read pointer has reached the watermark level.
            pub const WMF_1: u32 = 0b1;
        }
    }

    /// Underflow Flag
    pub mod UDFF {
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

            /// 0b0: No underflow has occurred since the last time the flag was cleared.
            pub const UDFF_0: u32 = 0b0;

            /// 0b1: At least one trigger underflow has occurred since the last time the flag was cleared.
            pub const UDFF_1: u32 = 0b1;
        }
    }

    /// Overflow Flag
    pub mod OVFF {
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

            /// 0b0: No overflow has occurred since the last time the flag was cleared.
            pub const OVFF_0: u32 = 0b0;

            /// 0b1: At least one FIFO overflow has occurred since the last time the flag was cleared.
            pub const OVFF_1: u32 = 0b1;
        }
    }

    /// Full Interrupt Enable
    pub mod FULLIE {
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

            /// 0b0: FIFO Full interrupt is disabled.
            pub const FULLIE_0: u32 = 0b0;

            /// 0b1: FIFO Full interrupt is enabled.
            pub const FULLIE_1: u32 = 0b1;
        }
    }

    /// Nearly Empty Interrupt Enable
    pub mod EMPTIE {
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

            /// 0b0: FIFO Nearly Empty interrupt is disabled.
            pub const EMPTIE_0: u32 = 0b0;

            /// 0b1: FIFO Nearly Empty interrupt is enabled.
            pub const EMPTIE_1: u32 = 0b1;
        }
    }

    /// Watermark Interrupt Enable
    pub mod WTMIE {
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

            /// 0b0: Watermark interrupt is disabled.
            pub const WTMIE_0: u32 = 0b0;

            /// 0b1: Watermark interrupt is enabled.
            pub const WTMIE_1: u32 = 0b1;
        }
    }

    /// DAC Software Trigger
    pub mod SWTRG {
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

            /// 0b0: The DAC soft trigger is not valid.
            pub const SWTRG_0: u32 = 0b0;

            /// 0b1: The DAC soft trigger is valid.
            pub const SWTRG_1: u32 = 0b1;
        }
    }

    /// DAC Trigger Select
    pub mod TRGSEL {
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

            /// 0b0: The DAC hardware trigger is selected.
            pub const TRGSEL_0: u32 = 0b0;

            /// 0b1: The DAC software trigger is selected.
            pub const TRGSEL_1: u32 = 0b1;
        }
    }

    /// DAC Reference Select
    pub mod DACRFS {
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

            /// 0b0: The DAC selects DACREF_1 as the reference voltage.
            pub const DACRFS_0: u32 = 0b0;

            /// 0b1: The DAC selects DACREF_2 as the reference voltage.
            pub const DACRFS_1: u32 = 0b1;
        }
    }

    /// DAC Enable
    pub mod DACEN {
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

            /// 0b0: The DAC system is disabled.
            pub const DACEN_0: u32 = 0b0;

            /// 0b1: The DAC system is enabled.
            pub const DACEN_1: u32 = 0b1;
        }
    }

    /// FIFO Enable
    pub mod FIFOEN {
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

            /// 0b0: FIFO is disabled and only one level buffer is enabled. Any data written from this buffer goes to conversion.
            pub const FIFOEN_0: u32 = 0b0;

            /// 0b1: FIFO is enabled. Data will first read from FIFO to buffer then go to conversion.
            pub const FIFOEN_1: u32 = 0b1;
        }
    }

    /// DAC FIFO Mode Select
    pub mod SWMD {
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

            /// 0b0: Normal mode
            pub const SWMD_0: u32 = 0b0;

            /// 0b1: Swing back mode
            pub const SWMD_1: u32 = 0b1;
        }
    }

    /// Underflow and overflow interrupt enable
    pub mod UVIE {
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

            /// 0b0: Underflow and overflow interrupt is disabled.
            pub const UVIE_0: u32 = 0b0;

            /// 0b1: Underflow and overflow interrupt is enabled.
            pub const UVIE_1: u32 = 0b1;
        }
    }

    /// FIFO Reset
    pub mod FIFORST {
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

            /// 0b0: No effect
            pub const FIFORST_0: u32 = 0b0;

            /// 0b1: FIFO reset
            pub const FIFORST_1: u32 = 0b1;
        }
    }

    /// Software reset
    pub mod SWRST {
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

    /// DMA Enable Select
    pub mod DMAEN {
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

            /// 0b0: DMA is disabled.
            pub const DMAEN_0: u32 = 0b0;

            /// 0b1: DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time.
            pub const DMAEN_1: u32 = 0b1;
        }
    }

    /// Watermark Level Select
    pub mod WML {
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

/// DAC FIFO Pointer Register
pub mod PTR {

    /// DACWFP
    pub mod DACWFP {
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

    /// DACRFP
    pub mod DACRFP {
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

/// DAC Status and Control Register 2
pub mod CR2 {

    /// Buffer Enable
    pub mod BFEN {
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

            /// 0b0: Opamp is not used as buffer
            pub const BFEN_0: u32 = 0b0;

            /// 0b1: Opamp is used as buffer
            pub const BFEN_1: u32 = 0b1;
        }
    }

    /// Optional Enable
    pub mod OEN {
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

            /// 0b0: Output buffer is not bypassed
            pub const OEN_0: u32 = 0b0;

            /// 0b1: Output buffer is bypassed
            pub const OEN_1: u32 = 0b1;
        }
    }

    /// Buffer Middle Speed Select
    pub mod BFMS {
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

            /// 0b0: Buffer middle speed not selected
            pub const BFMS_0: u32 = 0b0;

            /// 0b1: Buffer middle speed selected
            pub const BFMS_1: u32 = 0b1;
        }
    }

    /// Buffer High Speed Select
    pub mod BFHS {
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

            /// 0b0: Buffer high speed not selected
            pub const BFHS_0: u32 = 0b0;

            /// 0b1: Buffer high speed selected
            pub const BFHS_1: u32 = 0b1;
        }
    }

    /// Internal PTAT (Proportional To Absolute Temperature) Current Reference Select
    pub mod IREF2 {
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

            /// 0b0: Internal PTAT Current Reference not selected
            pub const IREF2_0: u32 = 0b0;

            /// 0b1: Internal PTAT Current Reference selected
            pub const IREF2_1: u32 = 0b1;
        }
    }

    /// Internal ZTC (Zero Temperature Coefficient) Current Reference Select
    pub mod IREF1 {
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

            /// 0b0: Internal ZTC Current Reference not selected
            pub const IREF1_0: u32 = 0b0;

            /// 0b1: Internal ZTC Current Reference selected
            pub const IREF1_1: u32 = 0b1;
        }
    }

    /// Internal Current Reference Select
    pub mod IREF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Internal Current Reference not selected
            pub const IREF_0: u32 = 0b0;

            /// 0b1: Internal Current Reference selected
            pub const IREF_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Version Identifier Register
    pub VERID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    /// DAC Data Register
    pub DATA: WORegister<u32>,

    /// DAC Status and Control Register
    pub CR: RWRegister<u32>,

    /// DAC FIFO Pointer Register
    pub PTR: RORegister<u32>,

    /// DAC Status and Control Register 2
    pub CR2: RWRegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub DATA: u32,
    pub CR: u32,
    pub PTR: u32,
    pub CR2: u32,
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
