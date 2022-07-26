#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CSI
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// CSI Control Register 1
pub mod CR1 {

    /// Pixel Bit
    pub mod PIXEL_BIT {
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

            /// 0b0: 8-bit data for each pixel
            pub const PIXEL_BIT_0: u32 = 0b0;

            /// 0b1: 10-bit data for each pixel
            pub const PIXEL_BIT_1: u32 = 0b1;
        }
    }

    /// Valid Pixel Clock Edge Select
    pub mod REDGE {
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

            /// 0b0: Pixel data is latched at the falling edge of CSI_PIXCLK
            pub const REDGE_0: u32 = 0b0;

            /// 0b1: Pixel data is latched at the rising edge of CSI_PIXCLK
            pub const REDGE_1: u32 = 0b1;
        }
    }

    /// Invert Pixel Clock Input
    pub mod INV_PCLK {
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

            /// 0b0: CSI_PIXCLK is directly applied to internal circuitry
            pub const INV_PCLK_0: u32 = 0b0;

            /// 0b1: CSI_PIXCLK is inverted before applied to internal circuitry
            pub const INV_PCLK_1: u32 = 0b1;
        }
    }

    /// Invert Data Input. This bit enables or disables internal inverters on the data lines.
    pub mod INV_DATA {
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

            /// 0b0: CSI_D\[7:0\] data lines are directly applied to internal circuitry
            pub const INV_DATA_0: u32 = 0b0;

            /// 0b1: CSI_D\[7:0\] data lines are inverted before applied to internal circuitry
            pub const INV_DATA_1: u32 = 0b1;
        }
    }

    /// Gated Clock Mode Enable
    pub mod GCLK_MODE {
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

            /// 0b0: Non-gated clock mode. All incoming pixel clocks are valid. HSYNC is ignored.
            pub const GCLK_MODE_0: u32 = 0b0;

            /// 0b1: Gated clock mode. Pixel clock signal is valid only when HSYNC is active.
            pub const GCLK_MODE_1: u32 = 0b1;
        }
    }

    /// Asynchronous RXFIFO Clear
    pub mod CLR_RXFIFO {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Asynchronous STATFIFO Clear
    pub mod CLR_STATFIFO {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Packing Direction
    pub mod PACK_DIR {
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

            /// 0b0: Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xBBBBAAAA in STAT FIFO.
            pub const PACK_DIR_0: u32 = 0b0;

            /// 0b1: Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO.
            pub const PACK_DIR_1: u32 = 0b1;
        }
    }

    /// FIFO Clear Control
    pub mod FCC {
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

            /// 0b0: Asynchronous FIFO clear is selected.
            pub const FCC_0: u32 = 0b0;

            /// 0b1: Synchronous FIFO clear is selected.
            pub const FCC_1: u32 = 0b1;
        }
    }

    /// BT.656 Interface Enable. This bit selects the type of interface used.
    pub mod CCIR_EN {
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

            /// 0b0: Traditional interface is selected.
            pub const CCIR_EN_0: u32 = 0b0;

            /// 0b1: BT.656 interface is selected.
            pub const CCIR_EN_1: u32 = 0b1;
        }
    }

    /// HSYNC Polarity Select
    pub mod HSYNC_POL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: HSYNC is active low
            pub const HSYNC_POL_0: u32 = 0b0;

            /// 0b1: HSYNC is active high
            pub const HSYNC_POL_1: u32 = 0b1;
        }
    }

    /// Histogram Interrupt Enable
    pub mod HISTOGRAM_CALC_DONE_IE {
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

            /// 0b0: Histogram done interrupt disable
            pub const HISTOGRAM_CALC_DONE_IE_0: u32 = 0b0;

            /// 0b1: Histogram done interrupt enable
            pub const HISTOGRAM_CALC_DONE_IE_1: u32 = 0b1;
        }
    }

    /// Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt.
    pub mod SOF_INTEN {
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

            /// 0b0: SOF interrupt disable
            pub const SOF_INTEN_0: u32 = 0b0;

            /// 0b1: SOF interrupt enable
            pub const SOF_INTEN_1: u32 = 0b1;
        }
    }

    /// SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt.
    pub mod SOF_POL {
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

            /// 0b0: SOF interrupt is generated on SOF falling edge
            pub const SOF_POL_0: u32 = 0b0;

            /// 0b1: SOF interrupt is generated on SOF rising edge
            pub const SOF_POL_1: u32 = 0b1;
        }
    }

    /// RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt.
    pub mod RXFF_INTEN {
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

            /// 0b0: RxFIFO full interrupt disable
            pub const RXFF_INTEN_0: u32 = 0b0;

            /// 0b1: RxFIFO full interrupt enable
            pub const RXFF_INTEN_1: u32 = 0b1;
        }
    }

    /// Frame Buffer1 DMA Transfer Done Interrupt Enable
    pub mod FB1_DMA_DONE_INTEN {
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

            /// 0b0: Frame Buffer1 DMA Transfer Done interrupt disable
            pub const FB1_DMA_DONE_INTEN_0: u32 = 0b0;

            /// 0b1: Frame Buffer1 DMA Transfer Done interrupt enable
            pub const FB1_DMA_DONE_INTEN_1: u32 = 0b1;
        }
    }

    /// Frame Buffer2 DMA Transfer Done Interrupt Enable
    pub mod FB2_DMA_DONE_INTEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Frame Buffer2 DMA Transfer Done interrupt disable
            pub const FB2_DMA_DONE_INTEN_0: u32 = 0b0;

            /// 0b1: Frame Buffer2 DMA Transfer Done interrupt enable
            pub const FB2_DMA_DONE_INTEN_1: u32 = 0b1;
        }
    }

    /// STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt.
    pub mod STATFF_INTEN {
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

            /// 0b0: STATFIFO full interrupt disable
            pub const STATFF_INTEN_0: u32 = 0b0;

            /// 0b1: STATFIFO full interrupt enable
            pub const STATFF_INTEN_1: u32 = 0b1;
        }
    }

    /// STATFIFO DMA Transfer Done Interrupt Enable
    pub mod SFF_DMA_DONE_INTEN {
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

            /// 0b0: STATFIFO DMA Transfer Done interrupt disable
            pub const SFF_DMA_DONE_INTEN_0: u32 = 0b0;

            /// 0b1: STATFIFO DMA Transfer Done interrupt enable
            pub const SFF_DMA_DONE_INTEN_1: u32 = 0b1;
        }
    }

    /// RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt.
    pub mod RF_OR_INTEN {
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

            /// 0b0: RxFIFO overrun interrupt is disabled
            pub const RF_OR_INTEN_0: u32 = 0b0;

            /// 0b1: RxFIFO overrun interrupt is enabled
            pub const RF_OR_INTEN_1: u32 = 0b1;
        }
    }

    /// STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt.
    pub mod SF_OR_INTEN {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: STATFIFO overrun interrupt is disabled
            pub const SF_OR_INTEN_0: u32 = 0b0;

            /// 0b1: STATFIFO overrun interrupt is enabled
            pub const SF_OR_INTEN_1: u32 = 0b1;
        }
    }

    /// Change Of Image Field (COF) Interrupt Enable
    pub mod COF_INT_EN {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: COF interrupt is disabled
            pub const COF_INT_EN_0: u32 = 0b0;

            /// 0b1: COF interrupt is enabled
            pub const COF_INT_EN_1: u32 = 0b1;
        }
    }

    /// Video mode select. This bit controls the video mode in BT.656 mode and TV decoder input.
    pub mod VIDEO_MODE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Progressive mode is selected
            pub const VIDEO_MODE_0: u32 = 0b0;

            /// 0b1: Interlace mode is selected
            pub const VIDEO_MODE_1: u32 = 0b1;
        }
    }

    /// End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt.
    pub mod EOF_INT_EN {
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

            /// 0b0: EOF interrupt is disabled.
            pub const EOF_INT_EN_0: u32 = 0b0;

            /// 0b1: EOF interrupt is generated when RX count value is reached.
            pub const EOF_INT_EN_1: u32 = 0b1;
        }
    }

    /// External VSYNC Enable
    pub mod EXT_VSYNC {
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

            /// 0b0: Internal VSYNC mode
            pub const EXT_VSYNC_0: u32 = 0b0;

            /// 0b1: External VSYNC mode
            pub const EXT_VSYNC_1: u32 = 0b1;
        }
    }

    /// SWAP 16-Bit Enable
    pub mod SWAP16_EN {
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

            /// 0b0: Disable swapping
            pub const SWAP16_EN_0: u32 = 0b0;

            /// 0b1: Enable swapping
            pub const SWAP16_EN_1: u32 = 0b1;
        }
    }
}

/// CSI Control Register 2
pub mod CR2 {

    /// Horizontal Skip Count
    pub mod HSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Number of pixels to skip minus 1
            pub const HSC_0: u32 = 0b00000000;

            /// 0b00000001: Number of pixels to skip minus 1
            pub const HSC_1: u32 = 0b00000001;

            /// 0b00000010: Number of pixels to skip minus 1
            pub const HSC_2: u32 = 0b00000010;

            /// 0b00000011: Number of pixels to skip minus 1
            pub const HSC_3: u32 = 0b00000011;

            /// 0b00000100: Number of pixels to skip minus 1
            pub const HSC_4: u32 = 0b00000100;

            /// 0b00000101: Number of pixels to skip minus 1
            pub const HSC_5: u32 = 0b00000101;

            /// 0b00000110: Number of pixels to skip minus 1
            pub const HSC_6: u32 = 0b00000110;

            /// 0b00000111: Number of pixels to skip minus 1
            pub const HSC_7: u32 = 0b00000111;

            /// 0b00001000: Number of pixels to skip minus 1
            pub const HSC_8: u32 = 0b00001000;

            /// 0b00001001: Number of pixels to skip minus 1
            pub const HSC_9: u32 = 0b00001001;
        }
    }

    /// Vertical Skip Count. Contains the number of rows to skip. SCE must be 1, otherwise VSC is ignored.
    pub mod VSC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Number of rows to skip minus 1
            pub const VSC_0: u32 = 0b00000000;

            /// 0b00000001: Number of rows to skip minus 1
            pub const VSC_1: u32 = 0b00000001;

            /// 0b00000010: Number of rows to skip minus 1
            pub const VSC_2: u32 = 0b00000010;

            /// 0b00000011: Number of rows to skip minus 1
            pub const VSC_3: u32 = 0b00000011;

            /// 0b00000100: Number of rows to skip minus 1
            pub const VSC_4: u32 = 0b00000100;

            /// 0b00000101: Number of rows to skip minus 1
            pub const VSC_5: u32 = 0b00000101;

            /// 0b00000110: Number of rows to skip minus 1
            pub const VSC_6: u32 = 0b00000110;

            /// 0b00000111: Number of rows to skip minus 1
            pub const VSC_7: u32 = 0b00000111;

            /// 0b00001000: Number of rows to skip minus 1
            pub const VSC_8: u32 = 0b00001000;

            /// 0b00001001: Number of rows to skip minus 1
            pub const VSC_9: u32 = 0b00001001;
        }
    }

    /// Live View Resolution Mode. Selects the grid size used for live view resolution.
    pub mod LVRM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 512 x 384
            pub const LVRM_0: u32 = 0b000;

            /// 0b001: 448 x 336
            pub const LVRM_1: u32 = 0b001;

            /// 0b010: 384 x 288
            pub const LVRM_2: u32 = 0b010;

            /// 0b011: 384 x 256
            pub const LVRM_3: u32 = 0b011;

            /// 0b100: 320 x 240
            pub const LVRM_4: u32 = 0b100;

            /// 0b101: 288 x 216
            pub const LVRM_5: u32 = 0b101;

            /// 0b110: 400 x 300
            pub const LVRM_6: u32 = 0b110;
        }
    }

    /// Bayer Tile Start. Controls the Bayer pattern starting point.
    pub mod BTS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: GR
            pub const BTS_0: u32 = 0b00;

            /// 0b01: RG
            pub const BTS_1: u32 = 0b01;

            /// 0b10: BG
            pub const BTS_2: u32 = 0b10;

            /// 0b11: GB
            pub const BTS_3: u32 = 0b11;
        }
    }

    /// Skip Count Enable
    pub mod SCE {
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

            /// 0b0: Skip count disable
            pub const SCE_0: u32 = 0b0;

            /// 0b1: Skip count enable
            pub const SCE_1: u32 = 0b1;
        }
    }

    /// Auto Focus Spread. Selects which green pixels are used for auto-focus.
    pub mod AFS {
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

            /// 0b00: Abs Diff on consecutive green pixels
            pub const AFS_0: u32 = 0b00;

            /// 0b01: Abs Diff on every third green pixels
            pub const AFS_1: u32 = 0b01;

            /// 0b00: Abs Diff on every four green pixels
            pub const AFS_2: u32 = 0b00;
        }
    }

    /// Double Resolution Mode. Controls size of statistics grid.
    pub mod DRM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Stats grid of 8 x 6
            pub const DRM_0: u32 = 0b0;

            /// 0b1: Stats grid of 8 x 12
            pub const DRM_1: u32 = 0b1;
        }
    }

    /// Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO.
    pub mod DMA_BURST_TYPE_SFF {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: INCR8
            pub const DMA_BURST_TYPE_SFF_0: u32 = 0b00;

            /// 0b01: INCR4
            pub const DMA_BURST_TYPE_SFF_1: u32 = 0b01;

            /// 0b11: INCR16
            pub const DMA_BURST_TYPE_SFF_3: u32 = 0b11;
        }
    }

    /// Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO.
    pub mod DMA_BURST_TYPE_RFF {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: INCR8
            pub const DMA_BURST_TYPE_RFF_0: u32 = 0b00;

            /// 0b01: INCR4
            pub const DMA_BURST_TYPE_RFF_1: u32 = 0b01;

            /// 0b11: INCR16
            pub const DMA_BURST_TYPE_RFF_3: u32 = 0b11;
        }
    }
}

/// CSI Control Register 3
pub mod CR3 {

    /// Automatic Error Correction Enable
    pub mod ECC_AUTO_EN {
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

            /// 0b0: Auto Error correction is disabled.
            pub const ECC_AUTO_EN_0: u32 = 0b0;

            /// 0b1: Auto Error correction is enabled.
            pub const ECC_AUTO_EN_1: u32 = 0b1;
        }
    }

    /// Error Detection Interrupt Enable
    pub mod ECC_INT_EN {
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

            /// 0b0: No interrupt is generated when error is detected. Only the status bit ECC_INT is set.
            pub const ECC_INT_EN_0: u32 = 0b0;

            /// 0b1: Interrupt is generated when error is detected.
            pub const ECC_INT_EN_1: u32 = 0b1;
        }
    }

    /// Dummy Zero Packing Enable
    pub mod ZERO_PACK_EN {
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

            /// 0b0: Zero packing disabled
            pub const ZERO_PACK_EN_0: u32 = 0b0;

            /// 0b1: Zero packing enabled
            pub const ZERO_PACK_EN_1: u32 = 0b1;
        }
    }

    /// 16-bit Sensor Mode
    pub mod SENSOR_16BITS {
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

            /// 0b0: Only one 8-bit sensor is connected.
            pub const SENSOR_16BITS_0: u32 = 0b0;

            /// 0b1: One 16-bit sensor is connected.
            pub const SENSOR_16BITS_1: u32 = 0b1;
        }
    }

    /// RxFIFO Full Level
    pub mod RxFF_LEVEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 4 Double words
            pub const RxFF_LEVEL_0: u32 = 0b000;

            /// 0b001: 8 Double words
            pub const RxFF_LEVEL_1: u32 = 0b001;

            /// 0b010: 16 Double words
            pub const RxFF_LEVEL_2: u32 = 0b010;

            /// 0b011: 24 Double words
            pub const RxFF_LEVEL_3: u32 = 0b011;

            /// 0b100: 32 Double words
            pub const RxFF_LEVEL_4: u32 = 0b100;

            /// 0b101: 48 Double words
            pub const RxFF_LEVEL_5: u32 = 0b101;

            /// 0b110: 64 Double words
            pub const RxFF_LEVEL_6: u32 = 0b110;

            /// 0b111: 96 Double words
            pub const RxFF_LEVEL_7: u32 = 0b111;
        }
    }

    /// Hresponse Error Enable. This bit enables the hresponse (AHB protocol standard) error interrupt.
    pub mod HRESP_ERR_EN {
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

            /// 0b0: Disable hresponse error interrupt
            pub const HRESP_ERR_EN_0: u32 = 0b0;

            /// 0b1: Enable hresponse error interrupt
            pub const HRESP_ERR_EN_1: u32 = 0b1;
        }
    }

    /// STATFIFO Full Level
    pub mod STATFF_LEVEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 4 Double words
            pub const STATFF_LEVEL_0: u32 = 0b000;

            /// 0b001: 8 Double words
            pub const STATFF_LEVEL_1: u32 = 0b001;

            /// 0b010: 12 Double words
            pub const STATFF_LEVEL_2: u32 = 0b010;

            /// 0b011: 16 Double words
            pub const STATFF_LEVEL_3: u32 = 0b011;

            /// 0b100: 24 Double words
            pub const STATFF_LEVEL_4: u32 = 0b100;

            /// 0b101: 32 Double words
            pub const STATFF_LEVEL_5: u32 = 0b101;

            /// 0b110: 48 Double words
            pub const STATFF_LEVEL_6: u32 = 0b110;

            /// 0b111: 64 Double words
            pub const STATFF_LEVEL_7: u32 = 0b111;
        }
    }

    /// DMA Request Enable for STATFIFO
    pub mod DMA_REQ_EN_SFF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable the dma request
            pub const DMA_REQ_EN_SFF_0: u32 = 0b0;

            /// 0b1: Enable the dma request
            pub const DMA_REQ_EN_SFF_1: u32 = 0b1;
        }
    }

    /// DMA Request Enable for RxFIFO
    pub mod DMA_REQ_EN_RFF {
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

            /// 0b0: Disable the dma request
            pub const DMA_REQ_EN_RFF_0: u32 = 0b0;

            /// 0b1: Enable the dma request
            pub const DMA_REQ_EN_RFF_1: u32 = 0b1;
        }
    }

    /// Reflash DMA Controller for STATFIFO
    pub mod DMA_REFLASH_SFF {
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

            /// 0b0: No reflashing
            pub const DMA_REFLASH_SFF_0: u32 = 0b0;

            /// 0b1: Reflash the embedded DMA controller
            pub const DMA_REFLASH_SFF_1: u32 = 0b1;
        }
    }

    /// Reflash DMA Controller for RxFIFO
    pub mod DMA_REFLASH_RFF {
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

            /// 0b0: No reflashing
            pub const DMA_REFLASH_RFF_0: u32 = 0b0;

            /// 0b1: Reflash the embedded DMA controller
            pub const DMA_REFLASH_RFF_1: u32 = 0b1;
        }
    }

    /// Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)
    pub mod FRMCNT_RST {
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

            /// 0b0: Do not reset
            pub const FRMCNT_RST_0: u32 = 0b0;

            /// 0b1: Reset frame counter immediately
            pub const FRMCNT_RST_1: u32 = 0b1;
        }
    }

    /// Frame Counter
    pub mod FRMCNT {
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

/// CSI Statistic FIFO Register
pub mod STATFIFO {

    /// Static data from sensor
    pub mod STAT {
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

/// CSI RX FIFO Register
pub mod RFIFO {

    /// Received image data
    pub mod IMAGE {
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

/// CSI RX Count Register
pub mod RXCNT {

    /// RxFIFO Count
    pub mod RXCNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CSI Status Register
pub mod SR {

    /// RXFIFO Data Ready
    pub mod DRDY {
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

            /// 0b0: No data (word) is ready
            pub const DRDY_0: u32 = 0b0;

            /// 0b1: At least 1 datum (word) is ready in RXFIFO.
            pub const DRDY_1: u32 = 0b1;
        }
    }

    /// BT
    pub mod ECC_INT {
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

            /// 0b0: No error detected
            pub const ECC_INT_0: u32 = 0b0;

            /// 0b1: Error is detected in BT.656 coding
            pub const ECC_INT_1: u32 = 0b1;
        }
    }

    /// no description available
    pub mod HISTOGRAM_CALC_DONE_INT {
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

            /// 0b0: Histogram calculation is not finished
            pub const HISTOGRAM_CALC_DONE_INT_0: u32 = 0b0;

            /// 0b1: Histogram calculation is done and driver can access the PIXEL_COUNTERS(CSI_CSICR21~CSI_CSICR276) to get the gray level
            pub const HISTOGRAM_CALC_DONE_INT_1: u32 = 0b1;
        }
    }

    /// Hresponse Error Interrupt Status
    pub mod HRESP_ERR_INT {
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

            /// 0b0: No hresponse error.
            pub const HRESP_ERR_INT_0: u32 = 0b0;

            /// 0b1: Hresponse error is detected.
            pub const HRESP_ERR_INT_1: u32 = 0b1;
        }
    }

    /// Change Of Field Interrupt Status
    pub mod COF_INT {
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

            /// 0b0: Video field has no change.
            pub const COF_INT_0: u32 = 0b0;

            /// 0b1: Change of video field is detected.
            pub const COF_INT_1: u32 = 0b1;
        }
    }

    /// BT
    pub mod F1_INT {
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

            /// 0b0: Field 1 of video is not detected.
            pub const F1_INT_0: u32 = 0b0;

            /// 0b1: Field 1 of video is about to start.
            pub const F1_INT_1: u32 = 0b1;
        }
    }

    /// BT
    pub mod F2_INT {
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

            /// 0b0: Field 2 of video is not detected
            pub const F2_INT_0: u32 = 0b0;

            /// 0b1: Field 2 of video is about to start
            pub const F2_INT_1: u32 = 0b1;
        }
    }

    /// Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)
    pub mod SOF_INT {
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

            /// 0b0: SOF is not detected.
            pub const SOF_INT_0: u32 = 0b0;

            /// 0b1: SOF is detected.
            pub const SOF_INT_1: u32 = 0b1;
        }
    }

    /// End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)
    pub mod EOF_INT {
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

            /// 0b0: EOF is not detected.
            pub const EOF_INT_0: u32 = 0b0;

            /// 0b1: EOF is detected.
            pub const EOF_INT_1: u32 = 0b1;
        }
    }

    /// RXFIFO Full Interrupt Status
    pub mod RxFF_INT {
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

            /// 0b0: RxFIFO is not full.
            pub const RxFF_INT_0: u32 = 0b0;

            /// 0b1: RxFIFO is full.
            pub const RxFF_INT_1: u32 = 0b1;
        }
    }

    /// DMA Transfer Done in Frame Buffer1
    pub mod DMA_TSF_DONE_FB1 {
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

            /// 0b0: DMA transfer is not completed.
            pub const DMA_TSF_DONE_FB1_0: u32 = 0b0;

            /// 0b1: DMA transfer is completed.
            pub const DMA_TSF_DONE_FB1_1: u32 = 0b1;
        }
    }

    /// DMA Transfer Done in Frame Buffer2
    pub mod DMA_TSF_DONE_FB2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA transfer is not completed.
            pub const DMA_TSF_DONE_FB2_0: u32 = 0b0;

            /// 0b1: DMA transfer is completed.
            pub const DMA_TSF_DONE_FB2_1: u32 = 0b1;
        }
    }

    /// STATFIFO Full Interrupt Status
    pub mod STATFF_INT {
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

            /// 0b0: STATFIFO is not full.
            pub const STATFF_INT_0: u32 = 0b0;

            /// 0b1: STATFIFO is full.
            pub const STATFF_INT_1: u32 = 0b1;
        }
    }

    /// DMA Transfer Done from StatFIFO
    pub mod DMA_TSF_DONE_SFF {
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

            /// 0b0: DMA transfer is not completed.
            pub const DMA_TSF_DONE_SFF_0: u32 = 0b0;

            /// 0b1: DMA transfer is completed.
            pub const DMA_TSF_DONE_SFF_1: u32 = 0b1;
        }
    }

    /// RxFIFO Overrun Interrupt Status
    pub mod RF_OR_INT {
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

            /// 0b0: RXFIFO has not overflowed.
            pub const RF_OR_INT_0: u32 = 0b0;

            /// 0b1: RXFIFO has overflowed.
            pub const RF_OR_INT_1: u32 = 0b1;
        }
    }

    /// STATFIFO Overrun Interrupt Status
    pub mod SF_OR_INT {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: STATFIFO has not overflowed.
            pub const SF_OR_INT_0: u32 = 0b0;

            /// 0b1: STATFIFO has overflowed.
            pub const SF_OR_INT_1: u32 = 0b1;
        }
    }

    /// When DMA field 1 is complete, this bit will be set to 1(clear by writing 1).
    pub mod DMA_FIELD1_DONE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When DMA field 0 is complete, this bit will be set to 1(clear by writing 1).
    pub mod DMA_FIELD0_DONE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When using base address switching enable, this bit will be 1 when switching occur before DMA complete
    pub mod BASEADDR_CHHANGE_ERROR {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CSI DMA Start Address Register - for STATFIFO
pub mod DMASA_STATFIFO {

    /// DMA Start Address for STATFIFO
    pub mod DMA_START_ADDR_SFF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (30 bits: 0x3fffffff << 2)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CSI DMA Transfer Size Register - for STATFIFO
pub mod DMATS_STATFIFO {

    /// DMA Transfer Size for STATFIFO
    pub mod DMA_TSF_SIZE_SFF {
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

/// CSI DMA Start Address Register - for Frame Buffer1
pub mod DMASA_FB1 {

    /// DMA Start Address in Frame Buffer1
    pub mod DMA_START_ADDR_FB1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (30 bits: 0x3fffffff << 2)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CSI DMA Transfer Size Register - for Frame Buffer2
pub mod DMASA_FB2 {

    /// DMA Start Address in Frame Buffer2
    pub mod DMA_START_ADDR_FB2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (30 bits: 0x3fffffff << 2)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CSI Frame Buffer Parameter Register
pub mod FBUF_PARA {

    /// Frame Buffer Parameter
    pub mod FBUF_STRIDE {
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

    /// DEINTERLACE_STRIDE is only used in the deinterlace mode
    pub mod DEINTERLACE_STRIDE {
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

/// CSI Image Parameter Register
pub mod IMAG_PARA {

    /// Image Height. Indicates how many pixels in a column of the image from the sensor.
    pub mod IMAGE_HEIGHT {
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

    /// This field indicates the number of active pixel cycles per line
    pub mod IMAGE_WIDTH {
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

/// CSI Control Register 18
pub mod CR18 {

    /// This bit is used to select NTSC/PAL mode When input is TVDECODER or standard BT.656 video.
    pub mod NTSC_EN {
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

            /// 0b0: PAL
            pub const NTSC_EN_0: u32 = 0b0;

            /// 0b1: NTSC
            pub const NTSC_EN_1: u32 = 0b1;
        }
    }

    /// When input is from TV decoder, this bit is enabled.
    pub mod TVDECODER_IN_EN {
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

    /// This bit is used to select the output method When input is TVDECODER or standard BT.656 video.
    pub mod DEINTERLACE_EN {
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

            /// 0b0: Deinterlace disabled
            pub const DEINTERLACE_EN_0: u32 = 0b0;

            /// 0b1: Deinterlace enabled
            pub const DEINTERLACE_EN_1: u32 = 0b1;
        }
    }

    /// Enable bit for Parallel RGB888/YUV444 24bit input
    pub mod PARALLEL24_EN {
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

            /// 0b0: Input is disabled
            pub const PARALLEL24_EN_0: u32 = 0b0;

            /// 0b1: Input is enabled
            pub const PARALLEL24_EN_1: u32 = 0b1;
        }
    }

    /// When this bit is enabled, CSI DMA will switch the base address according to BASEADDR_SWITCH_SEL rather than automatically by DMA completed
    pub mod BASEADDR_SWITCH_EN {
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

    /// CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1.
    pub mod BASEADDR_SWITCH_SEL {
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

            /// 0b0: Switching base address at the edge of the vsync
            pub const BASEADDR_SWITCH_SEL_0: u32 = 0b0;

            /// 0b1: Switching base address at the edge of the first data of each frame
            pub const BASEADDR_SWITCH_SEL_1: u32 = 0b1;
        }
    }

    /// In interlace mode, field 0 means interrupt enabled.
    pub mod FIELD0_DONE_IE {
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

            /// 0b0: Interrupt disabled
            pub const FIELD0_DONE_IE_0: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const FIELD0_DONE_IE_1: u32 = 0b1;
        }
    }

    /// When in interlace mode, field 1 done interrupt enable.
    pub mod DMA_FIELD1_DONE_IE {
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

            /// 0b0: Interrupt disabled
            pub const DMA_FIELD1_DONE_IE_0: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const DMA_FIELD1_DONE_IE_1: u32 = 0b1;
        }
    }

    /// Choosing the last DMA request condition
    pub mod LAST_DMA_REQ_SEL {
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

            /// 0b0: fifo_full_level
            pub const LAST_DMA_REQ_SEL_0: u32 = 0b0;

            /// 0b1: hburst_length
            pub const LAST_DMA_REQ_SEL_1: u32 = 0b1;
        }
    }

    /// Base address change error interrupt enable signal.
    pub mod BASEADDR_CHANGE_ERROR_IE {
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

            /// 0b0: Interrupt disabled
            pub const BASEADDR_CHANGE_ERROR_IE_0: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const BASEADDR_CHANGE_ERROR_IE_1: u32 = 0b1;
        }
    }

    /// Output is 32-bit format.
    pub mod RGB888A_FORMAT_SEL {
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

            /// 0b0: {8'h0, data\[23:0\]}
            pub const RGB888A_FORMAT_SEL_0: u32 = 0b0;

            /// 0b1: {data\[23:0\], 8'h0}
            pub const RGB888A_FORMAT_SEL_1: u32 = 0b1;
        }
    }

    /// Hprot value in AHB bus protocol.
    pub mod AHB_HPROT {
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

    /// These bits used to choose the method to mask the CSI input.
    pub mod MASK_OPTION {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Writing to memory (OCRAM or external DDR) from first completely frame, when using this option, the CSI_ENABLE should be 1.
            pub const MASK_OPTION_0: u32 = 0b00;

            /// 0b01: Writing to memory when CSI_ENABLE is 1.
            pub const MASK_OPTION_1: u32 = 0b01;

            /// 0b10: Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1.
            pub const MASK_OPTION_2: u32 = 0b10;

            /// 0b11: Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0.
            pub const MASK_OPTION_3: u32 = 0b11;
        }
    }

    /// Double component per clock cycle in YUV422 formats.
    pub mod MIPI_DOUBLE_CMPNT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Single component per clock cycle (half pixel per clock cycle)
            pub const MIPI_DOUBLE_CMPNT_0: u32 = 0b0;

            /// 0b1: Double component per clock cycle (a pixel per clock cycle)
            pub const MIPI_DOUBLE_CMPNT_1: u32 = 0b1;
        }
    }

    /// It only works in MIPI CSI YUV422 double component mode.
    pub mod MIPI_YU_SWAP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// no description available
    pub mod DATA_FROM_MIPI {
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

            /// 0b0: Data from parallel sensor
            pub const DATA_FROM_MIPI_0: u32 = 0b0;

            /// 0b1: Data from MIPI
            pub const DATA_FROM_MIPI_1: u32 = 0b1;
        }
    }

    /// When the line width are not the multiple of the burst length, assert this bit.
    pub mod LINE_STRIDE_EN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Image Data Format
    pub mod MIPI_DATA_FORMAT {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (6 bits: 0x3f << 25)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSI global enable signal
    pub mod CSI_ENABLE {
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

/// CSI Control Register 19
pub mod CR19 {

    /// This byte stores the highest FIFO level achieved by CSI FIFO timely and will be clear by writing 8'ff to it
    pub mod DMA_RFIFO_HIGHEST_FIFO_LEVEL {
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

/// CSI Control Register 20
pub mod CR20 {

    /// THRESHOLD used for binary function. When data value > THRESHOLD, output will be 1 Else will be 0.
    pub mod THRESHOLD {
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

    /// no description available
    pub mod BINARY_EN {
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

            /// 0b0: Output is Y8 format(8 bits each pixel)
            pub const BINARY_EN_0: u32 = 0b0;

            /// 0b1: Output is Y1 format(1 bit each pixel)
            pub const BINARY_EN_1: u32 = 0b1;
        }
    }

    /// no description available
    pub mod QR_DATA_FORMAT {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: YU YV one cycle per 1 pixel input
            pub const QR_DATA_FORMAT_0: u32 = 0b000;

            /// 0b001: UY VY one cycle per1 pixel input
            pub const QR_DATA_FORMAT_1: u32 = 0b001;

            /// 0b010: Y U Y V two cycles per 1 pixel input
            pub const QR_DATA_FORMAT_2: u32 = 0b010;

            /// 0b011: U Y V Y two cycles per 1 pixel input
            pub const QR_DATA_FORMAT_3: u32 = 0b011;

            /// 0b100: YUV one cycle per 1 pixel input
            pub const QR_DATA_FORMAT_4: u32 = 0b100;

            /// 0b101: Y U V three cycles per 1 pixel input
            pub const QR_DATA_FORMAT_5: u32 = 0b101;
        }
    }

    /// no description available
    pub mod BIG_END {
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

            /// 0b0: The newest (most recent) data will be assigned the lowest position when store to memory.
            pub const BIG_END_0: u32 = 0b0;

            /// 0b1: The newest (most recent) data will be assigned the highest position when store to memory.
            pub const BIG_END_1: u32 = 0b1;
        }
    }

    /// no description available
    pub mod _10BIT_NEW_EN {
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

            /// 0b0: When input 8bits data, it will use the data\[9:2\]
            pub const _10BIT_NEW_EN_0: u32 = 0b0;

            /// 0b1: If input is 10bits data, it will use the data\[7:0\] (optional)
            pub const _10BIT_NEW_EN_1: u32 = 0b1;
        }
    }

    /// Histogram enable
    pub mod HISTOGRAM_EN {
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

            /// 0b0: Histogram disable
            pub const HISTOGRAM_EN_0: u32 = 0b0;

            /// 0b1: Histogram enable
            pub const HISTOGRAM_EN_1: u32 = 0b1;
        }
    }

    /// Gray scale mode enable
    pub mod QRCODE_EN {
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

            /// 0b0: Normal mode
            pub const QRCODE_EN_0: u32 = 0b0;

            /// 0b1: Gray scale mode
            pub const QRCODE_EN_1: u32 = 0b1;
        }
    }
}

/// CSI Control Register
pub mod CR21 {

    /// Number of pixels (Y component of the input pixel) equals: 0 (CSICR21) 1 (CSICR22)
    pub mod PIXEL_COUNTERS {
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

/// CSI Control Register
pub mod CR22 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR23 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR24 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR25 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR26 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR27 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR28 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR29 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR30 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR31 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR32 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR33 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR34 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR35 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR36 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR37 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR38 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR39 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR40 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR41 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR42 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR43 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR44 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR45 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR46 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR47 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR48 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR49 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR50 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR51 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR52 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR53 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR54 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR55 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR56 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR57 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR58 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR59 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR60 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR61 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR62 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR63 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR64 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR65 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR66 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR67 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR68 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR69 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR70 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR71 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR72 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR73 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR74 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR75 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR76 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR77 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR78 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR79 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR80 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR81 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR82 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR83 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR84 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR85 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR86 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR87 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR88 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR89 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR90 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR91 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR92 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR93 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR94 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR95 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR96 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR97 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR98 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR99 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR100 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR101 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR102 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR103 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR104 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR105 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR106 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR107 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR108 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR109 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR110 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR111 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR112 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR113 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR114 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR115 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR116 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR117 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR118 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR119 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR120 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR121 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR122 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR123 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR124 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR125 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR126 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR127 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR128 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR129 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR130 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR131 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR132 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR133 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR134 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR135 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR136 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR137 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR138 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR139 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR140 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR141 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR142 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR143 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR144 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR145 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR146 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR147 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR148 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR149 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR150 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR151 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR152 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR153 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR154 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR155 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR156 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR157 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR158 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR159 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR160 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR161 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR162 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR163 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR164 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR165 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR166 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR167 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR168 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR169 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR170 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR171 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR172 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR173 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR174 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR175 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR176 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR177 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR178 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR179 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR180 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR181 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR182 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR183 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR184 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR185 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR186 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR187 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR188 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR189 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR190 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR191 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR192 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR193 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR194 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR195 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR196 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR197 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR198 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR199 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR200 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR201 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR202 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR203 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR204 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR205 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR206 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR207 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR208 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR209 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR210 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR211 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR212 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR213 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR214 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR215 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR216 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR217 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR218 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR219 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR220 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR221 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR222 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR223 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR224 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR225 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR226 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR227 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR228 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR229 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR230 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR231 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR232 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR233 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR234 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR235 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR236 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR237 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR238 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR239 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR240 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR241 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR242 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR243 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR244 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR245 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR246 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR247 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR248 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR249 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR250 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR251 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR252 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR253 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR254 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR255 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR256 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR257 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR258 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR259 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR260 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR261 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR262 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR263 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR264 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR265 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR266 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR267 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR268 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR269 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR270 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR271 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR272 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR273 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR274 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR275 {
    pub use super::CR21::PIXEL_COUNTERS;
}

/// CSI Control Register
pub mod CR276 {
    pub use super::CR21::PIXEL_COUNTERS;
}
#[repr(C)]
pub struct RegisterBlock {
    /// CSI Control Register 1
    pub CR1: RWRegister<u32>,

    /// CSI Control Register 2
    pub CR2: RWRegister<u32>,

    /// CSI Control Register 3
    pub CR3: RWRegister<u32>,

    /// CSI Statistic FIFO Register
    pub STATFIFO: RORegister<u32>,

    /// CSI RX FIFO Register
    pub RFIFO: RORegister<u32>,

    /// CSI RX Count Register
    pub RXCNT: RWRegister<u32>,

    /// CSI Status Register
    pub SR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// CSI DMA Start Address Register - for STATFIFO
    pub DMASA_STATFIFO: RWRegister<u32>,

    /// CSI DMA Transfer Size Register - for STATFIFO
    pub DMATS_STATFIFO: RWRegister<u32>,

    /// CSI DMA Start Address Register - for Frame Buffer1
    pub DMASA_FB1: RWRegister<u32>,

    /// CSI DMA Transfer Size Register - for Frame Buffer2
    pub DMASA_FB2: RWRegister<u32>,

    /// CSI Frame Buffer Parameter Register
    pub FBUF_PARA: RWRegister<u32>,

    /// CSI Image Parameter Register
    pub IMAG_PARA: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// CSI Control Register 18
    pub CR18: RWRegister<u32>,

    /// CSI Control Register 19
    pub CR19: RWRegister<u32>,

    /// CSI Control Register 20
    pub CR20: RWRegister<u32>,

    /// CSI Control Register
    pub CR21: RWRegister<u32>,

    /// CSI Control Register
    pub CR22: RWRegister<u32>,

    /// CSI Control Register
    pub CR23: RWRegister<u32>,

    /// CSI Control Register
    pub CR24: RWRegister<u32>,

    /// CSI Control Register
    pub CR25: RWRegister<u32>,

    /// CSI Control Register
    pub CR26: RWRegister<u32>,

    /// CSI Control Register
    pub CR27: RWRegister<u32>,

    /// CSI Control Register
    pub CR28: RWRegister<u32>,

    /// CSI Control Register
    pub CR29: RWRegister<u32>,

    /// CSI Control Register
    pub CR30: RWRegister<u32>,

    /// CSI Control Register
    pub CR31: RWRegister<u32>,

    /// CSI Control Register
    pub CR32: RWRegister<u32>,

    /// CSI Control Register
    pub CR33: RWRegister<u32>,

    /// CSI Control Register
    pub CR34: RWRegister<u32>,

    /// CSI Control Register
    pub CR35: RWRegister<u32>,

    /// CSI Control Register
    pub CR36: RWRegister<u32>,

    /// CSI Control Register
    pub CR37: RWRegister<u32>,

    /// CSI Control Register
    pub CR38: RWRegister<u32>,

    /// CSI Control Register
    pub CR39: RWRegister<u32>,

    /// CSI Control Register
    pub CR40: RWRegister<u32>,

    /// CSI Control Register
    pub CR41: RWRegister<u32>,

    /// CSI Control Register
    pub CR42: RWRegister<u32>,

    /// CSI Control Register
    pub CR43: RWRegister<u32>,

    /// CSI Control Register
    pub CR44: RWRegister<u32>,

    /// CSI Control Register
    pub CR45: RWRegister<u32>,

    /// CSI Control Register
    pub CR46: RWRegister<u32>,

    /// CSI Control Register
    pub CR47: RWRegister<u32>,

    /// CSI Control Register
    pub CR48: RWRegister<u32>,

    /// CSI Control Register
    pub CR49: RWRegister<u32>,

    /// CSI Control Register
    pub CR50: RWRegister<u32>,

    /// CSI Control Register
    pub CR51: RWRegister<u32>,

    /// CSI Control Register
    pub CR52: RWRegister<u32>,

    /// CSI Control Register
    pub CR53: RWRegister<u32>,

    /// CSI Control Register
    pub CR54: RWRegister<u32>,

    /// CSI Control Register
    pub CR55: RWRegister<u32>,

    /// CSI Control Register
    pub CR56: RWRegister<u32>,

    /// CSI Control Register
    pub CR57: RWRegister<u32>,

    /// CSI Control Register
    pub CR58: RWRegister<u32>,

    /// CSI Control Register
    pub CR59: RWRegister<u32>,

    /// CSI Control Register
    pub CR60: RWRegister<u32>,

    /// CSI Control Register
    pub CR61: RWRegister<u32>,

    /// CSI Control Register
    pub CR62: RWRegister<u32>,

    /// CSI Control Register
    pub CR63: RWRegister<u32>,

    /// CSI Control Register
    pub CR64: RWRegister<u32>,

    /// CSI Control Register
    pub CR65: RWRegister<u32>,

    /// CSI Control Register
    pub CR66: RWRegister<u32>,

    /// CSI Control Register
    pub CR67: RWRegister<u32>,

    /// CSI Control Register
    pub CR68: RWRegister<u32>,

    /// CSI Control Register
    pub CR69: RWRegister<u32>,

    /// CSI Control Register
    pub CR70: RWRegister<u32>,

    /// CSI Control Register
    pub CR71: RWRegister<u32>,

    /// CSI Control Register
    pub CR72: RWRegister<u32>,

    /// CSI Control Register
    pub CR73: RWRegister<u32>,

    /// CSI Control Register
    pub CR74: RWRegister<u32>,

    /// CSI Control Register
    pub CR75: RWRegister<u32>,

    /// CSI Control Register
    pub CR76: RWRegister<u32>,

    /// CSI Control Register
    pub CR77: RWRegister<u32>,

    /// CSI Control Register
    pub CR78: RWRegister<u32>,

    /// CSI Control Register
    pub CR79: RWRegister<u32>,

    /// CSI Control Register
    pub CR80: RWRegister<u32>,

    /// CSI Control Register
    pub CR81: RWRegister<u32>,

    /// CSI Control Register
    pub CR82: RWRegister<u32>,

    /// CSI Control Register
    pub CR83: RWRegister<u32>,

    /// CSI Control Register
    pub CR84: RWRegister<u32>,

    /// CSI Control Register
    pub CR85: RWRegister<u32>,

    /// CSI Control Register
    pub CR86: RWRegister<u32>,

    /// CSI Control Register
    pub CR87: RWRegister<u32>,

    /// CSI Control Register
    pub CR88: RWRegister<u32>,

    /// CSI Control Register
    pub CR89: RWRegister<u32>,

    /// CSI Control Register
    pub CR90: RWRegister<u32>,

    /// CSI Control Register
    pub CR91: RWRegister<u32>,

    /// CSI Control Register
    pub CR92: RWRegister<u32>,

    /// CSI Control Register
    pub CR93: RWRegister<u32>,

    /// CSI Control Register
    pub CR94: RWRegister<u32>,

    /// CSI Control Register
    pub CR95: RWRegister<u32>,

    /// CSI Control Register
    pub CR96: RWRegister<u32>,

    /// CSI Control Register
    pub CR97: RWRegister<u32>,

    /// CSI Control Register
    pub CR98: RWRegister<u32>,

    /// CSI Control Register
    pub CR99: RWRegister<u32>,

    /// CSI Control Register
    pub CR100: RWRegister<u32>,

    /// CSI Control Register
    pub CR101: RWRegister<u32>,

    /// CSI Control Register
    pub CR102: RWRegister<u32>,

    /// CSI Control Register
    pub CR103: RWRegister<u32>,

    /// CSI Control Register
    pub CR104: RWRegister<u32>,

    /// CSI Control Register
    pub CR105: RWRegister<u32>,

    /// CSI Control Register
    pub CR106: RWRegister<u32>,

    /// CSI Control Register
    pub CR107: RWRegister<u32>,

    /// CSI Control Register
    pub CR108: RWRegister<u32>,

    /// CSI Control Register
    pub CR109: RWRegister<u32>,

    /// CSI Control Register
    pub CR110: RWRegister<u32>,

    /// CSI Control Register
    pub CR111: RWRegister<u32>,

    /// CSI Control Register
    pub CR112: RWRegister<u32>,

    /// CSI Control Register
    pub CR113: RWRegister<u32>,

    /// CSI Control Register
    pub CR114: RWRegister<u32>,

    /// CSI Control Register
    pub CR115: RWRegister<u32>,

    /// CSI Control Register
    pub CR116: RWRegister<u32>,

    /// CSI Control Register
    pub CR117: RWRegister<u32>,

    /// CSI Control Register
    pub CR118: RWRegister<u32>,

    /// CSI Control Register
    pub CR119: RWRegister<u32>,

    /// CSI Control Register
    pub CR120: RWRegister<u32>,

    /// CSI Control Register
    pub CR121: RWRegister<u32>,

    /// CSI Control Register
    pub CR122: RWRegister<u32>,

    /// CSI Control Register
    pub CR123: RWRegister<u32>,

    /// CSI Control Register
    pub CR124: RWRegister<u32>,

    /// CSI Control Register
    pub CR125: RWRegister<u32>,

    /// CSI Control Register
    pub CR126: RWRegister<u32>,

    /// CSI Control Register
    pub CR127: RWRegister<u32>,

    /// CSI Control Register
    pub CR128: RWRegister<u32>,

    /// CSI Control Register
    pub CR129: RWRegister<u32>,

    /// CSI Control Register
    pub CR130: RWRegister<u32>,

    /// CSI Control Register
    pub CR131: RWRegister<u32>,

    /// CSI Control Register
    pub CR132: RWRegister<u32>,

    /// CSI Control Register
    pub CR133: RWRegister<u32>,

    /// CSI Control Register
    pub CR134: RWRegister<u32>,

    /// CSI Control Register
    pub CR135: RWRegister<u32>,

    /// CSI Control Register
    pub CR136: RWRegister<u32>,

    /// CSI Control Register
    pub CR137: RWRegister<u32>,

    /// CSI Control Register
    pub CR138: RWRegister<u32>,

    /// CSI Control Register
    pub CR139: RWRegister<u32>,

    /// CSI Control Register
    pub CR140: RWRegister<u32>,

    /// CSI Control Register
    pub CR141: RWRegister<u32>,

    /// CSI Control Register
    pub CR142: RWRegister<u32>,

    /// CSI Control Register
    pub CR143: RWRegister<u32>,

    /// CSI Control Register
    pub CR144: RWRegister<u32>,

    /// CSI Control Register
    pub CR145: RWRegister<u32>,

    /// CSI Control Register
    pub CR146: RWRegister<u32>,

    /// CSI Control Register
    pub CR147: RWRegister<u32>,

    /// CSI Control Register
    pub CR148: RWRegister<u32>,

    /// CSI Control Register
    pub CR149: RWRegister<u32>,

    /// CSI Control Register
    pub CR150: RWRegister<u32>,

    /// CSI Control Register
    pub CR151: RWRegister<u32>,

    /// CSI Control Register
    pub CR152: RWRegister<u32>,

    /// CSI Control Register
    pub CR153: RWRegister<u32>,

    /// CSI Control Register
    pub CR154: RWRegister<u32>,

    /// CSI Control Register
    pub CR155: RWRegister<u32>,

    /// CSI Control Register
    pub CR156: RWRegister<u32>,

    /// CSI Control Register
    pub CR157: RWRegister<u32>,

    /// CSI Control Register
    pub CR158: RWRegister<u32>,

    /// CSI Control Register
    pub CR159: RWRegister<u32>,

    /// CSI Control Register
    pub CR160: RWRegister<u32>,

    /// CSI Control Register
    pub CR161: RWRegister<u32>,

    /// CSI Control Register
    pub CR162: RWRegister<u32>,

    /// CSI Control Register
    pub CR163: RWRegister<u32>,

    /// CSI Control Register
    pub CR164: RWRegister<u32>,

    /// CSI Control Register
    pub CR165: RWRegister<u32>,

    /// CSI Control Register
    pub CR166: RWRegister<u32>,

    /// CSI Control Register
    pub CR167: RWRegister<u32>,

    /// CSI Control Register
    pub CR168: RWRegister<u32>,

    /// CSI Control Register
    pub CR169: RWRegister<u32>,

    /// CSI Control Register
    pub CR170: RWRegister<u32>,

    /// CSI Control Register
    pub CR171: RWRegister<u32>,

    /// CSI Control Register
    pub CR172: RWRegister<u32>,

    /// CSI Control Register
    pub CR173: RWRegister<u32>,

    /// CSI Control Register
    pub CR174: RWRegister<u32>,

    /// CSI Control Register
    pub CR175: RWRegister<u32>,

    /// CSI Control Register
    pub CR176: RWRegister<u32>,

    /// CSI Control Register
    pub CR177: RWRegister<u32>,

    /// CSI Control Register
    pub CR178: RWRegister<u32>,

    /// CSI Control Register
    pub CR179: RWRegister<u32>,

    /// CSI Control Register
    pub CR180: RWRegister<u32>,

    /// CSI Control Register
    pub CR181: RWRegister<u32>,

    /// CSI Control Register
    pub CR182: RWRegister<u32>,

    /// CSI Control Register
    pub CR183: RWRegister<u32>,

    /// CSI Control Register
    pub CR184: RWRegister<u32>,

    /// CSI Control Register
    pub CR185: RWRegister<u32>,

    /// CSI Control Register
    pub CR186: RWRegister<u32>,

    /// CSI Control Register
    pub CR187: RWRegister<u32>,

    /// CSI Control Register
    pub CR188: RWRegister<u32>,

    /// CSI Control Register
    pub CR189: RWRegister<u32>,

    /// CSI Control Register
    pub CR190: RWRegister<u32>,

    /// CSI Control Register
    pub CR191: RWRegister<u32>,

    /// CSI Control Register
    pub CR192: RWRegister<u32>,

    /// CSI Control Register
    pub CR193: RWRegister<u32>,

    /// CSI Control Register
    pub CR194: RWRegister<u32>,

    /// CSI Control Register
    pub CR195: RWRegister<u32>,

    /// CSI Control Register
    pub CR196: RWRegister<u32>,

    /// CSI Control Register
    pub CR197: RWRegister<u32>,

    /// CSI Control Register
    pub CR198: RWRegister<u32>,

    /// CSI Control Register
    pub CR199: RWRegister<u32>,

    /// CSI Control Register
    pub CR200: RWRegister<u32>,

    /// CSI Control Register
    pub CR201: RWRegister<u32>,

    /// CSI Control Register
    pub CR202: RWRegister<u32>,

    /// CSI Control Register
    pub CR203: RWRegister<u32>,

    /// CSI Control Register
    pub CR204: RWRegister<u32>,

    /// CSI Control Register
    pub CR205: RWRegister<u32>,

    /// CSI Control Register
    pub CR206: RWRegister<u32>,

    /// CSI Control Register
    pub CR207: RWRegister<u32>,

    /// CSI Control Register
    pub CR208: RWRegister<u32>,

    /// CSI Control Register
    pub CR209: RWRegister<u32>,

    /// CSI Control Register
    pub CR210: RWRegister<u32>,

    /// CSI Control Register
    pub CR211: RWRegister<u32>,

    /// CSI Control Register
    pub CR212: RWRegister<u32>,

    /// CSI Control Register
    pub CR213: RWRegister<u32>,

    /// CSI Control Register
    pub CR214: RWRegister<u32>,

    /// CSI Control Register
    pub CR215: RWRegister<u32>,

    /// CSI Control Register
    pub CR216: RWRegister<u32>,

    /// CSI Control Register
    pub CR217: RWRegister<u32>,

    /// CSI Control Register
    pub CR218: RWRegister<u32>,

    /// CSI Control Register
    pub CR219: RWRegister<u32>,

    /// CSI Control Register
    pub CR220: RWRegister<u32>,

    /// CSI Control Register
    pub CR221: RWRegister<u32>,

    /// CSI Control Register
    pub CR222: RWRegister<u32>,

    /// CSI Control Register
    pub CR223: RWRegister<u32>,

    /// CSI Control Register
    pub CR224: RWRegister<u32>,

    /// CSI Control Register
    pub CR225: RWRegister<u32>,

    /// CSI Control Register
    pub CR226: RWRegister<u32>,

    /// CSI Control Register
    pub CR227: RWRegister<u32>,

    /// CSI Control Register
    pub CR228: RWRegister<u32>,

    /// CSI Control Register
    pub CR229: RWRegister<u32>,

    /// CSI Control Register
    pub CR230: RWRegister<u32>,

    /// CSI Control Register
    pub CR231: RWRegister<u32>,

    /// CSI Control Register
    pub CR232: RWRegister<u32>,

    /// CSI Control Register
    pub CR233: RWRegister<u32>,

    /// CSI Control Register
    pub CR234: RWRegister<u32>,

    /// CSI Control Register
    pub CR235: RWRegister<u32>,

    /// CSI Control Register
    pub CR236: RWRegister<u32>,

    /// CSI Control Register
    pub CR237: RWRegister<u32>,

    /// CSI Control Register
    pub CR238: RWRegister<u32>,

    /// CSI Control Register
    pub CR239: RWRegister<u32>,

    /// CSI Control Register
    pub CR240: RWRegister<u32>,

    /// CSI Control Register
    pub CR241: RWRegister<u32>,

    /// CSI Control Register
    pub CR242: RWRegister<u32>,

    /// CSI Control Register
    pub CR243: RWRegister<u32>,

    /// CSI Control Register
    pub CR244: RWRegister<u32>,

    /// CSI Control Register
    pub CR245: RWRegister<u32>,

    /// CSI Control Register
    pub CR246: RWRegister<u32>,

    /// CSI Control Register
    pub CR247: RWRegister<u32>,

    /// CSI Control Register
    pub CR248: RWRegister<u32>,

    /// CSI Control Register
    pub CR249: RWRegister<u32>,

    /// CSI Control Register
    pub CR250: RWRegister<u32>,

    /// CSI Control Register
    pub CR251: RWRegister<u32>,

    /// CSI Control Register
    pub CR252: RWRegister<u32>,

    /// CSI Control Register
    pub CR253: RWRegister<u32>,

    /// CSI Control Register
    pub CR254: RWRegister<u32>,

    /// CSI Control Register
    pub CR255: RWRegister<u32>,

    /// CSI Control Register
    pub CR256: RWRegister<u32>,

    /// CSI Control Register
    pub CR257: RWRegister<u32>,

    /// CSI Control Register
    pub CR258: RWRegister<u32>,

    /// CSI Control Register
    pub CR259: RWRegister<u32>,

    /// CSI Control Register
    pub CR260: RWRegister<u32>,

    /// CSI Control Register
    pub CR261: RWRegister<u32>,

    /// CSI Control Register
    pub CR262: RWRegister<u32>,

    /// CSI Control Register
    pub CR263: RWRegister<u32>,

    /// CSI Control Register
    pub CR264: RWRegister<u32>,

    /// CSI Control Register
    pub CR265: RWRegister<u32>,

    /// CSI Control Register
    pub CR266: RWRegister<u32>,

    /// CSI Control Register
    pub CR267: RWRegister<u32>,

    /// CSI Control Register
    pub CR268: RWRegister<u32>,

    /// CSI Control Register
    pub CR269: RWRegister<u32>,

    /// CSI Control Register
    pub CR270: RWRegister<u32>,

    /// CSI Control Register
    pub CR271: RWRegister<u32>,

    /// CSI Control Register
    pub CR272: RWRegister<u32>,

    /// CSI Control Register
    pub CR273: RWRegister<u32>,

    /// CSI Control Register
    pub CR274: RWRegister<u32>,

    /// CSI Control Register
    pub CR275: RWRegister<u32>,

    /// CSI Control Register
    pub CR276: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub CR3: u32,
    pub STATFIFO: u32,
    pub RFIFO: u32,
    pub RXCNT: u32,
    pub SR: u32,
    pub DMASA_STATFIFO: u32,
    pub DMATS_STATFIFO: u32,
    pub DMASA_FB1: u32,
    pub DMASA_FB2: u32,
    pub FBUF_PARA: u32,
    pub IMAG_PARA: u32,
    pub CR18: u32,
    pub CR19: u32,
    pub CR20: u32,
    pub CR21: u32,
    pub CR22: u32,
    pub CR23: u32,
    pub CR24: u32,
    pub CR25: u32,
    pub CR26: u32,
    pub CR27: u32,
    pub CR28: u32,
    pub CR29: u32,
    pub CR30: u32,
    pub CR31: u32,
    pub CR32: u32,
    pub CR33: u32,
    pub CR34: u32,
    pub CR35: u32,
    pub CR36: u32,
    pub CR37: u32,
    pub CR38: u32,
    pub CR39: u32,
    pub CR40: u32,
    pub CR41: u32,
    pub CR42: u32,
    pub CR43: u32,
    pub CR44: u32,
    pub CR45: u32,
    pub CR46: u32,
    pub CR47: u32,
    pub CR48: u32,
    pub CR49: u32,
    pub CR50: u32,
    pub CR51: u32,
    pub CR52: u32,
    pub CR53: u32,
    pub CR54: u32,
    pub CR55: u32,
    pub CR56: u32,
    pub CR57: u32,
    pub CR58: u32,
    pub CR59: u32,
    pub CR60: u32,
    pub CR61: u32,
    pub CR62: u32,
    pub CR63: u32,
    pub CR64: u32,
    pub CR65: u32,
    pub CR66: u32,
    pub CR67: u32,
    pub CR68: u32,
    pub CR69: u32,
    pub CR70: u32,
    pub CR71: u32,
    pub CR72: u32,
    pub CR73: u32,
    pub CR74: u32,
    pub CR75: u32,
    pub CR76: u32,
    pub CR77: u32,
    pub CR78: u32,
    pub CR79: u32,
    pub CR80: u32,
    pub CR81: u32,
    pub CR82: u32,
    pub CR83: u32,
    pub CR84: u32,
    pub CR85: u32,
    pub CR86: u32,
    pub CR87: u32,
    pub CR88: u32,
    pub CR89: u32,
    pub CR90: u32,
    pub CR91: u32,
    pub CR92: u32,
    pub CR93: u32,
    pub CR94: u32,
    pub CR95: u32,
    pub CR96: u32,
    pub CR97: u32,
    pub CR98: u32,
    pub CR99: u32,
    pub CR100: u32,
    pub CR101: u32,
    pub CR102: u32,
    pub CR103: u32,
    pub CR104: u32,
    pub CR105: u32,
    pub CR106: u32,
    pub CR107: u32,
    pub CR108: u32,
    pub CR109: u32,
    pub CR110: u32,
    pub CR111: u32,
    pub CR112: u32,
    pub CR113: u32,
    pub CR114: u32,
    pub CR115: u32,
    pub CR116: u32,
    pub CR117: u32,
    pub CR118: u32,
    pub CR119: u32,
    pub CR120: u32,
    pub CR121: u32,
    pub CR122: u32,
    pub CR123: u32,
    pub CR124: u32,
    pub CR125: u32,
    pub CR126: u32,
    pub CR127: u32,
    pub CR128: u32,
    pub CR129: u32,
    pub CR130: u32,
    pub CR131: u32,
    pub CR132: u32,
    pub CR133: u32,
    pub CR134: u32,
    pub CR135: u32,
    pub CR136: u32,
    pub CR137: u32,
    pub CR138: u32,
    pub CR139: u32,
    pub CR140: u32,
    pub CR141: u32,
    pub CR142: u32,
    pub CR143: u32,
    pub CR144: u32,
    pub CR145: u32,
    pub CR146: u32,
    pub CR147: u32,
    pub CR148: u32,
    pub CR149: u32,
    pub CR150: u32,
    pub CR151: u32,
    pub CR152: u32,
    pub CR153: u32,
    pub CR154: u32,
    pub CR155: u32,
    pub CR156: u32,
    pub CR157: u32,
    pub CR158: u32,
    pub CR159: u32,
    pub CR160: u32,
    pub CR161: u32,
    pub CR162: u32,
    pub CR163: u32,
    pub CR164: u32,
    pub CR165: u32,
    pub CR166: u32,
    pub CR167: u32,
    pub CR168: u32,
    pub CR169: u32,
    pub CR170: u32,
    pub CR171: u32,
    pub CR172: u32,
    pub CR173: u32,
    pub CR174: u32,
    pub CR175: u32,
    pub CR176: u32,
    pub CR177: u32,
    pub CR178: u32,
    pub CR179: u32,
    pub CR180: u32,
    pub CR181: u32,
    pub CR182: u32,
    pub CR183: u32,
    pub CR184: u32,
    pub CR185: u32,
    pub CR186: u32,
    pub CR187: u32,
    pub CR188: u32,
    pub CR189: u32,
    pub CR190: u32,
    pub CR191: u32,
    pub CR192: u32,
    pub CR193: u32,
    pub CR194: u32,
    pub CR195: u32,
    pub CR196: u32,
    pub CR197: u32,
    pub CR198: u32,
    pub CR199: u32,
    pub CR200: u32,
    pub CR201: u32,
    pub CR202: u32,
    pub CR203: u32,
    pub CR204: u32,
    pub CR205: u32,
    pub CR206: u32,
    pub CR207: u32,
    pub CR208: u32,
    pub CR209: u32,
    pub CR210: u32,
    pub CR211: u32,
    pub CR212: u32,
    pub CR213: u32,
    pub CR214: u32,
    pub CR215: u32,
    pub CR216: u32,
    pub CR217: u32,
    pub CR218: u32,
    pub CR219: u32,
    pub CR220: u32,
    pub CR221: u32,
    pub CR222: u32,
    pub CR223: u32,
    pub CR224: u32,
    pub CR225: u32,
    pub CR226: u32,
    pub CR227: u32,
    pub CR228: u32,
    pub CR229: u32,
    pub CR230: u32,
    pub CR231: u32,
    pub CR232: u32,
    pub CR233: u32,
    pub CR234: u32,
    pub CR235: u32,
    pub CR236: u32,
    pub CR237: u32,
    pub CR238: u32,
    pub CR239: u32,
    pub CR240: u32,
    pub CR241: u32,
    pub CR242: u32,
    pub CR243: u32,
    pub CR244: u32,
    pub CR245: u32,
    pub CR246: u32,
    pub CR247: u32,
    pub CR248: u32,
    pub CR249: u32,
    pub CR250: u32,
    pub CR251: u32,
    pub CR252: u32,
    pub CR253: u32,
    pub CR254: u32,
    pub CR255: u32,
    pub CR256: u32,
    pub CR257: u32,
    pub CR258: u32,
    pub CR259: u32,
    pub CR260: u32,
    pub CR261: u32,
    pub CR262: u32,
    pub CR263: u32,
    pub CR264: u32,
    pub CR265: u32,
    pub CR266: u32,
    pub CR267: u32,
    pub CR268: u32,
    pub CR269: u32,
    pub CR270: u32,
    pub CR271: u32,
    pub CR272: u32,
    pub CR273: u32,
    pub CR274: u32,
    pub CR275: u32,
    pub CR276: u32,
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
