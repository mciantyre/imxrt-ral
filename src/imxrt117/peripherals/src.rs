#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SRC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// SRC Control Register
pub mod SCR {

    /// cm4 core reset will be held until boot core write this bit to 1 to release it.
    pub mod BT_RELEASE_M4 {
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

            /// 0b0: cm4 core reset is asserted
            pub const BT_RELEASE_M4_0: u32 = 0b0;

            /// 0b1: cm4 core reset is released
            pub const BT_RELEASE_M4_1: u32 = 0b1;
        }
    }

    /// cm7 core reset will be held until boot core write this bit to 1 to release it.
    pub mod BT_RELEASE_M7 {
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

            /// 0b0: cm7 core reset is asserted
            pub const BT_RELEASE_M7_0: u32 = 0b0;

            /// 0b1: cm7 core reset is released
            pub const BT_RELEASE_M7_1: u32 = 0b1;
        }
    }
}

/// SRC Reset Mode Register
pub mod SRMR {

    /// Wdog reset mode configuration
    pub mod WDOG_RESET_MODE {
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

            /// 0b00: reset system
            pub const WDOG_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const WDOG_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// Wdog3 reset mode configuration
    pub mod WDOG3_RESET_MODE {
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

            /// 0b00: reset system
            pub const WDOG3_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const WDOG3_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// Wdog4 reset mode configuration
    pub mod WDOG4_RESET_MODE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: reset system
            pub const WDOG4_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const WDOG4_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// M4 core lockup reset mode configuration
    pub mod M4LOCKUP_RESET_MODE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: reset system
            pub const M4LOCKUP_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const M4LOCKUP_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// M7 core lockup reset mode configuration
    pub mod M7LOCKUP_RESET_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: reset system
            pub const M7LOCKUP_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const M7LOCKUP_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// M4 request reset configuration
    pub mod M4REQ_RESET_MODE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: reset system
            pub const M4REQ_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const M4REQ_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// M7 request reset configuration
    pub mod M7REQ_RESET_MODE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: reset system
            pub const M7REQ_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const M7REQ_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// Tempsense reset mode configuration
    pub mod TEMPSENSE_RESET_MODE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: reset system
            pub const TEMPSENSE_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const TEMPSENSE_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// CSU reset mode configuration
    pub mod CSU_RESET_MODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: reset system
            pub const CSU_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const CSU_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// Jtag SW reset mode configuration
    pub mod JTAGSW_RESET_MODE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: reset system
            pub const JTAGSW_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const JTAGSW_RESET_MODE_3: u32 = 0b11;
        }
    }

    /// Jtag SW reset mode configuration
    pub mod OVERVOLT_RESET_MODE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: reset system
            pub const OVERVOLT_RESET_MODE_0: u32 = 0b00;

            /// 0b11: do not reset anything
            pub const OVERVOLT_RESET_MODE_3: u32 = 0b11;
        }
    }
}

/// SRC Boot Mode Register 1
pub mod SBMR1 {

    /// Please see fusemap.
    pub mod BOOT_CFG1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Please see fusemap.
    pub mod BOOT_CFG2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Please see fusemap.
    pub mod BOOT_CFG3 {
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

    /// Please see fusemap.
    pub mod BOOT_CFG4 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SRC Boot Mode Register 2
pub mod SBMR2 {

    /// SECONFIG\[1\] shows the state of the SECONFIG\[1\] fuse
    pub mod SEC_CONFIG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BT_FUSE_SEL shows the state of the BT_FUSE_SEL fuse
    pub mod BT_FUSE_SEL {
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

    /// BMOD\[1:0\] shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B
    pub mod BMOD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SRC Reset Status Register
pub mod SRSR {

    /// Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)
    pub mod IPP_RESET_B_M7 {
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

            /// 0b0: Reset is not a result of ipp_reset_b pin.
            pub const IPP_RESET_B_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of ipp_reset_b pin.
            pub const IPP_RESET_B_M7_1: u32 = 0b1;
        }
    }

    /// Indicates whether reset was the result of m7 reset request
    pub mod M7_REQUEST_M7 {
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

            /// 0b0: Reset is not a result of m7 reset request.
            pub const M7_REQUEST_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of m7 reset request.
            pub const M7_REQUEST_M7_1: u32 = 0b1;
        }
    }

    /// Indicates a reset has been caused by M7 CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core
    pub mod M7_LOCKUP_M7 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of the mentioned case.
            pub const M7_LOCKUP_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of the mentioned case.
            pub const M7_LOCKUP_M7_1: u32 = 0b1;
        }
    }

    /// Indicates whether the reset was the result of the csu_reset_b input.
    pub mod CSU_RESET_B_M7 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of the csu_reset_b event.
            pub const CSU_RESET_B_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of the csu_reset_b event.
            pub const CSU_RESET_B_M7_1: u32 = 0b1;
        }
    }

    /// Indicates whether the reset was the result of the ipp_user_reset_b qualified reset.
    pub mod IPP_USER_RESET_B_M7 {
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

            /// 0b0: Reset is not a result of the ipp_user_reset_b qualified as COLD reset event.
            pub const IPP_USER_RESET_B_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of the ipp_user_reset_b qualified as COLD reset event.
            pub const IPP_USER_RESET_B_M7_1: u32 = 0b1;
        }
    }

    /// IC Watchdog Time-out reset
    pub mod WDOG_RST_B_M7 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of the watchdog time-out event.
            pub const WDOG_RST_B_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of the watchdog time-out event.
            pub const WDOG_RST_B_M7_1: u32 = 0b1;
        }
    }

    /// HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG.
    pub mod JTAG_RST_B_M7 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of HIGH-Z reset from JTAG.
            pub const JTAG_RST_B_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of HIGH-Z reset from JTAG.
            pub const JTAG_RST_B_M7_1: u32 = 0b1;
        }
    }

    /// JTAG software reset. Indicates whether the reset was the result of software reset from JTAG.
    pub mod JTAG_SW_RST_M7 {
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

            /// 0b0: Reset is not a result of software reset from JTAG.
            pub const JTAG_SW_RST_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of software reset from JTAG.
            pub const JTAG_SW_RST_M7_1: u32 = 0b1;
        }
    }

    /// IC Watchdog3 Time-out reset
    pub mod WDOG3_RST_B_M7 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of the watchdog3 time-out event.
            pub const WDOG3_RST_B_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of the watchdog3 time-out event.
            pub const WDOG3_RST_B_M7_1: u32 = 0b1;
        }
    }

    /// IC Watchdog4 Time-out reset
    pub mod WDOG4_RST_B_M7 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of the watchdog4 time-out event.
            pub const WDOG4_RST_B_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of the watchdog4 time-out event.
            pub const WDOG4_RST_B_M7_1: u32 = 0b1;
        }
    }

    /// Temper Sensor software reset
    pub mod TEMPSENSE_RST_B_M7 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of software reset from Temperature Sensor.
            pub const TEMPSENSE_RST_B_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of software reset from Temperature Sensor.
            pub const TEMPSENSE_RST_B_M7_1: u32 = 0b1;
        }
    }

    /// Indicates whether reset was the result of m4 reset request.
    pub mod M4_REQUEST_M7 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of m4 reset request.
            pub const M4_REQUEST_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of m4 reset request.
            pub const M4_REQUEST_M7_1: u32 = 0b1;
        }
    }

    /// Indicates a reset has been caused by M4 CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core
    pub mod M4_LOCKUP_M7 {
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

            /// 0b0: Reset is not a result of the mentioned case.
            pub const M4_LOCKUP_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of the mentioned case.
            pub const M4_LOCKUP_M7_1: u32 = 0b1;
        }
    }

    /// Indicates a reset has been caused by power suppy voltage over the highest permitted level.
    pub mod OVERVOLT_RST_M7 {
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

            /// 0b0: Reset is not a result of the mentioned case.
            pub const OVERVOLT_RST_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of the mentioned case.
            pub const OVERVOLT_RST_M7_1: u32 = 0b1;
        }
    }

    /// Indicates a reset has been caused by CDOG reset.
    pub mod CDOG_RST_M7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of the mentioned case.
            pub const CDOG_RST_M7_0: u32 = 0b0;

            /// 0b1: Reset is a result of the mentioned case.
            pub const CDOG_RST_M7_1: u32 = 0b1;
        }
    }

    /// Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)
    pub mod IPP_RESET_B_M4 {
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

            /// 0b0: Reset is not a result of ipp_reset_b pin.
            pub const IPP_RESET_B_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of ipp_reset_b pin.
            pub const IPP_RESET_B_M4_1: u32 = 0b1;
        }
    }

    /// Indicates whether reset was the result of m4 reset request
    pub mod M4_REQUEST_M4 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of m4 reset request.
            pub const M4_REQUEST_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of m4 reset request.
            pub const M4_REQUEST_M4_1: u32 = 0b1;
        }
    }

    /// Indicates a reset has been caused by M4 CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core
    pub mod M4_LOCKUP_M4 {
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

            /// 0b0: Reset is not a result of the mentioned case.
            pub const M4_LOCKUP_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of the mentioned case.
            pub const M4_LOCKUP_M4_1: u32 = 0b1;
        }
    }

    /// Indicates whether the reset was the result of the csu_reset_b input.
    pub mod CSU_RESET_B_M4 {
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

            /// 0b0: Reset is not a result of the csu_reset_b event.
            pub const CSU_RESET_B_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of the csu_reset_b event.
            pub const CSU_RESET_B_M4_1: u32 = 0b1;
        }
    }

    /// Indicates whether the reset was the result of the ipp_user_reset_b qualified reset.
    pub mod IPP_USER_RESET_B_M4 {
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

            /// 0b0: Reset is not a result of the ipp_user_reset_b qualified as COLD reset event.
            pub const IPP_USER_RESET_B_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of the ipp_user_reset_b qualified as COLD reset event.
            pub const IPP_USER_RESET_B_M4_1: u32 = 0b1;
        }
    }

    /// IC Watchdog Time-out reset
    pub mod WDOG_RST_B_M4 {
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

            /// 0b0: Reset is not a result of the watchdog time-out event.
            pub const WDOG_RST_B_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of the watchdog time-out event.
            pub const WDOG_RST_B_M4_1: u32 = 0b1;
        }
    }

    /// HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG.
    pub mod JTAG_RST_B_M4 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of HIGH-Z reset from JTAG.
            pub const JTAG_RST_B_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of HIGH-Z reset from JTAG.
            pub const JTAG_RST_B_M4_1: u32 = 0b1;
        }
    }

    /// JTAG software reset. Indicates whether the reset was the result of software reset from JTAG.
    pub mod JTAG_SW_RST_M4 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of software reset from JTAG.
            pub const JTAG_SW_RST_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of software reset from JTAG.
            pub const JTAG_SW_RST_M4_1: u32 = 0b1;
        }
    }

    /// IC Watchdog3 Time-out reset
    pub mod WDOG3_RST_B_M4 {
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

            /// 0b0: Reset is not a result of the watchdog3 time-out event.
            pub const WDOG3_RST_B_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of the watchdog3 time-out event.
            pub const WDOG3_RST_B_M4_1: u32 = 0b1;
        }
    }

    /// IC Watchdog4 Time-out reset
    pub mod WDOG4_RST_B_M4 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of the watchdog4 time-out event.
            pub const WDOG4_RST_B_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of the watchdog4 time-out event.
            pub const WDOG4_RST_B_M4_1: u32 = 0b1;
        }
    }

    /// Temper Sensor software reset
    pub mod TEMPSENSE_RST_B_M4 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of software reset from Temperature Sensor.
            pub const TEMPSENSE_RST_B_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of software reset from Temperature Sensor.
            pub const TEMPSENSE_RST_B_M4_1: u32 = 0b1;
        }
    }

    /// Indicates whether reset was the result of m7 reset request.
    pub mod M7_REQUEST_M4 {
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

            /// 0b0: Reset is not a result of m7 reset request.
            pub const M7_REQUEST_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of m7 reset request.
            pub const M7_REQUEST_M4_1: u32 = 0b1;
        }
    }

    /// Indicates a reset has been caused by M7 CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core
    pub mod M7_LOCKUP_M4 {
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

            /// 0b0: Reset is not a result of the mentioned case.
            pub const M7_LOCKUP_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of the mentioned case.
            pub const M7_LOCKUP_M4_1: u32 = 0b1;
        }
    }

    /// Indicates a reset has been caused by power suppy voltage over the highest permitted level.
    pub mod OVERVOLT_RST_M4 {
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

            /// 0b0: Reset is not a result of the mentioned case.
            pub const OVERVOLT_RST_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of the mentioned case.
            pub const OVERVOLT_RST_M4_1: u32 = 0b1;
        }
    }

    /// Indicates a reset has been caused by CDOG reset.
    pub mod CDOG_RST_M4 {
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

            /// 0b0: Reset is not a result of the mentioned case.
            pub const CDOG_RST_M4_0: u32 = 0b0;

            /// 0b1: Reset is a result of the mentioned case.
            pub const CDOG_RST_M4_1: u32 = 0b1;
        }
    }
}

/// SRC General Purpose Register
pub mod GPR1 {

    /// General Purpose Register.
    pub mod GPR {
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

/// SRC General Purpose Register
pub mod GPR2 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR3 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR4 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR5 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR6 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR7 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR8 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR9 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR10 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR11 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR12 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR13 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR14 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR15 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR16 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR17 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR18 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR19 {
    pub use super::GPR1::GPR;
}

/// SRC General Purpose Register
pub mod GPR20 {
    pub use super::GPR1::GPR;
}

/// Slice Authentication Register
pub mod AUTHEN_MEGA {

    /// Control whether reset slice is in domain mode
    pub mod DOMAIN_MODE {
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

            /// 0b0: slice hardware reset will NOT be triggered by CPU power mode transition
            pub const DOMAIN_MODE_0: u32 = 0b0;

            /// 0b1: slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time.
            pub const DOMAIN_MODE_1: u32 = 0b1;
        }
    }

    /// Control whether reset slice is in Setpoint mode
    pub mod SETPOINT_MODE {
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

            /// 0b0: slice hardware reset will NOT be triggered by Setpoint transition
            pub const SETPOINT_MODE_0: u32 = 0b0;

            /// 0b1: slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time.
            pub const SETPOINT_MODE_1: u32 = 0b1;
        }
    }

    /// Domain/Setpoint mode lock
    pub mod LOCK_MODE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// when this bitfield set to 1, reset of slice would be subject to corresponding core status transition
    pub mod ASSIGN_LIST {
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

    /// Assign list lock
    pub mod LOCK_ASSIGN {
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

    /// Domain ID white list
    pub mod WHITE_LIST {
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

    /// White list lock
    pub mod LOCK_LIST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Allow user mode access
    pub mod USER {
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

    /// Allow non-secure mode access
    pub mod NONSECURE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock NONSECURE and USER
    pub mod LOCK_SETTING {
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

/// Slice Control Register
pub mod CTRL_MEGA {

    /// This is a self clearing bit
    pub mod SW_RESET {
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

            /// 0b0: do not assert slice software reset
            pub const SW_RESET_0: u32 = 0b0;

            /// 0b1: assert slice software reset
            pub const SW_RESET_1: u32 = 0b1;
        }
    }
}

/// Slice Setpoint Config Register
pub mod SETPOINT_MEGA {

    /// SETPOINT0
    pub mod SETPOINT0 {
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

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT0_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT0_1: u32 = 0b1;
        }
    }

    /// SETPOINT1
    pub mod SETPOINT1 {
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

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT1_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT1_1: u32 = 0b1;
        }
    }

    /// SETPOINT2
    pub mod SETPOINT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT2_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT2_1: u32 = 0b1;
        }
    }

    /// SETPOINT3
    pub mod SETPOINT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT3_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT3_1: u32 = 0b1;
        }
    }

    /// SETPOINT4
    pub mod SETPOINT4 {
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

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT4_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT4_1: u32 = 0b1;
        }
    }

    /// SETPOINT5
    pub mod SETPOINT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT5_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT5_1: u32 = 0b1;
        }
    }

    /// SETPOINT6
    pub mod SETPOINT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT6_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT6_1: u32 = 0b1;
        }
    }

    /// SETPOINT7
    pub mod SETPOINT7 {
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

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT7_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT7_1: u32 = 0b1;
        }
    }

    /// SETPOINT8
    pub mod SETPOINT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT8_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT8_1: u32 = 0b1;
        }
    }

    /// SETPOINT9
    pub mod SETPOINT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT9_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT9_1: u32 = 0b1;
        }
    }

    /// SETPOINT10
    pub mod SETPOINT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT10_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT10_1: u32 = 0b1;
        }
    }

    /// SETPOINT11
    pub mod SETPOINT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT11_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT11_1: u32 = 0b1;
        }
    }

    /// SETPOINT12
    pub mod SETPOINT12 {
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

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT12_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT12_1: u32 = 0b1;
        }
    }

    /// SETPOINT13
    pub mod SETPOINT13 {
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

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT13_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT13_1: u32 = 0b1;
        }
    }

    /// SETPOINT14
    pub mod SETPOINT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT14_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT14_1: u32 = 0b1;
        }
    }

    /// SETPOINT15
    pub mod SETPOINT15 {
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

            /// 0b0: Slice reset will be de-asserted when system in Setpoint n
            pub const SETPOINT15_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when system in Setpoint n
            pub const SETPOINT15_1: u32 = 0b1;
        }
    }
}

/// Slice Domain Config Register
pub mod DOMAIN_MEGA {

    /// CPU mode setting for RUN
    pub mod CPU0_RUN {
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

            /// 0b0: Slice reset will be de-asserted when CPU0 in RUN mode
            pub const CPU0_RUN_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when CPU0 in RUN mode
            pub const CPU0_RUN_1: u32 = 0b1;
        }
    }

    /// CPU mode setting for WAIT
    pub mod CPU0_WAIT {
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

            /// 0b0: Slice reset will be de-asserted when CPU0 in WAIT mode
            pub const CPU0_WAIT_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when CPU0 in WAIT mode
            pub const CPU0_WAIT_1: u32 = 0b1;
        }
    }

    /// CPU mode setting for STOP
    pub mod CPU0_STOP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when CPU0 in STOP mode
            pub const CPU0_STOP_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when CPU0 in STOP mode
            pub const CPU0_STOP_1: u32 = 0b1;
        }
    }

    /// CPU mode setting for SUSPEND
    pub mod CPU0_SUSP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when CPU0 in SUSPEND mode
            pub const CPU0_SUSP_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when CPU0 in SUSPEND mode
            pub const CPU0_SUSP_1: u32 = 0b1;
        }
    }

    /// CPU mode setting for RUN
    pub mod CPU1_RUN {
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

            /// 0b0: Slice reset will be de-asserted when CPU1 in RUN mode
            pub const CPU1_RUN_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when CPU1 in RUN mode
            pub const CPU1_RUN_1: u32 = 0b1;
        }
    }

    /// CPU mode setting for WAIT
    pub mod CPU1_WAIT {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when CPU1 in WAIT mode
            pub const CPU1_WAIT_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when CPU1 in WAIT mode
            pub const CPU1_WAIT_1: u32 = 0b1;
        }
    }

    /// CPU mode setting for STOP
    pub mod CPU1_STOP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slice reset will be de-asserted when CPU1 in STOP mode
            pub const CPU1_STOP_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when CPU1 in STOP mode
            pub const CPU1_STOP_1: u32 = 0b1;
        }
    }

    /// CPU mode setting for SUSPEND
    pub mod CPU1_SUSP {
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

            /// 0b0: Slice reset will be de-asserted when CPU1 in SUSPEND mode
            pub const CPU1_SUSP_0: u32 = 0b0;

            /// 0b1: Slice reset will be asserted when CPU1 in SUSPEND mode
            pub const CPU1_SUSP_1: u32 = 0b1;
        }
    }
}

/// Slice Status Register
pub mod STAT_MEGA {

    /// This is a Read Only bit. It indicate if the reset is in process.
    pub mod UNDER_RST {
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

            /// 0b0: the reset is finished
            pub const UNDER_RST_0: u32 = 0b0;

            /// 0b1: the reset is in process
            pub const UNDER_RST_1: u32 = 0b1;
        }
    }

    /// This bit indicate if the reset is caused by the power mode transfer.
    pub mod RST_BY_HW {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: the reset is not caused by the power mode transfer
            pub const RST_BY_HW_0: u32 = 0b0;

            /// 0b1: the reset is caused by the power mode transfer
            pub const RST_BY_HW_1: u32 = 0b1;
        }
    }

    /// This bit indicate if the reset is caused by setting SW_RESET bit.
    pub mod RST_BY_SW {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: the reset is not caused by software setting
            pub const RST_BY_SW_0: u32 = 0b0;

            /// 0b1: the reset is caused by software setting
            pub const RST_BY_SW_1: u32 = 0b1;
        }
    }
}

/// Slice Authentication Register
pub mod AUTHEN_DISPLAY {
    pub use super::AUTHEN_MEGA::ASSIGN_LIST;
    pub use super::AUTHEN_MEGA::DOMAIN_MODE;
    pub use super::AUTHEN_MEGA::LOCK_ASSIGN;
    pub use super::AUTHEN_MEGA::LOCK_LIST;
    pub use super::AUTHEN_MEGA::LOCK_MODE;
    pub use super::AUTHEN_MEGA::LOCK_SETTING;
    pub use super::AUTHEN_MEGA::NONSECURE;
    pub use super::AUTHEN_MEGA::SETPOINT_MODE;
    pub use super::AUTHEN_MEGA::USER;
    pub use super::AUTHEN_MEGA::WHITE_LIST;
}

/// Slice Control Register
pub mod CTRL_DISPLAY {
    pub use super::CTRL_MEGA::SW_RESET;
}

/// Slice Setpoint Config Register
pub mod SETPOINT_DISPLAY {
    pub use super::SETPOINT_MEGA::SETPOINT0;
    pub use super::SETPOINT_MEGA::SETPOINT1;
    pub use super::SETPOINT_MEGA::SETPOINT10;
    pub use super::SETPOINT_MEGA::SETPOINT11;
    pub use super::SETPOINT_MEGA::SETPOINT12;
    pub use super::SETPOINT_MEGA::SETPOINT13;
    pub use super::SETPOINT_MEGA::SETPOINT14;
    pub use super::SETPOINT_MEGA::SETPOINT15;
    pub use super::SETPOINT_MEGA::SETPOINT2;
    pub use super::SETPOINT_MEGA::SETPOINT3;
    pub use super::SETPOINT_MEGA::SETPOINT4;
    pub use super::SETPOINT_MEGA::SETPOINT5;
    pub use super::SETPOINT_MEGA::SETPOINT6;
    pub use super::SETPOINT_MEGA::SETPOINT7;
    pub use super::SETPOINT_MEGA::SETPOINT8;
    pub use super::SETPOINT_MEGA::SETPOINT9;
}

/// Slice Domain Config Register
pub mod DOMAIN_DISPLAY {
    pub use super::DOMAIN_MEGA::CPU0_RUN;
    pub use super::DOMAIN_MEGA::CPU0_STOP;
    pub use super::DOMAIN_MEGA::CPU0_SUSP;
    pub use super::DOMAIN_MEGA::CPU0_WAIT;
    pub use super::DOMAIN_MEGA::CPU1_RUN;
    pub use super::DOMAIN_MEGA::CPU1_STOP;
    pub use super::DOMAIN_MEGA::CPU1_SUSP;
    pub use super::DOMAIN_MEGA::CPU1_WAIT;
}

/// Slice Status Register
pub mod STAT_DISPLAY {
    pub use super::STAT_MEGA::RST_BY_HW;
    pub use super::STAT_MEGA::RST_BY_SW;
    pub use super::STAT_MEGA::UNDER_RST;
}

/// Slice Authentication Register
pub mod AUTHEN_WAKEUP {
    pub use super::AUTHEN_MEGA::ASSIGN_LIST;
    pub use super::AUTHEN_MEGA::DOMAIN_MODE;
    pub use super::AUTHEN_MEGA::LOCK_ASSIGN;
    pub use super::AUTHEN_MEGA::LOCK_LIST;
    pub use super::AUTHEN_MEGA::LOCK_MODE;
    pub use super::AUTHEN_MEGA::LOCK_SETTING;
    pub use super::AUTHEN_MEGA::NONSECURE;
    pub use super::AUTHEN_MEGA::SETPOINT_MODE;
    pub use super::AUTHEN_MEGA::USER;
    pub use super::AUTHEN_MEGA::WHITE_LIST;
}

/// Slice Control Register
pub mod CTRL_WAKEUP {
    pub use super::CTRL_MEGA::SW_RESET;
}

/// Slice Setpoint Config Register
pub mod SETPOINT_WAKEUP {
    pub use super::SETPOINT_MEGA::SETPOINT0;
    pub use super::SETPOINT_MEGA::SETPOINT1;
    pub use super::SETPOINT_MEGA::SETPOINT10;
    pub use super::SETPOINT_MEGA::SETPOINT11;
    pub use super::SETPOINT_MEGA::SETPOINT12;
    pub use super::SETPOINT_MEGA::SETPOINT13;
    pub use super::SETPOINT_MEGA::SETPOINT14;
    pub use super::SETPOINT_MEGA::SETPOINT15;
    pub use super::SETPOINT_MEGA::SETPOINT2;
    pub use super::SETPOINT_MEGA::SETPOINT3;
    pub use super::SETPOINT_MEGA::SETPOINT4;
    pub use super::SETPOINT_MEGA::SETPOINT5;
    pub use super::SETPOINT_MEGA::SETPOINT6;
    pub use super::SETPOINT_MEGA::SETPOINT7;
    pub use super::SETPOINT_MEGA::SETPOINT8;
    pub use super::SETPOINT_MEGA::SETPOINT9;
}

/// Slice Domain Config Register
pub mod DOMAIN_WAKEUP {
    pub use super::DOMAIN_MEGA::CPU0_RUN;
    pub use super::DOMAIN_MEGA::CPU0_STOP;
    pub use super::DOMAIN_MEGA::CPU0_SUSP;
    pub use super::DOMAIN_MEGA::CPU0_WAIT;
    pub use super::DOMAIN_MEGA::CPU1_RUN;
    pub use super::DOMAIN_MEGA::CPU1_STOP;
    pub use super::DOMAIN_MEGA::CPU1_SUSP;
    pub use super::DOMAIN_MEGA::CPU1_WAIT;
}

/// Slice Status Register
pub mod STAT_WAKEUP {
    pub use super::STAT_MEGA::RST_BY_HW;
    pub use super::STAT_MEGA::RST_BY_SW;
    pub use super::STAT_MEGA::UNDER_RST;
}

/// Slice Authentication Register
pub mod AUTHEN_M4CORE {
    pub use super::AUTHEN_MEGA::ASSIGN_LIST;
    pub use super::AUTHEN_MEGA::DOMAIN_MODE;
    pub use super::AUTHEN_MEGA::LOCK_ASSIGN;
    pub use super::AUTHEN_MEGA::LOCK_LIST;
    pub use super::AUTHEN_MEGA::LOCK_MODE;
    pub use super::AUTHEN_MEGA::LOCK_SETTING;
    pub use super::AUTHEN_MEGA::NONSECURE;
    pub use super::AUTHEN_MEGA::SETPOINT_MODE;
    pub use super::AUTHEN_MEGA::USER;
    pub use super::AUTHEN_MEGA::WHITE_LIST;
}

/// Slice Control Register
pub mod CTRL_M4CORE {
    pub use super::CTRL_MEGA::SW_RESET;
}

/// Slice Setpoint Config Register
pub mod SETPOINT_M4CORE {
    pub use super::SETPOINT_MEGA::SETPOINT0;
    pub use super::SETPOINT_MEGA::SETPOINT1;
    pub use super::SETPOINT_MEGA::SETPOINT10;
    pub use super::SETPOINT_MEGA::SETPOINT11;
    pub use super::SETPOINT_MEGA::SETPOINT12;
    pub use super::SETPOINT_MEGA::SETPOINT13;
    pub use super::SETPOINT_MEGA::SETPOINT14;
    pub use super::SETPOINT_MEGA::SETPOINT15;
    pub use super::SETPOINT_MEGA::SETPOINT2;
    pub use super::SETPOINT_MEGA::SETPOINT3;
    pub use super::SETPOINT_MEGA::SETPOINT4;
    pub use super::SETPOINT_MEGA::SETPOINT5;
    pub use super::SETPOINT_MEGA::SETPOINT6;
    pub use super::SETPOINT_MEGA::SETPOINT7;
    pub use super::SETPOINT_MEGA::SETPOINT8;
    pub use super::SETPOINT_MEGA::SETPOINT9;
}

/// Slice Domain Config Register
pub mod DOMAIN_M4CORE {
    pub use super::DOMAIN_MEGA::CPU0_RUN;
    pub use super::DOMAIN_MEGA::CPU0_STOP;
    pub use super::DOMAIN_MEGA::CPU0_SUSP;
    pub use super::DOMAIN_MEGA::CPU0_WAIT;
    pub use super::DOMAIN_MEGA::CPU1_RUN;
    pub use super::DOMAIN_MEGA::CPU1_STOP;
    pub use super::DOMAIN_MEGA::CPU1_SUSP;
    pub use super::DOMAIN_MEGA::CPU1_WAIT;
}

/// Slice Status Register
pub mod STAT_M4CORE {
    pub use super::STAT_MEGA::RST_BY_HW;
    pub use super::STAT_MEGA::RST_BY_SW;
    pub use super::STAT_MEGA::UNDER_RST;
}

/// Slice Authentication Register
pub mod AUTHEN_M7CORE {
    pub use super::AUTHEN_MEGA::ASSIGN_LIST;
    pub use super::AUTHEN_MEGA::DOMAIN_MODE;
    pub use super::AUTHEN_MEGA::LOCK_ASSIGN;
    pub use super::AUTHEN_MEGA::LOCK_LIST;
    pub use super::AUTHEN_MEGA::LOCK_MODE;
    pub use super::AUTHEN_MEGA::LOCK_SETTING;
    pub use super::AUTHEN_MEGA::NONSECURE;
    pub use super::AUTHEN_MEGA::SETPOINT_MODE;
    pub use super::AUTHEN_MEGA::USER;
    pub use super::AUTHEN_MEGA::WHITE_LIST;
}

/// Slice Control Register
pub mod CTRL_M7CORE {
    pub use super::CTRL_MEGA::SW_RESET;
}

/// Slice Setpoint Config Register
pub mod SETPOINT_M7CORE {
    pub use super::SETPOINT_MEGA::SETPOINT0;
    pub use super::SETPOINT_MEGA::SETPOINT1;
    pub use super::SETPOINT_MEGA::SETPOINT10;
    pub use super::SETPOINT_MEGA::SETPOINT11;
    pub use super::SETPOINT_MEGA::SETPOINT12;
    pub use super::SETPOINT_MEGA::SETPOINT13;
    pub use super::SETPOINT_MEGA::SETPOINT14;
    pub use super::SETPOINT_MEGA::SETPOINT15;
    pub use super::SETPOINT_MEGA::SETPOINT2;
    pub use super::SETPOINT_MEGA::SETPOINT3;
    pub use super::SETPOINT_MEGA::SETPOINT4;
    pub use super::SETPOINT_MEGA::SETPOINT5;
    pub use super::SETPOINT_MEGA::SETPOINT6;
    pub use super::SETPOINT_MEGA::SETPOINT7;
    pub use super::SETPOINT_MEGA::SETPOINT8;
    pub use super::SETPOINT_MEGA::SETPOINT9;
}

/// Slice Domain Config Register
pub mod DOMAIN_M7CORE {
    pub use super::DOMAIN_MEGA::CPU0_RUN;
    pub use super::DOMAIN_MEGA::CPU0_STOP;
    pub use super::DOMAIN_MEGA::CPU0_SUSP;
    pub use super::DOMAIN_MEGA::CPU0_WAIT;
    pub use super::DOMAIN_MEGA::CPU1_RUN;
    pub use super::DOMAIN_MEGA::CPU1_STOP;
    pub use super::DOMAIN_MEGA::CPU1_SUSP;
    pub use super::DOMAIN_MEGA::CPU1_WAIT;
}

/// Slice Status Register
pub mod STAT_M7CORE {
    pub use super::STAT_MEGA::RST_BY_HW;
    pub use super::STAT_MEGA::RST_BY_SW;
    pub use super::STAT_MEGA::UNDER_RST;
}

/// Slice Authentication Register
pub mod AUTHEN_M4DEBUG {
    pub use super::AUTHEN_MEGA::ASSIGN_LIST;
    pub use super::AUTHEN_MEGA::DOMAIN_MODE;
    pub use super::AUTHEN_MEGA::LOCK_ASSIGN;
    pub use super::AUTHEN_MEGA::LOCK_LIST;
    pub use super::AUTHEN_MEGA::LOCK_MODE;
    pub use super::AUTHEN_MEGA::LOCK_SETTING;
    pub use super::AUTHEN_MEGA::NONSECURE;
    pub use super::AUTHEN_MEGA::SETPOINT_MODE;
    pub use super::AUTHEN_MEGA::USER;
    pub use super::AUTHEN_MEGA::WHITE_LIST;
}

/// Slice Control Register
pub mod CTRL_M4DEBUG {
    pub use super::CTRL_MEGA::SW_RESET;
}

/// Slice Setpoint Config Register
pub mod SETPOINT_M4DEBUG {
    pub use super::SETPOINT_MEGA::SETPOINT0;
    pub use super::SETPOINT_MEGA::SETPOINT1;
    pub use super::SETPOINT_MEGA::SETPOINT10;
    pub use super::SETPOINT_MEGA::SETPOINT11;
    pub use super::SETPOINT_MEGA::SETPOINT12;
    pub use super::SETPOINT_MEGA::SETPOINT13;
    pub use super::SETPOINT_MEGA::SETPOINT14;
    pub use super::SETPOINT_MEGA::SETPOINT15;
    pub use super::SETPOINT_MEGA::SETPOINT2;
    pub use super::SETPOINT_MEGA::SETPOINT3;
    pub use super::SETPOINT_MEGA::SETPOINT4;
    pub use super::SETPOINT_MEGA::SETPOINT5;
    pub use super::SETPOINT_MEGA::SETPOINT6;
    pub use super::SETPOINT_MEGA::SETPOINT7;
    pub use super::SETPOINT_MEGA::SETPOINT8;
    pub use super::SETPOINT_MEGA::SETPOINT9;
}

/// Slice Domain Config Register
pub mod DOMAIN_M4DEBUG {
    pub use super::DOMAIN_MEGA::CPU0_RUN;
    pub use super::DOMAIN_MEGA::CPU0_STOP;
    pub use super::DOMAIN_MEGA::CPU0_SUSP;
    pub use super::DOMAIN_MEGA::CPU0_WAIT;
    pub use super::DOMAIN_MEGA::CPU1_RUN;
    pub use super::DOMAIN_MEGA::CPU1_STOP;
    pub use super::DOMAIN_MEGA::CPU1_SUSP;
    pub use super::DOMAIN_MEGA::CPU1_WAIT;
}

/// Slice Status Register
pub mod STAT_M4DEBUG {
    pub use super::STAT_MEGA::RST_BY_HW;
    pub use super::STAT_MEGA::RST_BY_SW;
    pub use super::STAT_MEGA::UNDER_RST;
}

/// Slice Authentication Register
pub mod AUTHEN_M7DEBUG {
    pub use super::AUTHEN_MEGA::ASSIGN_LIST;
    pub use super::AUTHEN_MEGA::DOMAIN_MODE;
    pub use super::AUTHEN_MEGA::LOCK_ASSIGN;
    pub use super::AUTHEN_MEGA::LOCK_LIST;
    pub use super::AUTHEN_MEGA::LOCK_MODE;
    pub use super::AUTHEN_MEGA::LOCK_SETTING;
    pub use super::AUTHEN_MEGA::NONSECURE;
    pub use super::AUTHEN_MEGA::SETPOINT_MODE;
    pub use super::AUTHEN_MEGA::USER;
    pub use super::AUTHEN_MEGA::WHITE_LIST;
}

/// Slice Control Register
pub mod CTRL_M7DEBUG {
    pub use super::CTRL_MEGA::SW_RESET;
}

/// Slice Setpoint Config Register
pub mod SETPOINT_M7DEBUG {
    pub use super::SETPOINT_MEGA::SETPOINT0;
    pub use super::SETPOINT_MEGA::SETPOINT1;
    pub use super::SETPOINT_MEGA::SETPOINT10;
    pub use super::SETPOINT_MEGA::SETPOINT11;
    pub use super::SETPOINT_MEGA::SETPOINT12;
    pub use super::SETPOINT_MEGA::SETPOINT13;
    pub use super::SETPOINT_MEGA::SETPOINT14;
    pub use super::SETPOINT_MEGA::SETPOINT15;
    pub use super::SETPOINT_MEGA::SETPOINT2;
    pub use super::SETPOINT_MEGA::SETPOINT3;
    pub use super::SETPOINT_MEGA::SETPOINT4;
    pub use super::SETPOINT_MEGA::SETPOINT5;
    pub use super::SETPOINT_MEGA::SETPOINT6;
    pub use super::SETPOINT_MEGA::SETPOINT7;
    pub use super::SETPOINT_MEGA::SETPOINT8;
    pub use super::SETPOINT_MEGA::SETPOINT9;
}

/// Slice Domain Config Register
pub mod DOMAIN_M7DEBUG {
    pub use super::DOMAIN_MEGA::CPU0_RUN;
    pub use super::DOMAIN_MEGA::CPU0_STOP;
    pub use super::DOMAIN_MEGA::CPU0_SUSP;
    pub use super::DOMAIN_MEGA::CPU0_WAIT;
    pub use super::DOMAIN_MEGA::CPU1_RUN;
    pub use super::DOMAIN_MEGA::CPU1_STOP;
    pub use super::DOMAIN_MEGA::CPU1_SUSP;
    pub use super::DOMAIN_MEGA::CPU1_WAIT;
}

/// Slice Status Register
pub mod STAT_M7DEBUG {
    pub use super::STAT_MEGA::RST_BY_HW;
    pub use super::STAT_MEGA::RST_BY_SW;
    pub use super::STAT_MEGA::UNDER_RST;
}

/// Slice Authentication Register
pub mod AUTHEN_USBPHY1 {
    pub use super::AUTHEN_MEGA::ASSIGN_LIST;
    pub use super::AUTHEN_MEGA::DOMAIN_MODE;
    pub use super::AUTHEN_MEGA::LOCK_ASSIGN;
    pub use super::AUTHEN_MEGA::LOCK_LIST;
    pub use super::AUTHEN_MEGA::LOCK_MODE;
    pub use super::AUTHEN_MEGA::LOCK_SETTING;
    pub use super::AUTHEN_MEGA::NONSECURE;
    pub use super::AUTHEN_MEGA::SETPOINT_MODE;
    pub use super::AUTHEN_MEGA::USER;
    pub use super::AUTHEN_MEGA::WHITE_LIST;
}

/// Slice Control Register
pub mod CTRL_USBPHY1 {
    pub use super::CTRL_MEGA::SW_RESET;
}

/// Slice Setpoint Config Register
pub mod SETPOINT_USBPHY1 {
    pub use super::SETPOINT_MEGA::SETPOINT0;
    pub use super::SETPOINT_MEGA::SETPOINT1;
    pub use super::SETPOINT_MEGA::SETPOINT10;
    pub use super::SETPOINT_MEGA::SETPOINT11;
    pub use super::SETPOINT_MEGA::SETPOINT12;
    pub use super::SETPOINT_MEGA::SETPOINT13;
    pub use super::SETPOINT_MEGA::SETPOINT14;
    pub use super::SETPOINT_MEGA::SETPOINT15;
    pub use super::SETPOINT_MEGA::SETPOINT2;
    pub use super::SETPOINT_MEGA::SETPOINT3;
    pub use super::SETPOINT_MEGA::SETPOINT4;
    pub use super::SETPOINT_MEGA::SETPOINT5;
    pub use super::SETPOINT_MEGA::SETPOINT6;
    pub use super::SETPOINT_MEGA::SETPOINT7;
    pub use super::SETPOINT_MEGA::SETPOINT8;
    pub use super::SETPOINT_MEGA::SETPOINT9;
}

/// Slice Domain Config Register
pub mod DOMAIN_USBPHY1 {
    pub use super::DOMAIN_MEGA::CPU0_RUN;
    pub use super::DOMAIN_MEGA::CPU0_STOP;
    pub use super::DOMAIN_MEGA::CPU0_SUSP;
    pub use super::DOMAIN_MEGA::CPU0_WAIT;
    pub use super::DOMAIN_MEGA::CPU1_RUN;
    pub use super::DOMAIN_MEGA::CPU1_STOP;
    pub use super::DOMAIN_MEGA::CPU1_SUSP;
    pub use super::DOMAIN_MEGA::CPU1_WAIT;
}

/// Slice Status Register
pub mod STAT_USBPHY1 {
    pub use super::STAT_MEGA::RST_BY_HW;
    pub use super::STAT_MEGA::RST_BY_SW;
    pub use super::STAT_MEGA::UNDER_RST;
}

/// Slice Authentication Register
pub mod AUTHEN_USBPHY2 {
    pub use super::AUTHEN_MEGA::ASSIGN_LIST;
    pub use super::AUTHEN_MEGA::DOMAIN_MODE;
    pub use super::AUTHEN_MEGA::LOCK_ASSIGN;
    pub use super::AUTHEN_MEGA::LOCK_LIST;
    pub use super::AUTHEN_MEGA::LOCK_MODE;
    pub use super::AUTHEN_MEGA::LOCK_SETTING;
    pub use super::AUTHEN_MEGA::NONSECURE;
    pub use super::AUTHEN_MEGA::SETPOINT_MODE;
    pub use super::AUTHEN_MEGA::USER;
    pub use super::AUTHEN_MEGA::WHITE_LIST;
}

/// Slice Control Register
pub mod CTRL_USBPHY2 {
    pub use super::CTRL_MEGA::SW_RESET;
}

/// Slice Setpoint Config Register
pub mod SETPOINT_USBPHY2 {
    pub use super::SETPOINT_MEGA::SETPOINT0;
    pub use super::SETPOINT_MEGA::SETPOINT1;
    pub use super::SETPOINT_MEGA::SETPOINT10;
    pub use super::SETPOINT_MEGA::SETPOINT11;
    pub use super::SETPOINT_MEGA::SETPOINT12;
    pub use super::SETPOINT_MEGA::SETPOINT13;
    pub use super::SETPOINT_MEGA::SETPOINT14;
    pub use super::SETPOINT_MEGA::SETPOINT15;
    pub use super::SETPOINT_MEGA::SETPOINT2;
    pub use super::SETPOINT_MEGA::SETPOINT3;
    pub use super::SETPOINT_MEGA::SETPOINT4;
    pub use super::SETPOINT_MEGA::SETPOINT5;
    pub use super::SETPOINT_MEGA::SETPOINT6;
    pub use super::SETPOINT_MEGA::SETPOINT7;
    pub use super::SETPOINT_MEGA::SETPOINT8;
    pub use super::SETPOINT_MEGA::SETPOINT9;
}

/// Slice Domain Config Register
pub mod DOMAIN_USBPHY2 {
    pub use super::DOMAIN_MEGA::CPU0_RUN;
    pub use super::DOMAIN_MEGA::CPU0_STOP;
    pub use super::DOMAIN_MEGA::CPU0_SUSP;
    pub use super::DOMAIN_MEGA::CPU0_WAIT;
    pub use super::DOMAIN_MEGA::CPU1_RUN;
    pub use super::DOMAIN_MEGA::CPU1_STOP;
    pub use super::DOMAIN_MEGA::CPU1_SUSP;
    pub use super::DOMAIN_MEGA::CPU1_WAIT;
}

/// Slice Status Register
pub mod STAT_USBPHY2 {
    pub use super::STAT_MEGA::RST_BY_HW;
    pub use super::STAT_MEGA::RST_BY_SW;
    pub use super::STAT_MEGA::UNDER_RST;
}
#[repr(C)]
pub struct RegisterBlock {
    /// SRC Control Register
    pub SCR: RWRegister<u32>,

    /// SRC Reset Mode Register
    pub SRMR: RWRegister<u32>,

    /// SRC Boot Mode Register 1
    pub SBMR1: RORegister<u32>,

    /// SRC Boot Mode Register 2
    pub SBMR2: RORegister<u32>,

    /// SRC Reset Status Register
    pub SRSR: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR1: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR2: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR3: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR4: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR5: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR6: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR7: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR8: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR9: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR10: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR11: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR12: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR13: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR14: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR15: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR16: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR17: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR18: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR19: RWRegister<u32>,

    /// SRC General Purpose Register
    pub GPR20: RWRegister<u32>,

    _reserved1: [u32; 103],

    /// Slice Authentication Register
    pub AUTHEN_MEGA: RWRegister<u32>,

    /// Slice Control Register
    pub CTRL_MEGA: RWRegister<u32>,

    /// Slice Setpoint Config Register
    pub SETPOINT_MEGA: RWRegister<u32>,

    /// Slice Domain Config Register
    pub DOMAIN_MEGA: RWRegister<u32>,

    /// Slice Status Register
    pub STAT_MEGA: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// Slice Authentication Register
    pub AUTHEN_DISPLAY: RWRegister<u32>,

    /// Slice Control Register
    pub CTRL_DISPLAY: RWRegister<u32>,

    /// Slice Setpoint Config Register
    pub SETPOINT_DISPLAY: RWRegister<u32>,

    /// Slice Domain Config Register
    pub DOMAIN_DISPLAY: RWRegister<u32>,

    /// Slice Status Register
    pub STAT_DISPLAY: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// Slice Authentication Register
    pub AUTHEN_WAKEUP: RWRegister<u32>,

    /// Slice Control Register
    pub CTRL_WAKEUP: RWRegister<u32>,

    /// Slice Setpoint Config Register
    pub SETPOINT_WAKEUP: RWRegister<u32>,

    /// Slice Domain Config Register
    pub DOMAIN_WAKEUP: RWRegister<u32>,

    /// Slice Status Register
    pub STAT_WAKEUP: RWRegister<u32>,

    _reserved4: [u32; 11],

    /// Slice Authentication Register
    pub AUTHEN_M4CORE: RWRegister<u32>,

    /// Slice Control Register
    pub CTRL_M4CORE: RWRegister<u32>,

    /// Slice Setpoint Config Register
    pub SETPOINT_M4CORE: RWRegister<u32>,

    /// Slice Domain Config Register
    pub DOMAIN_M4CORE: RWRegister<u32>,

    /// Slice Status Register
    pub STAT_M4CORE: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// Slice Authentication Register
    pub AUTHEN_M7CORE: RWRegister<u32>,

    /// Slice Control Register
    pub CTRL_M7CORE: RWRegister<u32>,

    /// Slice Setpoint Config Register
    pub SETPOINT_M7CORE: RWRegister<u32>,

    /// Slice Domain Config Register
    pub DOMAIN_M7CORE: RWRegister<u32>,

    /// Slice Status Register
    pub STAT_M7CORE: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// Slice Authentication Register
    pub AUTHEN_M4DEBUG: RWRegister<u32>,

    /// Slice Control Register
    pub CTRL_M4DEBUG: RWRegister<u32>,

    /// Slice Setpoint Config Register
    pub SETPOINT_M4DEBUG: RWRegister<u32>,

    /// Slice Domain Config Register
    pub DOMAIN_M4DEBUG: RWRegister<u32>,

    /// Slice Status Register
    pub STAT_M4DEBUG: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// Slice Authentication Register
    pub AUTHEN_M7DEBUG: RWRegister<u32>,

    /// Slice Control Register
    pub CTRL_M7DEBUG: RWRegister<u32>,

    /// Slice Setpoint Config Register
    pub SETPOINT_M7DEBUG: RWRegister<u32>,

    /// Slice Domain Config Register
    pub DOMAIN_M7DEBUG: RWRegister<u32>,

    /// Slice Status Register
    pub STAT_M7DEBUG: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// Slice Authentication Register
    pub AUTHEN_USBPHY1: RWRegister<u32>,

    /// Slice Control Register
    pub CTRL_USBPHY1: RWRegister<u32>,

    /// Slice Setpoint Config Register
    pub SETPOINT_USBPHY1: RWRegister<u32>,

    /// Slice Domain Config Register
    pub DOMAIN_USBPHY1: RWRegister<u32>,

    /// Slice Status Register
    pub STAT_USBPHY1: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// Slice Authentication Register
    pub AUTHEN_USBPHY2: RWRegister<u32>,

    /// Slice Control Register
    pub CTRL_USBPHY2: RWRegister<u32>,

    /// Slice Setpoint Config Register
    pub SETPOINT_USBPHY2: RWRegister<u32>,

    /// Slice Domain Config Register
    pub DOMAIN_USBPHY2: RWRegister<u32>,

    /// Slice Status Register
    pub STAT_USBPHY2: RWRegister<u32>,
}
pub struct ResetValues {
    pub SCR: u32,
    pub SRMR: u32,
    pub SBMR1: u32,
    pub SBMR2: u32,
    pub SRSR: u32,
    pub GPR1: u32,
    pub GPR2: u32,
    pub GPR3: u32,
    pub GPR4: u32,
    pub GPR5: u32,
    pub GPR6: u32,
    pub GPR7: u32,
    pub GPR8: u32,
    pub GPR9: u32,
    pub GPR10: u32,
    pub GPR11: u32,
    pub GPR12: u32,
    pub GPR13: u32,
    pub GPR14: u32,
    pub GPR15: u32,
    pub GPR16: u32,
    pub GPR17: u32,
    pub GPR18: u32,
    pub GPR19: u32,
    pub GPR20: u32,
    pub AUTHEN_MEGA: u32,
    pub CTRL_MEGA: u32,
    pub SETPOINT_MEGA: u32,
    pub DOMAIN_MEGA: u32,
    pub STAT_MEGA: u32,
    pub AUTHEN_DISPLAY: u32,
    pub CTRL_DISPLAY: u32,
    pub SETPOINT_DISPLAY: u32,
    pub DOMAIN_DISPLAY: u32,
    pub STAT_DISPLAY: u32,
    pub AUTHEN_WAKEUP: u32,
    pub CTRL_WAKEUP: u32,
    pub SETPOINT_WAKEUP: u32,
    pub DOMAIN_WAKEUP: u32,
    pub STAT_WAKEUP: u32,
    pub AUTHEN_M4CORE: u32,
    pub CTRL_M4CORE: u32,
    pub SETPOINT_M4CORE: u32,
    pub DOMAIN_M4CORE: u32,
    pub STAT_M4CORE: u32,
    pub AUTHEN_M7CORE: u32,
    pub CTRL_M7CORE: u32,
    pub SETPOINT_M7CORE: u32,
    pub DOMAIN_M7CORE: u32,
    pub STAT_M7CORE: u32,
    pub AUTHEN_M4DEBUG: u32,
    pub CTRL_M4DEBUG: u32,
    pub SETPOINT_M4DEBUG: u32,
    pub DOMAIN_M4DEBUG: u32,
    pub STAT_M4DEBUG: u32,
    pub AUTHEN_M7DEBUG: u32,
    pub CTRL_M7DEBUG: u32,
    pub SETPOINT_M7DEBUG: u32,
    pub DOMAIN_M7DEBUG: u32,
    pub STAT_M7DEBUG: u32,
    pub AUTHEN_USBPHY1: u32,
    pub CTRL_USBPHY1: u32,
    pub SETPOINT_USBPHY1: u32,
    pub DOMAIN_USBPHY1: u32,
    pub STAT_USBPHY1: u32,
    pub AUTHEN_USBPHY2: u32,
    pub CTRL_USBPHY2: u32,
    pub SETPOINT_USBPHY2: u32,
    pub DOMAIN_USBPHY2: u32,
    pub STAT_USBPHY2: u32,
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
