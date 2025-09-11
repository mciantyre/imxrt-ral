#[doc = "ASRC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ASRC Control Register"]
    pub ASRCTR: u32,
    #[doc = "ASRC Interrupt Enable Register"]
    pub ASRIER: u32,
    _reserved0: [u8; 0x04],
    #[doc = "ASRC Channel Number Configuration Register"]
    pub ASRCNCR: u32,
    #[doc = "ASRC Filter Configuration Status Register"]
    pub ASRCFG: u32,
    #[doc = "ASRC Clock Source Register"]
    pub ASRCSR: u32,
    #[doc = "ASRC Clock Divider Register 1"]
    pub ASRCDR1: u32,
    #[doc = "ASRC Clock Divider Register 2"]
    pub ASRCDR2: u32,
    #[doc = "ASRC Status Register"]
    pub ASRSTR: u32,
    _reserved1: [u8; 0x1c],
    #[doc = "ASRC Parameter Register n"]
    pub ASRPM: [u32; 5usize],
    #[doc = "ASRC Task Queue FIFO Register 1"]
    pub ASRTFR1: u32,
    _reserved2: [u8; 0x04],
    #[doc = "ASRC Channel Counter Register"]
    pub ASRCCR: u32,
    #[doc = "ASRC Data Input Register for Pair x"]
    pub ASRDIA: u32,
    #[doc = "ASRC Data Output Register for Pair x"]
    pub ASRDOA: u32,
    #[doc = "ASRC Data Input Register for Pair x"]
    pub ASRDIB: u32,
    #[doc = "ASRC Data Output Register for Pair x"]
    pub ASRDOB: u32,
    #[doc = "ASRC Data Input Register for Pair x"]
    pub ASRDIC: u32,
    #[doc = "ASRC Data Output Register for Pair x"]
    pub ASRDOC: u32,
    _reserved3: [u8; 0x08],
    #[doc = "ASRC Ideal Ratio for Pair A-High Part"]
    pub ASRIDRHA: u32,
    #[doc = "ASRC Ideal Ratio for Pair A -Low Part"]
    pub ASRIDRLA: u32,
    #[doc = "ASRC Ideal Ratio for Pair B-High Part"]
    pub ASRIDRHB: u32,
    #[doc = "ASRC Ideal Ratio for Pair B-Low Part"]
    pub ASRIDRLB: u32,
    #[doc = "ASRC Ideal Ratio for Pair C-High Part"]
    pub ASRIDRHC: u32,
    #[doc = "ASRC Ideal Ratio for Pair C-Low Part"]
    pub ASRIDRLC: u32,
    #[doc = "ASRC 76 kHz Period in terms of ASRC processing clock"]
    pub ASR76K: u32,
    #[doc = "ASRC 56 kHz Period in terms of ASRC processing clock"]
    pub ASR56K: u32,
    #[doc = "ASRC Misc Control Register for Pair A"]
    pub ASRMCRA: u32,
    #[doc = "ASRC FIFO Status Register for Pair A"]
    pub ASRFSTA: u32,
    #[doc = "ASRC Misc Control Register for Pair B"]
    pub ASRMCRB: u32,
    #[doc = "ASRC FIFO Status Register for Pair B"]
    pub ASRFSTB: u32,
    #[doc = "ASRC Misc Control Register for Pair C"]
    pub ASRMCRC: u32,
    #[doc = "ASRC FIFO Status Register for Pair C"]
    pub ASRFSTC: u32,
    _reserved4: [u8; 0x08],
    #[doc = "ASRC Misc Control Register 1 for Pair X"]
    pub ASRMCR1: [u32; 3usize],
}
#[doc = "ASRC Control Register"]
pub mod ASRCTR {
    pub use crate::RW as access;
    #[doc = "ASRCEN"]
    pub mod ASRCEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "operation of ASRC disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "operation ASRC is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ASREA"]
    pub mod ASREA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "operation of conversion A is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "operation of conversion A is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ASREB"]
    pub mod ASREB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "operation of conversion B is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "operation of conversion B is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ASREC"]
    pub mod ASREC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "operation of conversion C is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "operation of conversion C is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SRST"]
    pub mod SRST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ASRC Software reset cleared"]
            pub const CLEARED: u32 = 0;
            #[doc = "ASRC Software reset generated. NOTE: This is a self-clear bit"]
            pub const RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IDRA"]
    pub mod IDRA {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ASRC internal measured ratio is used"]
            pub const IDRA_MEASURED: u32 = 0;
            #[doc = "Ideal ratio from the interface register ASRIDRHA, ASRIDRLA is used"]
            pub const IDRA_IDEAL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USRA"]
    pub mod USRA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not use ratio as the input to ASRC for pair A"]
            pub const USE_RATIO_NO: u32 = 0;
            #[doc = "Use ratio as the input to ASRC for pair A"]
            pub const USE_RATIO: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IDRB"]
    pub mod IDRB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ASRC internal measured ratio is used"]
            pub const IDRA_MEASURED: u32 = 0;
            #[doc = "Ideal ratio from the interface register ASRIDRHB, ASRIDRLB is used"]
            pub const IDRA_IDEAL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USRB"]
    pub mod USRB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not use ratio as the input to ASRC for pair B"]
            pub const USE_RATIO_NO: u32 = 0;
            #[doc = "Use ratio as the input to ASRC for pair B"]
            pub const USE_RATIO: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IDRC"]
    pub mod IDRC {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ASRC internal measured ratio is used"]
            pub const IDRA_MEASURED: u32 = 0;
            #[doc = "Ideal ratio from the interface register ASRIDRHC, ASRIDRLC is used"]
            pub const IDRA_IDEAL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USRC"]
    pub mod USRC {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not use ratio as the input to ASRC for pair C"]
            pub const USE_RATIO_NO: u32 = 0;
            #[doc = "Use ratio as the input to ASRC for pair C"]
            pub const USE_RATIO: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ATSA"]
    pub mod ATSA {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pair A does not automatically update its pre-processing and post-processing options"]
            pub const NO_AUTO_SELECT: u32 = 0;
            #[doc = "Pair A automatically updates its pre-processing and post-processing options"]
            pub const AUTO_SELECT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ATSB"]
    pub mod ATSB {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pair B does not automatically update its pre-processing and post-processing options"]
            pub const NO_AUTO_SELECT: u32 = 0;
            #[doc = "Pair B automatically updates its pre-processing and post-processing options"]
            pub const AUTO_SELECT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ATSC"]
    pub mod ATSC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pair C does not automatically update its pre-processing and post-processing options"]
            pub const NO_AUTO_SELECT: u32 = 0;
            #[doc = "Pair C automatically updates its pre-processing and post-processing options"]
            pub const AUTO_SELECT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Interrupt Enable Register"]
pub mod ASRIER {
    pub use crate::RW as access;
    #[doc = "ADIEA"]
    pub mod ADIEA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "interrupt enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADIEB"]
    pub mod ADIEB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "interrupt enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADIEC"]
    pub mod ADIEC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "interrupt enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADOEA"]
    pub mod ADOEA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "interrupt enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADOEB"]
    pub mod ADOEB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "interrupt enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADOEC"]
    pub mod ADOEC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "interrupt enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOLIE"]
    pub mod AOLIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "interrupt enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AFPWE"]
    pub mod AFPWE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "interrupt enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Channel Number Configuration Register"]
pub mod ASRCNCR {
    pub use crate::RW as access;
    #[doc = "ANCA"]
    pub mod ANCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ANCB"]
    pub mod ANCB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ANCC"]
    pub mod ANCC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Filter Configuration Status Register"]
pub mod ASRCFG {
    pub use crate::RW as access;
    #[doc = "PREMODA"]
    pub mod PREMODA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
            #[doc = "Select passthrough mode. In this case, POSTMODA\\[1:0\\] have no use."]
            pub const PASSTHRU: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "POSTMODA"]
    pub mod POSTMODA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PREMODB"]
    pub mod PREMODB {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
            #[doc = "Select passthrough mode. In this case, POSTMODB\\[1:0\\] have no use."]
            pub const PASSTHRU: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "POSTMODB"]
    pub mod POSTMODB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PREMODC"]
    pub mod PREMODC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
            #[doc = "Select passthrough mode. In this case, POSTMODC\\[1:0\\] have no use."]
            pub const PASSTHRU: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "POSTMODC"]
    pub mod POSTMODC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select Upsampling-by-2 as defined in Signal Processing Flow."]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection as defined in Signal Processing Flow."]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2 as defined in Signal Processing Flow."]
            pub const DOWNSAMP_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NDPRA"]
    pub mod NDPRA {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use default parameters for RAM-stored parameters. Override any parameters already in RAM."]
            pub const USE_DEFAULT: u32 = 0;
            #[doc = "Don't use default parameters for RAM-stored parameters. Use the parameters already stored in RAM."]
            pub const NOT_DEFAULT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NDPRB"]
    pub mod NDPRB {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use default parameters for RAM-stored parameters. Override any parameters already in RAM."]
            pub const USE_DEFAULT: u32 = 0;
            #[doc = "Don't use default parameters for RAM-stored parameter. Use the parameters already stored in RAM."]
            pub const NOT_DEFAULT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NDPRC"]
    pub mod NDPRC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use default parameters for RAM-stored parameters. Override any parameters already in RAM."]
            pub const USE_DEFAULT: u32 = 0;
            #[doc = "Don't use default parameters for RAM-stored parameters. Use the parameters already stored in RAM."]
            pub const NOT_DEFAULT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "INIRQA"]
    pub mod INIRQA {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Initialization for Conversion Pair A not served"]
            pub const INIT_NOTSERVED: u32 = 0;
            #[doc = "Initialization for Conversion Pair A served"]
            pub const INIT_SERVED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "INIRQB"]
    pub mod INIRQB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Initialization for Conversion Pair B not served"]
            pub const INIT_NOTSERVED: u32 = 0;
            #[doc = "Initialization for Conversion Pair B served"]
            pub const INIT_SERVED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "INIRQC"]
    pub mod INIRQC {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Initialization for Conversion Pair C not served"]
            pub const INIT_NOTSERVED: u32 = 0;
            #[doc = "Initialization for Conversion Pair C served"]
            pub const INIT_SERVED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Clock Source Register"]
pub mod ASRCSR {
    pub use crate::RW as access;
    #[doc = "AICSA"]
    pub mod AICSA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AICSB"]
    pub mod AICSB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AICSC"]
    pub mod AICSC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOCSA"]
    pub mod AOCSA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOCSB"]
    pub mod AOCSB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOCSC"]
    pub mod AOCSC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Clock Divider Register 1"]
pub mod ASRCDR1 {
    pub use crate::RW as access;
    #[doc = "AICPA"]
    pub mod AICPA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AICDA"]
    pub mod AICDA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AICPB"]
    pub mod AICPB {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AICDB"]
    pub mod AICDB {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOCPA"]
    pub mod AOCPA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOCDA"]
    pub mod AOCDA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOCPB"]
    pub mod AOCPB {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOCDB"]
    pub mod AOCDB {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Clock Divider Register 2"]
pub mod ASRCDR2 {
    pub use crate::RW as access;
    #[doc = "AICPC"]
    pub mod AICPC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AICDC"]
    pub mod AICDC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOCPC"]
    pub mod AOCPC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOCDC"]
    pub mod AOCDC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Status Register"]
pub mod ASRSTR {
    pub use crate::RO as access;
    #[doc = "AIDEA"]
    pub mod AIDEA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The threshold has been met and no data input A interrupt is generated"]
            pub const THRESH_MET: u32 = 0;
            #[doc = "When AIDEA is set, the ASRC generates data input A interrupt request to the processor if ASRIER\\[AIDEA\\] = 1"]
            pub const LESSTHAN_THRESH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AIDEB"]
    pub mod AIDEB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The threshold has been met and no data input B interrupt is generated"]
            pub const THRESH_MET: u32 = 0;
            #[doc = "When AIDEB is set, the ASRC generates data input B interrupt request to the processor if ASRIER\\[AIDEB\\] = 1"]
            pub const LESSTHAN_THRESH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AIDEC"]
    pub mod AIDEC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The threshold has been met and no data input C interrupt is generated"]
            pub const THRESH_MET: u32 = 0;
            #[doc = "When AIDEC is set, the ASRC generates data input C interrupt request to the processor if ASRIER\\[AIDEC\\] = 1"]
            pub const LESSTHAN_THRESH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AODFA"]
    pub mod AODFA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The threshold has not yet been met and no data output A interrupt is generated"]
            pub const THRESH_NOTMET: u32 = 0;
            #[doc = "When AODFA is set, the ASRC generates data output A interrupt request to the processor if ASRIER\\[ADOEA\\] = 1"]
            pub const GREATERTHAN_THRESH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AODFB"]
    pub mod AODFB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The threshold has not yet been met and no data output B interrupt is generated"]
            pub const THRESH_NOTMET: u32 = 0;
            #[doc = "When AODFB is set, the ASRC generates data output B interrupt request to the processor if ASRIER\\[ADOEB\\] = 1"]
            pub const GREATERTHAN_THRESH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AODFC"]
    pub mod AODFC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The threshold has not yet been met and no data output C interrupt is generated"]
            pub const THRESH_NOTMET: u32 = 0;
            #[doc = "When AODFC is set, the ASRC generates data output C interrupt request to the processor if ASRIER\\[ADOEC\\] = 1"]
            pub const GREATERTHAN_THRESH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOLE"]
    pub mod AOLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No overload"]
            pub const TASK_OK: u32 = 0;
            #[doc = "Task rate is too high"]
            pub const TOO_HIGH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FPWT"]
    pub mod FPWT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ASRC is not in wait state"]
            pub const NO_WAITSTATE: u32 = 0;
            #[doc = "ASRC is in wait state"]
            pub const WAITSTATE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AIDUA"]
    pub mod AIDUA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Underflow in Input data buffer A"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Underflow in Input data buffer A"]
            pub const UNDERFLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AIDUB"]
    pub mod AIDUB {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Underflow in Input data buffer B"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Underflow in Input data buffer B"]
            pub const UNDERFLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AIDUC"]
    pub mod AIDUC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Underflow in Input data buffer C"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Underflow in Input data buffer C"]
            pub const UNDERFLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AODOA"]
    pub mod AODOA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Overflow in Output data buffer A"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow in Output data buffer A"]
            pub const OVERFLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AODOB"]
    pub mod AODOB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Overflow in Output data buffer B"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow in Output data buffer B"]
            pub const OVERFLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AODOC"]
    pub mod AODOC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Overflow in Output data buffer C"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow in Output data buffer C"]
            pub const OVERFLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AIOLA"]
    pub mod AIOLA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pair A input task is not oveloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair A input task is oveloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AIOLB"]
    pub mod AIOLB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pair B input task is not oveloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair B input task is oveloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AIOLC"]
    pub mod AIOLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pair C input task is not oveloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair C input task is oveloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOOLA"]
    pub mod AOOLA {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pair A output task is not oveloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair A output task is oveloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOOLB"]
    pub mod AOOLB {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pair B output task is not oveloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair B output task is oveloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AOOLC"]
    pub mod AOOLC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pair C output task is not oveloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair C output task is oveloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ATQOL"]
    pub mod ATQOL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Task queue FIFO logic is not oveloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Task queue FIFO logic is oveloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DSLCNT"]
    pub mod DSLCNT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "New DSL counter information is in the process of storage into the internal ASRC FIFO"]
            pub const DSLCNT_PROC: u32 = 0;
            #[doc = "New DSL counter information is stored in the internal ASRC FIFO"]
            pub const DSLCNT_STORED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Parameter Register n"]
pub mod ASRPM {
    pub use crate::RW as access;
    #[doc = "PARAMETER_VALUE"]
    pub mod PARAMETER_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Task Queue FIFO Register 1"]
pub mod ASRTFR1 {
    pub use crate::RW as access;
    #[doc = "TF_BASE"]
    pub mod TF_BASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "TF_FILL"]
    pub mod TF_FILL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Channel Counter Register"]
pub mod ASRCCR {
    pub use crate::RW as access;
    #[doc = "ACIA"]
    pub mod ACIA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ACIB"]
    pub mod ACIB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ACIC"]
    pub mod ACIC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ACOA"]
    pub mod ACOA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ACOB"]
    pub mod ACOB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ACOC"]
    pub mod ACOC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Data Input Register for Pair x"]
pub mod ASRDIA {
    pub use crate::WO as access;
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Data Output Register for Pair x"]
pub mod ASRDOA {
    pub use crate::RO as access;
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Data Input Register for Pair x"]
pub mod ASRDIB {
    pub use crate::WO as access;
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Data Output Register for Pair x"]
pub mod ASRDOB {
    pub use crate::RO as access;
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Data Input Register for Pair x"]
pub mod ASRDIC {
    pub use crate::WO as access;
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Data Output Register for Pair x"]
pub mod ASRDOC {
    pub use crate::RO as access;
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Ideal Ratio for Pair A-High Part"]
pub mod ASRIDRHA {
    pub use crate::RW as access;
    #[doc = "IDRATIOA_H"]
    pub mod IDRATIOA_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Ideal Ratio for Pair A -Low Part"]
pub mod ASRIDRLA {
    pub use crate::RW as access;
    #[doc = "IDRATIOA_L"]
    pub mod IDRATIOA_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Ideal Ratio for Pair B-High Part"]
pub mod ASRIDRHB {
    pub use crate::RW as access;
    #[doc = "IDRATIOB_H"]
    pub mod IDRATIOB_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Ideal Ratio for Pair B-Low Part"]
pub mod ASRIDRLB {
    pub use crate::RW as access;
    #[doc = "IDRATIOB_L"]
    pub mod IDRATIOB_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Ideal Ratio for Pair C-High Part"]
pub mod ASRIDRHC {
    pub use crate::RW as access;
    #[doc = "IDRATIOC_H"]
    pub mod IDRATIOC_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Ideal Ratio for Pair C-Low Part"]
pub mod ASRIDRLC {
    pub use crate::RW as access;
    #[doc = "IDRATIOC_L"]
    pub mod IDRATIOC_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC 76 kHz Period in terms of ASRC processing clock"]
pub mod ASR76K {
    pub use crate::RW as access;
    #[doc = "ASR76K"]
    pub mod ASR76K {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC 56 kHz Period in terms of ASRC processing clock"]
pub mod ASR56K {
    pub use crate::RW as access;
    #[doc = "ASR56K"]
    pub mod ASR56K {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Misc Control Register for Pair A"]
pub mod ASRMCRA {
    pub use crate::RW as access;
    #[doc = "INFIFO_THRESHOLDA"]
    pub mod INFIFO_THRESHOLDA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RSYNOFA"]
    pub mod RSYNOFA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not touch ASRCCR\\[ACOA\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACOA\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RSYNIFA"]
    pub mod RSYNIFA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not touch ASRCCR\\[ACIA\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACIA\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OUTFIFO_THRESHOLDA"]
    pub mod OUTFIFO_THRESHOLDA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BYPASSPOLYA"]
    pub mod BYPASSPOLYA {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Don't bypass polyphase filtering."]
            pub const NO_BYPASS: u32 = 0;
            #[doc = "Bypass polyphase filtering."]
            pub const BYPASS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BUFSTALLA"]
    pub mod BUFSTALLA {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Don't stall Pair A conversion even in case of near empty/full FIFO conditions."]
            pub const NO_STALL: u32 = 0;
            #[doc = "Stall Pair A conversion in case of near empty/full FIFO conditions."]
            pub const STALL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EXTTHRSHA"]
    pub mod EXTTHRSHA {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use default thresholds."]
            pub const USE_DEFAULT_THRESH: u32 = 0;
            #[doc = "Use external defined thresholds."]
            pub const USE_EXT_THRESH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ZEROBUFA"]
    pub mod ZEROBUFA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Zeroize the buffer"]
            pub const ZERO_BUF: u32 = 0;
            #[doc = "Don't zeroize the buffer"]
            pub const DO_NOT_ZERO_BUF: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC FIFO Status Register for Pair A"]
pub mod ASRFSTA {
    pub use crate::RO as access;
    #[doc = "INFIFO_FILLA"]
    pub mod INFIFO_FILLA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IAEA"]
    pub mod IAEA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Input FIFO is not near empty for Pair A"]
            pub const NOT_NEAR_EMPTY: u32 = 0;
            #[doc = "Input FIFO is near empty for Pair A"]
            pub const NEAR_EMPTY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OUTFIFO_FILLA"]
    pub mod OUTFIFO_FILLA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OAFA"]
    pub mod OAFA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Output FIFO is not near full for Pair A"]
            pub const NOT_NEAR_FULL: u32 = 0;
            #[doc = "Output FIFO is near full for Pair A"]
            pub const NEAR_FULL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Misc Control Register for Pair B"]
pub mod ASRMCRB {
    pub use crate::RW as access;
    #[doc = "INFIFO_THRESHOLDB"]
    pub mod INFIFO_THRESHOLDB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RSYNOFB"]
    pub mod RSYNOFB {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not touch ASRCCR\\[ACOB\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACOB\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RSYNIFB"]
    pub mod RSYNIFB {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not touch ASRCCR\\[ACIB\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACIB\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OUTFIFO_THRESHOLDB"]
    pub mod OUTFIFO_THRESHOLDB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BYPASSPOLYB"]
    pub mod BYPASSPOLYB {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Don't bypass polyphase filtering."]
            pub const NO_BYPASS: u32 = 0;
            #[doc = "Bypass polyphase filtering."]
            pub const BYPASS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BUFSTALLB"]
    pub mod BUFSTALLB {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Don't stall Pair B conversion even in case of near empty/full FIFO conditions."]
            pub const NO_STALL: u32 = 0;
            #[doc = "Stall Pair B conversion in case of near empty/full FIFO conditions."]
            pub const STALL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EXTTHRSHB"]
    pub mod EXTTHRSHB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use default thresholds."]
            pub const USE_DEFAULT_THRESH: u32 = 0;
            #[doc = "Use external defined thresholds."]
            pub const USE_EXT_THRESH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ZEROBUFB"]
    pub mod ZEROBUFB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Zeroize the buffer"]
            pub const ZERO_BUF: u32 = 0;
            #[doc = "Don't zeroize the buffer"]
            pub const DO_NOT_ZERO_BUF: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC FIFO Status Register for Pair B"]
pub mod ASRFSTB {
    pub use crate::RO as access;
    #[doc = "INFIFO_FILLB"]
    pub mod INFIFO_FILLB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IAEB"]
    pub mod IAEB {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Input FIFO is not near empty for Pair B"]
            pub const NOT_NEAR_EMPTY: u32 = 0;
            #[doc = "Input FIFO is near empty for Pair B"]
            pub const NEAR_EMPTY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OUTFIFO_FILLB"]
    pub mod OUTFIFO_FILLB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OAFB"]
    pub mod OAFB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Output FIFO is not near full for Pair B"]
            pub const NOT_NEAR_FULL: u32 = 0;
            #[doc = "Output FIFO is near full for Pair B"]
            pub const NEAR_FULL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Misc Control Register for Pair C"]
pub mod ASRMCRC {
    pub use crate::RW as access;
    #[doc = "INFIFO_THRESHOLDC"]
    pub mod INFIFO_THRESHOLDC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RSYNOFC"]
    pub mod RSYNOFC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not touch ASRCCR\\[ACOC\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACOC\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RSYNIFC"]
    pub mod RSYNIFC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not touch ASRCCR\\[ACIC\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACIC\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OUTFIFO_THRESHOLDC"]
    pub mod OUTFIFO_THRESHOLDC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BYPASSPOLYC"]
    pub mod BYPASSPOLYC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Don't bypass polyphase filtering."]
            pub const NO_BYPASS: u32 = 0;
            #[doc = "Bypass polyphase filtering."]
            pub const BYPASS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BUFSTALLC"]
    pub mod BUFSTALLC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Don't stall Pair C conversion even in case of near empty/full FIFO conditions."]
            pub const NO_STALL: u32 = 0;
            #[doc = "Stall Pair C conversion in case of near empty/full FIFO conditions."]
            pub const STALL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EXTTHRSHC"]
    pub mod EXTTHRSHC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use default thresholds."]
            pub const USE_DEFAULT_THRESH: u32 = 0;
            #[doc = "Use external defined thresholds."]
            pub const USE_EXT_THRESH: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ZEROBUFC"]
    pub mod ZEROBUFC {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Zeroize the buffer"]
            pub const ZERO_BUF: u32 = 0;
            #[doc = "Don't zeroize the buffer"]
            pub const DO_NOT_ZERO_BUF: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC FIFO Status Register for Pair C"]
pub mod ASRFSTC {
    pub use crate::RO as access;
    #[doc = "INFIFO_FILLC"]
    pub mod INFIFO_FILLC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IAEC"]
    pub mod IAEC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Input FIFO is not near empty for Pair C"]
            pub const NOT_NEAR_EMPTY: u32 = 0;
            #[doc = "Input FIFO is near empty for Pair C"]
            pub const NEAR_EMPTY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OUTFIFO_FILLC"]
    pub mod OUTFIFO_FILLC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OAFC"]
    pub mod OAFC {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Output FIFO is not near full for Pair C"]
            pub const NOT_NEAR_FULL: u32 = 0;
            #[doc = "Output FIFO is near full for Pair C"]
            pub const NEAR_FULL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASRC Misc Control Register 1 for Pair X"]
pub mod ASRMCR1 {
    pub use crate::RW as access;
    #[doc = "OW16"]
    pub mod OW16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "24-bit output data."]
            pub const OUT_24BIT: u32 = 0;
            #[doc = "16-bit output data"]
            pub const OUT_16BIT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OSGN"]
    pub mod OSGN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No sign extension."]
            pub const NO_SIGN_EXT: u32 = 0;
            #[doc = "Sign extension."]
            pub const SIGN_EXT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OMSB"]
    pub mod OMSB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LSB aligned."]
            pub const LSB_ALIGNED: u32 = 0;
            #[doc = "MSB aligned."]
            pub const MSB_ALIGNED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IMSB"]
    pub mod IMSB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LSB aligned."]
            pub const LSB_ALIGNED: u32 = 0;
            #[doc = "MSB aligned."]
            pub const MSB_ALIGNED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IWD"]
    pub mod IWD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "24-bit audio data."]
            pub const AUDIODATA_24BIT: u32 = 0;
            #[doc = "16-bit audio data."]
            pub const AUDIODATA_16BIT: u32 = 0x01;
            #[doc = "8-bit audio data."]
            pub const AUDIODATA_8BIT: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
