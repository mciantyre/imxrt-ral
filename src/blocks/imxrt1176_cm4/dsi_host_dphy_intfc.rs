#[doc = "DSI HOST DPHY INTFC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "PD_TX"]
    pub PD_TX: u32,
    #[doc = "M_PRG_HS_PREPARE"]
    pub M_PRG_HS_PREPARE: u32,
    #[doc = "MC_PRG_HS_PREPARE"]
    pub MC_PRG_HS_PREPARE: u32,
    #[doc = "M_PRG_HS_ZERO"]
    pub M_PRG_HS_ZERO: u32,
    #[doc = "MC_PRG_HS_ZERO"]
    pub MC_PRG_HS_ZERO: u32,
    #[doc = "M_PRG_HS_TRAIL"]
    pub M_PRG_HS_TRAIL: u32,
    #[doc = "MC_PRG_HS_TRAIL"]
    pub MC_PRG_HS_TRAIL: u32,
    #[doc = "PD_PLL"]
    pub PD_PLL: u32,
    #[doc = "TST"]
    pub TST: u32,
    #[doc = "CN"]
    pub CN: u32,
    #[doc = "CM"]
    pub CM: u32,
    #[doc = "CO"]
    pub CO: u32,
    #[doc = "LOCK"]
    pub LOCK: u32,
    #[doc = "LOCK_BYP"]
    pub LOCK_BYP: u32,
    #[doc = "TX_RCAL"]
    pub TX_RCAL: u32,
    #[doc = "AUTO_PD_EN"]
    pub AUTO_PD_EN: u32,
    #[doc = "RXLPRP"]
    pub RXLPRP: u32,
    #[doc = "RXCDRP"]
    pub RXCDRP: u32,
}
#[doc = "PD_TX"]
pub mod PD_TX {
    pub use crate::RW as access;
    #[doc = "Power Down input for D-PHY"]
    pub mod PD_TX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Power Up"]
            pub const PD_TX_0: u32 = 0;
            #[doc = "Power Down"]
            pub const PD_TX_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "M_PRG_HS_PREPARE"]
pub mod M_PRG_HS_PREPARE {
    pub use crate::RW as access;
    #[doc = "DPHY m_PRG_HS_PREPARE input"]
    pub mod M_PRG_HS_PREPARE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "MC_PRG_HS_PREPARE"]
pub mod MC_PRG_HS_PREPARE {
    pub use crate::RW as access;
    #[doc = "DPHY mc_PRG_HS_PREPARE input"]
    pub mod MC_PRG_HS_PREPARE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "M_PRG_HS_ZERO"]
pub mod M_PRG_HS_ZERO {
    pub use crate::RW as access;
    #[doc = "DPHY m_PRG_HS_ZERO input"]
    pub mod M_PRG_HS_ZERO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "MC_PRG_HS_ZERO"]
pub mod MC_PRG_HS_ZERO {
    pub use crate::RW as access;
    #[doc = "DPHY mc_PRG_HS_ZERO input"]
    pub mod MC_PRG_HS_ZERO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "M_PRG_HS_TRAIL"]
pub mod M_PRG_HS_TRAIL {
    pub use crate::RW as access;
    #[doc = "DPHY m_PRG_HS_TRAIL input"]
    pub mod M_PRG_HS_TRAIL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "MC_PRG_HS_TRAIL"]
pub mod MC_PRG_HS_TRAIL {
    pub use crate::RW as access;
    #[doc = "DPHY mc_PRG_HS_TRAIL input"]
    pub mod MC_PRG_HS_TRAIL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PD_PLL"]
pub mod PD_PLL {
    pub use crate::RW as access;
    #[doc = "Power-down signal"]
    pub mod PD_PLL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Power up PLL"]
            pub const PD_PLL_0: u32 = 0;
            #[doc = "Power down PLL"]
            pub const PD_PLL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TST"]
pub mod TST {
    pub use crate::RW as access;
    #[doc = "Test"]
    pub mod TST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CN"]
pub mod CN {
    pub use crate::RW as access;
    #[doc = "Control N divider"]
    pub mod CN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM"]
pub mod CM {
    pub use crate::RW as access;
    #[doc = "Control M divider"]
    pub mod CM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CO"]
pub mod CO {
    pub use crate::RW as access;
    #[doc = "Control O divider"]
    pub mod CO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Divide by 1"]
            pub const CO_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const CO_1: u32 = 0x01;
            #[doc = "Divide by 4"]
            pub const CO_2: u32 = 0x02;
            #[doc = "Divide by 8"]
            pub const CO_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LOCK"]
pub mod LOCK {
    pub use crate::RO as access;
    #[doc = "Lock Detect output"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL not locked"]
            pub const LOCK_0: u32 = 0;
            #[doc = "PLL has achieved frequency lock"]
            pub const LOCK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LOCK_BYP"]
pub mod LOCK_BYP {
    pub use crate::RW as access;
    #[doc = "DPHY LOCK_BYP input"]
    pub mod LOCK_BYP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL LOCK signal will gate TxByteClkHS clock"]
            pub const GATE: u32 = 0;
            #[doc = "PLL LOCK signal will not gate TxByteClkHS clock, CIL based counter will be used to gate the TxByteClkHS"]
            pub const NOGATE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TX_RCAL"]
pub mod TX_RCAL {
    pub use crate::RW as access;
    #[doc = "On-chip termination control bits for manual calibration of HS-TX"]
    pub mod TX_RCAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "20% higher than mid-range. Highest impedance setting"]
            pub const TX_RCAL_0: u32 = 0;
            #[doc = "Mid-range impedance setting (default)"]
            pub const TX_RCAL_1: u32 = 0x01;
            #[doc = "15% lower than mid-range"]
            pub const TX_RCAL_2: u32 = 0x02;
            #[doc = "25% lower than mid-range. Lowest impedance setting"]
            pub const TX_RCAL_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "AUTO_PD_EN"]
pub mod AUTO_PD_EN {
    pub use crate::RW as access;
    #[doc = "DPHY AUTO_PD_EN input"]
    pub mod AUTO_PD_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Inactive lanes are powered up and driving LP11"]
            pub const PWR_UP: u32 = 0;
            #[doc = "inactive lanes are powered down"]
            pub const PWR_DWN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "RXLPRP"]
pub mod RXLPRP {
    pub use crate::RW as access;
    #[doc = "DPHY RXLPRP input"]
    pub mod RXLPRP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "RXCDRP"]
pub mod RXCDRP {
    pub use crate::RW as access;
    #[doc = "DPHY RXCDRP input"]
    pub mod RXCDRP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "344mV"]
            pub const RXCDRP_0: u32 = 0;
            #[doc = "325mV (Default)"]
            pub const RXCDRP_1: u32 = 0x01;
            #[doc = "307mV"]
            pub const RXCDRP_2: u32 = 0x02;
            #[doc = "Invalid"]
            pub const RXCDRP_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
