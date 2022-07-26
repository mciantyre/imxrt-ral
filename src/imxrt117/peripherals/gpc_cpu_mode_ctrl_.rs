#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPC_CPU
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// CM Authentication Control
pub mod CM_AUTHEN_CTRL {

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

            /// 0b0: Allow only privilege mode to access CPU mode control registers
            pub const b0: u32 = 0b0;

            /// 0b1: Allow both privilege and user mode to access CPU mode control registers
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

            /// 0b0: Allow only secure mode to access CPU mode control registers
            pub const b0: u32 = 0b0;

            /// 0b1: Allow both secure and non-secure mode to access CPU mode control registers
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

/// CM Interrupt Control
pub mod CM_INT_CTRL {

    /// sp_req_not_allowed_for_sleep interrupt enable
    pub mod SP_REQ_NOT_ALLOWED_SLEEP_INT_EN {
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

            /// 0b0: Interrupt disable
            pub const b0: u32 = 0b0;

            /// 0b1: Interrupt enable
            pub const b1: u32 = 0b1;
        }
    }

    /// sp_req_not_allowed_for_wakeup interrupt enable
    pub mod SP_REQ_NOT_ALLOWED_WAKEUP_INT_EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SP_REQ_NOT_ALLOWED_SLEEP_INT_EN::RW;
    }

    /// sp_req_not_allowed_for_soft interrupt enable
    pub mod SP_REQ_NOT_ALLOWED_SOFT_INT_EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SP_REQ_NOT_ALLOWED_SLEEP_INT_EN::RW;
    }

    /// sp_req_not_allowed_for_sleep interrupt status and clear register
    pub mod SP_REQ_NOT_ALLOWED_SLEEP_INT {
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

    /// sp_req_not_allowed_for_wakeup interrupt status and clear register
    pub mod SP_REQ_NOT_ALLOWED_WAKEUP_INT {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// sp_req_not_allowed_for_soft interrupt status and clear register
    pub mod SP_REQ_NOT_ALLOWED_SOFT_INT {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Miscellaneous
pub mod CM_MISC {

    /// Non-masked interrupt status
    pub mod NMI_STAT {
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

            /// 0b0: NMI is not asserting
            pub const b0: u32 = 0b0;

            /// 0b1: NMI is asserting
            pub const b1: u32 = 0b1;
        }
    }

    /// Allow cpu_sleep_hold_req assert during CPU low power status
    pub mod SLEEP_HOLD_EN {
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

            /// 0b0: Disable cpu_sleep_hold_req
            pub const b0: u32 = 0b0;

            /// 0b1: Allow cpu_sleep_hold_req assert during CPU low power status
            pub const b1: u32 = 0b1;
        }
    }

    /// Status of cpu_sleep_hold_ack_b
    pub mod SLEEP_HOLD_STAT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Master CPU
    pub mod MASTER_CPU {
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
}

/// CPU mode control
pub mod CM_MODE_CTRL {

    /// The CPU mode the CPU platform should transit to on next sleep event
    pub mod CPU_MODE_TARGET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Stay in RUN mode
            pub const b0: u32 = 0b00;

            /// 0b01: Transit to WAIT mode
            pub const b1: u32 = 0b01;

            /// 0b10: Transit to STOP mode
            pub const b2: u32 = 0b10;

            /// 0b11: Transit to SUSPEND mode
            pub const b3: u32 = 0b11;
        }
    }

    /// WFE assertion can be sleep event
    pub mod WFE_EN {
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

            /// 0b0: WFE assertion can not trigger low power
            pub const b0: u32 = 0b0;

            /// 0b1: WFE assertion can trigger low power
            pub const b1: u32 = 0b1;
        }
    }
}

/// CM CPU mode Status
pub mod CM_MODE_STAT {

    /// Current CPU mode
    pub mod CPU_MODE_CURRENT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: CPU is currently in RUN mode
            pub const RUN: u32 = 0b00;

            /// 0b01: CPU is currently in WAIT mode
            pub const WAIT: u32 = 0b01;

            /// 0b10: CPU is currently in STOP mode
            pub const STOP: u32 = 0b10;

            /// 0b11: CPU is currently in SUSPEND mode
            pub const SUSPEND: u32 = 0b11;
        }
    }

    /// Previous CPU mode
    pub mod CPU_MODE_PREVIOUS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: CPU was previously in RUN mode
            pub const RUN: u32 = 0b00;

            /// 0b01: CPU was previously in WAIT mode
            pub const WAIT: u32 = 0b01;

            /// 0b10: CPU was previously in STOP mode
            pub const STOP: u32 = 0b10;

            /// 0b11: CPU was previously in SUSPEND mode
            pub const SUSPEND: u32 = 0b11;
        }
    }
}

/// CM IRQ0~31 wakeup mask
pub mod CM_IRQ_WAKEUP_MASK_0 {

    /// "1" means the IRQ cannot wakeup CPU platform
    pub mod IRQ_WAKEUP_MASK_0_31 {
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

/// CM IRQ32~63 wakeup mask
pub mod CM_IRQ_WAKEUP_MASK_1 {

    /// "1" means the IRQ cannot wakeup CPU platform
    pub mod IRQ_WAKEUP_MASK_32_63 {
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

/// CM IRQ64~95 wakeup mask
pub mod CM_IRQ_WAKEUP_MASK_2 {

    /// "1" means the IRQ cannot wakeup CPU platform
    pub mod IRQ_WAKEUP_MASK_64_95 {
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

/// CM IRQ96~127 wakeup mask
pub mod CM_IRQ_WAKEUP_MASK_3 {

    /// "1" means the IRQ cannot wakeup CPU platform
    pub mod IRQ_WAKEUP_MASK_96_127 {
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

/// CM IRQ128~159 wakeup mask
pub mod CM_IRQ_WAKEUP_MASK_4 {

    /// "1" means the IRQ cannot wakeup CPU platform
    pub mod IRQ_WAKEUP_MASK_128_159 {
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

/// CM IRQ160~191 wakeup mask
pub mod CM_IRQ_WAKEUP_MASK_5 {

    /// "1" means the IRQ cannot wakeup CPU platform
    pub mod IRQ_WAKEUP_MASK_160_191 {
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

/// CM IRQ192~223 wakeup mask
pub mod CM_IRQ_WAKEUP_MASK_6 {

    /// "1" means the IRQ cannot wakeup CPU platform
    pub mod IRQ_WAKEUP_MASK_192_223 {
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

/// CM IRQ224~255 wakeup mask
pub mod CM_IRQ_WAKEUP_MASK_7 {

    /// "1" means the IRQ cannot wakeup CPU platform
    pub mod IRQ_WAKEUP_MASK_224_255 {
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

/// CM non-irq wakeup mask
pub mod CM_NON_IRQ_WAKEUP_MASK {

    /// There are 256 interrupts and 1 event as a wakeup source for GPC. This field masks the 1 event wakeup source.
    pub mod EVENT_WAKEUP_MASK {
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

            /// 0b1: The event cannot wakeup CPU platform
            pub const b1: u32 = 0b1;
        }
    }

    /// "1" means the debug_wakeup_request cannot wakeup CPU platform
    pub mod DEBUG_WAKEUP_MASK {
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

/// CM IRQ0~31 wakeup status
pub mod CM_IRQ_WAKEUP_STAT_0 {

    /// IRQ status
    pub mod IRQ_WAKEUP_STAT_0_31 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: None
            pub const b0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Valid
            pub const b1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// CM IRQ32~63 wakeup status
pub mod CM_IRQ_WAKEUP_STAT_1 {

    /// IRQ status
    pub mod IRQ_WAKEUP_STAT_32_63 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: None
            pub const b0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Valid
            pub const b1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// CM IRQ64~95 wakeup status
pub mod CM_IRQ_WAKEUP_STAT_2 {

    /// IRQ status
    pub mod IRQ_WAKEUP_STAT_64_95 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: None
            pub const b0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Valid
            pub const b1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// CM IRQ96~127 wakeup status
pub mod CM_IRQ_WAKEUP_STAT_3 {

    /// IRQ status
    pub mod IRQ_WAKEUP_STAT_96_127 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: None
            pub const b0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Valid
            pub const b1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// CM IRQ128~159 wakeup status
pub mod CM_IRQ_WAKEUP_STAT_4 {

    /// IRQ status
    pub mod IRQ_WAKEUP_STAT_128_159 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: None
            pub const b0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Valid
            pub const b1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// CM IRQ160~191 wakeup status
pub mod CM_IRQ_WAKEUP_STAT_5 {

    /// IRQ status
    pub mod IRQ_WAKEUP_STAT_160_191 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: None
            pub const b0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Valid
            pub const b1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// CM IRQ192~223 wakeup status
pub mod CM_IRQ_WAKEUP_STAT_6 {

    /// IRQ status
    pub mod IRQ_WAKEUP_STAT_192_223 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: None
            pub const b0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Valid
            pub const b1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// CM IRQ224~255 wakeup status
pub mod CM_IRQ_WAKEUP_STAT_7 {

    /// IRQ status
    pub mod IRQ_WAKEUP_MASK_224_255 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: None
            pub const b0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Valid
            pub const b1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// CM non-irq wakeup status
pub mod CM_NON_IRQ_WAKEUP_STAT {

    /// Event wakeup status
    pub mod EVENT_WAKEUP_STAT {
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

            /// 0b1: Interrupt is asserting (pending)
            pub const b1: u32 = 0b1;
        }
    }

    /// Debug wakeup status
    pub mod DEBUG_WAKEUP_STAT {
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

/// CM sleep SSAR control
pub mod CM_SLEEP_SSAR_CTRL {

    /// Step count, useage is depending on CNT_MODE.
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

/// CM sleep LPCG control
pub mod CM_SLEEP_LPCG_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM sleep PLL control
pub mod CM_SLEEP_PLL_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM sleep isolation control
pub mod CM_SLEEP_ISO_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM sleep reset control
pub mod CM_SLEEP_RESET_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM sleep power control
pub mod CM_SLEEP_POWER_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM wakeup power control
pub mod CM_WAKEUP_POWER_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM wakeup reset control
pub mod CM_WAKEUP_RESET_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM wakeup isolation control
pub mod CM_WAKEUP_ISO_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM wakeup PLL control
pub mod CM_WAKEUP_PLL_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM wakeup LPCG control
pub mod CM_WAKEUP_LPCG_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM wakeup SSAR control
pub mod CM_WAKEUP_SSAR_CTRL {
    pub use super::CM_SLEEP_SSAR_CTRL::CNT_MODE;
    pub use super::CM_SLEEP_SSAR_CTRL::DISABLE;
    pub use super::CM_SLEEP_SSAR_CTRL::STEP_CNT;
}

/// CM Setpoint Control
pub mod CM_SP_CTRL {

    /// Request a Setpoint transition when this bit is set
    pub mod CPU_SP_RUN_EN {
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

    /// The Setpoint that CPU want the system to transit to when CPU_SP_RUN_EN is set
    pub mod CPU_SP_RUN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (4 bits: 0b1111 << 1)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1 means enable Setpoint transition on next CPU platform sleep sequence
    pub mod CPU_SP_SLEEP_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The Setpoint that CPU want the system to transit to on next CPU platform sleep sequence
    pub mod CPU_SP_SLEEP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (4 bits: 0b1111 << 6)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1 means enable Setpoint transition on next CPU platform wakeup sequence
    pub mod CPU_SP_WAKEUP_EN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The Setpoint that CPU want the system to transit to on next CPU platform wakeup sequence
    pub mod CPU_SP_WAKEUP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (4 bits: 0b1111 << 11)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Select the Setpoint transiton on the next CPU platform wakeup sequence
    pub mod CPU_SP_WAKEUP_SEL {
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

            /// 0b0: Request SP transition to CPU_SP_WAKEUP
            pub const b0: u32 = 0b0;

            /// 0b1: Request SP transition to the Setpoint when the sleep event happens, which is captured in CPU_SP_PREVIOUS
            pub const b1: u32 = 0b1;
        }
    }
}

/// CM Setpoint Status
pub mod CM_SP_STAT {

    /// The current Setpoint of the system
    pub mod CPU_SP_CURRENT {
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

    /// The previous Setpoint of the system
    pub mod CPU_SP_PREVIOUS {
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

    /// The requested Setpoint from the CPU platform
    pub mod CPU_SP_TARGET {
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
}

/// CM Run Mode Setpoint Allowed
pub mod CM_RUN_MODE_MAPPING {

    /// Defines which Setpoint is allowed when CPU enters RUN mode. Each bit stands for 1 Setpoint, locked by LOCK_CFG field
    pub mod CPU_RUN_MODE_MAPPING {
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

/// CM Wait Mode Setpoint Allowed
pub mod CM_WAIT_MODE_MAPPING {

    /// Defines which Setpoint is allowed when CPU enters WAIT mode. Each bit stands for 1 Setpoint, locked by LOCK_CFG
    pub mod CPU_WAIT_MODE_MAPPING {
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

/// CM Stop Mode Setpoint Allowed
pub mod CM_STOP_MODE_MAPPING {

    /// Defines which Setpoint is allowed when CPU enters STOP mode. Each bit stands for 1 Setpoint, locked by LOCK_CFG
    pub mod CPU_STOP_MODE_MAPPING {
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

/// CM Suspend Mode Setpoint Allowed
pub mod CM_SUSPEND_MODE_MAPPING {

    /// Defines which Setpoint is allowed when CPU enters SUSPEND mode. Each bit stands for 1 Setpoint, locked by LOCK_CFG
    pub mod CPU_SUSPEND_MODE_MAPPING {
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

/// CM Setpoint 0 Mapping
pub mod CM_SP0_MAPPING {

    /// Defines when SP0 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP0_MAPPING {
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

/// CM Setpoint 1 Mapping
pub mod CM_SP1_MAPPING {

    /// Defines when SP1 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP1_MAPPING {
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

/// CM Setpoint 2 Mapping
pub mod CM_SP2_MAPPING {

    /// Defines when SP2 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP2_MAPPING {
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

/// CM Setpoint 3 Mapping
pub mod CM_SP3_MAPPING {

    /// Defines when SP3 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP3_MAPPING {
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

/// CM Setpoint 4 Mapping
pub mod CM_SP4_MAPPING {

    /// Defines when SP4 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP4_MAPPING {
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

/// CM Setpoint 5 Mapping
pub mod CM_SP5_MAPPING {

    /// Defines when SP5 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP5_MAPPING {
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

/// CM Setpoint 6 Mapping
pub mod CM_SP6_MAPPING {

    /// Defines when SP6 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP6_MAPPING {
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

/// CM Setpoint 7 Mapping
pub mod CM_SP7_MAPPING {

    /// Defines when SP7 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP7_MAPPING {
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

/// CM Setpoint 8 Mapping
pub mod CM_SP8_MAPPING {

    /// Defines when SP8 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP8_MAPPING {
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

/// CM Setpoint 9 Mapping
pub mod CM_SP9_MAPPING {

    /// Defines when SP9 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP9_MAPPING {
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

/// CM Setpoint 10 Mapping
pub mod CM_SP10_MAPPING {

    /// Defines when SP10 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP10_MAPPING {
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

/// CM Setpoint 11 Mapping
pub mod CM_SP11_MAPPING {

    /// Defines when SP11 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP11_MAPPING {
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

/// CM Setpoint 12 Mapping
pub mod CM_SP12_MAPPING {

    /// Defines when SP12 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP12_MAPPING {
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

/// CM Setpoint 13 Mapping
pub mod CM_SP13_MAPPING {

    /// Defines when SP13 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP13_MAPPING {
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

/// CM Setpoint 14 Mapping
pub mod CM_SP14_MAPPING {

    /// Defines when SP14 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP14_MAPPING {
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

/// CM Setpoint 15 Mapping
pub mod CM_SP15_MAPPING {

    /// Defines when SP15 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field
    pub mod CPU_SP15_MAPPING {
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

/// CM standby control
pub mod CM_STBY_CTRL {

    /// 0x1: Request the chip into standby mode when CPU entering WAIT mode, locked by LOCK_CFG field.
    pub mod STBY_WAIT {
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

    /// 0x1: Request the chip into standby mode when CPU entering STOP mode, locked by LOCK_CFG field.
    pub mod STBY_STOP {
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

    /// 0x1: Request the chip into standby mode when CPU entering SUSPEND mode, locked by LOCK_CFG field.
    pub mod STBY_SUSPEND {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicate the CPU is busy entering standby mode.
    pub mod STBY_SLEEP_BUSY {
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

    /// Indicate the CPU is busy exiting standby mode.
    pub mod STBY_WAKEUP_BUSY {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
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
    _reserved1: [u32; 1],

    /// CM Authentication Control
    pub CM_AUTHEN_CTRL: RWRegister<u32>,

    /// CM Interrupt Control
    pub CM_INT_CTRL: RWRegister<u32>,

    /// Miscellaneous
    pub CM_MISC: RWRegister<u32>,

    /// CPU mode control
    pub CM_MODE_CTRL: RWRegister<u32>,

    /// CM CPU mode Status
    pub CM_MODE_STAT: RORegister<u32>,

    _reserved2: [u32; 58],

    /// CM IRQ0~31 wakeup mask
    pub CM_IRQ_WAKEUP_MASK_0: RWRegister<u32>,

    /// CM IRQ32~63 wakeup mask
    pub CM_IRQ_WAKEUP_MASK_1: RWRegister<u32>,

    /// CM IRQ64~95 wakeup mask
    pub CM_IRQ_WAKEUP_MASK_2: RWRegister<u32>,

    /// CM IRQ96~127 wakeup mask
    pub CM_IRQ_WAKEUP_MASK_3: RWRegister<u32>,

    /// CM IRQ128~159 wakeup mask
    pub CM_IRQ_WAKEUP_MASK_4: RWRegister<u32>,

    /// CM IRQ160~191 wakeup mask
    pub CM_IRQ_WAKEUP_MASK_5: RWRegister<u32>,

    /// CM IRQ192~223 wakeup mask
    pub CM_IRQ_WAKEUP_MASK_6: RWRegister<u32>,

    /// CM IRQ224~255 wakeup mask
    pub CM_IRQ_WAKEUP_MASK_7: RWRegister<u32>,

    _reserved3: [u32; 8],

    /// CM non-irq wakeup mask
    pub CM_NON_IRQ_WAKEUP_MASK: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// CM IRQ0~31 wakeup status
    pub CM_IRQ_WAKEUP_STAT_0: RORegister<u32>,

    /// CM IRQ32~63 wakeup status
    pub CM_IRQ_WAKEUP_STAT_1: RORegister<u32>,

    /// CM IRQ64~95 wakeup status
    pub CM_IRQ_WAKEUP_STAT_2: RORegister<u32>,

    /// CM IRQ96~127 wakeup status
    pub CM_IRQ_WAKEUP_STAT_3: RORegister<u32>,

    /// CM IRQ128~159 wakeup status
    pub CM_IRQ_WAKEUP_STAT_4: RORegister<u32>,

    /// CM IRQ160~191 wakeup status
    pub CM_IRQ_WAKEUP_STAT_5: RORegister<u32>,

    /// CM IRQ192~223 wakeup status
    pub CM_IRQ_WAKEUP_STAT_6: RORegister<u32>,

    /// CM IRQ224~255 wakeup status
    pub CM_IRQ_WAKEUP_STAT_7: RORegister<u32>,

    _reserved5: [u32; 8],

    /// CM non-irq wakeup status
    pub CM_NON_IRQ_WAKEUP_STAT: RORegister<u32>,

    _reserved6: [u32; 27],

    /// CM sleep SSAR control
    pub CM_SLEEP_SSAR_CTRL: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// CM sleep LPCG control
    pub CM_SLEEP_LPCG_CTRL: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// CM sleep PLL control
    pub CM_SLEEP_PLL_CTRL: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// CM sleep isolation control
    pub CM_SLEEP_ISO_CTRL: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// CM sleep reset control
    pub CM_SLEEP_RESET_CTRL: RWRegister<u32>,

    _reserved11: [u32; 1],

    /// CM sleep power control
    pub CM_SLEEP_POWER_CTRL: RWRegister<u32>,

    _reserved12: [u32; 25],

    /// CM wakeup power control
    pub CM_WAKEUP_POWER_CTRL: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// CM wakeup reset control
    pub CM_WAKEUP_RESET_CTRL: RWRegister<u32>,

    _reserved14: [u32; 1],

    /// CM wakeup isolation control
    pub CM_WAKEUP_ISO_CTRL: RWRegister<u32>,

    _reserved15: [u32; 1],

    /// CM wakeup PLL control
    pub CM_WAKEUP_PLL_CTRL: RWRegister<u32>,

    _reserved16: [u32; 1],

    /// CM wakeup LPCG control
    pub CM_WAKEUP_LPCG_CTRL: RWRegister<u32>,

    _reserved17: [u32; 1],

    /// CM wakeup SSAR control
    pub CM_WAKEUP_SSAR_CTRL: RWRegister<u32>,

    _reserved18: [u32; 17],

    /// CM Setpoint Control
    pub CM_SP_CTRL: RWRegister<u32>,

    /// CM Setpoint Status
    pub CM_SP_STAT: RORegister<u32>,

    _reserved19: [u32; 2],

    /// CM Run Mode Setpoint Allowed
    pub CM_RUN_MODE_MAPPING: RWRegister<u32>,

    /// CM Wait Mode Setpoint Allowed
    pub CM_WAIT_MODE_MAPPING: RWRegister<u32>,

    /// CM Stop Mode Setpoint Allowed
    pub CM_STOP_MODE_MAPPING: RWRegister<u32>,

    /// CM Suspend Mode Setpoint Allowed
    pub CM_SUSPEND_MODE_MAPPING: RWRegister<u32>,

    /// CM Setpoint 0 Mapping
    pub CM_SP0_MAPPING: RWRegister<u32>,

    /// CM Setpoint 1 Mapping
    pub CM_SP1_MAPPING: RWRegister<u32>,

    /// CM Setpoint 2 Mapping
    pub CM_SP2_MAPPING: RWRegister<u32>,

    /// CM Setpoint 3 Mapping
    pub CM_SP3_MAPPING: RWRegister<u32>,

    /// CM Setpoint 4 Mapping
    pub CM_SP4_MAPPING: RWRegister<u32>,

    /// CM Setpoint 5 Mapping
    pub CM_SP5_MAPPING: RWRegister<u32>,

    /// CM Setpoint 6 Mapping
    pub CM_SP6_MAPPING: RWRegister<u32>,

    /// CM Setpoint 7 Mapping
    pub CM_SP7_MAPPING: RWRegister<u32>,

    /// CM Setpoint 8 Mapping
    pub CM_SP8_MAPPING: RWRegister<u32>,

    /// CM Setpoint 9 Mapping
    pub CM_SP9_MAPPING: RWRegister<u32>,

    /// CM Setpoint 10 Mapping
    pub CM_SP10_MAPPING: RWRegister<u32>,

    /// CM Setpoint 11 Mapping
    pub CM_SP11_MAPPING: RWRegister<u32>,

    /// CM Setpoint 12 Mapping
    pub CM_SP12_MAPPING: RWRegister<u32>,

    /// CM Setpoint 13 Mapping
    pub CM_SP13_MAPPING: RWRegister<u32>,

    /// CM Setpoint 14 Mapping
    pub CM_SP14_MAPPING: RWRegister<u32>,

    /// CM Setpoint 15 Mapping
    pub CM_SP15_MAPPING: RWRegister<u32>,

    _reserved20: [u32; 8],

    /// CM standby control
    pub CM_STBY_CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub CM_AUTHEN_CTRL: u32,
    pub CM_INT_CTRL: u32,
    pub CM_MISC: u32,
    pub CM_MODE_CTRL: u32,
    pub CM_MODE_STAT: u32,
    pub CM_IRQ_WAKEUP_MASK_0: u32,
    pub CM_IRQ_WAKEUP_MASK_1: u32,
    pub CM_IRQ_WAKEUP_MASK_2: u32,
    pub CM_IRQ_WAKEUP_MASK_3: u32,
    pub CM_IRQ_WAKEUP_MASK_4: u32,
    pub CM_IRQ_WAKEUP_MASK_5: u32,
    pub CM_IRQ_WAKEUP_MASK_6: u32,
    pub CM_IRQ_WAKEUP_MASK_7: u32,
    pub CM_NON_IRQ_WAKEUP_MASK: u32,
    pub CM_IRQ_WAKEUP_STAT_0: u32,
    pub CM_IRQ_WAKEUP_STAT_1: u32,
    pub CM_IRQ_WAKEUP_STAT_2: u32,
    pub CM_IRQ_WAKEUP_STAT_3: u32,
    pub CM_IRQ_WAKEUP_STAT_4: u32,
    pub CM_IRQ_WAKEUP_STAT_5: u32,
    pub CM_IRQ_WAKEUP_STAT_6: u32,
    pub CM_IRQ_WAKEUP_STAT_7: u32,
    pub CM_NON_IRQ_WAKEUP_STAT: u32,
    pub CM_SLEEP_SSAR_CTRL: u32,
    pub CM_SLEEP_LPCG_CTRL: u32,
    pub CM_SLEEP_PLL_CTRL: u32,
    pub CM_SLEEP_ISO_CTRL: u32,
    pub CM_SLEEP_RESET_CTRL: u32,
    pub CM_SLEEP_POWER_CTRL: u32,
    pub CM_WAKEUP_POWER_CTRL: u32,
    pub CM_WAKEUP_RESET_CTRL: u32,
    pub CM_WAKEUP_ISO_CTRL: u32,
    pub CM_WAKEUP_PLL_CTRL: u32,
    pub CM_WAKEUP_LPCG_CTRL: u32,
    pub CM_WAKEUP_SSAR_CTRL: u32,
    pub CM_SP_CTRL: u32,
    pub CM_SP_STAT: u32,
    pub CM_RUN_MODE_MAPPING: u32,
    pub CM_WAIT_MODE_MAPPING: u32,
    pub CM_STOP_MODE_MAPPING: u32,
    pub CM_SUSPEND_MODE_MAPPING: u32,
    pub CM_SP0_MAPPING: u32,
    pub CM_SP1_MAPPING: u32,
    pub CM_SP2_MAPPING: u32,
    pub CM_SP3_MAPPING: u32,
    pub CM_SP4_MAPPING: u32,
    pub CM_SP5_MAPPING: u32,
    pub CM_SP6_MAPPING: u32,
    pub CM_SP7_MAPPING: u32,
    pub CM_SP8_MAPPING: u32,
    pub CM_SP9_MAPPING: u32,
    pub CM_SP10_MAPPING: u32,
    pub CM_SP11_MAPPING: u32,
    pub CM_SP12_MAPPING: u32,
    pub CM_SP13_MAPPING: u32,
    pub CM_SP14_MAPPING: u32,
    pub CM_SP15_MAPPING: u32,
    pub CM_STBY_CTRL: u32,
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
