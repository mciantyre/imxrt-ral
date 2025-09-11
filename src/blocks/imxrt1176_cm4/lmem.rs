#[doc = "LMEM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "PC bus Cache control register"]
    pub PCCCR: u32,
    #[doc = "PC bus Cache line control register"]
    pub PCCLCR: u32,
    #[doc = "PC bus Cache search address register"]
    pub PCCSAR: u32,
    #[doc = "PC bus Cache read/write value register"]
    pub PCCCVR: u32,
    _reserved0: [u8; 0x07f0],
    #[doc = "PS bus Cache control register"]
    pub PSCCR: u32,
    #[doc = "PS bus Cache line control register"]
    pub PSCLCR: u32,
    #[doc = "PS bus Cache search address register"]
    pub PSCSAR: u32,
    #[doc = "PS bus Cache read/write value register"]
    pub PSCCVR: u32,
}
#[doc = "PC bus Cache control register"]
pub mod PCCCR {
    pub use crate::RW as access;
    #[doc = "Cache enable"]
    pub mod ENCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cache disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Cache enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Write Buffer"]
    pub mod ENWRBUF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write buffer disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Write buffer enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Forces all cacheable spaces to write through"]
    pub mod PCCR2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Does NOT force all cacheable spaces to write through"]
            pub const PCCR2_0: u32 = 0;
            #[doc = "Forces all cacheable spaces to write through"]
            pub const PCCR2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Forces no allocation on cache misses"]
    pub mod PCCR3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allocation on cache misses"]
            pub const PCCR3_0: u32 = 0;
            #[doc = "Forces no allocation on cache misses (must also have PCCR2 asserted)"]
            pub const PCCR3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Invalidate Way 0"]
    pub mod INVW0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "When setting the GO bit, invalidate all lines in way 0."]
            pub const INVW0: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Push Way 0"]
    pub mod PUSHW0 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "When setting the GO bit, push all modified lines in way 0"]
            pub const PUSHW0: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Invalidate Way 1"]
    pub mod INVW1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "When setting the GO bit, invalidate all lines in way 1"]
            pub const INVW1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Push Way 1"]
    pub mod PUSHW1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "When setting the GO bit, push all modified lines in way 1"]
            pub const PUSHW1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Initiate Cache Command"]
    pub mod GO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write: no effect. Read: no cache command active."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
            pub const INIT_CMD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PC bus Cache line control register"]
pub mod PCCLCR {
    pub use crate::RW as access;
    #[doc = "Initiate Cache Line Command"]
    pub mod LGO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write: no effect. Read: no line command active."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
            pub const INIT_CMD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Cache address"]
    pub mod CACHEADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Way select"]
    pub mod WSEL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Way 0"]
            pub const WAY0: u32 = 0;
            #[doc = "Way 1"]
            pub const WAY1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Tag/Data Select"]
    pub mod TDSEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Data"]
            pub const DATA: u32 = 0;
            #[doc = "Tag"]
            pub const TAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Command Initial Valid Bit"]
    pub mod LCIVB {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Command Initial Modified Bit"]
    pub mod LCIMB {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Command Way"]
    pub mod LCWAY {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Command"]
    pub mod LCMD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Search and read or write"]
            pub const SEARCH_RW: u32 = 0;
            #[doc = "Invalidate"]
            pub const INVALIDATE: u32 = 0x01;
            #[doc = "Push"]
            pub const PUSH: u32 = 0x02;
            #[doc = "Clear"]
            pub const CLEAR: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Address Select"]
    pub mod LADSEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cache address"]
            pub const CACHE_ADDR: u32 = 0;
            #[doc = "Physical address"]
            pub const PHYS_ADDR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line access type"]
    pub mod LACC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Read"]
            pub const READ: u32 = 0;
            #[doc = "Write"]
            pub const WRITE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PC bus Cache search address register"]
pub mod PCCSAR {
    pub use crate::RW as access;
    #[doc = "Initiate Cache Line Command"]
    pub mod LGO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write: no effect. Read: no line command active."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
            pub const INIT_CMD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Physical Address"]
    pub mod PHYADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PC bus Cache read/write value register"]
pub mod PCCCVR {
    pub use crate::RW as access;
    #[doc = "Cache read/write Data"]
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
#[doc = "PS bus Cache control register"]
pub mod PSCCR {
    pub use crate::RW as access;
    #[doc = "Cache enable"]
    pub mod ENCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cache disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Cache enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Write Buffer"]
    pub mod ENWRBUF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write buffer disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Write buffer enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Forces all cacheable spaces to write through"]
    pub mod PSCR2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Does NOT force all cacheable spaces to write through"]
            pub const PSCR2_0: u32 = 0;
            #[doc = "Forces all cacheable spaces to write through"]
            pub const PSCR2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Forces no allocation on cache misses"]
    pub mod PSCR3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allocation on cache misses"]
            pub const PSCR3_0: u32 = 0;
            #[doc = "Forces no allocation on cache misses (must also have PSCR2 asserted)"]
            pub const PSCR3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Invalidate Way 0"]
    pub mod INVW0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "When setting the GO bit, invalidate all lines in way 0."]
            pub const INVW0: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Push Way 0"]
    pub mod PUSHW0 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "When setting the GO bit, push all modified lines in way 0"]
            pub const PUSHW0: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Invalidate Way 1"]
    pub mod INVW1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "When setting the GO bit, invalidate all lines in way 1"]
            pub const INVW1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Push Way 1"]
    pub mod PUSHW1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "When setting the GO bit, push all modified lines in way 1"]
            pub const PUSHW1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Initiate Cache Command"]
    pub mod GO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write: no effect. Read: no cache command active."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
            pub const INIT_CMD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PS bus Cache line control register"]
pub mod PSCLCR {
    pub use crate::RW as access;
    #[doc = "Initiate Cache Line Command"]
    pub mod LGO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write: no effect. Read: no line command active."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
            pub const INIT_CMD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Cache address"]
    pub mod CACHEADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Way select"]
    pub mod WSEL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Way 0"]
            pub const WAY0: u32 = 0;
            #[doc = "Way 1"]
            pub const WAY1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Tag/Data Select"]
    pub mod TDSEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Data"]
            pub const DATA: u32 = 0;
            #[doc = "Tag"]
            pub const TAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Command Initial Valid Bit"]
    pub mod LCIVB {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Command Initial Modified Bit"]
    pub mod LCIMB {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Command Way"]
    pub mod LCWAY {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Command"]
    pub mod LCMD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Search and read or write"]
            pub const SEARCH_RW: u32 = 0;
            #[doc = "Invalidate"]
            pub const INVALIDATE: u32 = 0x01;
            #[doc = "Push"]
            pub const PUSH: u32 = 0x02;
            #[doc = "Clear"]
            pub const CLEAR: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line Address Select"]
    pub mod LADSEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cache address"]
            pub const CACHE_ADDR: u32 = 0;
            #[doc = "Physical address"]
            pub const PHYS_ADDR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Line access type"]
    pub mod LACC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Read"]
            pub const READ: u32 = 0;
            #[doc = "Write"]
            pub const WRITE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PS bus Cache search address register"]
pub mod PSCSAR {
    pub use crate::RW as access;
    #[doc = "Initiate Cache Line Command"]
    pub mod LGO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write: no effect. Read: no line command active."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
            pub const INIT_CMD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Physical Address"]
    pub mod PHYADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PS bus Cache read/write value register"]
pub mod PSCCVR {
    pub use crate::RW as access;
    #[doc = "Cache read/write Data"]
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
