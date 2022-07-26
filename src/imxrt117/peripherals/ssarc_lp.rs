#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SSARC Registers
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Descriptor Control0 0 Register
pub mod DESC_CTRL0_0 {

    /// Start index
    pub mod START {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End index
    pub mod END {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (10 bits: 0x3ff << 10)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Save Order
    pub mod SV_ORDER {
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

            /// 0b0: Descriptors within the group are processed from start to end
            pub const SV_START_END: u32 = 0b0;

            /// 0b1: Descriptors within the group are processed from end to start
            pub const SV_END_START: u32 = 0b1;
        }
    }

    /// Restore order
    pub mod RT_ORDER {
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

            /// 0b0: Descriptors within the group are processed from start to end
            pub const RT_START_END: u32 = 0b0;

            /// 0b1: Descriptors within the group are processed from end to start
            pub const RT_END_START: u32 = 0b1;
        }
    }
}

/// Descriptor Control1 0 Register
pub mod DESC_CTRL1_0 {

    /// Software trigger save
    pub mod SW_TRIG_SV {
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

            /// 0b0: No software save request/software restore request complete
            pub const REQ_NO: u32 = 0b0;

            /// 0b1: Request a software save operation/software restore operation in progress
            pub const REQ_YES: u32 = 0b1;
        }
    }

    /// Software trigger restore
    pub mod SW_TRIG_RT {
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

            /// 0b0: No software restore request/software restore request complete
            pub const REQ_NO: u32 = 0b0;

            /// 0b1: Request a software restore operation/software restore operation in progress
            pub const REQ_YES: u32 = 0b1;
        }
    }

    /// This field describes the mapping (0-7) to external request signals from different domains
    pub mod POWER_DOMAIN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: PGMC_BPC0
            pub const DOMAIN0: u32 = 0b000;

            /// 0b001: PGMC_BPC1
            pub const DOMAIN1: u32 = 0b001;

            /// 0b010: PGMC_BPC2
            pub const DOMAIN2: u32 = 0b010;

            /// 0b011: PGMC_BPC3
            pub const DOMAIN3: u32 = 0b011;

            /// 0b100: PGMC_BPC4
            pub const DOMAIN4: u32 = 0b100;

            /// 0b101: PGMC_BPC5
            pub const DOMAIN5: u32 = 0b101;

            /// 0b110: PGMC_BPC6
            pub const DOMAIN6: u32 = 0b110;

            /// 0b111: PGMC_BPC7
            pub const DOMAIN7: u32 = 0b111;
        }
    }

    /// Group Enable
    pub mod GP_EN {
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

            /// 0b0: Group disabled
            pub const GP_DIS: u32 = 0b0;

            /// 0b1: Group enabled
            pub const GP_EN: u32 = 0b1;
        }
    }

    /// Save Priority
    pub mod SV_PRIORITY {
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

    /// Restore Priority
    pub mod RT_PRIORITY {
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

    /// CPU Domain
    pub mod CPUD {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read Lock
    pub mod RL {
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

            /// 0b0: Group is unlocked (read access allowed)
            pub const R_UNLOCK: u32 = 0b0;

            /// 0b1: Group is locked (read access not allowed)
            pub const R_LOCK: u32 = 0b1;
        }
    }

    /// Write Lock
    pub mod WL {
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

            /// 0b0: Group is unlocked (write access allowed)
            pub const W_UNLOCK: u32 = 0b0;

            /// 0b1: Group is locked (write access not allowed)
            pub const W_LOCK: u32 = 0b1;
        }
    }

    /// Domain lock
    pub mod DL {
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

            /// 0b0: Unlock
            pub const D_UNLOCK: u32 = 0b0;

            /// 0b1: Lock
            pub const D_LOCK: u32 = 0b1;
        }
    }
}

/// Descriptor Address Up 0 Register
pub mod DESC_ADDR_UP_0 {

    /// Address field (High)
    pub mod ADDR_UP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Descriptor Address Down 0 Register
pub mod DESC_ADDR_DOWN_0 {

    /// Address field (Low)
    pub mod ADDR_DOWN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Descriptor Control0 1 Register
pub mod DESC_CTRL0_1 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 1 Register
pub mod DESC_CTRL1_1 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 1 Register
pub mod DESC_ADDR_UP_1 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 1 Register
pub mod DESC_ADDR_DOWN_1 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 2 Register
pub mod DESC_CTRL0_2 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 2 Register
pub mod DESC_CTRL1_2 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 2 Register
pub mod DESC_ADDR_UP_2 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 2 Register
pub mod DESC_ADDR_DOWN_2 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 3 Register
pub mod DESC_CTRL0_3 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 3 Register
pub mod DESC_CTRL1_3 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 3 Register
pub mod DESC_ADDR_UP_3 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 3 Register
pub mod DESC_ADDR_DOWN_3 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 4 Register
pub mod DESC_CTRL0_4 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 4 Register
pub mod DESC_CTRL1_4 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 4 Register
pub mod DESC_ADDR_UP_4 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 4 Register
pub mod DESC_ADDR_DOWN_4 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 5 Register
pub mod DESC_CTRL0_5 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 5 Register
pub mod DESC_CTRL1_5 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 5 Register
pub mod DESC_ADDR_UP_5 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 5 Register
pub mod DESC_ADDR_DOWN_5 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 6 Register
pub mod DESC_CTRL0_6 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 6 Register
pub mod DESC_CTRL1_6 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 6 Register
pub mod DESC_ADDR_UP_6 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 6 Register
pub mod DESC_ADDR_DOWN_6 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 7 Register
pub mod DESC_CTRL0_7 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 7 Register
pub mod DESC_CTRL1_7 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 7 Register
pub mod DESC_ADDR_UP_7 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 7 Register
pub mod DESC_ADDR_DOWN_7 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 8 Register
pub mod DESC_CTRL0_8 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 8 Register
pub mod DESC_CTRL1_8 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 8 Register
pub mod DESC_ADDR_UP_8 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 8 Register
pub mod DESC_ADDR_DOWN_8 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 9 Register
pub mod DESC_CTRL0_9 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 9 Register
pub mod DESC_CTRL1_9 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 9 Register
pub mod DESC_ADDR_UP_9 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 9 Register
pub mod DESC_ADDR_DOWN_9 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 10 Register
pub mod DESC_CTRL0_10 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 10 Register
pub mod DESC_CTRL1_10 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 10 Register
pub mod DESC_ADDR_UP_10 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 10 Register
pub mod DESC_ADDR_DOWN_10 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 11 Register
pub mod DESC_CTRL0_11 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 11 Register
pub mod DESC_CTRL1_11 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 11 Register
pub mod DESC_ADDR_UP_11 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 11 Register
pub mod DESC_ADDR_DOWN_11 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 12 Register
pub mod DESC_CTRL0_12 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 12 Register
pub mod DESC_CTRL1_12 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 12 Register
pub mod DESC_ADDR_UP_12 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 12 Register
pub mod DESC_ADDR_DOWN_12 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 13 Register
pub mod DESC_CTRL0_13 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 13 Register
pub mod DESC_CTRL1_13 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 13 Register
pub mod DESC_ADDR_UP_13 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 13 Register
pub mod DESC_ADDR_DOWN_13 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 14 Register
pub mod DESC_CTRL0_14 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 14 Register
pub mod DESC_CTRL1_14 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 14 Register
pub mod DESC_ADDR_UP_14 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 14 Register
pub mod DESC_ADDR_DOWN_14 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Descriptor Control0 15 Register
pub mod DESC_CTRL0_15 {
    pub use super::DESC_CTRL0_0::END;
    pub use super::DESC_CTRL0_0::RT_ORDER;
    pub use super::DESC_CTRL0_0::START;
    pub use super::DESC_CTRL0_0::SV_ORDER;
}

/// Descriptor Control1 15 Register
pub mod DESC_CTRL1_15 {
    pub use super::DESC_CTRL1_0::CPUD;
    pub use super::DESC_CTRL1_0::DL;
    pub use super::DESC_CTRL1_0::GP_EN;
    pub use super::DESC_CTRL1_0::POWER_DOMAIN;
    pub use super::DESC_CTRL1_0::RL;
    pub use super::DESC_CTRL1_0::RT_PRIORITY;
    pub use super::DESC_CTRL1_0::SV_PRIORITY;
    pub use super::DESC_CTRL1_0::SW_TRIG_RT;
    pub use super::DESC_CTRL1_0::SW_TRIG_SV;
    pub use super::DESC_CTRL1_0::WL;
}

/// Descriptor Address Up 15 Register
pub mod DESC_ADDR_UP_15 {
    pub use super::DESC_ADDR_UP_0::ADDR_UP;
}

/// Descriptor Address Down 15 Register
pub mod DESC_ADDR_DOWN_15 {
    pub use super::DESC_ADDR_DOWN_0::ADDR_DOWN;
}

/// Control Register
pub mod CTRL {

    /// Save/Restore request disable
    pub mod DIS_HW_REQ {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PGMC save/restore requests enabled
            pub const ENABLE_PGMC: u32 = 0b0;

            /// 0b1: PGMC save/restore requests disabled
            pub const DIS_PGMC: u32 = 0b1;
        }
    }

    /// Software reset
    pub mod SW_RESET {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Status Register
pub mod INT_STATUS {

    /// Error Index
    pub mod ERR_INDEX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHB Bus response field
    pub mod AHB_RESP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Group Conflict field
    pub mod GROUP_CONFLICT {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No group conflict error
            pub const GRP_CONFLICT_ERR_NO: u32 = 0b0;

            /// 0b1: A group conflict error has occurred
            pub const GRP_CONFLICT_ERR: u32 = 0b1;
        }
    }

    /// Timeout field
    pub mod TIMEOUT {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No timeout event
            pub const ERR_INDEX_ERR_NO: u32 = 0b0;

            /// 0b1: A timeout event has occurred
            pub const ERR_INDEX_ERR: u32 = 0b1;
        }
    }

    /// Software Request Done
    pub mod SW_REQ_DONE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No software triggered requests or software triggered request still in progress
            pub const SW_REQ_ERR_A: u32 = 0b0;

            /// 0b1: Atleast one software triggered has been complete
            pub const SW_REQ_ERR: u32 = 0b1;
        }
    }

    /// AHB Error field
    pub mod AHB_ERR {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No AHB error
            pub const AHB_ERRNO: u32 = 0b0;

            /// 0b1: An AHB error has occurred
            pub const AHB_ERR: u32 = 0b1;
        }
    }

    /// Address Error field
    pub mod ADDR_ERR {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No address error
            pub const ADDERR_ERRNO: u32 = 0b0;

            /// 0b1: An address error has occurred
            pub const ADDERR_ERR: u32 = 0b1;
        }
    }
}

/// HP Timeout Register
pub mod HP_TIMEOUT {

    /// Time out value
    pub mod TIMEOUT_VALUE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Hardware Request Pending Register
pub mod HW_GROUP_PENDING {

    /// This field indicates which groups are pending for save from hardware request
    pub mod HW_SAVE_PENDING {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field indicates which groups are pending for restore from hardware request
    pub mod HW_RESTORE_PENDING {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Software Request Pending Register
pub mod SW_GROUP_PENDING {

    /// This field indicates which groups are pending for save from software request
    pub mod SW_SAVE_PENDING {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field indicates which groups are pending for restore from software request
    pub mod SW_RESTORE_PENDING {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Descriptor Control0 0 Register
    pub DESC_CTRL0_0: RWRegister<u32>,

    /// Descriptor Control1 0 Register
    pub DESC_CTRL1_0: RWRegister<u32>,

    /// Descriptor Address Up 0 Register
    pub DESC_ADDR_UP_0: RWRegister<u32>,

    /// Descriptor Address Down 0 Register
    pub DESC_ADDR_DOWN_0: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// Descriptor Control0 1 Register
    pub DESC_CTRL0_1: RWRegister<u32>,

    /// Descriptor Control1 1 Register
    pub DESC_CTRL1_1: RWRegister<u32>,

    /// Descriptor Address Up 1 Register
    pub DESC_ADDR_UP_1: RWRegister<u32>,

    /// Descriptor Address Down 1 Register
    pub DESC_ADDR_DOWN_1: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// Descriptor Control0 2 Register
    pub DESC_CTRL0_2: RWRegister<u32>,

    /// Descriptor Control1 2 Register
    pub DESC_CTRL1_2: RWRegister<u32>,

    /// Descriptor Address Up 2 Register
    pub DESC_ADDR_UP_2: RWRegister<u32>,

    /// Descriptor Address Down 2 Register
    pub DESC_ADDR_DOWN_2: RWRegister<u32>,

    _reserved3: [u32; 4],

    /// Descriptor Control0 3 Register
    pub DESC_CTRL0_3: RWRegister<u32>,

    /// Descriptor Control1 3 Register
    pub DESC_CTRL1_3: RWRegister<u32>,

    /// Descriptor Address Up 3 Register
    pub DESC_ADDR_UP_3: RWRegister<u32>,

    /// Descriptor Address Down 3 Register
    pub DESC_ADDR_DOWN_3: RWRegister<u32>,

    _reserved4: [u32; 4],

    /// Descriptor Control0 4 Register
    pub DESC_CTRL0_4: RWRegister<u32>,

    /// Descriptor Control1 4 Register
    pub DESC_CTRL1_4: RWRegister<u32>,

    /// Descriptor Address Up 4 Register
    pub DESC_ADDR_UP_4: RWRegister<u32>,

    /// Descriptor Address Down 4 Register
    pub DESC_ADDR_DOWN_4: RWRegister<u32>,

    _reserved5: [u32; 4],

    /// Descriptor Control0 5 Register
    pub DESC_CTRL0_5: RWRegister<u32>,

    /// Descriptor Control1 5 Register
    pub DESC_CTRL1_5: RWRegister<u32>,

    /// Descriptor Address Up 5 Register
    pub DESC_ADDR_UP_5: RWRegister<u32>,

    /// Descriptor Address Down 5 Register
    pub DESC_ADDR_DOWN_5: RWRegister<u32>,

    _reserved6: [u32; 4],

    /// Descriptor Control0 6 Register
    pub DESC_CTRL0_6: RWRegister<u32>,

    /// Descriptor Control1 6 Register
    pub DESC_CTRL1_6: RWRegister<u32>,

    /// Descriptor Address Up 6 Register
    pub DESC_ADDR_UP_6: RWRegister<u32>,

    /// Descriptor Address Down 6 Register
    pub DESC_ADDR_DOWN_6: RWRegister<u32>,

    _reserved7: [u32; 4],

    /// Descriptor Control0 7 Register
    pub DESC_CTRL0_7: RWRegister<u32>,

    /// Descriptor Control1 7 Register
    pub DESC_CTRL1_7: RWRegister<u32>,

    /// Descriptor Address Up 7 Register
    pub DESC_ADDR_UP_7: RWRegister<u32>,

    /// Descriptor Address Down 7 Register
    pub DESC_ADDR_DOWN_7: RWRegister<u32>,

    _reserved8: [u32; 4],

    /// Descriptor Control0 8 Register
    pub DESC_CTRL0_8: RWRegister<u32>,

    /// Descriptor Control1 8 Register
    pub DESC_CTRL1_8: RWRegister<u32>,

    /// Descriptor Address Up 8 Register
    pub DESC_ADDR_UP_8: RWRegister<u32>,

    /// Descriptor Address Down 8 Register
    pub DESC_ADDR_DOWN_8: RWRegister<u32>,

    _reserved9: [u32; 4],

    /// Descriptor Control0 9 Register
    pub DESC_CTRL0_9: RWRegister<u32>,

    /// Descriptor Control1 9 Register
    pub DESC_CTRL1_9: RWRegister<u32>,

    /// Descriptor Address Up 9 Register
    pub DESC_ADDR_UP_9: RWRegister<u32>,

    /// Descriptor Address Down 9 Register
    pub DESC_ADDR_DOWN_9: RWRegister<u32>,

    _reserved10: [u32; 4],

    /// Descriptor Control0 10 Register
    pub DESC_CTRL0_10: RWRegister<u32>,

    /// Descriptor Control1 10 Register
    pub DESC_CTRL1_10: RWRegister<u32>,

    /// Descriptor Address Up 10 Register
    pub DESC_ADDR_UP_10: RWRegister<u32>,

    /// Descriptor Address Down 10 Register
    pub DESC_ADDR_DOWN_10: RWRegister<u32>,

    _reserved11: [u32; 4],

    /// Descriptor Control0 11 Register
    pub DESC_CTRL0_11: RWRegister<u32>,

    /// Descriptor Control1 11 Register
    pub DESC_CTRL1_11: RWRegister<u32>,

    /// Descriptor Address Up 11 Register
    pub DESC_ADDR_UP_11: RWRegister<u32>,

    /// Descriptor Address Down 11 Register
    pub DESC_ADDR_DOWN_11: RWRegister<u32>,

    _reserved12: [u32; 4],

    /// Descriptor Control0 12 Register
    pub DESC_CTRL0_12: RWRegister<u32>,

    /// Descriptor Control1 12 Register
    pub DESC_CTRL1_12: RWRegister<u32>,

    /// Descriptor Address Up 12 Register
    pub DESC_ADDR_UP_12: RWRegister<u32>,

    /// Descriptor Address Down 12 Register
    pub DESC_ADDR_DOWN_12: RWRegister<u32>,

    _reserved13: [u32; 4],

    /// Descriptor Control0 13 Register
    pub DESC_CTRL0_13: RWRegister<u32>,

    /// Descriptor Control1 13 Register
    pub DESC_CTRL1_13: RWRegister<u32>,

    /// Descriptor Address Up 13 Register
    pub DESC_ADDR_UP_13: RWRegister<u32>,

    /// Descriptor Address Down 13 Register
    pub DESC_ADDR_DOWN_13: RWRegister<u32>,

    _reserved14: [u32; 4],

    /// Descriptor Control0 14 Register
    pub DESC_CTRL0_14: RWRegister<u32>,

    /// Descriptor Control1 14 Register
    pub DESC_CTRL1_14: RWRegister<u32>,

    /// Descriptor Address Up 14 Register
    pub DESC_ADDR_UP_14: RWRegister<u32>,

    /// Descriptor Address Down 14 Register
    pub DESC_ADDR_DOWN_14: RWRegister<u32>,

    _reserved15: [u32; 4],

    /// Descriptor Control0 15 Register
    pub DESC_CTRL0_15: RWRegister<u32>,

    /// Descriptor Control1 15 Register
    pub DESC_CTRL1_15: RWRegister<u32>,

    /// Descriptor Address Up 15 Register
    pub DESC_ADDR_UP_15: RWRegister<u32>,

    /// Descriptor Address Down 15 Register
    pub DESC_ADDR_DOWN_15: RWRegister<u32>,

    _reserved16: [u32; 4],

    /// Control Register
    pub CTRL: RWRegister<u32>,

    /// Interrupt Status Register
    pub INT_STATUS: RWRegister<u32>,

    _reserved17: [u32; 1],

    /// HP Timeout Register
    pub HP_TIMEOUT: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// Hardware Request Pending Register
    pub HW_GROUP_PENDING: RORegister<u32>,

    /// Software Request Pending Register
    pub SW_GROUP_PENDING: RORegister<u32>,
}
pub struct ResetValues {
    pub DESC_CTRL0_0: u32,
    pub DESC_CTRL1_0: u32,
    pub DESC_ADDR_UP_0: u32,
    pub DESC_ADDR_DOWN_0: u32,
    pub DESC_CTRL0_1: u32,
    pub DESC_CTRL1_1: u32,
    pub DESC_ADDR_UP_1: u32,
    pub DESC_ADDR_DOWN_1: u32,
    pub DESC_CTRL0_2: u32,
    pub DESC_CTRL1_2: u32,
    pub DESC_ADDR_UP_2: u32,
    pub DESC_ADDR_DOWN_2: u32,
    pub DESC_CTRL0_3: u32,
    pub DESC_CTRL1_3: u32,
    pub DESC_ADDR_UP_3: u32,
    pub DESC_ADDR_DOWN_3: u32,
    pub DESC_CTRL0_4: u32,
    pub DESC_CTRL1_4: u32,
    pub DESC_ADDR_UP_4: u32,
    pub DESC_ADDR_DOWN_4: u32,
    pub DESC_CTRL0_5: u32,
    pub DESC_CTRL1_5: u32,
    pub DESC_ADDR_UP_5: u32,
    pub DESC_ADDR_DOWN_5: u32,
    pub DESC_CTRL0_6: u32,
    pub DESC_CTRL1_6: u32,
    pub DESC_ADDR_UP_6: u32,
    pub DESC_ADDR_DOWN_6: u32,
    pub DESC_CTRL0_7: u32,
    pub DESC_CTRL1_7: u32,
    pub DESC_ADDR_UP_7: u32,
    pub DESC_ADDR_DOWN_7: u32,
    pub DESC_CTRL0_8: u32,
    pub DESC_CTRL1_8: u32,
    pub DESC_ADDR_UP_8: u32,
    pub DESC_ADDR_DOWN_8: u32,
    pub DESC_CTRL0_9: u32,
    pub DESC_CTRL1_9: u32,
    pub DESC_ADDR_UP_9: u32,
    pub DESC_ADDR_DOWN_9: u32,
    pub DESC_CTRL0_10: u32,
    pub DESC_CTRL1_10: u32,
    pub DESC_ADDR_UP_10: u32,
    pub DESC_ADDR_DOWN_10: u32,
    pub DESC_CTRL0_11: u32,
    pub DESC_CTRL1_11: u32,
    pub DESC_ADDR_UP_11: u32,
    pub DESC_ADDR_DOWN_11: u32,
    pub DESC_CTRL0_12: u32,
    pub DESC_CTRL1_12: u32,
    pub DESC_ADDR_UP_12: u32,
    pub DESC_ADDR_DOWN_12: u32,
    pub DESC_CTRL0_13: u32,
    pub DESC_CTRL1_13: u32,
    pub DESC_ADDR_UP_13: u32,
    pub DESC_ADDR_DOWN_13: u32,
    pub DESC_CTRL0_14: u32,
    pub DESC_CTRL1_14: u32,
    pub DESC_ADDR_UP_14: u32,
    pub DESC_ADDR_DOWN_14: u32,
    pub DESC_CTRL0_15: u32,
    pub DESC_CTRL1_15: u32,
    pub DESC_ADDR_UP_15: u32,
    pub DESC_ADDR_DOWN_15: u32,
    pub CTRL: u32,
    pub INT_STATUS: u32,
    pub HP_TIMEOUT: u32,
    pub HW_GROUP_PENDING: u32,
    pub SW_GROUP_PENDING: u32,
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
