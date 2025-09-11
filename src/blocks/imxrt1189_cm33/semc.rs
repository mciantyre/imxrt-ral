#[doc = "SEMC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Control Register"]
    pub MCR: u32,
    #[doc = "IO MUX Control Register"]
    pub IOCR: u32,
    #[doc = "Bus (AXI) Master Control Register 0"]
    pub BMCR0: u32,
    #[doc = "Bus (AXI) Master Control Register 1"]
    pub BMCR1: u32,
    #[doc = "Base Register n"]
    pub BR: [u32; 9usize],
    #[doc = "DLL Control Register"]
    pub DLLCR: u32,
    #[doc = "Interrupt Enable Register"]
    pub INTEN: u32,
    #[doc = "Interrupt Register"]
    pub INTR: u32,
    #[doc = "SDRAM Control Register 0"]
    pub SDRAMCR0: u32,
    #[doc = "SDRAM Control Register 1"]
    pub SDRAMCR1: u32,
    #[doc = "SDRAM Control Register 2"]
    pub SDRAMCR2: u32,
    #[doc = "SDRAM Control Register 3"]
    pub SDRAMCR3: u32,
    #[doc = "NAND Control Register 0"]
    pub NANDCR0: u32,
    #[doc = "NAND Control Register 1"]
    pub NANDCR1: u32,
    #[doc = "NAND Control Register 2"]
    pub NANDCR2: u32,
    #[doc = "NAND Control Register 3"]
    pub NANDCR3: u32,
    #[doc = "NOR Control Register 0"]
    pub NORCR0: u32,
    #[doc = "NOR Control Register 1"]
    pub NORCR1: u32,
    #[doc = "NOR Control Register 2"]
    pub NORCR2: u32,
    #[doc = "NOR Control Register 3"]
    pub NORCR3: u32,
    #[doc = "SRAM Control Register 0"]
    pub SRAMCR0: u32,
    #[doc = "SRAM Control Register 1"]
    pub SRAMCR1: u32,
    #[doc = "SRAM Control Register 2"]
    pub SRAMCR2: u32,
    #[doc = "SRAM Control Register 3"]
    pub SRAMCR3: u32,
    #[doc = "DBI-B Control Register 0"]
    pub DBICR0: u32,
    #[doc = "DBI-B Control Register 1"]
    pub DBICR1: u32,
    #[doc = "DBI-B Control Register 2"]
    pub DBICR2: u32,
    _reserved0: [u8; 0x04],
    #[doc = "IP Command Control Register 0"]
    pub IPCR0: u32,
    #[doc = "IP Command Control Register 1"]
    pub IPCR1: u32,
    #[doc = "IP Command Control Register 2"]
    pub IPCR2: u32,
    #[doc = "IP Command Register"]
    pub IPCMD: u32,
    #[doc = "TX DATA Register"]
    pub IPTXDAT: u32,
    _reserved1: [u8; 0x0c],
    #[doc = "RX DATA Register"]
    pub IPRXDAT: u32,
    _reserved2: [u8; 0x0c],
    #[doc = "Status Register 0"]
    pub STS0: u32,
    #[doc = "Status Register 1"]
    pub STS1: u32,
    #[doc = "Status Register 2"]
    pub STS2: u32,
    #[doc = "Status Register 3"]
    pub STS3: u32,
    #[doc = "Status Register 4"]
    pub STS4: u32,
    #[doc = "Status Register 5"]
    pub STS5: u32,
    #[doc = "Status Register 6"]
    pub STS6: u32,
    #[doc = "Status Register 7"]
    pub STS7: u32,
    #[doc = "Status Register 8"]
    pub STS8: u32,
    #[doc = "Status Register 9"]
    pub STS9: u32,
    #[doc = "Status Register 10"]
    pub STS10: u32,
    #[doc = "Status Register 11"]
    pub STS11: u32,
    #[doc = "Status Register 12"]
    pub STS12: u32,
    #[doc = "Status Register 13"]
    pub STS13: u32,
    #[doc = "Status Register 14"]
    pub STS14: u32,
    #[doc = "Status Register 15"]
    pub STS15: u32,
    #[doc = "Base Register 9"]
    pub BR9: u32,
    #[doc = "Base Register 10"]
    pub BR10: u32,
    #[doc = "Base Register 11"]
    pub BR11: u32,
    _reserved3: [u8; 0x14],
    #[doc = "SRAM Control Register 4"]
    pub SRAMCR4: u32,
    #[doc = "SRAM Control Register 5"]
    pub SRAMCR5: u32,
    #[doc = "SRAM Control Register 6"]
    pub SRAMCR6: u32,
    _reserved4: [u8; 0x14],
    #[doc = "NAND Buffer DATA Register"]
    pub NDBD: u32,
    #[doc = "NAND Buffer Address Register"]
    pub NDBA: u32,
    _reserved5: [u8; 0x08],
    #[doc = "Delay Chain Control Register"]
    pub DCCR: u32,
    #[doc = "SDRAM Prefetch Control Register"]
    pub SDRAMPCR: u32,
}
#[doc = "Module Control Register"]
pub mod MCR {
    pub use crate::RW as access;
    #[doc = "Software Reset"]
    pub mod SWRST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No reset"]
            pub const SWRST_0: u32 = 0;
            #[doc = "Reset"]
            pub const SWRST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Module Disable"]
    pub mod MDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Module enabled"]
            pub const MDIS_0: u32 = 0;
            #[doc = "Master disabled."]
            pub const MDIS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DQS (read strobe) mode"]
    pub mod DQSMD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Dummy read strobe loopbacked internally"]
            pub const DQSMD_0: u32 = 0;
            #[doc = "Dummy read strobe loopbacked from DQS pad"]
            pub const DQSMD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WAIT/RDY polarity for SRAM/NOR"]
    pub mod WPOL0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Low active"]
            pub const WPOL0_0: u32 = 0;
            #[doc = "High active"]
            pub const WPOL0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "R/B# polarity for NAND device"]
    pub mod WPOL1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Low active"]
            pub const WPOL1_0: u32 = 0;
            #[doc = "High active"]
            pub const WPOL1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command Execution timeout cycles"]
    pub mod CTO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bus timeout cycles"]
    pub mod BTO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "255*1"]
            pub const BTO_0: u32 = 0;
            #[doc = "255*2"]
            pub const BTO_1: u32 = 0x01;
            #[doc = "255*231"]
            pub const BTO_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IO MUX Control Register"]
pub mod IOCR {
    pub use crate::RW as access;
    #[doc = "SEMC_ADDR08 output selection"]
    pub mod MUX_A8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const MUX_A8_0: u32 = 0;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const MUX_A8_1: u32 = 0x01;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const MUX_A8_2: u32 = 0x02;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const MUX_A8_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const MUX_A8_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_A8_5: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const MUX_A8_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_A8_7: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const MUX_A8_8: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const MUX_A8_9: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const MUX_A8_10: u32 = 0x0a;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const MUX_A8_11: u32 = 0x0b;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const MUX_A8_12: u32 = 0x0c;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const MUX_A8_13: u32 = 0x0d;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const MUX_A8_14: u32 = 0x0e;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const MUX_A8_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEMC_CSX0 output selection"]
    pub mod MUX_CSX0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
            pub const MUX_CSX0_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_CSX0_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_CSX0_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_CSX0_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const MUX_CSX0_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_CSX0_5: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const MUX_CSX0_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_CSX0_7: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const MUX_CSX0_8: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const MUX_CSX0_9: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const MUX_CSX0_10: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 24 (A24)"]
            pub const MUX_CSX0_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 24 (A24)"]
            pub const MUX_CSX0_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 24 (A24)"]
            pub const MUX_CSX0_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 24 (A24)"]
            pub const MUX_CSX0_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 24 (A24)"]
            pub const MUX_CSX0_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEMC_CSX1 output selection"]
    pub mod MUX_CSX1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
            pub const MUX_CSX1_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_CSX1_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_CSX1_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_CSX1_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const MUX_CSX1_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_CSX1_5: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const MUX_CSX1_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_CSX1_7: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const MUX_CSX1_8: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const MUX_CSX1_9: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const MUX_CSX1_10: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 25 (A25)"]
            pub const MUX_CSX1_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 25 (A25)"]
            pub const MUX_CSX1_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 25 (A25)"]
            pub const MUX_CSX1_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 25 (A25)"]
            pub const MUX_CSX1_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 25 (A25)"]
            pub const MUX_CSX1_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEMC_CSX2 output selection"]
    pub mod MUX_CSX2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
            pub const MUX_CSX2_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_CSX2_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_CSX2_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_CSX2_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const MUX_CSX2_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_CSX2_5: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const MUX_CSX2_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_CSX2_7: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const MUX_CSX2_8: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const MUX_CSX2_9: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const MUX_CSX2_10: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 26 (A26)"]
            pub const MUX_CSX2_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 26 (A26)"]
            pub const MUX_CSX2_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 26 (A26)"]
            pub const MUX_CSX2_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 26 (A26)"]
            pub const MUX_CSX2_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 26 (A26)"]
            pub const MUX_CSX2_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEMC_CSX3 output selection"]
    pub mod MUX_CSX3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
            pub const MUX_CSX3_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_CSX3_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_CSX3_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_CSX3_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const MUX_CSX3_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_CSX3_5: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const MUX_CSX3_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_CSX3_7: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const MUX_CSX3_8: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const MUX_CSX3_9: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const MUX_CSX3_10: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 27 (A27)"]
            pub const MUX_CSX3_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 27 (A27)"]
            pub const MUX_CSX3_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 27 (A27)"]
            pub const MUX_CSX3_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 27 (A27)"]
            pub const MUX_CSX3_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 27 (A27)"]
            pub const MUX_CSX3_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEMC_RDY function selection"]
    pub mod MUX_RDY {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "NAND R/B# input"]
            pub const MUX_RDY_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const MUX_RDY_1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const MUX_RDY_2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const MUX_RDY_3: u32 = 0x03;
            #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
            pub const MUX_RDY_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const MUX_RDY_5: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const MUX_RDY_6: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const MUX_RDY_7: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const MUX_RDY_8: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const MUX_RDY_9: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const MUX_RDY_10: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 27"]
            pub const MUX_RDY_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 27"]
            pub const MUX_RDY_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 27"]
            pub const MUX_RDY_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 27"]
            pub const MUX_RDY_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 27"]
            pub const MUX_RDY_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEMC_CLKX0 function selection"]
    pub mod MUX_CLKX0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Keep low"]
            pub const MUX_CLKX0_0: u32 = 0;
            #[doc = "NOR clock"]
            pub const MUX_CLKX0_1: u32 = 0x01;
            #[doc = "SRAM clock"]
            pub const MUX_CLKX0_2: u32 = 0x02;
            #[doc = "NOR and SRAM clock, suitable for Multi-Chip Product package"]
            pub const MUX_CLKX0_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEMC_CLKX1 function selection"]
    pub mod MUX_CLKX1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Keep low"]
            pub const MUX_CLKX1_0: u32 = 0;
            #[doc = "NOR clock"]
            pub const MUX_CLKX1_1: u32 = 0x01;
            #[doc = "SRAM clock"]
            pub const MUX_CLKX1_2: u32 = 0x02;
            #[doc = "NOR and SRAM clock, suitable for Multi-Chip Product package"]
            pub const MUX_CLKX1_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEMC_CLKX0 Always On"]
    pub mod CLKX0_AO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SEMC_CLKX0 is controlled by MUX_CLKX0"]
            pub const CLKX0_AO_0: u32 = 0;
            #[doc = "SEMC_CLKX0 is always on"]
            pub const CLKX0_AO_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEMC_CLKX1 Always On"]
    pub mod CLKX1_AO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SEMC_CLKX1 is controlled by MUX_CLKX1"]
            pub const CLKX1_AO_0: u32 = 0;
            #[doc = "SEMC_CLKX1 is always on"]
            pub const CLKX1_AO_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Bus (AXI) Master Control Register 0"]
pub mod BMCR0 {
    pub use crate::RW as access;
    #[doc = "Weight of QoS"]
    pub mod WQOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Weight of Aging"]
    pub mod WAGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Weight of Slave Hit (no read/write switch)"]
    pub mod WSH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Weight of Slave Hit (Read/Write switch)"]
    pub mod WRWS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Bus (AXI) Master Control Register 1"]
pub mod BMCR1 {
    pub use crate::RW as access;
    #[doc = "Weight of QoS"]
    pub mod WQOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Weight of Aging"]
    pub mod WAGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Weight of Page Hit"]
    pub mod WPH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Weight of Read/Write switch"]
    pub mod WRWS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Weight of Bank Rotation"]
    pub mod WBR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Base Register n"]
pub mod BR {
    pub use crate::RW as access;
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The memory is invalid, can not be accessed."]
            pub const VLD_0: u32 = 0;
            #[doc = "The memory is valid, can be accessed."]
            pub const VLD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DLL Control Register"]
pub mod DLLCR {
    pub use crate::RW as access;
    #[doc = "DLL calibration enable"]
    pub mod DLLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DLL calibration is disabled."]
            pub const DLLEN_0: u32 = 0;
            #[doc = "DLL calibration is enabled."]
            pub const DLLEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DLL Reset"]
    pub mod DLLRESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DLL is not reset."]
            pub const DLLRESET_0: u32 = 0;
            #[doc = "DLL is reset."]
            pub const DLLRESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Delay Target for Slave"]
    pub mod SLVDLYTARGET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Override Enable"]
    pub mod OVRDEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The delay cell number is not overridden."]
            pub const OVRDEN_0: u32 = 0;
            #[doc = "The delay cell number is overridden."]
            pub const OVRDEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Override Value"]
    pub mod OVRDVAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INTEN {
    pub use crate::RW as access;
    #[doc = "IP command done interrupt enable"]
    pub mod IPCMDDONEEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const IPCMDDONEEN_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const IPCMDDONEEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IP command error interrupt enable"]
    pub mod IPCMDERREN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const IPCMDERREN_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const IPCMDERREN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AXI command error interrupt enable"]
    pub mod AXICMDERREN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const AXICMDERREN_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const AXICMDERREN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AXI bus error interrupt enable"]
    pub mod AXIBUSERREN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const AXIBUSERREN_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const AXIBUSERREN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND page end interrupt enable"]
    pub mod NDPAGEENDEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const NDPAGEENDEN_0: u32 = 0;
            #[doc = "Enable"]
            pub const NDPAGEENDEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND no pending AXI access interrupt enable"]
    pub mod NDNOPENDEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const NDNOPENDEN_0: u32 = 0;
            #[doc = "Enable"]
            pub const NDNOPENDEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND ECC fail interrupt enable"]
    pub mod NDECCFAILEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND buffer end interrupt enable"]
    pub mod NDBUFENDEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt Register"]
pub mod INTR {
    pub use crate::RW as access;
    #[doc = "IP command normal done interrupt"]
    pub mod IPCMDDONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "IP command is not done."]
            pub const IPCMDDONE_0: u32 = 0;
            #[doc = "IP command is done."]
            pub const IPCMDDONE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IP command error done interrupt"]
    pub mod IPCMDERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No IP command error."]
            pub const IPCMDERR_0: u32 = 0;
            #[doc = "IP command error occurs."]
            pub const IPCMDERR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AXI command error interrupt"]
    pub mod AXICMDERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No AXI command error."]
            pub const AXICMDERR_0: u32 = 0;
            #[doc = "AXI command error occurs."]
            pub const AXICMDERR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AXI bus error interrupt"]
    pub mod AXIBUSERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No AXI bus error."]
            pub const AXIBUSERR_0: u32 = 0;
            #[doc = "AXI bus error occurs."]
            pub const AXIBUSERR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND page end interrupt"]
    pub mod NDPAGEEND {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The last address of main space in the NAND is not written by AXI command."]
            pub const NDPAGEEND_0: u32 = 0;
            #[doc = "The last address of main space in the NAND is written by AXI command."]
            pub const NDPAGEEND_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND no pending AXI write transaction interrupt"]
    pub mod NDNOPEND {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "At least one NAND AXI write transaction is pending or no NAND write transaction is sent to the queue."]
            pub const NDNOPEND_0: u32 = 0;
            #[doc = "All NAND AXI write pending transactions are finished."]
            pub const NDNOPEND_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND ECC fail interrupt"]
    pub mod NDECCFAIL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "NAND ECC data correction pass."]
            pub const CORRECTION_PASS: u32 = 0;
            #[doc = "NAND ECC data correction fail."]
            pub const CORRECTION_FAIL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND buffer end interrupt"]
    pub mod NDBUFEND {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Last valid address of NAND buffer is not written by AXI command."]
            pub const NO_WRITE: u32 = 0;
            #[doc = "Last valid address of NAND buffer is written by AXI command."]
            pub const WRITE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SDRAM Control Register 0"]
pub mod SDRAMCR0 {
    pub use crate::RW as access;
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
            #[doc = "32bit"]
            pub const PS_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "8"]
            pub const BL_4: u32 = 0x04;
            #[doc = "8"]
            pub const BL_5: u32 = 0x05;
            #[doc = "8"]
            pub const BL_6: u32 = 0x06;
            #[doc = "8"]
            pub const BL_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Column 8 selection"]
    pub mod COL8 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Column address bit number is decided by COL field."]
            pub const COL8_0: u32 = 0;
            #[doc = "Column address bit number is 8. COL field is ignored."]
            pub const COL8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Column address bit number"]
    pub mod COL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "12 bit"]
            pub const COL_0: u32 = 0;
            #[doc = "11 bit"]
            pub const COL_1: u32 = 0x01;
            #[doc = "10 bit"]
            pub const COL_2: u32 = 0x02;
            #[doc = "9 bit"]
            pub const COL_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CAS Latency"]
    pub mod CL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1"]
            pub const CL_0: u32 = 0;
            #[doc = "1"]
            pub const CL_1: u32 = 0x01;
            #[doc = "2"]
            pub const CL_2: u32 = 0x02;
            #[doc = "3"]
            pub const CL_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "2 Bank selection bit"]
    pub mod BANK2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SDRAM device has 4 banks."]
            pub const BANK2_0: u32 = 0;
            #[doc = "SDRAM device has 2 banks."]
            pub const BANK2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SDRAM Control Register 1"]
pub mod SDRAMCR1 {
    pub use crate::RW as access;
    #[doc = "PRECHARGE to ACT/Refresh wait time"]
    pub mod PRE2ACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ACT to Read/Write wait time"]
    pub mod ACT2RW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Refresh recovery time"]
    pub mod RFRC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write recovery time"]
    pub mod WRC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CKE OFF minimum time"]
    pub mod CKEOFF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ACT to Precharge minimum time"]
    pub mod ACT2PRE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SDRAM Control Register 2"]
pub mod SDRAMCR2 {
    pub use crate::RW as access;
    #[doc = "Self Refresh Recovery time"]
    pub mod SRRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Refresh to Refresh wait time"]
    pub mod REF2REF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ACT to ACT wait time"]
    pub mod ACT2ACT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SDRAM Idle timeout"]
    pub mod ITO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "IDLE timeout period is 256*Prescale period."]
            pub const ITO_0: u32 = 0;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_1: u32 = 0x01;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_2: u32 = 0x02;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_3: u32 = 0x03;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_4: u32 = 0x04;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_5: u32 = 0x05;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_6: u32 = 0x06;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_7: u32 = 0x07;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_8: u32 = 0x08;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const ITO_9: u32 = 0x09;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SDRAM Control Register 3"]
pub mod SDRAMCR3 {
    pub use crate::RW as access;
    #[doc = "Refresh enable"]
    pub mod REN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The SEMC does not send AUTO REFRESH command automatically"]
            pub const REN_0: u32 = 0;
            #[doc = "The SEMC sends AUTO REFRESH command automatically"]
            pub const REN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Refresh burst length"]
    pub mod REBL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1"]
            pub const REBL_0: u32 = 0;
            #[doc = "2"]
            pub const REBL_1: u32 = 0x01;
            #[doc = "3"]
            pub const REBL_2: u32 = 0x02;
            #[doc = "4"]
            pub const REBL_3: u32 = 0x03;
            #[doc = "5"]
            pub const REBL_4: u32 = 0x04;
            #[doc = "6"]
            pub const REBL_5: u32 = 0x05;
            #[doc = "7"]
            pub const REBL_6: u32 = 0x06;
            #[doc = "8"]
            pub const REBL_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Prescaler period"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "256*16 cycle"]
            pub const PRESCALE_0: u32 = 0;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_1: u32 = 0x01;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_2: u32 = 0x02;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_3: u32 = 0x03;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_4: u32 = 0x04;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_5: u32 = 0x05;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_6: u32 = 0x06;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_7: u32 = 0x07;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_8: u32 = 0x08;
            #[doc = "PRESCALE*16 cycle"]
            pub const PRESCALE_9: u32 = 0x09;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Refresh timer period"]
    pub mod RT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "256*Prescaler period"]
            pub const RT_0: u32 = 0;
            #[doc = "RT*Prescaler period"]
            pub const RT_1: u32 = 0x01;
            #[doc = "RT*Prescaler period"]
            pub const RT_2: u32 = 0x02;
            #[doc = "RT*Prescaler period"]
            pub const RT_3: u32 = 0x03;
            #[doc = "RT*Prescaler period"]
            pub const RT_4: u32 = 0x04;
            #[doc = "RT*Prescaler period"]
            pub const RT_5: u32 = 0x05;
            #[doc = "RT*Prescaler period"]
            pub const RT_6: u32 = 0x06;
            #[doc = "RT*Prescaler period"]
            pub const RT_7: u32 = 0x07;
            #[doc = "RT*Prescaler period"]
            pub const RT_8: u32 = 0x08;
            #[doc = "RT*Prescaler period"]
            pub const RT_9: u32 = 0x09;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Urgent refresh threshold"]
    pub mod UT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "256*Prescaler period"]
            pub const UT_0: u32 = 0;
            #[doc = "UT*Prescaler period"]
            pub const UT_1: u32 = 0x01;
            #[doc = "UT*Prescaler period"]
            pub const UT_2: u32 = 0x02;
            #[doc = "UT*Prescaler period"]
            pub const UT_3: u32 = 0x03;
            #[doc = "UT*Prescaler period"]
            pub const UT_4: u32 = 0x04;
            #[doc = "UT*Prescaler period"]
            pub const UT_5: u32 = 0x05;
            #[doc = "UT*Prescaler period"]
            pub const UT_6: u32 = 0x06;
            #[doc = "UT*Prescaler period"]
            pub const UT_7: u32 = 0x07;
            #[doc = "UT*Prescaler period"]
            pub const UT_8: u32 = 0x08;
            #[doc = "UT*Prescaler period"]
            pub const UT_9: u32 = 0x09;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NAND Control Register 0"]
pub mod NANDCR0 {
    pub use crate::RW as access;
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Synchronous Mode Enable"]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Asynchronous mode is enabled."]
            pub const SYNCEN_0: u32 = 0;
            #[doc = "Synchronous mode is enabled."]
            pub const SYNCEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "16"]
            pub const BL_4: u32 = 0x04;
            #[doc = "32"]
            pub const BL_5: u32 = 0x05;
            #[doc = "64"]
            pub const BL_6: u32 = 0x06;
            #[doc = "64"]
            pub const BL_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EDO mode enabled"]
    pub mod EDO {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "EDO mode disabled"]
            pub const EDO_0: u32 = 0;
            #[doc = "EDO mode enabled"]
            pub const EDO_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Column address bit number"]
    pub mod COL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "16"]
            pub const COL_0: u32 = 0;
            #[doc = "15"]
            pub const COL_1: u32 = 0x01;
            #[doc = "14"]
            pub const COL_2: u32 = 0x02;
            #[doc = "13"]
            pub const COL_3: u32 = 0x03;
            #[doc = "12"]
            pub const COL_4: u32 = 0x04;
            #[doc = "11"]
            pub const COL_5: u32 = 0x05;
            #[doc = "10"]
            pub const COL_6: u32 = 0x06;
            #[doc = "9"]
            pub const COL_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND buffer enable for AXI access"]
    pub mod BUFEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "AXI access to NAND device directly"]
            pub const AXI_DIRECT: u32 = 0;
            #[doc = "AXI access through NAND buffer. It must be enabled for error correction schemes."]
            pub const AXI_BUFFER: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ECC mode selection"]
    pub mod ECC_MODE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No correction, ECC bypass"]
            pub const NO_ECC: u32 = 0;
            #[doc = "4-error correction (8 ECC bytes)"]
            pub const ECC_8BYTE: u32 = 0x01;
            #[doc = "8-error correction (16 ECC bytes)"]
            pub const ECC_16BYTE: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sector numbers in NAND buffer"]
    pub mod SECTOR_NUM {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "There is 1 sector in buffer"]
            pub const SECTOR1: u32 = 0;
            #[doc = "There are 2 sectors in buffer"]
            pub const SECTOR2: u32 = 0x01;
            #[doc = "There are 4 sectors in buffer"]
            pub const SECTOR4: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Size in bytes of one elementary unit of ECC correction."]
    pub mod SECTOR_SIZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NAND Control Register 1"]
pub mod NANDCR1 {
    pub use crate::RW as access;
    #[doc = "CE setup time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CE hold time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WE# LOW time"]
    pub mod WEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WE# HIGH time"]
    pub mod WEH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RE# LOW time"]
    pub mod REL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RE# HIGH time"]
    pub mod REH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Turnaround time"]
    pub mod TA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CE# interval time"]
    pub mod CEITV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NAND Control Register 2"]
pub mod NANDCR2 {
    pub use crate::RW as access;
    #[doc = "WE# HIGH to RE# LOW wait time"]
    pub mod TWHR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RE# HIGH to WE# LOW wait time"]
    pub mod TRHW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ALE to WRITE Data start wait time"]
    pub mod TADL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ready to RE# LOW min wait time"]
    pub mod TRR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WE# HIGH to busy wait time"]
    pub mod TWB {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NAND Control Register 3"]
pub mod NANDCR3 {
    pub use crate::RW as access;
    #[doc = "NAND option bit 1"]
    pub mod NDOPT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND option bit 2"]
    pub mod NDOPT2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND option bit 3"]
    pub mod NDOPT3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND CLE Option"]
    pub mod CLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Data Setup cycle time."]
    pub mod RDS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Data Hold cycle time."]
    pub mod RDH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Data Setup cycle time."]
    pub mod WDS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Data Hold cycle time."]
    pub mod WDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NOR Control Register 0"]
pub mod NORCR0 {
    pub use crate::RW as access;
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select NOR controller mode."]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Asynchronous mode is enabled."]
            pub const SYNCEN_0: u32 = 0;
            #[doc = "Synchronous mode is enabled."]
            pub const SYNCEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "16"]
            pub const BL_4: u32 = 0x04;
            #[doc = "32"]
            pub const BL_5: u32 = 0x05;
            #[doc = "64"]
            pub const BL_6: u32 = 0x06;
            #[doc = "64"]
            pub const BL_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address Mode"]
    pub mod AM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Address/Data MUX mode"]
            pub const AM_0: u32 = 0;
            #[doc = "Advanced Address/Data MUX mode"]
            pub const AM_1: u32 = 0x01;
            #[doc = "Address/Data non-MUX mode"]
            pub const AM_2: u32 = 0x02;
            #[doc = "Address/Data non-MUX mode"]
            pub const AM_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADV# polarity"]
    pub mod ADVP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADV# is Low Active. In ASYNC mode, device sample address with ADV# rise edge; In SYNC mode, device sample address when ADV# is LOW."]
            pub const ADVP_0: u32 = 0;
            #[doc = "ADV# is High Active. In ASYNC mode, device sample address with ADV# fall edge; In SYNC mode, device sample address when ADV# is HIGH."]
            pub const ADVP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADV# level control during address hold state"]
    pub mod ADVH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADV# is high during address hold state."]
            pub const ADVH_0: u32 = 0;
            #[doc = "ADV# is low during address hold state."]
            pub const ADVH_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "12 Bits"]
            pub const COL_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const COL_1: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const COL_2: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const COL_3: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const COL_4: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const COL_5: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const COL_6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const COL_7: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const COL_8: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const COL_9: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const COL_10: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const COL_11: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const COL_12: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const COL_13: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const COL_14: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const COL_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NOR Control Register 1"]
pub mod NORCR1 {
    pub use crate::RW as access;
    #[doc = "CE setup time cycle"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CE hold min time (CEH+1) cycle"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address setup time"]
    pub mod AS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address hold time"]
    pub mod AH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WE LOW time (WEL+1) cycle"]
    pub mod WEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WE HIGH time (WEH+1) cycle"]
    pub mod WEH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RE LOW time (REL+1) cycle"]
    pub mod REL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RE HIGH time (REH+1) cycle"]
    pub mod REH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NOR Control Register 2"]
pub mod NORCR2 {
    pub use crate::RW as access;
    #[doc = "Turnaround time cycle"]
    pub mod TA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address to write data hold time cycle"]
    pub mod AWDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Latency count"]
    pub mod LC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read cycle time"]
    pub mod RD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CE# interval min time"]
    pub mod CEITV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read cycle hold time"]
    pub mod RDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NOR Control Register 3"]
pub mod NORCR3 {
    pub use crate::RW as access;
    #[doc = "Address setup time for synchronous read"]
    pub mod ASSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address hold time for synchronous read"]
    pub mod AHSR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRAM Control Register 0"]
pub mod SRAMCR0 {
    pub use crate::RW as access;
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Synchronous Mode Enable"]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Asynchronous mode is enabled."]
            pub const SYNCEN_0: u32 = 0;
            #[doc = "Synchronous mode is enabled."]
            pub const SYNCEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wait Enable"]
    pub mod WAITEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The SEMC does not monitor wait pin."]
            pub const WAITEN_0: u32 = 0;
            #[doc = "The SEMC monitors wait pin. The SEMC does not transfer/receive data when wait pin is asserted."]
            pub const WAITEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wait Sample"]
    pub mod WAITSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Wait pin is directly used by the SEMC."]
            pub const WAITSP_0: u32 = 0;
            #[doc = "Wait pin is sampled by internal clock before it is used."]
            pub const WAITSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "16"]
            pub const BL_4: u32 = 0x04;
            #[doc = "32"]
            pub const BL_5: u32 = 0x05;
            #[doc = "64"]
            pub const BL_6: u32 = 0x06;
            #[doc = "64"]
            pub const BL_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address Mode"]
    pub mod AM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Address/Data MUX mode"]
            pub const AM_0: u32 = 0;
            #[doc = "Advanced Address/Data MUX mode"]
            pub const AM_1: u32 = 0x01;
            #[doc = "Address/Data non-MUX mode"]
            pub const AM_2: u32 = 0x02;
            #[doc = "Address/Data non-MUX mode"]
            pub const AM_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADV# polarity"]
    pub mod ADVP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADV# is Low Active. In ASYNC mode, device sample address with ADV# rise edge; In SYNC mode, device sample address when ADV# is LOW."]
            pub const ADVP_0: u32 = 0;
            #[doc = "ADV# is High Active. In ASYNC mode, device sample address with ADV# fall edge; In SYNC mode, device sample address when ADV# is HIGH."]
            pub const ADVP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADV# level control during address hold state"]
    pub mod ADVH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADV# is high during address hold state."]
            pub const ADVH_0: u32 = 0;
            #[doc = "ADV# is low during address hold state."]
            pub const ADVH_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "12 Bits"]
            pub const COL_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const COL_1: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const COL_2: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const COL_3: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const COL_4: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const COL_5: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const COL_6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const COL_7: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const COL_8: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const COL_9: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const COL_10: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const COL_11: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const COL_12: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const COL_13: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const COL_14: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const COL_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRAM Control Register 1"]
pub mod SRAMCR1 {
    pub use crate::RW as access;
    #[doc = "CE setup time cycle"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CE hold min time (CEH+1) cycle"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address setup time"]
    pub mod AS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address hold time"]
    pub mod AH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WE LOW time (WEL+1) cycle"]
    pub mod WEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WE HIGH time (WEH+1) cycle"]
    pub mod WEH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RE LOW time (REL+1) cycle"]
    pub mod REL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RE HIGH time (REH+1) cycle"]
    pub mod REH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRAM Control Register 2"]
pub mod SRAMCR2 {
    pub use crate::RW as access;
    #[doc = "Write Data setup time (WDS+1) cycle"]
    pub mod WDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Data hold time WDH cycle"]
    pub mod WDH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Turnaround time cycle"]
    pub mod TA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address to write data hold time cycle"]
    pub mod AWDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Latency count"]
    pub mod LC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read cycle time"]
    pub mod RD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CE# interval min time"]
    pub mod CEITV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read cycle hold time"]
    pub mod RDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRAM Control Register 3"]
pub mod SRAMCR3 {
    pub use crate::RW as access;
}
#[doc = "DBI-B Control Register 0"]
pub mod DBICR0 {
    pub use crate::RW as access;
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "16"]
            pub const BL_4: u32 = 0x04;
            #[doc = "32"]
            pub const BL_5: u32 = 0x05;
            #[doc = "64"]
            pub const BL_6: u32 = 0x06;
            #[doc = "64"]
            pub const BL_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "12 Bits"]
            pub const COL_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const COL_1: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const COL_2: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const COL_3: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const COL_4: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const COL_5: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const COL_6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const COL_7: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const COL_8: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const COL_9: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const COL_10: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const COL_11: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const COL_12: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const COL_13: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const COL_14: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const COL_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DBI-B Control Register 1"]
pub mod DBICR1 {
    pub use crate::RW as access;
    #[doc = "CSX Setup Time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CSX Hold Time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WRX Low Time"]
    pub mod WEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WRX High Time"]
    pub mod WEH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RDX Low Time"]
    pub mod REL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RDX High Time"]
    pub mod REH {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DBI-B Control Register 2"]
pub mod DBICR2 {
    pub use crate::RW as access;
    #[doc = "CSX interval time"]
    pub mod CEITV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IP Command Control Register 0"]
pub mod IPCR0 {
    pub use crate::RW as access;
    #[doc = "Slave address"]
    pub mod SA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IP Command Control Register 1"]
pub mod IPCR1 {
    pub use crate::RW as access;
    #[doc = "Data Size in Byte"]
    pub mod DATSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4"]
            pub const DATSZ_0: u32 = 0;
            #[doc = "1"]
            pub const DATSZ_1: u32 = 0x01;
            #[doc = "2"]
            pub const DATSZ_2: u32 = 0x02;
            #[doc = "3"]
            pub const DATSZ_3: u32 = 0x03;
            #[doc = "4"]
            pub const DATSZ_4: u32 = 0x04;
            #[doc = "4"]
            pub const DATSZ_5: u32 = 0x05;
            #[doc = "4"]
            pub const DATSZ_6: u32 = 0x06;
            #[doc = "4"]
            pub const DATSZ_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NAND Extended Address"]
    pub mod NAND_EXT_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IP Command Control Register 2"]
pub mod IPCR2 {
    pub use crate::RW as access;
    #[doc = "Byte Mask for Byte 0 (IPTXD bit 7:0)"]
    pub mod BM0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Byte Unmasked"]
            pub const BM0_0: u32 = 0;
            #[doc = "Byte Masked"]
            pub const BM0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Byte Mask for Byte 1 (IPTXD bit 15:8)"]
    pub mod BM1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Byte Unmasked"]
            pub const BM1_0: u32 = 0;
            #[doc = "Byte Masked"]
            pub const BM1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Byte Mask for Byte 2 (IPTXD bit 23:16)"]
    pub mod BM2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Byte Unmasked"]
            pub const BM2_0: u32 = 0;
            #[doc = "Byte Masked"]
            pub const BM2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Byte Mask for Byte 3 (IPTXD bit 31:24)"]
    pub mod BM3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Byte Unmasked"]
            pub const BM3_0: u32 = 0;
            #[doc = "Byte Masked"]
            pub const BM3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IP Command Register"]
pub mod IPCMD {
    pub use crate::RW as access;
    #[doc = "SDRAM Commands: 0x8: READ 0x9: WRITE 0xA: MODESET 0xB: ACTIVE 0xC: AUTO REFRESH 0xD: SELF REFRESH 0xE: PRECHARGE 0xF: PRECHARGE ALL Others: RSVD SELF REFRESH will be sent to all SDRAM devices because they shared same SEMC_CLK pin"]
    pub mod CMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field should be written with 0xA55A when trigging an IP command."]
    pub mod KEY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TX DATA Register"]
pub mod IPTXDAT {
    pub use crate::RW as access;
    #[doc = "no description available"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "RX DATA Register"]
pub mod IPRXDAT {
    pub use crate::RO as access;
    #[doc = "no description available"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status Register 0"]
pub mod STS0 {
    pub use crate::RO as access;
    #[doc = "Indicating whether SEMC is in IDLE state."]
    pub mod IDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicating NAND device Ready/WAIT# pin level."]
    pub mod NARDY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "NAND device is not ready"]
            pub const NARDY_0: u32 = 0;
            #[doc = "NAND device is ready"]
            pub const NARDY_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status Register 1"]
pub mod STS1 {
    pub use crate::RO as access;
}
#[doc = "Status Register 2"]
pub mod STS2 {
    pub use crate::RO as access;
    #[doc = "This field indicating whether there is pending AXI command (write) to NAND device."]
    pub mod NDWRPEND {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No pending"]
            pub const NDWRPEND_0: u32 = 0;
            #[doc = "Pending"]
            pub const NDWRPEND_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status Register 3"]
pub mod STS3 {
    pub use crate::RO as access;
}
#[doc = "Status Register 4"]
pub mod STS4 {
    pub use crate::RO as access;
}
#[doc = "Status Register 5"]
pub mod STS5 {
    pub use crate::RO as access;
}
#[doc = "Status Register 6"]
pub mod STS6 {
    pub use crate::RO as access;
}
#[doc = "Status Register 7"]
pub mod STS7 {
    pub use crate::RO as access;
}
#[doc = "Status Register 8"]
pub mod STS8 {
    pub use crate::RO as access;
}
#[doc = "Status Register 9"]
pub mod STS9 {
    pub use crate::RO as access;
}
#[doc = "Status Register 10"]
pub mod STS10 {
    pub use crate::RO as access;
}
#[doc = "Status Register 11"]
pub mod STS11 {
    pub use crate::RO as access;
}
#[doc = "Status Register 12"]
pub mod STS12 {
    pub use crate::RO as access;
    #[doc = "This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4)."]
    pub mod NDADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status Register 13"]
pub mod STS13 {
    pub use crate::RO as access;
    #[doc = "Sample clock slave delay line locked."]
    pub mod SLVLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slave delay line is not locked."]
            pub const SLVLOCK_0: u32 = 0;
            #[doc = "Slave delay line is locked."]
            pub const SLVLOCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample clock reference delay line locked."]
    pub mod REFLOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reference delay line is not locked."]
            pub const REFLOCK_0: u32 = 0;
            #[doc = "Reference delay line is locked."]
            pub const REFLOCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample clock slave delay line delay cell number selection."]
    pub mod SLVSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample clock reference delay line delay cell number selection."]
    pub mod REFSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status Register 14"]
pub mod STS14 {
    pub use crate::RO as access;
}
#[doc = "Status Register 15"]
pub mod STS15 {
    pub use crate::RO as access;
}
#[doc = "Base Register 9"]
pub mod BR9 {
    pub use crate::RW as access;
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The memory is invalid, can not be accessed."]
            pub const VLD_0: u32 = 0;
            #[doc = "The memory is valid, can be accessed."]
            pub const VLD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Base Register 10"]
pub mod BR10 {
    pub use crate::RW as access;
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The memory is invalid, can not be accessed."]
            pub const VLD_0: u32 = 0;
            #[doc = "The memory is valid, can be accessed."]
            pub const VLD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Base Register 11"]
pub mod BR11 {
    pub use crate::RW as access;
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The memory is invalid, can not be accessed."]
            pub const VLD_0: u32 = 0;
            #[doc = "The memory is valid, can be accessed."]
            pub const VLD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4KB"]
            pub const MS_0: u32 = 0;
            #[doc = "8KB"]
            pub const MS_1: u32 = 0x01;
            #[doc = "16KB"]
            pub const MS_2: u32 = 0x02;
            #[doc = "32KB"]
            pub const MS_3: u32 = 0x03;
            #[doc = "64KB"]
            pub const MS_4: u32 = 0x04;
            #[doc = "128KB"]
            pub const MS_5: u32 = 0x05;
            #[doc = "256KB"]
            pub const MS_6: u32 = 0x06;
            #[doc = "512KB"]
            pub const MS_7: u32 = 0x07;
            #[doc = "1MB"]
            pub const MS_8: u32 = 0x08;
            #[doc = "2MB"]
            pub const MS_9: u32 = 0x09;
            #[doc = "4MB"]
            pub const MS_10: u32 = 0x0a;
            #[doc = "8MB"]
            pub const MS_11: u32 = 0x0b;
            #[doc = "16MB"]
            pub const MS_12: u32 = 0x0c;
            #[doc = "32MB"]
            pub const MS_13: u32 = 0x0d;
            #[doc = "64MB"]
            pub const MS_14: u32 = 0x0e;
            #[doc = "128MB"]
            pub const MS_15: u32 = 0x0f;
            #[doc = "256MB"]
            pub const MS_16: u32 = 0x10;
            #[doc = "512MB"]
            pub const MS_17: u32 = 0x11;
            #[doc = "1GB"]
            pub const MS_18: u32 = 0x12;
            #[doc = "2GB"]
            pub const MS_19: u32 = 0x13;
            #[doc = "4GB"]
            pub const MS_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const MS_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const MS_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const MS_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const MS_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const MS_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const MS_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const MS_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const MS_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const MS_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const MS_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const MS_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRAM Control Register 4"]
pub mod SRAMCR4 {
    pub use crate::RW as access;
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8bit"]
            pub const PS_0: u32 = 0;
            #[doc = "16bit"]
            pub const PS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Synchronous Mode Enable"]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Asynchronous mode is enabled."]
            pub const SYNCEN_0: u32 = 0;
            #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
            pub const SYNCEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wait Enable"]
    pub mod WAITEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The SEMC does not monitor wait pin."]
            pub const WAITEN_0: u32 = 0;
            #[doc = "The SEMC monitors wait pin. The SEMC does not transfer/receive data when wait pin is asserted."]
            pub const WAITEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wait Sample"]
    pub mod WAITSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Wait pin is directly used by the SEMC."]
            pub const WAITSP_0: u32 = 0;
            #[doc = "Wait pin is sampled by internal clock before it is used."]
            pub const WAITSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1"]
            pub const BL_0: u32 = 0;
            #[doc = "2"]
            pub const BL_1: u32 = 0x01;
            #[doc = "4"]
            pub const BL_2: u32 = 0x02;
            #[doc = "8"]
            pub const BL_3: u32 = 0x03;
            #[doc = "16"]
            pub const BL_4: u32 = 0x04;
            #[doc = "32"]
            pub const BL_5: u32 = 0x05;
            #[doc = "64"]
            pub const BL_6: u32 = 0x06;
            #[doc = "64"]
            pub const BL_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address Mode"]
    pub mod AM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Address/Data MUX mode (ADMUX)"]
            pub const AM_0: u32 = 0;
            #[doc = "Advanced Address/Data MUX mode (AADM)"]
            pub const AM_1: u32 = 0x01;
            #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
            pub const AM_2: u32 = 0x02;
            #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
            pub const AM_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADV# polarity"]
    pub mod ADVP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADV# is active low."]
            pub const ADVP_0: u32 = 0;
            #[doc = "ADV# is active high."]
            pub const ADVP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADV# level control during address hold state"]
    pub mod ADVH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADV# is high during address hold state."]
            pub const ADVH_0: u32 = 0;
            #[doc = "ADV# is low during address hold state."]
            pub const ADVH_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "12 Bits"]
            pub const COL_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const COL_1: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const COL_2: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const COL_3: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const COL_4: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const COL_5: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const COL_6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const COL_7: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const COL_8: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const COL_9: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const COL_10: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const COL_11: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const COL_12: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const COL_13: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const COL_14: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const COL_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRAM Control Register 5"]
pub mod SRAMCR5 {
    pub use crate::RW as access;
    #[doc = "CE setup time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CE hold time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address setup time"]
    pub mod AS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address hold time"]
    pub mod AH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WE low time"]
    pub mod WEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WE high time"]
    pub mod WEH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RE low time"]
    pub mod REL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RE high time"]
    pub mod REH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRAM Control Register 6"]
pub mod SRAMCR6 {
    pub use crate::RW as access;
    #[doc = "Write Data setup time"]
    pub mod WDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Data hold time"]
    pub mod WDH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Turnaround time"]
    pub mod TA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address to write data hold time"]
    pub mod AWDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Latency count"]
    pub mod LC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read time"]
    pub mod RD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CE# interval time"]
    pub mod CEITV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read hold time"]
    pub mod RDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NAND Buffer DATA Register"]
pub mod NDBD {
    pub use crate::RW as access;
    #[doc = "NAND Buffer data. It is used for program or read operation from IPS bus."]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NAND Buffer Address Register"]
pub mod NDBA {
    pub use crate::RW as access;
    #[doc = "NAND Buffer address. It is used for program or read operation from IPS bus. It should be configured to proper value before access to NDBD register."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Delay Chain Control Register"]
pub mod DCCR {
    pub use crate::RW as access;
    #[doc = "Delay chain insertion enable for SRAM device."]
    pub mod SDRAMEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Delay chain is not inserted."]
            pub const SDRAMEN_0: u32 = 0;
            #[doc = "Delay chain is inserted."]
            pub const SDRAMEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Clock delay line delay cell number selection value for SDRAM device."]
    pub mod SDRAMVAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Delay chain insertion enable for NOR device."]
    pub mod NOREN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Delay chain is not inserted."]
            pub const NOREN_0: u32 = 0;
            #[doc = "Delay chain is inserted."]
            pub const NOREN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Clock delay line delay cell number selection value for NOR device."]
    pub mod NORVAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Delay chain insertion enable for SRAM device 0."]
    pub mod SRAM0EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Delay chain is not inserted."]
            pub const SRAM0EN_0: u32 = 0;
            #[doc = "Delay chain is inserted."]
            pub const SRAM0EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Clock delay line delay cell number selection value for SRAM device 0."]
    pub mod SRAM0VAL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Delay chain insertion enable for SRAM device 1-3."]
    pub mod SRAMXEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Delay chain is not inserted."]
            pub const SRAMXEN_0: u32 = 0;
            #[doc = "Delay chain is inserted."]
            pub const SRAMXEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Clock delay line delay cell number selection value for SRAM device 1-3."]
    pub mod SRAMXVAL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SDRAM Prefetch Control Register"]
pub mod SDRAMPCR {
    pub use crate::RW as access;
    #[doc = "SDRAM prefetch enable."]
    pub mod PREFETCH_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SDRAM prefetch function is disabled."]
            pub const PREFETCH_DISABLE: u32 = 0;
            #[doc = "SDRAM prefetch function is enabled."]
            pub const PREFETCH_ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SDRAM prefetch delay cycle."]
    pub mod PREFETCH_DLY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
