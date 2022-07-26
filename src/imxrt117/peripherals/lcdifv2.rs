#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LCDIF_V2
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// LCDIFv2 display control Register
pub mod CTRL {

    /// Invert Horizontal synchronization signal
    pub mod INV_HS {
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

            /// 0b0: HSYNC signal not inverted (active HIGH)
            pub const INV_HS_0: u32 = 0b0;

            /// 0b1: Invert HSYNC signal (active LOW)
            pub const INV_HS_1: u32 = 0b1;
        }
    }

    /// Invert Vertical synchronization signal
    pub mod INV_VS {
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

            /// 0b0: VSYNC signal not inverted (active HIGH)
            pub const INV_VS_0: u32 = 0b0;

            /// 0b1: Invert VSYNC signal (active LOW)
            pub const INV_VS_1: u32 = 0b1;
        }
    }

    /// Invert Data Enable polarity
    pub mod INV_DE {
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

            /// 0b0: Data enable is active high
            pub const INV_DE_0: u32 = 0b0;

            /// 0b1: Data enable is active low
            pub const INV_DE_1: u32 = 0b1;
        }
    }

    /// Polarity change of Pixel Clock
    pub mod INV_PXCK {
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

            /// 0b0: Display samples data on the falling edge
            pub const INV_PXCK_0: u32 = 0b0;

            /// 0b1: Display samples data on the rising edge
            pub const INV_PXCK_1: u32 = 0b1;
        }
    }

    /// Indicates if value at the output (pixel data output) needs to be negated
    pub mod NEG {
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

            /// 0b0: Output is to remain same
            pub const NEG_0: u32 = 0b0;

            /// 0b1: Output to be negated
            pub const NEG_1: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod SW_RESET {
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

            /// 0b0: No action
            pub const SW_RESET_0: u32 = 0b0;

            /// 0b1: All LCDIFv2 internal registers are forced into their reset state. User registers are not affected
            pub const SW_RESET_1: u32 = 0b1;
        }
    }
}

/// LCDIFv2 display control Register
pub mod CTRL_SET {

    /// Invert Horizontal synchronization signal
    pub mod INV_HS {
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

    /// Invert Vertical synchronization signal
    pub mod INV_VS {
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

    /// Invert Data Enable polarity
    pub mod INV_DE {
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

    /// Polarity change of Pixel Clock
    pub mod INV_PXCK {
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

    /// Indicates if value at the output (pixel data output) needs to be negated
    pub mod NEG {
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

    /// Software Reset
    pub mod SW_RESET {
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

/// LCDIFv2 display control Register
pub mod CTRL_CLR {
    pub use super::CTRL_SET::INV_DE;
    pub use super::CTRL_SET::INV_HS;
    pub use super::CTRL_SET::INV_PXCK;
    pub use super::CTRL_SET::INV_VS;
    pub use super::CTRL_SET::NEG;
    pub use super::CTRL_SET::SW_RESET;
}

/// LCDIFv2 display control Register
pub mod CTRL_TOG {
    pub use super::CTRL_SET::INV_DE;
    pub use super::CTRL_SET::INV_HS;
    pub use super::CTRL_SET::INV_PXCK;
    pub use super::CTRL_SET::INV_VS;
    pub use super::CTRL_SET::NEG;
    pub use super::CTRL_SET::SW_RESET;
}

/// Display Parameter Register
pub mod DISP_PARA {

    /// Blue component of the default color displayed in the sectors where no layer is active
    pub mod BGND_B {
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

    /// Green component of the default color displayed in the sectors where no layer is active
    pub mod BGND_G {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Red component of the default color displayed in the sectors where no layer is active
    pub mod BGND_R {
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

    /// LCDIFv2 operating mode
    pub mod DISP_MODE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Normal mode. Panel content controlled by layer configuration
            pub const DISP_MODE_0: u32 = 0b00;

            /// 0b01: Test Mode1(BGND Color Display)
            pub const DISP_MODE_1: u32 = 0b01;

            /// 0b10: Test Mode2(Column Color Bar)
            pub const DISP_MODE_2: u32 = 0b10;

            /// 0b11: Test Mode3(Row Color Bar)
            pub const DISP_MODE_3: u32 = 0b11;
        }
    }

    /// LCDIFv2 line output order
    pub mod LINE_PATTERN {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (3 bits: 0b111 << 26)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: RGB
            pub const LINE_PATTERN_0: u32 = 0b000;

            /// 0b001: RBG
            pub const LINE_PATTERN_1: u32 = 0b001;

            /// 0b010: GBR
            pub const LINE_PATTERN_2: u32 = 0b010;

            /// 0b011: GRB
            pub const LINE_PATTERN_3: u32 = 0b011;

            /// 0b100: BRG
            pub const LINE_PATTERN_4: u32 = 0b100;

            /// 0b101: BGR
            pub const LINE_PATTERN_5: u32 = 0b101;
        }
    }

    /// Display panel On/Off mode
    pub mod DISP_ON {
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

            /// 0b0: Display Off
            pub const DISP_ON_0: u32 = 0b0;

            /// 0b1: Display On
            pub const DISP_ON_1: u32 = 0b1;
        }
    }
}

/// Display Size Register
pub mod DISP_SIZE {

    /// Sets the display size horizontal resolution in pixels
    pub mod DELTA_X {
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

    /// Sets the display size vertical resolution in pixels
    pub mod DELTA_Y {
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

/// Horizontal Sync Parameter Register
pub mod HSYN_PARA {

    /// HSYNC front-porch pulse width (in pixel clock cycles). Pulse width has a minimum value of 1
    pub mod FP_H {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1
    pub mod PW_H {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (9 bits: 0x1ff << 11)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSYNC back-porch pulse width (in pixel clock cycles). Pulse width has a minimum value of 1
    pub mod BP_H {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (9 bits: 0x1ff << 22)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Vertical Sync Parameter Register
pub mod VSYN_PARA {

    /// VSYNC front-porch pulse width (in horizontal line cycles). Pulse width has a minimum value of 1
    pub mod FP_V {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1
    pub mod PW_V {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (9 bits: 0x1ff << 11)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VSYNC back-porch pulse width (in horizontal line cycles). Pulse width has a minimum value of 1
    pub mod BP_V {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (9 bits: 0x1ff << 22)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Status Register for domain 0
pub mod INT_STATUS_D0 {

    /// Interrupt flag to indicate that the vertical synchronization phase(The beginning of a frame)
    pub mod VSYNC {
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

            /// 0b0: VSYNC has not started
            pub const VSYNC_0: u32 = 0b0;

            /// 0b1: VSYNC has started
            pub const VSYNC_1: u32 = 0b1;
        }
    }

    /// Interrupt flag to indicate the output buffer underrun condition
    pub mod UNDERRUN {
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

            /// 0b0: Output buffer not underrun
            pub const UNDERRUN_0: u32 = 0b0;

            /// 0b1: Output buffer underrun
            pub const UNDERRUN_1: u32 = 0b1;
        }
    }

    /// Interrupt flag to indicate vertical blanking period
    pub mod VS_BLANK {
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

            /// 0b0: Vertical blanking period has not started
            pub const VS_BLANK_0: u32 = 0b0;

            /// 0b1: Vertical blanking period has started
            pub const VS_BLANK_1: u32 = 0b1;
        }
    }

    /// Interrupt flag to indicate that which PLANE has Read Error on the AXI interface
    pub mod DMA_ERR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt flag to indicate that which PLANE has fetched the last pixel from memory
    pub mod DMA_DONE {
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

    /// Interrupt flag to indicate that which FIFO in the pixel blending underflowed
    pub mod FIFO_EMPTY {
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

/// Interrupt Enable Register for domain 0
pub mod INT_ENABLE_D0 {

    /// Enable Interrupt flag to indicate that the vertical synchronization phase(The beginning of a frame)
    pub mod VSYNC_EN {
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

            /// 0b0: VSYNC interrupt disable
            pub const VSYNC_EN_0: u32 = 0b0;

            /// 0b1: VSYNC interrupt enable
            pub const VSYNC_EN_1: u32 = 0b1;
        }
    }

    /// Enable Interrupt flag to indicate the output buffer underrun condition
    pub mod UNDERRUN_EN {
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

            /// 0b0: Output buffer underrun disable
            pub const UNDERRUN_EN_0: u32 = 0b0;

            /// 0b1: Output buffer underrun enable
            pub const UNDERRUN_EN_1: u32 = 0b1;
        }
    }

    /// Enable Interrupt flag to indicate vertical blanking period
    pub mod VS_BLANK_EN {
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

            /// 0b0: Vertical blanking start interrupt disable
            pub const VS_BLANK_EN_0: u32 = 0b0;

            /// 0b1: Vertical blanking start interrupt enable
            pub const VS_BLANK_EN_1: u32 = 0b1;
        }
    }

    /// Enable Interrupt flag to indicate that which PLANE has Read Error on the AXI interface
    pub mod DMA_ERR_EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable Interrupt flag to indicate that which PLANE has fetched the last pixel from memory
    pub mod DMA_DONE_EN {
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

    /// Enable Interrupt flag to indicate that which FIFO in the pixel blending underflowed
    pub mod FIFO_EMPTY_EN {
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

/// Interrupt Status Register for domain 1
pub mod INT_STATUS_D1 {

    /// Interrupt flag to indicate that the vertical synchronization phase(The beginning of a frame)
    pub mod VSYNC {
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

    /// Interrupt flag to indicate the output buffer underrun condition
    pub mod UNDERRUN {
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

    /// Interrupt flag to indicate vertical blanking period
    pub mod VS_BLANK {
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

    /// Interrupt flag to indicate that which PLANE has Read Error on the AXI interface
    pub mod DMA_ERR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt flag to indicate that which PLANE has fetched the last pixel from memory
    pub mod DMA_DONE {
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

    /// Interrupt flag to indicate that which FIFO in the pixel blending underflowed
    pub mod FIFO_EMPTY {
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

/// Interrupt Enable Register for domain 1
pub mod INT_ENABLE_D1 {

    /// Enable Interrupt flag to indicate that the vertical synchronization phase(The beginning of a frame)
    pub mod VSYNC_EN {
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

    /// Enable Interrupt flag to indicate the output buffer underrun condition
    pub mod UNDERRUN_EN {
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

    /// Enable Interrupt flag to indicate vertical blanking period
    pub mod VS_BLANK_EN {
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

    /// Enable Interrupt flag to indicate that which PLANE has Read Error on the AXI interface
    pub mod DMA_ERR_EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable Interrupt flag to indicate that which PLANE has fetched the last pixel from memory
    pub mod DMA_DONE_EN {
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

    /// Enable Interrupt flag to indicate that which FIFO in the pixel blending underflowed
    pub mod FIFO_EMPTY_EN {
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

/// Parallel Data Interface Parameter Register
pub mod PDI_PARA {

    /// Polarity of PDI input HSYNC
    pub mod INV_PDI_HS {
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

            /// 0b0: HSYNC is active HIGH
            pub const INV_PDI_HS_0: u32 = 0b0;

            /// 0b1: HSYNC is active LOW
            pub const INV_PDI_HS_1: u32 = 0b1;
        }
    }

    /// Polarity of PDI input VSYNC
    pub mod INV_PDI_VS {
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

            /// 0b0: VSYNC is active HIGH
            pub const INV_PDI_VS_0: u32 = 0b0;

            /// 0b1: VSYNC is active LOW
            pub const INV_PDI_VS_1: u32 = 0b1;
        }
    }

    /// Polarity of PDI input Data Enable
    pub mod INV_PDI_DE {
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

            /// 0b0: Data enable is active HIGH
            pub const INV_PDI_DE_0: u32 = 0b0;

            /// 0b1: Data enable is active LOW
            pub const INV_PDI_DE_1: u32 = 0b1;
        }
    }

    /// Polarity of PDI input Pixel Clock
    pub mod INV_PDI_PXCK {
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

            /// 0b0: Samples data on the falling edge
            pub const INV_PDI_PXCK_0: u32 = 0b0;

            /// 0b1: Samples data on the rising edge
            pub const INV_PDI_PXCK_1: u32 = 0b1;
        }
    }

    /// The PDI mode for input data format
    pub mod MODE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 32 bpp (ARGB8888)
            pub const MODE_0: u32 = 0b0000;

            /// 0b0001: 24 bpp (RGB888)
            pub const MODE_1: u32 = 0b0001;

            /// 0b0010: 24 bpp (RGB666)
            pub const MODE_2: u32 = 0b0010;

            /// 0b0011: 16 bpp (RGB565)
            pub const MODE_3: u32 = 0b0011;

            /// 0b0100: 16 bpp (RGB444)
            pub const MODE_4: u32 = 0b0100;

            /// 0b0101: 16 bpp (RGB555)
            pub const MODE_5: u32 = 0b0101;

            /// 0b0110: 16 bpp (YCbCr422)
            pub const MODE_6: u32 = 0b0110;
        }
    }

    /// PDI selected on LCDIFv2 plane number
    pub mod PDI_SEL {
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

            /// 0b0: PDI selected on LCDIFv2 plane 0
            pub const PDI_SEL_0: u32 = 0b0;

            /// 0b1: PDI selected on LCDIFv2 plane 1
            pub const PDI_SEL_1: u32 = 0b1;
        }
    }

    /// Enable PDI input data to LCDIFv2 display
    pub mod PDI_EN {
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

            /// 0b0: Disable PDI input data
            pub const PDI_EN_0: u32 = 0b0;

            /// 0b1: Enable PDI input data
            pub const PDI_EN_1: u32 = 0b1;
        }
    }
}

/// Control Descriptor Layer 1 Register
pub mod CTRLDESCL0_1 {

    /// Width of the layer in pixels
    pub mod WIDTH {
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

    /// Height of the layer in pixels
    pub mod HEIGHT {
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

/// Control Descriptor Layer 2 Register
pub mod CTRLDESCL0_2 {

    /// The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, only positive values are to the right the left-hand column of the panel
    pub mod POSX {
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

    /// The vertical position of top row of the layer, where 0 is the top row of the panel, only positive values are below the top row of the panel
    pub mod POSY {
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

/// Control Descriptor Layer 3 Register
pub mod CTRLDESCL0_3 {

    /// Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry
    pub mod PITCH {
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

/// Control Descriptor Layer 4 Register
pub mod CTRLDESCL0_4 {

    /// Address of layer data in the memory. The address programmed should be 64-bit aligned
    pub mod ADDR {
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

/// Control Descriptor Layer 5 Register
pub mod CTRLDESCL0_5 {

    /// Alpha Blending Mode
    pub mod AB_MODE {
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

            /// 0b00: No alpha Blending (The SAFETY_EN bit need set to 1)
            pub const AB_MODE_0: u32 = 0b00;

            /// 0b01: Blend with global ALPHA
            pub const AB_MODE_1: u32 = 0b01;

            /// 0b10: Blend with embedded ALPHA
            pub const AB_MODE_2: u32 = 0b10;

            /// 0b11: Blend with PoterDuff enable
            pub const AB_MODE_3: u32 = 0b11;
        }
    }

    /// PoterDuff factor mode
    pub mod PD_FACTOR_MODE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Using 1
            pub const PD_FACTOR_MODE_0: u32 = 0b00;

            /// 0b01: Using 0
            pub const PD_FACTOR_MODE_1: u32 = 0b01;

            /// 0b10: Using straight alpha
            pub const PD_FACTOR_MODE_2: u32 = 0b10;

            /// 0b11: Using inverse alpha
            pub const PD_FACTOR_MODE_3: u32 = 0b11;
        }
    }

    /// PoterDuff global alpha mode
    pub mod PD_GLOBAL_ALPHA_MODE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Using global alpha
            pub const PD_GLOBAL_ALPHA_MODE_0: u32 = 0b00;

            /// 0b01: Using local alpha
            pub const PD_GLOBAL_ALPHA_MODE_1: u32 = 0b01;

            /// 0b10: Using scaled alpha
            pub const PD_GLOBAL_ALPHA_MODE_2: u32 = 0b10;

            /// 0b11: Using scaled alpha
            pub const PD_GLOBAL_ALPHA_MODE_3: u32 = 0b11;
        }
    }

    /// PoterDuff alpha mode
    pub mod PD_ALPHA_MODE {
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

            /// 0b0: Straight mode for Porter Duff alpha
            pub const PD_ALPHA_MODE_0: u32 = 0b0;

            /// 0b1: Inversed mode for Porter Duff alpha
            pub const PD_ALPHA_MODE_1: u32 = 0b1;
        }
    }

    /// PoterDuff alpha mode
    pub mod PD_COLOR_MODE {
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

            /// 0b0: Straight mode for Porter Duff color
            pub const PD_COLOR_MODE_0: u32 = 0b0;

            /// 0b1: Inversed mode for Porter Duff color
            pub const PD_COLOR_MODE_1: u32 = 0b1;
        }
    }

    /// The YUV422 input format selection
    pub mod YUV_FORMAT {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The YVYU422 8bit sequence is U1,Y1,V1,Y2
            pub const YUV_FORMAT_0: u32 = 0b00;

            /// 0b01: The YVYU422 8bit sequence is V1,Y1,U1,Y2
            pub const YUV_FORMAT_1: u32 = 0b01;

            /// 0b10: The YVYU422 8bit sequence is Y1,U1,Y2,V1
            pub const YUV_FORMAT_2: u32 = 0b10;

            /// 0b11: The YVYU422 8bit sequence is Y1,V1,Y2,U1
            pub const YUV_FORMAT_3: u32 = 0b11;
        }
    }

    /// Global Alpha
    pub mod GLOBAL_ALPHA {
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

    /// Layer encoding format (bit per pixel)
    pub mod BPP {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 1 bpp
            pub const BPP_0: u32 = 0b0000;

            /// 0b0001: 2 bpp
            pub const BPP_1: u32 = 0b0001;

            /// 0b0010: 4 bpp
            pub const BPP_2: u32 = 0b0010;

            /// 0b0011: 8 bpp
            pub const BPP_3: u32 = 0b0011;

            /// 0b0100: 16 bpp (RGB565)
            pub const BPP_4: u32 = 0b0100;

            /// 0b0101: 16 bpp (ARGB1555)
            pub const BPP_5: u32 = 0b0101;

            /// 0b0110: 16 bpp (ARGB4444)
            pub const BPP_6: u32 = 0b0110;

            /// 0b0111: YCbCr422 (Only layer 0/1 can support this format)
            pub const BPP_7: u32 = 0b0111;

            /// 0b1000: 24 bpp (RGB888)
            pub const BPP_8: u32 = 0b1000;

            /// 0b1001: 32 bpp (ARGB8888)
            pub const BPP_9: u32 = 0b1001;

            /// 0b1010: 32 bpp (ABGR8888)
            pub const BPP_10: u32 = 0b1010;
        }
    }

    /// Safety Mode Enable Bit
    pub mod SAFETY_EN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Safety Mode is disabled
            pub const SAFETY_EN_0: u32 = 0b0;

            /// 0b1: Safety Mode is enabled for this layer
            pub const SAFETY_EN_1: u32 = 0b1;
        }
    }

    /// Shadow Load Enable
    pub mod SHADOW_LOAD_EN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable the layer for DMA
    pub mod EN {
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

            /// 0b0: OFF
            pub const EN_0: u32 = 0b0;

            /// 0b1: ON
            pub const EN_1: u32 = 0b1;
        }
    }
}

/// Control Descriptor Layer 6 Register
pub mod CTRLDESCL0_6 {

    /// Background B component value
    pub mod BCLR_B {
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

    /// Background G component value
    pub mod BCLR_G {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Background R component value
    pub mod BCLR_R {
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

/// Color Space Conversion Coefficient Register 0
pub mod CSC0_COEF0 {

    /// Two's compliment amplitude offset implicit in the Y data. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)
    pub mod Y_OFFSET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Two's compliment phase offset implicit for CbCr data. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)
    pub mod UV_OFFSET {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (9 bits: 0x1ff << 9)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)
    pub mod C0 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (11 bits: 0x7ff << 18)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable the CSC unit in the LCDIFv2 plane data path
    pub mod ENABLE {
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

            /// 0b0: The CSC is bypassed and the input pixels are RGB data already
            pub const ENABLE_0: u32 = 0b0;

            /// 0b1: The CSC is enabled and the pixels will be converted to RGB data
            pub const ENABLE_1: u32 = 0b1;
        }
    }

    /// This bit changes the behavior when performing U/V converting
    pub mod YCBCR_MODE {
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

            /// 0b0: Converting YUV to RGB data
            pub const YCBCR_MODE_0: u32 = 0b0;

            /// 0b1: Converting YCbCr to RGB data
            pub const YCBCR_MODE_1: u32 = 0b1;
        }
    }
}

/// Color Space Conversion Coefficient Register 1
pub mod CSC0_COEF1 {

    /// Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)
    pub mod C4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)
    pub mod C1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Color Space Conversion Coefficient Register 2
pub mod CSC0_COEF2 {

    /// Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)
    pub mod C3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)
    pub mod C2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Control Descriptor Layer 1 Register
pub mod CTRLDESCL1_1 {
    pub use super::CTRLDESCL0_1::HEIGHT;
    pub use super::CTRLDESCL0_1::WIDTH;
}

/// Control Descriptor Layer 2 Register
pub mod CTRLDESCL1_2 {
    pub use super::CTRLDESCL0_2::POSX;
    pub use super::CTRLDESCL0_2::POSY;
}

/// Control Descriptor Layer 3 Register
pub mod CTRLDESCL1_3 {
    pub use super::CTRLDESCL0_3::PITCH;
}

/// Control Descriptor Layer 4 Register
pub mod CTRLDESCL1_4 {
    pub use super::CTRLDESCL0_4::ADDR;
}

/// Control Descriptor Layer 5 Register
pub mod CTRLDESCL1_5 {
    pub use super::CTRLDESCL0_5::AB_MODE;
    pub use super::CTRLDESCL0_5::BPP;
    pub use super::CTRLDESCL0_5::EN;
    pub use super::CTRLDESCL0_5::GLOBAL_ALPHA;
    pub use super::CTRLDESCL0_5::PD_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::PD_COLOR_MODE;
    pub use super::CTRLDESCL0_5::PD_FACTOR_MODE;
    pub use super::CTRLDESCL0_5::PD_GLOBAL_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::SAFETY_EN;
    pub use super::CTRLDESCL0_5::SHADOW_LOAD_EN;
    pub use super::CTRLDESCL0_5::YUV_FORMAT;
}

/// Control Descriptor Layer 6 Register
pub mod CTRLDESCL1_6 {
    pub use super::CTRLDESCL0_6::BCLR_B;
    pub use super::CTRLDESCL0_6::BCLR_G;
    pub use super::CTRLDESCL0_6::BCLR_R;
}

/// Color Space Conversion Coefficient Register 0
pub mod CSC1_COEF0 {
    pub use super::CSC0_COEF0::C0;
    pub use super::CSC0_COEF0::ENABLE;
    pub use super::CSC0_COEF0::UV_OFFSET;
    pub use super::CSC0_COEF0::YCBCR_MODE;
    pub use super::CSC0_COEF0::Y_OFFSET;
}

/// Color Space Conversion Coefficient Register 1
pub mod CSC1_COEF1 {
    pub use super::CSC0_COEF1::C1;
    pub use super::CSC0_COEF1::C4;
}

/// Color Space Conversion Coefficient Register 2
pub mod CSC1_COEF2 {
    pub use super::CSC0_COEF2::C2;
    pub use super::CSC0_COEF2::C3;
}

/// Control Descriptor Layer 1 Register
pub mod CTRLDESCL2_1 {
    pub use super::CTRLDESCL0_1::HEIGHT;
    pub use super::CTRLDESCL0_1::WIDTH;
}

/// Control Descriptor Layer 2 Register
pub mod CTRLDESCL2_2 {
    pub use super::CTRLDESCL0_2::POSX;
    pub use super::CTRLDESCL0_2::POSY;
}

/// Control Descriptor Layer 3 Register
pub mod CTRLDESCL2_3 {
    pub use super::CTRLDESCL0_3::PITCH;
}

/// Control Descriptor Layer 4 Register
pub mod CTRLDESCL2_4 {
    pub use super::CTRLDESCL0_4::ADDR;
}

/// Control Descriptor Layer 5 Register
pub mod CTRLDESCL2_5 {
    pub use super::CTRLDESCL0_5::AB_MODE;
    pub use super::CTRLDESCL0_5::BPP;
    pub use super::CTRLDESCL0_5::EN;
    pub use super::CTRLDESCL0_5::GLOBAL_ALPHA;
    pub use super::CTRLDESCL0_5::PD_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::PD_COLOR_MODE;
    pub use super::CTRLDESCL0_5::PD_FACTOR_MODE;
    pub use super::CTRLDESCL0_5::PD_GLOBAL_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::SAFETY_EN;
    pub use super::CTRLDESCL0_5::SHADOW_LOAD_EN;
    pub use super::CTRLDESCL0_5::YUV_FORMAT;
}

/// Control Descriptor Layer 6 Register
pub mod CTRLDESCL2_6 {
    pub use super::CTRLDESCL0_6::BCLR_B;
    pub use super::CTRLDESCL0_6::BCLR_G;
    pub use super::CTRLDESCL0_6::BCLR_R;
}

/// Control Descriptor Layer 1 Register
pub mod CTRLDESCL3_1 {
    pub use super::CTRLDESCL0_1::HEIGHT;
    pub use super::CTRLDESCL0_1::WIDTH;
}

/// Control Descriptor Layer 2 Register
pub mod CTRLDESCL3_2 {
    pub use super::CTRLDESCL0_2::POSX;
    pub use super::CTRLDESCL0_2::POSY;
}

/// Control Descriptor Layer 3 Register
pub mod CTRLDESCL3_3 {
    pub use super::CTRLDESCL0_3::PITCH;
}

/// Control Descriptor Layer 4 Register
pub mod CTRLDESCL3_4 {
    pub use super::CTRLDESCL0_4::ADDR;
}

/// Control Descriptor Layer 5 Register
pub mod CTRLDESCL3_5 {
    pub use super::CTRLDESCL0_5::AB_MODE;
    pub use super::CTRLDESCL0_5::BPP;
    pub use super::CTRLDESCL0_5::EN;
    pub use super::CTRLDESCL0_5::GLOBAL_ALPHA;
    pub use super::CTRLDESCL0_5::PD_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::PD_COLOR_MODE;
    pub use super::CTRLDESCL0_5::PD_FACTOR_MODE;
    pub use super::CTRLDESCL0_5::PD_GLOBAL_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::SAFETY_EN;
    pub use super::CTRLDESCL0_5::SHADOW_LOAD_EN;
    pub use super::CTRLDESCL0_5::YUV_FORMAT;
}

/// Control Descriptor Layer 6 Register
pub mod CTRLDESCL3_6 {
    pub use super::CTRLDESCL0_6::BCLR_B;
    pub use super::CTRLDESCL0_6::BCLR_G;
    pub use super::CTRLDESCL0_6::BCLR_R;
}

/// Control Descriptor Layer 1 Register
pub mod CTRLDESCL4_1 {
    pub use super::CTRLDESCL0_1::HEIGHT;
    pub use super::CTRLDESCL0_1::WIDTH;
}

/// Control Descriptor Layer 2 Register
pub mod CTRLDESCL4_2 {
    pub use super::CTRLDESCL0_2::POSX;
    pub use super::CTRLDESCL0_2::POSY;
}

/// Control Descriptor Layer 3 Register
pub mod CTRLDESCL4_3 {
    pub use super::CTRLDESCL0_3::PITCH;
}

/// Control Descriptor Layer 4 Register
pub mod CTRLDESCL4_4 {
    pub use super::CTRLDESCL0_4::ADDR;
}

/// Control Descriptor Layer 5 Register
pub mod CTRLDESCL4_5 {
    pub use super::CTRLDESCL0_5::AB_MODE;
    pub use super::CTRLDESCL0_5::BPP;
    pub use super::CTRLDESCL0_5::EN;
    pub use super::CTRLDESCL0_5::GLOBAL_ALPHA;
    pub use super::CTRLDESCL0_5::PD_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::PD_COLOR_MODE;
    pub use super::CTRLDESCL0_5::PD_FACTOR_MODE;
    pub use super::CTRLDESCL0_5::PD_GLOBAL_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::SAFETY_EN;
    pub use super::CTRLDESCL0_5::SHADOW_LOAD_EN;
    pub use super::CTRLDESCL0_5::YUV_FORMAT;
}

/// Control Descriptor Layer 6 Register
pub mod CTRLDESCL4_6 {
    pub use super::CTRLDESCL0_6::BCLR_B;
    pub use super::CTRLDESCL0_6::BCLR_G;
    pub use super::CTRLDESCL0_6::BCLR_R;
}

/// Control Descriptor Layer 1 Register
pub mod CTRLDESCL5_1 {
    pub use super::CTRLDESCL0_1::HEIGHT;
    pub use super::CTRLDESCL0_1::WIDTH;
}

/// Control Descriptor Layer 2 Register
pub mod CTRLDESCL5_2 {
    pub use super::CTRLDESCL0_2::POSX;
    pub use super::CTRLDESCL0_2::POSY;
}

/// Control Descriptor Layer 3 Register
pub mod CTRLDESCL5_3 {
    pub use super::CTRLDESCL0_3::PITCH;
}

/// Control Descriptor Layer 4 Register
pub mod CTRLDESCL5_4 {
    pub use super::CTRLDESCL0_4::ADDR;
}

/// Control Descriptor Layer 5 Register
pub mod CTRLDESCL5_5 {
    pub use super::CTRLDESCL0_5::AB_MODE;
    pub use super::CTRLDESCL0_5::BPP;
    pub use super::CTRLDESCL0_5::EN;
    pub use super::CTRLDESCL0_5::GLOBAL_ALPHA;
    pub use super::CTRLDESCL0_5::PD_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::PD_COLOR_MODE;
    pub use super::CTRLDESCL0_5::PD_FACTOR_MODE;
    pub use super::CTRLDESCL0_5::PD_GLOBAL_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::SAFETY_EN;
    pub use super::CTRLDESCL0_5::SHADOW_LOAD_EN;
    pub use super::CTRLDESCL0_5::YUV_FORMAT;
}

/// Control Descriptor Layer 6 Register
pub mod CTRLDESCL5_6 {
    pub use super::CTRLDESCL0_6::BCLR_B;
    pub use super::CTRLDESCL0_6::BCLR_G;
    pub use super::CTRLDESCL0_6::BCLR_R;
}

/// Control Descriptor Layer 1 Register
pub mod CTRLDESCL6_1 {
    pub use super::CTRLDESCL0_1::HEIGHT;
    pub use super::CTRLDESCL0_1::WIDTH;
}

/// Control Descriptor Layer 2 Register
pub mod CTRLDESCL6_2 {
    pub use super::CTRLDESCL0_2::POSX;
    pub use super::CTRLDESCL0_2::POSY;
}

/// Control Descriptor Layer 3 Register
pub mod CTRLDESCL6_3 {
    pub use super::CTRLDESCL0_3::PITCH;
}

/// Control Descriptor Layer 4 Register
pub mod CTRLDESCL6_4 {
    pub use super::CTRLDESCL0_4::ADDR;
}

/// Control Descriptor Layer 5 Register
pub mod CTRLDESCL6_5 {
    pub use super::CTRLDESCL0_5::AB_MODE;
    pub use super::CTRLDESCL0_5::BPP;
    pub use super::CTRLDESCL0_5::EN;
    pub use super::CTRLDESCL0_5::GLOBAL_ALPHA;
    pub use super::CTRLDESCL0_5::PD_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::PD_COLOR_MODE;
    pub use super::CTRLDESCL0_5::PD_FACTOR_MODE;
    pub use super::CTRLDESCL0_5::PD_GLOBAL_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::SAFETY_EN;
    pub use super::CTRLDESCL0_5::SHADOW_LOAD_EN;
    pub use super::CTRLDESCL0_5::YUV_FORMAT;
}

/// Control Descriptor Layer 6 Register
pub mod CTRLDESCL6_6 {
    pub use super::CTRLDESCL0_6::BCLR_B;
    pub use super::CTRLDESCL0_6::BCLR_G;
    pub use super::CTRLDESCL0_6::BCLR_R;
}

/// Control Descriptor Layer 1 Register
pub mod CTRLDESCL7_1 {
    pub use super::CTRLDESCL0_1::HEIGHT;
    pub use super::CTRLDESCL0_1::WIDTH;
}

/// Control Descriptor Layer 2 Register
pub mod CTRLDESCL7_2 {
    pub use super::CTRLDESCL0_2::POSX;
    pub use super::CTRLDESCL0_2::POSY;
}

/// Control Descriptor Layer 3 Register
pub mod CTRLDESCL7_3 {
    pub use super::CTRLDESCL0_3::PITCH;
}

/// Control Descriptor Layer 4 Register
pub mod CTRLDESCL7_4 {
    pub use super::CTRLDESCL0_4::ADDR;
}

/// Control Descriptor Layer 5 Register
pub mod CTRLDESCL7_5 {
    pub use super::CTRLDESCL0_5::AB_MODE;
    pub use super::CTRLDESCL0_5::BPP;
    pub use super::CTRLDESCL0_5::EN;
    pub use super::CTRLDESCL0_5::GLOBAL_ALPHA;
    pub use super::CTRLDESCL0_5::PD_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::PD_COLOR_MODE;
    pub use super::CTRLDESCL0_5::PD_FACTOR_MODE;
    pub use super::CTRLDESCL0_5::PD_GLOBAL_ALPHA_MODE;
    pub use super::CTRLDESCL0_5::SAFETY_EN;
    pub use super::CTRLDESCL0_5::SHADOW_LOAD_EN;
    pub use super::CTRLDESCL0_5::YUV_FORMAT;
}

/// Control Descriptor Layer 6 Register
pub mod CTRLDESCL7_6 {
    pub use super::CTRLDESCL0_6::BCLR_B;
    pub use super::CTRLDESCL0_6::BCLR_G;
    pub use super::CTRLDESCL0_6::BCLR_R;
}

/// LCDIFv2 CLUT load Register
pub mod CLUT_LOAD {

    /// CLUT Update Enable
    pub mod CLUT_UPDATE_EN {
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

    /// Selected CLUT Number
    pub mod SEL_CLUT_NUM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
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
    /// LCDIFv2 display control Register
    pub CTRL: RWRegister<u32>,

    /// LCDIFv2 display control Register
    pub CTRL_SET: RWRegister<u32>,

    /// LCDIFv2 display control Register
    pub CTRL_CLR: RWRegister<u32>,

    /// LCDIFv2 display control Register
    pub CTRL_TOG: RWRegister<u32>,

    /// Display Parameter Register
    pub DISP_PARA: RWRegister<u32>,

    /// Display Size Register
    pub DISP_SIZE: RWRegister<u32>,

    /// Horizontal Sync Parameter Register
    pub HSYN_PARA: RWRegister<u32>,

    /// Vertical Sync Parameter Register
    pub VSYN_PARA: RWRegister<u32>,

    /// Interrupt Status Register for domain 0
    pub INT_STATUS_D0: RWRegister<u32>,

    /// Interrupt Enable Register for domain 0
    pub INT_ENABLE_D0: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Interrupt Status Register for domain 1
    pub INT_STATUS_D1: RWRegister<u32>,

    /// Interrupt Enable Register for domain 1
    pub INT_ENABLE_D1: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// Parallel Data Interface Parameter Register
    pub PDI_PARA: RWRegister<u32>,

    _reserved3: [u32; 111],

    /// Control Descriptor Layer 1 Register
    pub CTRLDESCL0_1: RWRegister<u32>,

    /// Control Descriptor Layer 2 Register
    pub CTRLDESCL0_2: RWRegister<u32>,

    /// Control Descriptor Layer 3 Register
    pub CTRLDESCL0_3: RWRegister<u32>,

    /// Control Descriptor Layer 4 Register
    pub CTRLDESCL0_4: RWRegister<u32>,

    /// Control Descriptor Layer 5 Register
    pub CTRLDESCL0_5: RWRegister<u32>,

    /// Control Descriptor Layer 6 Register
    pub CTRLDESCL0_6: RWRegister<u32>,

    /// Color Space Conversion Coefficient Register 0
    pub CSC0_COEF0: RWRegister<u32>,

    /// Color Space Conversion Coefficient Register 1
    pub CSC0_COEF1: RWRegister<u32>,

    /// Color Space Conversion Coefficient Register 2
    pub CSC0_COEF2: RWRegister<u32>,

    _reserved4: [u32; 7],

    /// Control Descriptor Layer 1 Register
    pub CTRLDESCL1_1: RWRegister<u32>,

    /// Control Descriptor Layer 2 Register
    pub CTRLDESCL1_2: RWRegister<u32>,

    /// Control Descriptor Layer 3 Register
    pub CTRLDESCL1_3: RWRegister<u32>,

    /// Control Descriptor Layer 4 Register
    pub CTRLDESCL1_4: RWRegister<u32>,

    /// Control Descriptor Layer 5 Register
    pub CTRLDESCL1_5: RWRegister<u32>,

    /// Control Descriptor Layer 6 Register
    pub CTRLDESCL1_6: RWRegister<u32>,

    /// Color Space Conversion Coefficient Register 0
    pub CSC1_COEF0: RWRegister<u32>,

    /// Color Space Conversion Coefficient Register 1
    pub CSC1_COEF1: RWRegister<u32>,

    /// Color Space Conversion Coefficient Register 2
    pub CSC1_COEF2: RWRegister<u32>,

    _reserved5: [u32; 7],

    /// Control Descriptor Layer 1 Register
    pub CTRLDESCL2_1: RWRegister<u32>,

    /// Control Descriptor Layer 2 Register
    pub CTRLDESCL2_2: RWRegister<u32>,

    /// Control Descriptor Layer 3 Register
    pub CTRLDESCL2_3: RWRegister<u32>,

    /// Control Descriptor Layer 4 Register
    pub CTRLDESCL2_4: RWRegister<u32>,

    /// Control Descriptor Layer 5 Register
    pub CTRLDESCL2_5: RWRegister<u32>,

    /// Control Descriptor Layer 6 Register
    pub CTRLDESCL2_6: RWRegister<u32>,

    _reserved6: [u32; 10],

    /// Control Descriptor Layer 1 Register
    pub CTRLDESCL3_1: RWRegister<u32>,

    /// Control Descriptor Layer 2 Register
    pub CTRLDESCL3_2: RWRegister<u32>,

    /// Control Descriptor Layer 3 Register
    pub CTRLDESCL3_3: RWRegister<u32>,

    /// Control Descriptor Layer 4 Register
    pub CTRLDESCL3_4: RWRegister<u32>,

    /// Control Descriptor Layer 5 Register
    pub CTRLDESCL3_5: RWRegister<u32>,

    /// Control Descriptor Layer 6 Register
    pub CTRLDESCL3_6: RWRegister<u32>,

    _reserved7: [u32; 10],

    /// Control Descriptor Layer 1 Register
    pub CTRLDESCL4_1: RWRegister<u32>,

    /// Control Descriptor Layer 2 Register
    pub CTRLDESCL4_2: RWRegister<u32>,

    /// Control Descriptor Layer 3 Register
    pub CTRLDESCL4_3: RWRegister<u32>,

    /// Control Descriptor Layer 4 Register
    pub CTRLDESCL4_4: RWRegister<u32>,

    /// Control Descriptor Layer 5 Register
    pub CTRLDESCL4_5: RWRegister<u32>,

    /// Control Descriptor Layer 6 Register
    pub CTRLDESCL4_6: RWRegister<u32>,

    _reserved8: [u32; 10],

    /// Control Descriptor Layer 1 Register
    pub CTRLDESCL5_1: RWRegister<u32>,

    /// Control Descriptor Layer 2 Register
    pub CTRLDESCL5_2: RWRegister<u32>,

    /// Control Descriptor Layer 3 Register
    pub CTRLDESCL5_3: RWRegister<u32>,

    /// Control Descriptor Layer 4 Register
    pub CTRLDESCL5_4: RWRegister<u32>,

    /// Control Descriptor Layer 5 Register
    pub CTRLDESCL5_5: RWRegister<u32>,

    /// Control Descriptor Layer 6 Register
    pub CTRLDESCL5_6: RWRegister<u32>,

    _reserved9: [u32; 10],

    /// Control Descriptor Layer 1 Register
    pub CTRLDESCL6_1: RWRegister<u32>,

    /// Control Descriptor Layer 2 Register
    pub CTRLDESCL6_2: RWRegister<u32>,

    /// Control Descriptor Layer 3 Register
    pub CTRLDESCL6_3: RWRegister<u32>,

    /// Control Descriptor Layer 4 Register
    pub CTRLDESCL6_4: RWRegister<u32>,

    /// Control Descriptor Layer 5 Register
    pub CTRLDESCL6_5: RWRegister<u32>,

    /// Control Descriptor Layer 6 Register
    pub CTRLDESCL6_6: RWRegister<u32>,

    _reserved10: [u32; 10],

    /// Control Descriptor Layer 1 Register
    pub CTRLDESCL7_1: RWRegister<u32>,

    /// Control Descriptor Layer 2 Register
    pub CTRLDESCL7_2: RWRegister<u32>,

    /// Control Descriptor Layer 3 Register
    pub CTRLDESCL7_3: RWRegister<u32>,

    /// Control Descriptor Layer 4 Register
    pub CTRLDESCL7_4: RWRegister<u32>,

    /// Control Descriptor Layer 5 Register
    pub CTRLDESCL7_5: RWRegister<u32>,

    /// Control Descriptor Layer 6 Register
    pub CTRLDESCL7_6: RWRegister<u32>,

    _reserved11: [u32; 10],

    /// LCDIFv2 CLUT load Register
    pub CLUT_LOAD: RWRegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub CTRL_SET: u32,
    pub CTRL_CLR: u32,
    pub CTRL_TOG: u32,
    pub DISP_PARA: u32,
    pub DISP_SIZE: u32,
    pub HSYN_PARA: u32,
    pub VSYN_PARA: u32,
    pub INT_STATUS_D0: u32,
    pub INT_ENABLE_D0: u32,
    pub INT_STATUS_D1: u32,
    pub INT_ENABLE_D1: u32,
    pub PDI_PARA: u32,
    pub CTRLDESCL0_1: u32,
    pub CTRLDESCL0_2: u32,
    pub CTRLDESCL0_3: u32,
    pub CTRLDESCL0_4: u32,
    pub CTRLDESCL0_5: u32,
    pub CTRLDESCL0_6: u32,
    pub CSC0_COEF0: u32,
    pub CSC0_COEF1: u32,
    pub CSC0_COEF2: u32,
    pub CTRLDESCL1_1: u32,
    pub CTRLDESCL1_2: u32,
    pub CTRLDESCL1_3: u32,
    pub CTRLDESCL1_4: u32,
    pub CTRLDESCL1_5: u32,
    pub CTRLDESCL1_6: u32,
    pub CSC1_COEF0: u32,
    pub CSC1_COEF1: u32,
    pub CSC1_COEF2: u32,
    pub CTRLDESCL2_1: u32,
    pub CTRLDESCL2_2: u32,
    pub CTRLDESCL2_3: u32,
    pub CTRLDESCL2_4: u32,
    pub CTRLDESCL2_5: u32,
    pub CTRLDESCL2_6: u32,
    pub CTRLDESCL3_1: u32,
    pub CTRLDESCL3_2: u32,
    pub CTRLDESCL3_3: u32,
    pub CTRLDESCL3_4: u32,
    pub CTRLDESCL3_5: u32,
    pub CTRLDESCL3_6: u32,
    pub CTRLDESCL4_1: u32,
    pub CTRLDESCL4_2: u32,
    pub CTRLDESCL4_3: u32,
    pub CTRLDESCL4_4: u32,
    pub CTRLDESCL4_5: u32,
    pub CTRLDESCL4_6: u32,
    pub CTRLDESCL5_1: u32,
    pub CTRLDESCL5_2: u32,
    pub CTRLDESCL5_3: u32,
    pub CTRLDESCL5_4: u32,
    pub CTRLDESCL5_5: u32,
    pub CTRLDESCL5_6: u32,
    pub CTRLDESCL6_1: u32,
    pub CTRLDESCL6_2: u32,
    pub CTRLDESCL6_3: u32,
    pub CTRLDESCL6_4: u32,
    pub CTRLDESCL6_5: u32,
    pub CTRLDESCL6_6: u32,
    pub CTRLDESCL7_1: u32,
    pub CTRLDESCL7_2: u32,
    pub CTRLDESCL7_3: u32,
    pub CTRLDESCL7_4: u32,
    pub CTRLDESCL7_5: u32,
    pub CTRLDESCL7_6: u32,
    pub CLUT_LOAD: u32,
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
