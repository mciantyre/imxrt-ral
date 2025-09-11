#[doc = "FlexSPI_FLR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Control"]
    pub MODULE_CONTROL: u32,
    #[doc = "Read Command Control"]
    pub READ_COMMAND_CONTROL: u32,
    #[doc = "Read Register Command Setting"]
    pub READ_REGISTER_COMMAND0: u32,
    #[doc = "Read Command 1 setting"]
    pub READ_COMMAND1: u32,
    #[doc = "Read Command 2 setting"]
    pub READ_COMMAND2: u32,
    #[doc = "Write Command Control"]
    pub WRITE_COMMAND_CONTROL: u32,
    #[doc = "Write Register Command 0 Setting"]
    pub WRITE_REGISTER_COMMAND0: u32,
    #[doc = "Write Command 1 Setting"]
    pub WRITE_COMMAND1: u32,
    #[doc = "Write Command 2 Setting"]
    pub WRITE_COMMAND2: u32,
    #[doc = "Read Write Command Address Base"]
    pub RW_COMMAND_BASE: u32,
    #[doc = "Command Suit 1 Range"]
    pub CMD1_RANGE: u32,
    #[doc = "Command Suit 2 Range"]
    pub CMD2_RANGE: u32,
    #[doc = "Module Status"]
    pub MODULE_STATUS: u32,
    #[doc = "SPI FLR interrupt"]
    pub MODULE_INT: u32,
    #[doc = "SPI FLR Interrupt Enable"]
    pub MODULE_INTEN: u32,
    #[doc = "SPI Mailbox control"]
    pub SPI_MAIL_CTRL: u32,
    #[doc = "SPI Mail Interrupt"]
    pub SPIMAIL: [u32; 9usize],
}
#[doc = "Module Control"]
pub mod MODULE_CONTROL {
    pub use crate::RW as access;
    #[doc = "Software Reset"]
    pub mod SWRESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Finished"]
            pub const FINISH: u32 = 0;
            #[doc = "Initiate"]
            pub const INITIATE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SPI IO Mode Control"]
    pub mod IOMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SDR*4"]
            pub const SDRX4: u32 = 0;
            #[doc = "SDR*8"]
            pub const SDRX8: u32 = 0x01;
            #[doc = "DDR*4"]
            pub const DDRX4: u32 = 0x02;
            #[doc = "DDR*8"]
            pub const DDRX8: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DQS Stop Feature"]
    pub mod DQSSTOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Chip Select Mask"]
    pub mod CSMASK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not masked"]
            pub const DISABLE: u32 = 0;
            #[doc = "Masked"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Block Read"]
    pub mod BLKREAD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allowed"]
            pub const ALLOW: u32 = 0;
            #[doc = "Blocked"]
            pub const BLOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Block Write"]
    pub mod BLKWRITE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allowed"]
            pub const ALLOW: u32 = 0;
            #[doc = "Blocked"]
            pub const BLOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Block Next Write Command"]
    pub mod BLKNXTWR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allowed"]
            pub const ALLOW: u32 = 0;
            #[doc = "Blocked"]
            pub const BLOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Block Next Read"]
    pub mod BLKNXTRD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allowed"]
            pub const ALLOW: u32 = 0;
            #[doc = "Blocked"]
            pub const BLOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow One More Write"]
    pub mod ALLOWONEWR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow One More Read"]
    pub mod ALLOWONERD {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AXI Command Range Base Update"]
    pub mod CMDRANGEBASEUPDATE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not updated"]
            pub const UNCHANGED: u32 = 0;
            #[doc = "Updated"]
            pub const CHANGED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Read Command Control"]
pub mod READ_COMMAND_CONTROL {
    pub use crate::RW as access;
    #[doc = "Read Fetch Size"]
    pub mod RDFETCHSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "256 bytes"]
            pub const SIZE256: u32 = 0;
            #[doc = "512 bytes"]
            pub const SIZE512: u32 = 0x01;
            #[doc = "1 KB"]
            pub const SIZE1K: u32 = 0x02;
            #[doc = "2 KB"]
            pub const SIZE2K: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Watermark Level"]
    pub mod RDWM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Outstanding"]
    pub mod RDOT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Send requests after previous leaders finish"]
            pub const DISABLE: u32 = 0;
            #[doc = "Send requests outstandingly"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Water Mark Enable"]
    pub mod WMEN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Read Register Command Setting"]
pub mod READ_REGISTER_COMMAND0 {
    pub use crate::RW as access;
    #[doc = "Read Register Dummy Cycles"]
    pub mod DUMMYCYCLES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Register Command Setting"]
    pub mod COMMANDSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Read Command 1 setting"]
pub mod READ_COMMAND1 {
    pub use crate::RW as access;
    #[doc = "Read Command 1 Dummy Cycles"]
    pub mod DUMMYCYCLES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Command 1 Setting"]
    pub mod COMMANDSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Read Command 2 setting"]
pub mod READ_COMMAND2 {
    pub use crate::RW as access;
    #[doc = "Read Command 2 Dummy Cycles"]
    pub mod DUMMYCYCLES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Command 2 Setting"]
    pub mod COMMANDSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Write Command Control"]
pub mod WRITE_COMMAND_CONTROL {
    pub use crate::RW as access;
    #[doc = "Write Watermark"]
    pub mod WRWM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "32 bytes"]
            pub const SIZE32: u32 = 0;
            #[doc = "64 bytes"]
            pub const SIZE64: u32 = 0x01;
            #[doc = "128 bytes"]
            pub const SIZE128: u32 = 0x02;
            #[doc = "256 bytes"]
            pub const SIZE256: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Write Register Command 0 Setting"]
pub mod WRITE_REGISTER_COMMAND0 {
    pub use crate::RW as access;
    #[doc = "Write Register Command Setting"]
    pub mod COMMANDSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Write Command 1 Setting"]
pub mod WRITE_COMMAND1 {
    pub use crate::RW as access;
    #[doc = "Write Command 1 Setting"]
    pub mod COMMANDSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Write Command 2 Setting"]
pub mod WRITE_COMMAND2 {
    pub use crate::RW as access;
    #[doc = "Write Command 2 Setting"]
    pub mod COMMANDSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Read Write Command Address Base"]
pub mod RW_COMMAND_BASE {
    pub use crate::RW as access;
    #[doc = "Address Base 1"]
    pub mod ADDRBASE1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address Base 2"]
    pub mod ADDRBASE2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Suit 1 Range"]
pub mod CMD1_RANGE {
    pub use crate::RW as access;
    #[doc = "Memory Range"]
    pub mod RANGE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x003f_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Suit 2 Range"]
pub mod CMD2_RANGE {
    pub use crate::RW as access;
    #[doc = "Memory Range"]
    pub mod RANGE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x003f_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Module Status"]
pub mod MODULE_STATUS {
    pub use crate::RO as access;
    #[doc = "Write in Progress"]
    pub mod WIP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not busy"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AXI Read Leader Idle"]
    pub mod AXIREADIDLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Busy"]
            pub const BUSY: u32 = 0;
            #[doc = "Idle"]
            pub const IDLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Register Read Write Idle"]
    pub mod REGRWIDLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Busy"]
            pub const BUSY: u32 = 0;
            #[doc = "Idle"]
            pub const IDLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEQ Controller Idle"]
    pub mod SEQIDLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Busy"]
            pub const BUSY: u32 = 0;
            #[doc = "Idle"]
            pub const IDLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Out-of-Range Counter"]
    pub mod RDOFR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Out-of-Range Counter"]
    pub mod WROFR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow AXI Read Access"]
    pub mod ALLOWAXIRD {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Denied"]
            pub const DENY: u32 = 0;
            #[doc = "Allowed"]
            pub const ALLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow AXI Write Access"]
    pub mod ALLOWAXIWR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Denied"]
            pub const DENY: u32 = 0;
            #[doc = "Allowed"]
            pub const ALLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SPI FLR interrupt"]
pub mod MODULE_INT {
    pub use crate::RW as access;
    #[doc = "Write Overflow Interrupt"]
    pub mod WOF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Did not occur"]
            pub const NONE: u32 = 0;
            #[doc = "Occurred"]
            pub const OVERFLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Underflow"]
    pub mod RUF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Did not occur"]
            pub const NONE: u32 = 0;
            #[doc = "Occurred"]
            pub const UNDERFLOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Error Command"]
    pub mod ERRCMD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not received"]
            pub const NONE: u32 = 0;
            #[doc = "Received"]
            pub const ERROR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SPI FLR Interrupt Enable"]
pub mod MODULE_INTEN {
    pub use crate::RW as access;
    #[doc = "Write Overflow Interrupt Enable"]
    pub mod WOFEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Underflow Interrupt Enable"]
    pub mod RUFEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Error Command Interrupt Enable"]
    pub mod ERRCMDEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SPI Mailbox control"]
pub mod SPI_MAIL_CTRL {
    pub use crate::RW as access;
    #[doc = "Clear Interrupt"]
    pub mod CLRINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not clear"]
            pub const NONE: u32 = 0;
            #[doc = "Clear"]
            pub const CLEAR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SPI Leader Interrupt Enable"]
    pub mod SPIINTEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SPI Mail Interrupt"]
pub mod SPIMAIL {
    pub use crate::RO as access;
    #[doc = "Specifies the use of MAILn for Inter-Processor Communication (IPC) protocol for SPI leader and follower communication"]
    pub mod MAILN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
