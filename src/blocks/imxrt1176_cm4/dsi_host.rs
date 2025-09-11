#[doc = "DSI HOST"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "CFG_NUM_LANES"]
    pub CFG_NUM_LANES: u32,
    #[doc = "CFG_NONCONTINUOUS_CLK"]
    pub CFG_NONCONTINUOUS_CLK: u32,
    #[doc = "CFG_T_PRE"]
    pub CFG_T_PRE: u32,
    #[doc = "CFG_T_POST"]
    pub CFG_T_POST: u32,
    #[doc = "CFG_TX_GAP"]
    pub CFG_TX_GAP: u32,
    #[doc = "CFG_AUTOINSERT_ETOP"]
    pub CFG_AUTOINSERT_EOTP: u32,
    #[doc = "CFG_EXTRA_CMDS_AFTER_ETOP"]
    pub CFG_EXTRA_CMDS_AFTER_EOTP: u32,
    #[doc = "CFG_HTX_TO_COUNT"]
    pub CFG_HTX_TO_COUNT: u32,
    #[doc = "CFG_LRX_H_TO_COUNT"]
    pub CFG_LRX_H_TO_COUNT: u32,
    #[doc = "CFG_BTA_H_TO_COUNT"]
    pub CFG_BTA_H_TO_COUNT: u32,
    #[doc = "CFG_TWAKEUP"]
    pub CFG_TWAKEUP: u32,
    #[doc = "CFG_STATUS_OUT"]
    pub CFG_STATUS_OUT: u32,
    #[doc = "RX_ERROR_STATUS"]
    pub RX_ERROR_STATUS: u32,
}
#[doc = "CFG_NUM_LANES"]
pub mod CFG_NUM_LANES {
    pub use crate::RW as access;
    #[doc = "Sets the number of active lanes that are to be used for transmitting data."]
    pub mod NUM_LANES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1 lane"]
            pub const NUM_LANES_0: u32 = 0;
            #[doc = "2 lanes"]
            pub const NUM_LANES_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_NONCONTINUOUS_CLK"]
pub mod CFG_NONCONTINUOUS_CLK {
    pub use crate::RW as access;
    #[doc = "Sets the Host Controller into non-continuous MIPI clock mode. When in non-continuous clock mode, the high speed clock will transition into low power mode between transmissions."]
    pub mod CLK_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Continuous high speed clock"]
            pub const CLK_MODE_0: u32 = 0;
            #[doc = "Non-Continuous high speed clock"]
            pub const CLK_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_T_PRE"]
pub mod CFG_T_PRE {
    pub use crate::RW as access;
    #[doc = "Sets the number of byte clock periods ('clk_byte' input) that the controller will wait after enabling the clock lane for HS operation before enabling the data lanes for HS operation. This setting represents the TCLK-PRE DPHY timing parameter. The minimum value for this port is 1."]
    pub mod NUM_PERIODS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_T_POST"]
pub mod CFG_T_POST {
    pub use crate::RW as access;
    #[doc = "Sets the number of byte clock periods ('clk_byte' input) to wait before putting the clock lane into LP mode after the data lanes have been detected to be in Stop State. This setting represents the DPHY timing parameters TLPX + TCLK-PREPARE + TCLK-ZERO + TCLK-PRE requirement for the clock lane before the data lane is allowed to change from LP11 to start a high speed transmission. The minimum value for this port is 1."]
    pub mod NUM_PERIODS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_TX_GAP"]
pub mod CFG_TX_GAP {
    pub use crate::RW as access;
    #[doc = "Sets the number of byte clock periods ('clk_byte' input) that the controller will wait after the clock lane has been put into LP mode before enabling the clock lane for HS mode again. This setting represents the THS-EXIT DPHY timing parameter. The minimum value for this port is 1."]
    pub mod NUM_PERIODS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_AUTOINSERT_ETOP"]
pub mod CFG_AUTOINSERT_EOTP {
    pub use crate::RW as access;
    #[doc = "Enables the Host Controller to automatically insert an EoTp short packet when switching from HS to LP mode."]
    pub mod AUTOINSERT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "EoTp is not automatically inserted"]
            pub const NOT_AUTO: u32 = 0;
            #[doc = "EoTp is automatically inserted"]
            pub const AUTO: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_EXTRA_CMDS_AFTER_ETOP"]
pub mod CFG_EXTRA_CMDS_AFTER_EOTP {
    pub use crate::RW as access;
    #[doc = "Configures the DSI Host Controller to send extra End Of Transmission Packets after the end of a packet. The value is the number of extra EOTP packets sent."]
    pub mod EXTRA_EOTP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_HTX_TO_COUNT"]
pub mod CFG_HTX_TO_COUNT {
    pub use crate::RW as access;
    #[doc = "Sets the value of the DSI Host High Speed TX timeout count in clk_byte clock periods that once reached will initiate a timeout error and follow the recovery procedure documented in the DSI specification."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_LRX_H_TO_COUNT"]
pub mod CFG_LRX_H_TO_COUNT {
    pub use crate::RW as access;
    #[doc = "Sets the value of the DSI Host low power RX timeout count in clk_byte clock periods that once reached will initiate a timeout error and follow the recovery procedure documented in the DSI specification."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_BTA_H_TO_COUNT"]
pub mod CFG_BTA_H_TO_COUNT {
    pub use crate::RW as access;
    #[doc = "Sets the value of the DSI Host Bus Turn Around (BTA) timeout in clk_byte clock periods that once reached will initiate a timeout error."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_TWAKEUP"]
pub mod CFG_TWAKEUP {
    pub use crate::RW as access;
    #[doc = "DPHY Twakeup timing parameter. Sets the number of clk_esc clock periods to keep a clock or data lane in Mark-1 state after exiting ULPS. The MIPI DPHY spec requires a minimum of 1ms in Mark-1 state after leaving ULPS."]
    pub mod NUM_PERIODS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CFG_STATUS_OUT"]
pub mod CFG_STATUS_OUT {
    pub use crate::RO as access;
    #[doc = "Status Register"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "RX_ERROR_STATUS"]
pub mod RX_ERROR_STATUS {
    pub use crate::RO as access;
    #[doc = "Status Register for Host receive error detection, ECC errors, CRC errors and for timeout indicators"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
