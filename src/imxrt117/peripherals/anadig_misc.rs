#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Chip Silicon Version Register
pub mod MISC_DIFPROG {

    /// Chip ID
    pub mod CHIPID {
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

/// VDDSOC_AI_CTRL_REGISTER
pub mod VDDSOC_AI_CTRL {

    /// VDDSOC_AI_ADDR
    pub mod VDDSOC_AI_ADDR {
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

    /// VDDSOC_AIRWB
    pub mod VDDSOC_AIRWB {
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
}

/// VDDSOC_AI_WDATA_REGISTER
pub mod VDDSOC_AI_WDATA {

    /// VDDSOC_AI_WDATA
    pub mod VDDSOC_AI_WDATA {
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

/// VDDSOC_AI_RDATA_REGISTER
pub mod VDDSOC_AI_RDATA {

    /// VDDSOC_AI_RDATA
    pub mod VDDSOC_AI_RDATA {
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

/// VDDSOC2PLL_AI_CTRL_1G_REGISTER
pub mod VDDSOC2PLL_AI_CTRL_1G {

    /// VDDSOC2PLL_AIADDR_1G
    pub mod VDDSOC2PLL_AIADDR_1G {
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

    /// VDDSOC2PLL_AITOGGLE_1G
    pub mod VDDSOC2PLL_AITOGGLE_1G {
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

    /// VDDSOC2PLL_AITOGGLE_DONE_1G
    pub mod VDDSOC2PLL_AITOGGLE_DONE_1G {
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

    /// VDDSOC2PLL_AIRWB_1G
    pub mod VDDSOC2PLL_AIRWB_1G {
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
}

/// VDDSOC2PLL_AI_WDATA_1G_REGISTER
pub mod VDDSOC2PLL_AI_WDATA_1G {

    /// VDDSOC2PLL_AI_WDATA_1G
    pub mod VDDSOC2PLL_AI_WDATA_1G {
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

/// VDDSOC2PLL_AI_RDATA_1G_REGISTER
pub mod VDDSOC2PLL_AI_RDATA_1G {

    /// VDDSOC2PLL_AI_RDATA_1G
    pub mod VDDSOC2PLL_AI_RDATA_1G {
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

/// VDDSOC_AI_CTRL_AUDIO_REGISTER
pub mod VDDSOC2PLL_AI_CTRL_AUDIO {

    /// VDDSOC2PLL_AI_ADDR_AUDIO
    pub mod VDDSOC2PLL_AI_ADDR_AUDIO {
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

    /// VDDSOC2PLL_AITOGGLE_AUDIO
    pub mod VDDSOC2PLL_AITOGGLE_AUDIO {
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

    /// VDDSOC2PLL_AITOGGLE_DONE_AUDIO
    pub mod VDDSOC2PLL_AITOGGLE_DONE_AUDIO {
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

    /// VDDSOC_AIRWB
    pub mod VDDSOC2PLL_AIRWB_AUDIO {
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
}

/// VDDSOC_AI_WDATA_AUDIO_REGISTER
pub mod VDDSOC2PLL_AI_WDATA_AUDIO {

    /// VDDSOC2PLL_AI_WDATA_AUDIO
    pub mod VDDSOC2PLL_AI_WDATA_AUDIO {
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

/// VDDSOC2PLL_AI_RDATA_REGISTER
pub mod VDDSOC2PLL_AI_RDATA_AUDIO {

    /// VDDSOC2PLL_AI_RDATA_AUDIO
    pub mod VDDSOC2PLL_AI_RDATA_AUDIO {
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

/// VDDSOC2PLL_AI_CTRL_VIDEO_REGISTER
pub mod VDDSOC2PLL_AI_CTRL_VIDEO {

    /// VDDSOC2PLL_AIADDR_VIDEO
    pub mod VDDSOC2PLL_AIADDR_VIDEO {
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

    /// VDDSOC2PLL_AITOGGLE_VIDEO
    pub mod VDDSOC2PLL_AITOGGLE_VIDEO {
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

    /// VDDSOC2PLL_AITOGGLE_DONE_VIDEO
    pub mod VDDSOC2PLL_AITOGGLE_DONE_VIDEO {
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

    /// VDDSOC2PLL_AIRWB_VIDEO
    pub mod VDDSOC2PLL_AIRWB_VIDEO {
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
}

/// VDDSOC2PLL_AI_WDATA_VIDEO_REGISTER
pub mod VDDSOC2PLL_AI_WDATA_VIDEO {

    /// VDDSOC2PLL_AI_WDATA_VIDEO
    pub mod VDDSOC2PLL_AI_WDATA_VIDEO {
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

/// VDDSOC2PLL_AI_RDATA_VIDEO_REGISTER
pub mod VDDSOC2PLL_AI_RDATA_VIDEO {

    /// VDDSOC2PLL_AI_RDATA_VIDEO
    pub mod VDDSOC2PLL_AI_RDATA_VIDEO {
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

/// VDDSOC_AI_CTRL_REGISTER
pub mod VDDLPSR_AI_CTRL {

    /// VDDLPSR_AI_ADDR
    pub mod VDDLPSR_AI_ADDR {
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

    /// VDDLPSR_AIRWB
    pub mod VDDLPSR_AIRWB {
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
}

/// VDDLPSR_AI_WDATA_REGISTER
pub mod VDDLPSR_AI_WDATA {

    /// VDD_LPSR_AI_WDATA
    pub mod VDDLPSR_AI_WDATA {
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

/// VDDLPSR_AI_RDATA_REFTOP_REGISTER
pub mod VDDLPSR_AI_RDATA_REFTOP {

    /// VDDLPSR_AI_RDATA_REFTOP
    pub mod VDDLPSR_AI_RDATA_REFTOP {
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

/// VDDLPSR_AI_RDATA_TMPSNS_REGISTER
pub mod VDDLPSR_AI_RDATA_TMPSNS {

    /// VDDLPSR_AI_RDATA_TMPSNS
    pub mod VDDLPSR_AI_RDATA_TMPSNS {
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

/// VDDLPSR_AI400M_CTRL_REGISTER
pub mod VDDLPSR_AI400M_CTRL {

    /// VDDLPSR_AI400M_ADDR
    pub mod VDDLPSR_AI400M_ADDR {
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

    /// VDDLPSR_AITOGGLE_400M
    pub mod VDDLPSR_AITOGGLE_400M {
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

    /// VDDLPSR_AITOGGLE_DONE_400M
    pub mod VDDLPSR_AITOGGLE_DONE_400M {
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

    /// VDDLPSR_AI400M_RWB
    pub mod VDDLPSR_AI400M_RWB {
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
}

/// VDDLPSR_AI400M_WDATA_REGISTER
pub mod VDDLPSR_AI400M_WDATA {

    /// VDDLPSR_AI400M_WDATA
    pub mod VDDLPSR_AI400M_WDATA {
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

/// VDDLPSR_AI400M_RDATA_REGISTER
pub mod VDDLPSR_AI400M_RDATA {

    /// VDDLPSR_AI400M_RDATA
    pub mod VDDLPSR_AI400M_RDATA {
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
    _reserved1: [u32; 512],

    /// Chip Silicon Version Register
    pub MISC_DIFPROG: RORegister<u32>,

    _reserved2: [u32; 7],

    /// VDDSOC_AI_CTRL_REGISTER
    pub VDDSOC_AI_CTRL: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// VDDSOC_AI_WDATA_REGISTER
    pub VDDSOC_AI_WDATA: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// VDDSOC_AI_RDATA_REGISTER
    pub VDDSOC_AI_RDATA: RORegister<u32>,

    _reserved5: [u32; 3],

    /// VDDSOC2PLL_AI_CTRL_1G_REGISTER
    pub VDDSOC2PLL_AI_CTRL_1G: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// VDDSOC2PLL_AI_WDATA_1G_REGISTER
    pub VDDSOC2PLL_AI_WDATA_1G: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// VDDSOC2PLL_AI_RDATA_1G_REGISTER
    pub VDDSOC2PLL_AI_RDATA_1G: RORegister<u32>,

    _reserved8: [u32; 3],

    /// VDDSOC_AI_CTRL_AUDIO_REGISTER
    pub VDDSOC2PLL_AI_CTRL_AUDIO: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// VDDSOC_AI_WDATA_AUDIO_REGISTER
    pub VDDSOC2PLL_AI_WDATA_AUDIO: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// VDDSOC2PLL_AI_RDATA_REGISTER
    pub VDDSOC2PLL_AI_RDATA_AUDIO: RORegister<u32>,

    _reserved11: [u32; 3],

    /// VDDSOC2PLL_AI_CTRL_VIDEO_REGISTER
    pub VDDSOC2PLL_AI_CTRL_VIDEO: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// VDDSOC2PLL_AI_WDATA_VIDEO_REGISTER
    pub VDDSOC2PLL_AI_WDATA_VIDEO: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// VDDSOC2PLL_AI_RDATA_VIDEO_REGISTER
    pub VDDSOC2PLL_AI_RDATA_VIDEO: RORegister<u32>,

    _reserved14: [u32; 3],

    /// VDDSOC_AI_CTRL_REGISTER
    pub VDDLPSR_AI_CTRL: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// VDDLPSR_AI_WDATA_REGISTER
    pub VDDLPSR_AI_WDATA: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// VDDLPSR_AI_RDATA_REFTOP_REGISTER
    pub VDDLPSR_AI_RDATA_REFTOP: RORegister<u32>,

    _reserved17: [u32; 3],

    /// VDDLPSR_AI_RDATA_TMPSNS_REGISTER
    pub VDDLPSR_AI_RDATA_TMPSNS: RORegister<u32>,

    _reserved18: [u32; 3],

    /// VDDLPSR_AI400M_CTRL_REGISTER
    pub VDDLPSR_AI400M_CTRL: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// VDDLPSR_AI400M_WDATA_REGISTER
    pub VDDLPSR_AI400M_WDATA: RWRegister<u32>,

    _reserved20: [u32; 3],

    /// VDDLPSR_AI400M_RDATA_REGISTER
    pub VDDLPSR_AI400M_RDATA: RORegister<u32>,
}
pub struct ResetValues {
    pub MISC_DIFPROG: u32,
    pub VDDSOC_AI_CTRL: u32,
    pub VDDSOC_AI_WDATA: u32,
    pub VDDSOC_AI_RDATA: u32,
    pub VDDSOC2PLL_AI_CTRL_1G: u32,
    pub VDDSOC2PLL_AI_WDATA_1G: u32,
    pub VDDSOC2PLL_AI_RDATA_1G: u32,
    pub VDDSOC2PLL_AI_CTRL_AUDIO: u32,
    pub VDDSOC2PLL_AI_WDATA_AUDIO: u32,
    pub VDDSOC2PLL_AI_RDATA_AUDIO: u32,
    pub VDDSOC2PLL_AI_CTRL_VIDEO: u32,
    pub VDDSOC2PLL_AI_WDATA_VIDEO: u32,
    pub VDDSOC2PLL_AI_RDATA_VIDEO: u32,
    pub VDDLPSR_AI_CTRL: u32,
    pub VDDLPSR_AI_WDATA: u32,
    pub VDDLPSR_AI_RDATA_REFTOP: u32,
    pub VDDLPSR_AI_RDATA_TMPSNS: u32,
    pub VDDLPSR_AI400M_CTRL: u32,
    pub VDDLPSR_AI400M_WDATA: u32,
    pub VDDLPSR_AI400M_RDATA: u32,
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
