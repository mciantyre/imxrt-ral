#[doc = "RDC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version Information"]
    pub VIR: u32,
    _reserved0: [u8; 0x20],
    #[doc = "Status"]
    pub STAT: u32,
    #[doc = "Interrupt and Control"]
    pub INTCTRL: u32,
    #[doc = "Interrupt Status"]
    pub INTSTAT: u32,
    _reserved1: [u8; 0x01d0],
    #[doc = "Master Domain Assignment"]
    pub MDA: [u32; 12usize],
    _reserved2: [u8; 0x01d0],
    #[doc = "Peripheral Domain Access Permissions"]
    pub PDAP: [u32; 128usize],
    _reserved3: [u8; 0x0200],
    #[doc = "Memory Region Start Address"]
    pub MRSA0: u32,
    #[doc = "Memory Region End Address"]
    pub MREA0: u32,
    #[doc = "Memory Region Control"]
    pub MRC0: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS0: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA1: u32,
    #[doc = "Memory Region End Address"]
    pub MREA1: u32,
    #[doc = "Memory Region Control"]
    pub MRC1: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS1: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA2: u32,
    #[doc = "Memory Region End Address"]
    pub MREA2: u32,
    #[doc = "Memory Region Control"]
    pub MRC2: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS2: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA3: u32,
    #[doc = "Memory Region End Address"]
    pub MREA3: u32,
    #[doc = "Memory Region Control"]
    pub MRC3: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS3: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA4: u32,
    #[doc = "Memory Region End Address"]
    pub MREA4: u32,
    #[doc = "Memory Region Control"]
    pub MRC4: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS4: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA5: u32,
    #[doc = "Memory Region End Address"]
    pub MREA5: u32,
    #[doc = "Memory Region Control"]
    pub MRC5: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS5: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA6: u32,
    #[doc = "Memory Region End Address"]
    pub MREA6: u32,
    #[doc = "Memory Region Control"]
    pub MRC6: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS6: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA7: u32,
    #[doc = "Memory Region End Address"]
    pub MREA7: u32,
    #[doc = "Memory Region Control"]
    pub MRC7: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS7: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA8: u32,
    #[doc = "Memory Region End Address"]
    pub MREA8: u32,
    #[doc = "Memory Region Control"]
    pub MRC8: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS8: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA9: u32,
    #[doc = "Memory Region End Address"]
    pub MREA9: u32,
    #[doc = "Memory Region Control"]
    pub MRC9: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS9: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA10: u32,
    #[doc = "Memory Region End Address"]
    pub MREA10: u32,
    #[doc = "Memory Region Control"]
    pub MRC10: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS10: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA11: u32,
    #[doc = "Memory Region End Address"]
    pub MREA11: u32,
    #[doc = "Memory Region Control"]
    pub MRC11: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS11: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA12: u32,
    #[doc = "Memory Region End Address"]
    pub MREA12: u32,
    #[doc = "Memory Region Control"]
    pub MRC12: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS12: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA13: u32,
    #[doc = "Memory Region End Address"]
    pub MREA13: u32,
    #[doc = "Memory Region Control"]
    pub MRC13: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS13: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA14: u32,
    #[doc = "Memory Region End Address"]
    pub MREA14: u32,
    #[doc = "Memory Region Control"]
    pub MRC14: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS14: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA15: u32,
    #[doc = "Memory Region End Address"]
    pub MREA15: u32,
    #[doc = "Memory Region Control"]
    pub MRC15: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS15: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA16: u32,
    #[doc = "Memory Region End Address"]
    pub MREA16: u32,
    #[doc = "Memory Region Control"]
    pub MRC16: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS16: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA17: u32,
    #[doc = "Memory Region End Address"]
    pub MREA17: u32,
    #[doc = "Memory Region Control"]
    pub MRC17: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS17: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA18: u32,
    #[doc = "Memory Region End Address"]
    pub MREA18: u32,
    #[doc = "Memory Region Control"]
    pub MRC18: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS18: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA19: u32,
    #[doc = "Memory Region End Address"]
    pub MREA19: u32,
    #[doc = "Memory Region Control"]
    pub MRC19: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS19: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA20: u32,
    #[doc = "Memory Region End Address"]
    pub MREA20: u32,
    #[doc = "Memory Region Control"]
    pub MRC20: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS20: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA21: u32,
    #[doc = "Memory Region End Address"]
    pub MREA21: u32,
    #[doc = "Memory Region Control"]
    pub MRC21: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS21: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA22: u32,
    #[doc = "Memory Region End Address"]
    pub MREA22: u32,
    #[doc = "Memory Region Control"]
    pub MRC22: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS22: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA23: u32,
    #[doc = "Memory Region End Address"]
    pub MREA23: u32,
    #[doc = "Memory Region Control"]
    pub MRC23: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS23: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA24: u32,
    #[doc = "Memory Region End Address"]
    pub MREA24: u32,
    #[doc = "Memory Region Control"]
    pub MRC24: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS24: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA25: u32,
    #[doc = "Memory Region End Address"]
    pub MREA25: u32,
    #[doc = "Memory Region Control"]
    pub MRC25: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS25: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA26: u32,
    #[doc = "Memory Region End Address"]
    pub MREA26: u32,
    #[doc = "Memory Region Control"]
    pub MRC26: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS26: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA27: u32,
    #[doc = "Memory Region End Address"]
    pub MREA27: u32,
    #[doc = "Memory Region Control"]
    pub MRC27: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS27: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA28: u32,
    #[doc = "Memory Region End Address"]
    pub MREA28: u32,
    #[doc = "Memory Region Control"]
    pub MRC28: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS28: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA29: u32,
    #[doc = "Memory Region End Address"]
    pub MREA29: u32,
    #[doc = "Memory Region Control"]
    pub MRC29: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS29: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA30: u32,
    #[doc = "Memory Region End Address"]
    pub MREA30: u32,
    #[doc = "Memory Region Control"]
    pub MRC30: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS30: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA31: u32,
    #[doc = "Memory Region End Address"]
    pub MREA31: u32,
    #[doc = "Memory Region Control"]
    pub MRC31: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS31: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA32: u32,
    #[doc = "Memory Region End Address"]
    pub MREA32: u32,
    #[doc = "Memory Region Control"]
    pub MRC32: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS32: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA33: u32,
    #[doc = "Memory Region End Address"]
    pub MREA33: u32,
    #[doc = "Memory Region Control"]
    pub MRC33: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS33: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA34: u32,
    #[doc = "Memory Region End Address"]
    pub MREA34: u32,
    #[doc = "Memory Region Control"]
    pub MRC34: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS34: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA35: u32,
    #[doc = "Memory Region End Address"]
    pub MREA35: u32,
    #[doc = "Memory Region Control"]
    pub MRC35: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS35: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA36: u32,
    #[doc = "Memory Region End Address"]
    pub MREA36: u32,
    #[doc = "Memory Region Control"]
    pub MRC36: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS36: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA37: u32,
    #[doc = "Memory Region End Address"]
    pub MREA37: u32,
    #[doc = "Memory Region Control"]
    pub MRC37: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS37: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA38: u32,
    #[doc = "Memory Region End Address"]
    pub MREA38: u32,
    #[doc = "Memory Region Control"]
    pub MRC38: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS38: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA39: u32,
    #[doc = "Memory Region End Address"]
    pub MREA39: u32,
    #[doc = "Memory Region Control"]
    pub MRC39: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS39: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA40: u32,
    #[doc = "Memory Region End Address"]
    pub MREA40: u32,
    #[doc = "Memory Region Control"]
    pub MRC40: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS40: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA41: u32,
    #[doc = "Memory Region End Address"]
    pub MREA41: u32,
    #[doc = "Memory Region Control"]
    pub MRC41: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS41: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA42: u32,
    #[doc = "Memory Region End Address"]
    pub MREA42: u32,
    #[doc = "Memory Region Control"]
    pub MRC42: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS42: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA43: u32,
    #[doc = "Memory Region End Address"]
    pub MREA43: u32,
    #[doc = "Memory Region Control"]
    pub MRC43: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS43: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA44: u32,
    #[doc = "Memory Region End Address"]
    pub MREA44: u32,
    #[doc = "Memory Region Control"]
    pub MRC44: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS44: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA45: u32,
    #[doc = "Memory Region End Address"]
    pub MREA45: u32,
    #[doc = "Memory Region Control"]
    pub MRC45: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS45: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA46: u32,
    #[doc = "Memory Region End Address"]
    pub MREA46: u32,
    #[doc = "Memory Region Control"]
    pub MRC46: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS46: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA47: u32,
    #[doc = "Memory Region End Address"]
    pub MREA47: u32,
    #[doc = "Memory Region Control"]
    pub MRC47: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS47: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA48: u32,
    #[doc = "Memory Region End Address"]
    pub MREA48: u32,
    #[doc = "Memory Region Control"]
    pub MRC48: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS48: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA49: u32,
    #[doc = "Memory Region End Address"]
    pub MREA49: u32,
    #[doc = "Memory Region Control"]
    pub MRC49: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS49: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA50: u32,
    #[doc = "Memory Region End Address"]
    pub MREA50: u32,
    #[doc = "Memory Region Control"]
    pub MRC50: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS50: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA51: u32,
    #[doc = "Memory Region End Address"]
    pub MREA51: u32,
    #[doc = "Memory Region Control"]
    pub MRC51: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS51: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA52: u32,
    #[doc = "Memory Region End Address"]
    pub MREA52: u32,
    #[doc = "Memory Region Control"]
    pub MRC52: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS52: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA53: u32,
    #[doc = "Memory Region End Address"]
    pub MREA53: u32,
    #[doc = "Memory Region Control"]
    pub MRC53: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS53: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA54: u32,
    #[doc = "Memory Region End Address"]
    pub MREA54: u32,
    #[doc = "Memory Region Control"]
    pub MRC54: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS54: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA55: u32,
    #[doc = "Memory Region End Address"]
    pub MREA55: u32,
    #[doc = "Memory Region Control"]
    pub MRC55: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS55: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA56: u32,
    #[doc = "Memory Region End Address"]
    pub MREA56: u32,
    #[doc = "Memory Region Control"]
    pub MRC56: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS56: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA57: u32,
    #[doc = "Memory Region End Address"]
    pub MREA57: u32,
    #[doc = "Memory Region Control"]
    pub MRC57: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS57: u32,
    #[doc = "Memory Region Start Address"]
    pub MRSA58: u32,
    #[doc = "Memory Region End Address"]
    pub MREA58: u32,
    #[doc = "Memory Region Control"]
    pub MRC58: u32,
    #[doc = "Memory Region Violation Status"]
    pub MRVS58: u32,
}
#[doc = "Version Information"]
pub mod VIR {
    pub use crate::RO as access;
    #[doc = "Number of Domains"]
    pub mod NDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of Masters"]
    pub mod NMSTR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of Peripherals"]
    pub mod NPER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of Memory Regions"]
    pub mod NRGN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status"]
pub mod STAT {
    pub use crate::RW as access;
    #[doc = "Domain ID"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Power Domain Status"]
    pub mod PDS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Power Down Domain is OFF"]
            pub const PDS_0: u32 = 0;
            #[doc = "Power Down Domain is ON"]
            pub const PDS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt and Control"]
pub mod INTCTRL {
    pub use crate::RW as access;
    #[doc = "Restoration Complete Interrupt"]
    pub mod RCI_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt Disabled"]
            pub const RCI_EN_0: u32 = 0;
            #[doc = "Interrupt Enabled"]
            pub const RCI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt Status"]
pub mod INTSTAT {
    pub use crate::RW as access;
    #[doc = "Interrupt Status"]
    pub mod INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Interrupt Pending"]
            pub const INT_0: u32 = 0;
            #[doc = "Interrupt Pending"]
            pub const INT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Master Domain Assignment"]
pub mod MDA {
    pub use crate::RW as access;
    #[doc = "Domain ID"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Master assigned to Processing Domain 0"]
            pub const DID_0: u32 = 0;
            #[doc = "Master assigned to Processing Domain 1"]
            pub const DID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assignment Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not Locked"]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked"]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Peripheral Domain Access Permissions"]
pub mod PDAP {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Write Access"]
            pub const D0W_0: u32 = 0;
            #[doc = "Write Access Allowed"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Read Access"]
            pub const D0R_0: u32 = 0;
            #[doc = "Read Access Allowed"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Write Access"]
            pub const D1W_0: u32 = 0;
            #[doc = "Write Access Allowed"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Read Access"]
            pub const D1R_0: u32 = 0;
            #[doc = "Read Access Allowed"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Semaphore Required"]
    pub mod SREQ {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Semaphores have no effect"]
            pub const SREQ_0: u32 = 0;
            #[doc = "Semaphores are enforced"]
            pub const SREQ_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Peripheral Permissions Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not Locked"]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked"]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA0 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA0 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC0 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS0 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA1 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA1 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC1 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS1 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA2 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA2 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC2 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS2 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA3 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA3 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC3 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS3 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA4 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA4 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC4 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS4 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA5 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA5 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC5 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS5 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA6 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA6 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC6 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS6 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA7 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA7 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC7 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS7 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA8 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA8 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC8 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS8 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA9 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA9 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC9 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS9 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA10 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA10 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC10 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS10 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA11 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA11 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC11 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS11 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA12 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA12 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC12 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS12 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA13 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA13 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC13 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS13 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA14 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA14 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC14 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS14 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA15 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA15 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC15 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS15 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA16 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA16 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC16 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS16 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA17 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA17 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC17 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS17 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA18 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA18 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC18 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS18 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA19 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA19 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC19 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS19 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA20 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA20 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC20 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS20 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA21 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA21 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC21 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS21 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA22 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA22 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC22 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS22 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA23 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA23 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC23 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS23 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA24 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA24 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC24 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS24 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA25 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA25 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC25 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS25 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA26 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA26 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC26 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS26 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA27 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA27 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC27 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS27 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA28 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA28 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC28 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS28 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA29 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA29 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC29 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS29 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA30 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA30 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC30 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS30 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA31 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA31 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC31 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS31 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA32 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA32 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC32 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS32 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA33 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA33 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC33 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS33 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA34 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA34 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC34 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS34 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA35 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA35 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC35 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS35 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA36 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA36 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC36 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS36 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA37 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA37 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC37 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS37 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA38 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA38 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC38 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS38 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA39 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA39 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC39 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS39 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA40 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA40 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC40 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS40 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA41 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA41 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC41 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS41 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA42 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA42 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC42 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS42 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA43 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA43 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC43 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS43 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA44 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA44 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC44 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS44 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA45 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA45 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC45 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS45 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA46 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA46 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC46 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS46 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA47 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA47 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC47 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS47 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA48 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA48 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC48 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS48 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA49 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA49 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC49 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS49 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA50 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA50 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC50 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS50 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA51 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA51 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC51 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS51 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA52 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA52 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC52 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS52 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA53 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA53 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC53 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS53 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA54 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA54 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC54 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS54 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA55 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA55 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC55 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS55 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA56 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA56 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC56 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS56 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA57 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA57 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC57 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS57 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA58 {
    pub use crate::RW as access;
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA58 {
    pub use crate::RW as access;
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Control"]
pub mod MRC58 {
    pub use crate::RW as access;
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS58 {
    pub use crate::RW as access;
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
