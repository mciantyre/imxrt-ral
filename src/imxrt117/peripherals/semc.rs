#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SEMC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Module Control Register
pub mod MCR {

    /// Software Reset
    pub mod SWRST {
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

            /// 0b0: No reset
            pub const SWRST_0: u32 = 0b0;

            /// 0b1: Reset
            pub const SWRST_1: u32 = 0b1;
        }
    }

    /// Module Disable
    pub mod MDIS {
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

            /// 0b0: Module enabled
            pub const MDIS_0: u32 = 0b0;

            /// 0b1: Module disabled
            pub const MDIS_1: u32 = 0b1;
        }
    }

    /// DQS (read strobe) mode
    pub mod DQSMD {
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

            /// 0b0: Dummy read strobe loopbacked internally
            pub const DQSMD_0: u32 = 0b0;

            /// 0b1: Dummy read strobe loopbacked from DQS pad
            pub const DQSMD_1: u32 = 0b1;
        }
    }

    /// WAIT/RDY polarity for SRAM/NOR
    pub mod WPOL0 {
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

            /// 0b0: WAIT/RDY polarity is not changed.
            pub const WPOL0_0: u32 = 0b0;

            /// 0b1: WAIT/RDY polarity is inverted.
            pub const WPOL0_1: u32 = 0b1;
        }
    }

    /// R/B# polarity for NAND device
    pub mod WPOL1 {
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

            /// 0b0: R/B# polarity is not changed.
            pub const WPOL1_0: u32 = 0b0;

            /// 0b1: R/B# polarity is inverted.
            pub const WPOL1_1: u32 = 0b1;
        }
    }

    /// Command Execution timeout cycles
    pub mod CTO {
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

    /// Bus timeout cycles
    pub mod BTO {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 255*1
            pub const BTO_0: u32 = 0b00000;

            /// 0b00001: 255*2
            pub const BTO_1: u32 = 0b00001;

            /// 0b11111: 255*231
            pub const BTO_31: u32 = 0b11111;
        }
    }
}

/// IO MUX Control Register
pub mod IOCR {

    /// SEMC_ADDR08 output selection
    pub mod MUX_A8 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode
            pub const MUX_A8_0: u32 = 0b0000;

            /// 0b0001: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode
            pub const MUX_A8_1: u32 = 0b0001;

            /// 0b0010: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode
            pub const MUX_A8_2: u32 = 0b0010;

            /// 0b0011: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode
            pub const MUX_A8_3: u32 = 0b0011;

            /// 0b0100: NAND CE#
            pub const MUX_A8_4: u32 = 0b0100;

            /// 0b0101: NOR CE#
            pub const MUX_A8_5: u32 = 0b0101;

            /// 0b0110: SRAM CE# 0
            pub const MUX_A8_6: u32 = 0b0110;

            /// 0b0111: DBI CSX
            pub const MUX_A8_7: u32 = 0b0111;

            /// 0b1000: SRAM CE# 1
            pub const MUX_A8_8: u32 = 0b1000;

            /// 0b1001: SRAM CE# 2
            pub const MUX_A8_9: u32 = 0b1001;

            /// 0b1010: SRAM CE# 3
            pub const MUX_A8_10: u32 = 0b1010;

            /// 0b1011: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode
            pub const MUX_A8_11: u32 = 0b1011;

            /// 0b1100: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode
            pub const MUX_A8_12: u32 = 0b1100;

            /// 0b1101: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode
            pub const MUX_A8_13: u32 = 0b1101;

            /// 0b1110: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode
            pub const MUX_A8_14: u32 = 0b1110;

            /// 0b1111: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode
            pub const MUX_A8_15: u32 = 0b1111;
        }
    }

    /// SEMC_CSX0 output selection
    pub mod MUX_CSX0 {
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

            /// 0b0000: NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode
            pub const MUX_CSX0_0: u32 = 0b0000;

            /// 0b0001: SDRAM CS1
            pub const MUX_CSX0_1: u32 = 0b0001;

            /// 0b0010: SDRAM CS2
            pub const MUX_CSX0_2: u32 = 0b0010;

            /// 0b0011: SDRAM CS3
            pub const MUX_CSX0_3: u32 = 0b0011;

            /// 0b0100: NAND CE#
            pub const MUX_CSX0_4: u32 = 0b0100;

            /// 0b0101: NOR CE#
            pub const MUX_CSX0_5: u32 = 0b0101;

            /// 0b0110: SRAM CE# 0
            pub const MUX_CSX0_6: u32 = 0b0110;

            /// 0b0111: DBI CSX
            pub const MUX_CSX0_7: u32 = 0b0111;

            /// 0b1000: SRAM CE# 1
            pub const MUX_CSX0_8: u32 = 0b1000;

            /// 0b1001: SRAM CE# 2
            pub const MUX_CSX0_9: u32 = 0b1001;

            /// 0b1010: SRAM CE# 3
            pub const MUX_CSX0_10: u32 = 0b1010;

            /// 0b1011: NOR/SRAM Address bit 24 (A24)
            pub const MUX_CSX0_11: u32 = 0b1011;

            /// 0b1100: NOR/SRAM Address bit 24 (A24)
            pub const MUX_CSX0_12: u32 = 0b1100;

            /// 0b1101: NOR/SRAM Address bit 24 (A24)
            pub const MUX_CSX0_13: u32 = 0b1101;

            /// 0b1110: NOR/SRAM Address bit 24 (A24)
            pub const MUX_CSX0_14: u32 = 0b1110;

            /// 0b1111: NOR/SRAM Address bit 24 (A24)
            pub const MUX_CSX0_15: u32 = 0b1111;
        }
    }

    /// SEMC_CSX1 output selection
    pub mod MUX_CSX1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode
            pub const MUX_CSX1_0: u32 = 0b0000;

            /// 0b0001: SDRAM CS1
            pub const MUX_CSX1_1: u32 = 0b0001;

            /// 0b0010: SDRAM CS2
            pub const MUX_CSX1_2: u32 = 0b0010;

            /// 0b0011: SDRAM CS3
            pub const MUX_CSX1_3: u32 = 0b0011;

            /// 0b0100: NAND CE#
            pub const MUX_CSX1_4: u32 = 0b0100;

            /// 0b0101: NOR CE#
            pub const MUX_CSX1_5: u32 = 0b0101;

            /// 0b0110: SRAM CE# 0
            pub const MUX_CSX1_6: u32 = 0b0110;

            /// 0b0111: DBI CSX
            pub const MUX_CSX1_7: u32 = 0b0111;

            /// 0b1000: SRAM CE# 1
            pub const MUX_CSX1_8: u32 = 0b1000;

            /// 0b1001: SRAM CE# 2
            pub const MUX_CSX1_9: u32 = 0b1001;

            /// 0b1010: SRAM CE# 3
            pub const MUX_CSX1_10: u32 = 0b1010;

            /// 0b1011: NOR/SRAM Address bit 25 (A25)
            pub const MUX_CSX1_11: u32 = 0b1011;

            /// 0b1100: NOR/SRAM Address bit 25 (A25)
            pub const MUX_CSX1_12: u32 = 0b1100;

            /// 0b1101: NOR/SRAM Address bit 25 (A25)
            pub const MUX_CSX1_13: u32 = 0b1101;

            /// 0b1110: NOR/SRAM Address bit 25 (A25)
            pub const MUX_CSX1_14: u32 = 0b1110;

            /// 0b1111: NOR/SRAM Address bit 25 (A25)
            pub const MUX_CSX1_15: u32 = 0b1111;
        }
    }

    /// SEMC_CSX2 output selection
    pub mod MUX_CSX2 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode
            pub const MUX_CSX2_0: u32 = 0b0000;

            /// 0b0001: SDRAM CS1
            pub const MUX_CSX2_1: u32 = 0b0001;

            /// 0b0010: SDRAM CS2
            pub const MUX_CSX2_2: u32 = 0b0010;

            /// 0b0011: SDRAM CS3
            pub const MUX_CSX2_3: u32 = 0b0011;

            /// 0b0100: NAND CE#
            pub const MUX_CSX2_4: u32 = 0b0100;

            /// 0b0101: NOR CE#
            pub const MUX_CSX2_5: u32 = 0b0101;

            /// 0b0110: SRAM CE# 0
            pub const MUX_CSX2_6: u32 = 0b0110;

            /// 0b0111: DBI CSX
            pub const MUX_CSX2_7: u32 = 0b0111;

            /// 0b1000: SRAM CE# 1
            pub const MUX_CSX2_8: u32 = 0b1000;

            /// 0b1001: SRAM CE# 2
            pub const MUX_CSX2_9: u32 = 0b1001;

            /// 0b1010: SRAM CE# 3
            pub const MUX_CSX2_10: u32 = 0b1010;

            /// 0b1011: NOR/SRAM Address bit 26 (A26)
            pub const MUX_CSX2_11: u32 = 0b1011;

            /// 0b1100: NOR/SRAM Address bit 26 (A26)
            pub const MUX_CSX2_12: u32 = 0b1100;

            /// 0b1101: NOR/SRAM Address bit 26 (A26)
            pub const MUX_CSX2_13: u32 = 0b1101;

            /// 0b1110: NOR/SRAM Address bit 26 (A26)
            pub const MUX_CSX2_14: u32 = 0b1110;

            /// 0b1111: NOR/SRAM Address bit 26 (A26)
            pub const MUX_CSX2_15: u32 = 0b1111;
        }
    }

    /// SEMC_CSX3 output selection
    pub mod MUX_CSX3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode
            pub const MUX_CSX3_0: u32 = 0b0000;

            /// 0b0001: SDRAM CS1
            pub const MUX_CSX3_1: u32 = 0b0001;

            /// 0b0010: SDRAM CS2
            pub const MUX_CSX3_2: u32 = 0b0010;

            /// 0b0011: SDRAM CS3
            pub const MUX_CSX3_3: u32 = 0b0011;

            /// 0b0100: NAND CE#
            pub const MUX_CSX3_4: u32 = 0b0100;

            /// 0b0101: NOR CE#
            pub const MUX_CSX3_5: u32 = 0b0101;

            /// 0b0110: SRAM CE# 0
            pub const MUX_CSX3_6: u32 = 0b0110;

            /// 0b0111: DBI CSX
            pub const MUX_CSX3_7: u32 = 0b0111;

            /// 0b1000: SRAM CE# 1
            pub const MUX_CSX3_8: u32 = 0b1000;

            /// 0b1001: SRAM CE# 2
            pub const MUX_CSX3_9: u32 = 0b1001;

            /// 0b1010: SRAM CE# 3
            pub const MUX_CSX3_10: u32 = 0b1010;

            /// 0b1011: NOR/SRAM Address bit 27 (A27)
            pub const MUX_CSX3_11: u32 = 0b1011;

            /// 0b1100: NOR/SRAM Address bit 27 (A27)
            pub const MUX_CSX3_12: u32 = 0b1100;

            /// 0b1101: NOR/SRAM Address bit 27 (A27)
            pub const MUX_CSX3_13: u32 = 0b1101;

            /// 0b1110: NOR/SRAM Address bit 27 (A27)
            pub const MUX_CSX3_14: u32 = 0b1110;

            /// 0b1111: NOR/SRAM Address bit 27 (A27)
            pub const MUX_CSX3_15: u32 = 0b1111;
        }
    }

    /// SEMC_RDY function selection
    pub mod MUX_RDY {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: NAND R/B# input
            pub const MUX_RDY_0: u32 = 0b0000;

            /// 0b0001: SDRAM CS1
            pub const MUX_RDY_1: u32 = 0b0001;

            /// 0b0010: SDRAM CS2
            pub const MUX_RDY_2: u32 = 0b0010;

            /// 0b0011: SDRAM CS3
            pub const MUX_RDY_3: u32 = 0b0011;

            /// 0b0100: NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode
            pub const MUX_RDY_4: u32 = 0b0100;

            /// 0b0101: NOR CE#
            pub const MUX_RDY_5: u32 = 0b0101;

            /// 0b0110: SRAM CE# 0
            pub const MUX_RDY_6: u32 = 0b0110;

            /// 0b0111: DBI CSX
            pub const MUX_RDY_7: u32 = 0b0111;

            /// 0b1000: SRAM CE# 1
            pub const MUX_RDY_8: u32 = 0b1000;

            /// 0b1001: SRAM CE# 2
            pub const MUX_RDY_9: u32 = 0b1001;

            /// 0b1010: SRAM CE# 3
            pub const MUX_RDY_10: u32 = 0b1010;

            /// 0b1011: NOR/SRAM Address bit 27
            pub const MUX_RDY_11: u32 = 0b1011;

            /// 0b1100: NOR/SRAM Address bit 27
            pub const MUX_RDY_12: u32 = 0b1100;

            /// 0b1101: NOR/SRAM Address bit 27
            pub const MUX_RDY_13: u32 = 0b1101;

            /// 0b1110: NOR/SRAM Address bit 27
            pub const MUX_RDY_14: u32 = 0b1110;

            /// 0b1111: NOR/SRAM Address bit 27
            pub const MUX_RDY_15: u32 = 0b1111;
        }
    }

    /// SEMC_CLKX0 function selection
    pub mod MUX_CLKX0 {
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

            /// 0b00: Keep low
            pub const MUX_CLKX0_0: u32 = 0b00;

            /// 0b01: NOR clock
            pub const MUX_CLKX0_1: u32 = 0b01;

            /// 0b10: SRAM clock
            pub const MUX_CLKX0_2: u32 = 0b10;

            /// 0b11: NOR and SRAM clock, suitable for Multi-Chip Product package
            pub const MUX_CLKX0_3: u32 = 0b11;
        }
    }

    /// SEMC_CLKX1 function selection
    pub mod MUX_CLKX1 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Keep low
            pub const MUX_CLKX1_0: u32 = 0b00;

            /// 0b01: NOR clock
            pub const MUX_CLKX1_1: u32 = 0b01;

            /// 0b10: SRAM clock
            pub const MUX_CLKX1_2: u32 = 0b10;

            /// 0b11: NOR and SRAM clock, suitable for Multi-Chip Product package
            pub const MUX_CLKX1_3: u32 = 0b11;
        }
    }

    /// SEMC_CLKX0 Always On
    pub mod CLKX0_AO {
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

            /// 0b0: SEMC_CLKX0 is controlled by MUX_CLKX0
            pub const CLKX0_AO_0: u32 = 0b0;

            /// 0b1: SEMC_CLKX0 is always on
            pub const CLKX0_AO_1: u32 = 0b1;
        }
    }

    /// SEMC_CLKX1 Always On
    pub mod CLKX1_AO {
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

            /// 0b0: SEMC_CLKX1 is controlled by MUX_CLKX1
            pub const CLKX1_AO_0: u32 = 0b0;

            /// 0b1: SEMC_CLKX1 is always on
            pub const CLKX1_AO_1: u32 = 0b1;
        }
    }
}

/// Bus (AXI) Master Control Register 0
pub mod BMCR0 {

    /// Weight of QOS
    pub mod WQOS {
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

    /// Weight of AGE
    pub mod WAGE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Weight of Slave Hit without read/write switch
    pub mod WSH {
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

    /// Weight of slave hit with Read/Write Switch
    pub mod WRWS {
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

/// Bus (AXI) Master Control Register 1
pub mod BMCR1 {

    /// Weight of QOS
    pub mod WQOS {
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

    /// Weight of AGE
    pub mod WAGE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Weight of Page Hit
    pub mod WPH {
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

    /// Weight of slave hit without Read/Write Switch
    pub mod WRWS {
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

    /// Weight of Bank Rotation
    pub mod WBR {
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

/// Base Register n
pub mod BR0 {

    /// Valid
    pub mod VLD {
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

            /// 0b0: The memory is invalid, can not be accessed.
            pub const VLD_0: u32 = 0b0;

            /// 0b1: The memory is valid, can be accessed.
            pub const VLD_1: u32 = 0b1;
        }
    }

    /// Memory size
    pub mod MS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (5 bits: 0b11111 << 1)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 4KB
            pub const MS_0: u32 = 0b00000;

            /// 0b00001: 8KB
            pub const MS_1: u32 = 0b00001;

            /// 0b00010: 16KB
            pub const MS_2: u32 = 0b00010;

            /// 0b00011: 32KB
            pub const MS_3: u32 = 0b00011;

            /// 0b00100: 64KB
            pub const MS_4: u32 = 0b00100;

            /// 0b00101: 128KB
            pub const MS_5: u32 = 0b00101;

            /// 0b00110: 256KB
            pub const MS_6: u32 = 0b00110;

            /// 0b00111: 512KB
            pub const MS_7: u32 = 0b00111;

            /// 0b01000: 1MB
            pub const MS_8: u32 = 0b01000;

            /// 0b01001: 2MB
            pub const MS_9: u32 = 0b01001;

            /// 0b01010: 4MB
            pub const MS_10: u32 = 0b01010;

            /// 0b01011: 8MB
            pub const MS_11: u32 = 0b01011;

            /// 0b01100: 16MB
            pub const MS_12: u32 = 0b01100;

            /// 0b01101: 32MB
            pub const MS_13: u32 = 0b01101;

            /// 0b01110: 64MB
            pub const MS_14: u32 = 0b01110;

            /// 0b01111: 128MB
            pub const MS_15: u32 = 0b01111;

            /// 0b10000: 256MB
            pub const MS_16: u32 = 0b10000;

            /// 0b10001: 512MB
            pub const MS_17: u32 = 0b10001;

            /// 0b10010: 1GB
            pub const MS_18: u32 = 0b10010;

            /// 0b10011: 2GB
            pub const MS_19: u32 = 0b10011;

            /// 0b10100: 4GB
            pub const MS_20: u32 = 0b10100;

            /// 0b10101: 4GB
            pub const MS_21: u32 = 0b10101;

            /// 0b10110: 4GB
            pub const MS_22: u32 = 0b10110;

            /// 0b10111: 4GB
            pub const MS_23: u32 = 0b10111;

            /// 0b11000: 4GB
            pub const MS_24: u32 = 0b11000;

            /// 0b11001: 4GB
            pub const MS_25: u32 = 0b11001;

            /// 0b11010: 4GB
            pub const MS_26: u32 = 0b11010;

            /// 0b11011: 4GB
            pub const MS_27: u32 = 0b11011;

            /// 0b11100: 4GB
            pub const MS_28: u32 = 0b11100;

            /// 0b11101: 4GB
            pub const MS_29: u32 = 0b11101;

            /// 0b11110: 4GB
            pub const MS_30: u32 = 0b11110;

            /// 0b11111: 4GB
            pub const MS_31: u32 = 0b11111;
        }
    }

    /// Base Address
    pub mod BA {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Base Register n
pub mod BR1 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register n
pub mod BR2 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register n
pub mod BR3 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register n
pub mod BR4 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register n
pub mod BR5 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register n
pub mod BR6 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register n
pub mod BR7 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register n
pub mod BR8 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// DLL Control Register
pub mod DLLCR {

    /// DLL calibration enable
    pub mod DLLEN {
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

            /// 0b0: DLL calibration is disabled.
            pub const DLLEN_0: u32 = 0b0;

            /// 0b1: DLL calibration is enabled.
            pub const DLLEN_1: u32 = 0b1;
        }
    }

    /// DLL Reset
    pub mod DLLRESET {
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

            /// 0b0: DLL is not reset.
            pub const DLLRESET_0: u32 = 0b0;

            /// 0b1: DLL is reset.
            pub const DLLRESET_1: u32 = 0b1;
        }
    }

    /// Delay Target for Slave
    pub mod SLVDLYTARGET {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (4 bits: 0b1111 << 3)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Override Enable
    pub mod OVRDEN {
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

            /// 0b0: The delay cell number is not overridden.
            pub const OVRDEN_0: u32 = 0b0;

            /// 0b1: The delay cell number is overridden.
            pub const OVRDEN_1: u32 = 0b1;
        }
    }

    /// Override Value
    pub mod OVRDVAL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (6 bits: 0x3f << 9)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Enable Register
pub mod INTEN {

    /// IP command done interrupt enable
    pub mod IPCMDDONEEN {
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

            /// 0b0: Interrupt is disabled
            pub const IPCMDDONEEN_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const IPCMDDONEEN_1: u32 = 0b1;
        }
    }

    /// IP command error interrupt enable
    pub mod IPCMDERREN {
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

            /// 0b0: Interrupt is disabled
            pub const IPCMDERREN_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const IPCMDERREN_1: u32 = 0b1;
        }
    }

    /// AXI command error interrupt enable
    pub mod AXICMDERREN {
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

            /// 0b0: Interrupt is disabled
            pub const AXICMDERREN_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const AXICMDERREN_1: u32 = 0b1;
        }
    }

    /// AXI bus error interrupt enable
    pub mod AXIBUSERREN {
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

            /// 0b0: Interrupt is disabled
            pub const AXIBUSERREN_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const AXIBUSERREN_1: u32 = 0b1;
        }
    }

    /// NAND page end interrupt enable
    pub mod NDPAGEENDEN {
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

            /// 0b0: Interrupt is disabled
            pub const NDPAGEENDEN_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const NDPAGEENDEN_1: u32 = 0b1;
        }
    }

    /// NAND no pending AXI access interrupt enable
    pub mod NDNOPENDEN {
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

            /// 0b0: Interrupt is disabled
            pub const NDNOPENDEN_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const NDNOPENDEN_1: u32 = 0b1;
        }
    }
}

/// Interrupt Register
pub mod INTR {

    /// IP command normal done interrupt
    pub mod IPCMDDONE {
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

            /// 0b0: IP command is not done.
            pub const IPCMDDONE_0: u32 = 0b0;

            /// 0b1: IP command is done.
            pub const IPCMDDONE_1: u32 = 0b1;
        }
    }

    /// IP command error done interrupt
    pub mod IPCMDERR {
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

            /// 0b0: No IP command error.
            pub const IPCMDERR_0: u32 = 0b0;

            /// 0b1: IP command error occurs.
            pub const IPCMDERR_1: u32 = 0b1;
        }
    }

    /// AXI command error interrupt
    pub mod AXICMDERR {
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

            /// 0b0: No AXI command error.
            pub const AXICMDERR_0: u32 = 0b0;

            /// 0b1: AXI command error occurs.
            pub const AXICMDERR_1: u32 = 0b1;
        }
    }

    /// AXI bus error interrupt
    pub mod AXIBUSERR {
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

            /// 0b0: No AXI bus error.
            pub const AXIBUSERR_0: u32 = 0b0;

            /// 0b1: AXI bus error occurs.
            pub const AXIBUSERR_1: u32 = 0b1;
        }
    }

    /// NAND page end interrupt
    pub mod NDPAGEEND {
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

            /// 0b0: The last address of main space in the NAND is not written by AXI command.
            pub const NDPAGEEND_0: u32 = 0b0;

            /// 0b1: The last address of main space in the NAND is written by AXI command.
            pub const NDPAGEEND_1: u32 = 0b1;
        }
    }

    /// NAND no pending AXI write transaction interrupt
    pub mod NDNOPEND {
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

            /// 0b0: At least one NAND AXI write transaction is pending or no NAND write transaction is sent to the queue.
            pub const NDNOPEND_0: u32 = 0b0;

            /// 0b1: All NAND AXI write pending transactions are finished.
            pub const NDNOPEND_1: u32 = 0b1;
        }
    }
}

/// SDRAM Control Register 0
pub mod SDRAMCR0 {

    /// Port Size
    pub mod PS {
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

            /// 0b00: 8bit
            pub const PS_0: u32 = 0b00;

            /// 0b01: 16bit
            pub const PS_1: u32 = 0b01;

            /// 0b10: 32bit
            pub const PS_2: u32 = 0b10;
        }
    }

    /// Burst Length
    pub mod BL {
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

            /// 0b000: 1
            pub const BL_0: u32 = 0b000;

            /// 0b001: 2
            pub const BL_1: u32 = 0b001;

            /// 0b010: 4
            pub const BL_2: u32 = 0b010;

            /// 0b011: 8
            pub const BL_3: u32 = 0b011;

            /// 0b100: 8
            pub const BL_4: u32 = 0b100;

            /// 0b101: 8
            pub const BL_5: u32 = 0b101;

            /// 0b110: 8
            pub const BL_6: u32 = 0b110;

            /// 0b111: 8
            pub const BL_7: u32 = 0b111;
        }
    }

    /// Column 8 selection
    pub mod COL8 {
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

            /// 0b0: Column address bit number is decided by COL field.
            pub const COL8_0: u32 = 0b0;

            /// 0b1: Column address bit number is 8. COL field is ignored.
            pub const COL8_1: u32 = 0b1;
        }
    }

    /// Column address bit number
    pub mod COL {
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

            /// 0b00: 12
            pub const COL_0: u32 = 0b00;

            /// 0b01: 11
            pub const COL_1: u32 = 0b01;

            /// 0b10: 10
            pub const COL_2: u32 = 0b10;

            /// 0b11: 9
            pub const COL_3: u32 = 0b11;
        }
    }

    /// CAS Latency
    pub mod CL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 1
            pub const CL_0: u32 = 0b00;

            /// 0b01: 1
            pub const CL_1: u32 = 0b01;

            /// 0b10: 2
            pub const CL_2: u32 = 0b10;

            /// 0b11: 3
            pub const CL_3: u32 = 0b11;
        }
    }

    /// 2 Bank selection bit
    pub mod BANK2 {
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

            /// 0b0: SDRAM device has 4 banks.
            pub const BANK2_0: u32 = 0b0;

            /// 0b1: SDRAM device has 2 banks.
            pub const BANK2_1: u32 = 0b1;
        }
    }
}

/// SDRAM Control Register 1
pub mod SDRAMCR1 {

    /// PRECHARGE to ACTIVE/REFRESH command wait time
    pub mod PRE2ACT {
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

    /// ACTIVE to READ/WRITE delay
    pub mod ACT2RW {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REFRESH recovery time
    pub mod RFRC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WRITE recovery time
    pub mod WRC {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CKE off minimum time
    pub mod CKEOFF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ACTIVE to PRECHARGE minimum time
    pub mod ACT2PRE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SDRAM Control Register 2
pub mod SDRAMCR2 {

    /// SELF REFRESH recovery time
    pub mod SRRC {
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

    /// REFRESH to REFRESH delay
    pub mod REF2REF {
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

    /// ACTIVE to ACTIVE delay
    pub mod ACT2ACT {
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

    /// SDRAM idle timeout
    pub mod ITO {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: IDLE timeout period is 256*Prescale period.
            pub const ITO_0: u32 = 0b00000000;

            /// 0b00000001: IDLE timeout period is ITO*Prescale period.
            pub const ITO_1: u32 = 0b00000001;

            /// 0b00000010: IDLE timeout period is ITO*Prescale period.
            pub const ITO_2: u32 = 0b00000010;

            /// 0b00000011: IDLE timeout period is ITO*Prescale period.
            pub const ITO_3: u32 = 0b00000011;

            /// 0b00000100: IDLE timeout period is ITO*Prescale period.
            pub const ITO_4: u32 = 0b00000100;

            /// 0b00000101: IDLE timeout period is ITO*Prescale period.
            pub const ITO_5: u32 = 0b00000101;

            /// 0b00000110: IDLE timeout period is ITO*Prescale period.
            pub const ITO_6: u32 = 0b00000110;

            /// 0b00000111: IDLE timeout period is ITO*Prescale period.
            pub const ITO_7: u32 = 0b00000111;

            /// 0b00001000: IDLE timeout period is ITO*Prescale period.
            pub const ITO_8: u32 = 0b00001000;

            /// 0b00001001: IDLE timeout period is ITO*Prescale period.
            pub const ITO_9: u32 = 0b00001001;
        }
    }
}

/// SDRAM Control Register 3
pub mod SDRAMCR3 {

    /// Refresh enable
    pub mod REN {
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

            /// 0b0: The SEMC does not send AUTO REFRESH command automatically
            pub const REN_0: u32 = 0b0;

            /// 0b1: The SEMC sends AUTO REFRESH command automatically
            pub const REN_1: u32 = 0b1;
        }
    }

    /// Refresh burst length
    pub mod REBL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1
            pub const REBL_0: u32 = 0b000;

            /// 0b001: 2
            pub const REBL_1: u32 = 0b001;

            /// 0b010: 3
            pub const REBL_2: u32 = 0b010;

            /// 0b011: 4
            pub const REBL_3: u32 = 0b011;

            /// 0b100: 5
            pub const REBL_4: u32 = 0b100;

            /// 0b101: 6
            pub const REBL_5: u32 = 0b101;

            /// 0b110: 7
            pub const REBL_6: u32 = 0b110;

            /// 0b111: 8
            pub const REBL_7: u32 = 0b111;
        }
    }

    /// Prescaler period
    pub mod PRESCALE {
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

            /// 0b00000000: (256*16+1) clock cycles
            pub const PRESCALE_0: u32 = 0b00000000;

            /// 0b00000001: (PRESCALE*16+1) clock cycles
            pub const PRESCALE_1: u32 = 0b00000001;

            /// 0b00000010: (PRESCALE*16+1) clock cycles
            pub const PRESCALE_2: u32 = 0b00000010;

            /// 0b00000011: (PRESCALE*16+1) clock cycles
            pub const PRESCALE_3: u32 = 0b00000011;

            /// 0b00000100: (PRESCALE*16+1) clock cycles
            pub const PRESCALE_4: u32 = 0b00000100;

            /// 0b00000101: (PRESCALE*16+1) clock cycles
            pub const PRESCALE_5: u32 = 0b00000101;

            /// 0b00000110: (PRESCALE*16+1) clock cycles
            pub const PRESCALE_6: u32 = 0b00000110;

            /// 0b00000111: (PRESCALE*16+1) clock cycles
            pub const PRESCALE_7: u32 = 0b00000111;

            /// 0b00001000: (PRESCALE*16+1) clock cycles
            pub const PRESCALE_8: u32 = 0b00001000;

            /// 0b00001001: (PRESCALE*16+1) clock cycles
            pub const PRESCALE_9: u32 = 0b00001001;
        }
    }

    /// Refresh timer period
    pub mod RT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: (256+1)*(Prescaler period)
            pub const RT_0: u32 = 0b00000000;

            /// 0b00000001: (RT+1)*(Prescaler period)
            pub const RT_1: u32 = 0b00000001;

            /// 0b00000010: (RT+1)*(Prescaler period)
            pub const RT_2: u32 = 0b00000010;

            /// 0b00000011: (RT+1)*(Prescaler period)
            pub const RT_3: u32 = 0b00000011;

            /// 0b00000100: (RT+1)*(Prescaler period)
            pub const RT_4: u32 = 0b00000100;

            /// 0b00000101: (RT+1)*(Prescaler period)
            pub const RT_5: u32 = 0b00000101;

            /// 0b00000110: (RT+1)*(Prescaler period)
            pub const RT_6: u32 = 0b00000110;

            /// 0b00000111: (RT+1)*(Prescaler period)
            pub const RT_7: u32 = 0b00000111;

            /// 0b00001000: (RT+1)*(Prescaler period)
            pub const RT_8: u32 = 0b00001000;

            /// 0b00001001: (RT+1)*(Prescaler period)
            pub const RT_9: u32 = 0b00001001;
        }
    }

    /// Urgent refresh threshold
    pub mod UT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: 256*(Prescaler period)
            pub const UT_0: u32 = 0b00000000;

            /// 0b00000001: UT*(Prescaler period)
            pub const UT_1: u32 = 0b00000001;

            /// 0b00000010: UT*(Prescaler period)
            pub const UT_2: u32 = 0b00000010;

            /// 0b00000011: UT*(Prescaler period)
            pub const UT_3: u32 = 0b00000011;

            /// 0b00000100: UT*(Prescaler period)
            pub const UT_4: u32 = 0b00000100;

            /// 0b00000101: UT*(Prescaler period)
            pub const UT_5: u32 = 0b00000101;

            /// 0b00000110: UT*(Prescaler period)
            pub const UT_6: u32 = 0b00000110;

            /// 0b00000111: UT*(Prescaler period)
            pub const UT_7: u32 = 0b00000111;

            /// 0b00001000: UT*(Prescaler period)
            pub const UT_8: u32 = 0b00001000;

            /// 0b00001001: UT*(Prescaler period)
            pub const UT_9: u32 = 0b00001001;
        }
    }
}

/// NAND Control Register 0
pub mod NANDCR0 {

    /// Port Size
    pub mod PS {
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

            /// 0b0: 8bit
            pub const PS_0: u32 = 0b0;

            /// 0b1: 16bit
            pub const PS_1: u32 = 0b1;
        }
    }

    /// Synchronous Mode Enable
    pub mod SYNCEN {
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

            /// 0b0: Asynchronous mode is enabled.
            pub const SYNCEN_0: u32 = 0b0;

            /// 0b1: Synchronous mode is enabled.
            pub const SYNCEN_1: u32 = 0b1;
        }
    }

    /// Burst Length
    pub mod BL {
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

            /// 0b000: 1
            pub const BL_0: u32 = 0b000;

            /// 0b001: 2
            pub const BL_1: u32 = 0b001;

            /// 0b010: 4
            pub const BL_2: u32 = 0b010;

            /// 0b011: 8
            pub const BL_3: u32 = 0b011;

            /// 0b100: 16
            pub const BL_4: u32 = 0b100;

            /// 0b101: 32
            pub const BL_5: u32 = 0b101;

            /// 0b110: 64
            pub const BL_6: u32 = 0b110;

            /// 0b111: 64
            pub const BL_7: u32 = 0b111;
        }
    }

    /// EDO mode enabled
    pub mod EDO {
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

            /// 0b0: EDO mode disabled
            pub const EDO_0: u32 = 0b0;

            /// 0b1: EDO mode enabled
            pub const EDO_1: u32 = 0b1;
        }
    }

    /// Column address bit number
    pub mod COL {
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

            /// 0b000: 16
            pub const COL_0: u32 = 0b000;

            /// 0b001: 15
            pub const COL_1: u32 = 0b001;

            /// 0b010: 14
            pub const COL_2: u32 = 0b010;

            /// 0b011: 13
            pub const COL_3: u32 = 0b011;

            /// 0b100: 12
            pub const COL_4: u32 = 0b100;

            /// 0b101: 11
            pub const COL_5: u32 = 0b101;

            /// 0b110: 10
            pub const COL_6: u32 = 0b110;

            /// 0b111: 9
            pub const COL_7: u32 = 0b111;
        }
    }
}

/// NAND Control Register 1
pub mod NANDCR1 {

    /// CE# setup time
    pub mod CES {
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

    /// CE# hold time
    pub mod CEH {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WE# low time
    pub mod WEL {
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

    /// WE# high time
    pub mod WEH {
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

    /// RE# low time
    pub mod REL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RE# high time
    pub mod REH {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Turnaround time
    pub mod TA {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CE# interval time
    pub mod CEITV {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// NAND Control Register 2
pub mod NANDCR2 {

    /// WE# high to RE# low time
    pub mod TWHR {
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

    /// RE# high to WE# low time
    pub mod TRHW {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (6 bits: 0x3f << 6)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Address cycle to data loading time
    pub mod TADL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (6 bits: 0x3f << 12)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Ready to RE# low time
    pub mod TRR {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (6 bits: 0x3f << 18)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WE# high to busy time
    pub mod TWB {
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

/// NAND Control Register 3
pub mod NANDCR3 {

    /// NAND option bit 1
    pub mod NDOPT1 {
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

    /// NAND option bit 2
    pub mod NDOPT2 {
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

    /// NAND option bit 3
    pub mod NDOPT3 {
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

    /// NAND CLE Option
    pub mod CLE {
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

    /// Read Data Setup time
    pub mod RDS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read Data Hold time
    pub mod RDH {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Write Data Setup time
    pub mod WDS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Write Data Hold time
    pub mod WDH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// NOR Control Register 0
pub mod NORCR0 {

    /// Port Size
    pub mod PS {
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

            /// 0b0: 8bit
            pub const PS_0: u32 = 0b0;

            /// 0b1: 16bit
            pub const PS_1: u32 = 0b1;
        }
    }

    /// Synchronous Mode Enable
    pub mod SYNCEN {
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

            /// 0b0: Asynchronous mode is enabled.
            pub const SYNCEN_0: u32 = 0b0;

            /// 0b1: Synchronous mode is enabled. Only fixed latency mode is supported.
            pub const SYNCEN_1: u32 = 0b1;
        }
    }

    /// Burst Length
    pub mod BL {
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

            /// 0b000: 1
            pub const BL_0: u32 = 0b000;

            /// 0b001: 2
            pub const BL_1: u32 = 0b001;

            /// 0b010: 4
            pub const BL_2: u32 = 0b010;

            /// 0b011: 8
            pub const BL_3: u32 = 0b011;

            /// 0b100: 16
            pub const BL_4: u32 = 0b100;

            /// 0b101: 32
            pub const BL_5: u32 = 0b101;

            /// 0b110: 64
            pub const BL_6: u32 = 0b110;

            /// 0b111: 64
            pub const BL_7: u32 = 0b111;
        }
    }

    /// Address Mode
    pub mod AM {
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

            /// 0b00: Address/Data MUX mode (ADMUX)
            pub const AM_0: u32 = 0b00;

            /// 0b01: Advanced Address/Data MUX mode (AADM)
            pub const AM_1: u32 = 0b01;

            /// 0b10: Address/Data non-MUX mode (Non-ADMUX)
            pub const AM_2: u32 = 0b10;

            /// 0b11: Address/Data non-MUX mode (Non-ADMUX)
            pub const AM_3: u32 = 0b11;
        }
    }

    /// ADV# Polarity
    pub mod ADVP {
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

            /// 0b0: ADV# is active low.
            pub const ADVP_0: u32 = 0b0;

            /// 0b1: ADV# is active high.
            pub const ADVP_1: u32 = 0b1;
        }
    }

    /// ADV# level control during address hold state
    pub mod ADVH {
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

            /// 0b0: ADV# is high during address hold state.
            pub const ADVH_0: u32 = 0b0;

            /// 0b1: ADV# is low during address hold state.
            pub const ADVH_1: u32 = 0b1;
        }
    }

    /// Column Address bit width
    pub mod COL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 12 Bits
            pub const COL_0: u32 = 0b0000;

            /// 0b0001: 11 Bits
            pub const COL_1: u32 = 0b0001;

            /// 0b0010: 10 Bits
            pub const COL_2: u32 = 0b0010;

            /// 0b0011: 9 Bits
            pub const COL_3: u32 = 0b0011;

            /// 0b0100: 8 Bits
            pub const COL_4: u32 = 0b0100;

            /// 0b0101: 7 Bits
            pub const COL_5: u32 = 0b0101;

            /// 0b0110: 6 Bits
            pub const COL_6: u32 = 0b0110;

            /// 0b0111: 5 Bits
            pub const COL_7: u32 = 0b0111;

            /// 0b1000: 4 Bits
            pub const COL_8: u32 = 0b1000;

            /// 0b1001: 3 Bits
            pub const COL_9: u32 = 0b1001;

            /// 0b1010: 2 Bits
            pub const COL_10: u32 = 0b1010;

            /// 0b1011: 12 Bits
            pub const COL_11: u32 = 0b1011;

            /// 0b1100: 12 Bits
            pub const COL_12: u32 = 0b1100;

            /// 0b1101: 12 Bits
            pub const COL_13: u32 = 0b1101;

            /// 0b1110: 12 Bits
            pub const COL_14: u32 = 0b1110;

            /// 0b1111: 12 Bits
            pub const COL_15: u32 = 0b1111;
        }
    }
}

/// NOR Control Register 1
pub mod NORCR1 {

    /// CE setup time
    pub mod CES {
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

    /// CE hold time
    pub mod CEH {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Address setup time
    pub mod AS {
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

    /// Address hold time
    pub mod AH {
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

    /// WE low time
    pub mod WEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WE high time
    pub mod WEH {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RE low time
    pub mod REL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RE high time
    pub mod REH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// NOR Control Register 2
pub mod NORCR2 {

    /// Turnaround time
    pub mod TA {
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

    /// Address to write data hold time
    pub mod AWDH {
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

    /// Latency count
    pub mod LC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read time
    pub mod RD {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CE# interval time
    pub mod CEITV {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read hold time
    pub mod RDH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// NOR Control Register 3
pub mod NORCR3 {

    /// Address setup time for SYNC read
    pub mod ASSR {
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

    /// Address hold time for SYNC read
    pub mod AHSR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SRAM Control Register 0
pub mod SRAMCR0 {

    /// Port Size
    pub mod PS {
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

            /// 0b0: 8bit
            pub const PS_0: u32 = 0b0;

            /// 0b1: 16bit
            pub const PS_1: u32 = 0b1;
        }
    }

    /// Synchronous Mode Enable
    pub mod SYNCEN {
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

            /// 0b0: Asynchronous mode is enabled.
            pub const SYNCEN_0: u32 = 0b0;

            /// 0b1: Synchronous mode is enabled. Only fixed latency mode is supported.
            pub const SYNCEN_1: u32 = 0b1;
        }
    }

    /// Wait Enable
    pub mod WAITEN {
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

            /// 0b0: The SEMC does not monitor wait pin.
            pub const WAITEN_0: u32 = 0b0;

            /// 0b1: The SEMC monitors wait pin. The SEMC does not transfer/receive data when wait pin is asserted.
            pub const WAITEN_1: u32 = 0b1;
        }
    }

    /// Wait Sample
    pub mod WAITSP {
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

            /// 0b0: Wait pin is directly used by the SEMC.
            pub const WAITSP_0: u32 = 0b0;

            /// 0b1: Wait pin is sampled by internal clock before it is used.
            pub const WAITSP_1: u32 = 0b1;
        }
    }

    /// Burst Length
    pub mod BL {
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

            /// 0b000: 1
            pub const BL_0: u32 = 0b000;

            /// 0b001: 2
            pub const BL_1: u32 = 0b001;

            /// 0b010: 4
            pub const BL_2: u32 = 0b010;

            /// 0b011: 8
            pub const BL_3: u32 = 0b011;

            /// 0b100: 16
            pub const BL_4: u32 = 0b100;

            /// 0b101: 32
            pub const BL_5: u32 = 0b101;

            /// 0b110: 64
            pub const BL_6: u32 = 0b110;

            /// 0b111: 64
            pub const BL_7: u32 = 0b111;
        }
    }

    /// Address Mode
    pub mod AM {
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

            /// 0b00: Address/Data MUX mode (ADMUX)
            pub const AM_0: u32 = 0b00;

            /// 0b01: Advanced Address/Data MUX mode (AADM)
            pub const AM_1: u32 = 0b01;

            /// 0b10: Address/Data non-MUX mode (Non-ADMUX)
            pub const AM_2: u32 = 0b10;

            /// 0b11: Address/Data non-MUX mode (Non-ADMUX)
            pub const AM_3: u32 = 0b11;
        }
    }

    /// ADV# polarity
    pub mod ADVP {
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

            /// 0b0: ADV# is active low.
            pub const ADVP_0: u32 = 0b0;

            /// 0b1: ADV# is active high.
            pub const ADVP_1: u32 = 0b1;
        }
    }

    /// ADV# level control during address hold state
    pub mod ADVH {
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

            /// 0b0: ADV# is high during address hold state.
            pub const ADVH_0: u32 = 0b0;

            /// 0b1: ADV# is low during address hold state.
            pub const ADVH_1: u32 = 0b1;
        }
    }

    /// Column Address bit width
    pub mod COL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 12 Bits
            pub const COL_0: u32 = 0b0000;

            /// 0b0001: 11 Bits
            pub const COL_1: u32 = 0b0001;

            /// 0b0010: 10 Bits
            pub const COL_2: u32 = 0b0010;

            /// 0b0011: 9 Bits
            pub const COL_3: u32 = 0b0011;

            /// 0b0100: 8 Bits
            pub const COL_4: u32 = 0b0100;

            /// 0b0101: 7 Bits
            pub const COL_5: u32 = 0b0101;

            /// 0b0110: 6 Bits
            pub const COL_6: u32 = 0b0110;

            /// 0b0111: 5 Bits
            pub const COL_7: u32 = 0b0111;

            /// 0b1000: 4 Bits
            pub const COL_8: u32 = 0b1000;

            /// 0b1001: 3 Bits
            pub const COL_9: u32 = 0b1001;

            /// 0b1010: 2 Bits
            pub const COL_10: u32 = 0b1010;

            /// 0b1011: 12 Bits
            pub const COL_11: u32 = 0b1011;

            /// 0b1100: 12 Bits
            pub const COL_12: u32 = 0b1100;

            /// 0b1101: 12 Bits
            pub const COL_13: u32 = 0b1101;

            /// 0b1110: 12 Bits
            pub const COL_14: u32 = 0b1110;

            /// 0b1111: 12 Bits
            pub const COL_15: u32 = 0b1111;
        }
    }
}

/// SRAM Control Register 1
pub mod SRAMCR1 {
    pub use super::NORCR1::AH;
    pub use super::NORCR1::AS;
    pub use super::NORCR1::CEH;
    pub use super::NORCR1::CES;
    pub use super::NORCR1::REH;
    pub use super::NORCR1::REL;
    pub use super::NORCR1::WEH;
    pub use super::NORCR1::WEL;
}

/// SRAM Control Register 2
pub mod SRAMCR2 {

    /// Write Data setup time
    pub mod WDS {
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

    /// Write Data hold time
    pub mod WDH {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Turnaround time
    pub mod TA {
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

    /// Address to write data hold time
    pub mod AWDH {
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

    /// Latency count
    pub mod LC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read time
    pub mod RD {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CE# interval time
    pub mod CEITV {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read hold time
    pub mod RDH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SRAM Control Register 3
pub mod SRAMCR3 {}

/// DBI-B Control Register 0
pub mod DBICR0 {

    /// Port Size
    pub mod PS {
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

            /// 0b0: 8bit
            pub const PS_0: u32 = 0b0;

            /// 0b1: 16bit
            pub const PS_1: u32 = 0b1;
        }
    }

    /// Burst Length
    pub mod BL {
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

            /// 0b000: 1
            pub const BL_0: u32 = 0b000;

            /// 0b001: 2
            pub const BL_1: u32 = 0b001;

            /// 0b010: 4
            pub const BL_2: u32 = 0b010;

            /// 0b011: 8
            pub const BL_3: u32 = 0b011;

            /// 0b100: 16
            pub const BL_4: u32 = 0b100;

            /// 0b101: 32
            pub const BL_5: u32 = 0b101;

            /// 0b110: 64
            pub const BL_6: u32 = 0b110;

            /// 0b111: 64
            pub const BL_7: u32 = 0b111;
        }
    }

    /// Column Address bit width
    pub mod COL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 12 Bits
            pub const COL_0: u32 = 0b0000;

            /// 0b0001: 11 Bits
            pub const COL_1: u32 = 0b0001;

            /// 0b0010: 10 Bits
            pub const COL_2: u32 = 0b0010;

            /// 0b0011: 9 Bits
            pub const COL_3: u32 = 0b0011;

            /// 0b0100: 8 Bits
            pub const COL_4: u32 = 0b0100;

            /// 0b0101: 7 Bits
            pub const COL_5: u32 = 0b0101;

            /// 0b0110: 6 Bits
            pub const COL_6: u32 = 0b0110;

            /// 0b0111: 5 Bits
            pub const COL_7: u32 = 0b0111;

            /// 0b1000: 4 Bits
            pub const COL_8: u32 = 0b1000;

            /// 0b1001: 3 Bits
            pub const COL_9: u32 = 0b1001;

            /// 0b1010: 2 Bits
            pub const COL_10: u32 = 0b1010;

            /// 0b1011: 12 Bits
            pub const COL_11: u32 = 0b1011;

            /// 0b1100: 12 Bits
            pub const COL_12: u32 = 0b1100;

            /// 0b1101: 12 Bits
            pub const COL_13: u32 = 0b1101;

            /// 0b1110: 12 Bits
            pub const COL_14: u32 = 0b1110;

            /// 0b1111: 12 Bits
            pub const COL_15: u32 = 0b1111;
        }
    }
}

/// DBI-B Control Register 1
pub mod DBICR1 {

    /// CSX Setup Time
    pub mod CES {
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

    /// CSX Hold Time
    pub mod CEH {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WRX Low Time
    pub mod WEL {
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

    /// WRX High Time
    pub mod WEH {
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

    /// RDX Low Time
    pub mod REL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RDX High Time
    pub mod REH {
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
}

/// DBI-B Control Register 2
pub mod DBICR2 {

    /// CSX interval time
    pub mod CEITV {
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

/// IP Command Control Register 0
pub mod IPCR0 {

    /// Slave address
    pub mod SA {
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

/// IP Command Control Register 1
pub mod IPCR1 {

    /// Data Size in Byte
    pub mod DATSZ {
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

            /// 0b000: 4
            pub const DATSZ_0: u32 = 0b000;

            /// 0b001: 1
            pub const DATSZ_1: u32 = 0b001;

            /// 0b010: 2
            pub const DATSZ_2: u32 = 0b010;

            /// 0b011: 3
            pub const DATSZ_3: u32 = 0b011;

            /// 0b100: 4
            pub const DATSZ_4: u32 = 0b100;

            /// 0b101: 4
            pub const DATSZ_5: u32 = 0b101;

            /// 0b110: 4
            pub const DATSZ_6: u32 = 0b110;

            /// 0b111: 4
            pub const DATSZ_7: u32 = 0b111;
        }
    }

    /// NAND Extended Address
    pub mod NAND_EXT_ADDR {
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
}

/// IP Command Control Register 2
pub mod IPCR2 {

    /// Byte Mask for Byte 0 (IPTXDAT bit 7:0)
    pub mod BM0 {
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

            /// 0b0: Byte is unmasked
            pub const BM0_0: u32 = 0b0;

            /// 0b1: Byte is masked
            pub const BM0_1: u32 = 0b1;
        }
    }

    /// Byte Mask for Byte 1 (IPTXDAT bit 15:8)
    pub mod BM1 {
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

            /// 0b0: Byte is unmasked
            pub const BM1_0: u32 = 0b0;

            /// 0b1: Byte is masked
            pub const BM1_1: u32 = 0b1;
        }
    }

    /// Byte Mask for Byte 2 (IPTXDAT bit 23:16)
    pub mod BM2 {
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

            /// 0b0: Byte is unmasked
            pub const BM2_0: u32 = 0b0;

            /// 0b1: Byte is masked
            pub const BM2_1: u32 = 0b1;
        }
    }

    /// Byte Mask for Byte 3 (IPTXDAT bit 31:24)
    pub mod BM3 {
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

            /// 0b0: Byte is unmasked
            pub const BM3_0: u32 = 0b0;

            /// 0b1: Byte is masked
            pub const BM3_1: u32 = 0b1;
        }
    }
}

/// IP Command Register
pub mod IPCMD {

    /// SDRAM Commands: 0x8: Read 0x9: Write 0xA: Mode Register Set 0xB: Active 0xC: Auto Refresh 0xD: Self Refresh 0xE: Precharge 0xF: Precharge All Others: Reserved Self Refresh is sent to all SDRAM devices because they share the same SEMC_CLK pin
    pub mod CMD {
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

    /// This field should be written with 0xA55A when trigging an IP command for all device types
    pub mod KEY {
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

/// TX DATA Register
pub mod IPTXDAT {

    /// Data value to use for an IP write command
    pub mod DAT {
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

/// RX DATA Register
pub mod IPRXDAT {

    /// Data returned by device for an IP read command.
    pub mod DAT {
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

/// Status Register 0
pub mod STS0 {

    /// Indicating whether the SEMC is in idle state.
    pub mod IDLE {
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

    /// Indicating NAND device Ready/WAIT# pin level.
    pub mod NARDY {
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

            /// 0b0: NAND device is not ready
            pub const NARDY_0: u32 = 0b0;

            /// 0b1: NAND device is ready
            pub const NARDY_1: u32 = 0b1;
        }
    }
}

/// Status Register 1
pub mod STS1 {}

/// Status Register 2
pub mod STS2 {

    /// This field indicating whether there is pending AXI command (write) to NAND device.
    pub mod NDWRPEND {
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

            /// 0b0: No pending
            pub const NDWRPEND_0: u32 = 0b0;

            /// 0b1: Pending
            pub const NDWRPEND_1: u32 = 0b1;
        }
    }
}

/// Status Register 3
pub mod STS3 {}

/// Status Register 4
pub mod STS4 {}

/// Status Register 5
pub mod STS5 {}

/// Status Register 6
pub mod STS6 {}

/// Status Register 7
pub mod STS7 {}

/// Status Register 8
pub mod STS8 {}

/// Status Register 9
pub mod STS9 {}

/// Status Register 10
pub mod STS10 {}

/// Status Register 11
pub mod STS11 {}

/// Status Register 12
pub mod STS12 {

    /// This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4).
    pub mod NDADDR {
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

/// Status Register 13
pub mod STS13 {

    /// Sample clock slave delay line locked.
    pub mod SLVLOCK {
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

            /// 0b0: Slave delay line is not locked.
            pub const SLVLOCK_0: u32 = 0b0;

            /// 0b1: Slave delay line is locked.
            pub const SLVLOCK_1: u32 = 0b1;
        }
    }

    /// Sample clock reference delay line locked.
    pub mod REFLOCK {
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

            /// 0b0: Reference delay line is not locked.
            pub const REFLOCK_0: u32 = 0b0;

            /// 0b1: Reference delay line is locked.
            pub const REFLOCK_1: u32 = 0b1;
        }
    }

    /// Sample clock slave delay line delay cell number selection.
    pub mod SLVSEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (6 bits: 0x3f << 2)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sample clock reference delay line delay cell number selection.
    pub mod REFSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status Register 14
pub mod STS14 {}

/// Status Register 15
pub mod STS15 {}

/// Base Register 9
pub mod BR9 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register 10
pub mod BR10 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register 11
pub mod BR11 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// SRAM Control Register 4
pub mod SRAMCR4 {
    pub use super::SRAMCR0::ADVH;
    pub use super::SRAMCR0::ADVP;
    pub use super::SRAMCR0::AM;
    pub use super::SRAMCR0::BL;
    pub use super::SRAMCR0::COL;
    pub use super::SRAMCR0::PS;
    pub use super::SRAMCR0::SYNCEN;
    pub use super::SRAMCR0::WAITEN;
    pub use super::SRAMCR0::WAITSP;
}

/// SRAM Control Register 5
pub mod SRAMCR5 {
    pub use super::NORCR1::AH;
    pub use super::NORCR1::AS;
    pub use super::NORCR1::CEH;
    pub use super::NORCR1::CES;
    pub use super::NORCR1::REH;
    pub use super::NORCR1::REL;
    pub use super::NORCR1::WEH;
    pub use super::NORCR1::WEL;
}

/// SRAM Control Register 6
pub mod SRAMCR6 {
    pub use super::SRAMCR2::AWDH;
    pub use super::SRAMCR2::CEITV;
    pub use super::SRAMCR2::LC;
    pub use super::SRAMCR2::RD;
    pub use super::SRAMCR2::RDH;
    pub use super::SRAMCR2::TA;
    pub use super::SRAMCR2::WDH;
    pub use super::SRAMCR2::WDS;
}

/// Delay Chain Control Register
pub mod DCCR {

    /// Delay chain insertion enable for SRAM device.
    pub mod SDRAMEN {
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

            /// 0b0: Delay chain is not inserted.
            pub const SDRAMEN_0: u32 = 0b0;

            /// 0b1: Delay chain is inserted.
            pub const SDRAMEN_1: u32 = 0b1;
        }
    }

    /// Clock delay line delay cell number selection value for SDRAM device.
    pub mod SDRAMVAL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (5 bits: 0b11111 << 1)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay chain insertion enable for NOR device.
    pub mod NOREN {
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

            /// 0b0: Delay chain is not inserted.
            pub const NOREN_0: u32 = 0b0;

            /// 0b1: Delay chain is inserted.
            pub const NOREN_1: u32 = 0b1;
        }
    }

    /// Clock delay line delay cell number selection value for NOR device.
    pub mod NORVAL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (5 bits: 0b11111 << 9)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay chain insertion enable for SRAM device 0.
    pub mod SRAM0EN {
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

            /// 0b0: Delay chain is not inserted.
            pub const SRAM0EN_0: u32 = 0b0;

            /// 0b1: Delay chain is inserted.
            pub const SRAM0EN_1: u32 = 0b1;
        }
    }

    /// Clock delay line delay cell number selection value for SRAM device 0.
    pub mod SRAM0VAL {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (5 bits: 0b11111 << 17)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay chain insertion enable for SRAM device 1-3.
    pub mod SRAMXEN {
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

            /// 0b0: Delay chain is not inserted.
            pub const SRAMXEN_0: u32 = 0b0;

            /// 0b1: Delay chain is inserted.
            pub const SRAMXEN_1: u32 = 0b1;
        }
    }

    /// Clock delay line delay cell number selection value for SRAM device 1-3.
    pub mod SRAMXVAL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (5 bits: 0b11111 << 25)
        pub const mask: u32 = 0b11111 << offset;
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
    /// Module Control Register
    pub MCR: RWRegister<u32>,

    /// IO MUX Control Register
    pub IOCR: RWRegister<u32>,

    /// Bus (AXI) Master Control Register 0
    pub BMCR0: RWRegister<u32>,

    /// Bus (AXI) Master Control Register 1
    pub BMCR1: RWRegister<u32>,

    /// Base Register n
    pub BR0: RWRegister<u32>,

    /// Base Register n
    pub BR1: RWRegister<u32>,

    /// Base Register n
    pub BR2: RWRegister<u32>,

    /// Base Register n
    pub BR3: RWRegister<u32>,

    /// Base Register n
    pub BR4: RWRegister<u32>,

    /// Base Register n
    pub BR5: RWRegister<u32>,

    /// Base Register n
    pub BR6: RWRegister<u32>,

    /// Base Register n
    pub BR7: RWRegister<u32>,

    /// Base Register n
    pub BR8: RWRegister<u32>,

    /// DLL Control Register
    pub DLLCR: RWRegister<u32>,

    /// Interrupt Enable Register
    pub INTEN: RWRegister<u32>,

    /// Interrupt Register
    pub INTR: RWRegister<u32>,

    /// SDRAM Control Register 0
    pub SDRAMCR0: RWRegister<u32>,

    /// SDRAM Control Register 1
    pub SDRAMCR1: RWRegister<u32>,

    /// SDRAM Control Register 2
    pub SDRAMCR2: RWRegister<u32>,

    /// SDRAM Control Register 3
    pub SDRAMCR3: RWRegister<u32>,

    /// NAND Control Register 0
    pub NANDCR0: RWRegister<u32>,

    /// NAND Control Register 1
    pub NANDCR1: RWRegister<u32>,

    /// NAND Control Register 2
    pub NANDCR2: RWRegister<u32>,

    /// NAND Control Register 3
    pub NANDCR3: RWRegister<u32>,

    /// NOR Control Register 0
    pub NORCR0: RWRegister<u32>,

    /// NOR Control Register 1
    pub NORCR1: RWRegister<u32>,

    /// NOR Control Register 2
    pub NORCR2: RWRegister<u32>,

    /// NOR Control Register 3
    pub NORCR3: RWRegister<u32>,

    /// SRAM Control Register 0
    pub SRAMCR0: RWRegister<u32>,

    /// SRAM Control Register 1
    pub SRAMCR1: RWRegister<u32>,

    /// SRAM Control Register 2
    pub SRAMCR2: RWRegister<u32>,

    /// SRAM Control Register 3
    pub SRAMCR3: RWRegister<u32>,

    /// DBI-B Control Register 0
    pub DBICR0: RWRegister<u32>,

    /// DBI-B Control Register 1
    pub DBICR1: RWRegister<u32>,

    /// DBI-B Control Register 2
    pub DBICR2: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// IP Command Control Register 0
    pub IPCR0: RWRegister<u32>,

    /// IP Command Control Register 1
    pub IPCR1: RWRegister<u32>,

    /// IP Command Control Register 2
    pub IPCR2: RWRegister<u32>,

    /// IP Command Register
    pub IPCMD: RWRegister<u32>,

    /// TX DATA Register
    pub IPTXDAT: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// RX DATA Register
    pub IPRXDAT: RORegister<u32>,

    _reserved3: [u32; 3],

    /// Status Register 0
    pub STS0: RORegister<u32>,

    /// Status Register 1
    pub STS1: RORegister<u32>,

    /// Status Register 2
    pub STS2: RORegister<u32>,

    /// Status Register 3
    pub STS3: RORegister<u32>,

    /// Status Register 4
    pub STS4: RORegister<u32>,

    /// Status Register 5
    pub STS5: RORegister<u32>,

    /// Status Register 6
    pub STS6: RORegister<u32>,

    /// Status Register 7
    pub STS7: RORegister<u32>,

    /// Status Register 8
    pub STS8: RORegister<u32>,

    /// Status Register 9
    pub STS9: RORegister<u32>,

    /// Status Register 10
    pub STS10: RORegister<u32>,

    /// Status Register 11
    pub STS11: RORegister<u32>,

    /// Status Register 12
    pub STS12: RORegister<u32>,

    /// Status Register 13
    pub STS13: RORegister<u32>,

    /// Status Register 14
    pub STS14: RORegister<u32>,

    /// Status Register 15
    pub STS15: RORegister<u32>,

    /// Base Register 9
    pub BR9: RWRegister<u32>,

    /// Base Register 10
    pub BR10: RWRegister<u32>,

    /// Base Register 11
    pub BR11: RWRegister<u32>,

    _reserved4: [u32; 5],

    /// SRAM Control Register 4
    pub SRAMCR4: RWRegister<u32>,

    /// SRAM Control Register 5
    pub SRAMCR5: RWRegister<u32>,

    /// SRAM Control Register 6
    pub SRAMCR6: RWRegister<u32>,

    _reserved5: [u32; 9],

    /// Delay Chain Control Register
    pub DCCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub IOCR: u32,
    pub BMCR0: u32,
    pub BMCR1: u32,
    pub BR0: u32,
    pub BR1: u32,
    pub BR2: u32,
    pub BR3: u32,
    pub BR4: u32,
    pub BR5: u32,
    pub BR6: u32,
    pub BR7: u32,
    pub BR8: u32,
    pub DLLCR: u32,
    pub INTEN: u32,
    pub INTR: u32,
    pub SDRAMCR0: u32,
    pub SDRAMCR1: u32,
    pub SDRAMCR2: u32,
    pub SDRAMCR3: u32,
    pub NANDCR0: u32,
    pub NANDCR1: u32,
    pub NANDCR2: u32,
    pub NANDCR3: u32,
    pub NORCR0: u32,
    pub NORCR1: u32,
    pub NORCR2: u32,
    pub NORCR3: u32,
    pub SRAMCR0: u32,
    pub SRAMCR1: u32,
    pub SRAMCR2: u32,
    pub SRAMCR3: u32,
    pub DBICR0: u32,
    pub DBICR1: u32,
    pub DBICR2: u32,
    pub IPCR0: u32,
    pub IPCR1: u32,
    pub IPCR2: u32,
    pub IPCMD: u32,
    pub IPTXDAT: u32,
    pub IPRXDAT: u32,
    pub STS0: u32,
    pub STS1: u32,
    pub STS2: u32,
    pub STS3: u32,
    pub STS4: u32,
    pub STS5: u32,
    pub STS6: u32,
    pub STS7: u32,
    pub STS8: u32,
    pub STS9: u32,
    pub STS10: u32,
    pub STS11: u32,
    pub STS12: u32,
    pub STS13: u32,
    pub STS14: u32,
    pub STS15: u32,
    pub BR9: u32,
    pub BR10: u32,
    pub BR11: u32,
    pub SRAMCR4: u32,
    pub SRAMCR5: u32,
    pub SRAMCR6: u32,
    pub DCCR: u32,
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
