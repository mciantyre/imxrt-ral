#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CCM_OBS
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Observe control
pub mod OBSERVE_CONTROL_0 {

    /// Observe signal selector
    pub mod SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Observe raw signal
    pub mod RAW {
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

            /// 0b0: Select divided signal.
            pub const RAW_0: u32 = 0b0;

            /// 0b1: Select raw signal.
            pub const RAW_1: u32 = 0b1;
        }
    }

    /// Invert
    pub mod INV {
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

            /// 0b0: Clock phase remain same.
            pub const INV_0: u32 = 0b0;

            /// 0b1: Invert clock phase before measurement or send to IO.
            pub const INV_1: u32 = 0b1;
        }
    }

    /// Reset observe divider
    pub mod RESET {
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

            /// 0b0: No reset
            pub const RESET_0: u32 = 0b0;

            /// 0b1: Reset observe divider
            pub const RESET_1: u32 = 0b1;
        }
    }

    /// Divider for observe signal
    pub mod DIVIDE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Turn off
    pub mod OFF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: observe slice is on
            pub const OFF_0: u32 = 0b0;

            /// 0b1: observe slice is off
            pub const OFF_1: u32 = 0b1;
        }
    }
}

/// Observe control
pub mod OBSERVE_CONTROL_SET_0 {

    /// Observe signal selector
    pub mod SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Observe raw signal
    pub mod RAW {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Invert
    pub mod INV {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset observe divider
    pub mod RESET {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Divider for observe signal
    pub mod DIVIDE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Turn off
    pub mod OFF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Observe control
pub mod OBSERVE_CONTROL_CLR_0 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_TOG_0 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe status
pub mod OBSERVE_STATUS0_0 {

    /// Select value
    pub mod SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Observe raw signal
    pub mod RAW {
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

            /// 0b0: Divided signal is selected
            pub const RAW_0: u32 = 0b0;

            /// 0b1: Raw signal is selected
            pub const RAW_1: u32 = 0b1;
        }
    }

    /// Polarity of the observe target
    pub mod INV {
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

            /// 0b0: Polarity is not inverted
            pub const INV_0: u32 = 0b0;

            /// 0b1: Polarity of the observe target is inverted
            pub const INV_1: u32 = 0b1;
        }
    }

    /// Reset state
    pub mod RESET {
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

            /// 0b0: Observe divider is not in reset state
            pub const RESET_0: u32 = 0b0;

            /// 0b1: Observe divider is in reset state
            pub const RESET_1: u32 = 0b1;
        }
    }

    /// Divide value status. The clock will be divided by DIVIDE + 1.
    pub mod DIVIDE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Turn off slice
    pub mod OFF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: observe slice is on
            pub const OFF_0: u32 = 0b0;

            /// 0b1: observe slice is off
            pub const OFF_1: u32 = 0b1;
        }
    }
}

/// Observe access control
pub mod OBSERVE_AUTHEN_0 {

    /// User access
    pub mod TZ_USER {
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

            /// 0b0: Clock cannot be changed in user mode.
            pub const TZ_USER_0: u32 = 0b0;

            /// 0b1: Clock can be changed in user mode.
            pub const TZ_USER_1: u32 = 0b1;
        }
    }

    /// Non-secure access
    pub mod TZ_NS {
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

            /// 0b0: Cannot be changed in Non-secure mode.
            pub const TZ_NS_0: u32 = 0b0;

            /// 0b1: Can be changed in Non-secure mode.
            pub const TZ_NS_1: u32 = 0b1;
        }
    }

    /// Lock truszone setting
    pub mod LOCK_TZ {
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

            /// 0b0: Trustzone setting is not locked.
            pub const LOCK_TZ_0: u32 = 0b0;

            /// 0b1: Trustzone setting is locked.
            pub const LOCK_TZ_1: u32 = 0b1;
        }
    }

    /// White list
    pub mod WHITE_LIST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No domain can change.
            pub const WHITE_LIST_0: u32 = 0b0000;

            /// 0b0001: Domain 0 can change.
            pub const WHITE_LIST_1: u32 = 0b0001;

            /// 0b0010: Domain 1 can change.
            pub const WHITE_LIST_2: u32 = 0b0010;

            /// 0b0011: Domain 0 and domain 1 can change.
            pub const WHITE_LIST_3: u32 = 0b0011;

            /// 0b0100: Domain 2 can change.
            pub const WHITE_LIST_4: u32 = 0b0100;

            /// 0b1111: All domain can change.
            pub const WHITE_LIST_15: u32 = 0b1111;
        }
    }

    /// Lock white list
    pub mod LOCK_LIST {
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

            /// 0b0: White list is not locked.
            pub const LOCK_LIST_0: u32 = 0b0;

            /// 0b1: White list is locked.
            pub const LOCK_LIST_1: u32 = 0b1;
        }
    }

    /// Low power and access control by domain
    pub mod DOMAIN_MODE {
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

            /// 0b0: Clock does not work in domain mode.
            pub const DOMAIN_MODE_0: u32 = 0b0;

            /// 0b1: Clock works in domain mode.
            pub const DOMAIN_MODE_1: u32 = 0b1;
        }
    }

    /// Lock low power and access mode
    pub mod LOCK_MODE {
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

            /// 0b0: MODE is not locked.
            pub const LOCK_MODE_0: u32 = 0b0;

            /// 0b1: MODE is locked.
            pub const LOCK_MODE_1: u32 = 0b1;
        }
    }
}

/// Observe access control
pub mod OBSERVE_AUTHEN_SET_0 {

    /// User access
    pub mod TZ_USER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Non-secure access
    pub mod TZ_NS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock truszone setting
    pub mod LOCK_TZ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// White list
    pub mod WHITE_LIST {
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

    /// Lock white list
    pub mod LOCK_LIST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low power and access control by domain
    pub mod DOMAIN_MODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock low power and access mode
    pub mod LOCK_MODE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Observe access control
pub mod OBSERVE_AUTHEN_CLR_0 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_TOG_0 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Current frequency detected
pub mod OBSERVE_FREQUENCY_CURRENT_0 {

    /// Frequency
    pub mod FREQUENCY {
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

/// Minimum frequency detected
pub mod OBSERVE_FREQUENCY_MIN_0 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Maximum frequency detected
pub mod OBSERVE_FREQUENCY_MAX_0 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Observe control
pub mod OBSERVE_CONTROL_1 {
    pub use super::OBSERVE_CONTROL_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_0::INV;
    pub use super::OBSERVE_CONTROL_0::OFF;
    pub use super::OBSERVE_CONTROL_0::RAW;
    pub use super::OBSERVE_CONTROL_0::RESET;
    pub use super::OBSERVE_CONTROL_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_SET_1 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_CLR_1 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_TOG_1 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe status
pub mod OBSERVE_STATUS0_1 {
    pub use super::OBSERVE_STATUS0_0::DIVIDE;
    pub use super::OBSERVE_STATUS0_0::INV;
    pub use super::OBSERVE_STATUS0_0::OFF;
    pub use super::OBSERVE_STATUS0_0::RAW;
    pub use super::OBSERVE_STATUS0_0::RESET;
    pub use super::OBSERVE_STATUS0_0::SELECT;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_1 {
    pub use super::OBSERVE_AUTHEN_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_SET_1 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_CLR_1 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_TOG_1 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Current frequency detected
pub mod OBSERVE_FREQUENCY_CURRENT_1 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Minimum frequency detected
pub mod OBSERVE_FREQUENCY_MIN_1 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Maximum frequency detected
pub mod OBSERVE_FREQUENCY_MAX_1 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Observe control
pub mod OBSERVE_CONTROL_2 {
    pub use super::OBSERVE_CONTROL_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_0::INV;
    pub use super::OBSERVE_CONTROL_0::OFF;
    pub use super::OBSERVE_CONTROL_0::RAW;
    pub use super::OBSERVE_CONTROL_0::RESET;
    pub use super::OBSERVE_CONTROL_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_SET_2 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_CLR_2 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_TOG_2 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe status
pub mod OBSERVE_STATUS0_2 {
    pub use super::OBSERVE_STATUS0_0::DIVIDE;
    pub use super::OBSERVE_STATUS0_0::INV;
    pub use super::OBSERVE_STATUS0_0::OFF;
    pub use super::OBSERVE_STATUS0_0::RAW;
    pub use super::OBSERVE_STATUS0_0::RESET;
    pub use super::OBSERVE_STATUS0_0::SELECT;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_2 {
    pub use super::OBSERVE_AUTHEN_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_SET_2 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_CLR_2 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_TOG_2 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Current frequency detected
pub mod OBSERVE_FREQUENCY_CURRENT_2 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Minimum frequency detected
pub mod OBSERVE_FREQUENCY_MIN_2 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Maximum frequency detected
pub mod OBSERVE_FREQUENCY_MAX_2 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Observe control
pub mod OBSERVE_CONTROL_3 {
    pub use super::OBSERVE_CONTROL_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_0::INV;
    pub use super::OBSERVE_CONTROL_0::OFF;
    pub use super::OBSERVE_CONTROL_0::RAW;
    pub use super::OBSERVE_CONTROL_0::RESET;
    pub use super::OBSERVE_CONTROL_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_SET_3 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_CLR_3 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_TOG_3 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe status
pub mod OBSERVE_STATUS0_3 {
    pub use super::OBSERVE_STATUS0_0::DIVIDE;
    pub use super::OBSERVE_STATUS0_0::INV;
    pub use super::OBSERVE_STATUS0_0::OFF;
    pub use super::OBSERVE_STATUS0_0::RAW;
    pub use super::OBSERVE_STATUS0_0::RESET;
    pub use super::OBSERVE_STATUS0_0::SELECT;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_3 {
    pub use super::OBSERVE_AUTHEN_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_SET_3 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_CLR_3 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_TOG_3 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Current frequency detected
pub mod OBSERVE_FREQUENCY_CURRENT_3 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Minimum frequency detected
pub mod OBSERVE_FREQUENCY_MIN_3 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Maximum frequency detected
pub mod OBSERVE_FREQUENCY_MAX_3 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Observe control
pub mod OBSERVE_CONTROL_4 {
    pub use super::OBSERVE_CONTROL_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_0::INV;
    pub use super::OBSERVE_CONTROL_0::OFF;
    pub use super::OBSERVE_CONTROL_0::RAW;
    pub use super::OBSERVE_CONTROL_0::RESET;
    pub use super::OBSERVE_CONTROL_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_SET_4 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_CLR_4 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_TOG_4 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe status
pub mod OBSERVE_STATUS0_4 {
    pub use super::OBSERVE_STATUS0_0::DIVIDE;
    pub use super::OBSERVE_STATUS0_0::INV;
    pub use super::OBSERVE_STATUS0_0::OFF;
    pub use super::OBSERVE_STATUS0_0::RAW;
    pub use super::OBSERVE_STATUS0_0::RESET;
    pub use super::OBSERVE_STATUS0_0::SELECT;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_4 {
    pub use super::OBSERVE_AUTHEN_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_SET_4 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_CLR_4 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_TOG_4 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Current frequency detected
pub mod OBSERVE_FREQUENCY_CURRENT_4 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Minimum frequency detected
pub mod OBSERVE_FREQUENCY_MIN_4 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Maximum frequency detected
pub mod OBSERVE_FREQUENCY_MAX_4 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Observe control
pub mod OBSERVE_CONTROL_5 {
    pub use super::OBSERVE_CONTROL_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_0::INV;
    pub use super::OBSERVE_CONTROL_0::OFF;
    pub use super::OBSERVE_CONTROL_0::RAW;
    pub use super::OBSERVE_CONTROL_0::RESET;
    pub use super::OBSERVE_CONTROL_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_SET_5 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_CLR_5 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe control
pub mod OBSERVE_CONTROL_TOG_5 {
    pub use super::OBSERVE_CONTROL_SET_0::DIVIDE;
    pub use super::OBSERVE_CONTROL_SET_0::INV;
    pub use super::OBSERVE_CONTROL_SET_0::OFF;
    pub use super::OBSERVE_CONTROL_SET_0::RAW;
    pub use super::OBSERVE_CONTROL_SET_0::RESET;
    pub use super::OBSERVE_CONTROL_SET_0::SELECT;
}

/// Observe status
pub mod OBSERVE_STATUS0_5 {
    pub use super::OBSERVE_STATUS0_0::DIVIDE;
    pub use super::OBSERVE_STATUS0_0::INV;
    pub use super::OBSERVE_STATUS0_0::OFF;
    pub use super::OBSERVE_STATUS0_0::RAW;
    pub use super::OBSERVE_STATUS0_0::RESET;
    pub use super::OBSERVE_STATUS0_0::SELECT;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_5 {
    pub use super::OBSERVE_AUTHEN_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_SET_5 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_CLR_5 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Observe access control
pub mod OBSERVE_AUTHEN_TOG_5 {
    pub use super::OBSERVE_AUTHEN_SET_0::DOMAIN_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_LIST;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_MODE;
    pub use super::OBSERVE_AUTHEN_SET_0::LOCK_TZ;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_NS;
    pub use super::OBSERVE_AUTHEN_SET_0::TZ_USER;
    pub use super::OBSERVE_AUTHEN_SET_0::WHITE_LIST;
}

/// Current frequency detected
pub mod OBSERVE_FREQUENCY_CURRENT_5 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Minimum frequency detected
pub mod OBSERVE_FREQUENCY_MIN_5 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}

/// Maximum frequency detected
pub mod OBSERVE_FREQUENCY_MAX_5 {
    pub use super::OBSERVE_FREQUENCY_CURRENT_0::FREQUENCY;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Observe control
    pub OBSERVE_CONTROL_0: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_SET_0: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_CLR_0: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_TOG_0: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// Observe status
    pub OBSERVE_STATUS0_0: RORegister<u32>,

    _reserved2: [u32; 3],

    /// Observe access control
    pub OBSERVE_AUTHEN_0: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_SET_0: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_CLR_0: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_TOG_0: RWRegister<u32>,

    /// Current frequency detected
    pub OBSERVE_FREQUENCY_CURRENT_0: RORegister<u32>,

    /// Minimum frequency detected
    pub OBSERVE_FREQUENCY_MIN_0: RORegister<u32>,

    /// Maximum frequency detected
    pub OBSERVE_FREQUENCY_MAX_0: RORegister<u32>,

    _reserved3: [u32; 13],

    /// Observe control
    pub OBSERVE_CONTROL_1: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_SET_1: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_CLR_1: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_TOG_1: RWRegister<u32>,

    _reserved4: [u32; 4],

    /// Observe status
    pub OBSERVE_STATUS0_1: RORegister<u32>,

    _reserved5: [u32; 3],

    /// Observe access control
    pub OBSERVE_AUTHEN_1: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_SET_1: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_CLR_1: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_TOG_1: RWRegister<u32>,

    /// Current frequency detected
    pub OBSERVE_FREQUENCY_CURRENT_1: RORegister<u32>,

    /// Minimum frequency detected
    pub OBSERVE_FREQUENCY_MIN_1: RORegister<u32>,

    /// Maximum frequency detected
    pub OBSERVE_FREQUENCY_MAX_1: RORegister<u32>,

    _reserved6: [u32; 13],

    /// Observe control
    pub OBSERVE_CONTROL_2: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_SET_2: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_CLR_2: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_TOG_2: RWRegister<u32>,

    _reserved7: [u32; 4],

    /// Observe status
    pub OBSERVE_STATUS0_2: RORegister<u32>,

    _reserved8: [u32; 3],

    /// Observe access control
    pub OBSERVE_AUTHEN_2: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_SET_2: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_CLR_2: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_TOG_2: RWRegister<u32>,

    /// Current frequency detected
    pub OBSERVE_FREQUENCY_CURRENT_2: RORegister<u32>,

    /// Minimum frequency detected
    pub OBSERVE_FREQUENCY_MIN_2: RORegister<u32>,

    /// Maximum frequency detected
    pub OBSERVE_FREQUENCY_MAX_2: RORegister<u32>,

    _reserved9: [u32; 13],

    /// Observe control
    pub OBSERVE_CONTROL_3: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_SET_3: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_CLR_3: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_TOG_3: RWRegister<u32>,

    _reserved10: [u32; 4],

    /// Observe status
    pub OBSERVE_STATUS0_3: RORegister<u32>,

    _reserved11: [u32; 3],

    /// Observe access control
    pub OBSERVE_AUTHEN_3: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_SET_3: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_CLR_3: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_TOG_3: RWRegister<u32>,

    /// Current frequency detected
    pub OBSERVE_FREQUENCY_CURRENT_3: RORegister<u32>,

    /// Minimum frequency detected
    pub OBSERVE_FREQUENCY_MIN_3: RORegister<u32>,

    /// Maximum frequency detected
    pub OBSERVE_FREQUENCY_MAX_3: RORegister<u32>,

    _reserved12: [u32; 13],

    /// Observe control
    pub OBSERVE_CONTROL_4: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_SET_4: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_CLR_4: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_TOG_4: RWRegister<u32>,

    _reserved13: [u32; 4],

    /// Observe status
    pub OBSERVE_STATUS0_4: RORegister<u32>,

    _reserved14: [u32; 3],

    /// Observe access control
    pub OBSERVE_AUTHEN_4: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_SET_4: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_CLR_4: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_TOG_4: RWRegister<u32>,

    /// Current frequency detected
    pub OBSERVE_FREQUENCY_CURRENT_4: RORegister<u32>,

    /// Minimum frequency detected
    pub OBSERVE_FREQUENCY_MIN_4: RORegister<u32>,

    /// Maximum frequency detected
    pub OBSERVE_FREQUENCY_MAX_4: RORegister<u32>,

    _reserved15: [u32; 13],

    /// Observe control
    pub OBSERVE_CONTROL_5: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_SET_5: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_CLR_5: RWRegister<u32>,

    /// Observe control
    pub OBSERVE_CONTROL_TOG_5: RWRegister<u32>,

    _reserved16: [u32; 4],

    /// Observe status
    pub OBSERVE_STATUS0_5: RORegister<u32>,

    _reserved17: [u32; 3],

    /// Observe access control
    pub OBSERVE_AUTHEN_5: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_SET_5: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_CLR_5: RWRegister<u32>,

    /// Observe access control
    pub OBSERVE_AUTHEN_TOG_5: RWRegister<u32>,

    /// Current frequency detected
    pub OBSERVE_FREQUENCY_CURRENT_5: RORegister<u32>,

    /// Minimum frequency detected
    pub OBSERVE_FREQUENCY_MIN_5: RORegister<u32>,

    /// Maximum frequency detected
    pub OBSERVE_FREQUENCY_MAX_5: RORegister<u32>,
}
pub struct ResetValues {
    pub OBSERVE_CONTROL_0: u32,
    pub OBSERVE_CONTROL_SET_0: u32,
    pub OBSERVE_CONTROL_CLR_0: u32,
    pub OBSERVE_CONTROL_TOG_0: u32,
    pub OBSERVE_STATUS0_0: u32,
    pub OBSERVE_AUTHEN_0: u32,
    pub OBSERVE_AUTHEN_SET_0: u32,
    pub OBSERVE_AUTHEN_CLR_0: u32,
    pub OBSERVE_AUTHEN_TOG_0: u32,
    pub OBSERVE_FREQUENCY_CURRENT_0: u32,
    pub OBSERVE_FREQUENCY_MIN_0: u32,
    pub OBSERVE_FREQUENCY_MAX_0: u32,
    pub OBSERVE_CONTROL_1: u32,
    pub OBSERVE_CONTROL_SET_1: u32,
    pub OBSERVE_CONTROL_CLR_1: u32,
    pub OBSERVE_CONTROL_TOG_1: u32,
    pub OBSERVE_STATUS0_1: u32,
    pub OBSERVE_AUTHEN_1: u32,
    pub OBSERVE_AUTHEN_SET_1: u32,
    pub OBSERVE_AUTHEN_CLR_1: u32,
    pub OBSERVE_AUTHEN_TOG_1: u32,
    pub OBSERVE_FREQUENCY_CURRENT_1: u32,
    pub OBSERVE_FREQUENCY_MIN_1: u32,
    pub OBSERVE_FREQUENCY_MAX_1: u32,
    pub OBSERVE_CONTROL_2: u32,
    pub OBSERVE_CONTROL_SET_2: u32,
    pub OBSERVE_CONTROL_CLR_2: u32,
    pub OBSERVE_CONTROL_TOG_2: u32,
    pub OBSERVE_STATUS0_2: u32,
    pub OBSERVE_AUTHEN_2: u32,
    pub OBSERVE_AUTHEN_SET_2: u32,
    pub OBSERVE_AUTHEN_CLR_2: u32,
    pub OBSERVE_AUTHEN_TOG_2: u32,
    pub OBSERVE_FREQUENCY_CURRENT_2: u32,
    pub OBSERVE_FREQUENCY_MIN_2: u32,
    pub OBSERVE_FREQUENCY_MAX_2: u32,
    pub OBSERVE_CONTROL_3: u32,
    pub OBSERVE_CONTROL_SET_3: u32,
    pub OBSERVE_CONTROL_CLR_3: u32,
    pub OBSERVE_CONTROL_TOG_3: u32,
    pub OBSERVE_STATUS0_3: u32,
    pub OBSERVE_AUTHEN_3: u32,
    pub OBSERVE_AUTHEN_SET_3: u32,
    pub OBSERVE_AUTHEN_CLR_3: u32,
    pub OBSERVE_AUTHEN_TOG_3: u32,
    pub OBSERVE_FREQUENCY_CURRENT_3: u32,
    pub OBSERVE_FREQUENCY_MIN_3: u32,
    pub OBSERVE_FREQUENCY_MAX_3: u32,
    pub OBSERVE_CONTROL_4: u32,
    pub OBSERVE_CONTROL_SET_4: u32,
    pub OBSERVE_CONTROL_CLR_4: u32,
    pub OBSERVE_CONTROL_TOG_4: u32,
    pub OBSERVE_STATUS0_4: u32,
    pub OBSERVE_AUTHEN_4: u32,
    pub OBSERVE_AUTHEN_SET_4: u32,
    pub OBSERVE_AUTHEN_CLR_4: u32,
    pub OBSERVE_AUTHEN_TOG_4: u32,
    pub OBSERVE_FREQUENCY_CURRENT_4: u32,
    pub OBSERVE_FREQUENCY_MIN_4: u32,
    pub OBSERVE_FREQUENCY_MAX_4: u32,
    pub OBSERVE_CONTROL_5: u32,
    pub OBSERVE_CONTROL_SET_5: u32,
    pub OBSERVE_CONTROL_CLR_5: u32,
    pub OBSERVE_CONTROL_TOG_5: u32,
    pub OBSERVE_STATUS0_5: u32,
    pub OBSERVE_AUTHEN_5: u32,
    pub OBSERVE_AUTHEN_SET_5: u32,
    pub OBSERVE_AUTHEN_CLR_5: u32,
    pub OBSERVE_AUTHEN_TOG_5: u32,
    pub OBSERVE_FREQUENCY_CURRENT_5: u32,
    pub OBSERVE_FREQUENCY_MIN_5: u32,
    pub OBSERVE_FREQUENCY_MAX_5: u32,
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
