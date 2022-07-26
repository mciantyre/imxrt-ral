#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! VIDEO_MUX
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// Video mux Control Register
pub mod VID_MUX_CTRL {

    /// CSI sensor data input mux selector
    pub mod CSI_SEL {
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

            /// 0b0: CSI sensor data is from Parallel CSI
            pub const PARALLEL_CSI: u32 = 0b0;

            /// 0b1: CSI sensor data is from MIPI CSI
            pub const MIPI_CSI: u32 = 0b1;
        }
    }

    /// LCDIF2 sensor data input mux selector
    pub mod LCDIF2_SEL {
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

            /// 0b0: LCDIFv2 sensor data is from Parallel CSI
            pub const PARALLEL_CSI: u32 = 0b0;

            /// 0b1: LCDIFv2 sensor data is from MIPI CSI
            pub const MIPI_CSI: u32 = 0b1;
        }
    }

    /// MIPI DSI video data input mux selector
    pub mod MIPI_DSI_SEL {
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

            /// 0b0: MIPI DSI video data is from eLCDIF
            pub const PARALLEL_CSI: u32 = 0b0;

            /// 0b1: MIPI DSI video data is from LCDIFv2
            pub const MIPI_CSI: u32 = 0b1;
        }
    }

    /// Parallel LCDIF video data input mux selector
    pub mod PARA_LCD_SEL {
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

            /// 0b0: Parallel LCDIF video data is from eLCDIF
            pub const PARALLEL_CSI: u32 = 0b0;

            /// 0b1: Parallel LCDIF video data is from LCDIFv2
            pub const MIPI_CSI: u32 = 0b1;
        }
    }
}

/// Video mux Control Register
pub mod VID_MUX_CTRL_SET {

    /// CSI sensor data input mux selector
    pub mod CSI_SEL {
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

    /// LCDIF2 sensor data input mux selector
    pub mod LCDIF2_SEL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MIPI DSI video data input mux selector
    pub mod MIPI_DSI_SEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Parallel LCDIF video data input mux selector
    pub mod PARA_LCD_SEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Video mux Control Register
pub mod VID_MUX_CTRL_CLR {
    pub use super::VID_MUX_CTRL_SET::CSI_SEL;
    pub use super::VID_MUX_CTRL_SET::LCDIF2_SEL;
    pub use super::VID_MUX_CTRL_SET::MIPI_DSI_SEL;
    pub use super::VID_MUX_CTRL_SET::PARA_LCD_SEL;
}

/// Video mux Control Register
pub mod VID_MUX_CTRL_TOG {
    pub use super::VID_MUX_CTRL_SET::CSI_SEL;
    pub use super::VID_MUX_CTRL_SET::LCDIF2_SEL;
    pub use super::VID_MUX_CTRL_SET::MIPI_DSI_SEL;
    pub use super::VID_MUX_CTRL_SET::PARA_LCD_SEL;
}

/// Pixel Link Master(PLM) Control Register
pub mod PLM_CTRL {

    /// Enable the output of HYSNC and VSYNC
    pub mod ENABLE {
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

            /// 0b0: No active HSYNC and VSYNC output
            pub const NO_ACTIVE: u32 = 0b0;

            /// 0b1: Active HSYNC and VSYNC output
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// VSYNC override
    pub mod VSYNC_OVERRIDE {
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

            /// 0b0: VSYNC is not asserted
            pub const DEASSERT: u32 = 0b0;

            /// 0b1: VSYNC is asserted
            pub const ASSERT: u32 = 0b1;
        }
    }

    /// HSYNC override
    pub mod HSYNC_OVERRIDE {
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

            /// 0b0: HSYNC is not asserted
            pub const DEASSERT: u32 = 0b0;

            /// 0b1: HSYNC is asserted
            pub const ASSERT: u32 = 0b1;
        }
    }

    /// Valid override
    pub mod VALID_OVERRIDE {
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

            /// 0b0: HSYNC and VSYNC is asserted
            pub const ASSERT: u32 = 0b0;

            /// 0b1: HSYNC and VSYNC is not asserted
            pub const DEASSERT: u32 = 0b1;
        }
    }

    /// Polarity of HYSNC/VSYNC
    pub mod POLARITY {
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

            /// 0b0: Keep the current polarity of HSYNC and VSYNC
            pub const KEEP: u32 = 0b0;

            /// 0b1: Invert the polarity of HSYNC and VSYNC
            pub const INVERT: u32 = 0b1;
        }
    }
}

/// Pixel Link Master(PLM) Control Register
pub mod PLM_CTRL_SET {

    /// Enable the output of HYSNC and VSYNC
    pub mod ENABLE {
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

    /// VSYNC override
    pub mod VSYNC_OVERRIDE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSYNC override
    pub mod HSYNC_OVERRIDE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Valid override
    pub mod VALID_OVERRIDE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Polarity of HYSNC/VSYNC
    pub mod POLARITY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Pixel Link Master(PLM) Control Register
pub mod PLM_CTRL_CLR {
    pub use super::PLM_CTRL_SET::ENABLE;
    pub use super::PLM_CTRL_SET::HSYNC_OVERRIDE;
    pub use super::PLM_CTRL_SET::POLARITY;
    pub use super::PLM_CTRL_SET::VALID_OVERRIDE;
    pub use super::PLM_CTRL_SET::VSYNC_OVERRIDE;
}

/// Pixel Link Master(PLM) Control Register
pub mod PLM_CTRL_TOG {
    pub use super::PLM_CTRL_SET::ENABLE;
    pub use super::PLM_CTRL_SET::HSYNC_OVERRIDE;
    pub use super::PLM_CTRL_SET::POLARITY;
    pub use super::PLM_CTRL_SET::VALID_OVERRIDE;
    pub use super::PLM_CTRL_SET::VSYNC_OVERRIDE;
}

/// YUV420 Control Register
pub mod YUV420_CTRL {

    /// Data type of First Line
    pub mod FST_LN_DATA_TYPE {
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

            /// 0b0: Odd (default)
            pub const ODD: u32 = 0b0;

            /// 0b1: Even
            pub const EVEN: u32 = 0b1;
        }
    }
}

/// YUV420 Control Register
pub mod YUV420_CTRL_SET {

    /// Data type of First Line
    pub mod FST_LN_DATA_TYPE {
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

/// YUV420 Control Register
pub mod YUV420_CTRL_CLR {
    pub use super::YUV420_CTRL_SET::FST_LN_DATA_TYPE;
}

/// YUV420 Control Register
pub mod YUV420_CTRL_TOG {
    pub use super::YUV420_CTRL_SET::FST_LN_DATA_TYPE;
}

/// Data Disable Register
pub mod CFG_DT_DISABLE {

    /// Data Type Disable
    pub mod CFG_DT_DISABLE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data Disable Register
pub mod CFG_DT_DISABLE_SET {
    pub use super::CFG_DT_DISABLE::CFG_DT_DISABLE;
}

/// Data Disable Register
pub mod CFG_DT_DISABLE_CLR {
    pub use super::CFG_DT_DISABLE::CFG_DT_DISABLE;
}

/// Data Disable Register
pub mod CFG_DT_DISABLE_TOG {
    pub use super::CFG_DT_DISABLE::CFG_DT_DISABLE;
}

/// MIPI DSI Control Register
pub mod MIPI_DSI_CTRL {

    /// Shut Down - Control to shutdown display (type 4 only)
    pub mod DPI_SD {
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
            pub const NO: u32 = 0b0;

            /// 0b1: Send shutdown command
            pub const SENDCMD: u32 = 0b1;
        }
    }

    /// Color Mode control
    pub mod DPI_CM {
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

            /// 0b0: Normal Mode
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Low-color mode
            pub const LOWCLR: u32 = 0b1;
        }
    }
}

/// MIPI DSI Control Register
pub mod MIPI_DSI_CTRL_SET {

    /// Shut Down - Control to shutdown display (type 4 only)
    pub mod DPI_SD {
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

    /// Color Mode control
    pub mod DPI_CM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MIPI DSI Control Register
pub mod MIPI_DSI_CTRL_CLR {
    pub use super::MIPI_DSI_CTRL_SET::DPI_CM;
    pub use super::MIPI_DSI_CTRL_SET::DPI_SD;
}

/// MIPI DSI Control Register
pub mod MIPI_DSI_CTRL_TOG {
    pub use super::MIPI_DSI_CTRL_SET::DPI_CM;
    pub use super::MIPI_DSI_CTRL_SET::DPI_SD;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Video mux Control Register
    pub VID_MUX_CTRL: RWRegister<u32>,

    /// Video mux Control Register
    pub VID_MUX_CTRL_SET: RWRegister<u32>,

    /// Video mux Control Register
    pub VID_MUX_CTRL_CLR: RWRegister<u32>,

    /// Video mux Control Register
    pub VID_MUX_CTRL_TOG: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// Pixel Link Master(PLM) Control Register
    pub PLM_CTRL: RWRegister<u32>,

    /// Pixel Link Master(PLM) Control Register
    pub PLM_CTRL_SET: RWRegister<u32>,

    /// Pixel Link Master(PLM) Control Register
    pub PLM_CTRL_CLR: RWRegister<u32>,

    /// Pixel Link Master(PLM) Control Register
    pub PLM_CTRL_TOG: RWRegister<u32>,

    /// YUV420 Control Register
    pub YUV420_CTRL: RWRegister<u32>,

    /// YUV420 Control Register
    pub YUV420_CTRL_SET: RWRegister<u32>,

    /// YUV420 Control Register
    pub YUV420_CTRL_CLR: RWRegister<u32>,

    /// YUV420 Control Register
    pub YUV420_CTRL_TOG: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// Data Disable Register
    pub CFG_DT_DISABLE: RWRegister<u32>,

    /// Data Disable Register
    pub CFG_DT_DISABLE_SET: RWRegister<u32>,

    /// Data Disable Register
    pub CFG_DT_DISABLE_CLR: RWRegister<u32>,

    /// Data Disable Register
    pub CFG_DT_DISABLE_TOG: RWRegister<u32>,

    _reserved3: [u32; 4],

    /// MIPI DSI Control Register
    pub MIPI_DSI_CTRL: RWRegister<u32>,

    /// MIPI DSI Control Register
    pub MIPI_DSI_CTRL_SET: RWRegister<u32>,

    /// MIPI DSI Control Register
    pub MIPI_DSI_CTRL_CLR: RWRegister<u32>,

    /// MIPI DSI Control Register
    pub MIPI_DSI_CTRL_TOG: RWRegister<u32>,
}
pub struct ResetValues {
    pub VID_MUX_CTRL: u32,
    pub VID_MUX_CTRL_SET: u32,
    pub VID_MUX_CTRL_CLR: u32,
    pub VID_MUX_CTRL_TOG: u32,
    pub PLM_CTRL: u32,
    pub PLM_CTRL_SET: u32,
    pub PLM_CTRL_CLR: u32,
    pub PLM_CTRL_TOG: u32,
    pub YUV420_CTRL: u32,
    pub YUV420_CTRL_SET: u32,
    pub YUV420_CTRL_CLR: u32,
    pub YUV420_CTRL_TOG: u32,
    pub CFG_DT_DISABLE: u32,
    pub CFG_DT_DISABLE_SET: u32,
    pub CFG_DT_DISABLE_CLR: u32,
    pub CFG_DT_DISABLE_TOG: u32,
    pub MIPI_DSI_CTRL: u32,
    pub MIPI_DSI_CTRL_SET: u32,
    pub MIPI_DSI_CTRL_CLR: u32,
    pub MIPI_DSI_CTRL_TOG: u32,
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
