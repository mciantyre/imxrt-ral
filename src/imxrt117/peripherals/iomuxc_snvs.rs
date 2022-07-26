#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC SNVS
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// SW_MUX_CTL_PAD_WAKEUP_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_WAKEUP_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO00 of instance: GPIO13
            pub const ALT5_gpio13_IO0: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: NMI_GLUE_NMI of instance: NMI_GLUE
            pub const ALT7_nmi_glue_NMI: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad WAKEUP_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_LP_PMIC_ON_REQ of instance: SNVS_LP
            pub const ALT0_snvs_lp_PMIC_ON_REQ: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO01 of instance: GPIO13
            pub const ALT5_gpio13_IO1: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad PMIC_ON_REQ_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: CCM_PMIC_VSTBY_REQ of instance: CCM
            pub const ALT0_ccm_PMIC_VSTBY_REQ: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO02 of instance: GPIO13
            pub const ALT5_gpio13_IO2: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad PMIC_STBY_REQ_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER0 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER0: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO03 of instance: GPIO13
            pub const ALT5_gpio13_IO3: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_00_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER1 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER1: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO04 of instance: GPIO13
            pub const ALT5_gpio13_IO4: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_01_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER2 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER2: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO05 of instance: GPIO13
            pub const ALT5_gpio13_IO5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_02_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER3 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER3: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO06 of instance: GPIO13
            pub const ALT5_gpio13_IO6: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_03_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER4 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER4: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO07 of instance: GPIO13
            pub const ALT5_gpio13_IO7: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_04_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER5 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER5: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO08 of instance: GPIO13
            pub const ALT5_gpio13_IO8: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_05_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER6 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER6: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO09 of instance: GPIO13
            pub const ALT5_gpio13_IO9: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_06_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER7 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER7: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO10 of instance: GPIO13
            pub const ALT5_gpio13_IO10: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_07_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER8 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER8: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO11 of instance: GPIO13
            pub const ALT5_gpio13_IO11: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_08_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_TAMPER9 of instance: SNVS_LP
            pub const ALT0_SNVS_TAMPER9: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO13_IO12 of instance: GPIO13
            pub const ALT5_gpio13_IO12: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
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

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SNVS_09_DIG
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_PAD_CTL_PAD_TEST_MODE_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_TEST_MODE_DIG {

    /// Slew Rate Field
    pub mod SRE {
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

            /// 0b0: Slow Slew Rate
            pub const SRE_0_Slow_Slew_Rate: u32 = 0b0;

            /// 0b1: Fast Slew Rate
            pub const SRE_1_Fast_Slew_Rate: u32 = 0b1;
        }
    }

    /// Drive Strength Field
    pub mod DSE {
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

            /// 0b0: normal driver
            pub const DSE_0_normal_driver: u32 = 0b0;

            /// 0b1: high driver
            pub const DSE_1_high_driver: u32 = 0b1;
        }
    }

    /// Pull / Keep Select Field
    pub mod PUE {
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

            /// 0b0: Pull Disable
            pub const PUE_0_Disable: u32 = 0b0;

            /// 0b1: Pull Enable
            pub const PUE_1_Pull: u32 = 0b1;
        }
    }

    /// Pull Up / Down Config. Field
    pub mod PUS {
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

            /// 0b0: Weak pull down
            pub const PUS_0_Weak_pull_down: u32 = 0b0;

            /// 0b1: Weak pull up
            pub const PUS_1_Weak_pull_up: u32 = 0b1;
        }
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// SW_PAD_CTL_PAD_POR_B_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_POR_B_DIG {
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::SRE;
}

/// SW_PAD_CTL_PAD_ONOFF_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_ONOFF_DIG {
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE_DIG::SRE;
}

/// SW_PAD_CTL_PAD_WAKEUP_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_WAKEUP_DIG {

    /// Slew Rate Field
    pub mod SRE {
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

            /// 0b0: Slow Slew Rate
            pub const SRE_0_Slow_Slew_Rate: u32 = 0b0;

            /// 0b1: Fast Slew Rate
            pub const SRE_1_Fast_Slew_Rate: u32 = 0b1;
        }
    }

    /// Drive Strength Field
    pub mod DSE {
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

            /// 0b0: normal driver
            pub const DSE_0_normal_driver: u32 = 0b0;

            /// 0b1: high driver
            pub const DSE_1_high_driver: u32 = 0b1;
        }
    }

    /// Pull / Keep Select Field
    pub mod PUE {
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

            /// 0b0: Pull Disable
            pub const PUE_0_Disable: u32 = 0b0;

            /// 0b1: Pull Enable
            pub const PUE_1_Pull: u32 = 0b1;
        }
    }

    /// Pull Up / Down Config. Field
    pub mod PUS {
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

            /// 0b0: Weak pull down
            pub const PUS_0_Weak_pull_down: u32 = 0b0;

            /// 0b1: Weak pull up
            pub const PUS_1_Weak_pull_up: u32 = 0b1;
        }
    }

    /// Open Drain SNVS Field
    pub mod ODE_SNVS {
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

            /// 0b0: Disabled
            pub const ODE_SNVS_0_Disabled: u32 = 0b0;

            /// 0b1: Enabled
            pub const ODE_SNVS_1_Enabled: u32 = 0b1;
        }
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG {
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DSE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::ODE_SNVS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUE;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::PUS;
    pub use super::SW_PAD_CTL_PAD_WAKEUP_DIG::SRE;
}
#[repr(C)]
pub struct RegisterBlock {
    /// SW_MUX_CTL_PAD_WAKEUP_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_WAKEUP_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_TEST_MODE_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_TEST_MODE_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_POR_B_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_POR_B_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_ONOFF_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_ONOFF_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_WAKEUP_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_WAKEUP_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG: RWRegister<u32>,
}
pub struct ResetValues {
    pub SW_MUX_CTL_PAD_WAKEUP_DIG: u32,
    pub SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG: u32,
    pub SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG: u32,
    pub SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG: u32,
    pub SW_PAD_CTL_PAD_TEST_MODE_DIG: u32,
    pub SW_PAD_CTL_PAD_POR_B_DIG: u32,
    pub SW_PAD_CTL_PAD_ONOFF_DIG: u32,
    pub SW_PAD_CTL_PAD_WAKEUP_DIG: u32,
    pub SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG: u32,
    pub SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG: u32,
    pub SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG: u32,
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
