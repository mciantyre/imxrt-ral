#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPC_SP
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// SP Authentication Control
pub mod SP_AUTHEN_CTRL {

    /// Allow user mode access
    pub mod USER {
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

            /// 0b0: Allow only privilege mode to access setpoint control registers
            pub const b0: u32 = 0b0;

            /// 0b1: Allow both privilege and user mode to access setpoint control registers
            pub const b1: u32 = 0b1;
        }
    }

    /// Allow non-secure mode access
    pub mod NONSECURE {
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

            /// 0b0: Allow only secure mode to access setpoint control registers
            pub const b0: u32 = 0b0;

            /// 0b1: Allow both secure and non-secure mode to access setpoint control registers
            pub const b1: u32 = 0b1;
        }
    }

    /// Lock NONSECURE and USER
    pub mod LOCK_SETTING {
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

    /// Domain ID white list
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

    /// White list lock
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

    /// Configuration lock
    pub mod LOCK_CFG {
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

/// SP Interrupt Control
pub mod SP_INT_CTRL {

    /// no_allowed_set_point interrupt enable
    pub mod NO_ALLOWED_SP_INT_EN {
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

    /// no_allowed_set_point interrupt
    pub mod NO_ALLOWED_SP_INT {
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
}

/// CPU SP Request
pub mod SP_CPU_REQ {

    /// Setpoint requested by CPU0
    pub mod SP_REQ_CPU0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Setpoint requested by CPU1
    pub mod SP_REQ_CPU1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Setpoint requested by CPU2
    pub mod SP_REQ_CPU2 {
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

    /// Setpoint requested by CPU3
    pub mod SP_REQ_CPU3 {
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

    /// CPU0 Setpoint accepted by SP controller
    pub mod SP_ACCEPTED_CPU0 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU1 Setpoint accepted by SP controller
    pub mod SP_ACCEPTED_CPU1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU2 Setpoint accepted by SP controller
    pub mod SP_ACCEPTED_CPU2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU3 Setpoint accepted by SP controller
    pub mod SP_ACCEPTED_CPU3 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SP System Status
pub mod SP_SYS_STAT {

    /// Allowed Setpoints by all current CPU Setpoint requests
    pub mod SYS_SP_ALLOWED {
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

    /// The Setpoint chosen as the target setpoint
    pub mod SYS_SP_TARGET {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Current Setpoint, only valid when not SP trans busy
    pub mod SYS_SP_CURRENT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Previous Setpoint, only valid when not SP trans busy
    pub mod SYS_SP_PREVIOUS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SP ROSC Control
pub mod SP_ROSC_CTRL {

    /// Allow shutting off the ROSC
    pub mod SP_ALLOW_ROSC_OFF {
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
}

/// SP0~7 Priority
pub mod SP_PRIORITY_0_7 {

    /// priority of Setpoint 0
    pub mod SYS_SP0_PRIORITY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 1
    pub mod SYS_SP1_PRIORITY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 2
    pub mod SYS_SP2_PRIORITY {
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

    /// priority of Setpoint 3
    pub mod SYS_SP3_PRIORITY {
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

    /// priority of Setpoint 4
    pub mod SYS_SP4_PRIORITY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 5
    pub mod SYS_SP5_PRIORITY {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 6
    pub mod SYS_SP6_PRIORITY {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 7
    pub mod SYS_SP7_PRIORITY {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SP8~15 Priority
pub mod SP_PRIORITY_8_15 {

    /// priority of Setpoint 8
    pub mod SYS_SP8_PRIORITY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 9
    pub mod SYS_SP9_PRIORITY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 10
    pub mod SYS_SP10_PRIORITY {
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

    /// priority of Setpoint 11
    pub mod SYS_SP11_PRIORITY {
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

    /// priority of Setpoint 12
    pub mod SYS_SP12_PRIORITY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 13
    pub mod SYS_SP13_PRIORITY {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 14
    pub mod SYS_SP14_PRIORITY {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// priority of Setpoint 15
    pub mod SYS_SP15_PRIORITY {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SP SSAR save control
pub mod SP_SSAR_SAVE_CTRL {

    /// Step count, useage is depending on CNT_MODE
    pub mod STEP_CNT {
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

    /// Count mode
    pub mod CNT_MODE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Counter disable mode: not use step counter, step completes once receiving step_done
            pub const b0: u32 = 0b00;

            /// 0b01: Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT
            pub const b1: u32 = 0b01;

            /// 0b10: Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes
            pub const b2: u32 = 0b10;

            /// 0b11: Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value
            pub const b3: u32 = 0b11;
        }
    }

    /// Disable this step
    pub mod DISABLE {
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

/// SP LPCG off control
pub mod SP_LPCG_OFF_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP group down control
pub mod SP_GROUP_DOWN_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP root down control
pub mod SP_ROOT_DOWN_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP PLL off control
pub mod SP_PLL_OFF_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP ISO on control
pub mod SP_ISO_ON_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP reset early control
pub mod SP_RESET_EARLY_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP power off control
pub mod SP_POWER_OFF_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP bias off control
pub mod SP_BIAS_OFF_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP bandgap and PLL_LDO off control
pub mod SP_BG_PLDO_OFF_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP LDO pre control
pub mod SP_LDO_PRE_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP DCDC down control
pub mod SP_DCDC_DOWN_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP DCDC up control
pub mod SP_DCDC_UP_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP LDO post control
pub mod SP_LDO_POST_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP bandgap and PLL_LDO on control
pub mod SP_BG_PLDO_ON_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP bias on control
pub mod SP_BIAS_ON_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP power on control
pub mod SP_POWER_ON_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP reset late control
pub mod SP_RESET_LATE_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP ISO off control
pub mod SP_ISO_OFF_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP PLL on control
pub mod SP_PLL_ON_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP root up control
pub mod SP_ROOT_UP_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP group up control
pub mod SP_GROUP_UP_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP LPCG on control
pub mod SP_LPCG_ON_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}

/// SP SSAR restore control
pub mod SP_SSAR_RESTORE_CTRL {
    pub use super::SP_SSAR_SAVE_CTRL::CNT_MODE;
    pub use super::SP_SSAR_SAVE_CTRL::DISABLE;
    pub use super::SP_SSAR_SAVE_CTRL::STEP_CNT;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 1],

    /// SP Authentication Control
    pub SP_AUTHEN_CTRL: RWRegister<u32>,

    /// SP Interrupt Control
    pub SP_INT_CTRL: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// CPU SP Request
    pub SP_CPU_REQ: RORegister<u32>,

    /// SP System Status
    pub SP_SYS_STAT: RORegister<u32>,

    _reserved3: [u32; 1],

    /// SP ROSC Control
    pub SP_ROSC_CTRL: RWRegister<u32>,

    _reserved4: [u32; 8],

    /// SP0~7 Priority
    pub SP_PRIORITY_0_7: RWRegister<u32>,

    /// SP8~15 Priority
    pub SP_PRIORITY_8_15: RWRegister<u32>,

    _reserved5: [u32; 46],

    /// SP SSAR save control
    pub SP_SSAR_SAVE_CTRL: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// SP LPCG off control
    pub SP_LPCG_OFF_CTRL: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// SP group down control
    pub SP_GROUP_DOWN_CTRL: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// SP root down control
    pub SP_ROOT_DOWN_CTRL: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// SP PLL off control
    pub SP_PLL_OFF_CTRL: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// SP ISO on control
    pub SP_ISO_ON_CTRL: RWRegister<u32>,

    _reserved11: [u32; 3],

    /// SP reset early control
    pub SP_RESET_EARLY_CTRL: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// SP power off control
    pub SP_POWER_OFF_CTRL: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// SP bias off control
    pub SP_BIAS_OFF_CTRL: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// SP bandgap and PLL_LDO off control
    pub SP_BG_PLDO_OFF_CTRL: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// SP LDO pre control
    pub SP_LDO_PRE_CTRL: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// SP DCDC down control
    pub SP_DCDC_DOWN_CTRL: RWRegister<u32>,

    _reserved17: [u32; 19],

    /// SP DCDC up control
    pub SP_DCDC_UP_CTRL: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// SP LDO post control
    pub SP_LDO_POST_CTRL: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// SP bandgap and PLL_LDO on control
    pub SP_BG_PLDO_ON_CTRL: RWRegister<u32>,

    _reserved20: [u32; 3],

    /// SP bias on control
    pub SP_BIAS_ON_CTRL: RWRegister<u32>,

    _reserved21: [u32; 3],

    /// SP power on control
    pub SP_POWER_ON_CTRL: RWRegister<u32>,

    _reserved22: [u32; 3],

    /// SP reset late control
    pub SP_RESET_LATE_CTRL: RWRegister<u32>,

    _reserved23: [u32; 3],

    /// SP ISO off control
    pub SP_ISO_OFF_CTRL: RWRegister<u32>,

    _reserved24: [u32; 3],

    /// SP PLL on control
    pub SP_PLL_ON_CTRL: RWRegister<u32>,

    _reserved25: [u32; 3],

    /// SP root up control
    pub SP_ROOT_UP_CTRL: RWRegister<u32>,

    _reserved26: [u32; 3],

    /// SP group up control
    pub SP_GROUP_UP_CTRL: RWRegister<u32>,

    _reserved27: [u32; 3],

    /// SP LPCG on control
    pub SP_LPCG_ON_CTRL: RWRegister<u32>,

    _reserved28: [u32; 3],

    /// SP SSAR restore control
    pub SP_SSAR_RESTORE_CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub SP_AUTHEN_CTRL: u32,
    pub SP_INT_CTRL: u32,
    pub SP_CPU_REQ: u32,
    pub SP_SYS_STAT: u32,
    pub SP_ROSC_CTRL: u32,
    pub SP_PRIORITY_0_7: u32,
    pub SP_PRIORITY_8_15: u32,
    pub SP_SSAR_SAVE_CTRL: u32,
    pub SP_LPCG_OFF_CTRL: u32,
    pub SP_GROUP_DOWN_CTRL: u32,
    pub SP_ROOT_DOWN_CTRL: u32,
    pub SP_PLL_OFF_CTRL: u32,
    pub SP_ISO_ON_CTRL: u32,
    pub SP_RESET_EARLY_CTRL: u32,
    pub SP_POWER_OFF_CTRL: u32,
    pub SP_BIAS_OFF_CTRL: u32,
    pub SP_BG_PLDO_OFF_CTRL: u32,
    pub SP_LDO_PRE_CTRL: u32,
    pub SP_DCDC_DOWN_CTRL: u32,
    pub SP_DCDC_UP_CTRL: u32,
    pub SP_LDO_POST_CTRL: u32,
    pub SP_BG_PLDO_ON_CTRL: u32,
    pub SP_BIAS_ON_CTRL: u32,
    pub SP_POWER_ON_CTRL: u32,
    pub SP_RESET_LATE_CTRL: u32,
    pub SP_ISO_OFF_CTRL: u32,
    pub SP_PLL_ON_CTRL: u32,
    pub SP_ROOT_UP_CTRL: u32,
    pub SP_GROUP_UP_CTRL: u32,
    pub SP_LPCG_ON_CTRL: u32,
    pub SP_SSAR_RESTORE_CTRL: u32,
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
