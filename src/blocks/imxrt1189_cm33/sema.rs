#[doc = "SEMA42"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Gate"]
    pub GATE3: u8,
    #[doc = "Gate"]
    pub GATE2: u8,
    #[doc = "Gate"]
    pub GATE1: u8,
    #[doc = "Gate"]
    pub GATE0: u8,
    #[doc = "Gate"]
    pub GATE7: u8,
    #[doc = "Gate"]
    pub GATE6: u8,
    #[doc = "Gate"]
    pub GATE5: u8,
    #[doc = "Gate"]
    pub GATE4: u8,
    #[doc = "Gate"]
    pub GATE11: u8,
    #[doc = "Gate"]
    pub GATE10: u8,
    #[doc = "Gate"]
    pub GATE9: u8,
    #[doc = "Gate"]
    pub GATE8: u8,
    #[doc = "Gate"]
    pub GATE15: u8,
    #[doc = "Gate"]
    pub GATE14: u8,
    #[doc = "Gate"]
    pub GATE13: u8,
    #[doc = "Gate"]
    pub GATE12: u8,
    #[doc = "Gate"]
    pub GATE19: u8,
    #[doc = "Gate"]
    pub GATE18: u8,
    #[doc = "Gate"]
    pub GATE17: u8,
    #[doc = "Gate"]
    pub GATE16: u8,
    #[doc = "Gate"]
    pub GATE23: u8,
    #[doc = "Gate"]
    pub GATE22: u8,
    #[doc = "Gate"]
    pub GATE21: u8,
    #[doc = "Gate"]
    pub GATE20: u8,
    #[doc = "Gate"]
    pub GATE27: u8,
    #[doc = "Gate"]
    pub GATE26: u8,
    #[doc = "Gate"]
    pub GATE25: u8,
    #[doc = "Gate"]
    pub GATE24: u8,
    #[doc = "Gate"]
    pub GATE31: u8,
    #[doc = "Gate"]
    pub GATE30: u8,
    #[doc = "Gate"]
    pub GATE29: u8,
    #[doc = "Gate"]
    pub GATE28: u8,
    #[doc = "Gate"]
    pub GATE35: u8,
    #[doc = "Gate"]
    pub GATE34: u8,
    #[doc = "Gate"]
    pub GATE33: u8,
    #[doc = "Gate"]
    pub GATE32: u8,
    #[doc = "Gate"]
    pub GATE39: u8,
    #[doc = "Gate"]
    pub GATE38: u8,
    #[doc = "Gate"]
    pub GATE37: u8,
    #[doc = "Gate"]
    pub GATE36: u8,
    #[doc = "Gate"]
    pub GATE43: u8,
    #[doc = "Gate"]
    pub GATE42: u8,
    #[doc = "Gate"]
    pub GATE41: u8,
    #[doc = "Gate"]
    pub GATE40: u8,
    #[doc = "Gate"]
    pub GATE47: u8,
    #[doc = "Gate"]
    pub GATE46: u8,
    #[doc = "Gate"]
    pub GATE45: u8,
    #[doc = "Gate"]
    pub GATE44: u8,
    #[doc = "Gate"]
    pub GATE51: u8,
    #[doc = "Gate"]
    pub GATE50: u8,
    #[doc = "Gate"]
    pub GATE49: u8,
    #[doc = "Gate"]
    pub GATE48: u8,
    #[doc = "Gate"]
    pub GATE55: u8,
    #[doc = "Gate"]
    pub GATE54: u8,
    #[doc = "Gate"]
    pub GATE53: u8,
    #[doc = "Gate"]
    pub GATE52: u8,
    #[doc = "Gate"]
    pub GATE59: u8,
    #[doc = "Gate"]
    pub GATE58: u8,
    #[doc = "Gate"]
    pub GATE57: u8,
    #[doc = "Gate"]
    pub GATE56: u8,
    #[doc = "Gate"]
    pub GATE63: u8,
    #[doc = "Gate"]
    pub GATE62: u8,
    #[doc = "Gate"]
    pub GATE61: u8,
    #[doc = "Gate"]
    pub GATE60: u8,
    _reserved0: [u8; 0x02],
    #[doc = "Reset Gate Read"]
    pub RSTGT_R: u16,
}
#[doc = "Gate"]
pub mod GATE3 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE2 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE1 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE0 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE7 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE6 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE5 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE4 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE11 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE10 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE9 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE8 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE15 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE14 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE13 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE12 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE19 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE18 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE17 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE16 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE23 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE22 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE21 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE20 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE27 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE26 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE25 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE24 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE31 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE30 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE29 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE28 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE35 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE34 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE33 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE32 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE39 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE38 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE37 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE36 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE43 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE42 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE41 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE40 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE47 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE46 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE45 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE44 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE51 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE50 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE49 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE48 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE55 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE54 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE53 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE52 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE59 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE58 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE57 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE56 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE63 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE62 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE61 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gate"]
pub mod GATE60 {
    pub use crate::RW as access;
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Reset Gate Read"]
pub mod RSTGT_R {
    pub use crate::RO as access;
    #[doc = "Reset Gate Number"]
    pub mod RSTGTN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reset Gate Domain"]
    pub mod RSTGMS {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reset Gate Finite State Machine"]
    pub mod RSTGSM {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Idle, waiting for the first data pattern write."]
            pub const IDLE: u16 = 0;
            #[doc = "Waiting for the second data pattern write"]
            pub const WAITING: u16 = 0x01;
            #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state."]
            pub const TWO_WRITE_DONE: u16 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
