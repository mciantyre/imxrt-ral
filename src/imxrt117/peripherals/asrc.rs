#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ASRC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// ASRC Control Register
pub mod ASRCTR {

    /// ASRCEN
    pub mod ASRCEN {
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

            /// 0b0: operation of ASRC disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: operation ASRC is enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// ASREA
    pub mod ASREA {
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

            /// 0b0: operation of conversion A is disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: operation of conversion A is enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// ASREB
    pub mod ASREB {
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

            /// 0b0: operation of conversion B is disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: operation of conversion B is enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// ASREC
    pub mod ASREC {
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

            /// 0b0: operation of conversion C is disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: operation of conversion C is enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// SRST
    pub mod SRST {
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

            /// 0b0: ASRC Software reset cleared
            pub const cleared: u32 = 0b0;

            /// 0b1: ASRC Software reset generated. NOTE: This is a self-clear bit
            pub const reset: u32 = 0b1;
        }
    }

    /// IDRA
    pub mod IDRA {
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

            /// 0b0: ASRC internal measured ratio is used
            pub const idra_measured: u32 = 0b0;

            /// 0b1: Ideal ratio from the interface register ASRIDRHA, ASRIDRLA is used
            pub const idra_ideal: u32 = 0b1;
        }
    }

    /// USRA
    pub mod USRA {
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

            /// 0b0: Do not use ratio as the input to ASRC for pair A
            pub const use_ratio_no: u32 = 0b0;

            /// 0b1: Use ratio as the input to ASRC for pair A
            pub const use_ratio: u32 = 0b1;
        }
    }

    /// IDRB
    pub mod IDRB {
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

            /// 0b0: ASRC internal measured ratio is used
            pub const idra_measured: u32 = 0b0;

            /// 0b1: Ideal ratio from the interface register ASRIDRHB, ASRIDRLB is used
            pub const idra_ideal: u32 = 0b1;
        }
    }

    /// USRB
    pub mod USRB {
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

            /// 0b0: Do not use ratio as the input to ASRC for pair B
            pub const use_ratio_no: u32 = 0b0;

            /// 0b1: Use ratio as the input to ASRC for pair B
            pub const use_ratio: u32 = 0b1;
        }
    }

    /// IDRC
    pub mod IDRC {
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

            /// 0b0: ASRC internal measured ratio is used
            pub const idra_measured: u32 = 0b0;

            /// 0b1: Ideal ratio from the interface register ASRIDRHC, ASRIDRLC is used
            pub const idra_ideal: u32 = 0b1;
        }
    }

    /// USRC
    pub mod USRC {
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

            /// 0b0: Do not use ratio as the input to ASRC for pair C
            pub const use_ratio_no: u32 = 0b0;

            /// 0b1: Use ratio as the input to ASRC for pair C
            pub const use_ratio: u32 = 0b1;
        }
    }

    /// ATSA
    pub mod ATSA {
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

            /// 0b0: Pair A does not automatically update its pre-processing and post-processing options
            pub const no_auto_select: u32 = 0b0;

            /// 0b1: Pair A automatically updates its pre-processing and post-processing options
            pub const auto_select: u32 = 0b1;
        }
    }

    /// ATSB
    pub mod ATSB {
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

            /// 0b0: Pair B does not automatically update its pre-processing and post-processing options
            pub const no_auto_select: u32 = 0b0;

            /// 0b1: Pair B automatically updates its pre-processing and post-processing options
            pub const auto_select: u32 = 0b1;
        }
    }

    /// ATSC
    pub mod ATSC {
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

            /// 0b0: Pair C does not automatically update its pre-processing and post-processing options
            pub const no_auto_select: u32 = 0b0;

            /// 0b1: Pair C automatically updates its pre-processing and post-processing options
            pub const auto_select: u32 = 0b1;
        }
    }
}

/// ASRC Interrupt Enable Register
pub mod ASRIER {

    /// ADIEA
    pub mod ADIEA {
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

            /// 0b0: interrupt disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: interrupt enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// ADIEB
    pub mod ADIEB {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADIEA::RW;
    }

    /// ADIEC
    pub mod ADIEC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADIEA::RW;
    }

    /// ADOEA
    pub mod ADOEA {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADIEA::RW;
    }

    /// ADOEB
    pub mod ADOEB {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADIEA::RW;
    }

    /// ADOEC
    pub mod ADOEC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADIEA::RW;
    }

    /// AOLIE
    pub mod AOLIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADIEA::RW;
    }

    /// AFPWE
    pub mod AFPWE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADIEA::RW;
    }
}

/// ASRC Channel Number Configuration Register
pub mod ASRCNCR {

    /// ANCA
    pub mod ANCA {
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

    /// ANCB
    pub mod ANCB {
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

    /// ANCC
    pub mod ANCC {
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
}

/// ASRC Filter Configuration Status Register
pub mod ASRCFG {

    /// PREMODA
    pub mod PREMODA {
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

            /// 0b00: Select Upsampling-by-2
            pub const upsamp_2: u32 = 0b00;

            /// 0b01: Select Direct-Connection
            pub const direct_connect: u32 = 0b01;

            /// 0b10: Select Downsampling-by-2
            pub const downsamp_2: u32 = 0b10;

            /// 0b11: Select passthrough mode. In this case, POSTMODA\[1:0\] have no use.
            pub const passthru: u32 = 0b11;
        }
    }

    /// POSTMODA
    pub mod POSTMODA {
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

            /// 0b00: Select Upsampling-by-2
            pub const upsamp_2: u32 = 0b00;

            /// 0b01: Select Direct-Connection
            pub const direct_connect: u32 = 0b01;

            /// 0b10: Select Downsampling-by-2
            pub const downsamp_2: u32 = 0b10;
        }
    }

    /// PREMODB
    pub mod PREMODB {
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

            /// 0b00: Select Upsampling-by-2
            pub const upsamp_2: u32 = 0b00;

            /// 0b01: Select Direct-Connection
            pub const direct_connect: u32 = 0b01;

            /// 0b10: Select Downsampling-by-2
            pub const downsamp_2: u32 = 0b10;

            /// 0b11: Select passthrough mode. In this case, POSTMODB\[1:0\] have no use.
            pub const passthru: u32 = 0b11;
        }
    }

    /// POSTMODB
    pub mod POSTMODB {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::POSTMODA::RW;
    }

    /// PREMODC
    pub mod PREMODC {
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

            /// 0b00: Select Upsampling-by-2
            pub const upsamp_2: u32 = 0b00;

            /// 0b01: Select Direct-Connection
            pub const direct_connect: u32 = 0b01;

            /// 0b10: Select Downsampling-by-2
            pub const downsamp_2: u32 = 0b10;

            /// 0b11: Select passthrough mode. In this case, POSTMODC\[1:0\] have no use.
            pub const passthru: u32 = 0b11;
        }
    }

    /// POSTMODC
    pub mod POSTMODC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Select Upsampling-by-2 as defined in Signal Processing Flow.
            pub const upsamp_2: u32 = 0b00;

            /// 0b01: Select Direct-Connection as defined in Signal Processing Flow.
            pub const direct_connect: u32 = 0b01;

            /// 0b10: Select Downsampling-by-2 as defined in Signal Processing Flow.
            pub const downsamp_2: u32 = 0b10;
        }
    }

    /// NDPRA
    pub mod NDPRA {
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

            /// 0b0: Use default parameters for RAM-stored parameters. Override any parameters already in RAM.
            pub const use_default: u32 = 0b0;

            /// 0b1: Don't use default parameters for RAM-stored parameters. Use the parameters already stored in RAM.
            pub const not_default: u32 = 0b1;
        }
    }

    /// NDPRB
    pub mod NDPRB {
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

            /// 0b0: Use default parameters for RAM-stored parameters. Override any parameters already in RAM.
            pub const use_default: u32 = 0b0;

            /// 0b1: Don't use default parameters for RAM-stored parameter. Use the parameters already stored in RAM.
            pub const not_default: u32 = 0b1;
        }
    }

    /// NDPRC
    pub mod NDPRC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::NDPRA::RW;
    }

    /// INIRQA
    pub mod INIRQA {
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

            /// 0b0: Initialization for Conversion Pair A not served
            pub const init_notserved: u32 = 0b0;

            /// 0b1: Initialization for Conversion Pair A served
            pub const init_served: u32 = 0b1;
        }
    }

    /// INIRQB
    pub mod INIRQB {
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

            /// 0b0: Initialization for Conversion Pair B not served
            pub const init_notserved: u32 = 0b0;

            /// 0b1: Initialization for Conversion Pair B served
            pub const init_served: u32 = 0b1;
        }
    }

    /// INIRQC
    pub mod INIRQC {
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

            /// 0b0: Initialization for Conversion Pair C not served
            pub const init_notserved: u32 = 0b0;

            /// 0b1: Initialization for Conversion Pair C served
            pub const init_served: u32 = 0b1;
        }
    }
}

/// ASRC Clock Source Register
pub mod ASRCSR {

    /// AICSA
    pub mod AICSA {
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

            /// 0b0000: bit clock 0
            pub const bitclk0: u32 = 0b0000;

            /// 0b0001: bit clock 1
            pub const bitclk1: u32 = 0b0001;

            /// 0b0010: bit clock 2
            pub const bitclk2: u32 = 0b0010;

            /// 0b0011: bit clock 3
            pub const bitclk3: u32 = 0b0011;

            /// 0b0100: bit clock 4
            pub const bitclk4: u32 = 0b0100;

            /// 0b0101: bit clock 5
            pub const bitclk5: u32 = 0b0101;

            /// 0b0110: bit clock 6
            pub const bitclk6: u32 = 0b0110;

            /// 0b0111: bit clock 7
            pub const bitclk7: u32 = 0b0111;

            /// 0b1000: bit clock 8
            pub const bitclk8: u32 = 0b1000;

            /// 0b1001: bit clock 9
            pub const bitclk9: u32 = 0b1001;

            /// 0b1010: bit clock A
            pub const bitclka: u32 = 0b1010;

            /// 0b1011: bit clock B
            pub const bitclkb: u32 = 0b1011;

            /// 0b1100: bit clock C
            pub const bitclkc: u32 = 0b1100;

            /// 0b1101: bit clock D
            pub const bitclkd: u32 = 0b1101;

            /// 0b1110: bit clock E
            pub const bitclke: u32 = 0b1110;

            /// 0b1111: clock disabled, connected to zero
            pub const clk_disabled: u32 = 0b1111;
        }
    }

    /// AICSB
    pub mod AICSB {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AICSA::RW;
    }

    /// AICSC
    pub mod AICSC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AICSA::RW;
    }

    /// AOCSA
    pub mod AOCSA {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AICSA::RW;
    }

    /// AOCSB
    pub mod AOCSB {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AICSA::RW;
    }

    /// AOCSC
    pub mod AOCSC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AICSA::RW;
    }
}

/// ASRC Clock Divider Register 1
pub mod ASRCDR1 {

    /// AICPA
    pub mod AICPA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AICDA
    pub mod AICDA {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AICPB
    pub mod AICPB {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AICDB
    pub mod AICDB {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AOCPA
    pub mod AOCPA {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AOCDA
    pub mod AOCDA {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AOCPB
    pub mod AOCPB {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AOCDB
    pub mod AOCDB {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ASRC Clock Divider Register 2
pub mod ASRCDR2 {

    /// AICPC
    pub mod AICPC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AICDC
    pub mod AICDC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AOCPC
    pub mod AOCPC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AOCDC
    pub mod AOCDC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ASRC Status Register
pub mod ASRSTR {

    /// AIDEA
    pub mod AIDEA {
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

            /// 0b0: The threshold has been met and no data input A interrupt is generated
            pub const thresh_met: u32 = 0b0;

            /// 0b1: When AIDEA is set, the ASRC generates data input A interrupt request to the processor if ASRIER\[AIDEA\] = 1
            pub const lessthan_thresh: u32 = 0b1;
        }
    }

    /// AIDEB
    pub mod AIDEB {
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

            /// 0b0: The threshold has been met and no data input B interrupt is generated
            pub const thresh_met: u32 = 0b0;

            /// 0b1: When AIDEB is set, the ASRC generates data input B interrupt request to the processor if ASRIER\[AIDEB\] = 1
            pub const lessthan_thresh: u32 = 0b1;
        }
    }

    /// AIDEC
    pub mod AIDEC {
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

            /// 0b0: The threshold has been met and no data input C interrupt is generated
            pub const thresh_met: u32 = 0b0;

            /// 0b1: When AIDEC is set, the ASRC generates data input C interrupt request to the processor if ASRIER\[AIDEC\] = 1
            pub const lessthan_thresh: u32 = 0b1;
        }
    }

    /// AODFA
    pub mod AODFA {
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

            /// 0b0: The threshold has not yet been met and no data output A interrupt is generated
            pub const thresh_notmet: u32 = 0b0;

            /// 0b1: When AODFA is set, the ASRC generates data output A interrupt request to the processor if ASRIER\[ADOEA\] = 1
            pub const greaterthan_thresh: u32 = 0b1;
        }
    }

    /// AODFB
    pub mod AODFB {
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

            /// 0b0: The threshold has not yet been met and no data output B interrupt is generated
            pub const thresh_notmet: u32 = 0b0;

            /// 0b1: When AODFB is set, the ASRC generates data output B interrupt request to the processor if ASRIER\[ADOEB\] = 1
            pub const greaterthan_thresh: u32 = 0b1;
        }
    }

    /// AODFC
    pub mod AODFC {
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

            /// 0b0: The threshold has not yet been met and no data output C interrupt is generated
            pub const thresh_notmet: u32 = 0b0;

            /// 0b1: When AODFC is set, the ASRC generates data output C interrupt request to the processor if ASRIER\[ADOEC\] = 1
            pub const greaterthan_thresh: u32 = 0b1;
        }
    }

    /// AOLE
    pub mod AOLE {
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

            /// 0b0: No overload
            pub const task_ok: u32 = 0b0;

            /// 0b1: Task rate is too high
            pub const too_high: u32 = 0b1;
        }
    }

    /// FPWT
    pub mod FPWT {
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

            /// 0b0: ASRC is not in wait state
            pub const no_waitstate: u32 = 0b0;

            /// 0b1: ASRC is in wait state
            pub const waitstate: u32 = 0b1;
        }
    }

    /// AIDUA
    pub mod AIDUA {
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

            /// 0b0: No Underflow in Input data buffer A
            pub const no_underflow: u32 = 0b0;

            /// 0b1: Underflow in Input data buffer A
            pub const underflow: u32 = 0b1;
        }
    }

    /// AIDUB
    pub mod AIDUB {
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

            /// 0b0: No Underflow in Input data buffer B
            pub const no_underflow: u32 = 0b0;

            /// 0b1: Underflow in Input data buffer B
            pub const underflow: u32 = 0b1;
        }
    }

    /// AIDUC
    pub mod AIDUC {
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

            /// 0b0: No Underflow in Input data buffer C
            pub const no_underflow: u32 = 0b0;

            /// 0b1: Underflow in Input data buffer C
            pub const underflow: u32 = 0b1;
        }
    }

    /// AODOA
    pub mod AODOA {
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

            /// 0b0: No Overflow in Output data buffer A
            pub const no_overflow: u32 = 0b0;

            /// 0b1: Overflow in Output data buffer A
            pub const overflow: u32 = 0b1;
        }
    }

    /// AODOB
    pub mod AODOB {
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

            /// 0b0: No Overflow in Output data buffer B
            pub const no_overflow: u32 = 0b0;

            /// 0b1: Overflow in Output data buffer B
            pub const overflow: u32 = 0b1;
        }
    }

    /// AODOC
    pub mod AODOC {
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

            /// 0b0: No Overflow in Output data buffer C
            pub const no_overflow: u32 = 0b0;

            /// 0b1: Overflow in Output data buffer C
            pub const overflow: u32 = 0b1;
        }
    }

    /// AIOLA
    pub mod AIOLA {
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

            /// 0b0: Pair A input task is not oveloaded
            pub const no_overload: u32 = 0b0;

            /// 0b1: Pair A input task is oveloaded
            pub const overload: u32 = 0b1;
        }
    }

    /// AIOLB
    pub mod AIOLB {
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

            /// 0b0: Pair B input task is not oveloaded
            pub const no_overload: u32 = 0b0;

            /// 0b1: Pair B input task is oveloaded
            pub const overload: u32 = 0b1;
        }
    }

    /// AIOLC
    pub mod AIOLC {
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

            /// 0b0: Pair C input task is not oveloaded
            pub const no_overload: u32 = 0b0;

            /// 0b1: Pair C input task is oveloaded
            pub const overload: u32 = 0b1;
        }
    }

    /// AOOLA
    pub mod AOOLA {
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

            /// 0b0: Pair A output task is not oveloaded
            pub const no_overload: u32 = 0b0;

            /// 0b1: Pair A output task is oveloaded
            pub const overload: u32 = 0b1;
        }
    }

    /// AOOLB
    pub mod AOOLB {
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

            /// 0b0: Pair B output task is not oveloaded
            pub const no_overload: u32 = 0b0;

            /// 0b1: Pair B output task is oveloaded
            pub const overload: u32 = 0b1;
        }
    }

    /// AOOLC
    pub mod AOOLC {
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

            /// 0b0: Pair C output task is not oveloaded
            pub const no_overload: u32 = 0b0;

            /// 0b1: Pair C output task is oveloaded
            pub const overload: u32 = 0b1;
        }
    }

    /// ATQOL
    pub mod ATQOL {
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

            /// 0b0: Task queue FIFO logic is not oveloaded
            pub const no_overload: u32 = 0b0;

            /// 0b1: Task queue FIFO logic is oveloaded
            pub const overload: u32 = 0b1;
        }
    }

    /// DSLCNT
    pub mod DSLCNT {
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

            /// 0b0: New DSL counter information is in the process of storage into the internal ASRC FIFO
            pub const dslcnt_proc: u32 = 0b0;

            /// 0b1: New DSL counter information is stored in the internal ASRC FIFO
            pub const dslcnt_stored: u32 = 0b1;
        }
    }
}

/// ASRC Parameter Register n
pub mod ASRPM1 {

    /// PARAMETER_VALUE
    pub mod PARAMETER_VALUE {
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

/// ASRC Parameter Register n
pub mod ASRPM2 {
    pub use super::ASRPM1::PARAMETER_VALUE;
}

/// ASRC Parameter Register n
pub mod ASRPM3 {
    pub use super::ASRPM1::PARAMETER_VALUE;
}

/// ASRC Parameter Register n
pub mod ASRPM4 {
    pub use super::ASRPM1::PARAMETER_VALUE;
}

/// ASRC Parameter Register n
pub mod ASRPM5 {
    pub use super::ASRPM1::PARAMETER_VALUE;
}

/// ASRC Task Queue FIFO Register 1
pub mod ASRTFR1 {

    /// TF_BASE
    pub mod TF_BASE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (7 bits: 0x7f << 6)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TF_FILL
    pub mod TF_FILL {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (7 bits: 0x7f << 13)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ASRC Channel Counter Register
pub mod ASRCCR {

    /// ACIA
    pub mod ACIA {
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

    /// ACIB
    pub mod ACIB {
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

    /// ACIC
    pub mod ACIC {
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

    /// ACOA
    pub mod ACOA {
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

    /// ACOB
    pub mod ACOB {
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

    /// ACOC
    pub mod ACOC {
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

/// ASRC Data Input Register for Pair x
pub mod ASRDIA {

    /// DATA
    pub mod DATA {
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

/// ASRC Data Output Register for Pair x
pub mod ASRDOA {

    /// DATA
    pub mod DATA {
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

/// ASRC Data Input Register for Pair x
pub mod ASRDIB {
    pub use super::ASRDIA::DATA;
}

/// ASRC Data Output Register for Pair x
pub mod ASRDOB {
    pub use super::ASRDOA::DATA;
}

/// ASRC Data Input Register for Pair x
pub mod ASRDIC {
    pub use super::ASRDIA::DATA;
}

/// ASRC Data Output Register for Pair x
pub mod ASRDOC {
    pub use super::ASRDOA::DATA;
}

/// ASRC Ideal Ratio for Pair A-High Part
pub mod ASRIDRHA {

    /// IDRATIOA_H
    pub mod IDRATIOA_H {
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

/// ASRC Ideal Ratio for Pair A -Low Part
pub mod ASRIDRLA {

    /// IDRATIOA_L
    pub mod IDRATIOA_L {
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

/// ASRC Ideal Ratio for Pair B-High Part
pub mod ASRIDRHB {

    /// IDRATIOB_H
    pub mod IDRATIOB_H {
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

/// ASRC Ideal Ratio for Pair B-Low Part
pub mod ASRIDRLB {

    /// IDRATIOB_L
    pub mod IDRATIOB_L {
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

/// ASRC Ideal Ratio for Pair C-High Part
pub mod ASRIDRHC {

    /// IDRATIOC_H
    pub mod IDRATIOC_H {
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

/// ASRC Ideal Ratio for Pair C-Low Part
pub mod ASRIDRLC {

    /// IDRATIOC_L
    pub mod IDRATIOC_L {
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

/// ASRC 76 kHz Period in terms of ASRC processing clock
pub mod ASR76K {

    /// ASR76K
    pub mod ASR76K {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ASRC 56 kHz Period in terms of ASRC processing clock
pub mod ASR56K {

    /// ASR56K
    pub mod ASR56K {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ASRC Misc Control Register for Pair A
pub mod ASRMCRA {

    /// INFIFO_THRESHOLDA
    pub mod INFIFO_THRESHOLDA {
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

    /// RSYNOFA
    pub mod RSYNOFA {
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

            /// 0b0: Do not touch ASRCCR\[ACOA\]
            pub const no_resync: u32 = 0b0;

            /// 0b1: Force ASRCCR\[ACOA\]=0
            pub const resync: u32 = 0b1;
        }
    }

    /// RSYNIFA
    pub mod RSYNIFA {
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

            /// 0b0: Do not touch ASRCCR\[ACIA\]
            pub const no_resync: u32 = 0b0;

            /// 0b1: Force ASRCCR\[ACIA\]=0
            pub const resync: u32 = 0b1;
        }
    }

    /// OUTFIFO_THRESHOLDA
    pub mod OUTFIFO_THRESHOLDA {
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

    /// BYPASSPOLYA
    pub mod BYPASSPOLYA {
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

            /// 0b0: Don't bypass polyphase filtering.
            pub const no_bypass: u32 = 0b0;

            /// 0b1: Bypass polyphase filtering.
            pub const bypass: u32 = 0b1;
        }
    }

    /// BUFSTALLA
    pub mod BUFSTALLA {
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

            /// 0b0: Don't stall Pair A conversion even in case of near empty/full FIFO conditions.
            pub const no_stall: u32 = 0b0;

            /// 0b1: Stall Pair A conversion in case of near empty/full FIFO conditions.
            pub const stall: u32 = 0b1;
        }
    }

    /// EXTTHRSHA
    pub mod EXTTHRSHA {
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

            /// 0b0: Use default thresholds.
            pub const use_default_thresh: u32 = 0b0;

            /// 0b1: Use external defined thresholds.
            pub const use_ext_thresh: u32 = 0b1;
        }
    }

    /// ZEROBUFA
    pub mod ZEROBUFA {
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

            /// 0b0: Zeroize the buffer
            pub const zero_buf: u32 = 0b0;

            /// 0b1: Don't zeroize the buffer
            pub const do_not_zero_buf: u32 = 0b1;
        }
    }
}

/// ASRC FIFO Status Register for Pair A
pub mod ASRFSTA {

    /// INFIFO_FILLA
    pub mod INFIFO_FILLA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IAEA
    pub mod IAEA {
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

            /// 0b0: Input FIFO is not near empty for Pair A
            pub const not_near_empty: u32 = 0b0;

            /// 0b1: Input FIFO is near empty for Pair A
            pub const near_empty: u32 = 0b1;
        }
    }

    /// OUTFIFO_FILLA
    pub mod OUTFIFO_FILLA {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (7 bits: 0x7f << 12)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OAFA
    pub mod OAFA {
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

            /// 0b0: Output FIFO is not near full for Pair A
            pub const not_near_full: u32 = 0b0;

            /// 0b1: Output FIFO is near full for Pair A
            pub const near_full: u32 = 0b1;
        }
    }
}

/// ASRC Misc Control Register for Pair B
pub mod ASRMCRB {

    /// INFIFO_THRESHOLDB
    pub mod INFIFO_THRESHOLDB {
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

    /// RSYNOFB
    pub mod RSYNOFB {
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

            /// 0b0: Do not touch ASRCCR\[ACOB\]
            pub const no_resync: u32 = 0b0;

            /// 0b1: Force ASRCCR\[ACOB\]=0
            pub const resync: u32 = 0b1;
        }
    }

    /// RSYNIFB
    pub mod RSYNIFB {
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

            /// 0b0: Do not touch ASRCCR\[ACIB\]
            pub const no_resync: u32 = 0b0;

            /// 0b1: Force ASRCCR\[ACIB\]=0
            pub const resync: u32 = 0b1;
        }
    }

    /// OUTFIFO_THRESHOLDB
    pub mod OUTFIFO_THRESHOLDB {
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

    /// BYPASSPOLYB
    pub mod BYPASSPOLYB {
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

            /// 0b0: Don't bypass polyphase filtering.
            pub const no_bypass: u32 = 0b0;

            /// 0b1: Bypass polyphase filtering.
            pub const bypass: u32 = 0b1;
        }
    }

    /// BUFSTALLB
    pub mod BUFSTALLB {
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

            /// 0b0: Don't stall Pair B conversion even in case of near empty/full FIFO conditions.
            pub const no_stall: u32 = 0b0;

            /// 0b1: Stall Pair B conversion in case of near empty/full FIFO conditions.
            pub const stall: u32 = 0b1;
        }
    }

    /// EXTTHRSHB
    pub mod EXTTHRSHB {
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

            /// 0b0: Use default thresholds.
            pub const use_default_thresh: u32 = 0b0;

            /// 0b1: Use external defined thresholds.
            pub const use_ext_thresh: u32 = 0b1;
        }
    }

    /// ZEROBUFB
    pub mod ZEROBUFB {
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

            /// 0b0: Zeroize the buffer
            pub const zero_buf: u32 = 0b0;

            /// 0b1: Don't zeroize the buffer
            pub const do_not_zero_buf: u32 = 0b1;
        }
    }
}

/// ASRC FIFO Status Register for Pair B
pub mod ASRFSTB {

    /// INFIFO_FILLB
    pub mod INFIFO_FILLB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IAEB
    pub mod IAEB {
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

            /// 0b0: Input FIFO is not near empty for Pair B
            pub const not_near_empty: u32 = 0b0;

            /// 0b1: Input FIFO is near empty for Pair B
            pub const near_empty: u32 = 0b1;
        }
    }

    /// OUTFIFO_FILLB
    pub mod OUTFIFO_FILLB {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (7 bits: 0x7f << 12)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OAFB
    pub mod OAFB {
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

            /// 0b0: Output FIFO is not near full for Pair B
            pub const not_near_full: u32 = 0b0;

            /// 0b1: Output FIFO is near full for Pair B
            pub const near_full: u32 = 0b1;
        }
    }
}

/// ASRC Misc Control Register for Pair C
pub mod ASRMCRC {

    /// INFIFO_THRESHOLDC
    pub mod INFIFO_THRESHOLDC {
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

    /// RSYNOFC
    pub mod RSYNOFC {
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

            /// 0b0: Do not touch ASRCCR\[ACOC\]
            pub const no_resync: u32 = 0b0;

            /// 0b1: Force ASRCCR\[ACOC\]=0
            pub const resync: u32 = 0b1;
        }
    }

    /// RSYNIFC
    pub mod RSYNIFC {
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

            /// 0b0: Do not touch ASRCCR\[ACIC\]
            pub const no_resync: u32 = 0b0;

            /// 0b1: Force ASRCCR\[ACIC\]=0
            pub const resync: u32 = 0b1;
        }
    }

    /// OUTFIFO_THRESHOLDC
    pub mod OUTFIFO_THRESHOLDC {
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

    /// BYPASSPOLYC
    pub mod BYPASSPOLYC {
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

            /// 0b0: Don't bypass polyphase filtering.
            pub const no_bypass: u32 = 0b0;

            /// 0b1: Bypass polyphase filtering.
            pub const bypass: u32 = 0b1;
        }
    }

    /// BUFSTALLC
    pub mod BUFSTALLC {
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

            /// 0b0: Don't stall Pair C conversion even in case of near empty/full FIFO conditions.
            pub const no_stall: u32 = 0b0;

            /// 0b1: Stall Pair C conversion in case of near empty/full FIFO conditions.
            pub const stall: u32 = 0b1;
        }
    }

    /// EXTTHRSHC
    pub mod EXTTHRSHC {
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

            /// 0b0: Use default thresholds.
            pub const use_default_thresh: u32 = 0b0;

            /// 0b1: Use external defined thresholds.
            pub const use_ext_thresh: u32 = 0b1;
        }
    }

    /// ZEROBUFC
    pub mod ZEROBUFC {
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

            /// 0b0: Zeroize the buffer
            pub const zero_buf: u32 = 0b0;

            /// 0b1: Don't zeroize the buffer
            pub const do_not_zero_buf: u32 = 0b1;
        }
    }
}

/// ASRC FIFO Status Register for Pair C
pub mod ASRFSTC {

    /// INFIFO_FILLC
    pub mod INFIFO_FILLC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IAEC
    pub mod IAEC {
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

            /// 0b0: Input FIFO is not near empty for Pair C
            pub const not_near_empty: u32 = 0b0;

            /// 0b1: Input FIFO is near empty for Pair C
            pub const near_empty: u32 = 0b1;
        }
    }

    /// OUTFIFO_FILLC
    pub mod OUTFIFO_FILLC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (7 bits: 0x7f << 12)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OAFC
    pub mod OAFC {
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

            /// 0b0: Output FIFO is not near full for Pair C
            pub const not_near_full: u32 = 0b0;

            /// 0b1: Output FIFO is near full for Pair C
            pub const near_full: u32 = 0b1;
        }
    }
}

/// ASRC Misc Control Register 1 for Pair X
pub mod ASRMCR1A {

    /// OW16
    pub mod OW16 {
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

            /// 0b0: 24-bit output data.
            pub const out_24bit: u32 = 0b0;

            /// 0b1: 16-bit output data
            pub const out_16bit: u32 = 0b1;
        }
    }

    /// OSGN
    pub mod OSGN {
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

            /// 0b0: No sign extension.
            pub const no_sign_ext: u32 = 0b0;

            /// 0b1: Sign extension.
            pub const sign_ext: u32 = 0b1;
        }
    }

    /// OMSB
    pub mod OMSB {
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

            /// 0b0: LSB aligned.
            pub const lsb_aligned: u32 = 0b0;

            /// 0b1: MSB aligned.
            pub const msb_aligned: u32 = 0b1;
        }
    }

    /// IMSB
    pub mod IMSB {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OMSB::RW;
    }

    /// IWD
    pub mod IWD {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 24-bit audio data.
            pub const audiodata_24bit: u32 = 0b00;

            /// 0b01: 16-bit audio data.
            pub const audiodata_16bit: u32 = 0b01;

            /// 0b10: 8-bit audio data.
            pub const audiodata_8bit: u32 = 0b10;
        }
    }
}

/// ASRC Misc Control Register 1 for Pair X
pub mod ASRMCR1B {
    pub use super::ASRMCR1A::IMSB;
    pub use super::ASRMCR1A::IWD;
    pub use super::ASRMCR1A::OMSB;
    pub use super::ASRMCR1A::OSGN;
    pub use super::ASRMCR1A::OW16;
}

/// ASRC Misc Control Register 1 for Pair X
pub mod ASRMCR1C {
    pub use super::ASRMCR1A::IMSB;
    pub use super::ASRMCR1A::IWD;
    pub use super::ASRMCR1A::OMSB;
    pub use super::ASRMCR1A::OSGN;
    pub use super::ASRMCR1A::OW16;
}
#[repr(C)]
pub struct RegisterBlock {
    /// ASRC Control Register
    pub ASRCTR: RWRegister<u32>,

    /// ASRC Interrupt Enable Register
    pub ASRIER: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// ASRC Channel Number Configuration Register
    pub ASRCNCR: RWRegister<u32>,

    /// ASRC Filter Configuration Status Register
    pub ASRCFG: RWRegister<u32>,

    /// ASRC Clock Source Register
    pub ASRCSR: RWRegister<u32>,

    /// ASRC Clock Divider Register 1
    pub ASRCDR1: RWRegister<u32>,

    /// ASRC Clock Divider Register 2
    pub ASRCDR2: RWRegister<u32>,

    /// ASRC Status Register
    pub ASRSTR: RORegister<u32>,

    _reserved2: [u32; 7],

    /// ASRC Parameter Register n
    pub ASRPM1: RWRegister<u32>,

    /// ASRC Parameter Register n
    pub ASRPM2: RWRegister<u32>,

    /// ASRC Parameter Register n
    pub ASRPM3: RWRegister<u32>,

    /// ASRC Parameter Register n
    pub ASRPM4: RWRegister<u32>,

    /// ASRC Parameter Register n
    pub ASRPM5: RWRegister<u32>,

    /// ASRC Task Queue FIFO Register 1
    pub ASRTFR1: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// ASRC Channel Counter Register
    pub ASRCCR: RWRegister<u32>,

    /// ASRC Data Input Register for Pair x
    pub ASRDIA: WORegister<u32>,

    /// ASRC Data Output Register for Pair x
    pub ASRDOA: RORegister<u32>,

    /// ASRC Data Input Register for Pair x
    pub ASRDIB: WORegister<u32>,

    /// ASRC Data Output Register for Pair x
    pub ASRDOB: RORegister<u32>,

    /// ASRC Data Input Register for Pair x
    pub ASRDIC: WORegister<u32>,

    /// ASRC Data Output Register for Pair x
    pub ASRDOC: RORegister<u32>,

    _reserved4: [u32; 2],

    /// ASRC Ideal Ratio for Pair A-High Part
    pub ASRIDRHA: RWRegister<u32>,

    /// ASRC Ideal Ratio for Pair A -Low Part
    pub ASRIDRLA: RWRegister<u32>,

    /// ASRC Ideal Ratio for Pair B-High Part
    pub ASRIDRHB: RWRegister<u32>,

    /// ASRC Ideal Ratio for Pair B-Low Part
    pub ASRIDRLB: RWRegister<u32>,

    /// ASRC Ideal Ratio for Pair C-High Part
    pub ASRIDRHC: RWRegister<u32>,

    /// ASRC Ideal Ratio for Pair C-Low Part
    pub ASRIDRLC: RWRegister<u32>,

    /// ASRC 76 kHz Period in terms of ASRC processing clock
    pub ASR76K: RWRegister<u32>,

    /// ASRC 56 kHz Period in terms of ASRC processing clock
    pub ASR56K: RWRegister<u32>,

    /// ASRC Misc Control Register for Pair A
    pub ASRMCRA: RWRegister<u32>,

    /// ASRC FIFO Status Register for Pair A
    pub ASRFSTA: RORegister<u32>,

    /// ASRC Misc Control Register for Pair B
    pub ASRMCRB: RWRegister<u32>,

    /// ASRC FIFO Status Register for Pair B
    pub ASRFSTB: RORegister<u32>,

    /// ASRC Misc Control Register for Pair C
    pub ASRMCRC: RWRegister<u32>,

    /// ASRC FIFO Status Register for Pair C
    pub ASRFSTC: RORegister<u32>,

    _reserved5: [u32; 2],

    /// ASRC Misc Control Register 1 for Pair X
    pub ASRMCR1A: RWRegister<u32>,

    /// ASRC Misc Control Register 1 for Pair X
    pub ASRMCR1B: RWRegister<u32>,

    /// ASRC Misc Control Register 1 for Pair X
    pub ASRMCR1C: RWRegister<u32>,
}
pub struct ResetValues {
    pub ASRCTR: u32,
    pub ASRIER: u32,
    pub ASRCNCR: u32,
    pub ASRCFG: u32,
    pub ASRCSR: u32,
    pub ASRCDR1: u32,
    pub ASRCDR2: u32,
    pub ASRSTR: u32,
    pub ASRPM1: u32,
    pub ASRPM2: u32,
    pub ASRPM3: u32,
    pub ASRPM4: u32,
    pub ASRPM5: u32,
    pub ASRTFR1: u32,
    pub ASRCCR: u32,
    pub ASRDIA: u32,
    pub ASRDOA: u32,
    pub ASRDIB: u32,
    pub ASRDOB: u32,
    pub ASRDIC: u32,
    pub ASRDOC: u32,
    pub ASRIDRHA: u32,
    pub ASRIDRLA: u32,
    pub ASRIDRHB: u32,
    pub ASRIDRLB: u32,
    pub ASRIDRHC: u32,
    pub ASRIDRLC: u32,
    pub ASR76K: u32,
    pub ASR56K: u32,
    pub ASRMCRA: u32,
    pub ASRFSTA: u32,
    pub ASRMCRB: u32,
    pub ASRFSTB: u32,
    pub ASRMCRC: u32,
    pub ASRFSTC: u32,
    pub ASRMCR1A: u32,
    pub ASRMCR1B: u32,
    pub ASRMCR1C: u32,
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
