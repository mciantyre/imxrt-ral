#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DSI Host DPI Interface
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// PEXEL_PAYLOAD_SIZE
pub mod PIXEL_PAYLOAD_SIZE {

    /// Maximum number of pixels that should be sent as one DSI packet. Recommended to be evenly divisible by the line size (in pixels).
    pub mod PAYLOAD_SIZE {
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

/// PIXEL_FIFO_SEND_LEVEL
pub mod PIXEL_FIFO_SEND_LEVEL {

    /// In order to optimize DSI utility, the DPI bridge buffers a certain number of DPI pixels before initiating a DSI packet. This configuration port controls the level at which the DPI Host bridge begins sending pixels.
    pub mod FIFO_SEND_LEVEL {
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

/// INTERFACE_COLOR_CODING
pub mod INTERFACE_COLOR_CODING {

    /// Sets the distribution of RGB bits within the 24-bit d bus, as specified by the DPI specification.
    pub mod RGB_CONFIG {
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

            /// 0b000: 16-bit Configuration 1
            pub const RGB_CONFIG_0: u32 = 0b000;

            /// 0b001: 16-bit Configuration 2
            pub const RGB_CONFIG_1: u32 = 0b001;

            /// 0b010: 16-bit Configuration 3
            pub const RGB_CONFIG_2: u32 = 0b010;

            /// 0b011: 18-bit Configuration 1
            pub const RGB_CONFIG_3: u32 = 0b011;

            /// 0b100: 18-bit Configuration 2
            pub const RGB_CONFIG_4: u32 = 0b100;

            /// 0b101: 24-bit
            pub const RGB_CONFIG_5: u32 = 0b101;
        }
    }
}

/// PIXEL_FORMAT
pub mod PIXEL_FORMAT {

    /// Sets the DSI packet type of the pixels
    pub mod PIXEL_FORMAT {
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

            /// 0b00: 16 bit
            pub const PIXEL_FORMAT_0: u32 = 0b00;

            /// 0b01: 18 bit
            pub const PIXEL_FORMAT_1: u32 = 0b01;

            /// 0b10: 18 bit loosely packed
            pub const PIXEL_FORMAT_2: u32 = 0b10;

            /// 0b11: 24 bit
            pub const PIXEL_FORMAT_3: u32 = 0b11;
        }
    }
}

/// VSYNC_POLARITY
pub mod VSYNC_POLARITY {

    /// Sets polarity of dpi_vsync_input
    pub mod VSYNC_POLARITY {
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

            /// 0b0: active low
            pub const VSYNC_POLARITY_0: u32 = 0b0;

            /// 0b1: active high
            pub const VSYNC_POLARITY_1: u32 = 0b1;
        }
    }
}

/// HSYNC_POLARITY
pub mod HSYNC_POLARITY {

    /// Sets polarity of dpi_hsync_input
    pub mod HSYNC_POLARITY {
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

            /// 0b0: active low
            pub const HSYNC_POLARITY_0: u32 = 0b0;

            /// 0b1: active high
            pub const HSYNC_POLARITY_1: u32 = 0b1;
        }
    }
}

/// VIDEO_MODE
pub mod VIDEO_MODE {

    /// Select DSI video mode that the host DPI module should generate packets for.
    pub mod VIDEO_MODE {
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

            /// 0b00: Non-Burst mode with Sync Pulses
            pub const VIDEO_MODE_0: u32 = 0b00;

            /// 0b01: Non-Burst mode with Sync Events
            pub const VIDEO_MODE_1: u32 = 0b01;

            /// 0b10: Burst mode
            pub const VIDEO_MODE_2: u32 = 0b10;
        }
    }
}

/// HFP
pub mod HFP {
    pub use super::PIXEL_PAYLOAD_SIZE::PAYLOAD_SIZE;
}

/// HBP
pub mod HBP {
    pub use super::PIXEL_PAYLOAD_SIZE::PAYLOAD_SIZE;
}

/// HSA
pub mod HSA {
    pub use super::PIXEL_PAYLOAD_SIZE::PAYLOAD_SIZE;
}

/// ENABLE_MULT_PKTS
pub mod ENABLE_MULT_PKTS {

    /// Enable Multiple packets per video line. When enabled, PIXEL_PAYLOAD_SIZE\[PAYLOAD_SIZE\] must be set to exactly half the size of the video line
    pub mod ENABLE_MULT_PKTS {
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

            /// 0b0: Video Line is sent in a single packet
            pub const ENABLE_MULT_PKTS_0: u32 = 0b0;

            /// 0b1: Video Line is sent in two packets
            pub const ENABLE_MULT_PKTS_1: u32 = 0b1;
        }
    }
}

/// VBP
pub mod VBP {

    /// Sets the number of lines in the vertical back porch.
    pub mod NUM_LINES {
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

/// VFP
pub mod VFP {
    pub use super::VBP::NUM_LINES;
}

/// BLLP_MODE
pub mod BLLP_MODE {

    /// Optimize bllp periods to Low Power mode when possible
    pub mod LP {
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

            /// 0b0: Blanking packets are sent during BLLP periods
            pub const LP_0: u32 = 0b0;

            /// 0b1: LP mode is used for BLLP periods
            pub const LP_1: u32 = 0b1;
        }
    }
}

/// USE_NULL_PKT_BLLP
pub mod USE_NULL_PKT_BLLP {

    /// Selects type of blanking packet to be sent during bllp
    pub mod NULL {
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

            /// 0b0: Blanking packet used in bllp region 1
            pub const NULL_0: u32 = 0b0;

            /// 0b1: Null packet used in bllp region
            pub const NULL_1: u32 = 0b1;
        }
    }
}

/// VACTIVE
pub mod VACTIVE {

    /// Sets the number of lines in the vertical active aread.
    pub mod NUM_LINES {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
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
    /// PEXEL_PAYLOAD_SIZE
    pub PIXEL_PAYLOAD_SIZE: RWRegister<u32>,

    /// PIXEL_FIFO_SEND_LEVEL
    pub PIXEL_FIFO_SEND_LEVEL: RWRegister<u32>,

    /// INTERFACE_COLOR_CODING
    pub INTERFACE_COLOR_CODING: RWRegister<u32>,

    /// PIXEL_FORMAT
    pub PIXEL_FORMAT: RWRegister<u32>,

    /// VSYNC_POLARITY
    pub VSYNC_POLARITY: RWRegister<u32>,

    /// HSYNC_POLARITY
    pub HSYNC_POLARITY: RWRegister<u32>,

    /// VIDEO_MODE
    pub VIDEO_MODE: RWRegister<u32>,

    /// HFP
    pub HFP: RWRegister<u32>,

    /// HBP
    pub HBP: RWRegister<u32>,

    /// HSA
    pub HSA: RWRegister<u32>,

    /// ENABLE_MULT_PKTS
    pub ENABLE_MULT_PKTS: RWRegister<u32>,

    /// VBP
    pub VBP: RWRegister<u32>,

    /// VFP
    pub VFP: RWRegister<u32>,

    /// BLLP_MODE
    pub BLLP_MODE: RWRegister<u32>,

    /// USE_NULL_PKT_BLLP
    pub USE_NULL_PKT_BLLP: RWRegister<u32>,

    /// VACTIVE
    pub VACTIVE: RWRegister<u32>,
}
pub struct ResetValues {
    pub PIXEL_PAYLOAD_SIZE: u32,
    pub PIXEL_FIFO_SEND_LEVEL: u32,
    pub INTERFACE_COLOR_CODING: u32,
    pub PIXEL_FORMAT: u32,
    pub VSYNC_POLARITY: u32,
    pub HSYNC_POLARITY: u32,
    pub VIDEO_MODE: u32,
    pub HFP: u32,
    pub HBP: u32,
    pub HSA: u32,
    pub ENABLE_MULT_PKTS: u32,
    pub VBP: u32,
    pub VFP: u32,
    pub BLLP_MODE: u32,
    pub USE_NULL_PKT_BLLP: u32,
    pub VACTIVE: u32,
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
