#[doc = "MUA"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Processor A Transmit Register 0"]
    pub TR0: u32,
    #[doc = "Processor A Transmit Register 1"]
    pub TR1: u32,
    #[doc = "Processor A Transmit Register 2"]
    pub TR2: u32,
    #[doc = "Processor A Transmit Register 3"]
    pub TR3: u32,
    #[doc = "Processor A Receive Register 0"]
    pub RR0: u32,
    #[doc = "Processor A Receive Register 1"]
    pub RR1: u32,
    #[doc = "Processor A Receive Register 2"]
    pub RR2: u32,
    #[doc = "Processor A Receive Register 3"]
    pub RR3: u32,
    #[doc = "Processor A Status Register"]
    pub SR: u32,
    #[doc = "Processor A Control Register"]
    pub CR: u32,
}
#[doc = "Processor A Transmit Register 0"]
pub mod TR0 {
    pub use crate::RW as access;
    #[doc = "TR0"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Processor A Transmit Register 1"]
pub mod TR1 {
    pub use crate::RW as access;
    #[doc = "TR1"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Processor A Transmit Register 2"]
pub mod TR2 {
    pub use crate::RW as access;
    #[doc = "TR2"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Processor A Transmit Register 3"]
pub mod TR3 {
    pub use crate::RW as access;
    #[doc = "TR3"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Processor A Receive Register 0"]
pub mod RR0 {
    pub use crate::RO as access;
    #[doc = "RR0"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Processor A Receive Register 1"]
pub mod RR1 {
    pub use crate::RO as access;
    #[doc = "RR1"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Processor A Receive Register 2"]
pub mod RR2 {
    pub use crate::RO as access;
    #[doc = "RR2"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Processor A Receive Register 3"]
pub mod RR3 {
    pub use crate::RO as access;
    #[doc = "RR3"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Processor A Status Register"]
pub mod SR {
    pub use crate::RW as access;
    #[doc = "Fn"]
    pub mod FN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "BAFn bit in MUB.CR register is written 0 (default)."]
            pub const ZERO: u32 = 0;
            #[doc = "BAFn bit in MUB.CR register is written 1."]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EP"]
    pub mod EP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The Processor A-side event is not pending (default)."]
            pub const NOT_PENDING: u32 = 0;
            #[doc = "The Processor A-side event is pending."]
            pub const PENDING: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RS"]
    pub mod RS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The Processor B-side of the MU is not in reset."]
            pub const NOT_RESET: u32 = 0;
            #[doc = "The Processor B-side of the MU is in reset."]
            pub const RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FUP"]
    pub mod FUP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No flags updated, initiated by the Processor A, in progress (default)"]
            pub const NO_UPDATE: u32 = 0;
            #[doc = "Processor A initiated flags update, processing"]
            pub const UPDATE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "TEn"]
    pub mod TEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "MUA.TRn register is not empty."]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "MUA.TRn register is empty (default)."]
            pub const EMPTY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RFn"]
    pub mod RFN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "MUA.RRn register is not full (default)."]
            pub const NOT_FULL: u32 = 0;
            #[doc = "MUA.RRn register has received data from MUB.TRn register and is ready to be read by the Processor A."]
            pub const FULL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GIPn"]
    pub mod GIPN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processor A general purpose interrupt n is not pending. (default)"]
            pub const NOT_PENDING: u32 = 0;
            #[doc = "Processor A general purpose interrupt n is pending."]
            pub const PENDING: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Processor A Control Register"]
pub mod CR {
    pub use crate::RW as access;
    #[doc = "Fn"]
    pub mod FN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "N/A. Self clearing bit (default)."]
            pub const NOT_APPL: u32 = 0;
            #[doc = "Asserts the Processor A MU reset."]
            pub const ASSERT_RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MUR"]
    pub mod MUR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "N/A. Self clearing bit (default)."]
            pub const NOT_APPL: u32 = 0;
            #[doc = "Asserts the Processor A MU reset."]
            pub const ASSERT_RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GIRn"]
    pub mod GIRN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processor A General Interrupt n is not requested to the Processor B (default)."]
            pub const NOT_REQUESTED: u32 = 0;
            #[doc = "Processor A General Interrupt n is requested to the Processor B."]
            pub const REQUESTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "TIEn"]
    pub mod TIEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables Processor A Transmit Interrupt n. (default)"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables Processor A Transmit Interrupt n."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RIEn"]
    pub mod RIEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables Processor A Receive Interrupt n. (default)"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables Processor A Receive Interrupt n."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GIEn"]
    pub mod GIEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables Processor A General Interrupt n. (default)"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables Processor A General Interrupt n."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
