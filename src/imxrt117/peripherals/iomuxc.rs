#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA00 of instance: SEMC
            pub const ALT0_semc_DATA0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWM0_A of instance: FLEXPWM4
            pub const ALT1_flexpwm4_PWMA0: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO00 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO0: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D00 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO0: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO00 of instance: GPIO7
            pub const ALT10_gpio7_IO0: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA01 of instance: SEMC
            pub const ALT0_semc_DATA1: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWM0_B of instance: FLEXPWM4
            pub const ALT1_flexpwm4_PWMB0: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO01 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO1: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D01 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO1: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO01 of instance: GPIO7
            pub const ALT10_gpio7_IO1: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA02 of instance: SEMC
            pub const ALT0_semc_DATA2: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWM1_A of instance: FLEXPWM4
            pub const ALT1_flexpwm4_PWMA1: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO02 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO2: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D02 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO2: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO02 of instance: GPIO7
            pub const ALT10_gpio7_IO2: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA03 of instance: SEMC
            pub const ALT0_semc_DATA3: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWM1_B of instance: FLEXPWM4
            pub const ALT1_flexpwm4_PWMB1: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO03 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO3: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D03 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO3: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO03 of instance: GPIO7
            pub const ALT10_gpio7_IO3: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA04 of instance: SEMC
            pub const ALT0_semc_DATA4: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWM2_A of instance: FLEXPWM4
            pub const ALT1_flexpwm4_PWMA2: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO04 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO4: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D04 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO4: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO04 of instance: GPIO7
            pub const ALT10_gpio7_IO4: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA05 of instance: SEMC
            pub const ALT0_semc_DATA5: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWM2_B of instance: FLEXPWM4
            pub const ALT1_flexpwm4_PWMB2: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO05 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D05 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO5: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO05 of instance: GPIO7
            pub const ALT10_gpio7_IO5: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA06 of instance: SEMC
            pub const ALT0_semc_DATA6: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWM0_A of instance: FLEXPWM2
            pub const ALT1_flexpwm2_PWMA0: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO06 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO6: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D06 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO6: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO06 of instance: GPIO7
            pub const ALT10_gpio7_IO6: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA07 of instance: SEMC
            pub const ALT0_semc_DATA7: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWM0_B of instance: FLEXPWM2
            pub const ALT1_flexpwm2_PWMB0: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO07 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO7: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D07 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO7: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO07 of instance: GPIO7
            pub const ALT10_gpio7_IO7: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DM00 of instance: SEMC
            pub const ALT0_semc_DM0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWM1_A of instance: FLEXPWM2
            pub const ALT1_flexpwm2_PWMA1: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO08 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO8: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D08 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO8: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO08 of instance: GPIO7
            pub const ALT10_gpio7_IO8: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR00 of instance: SEMC
            pub const ALT0_semc_ADDR0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWM1_B of instance: FLEXPWM2
            pub const ALT1_flexpwm2_PWMB1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT5_CAPTURE1 of instance: GPT5
            pub const ALT2_gpt5_CAPTURE1: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO09 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO9: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D09 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO9: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO09 of instance: GPIO7
            pub const ALT10_gpio7_IO9: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR01 of instance: SEMC
            pub const ALT0_semc_ADDR1: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWM2_A of instance: FLEXPWM2
            pub const ALT1_flexpwm2_PWMA2: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT5_CAPTURE2 of instance: GPT5
            pub const ALT2_gpt5_CAPTURE2: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO10 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO10: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D10 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO10: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO10 of instance: GPIO7
            pub const ALT10_gpio7_IO10: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR02 of instance: SEMC
            pub const ALT0_semc_ADDR2: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWM2_B of instance: FLEXPWM2
            pub const ALT1_flexpwm2_PWMB2: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT5_COMPARE1 of instance: GPT5
            pub const ALT2_gpt5_COMPARE1: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO11 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO11: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D11 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO11: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO11 of instance: GPIO7
            pub const ALT10_gpio7_IO11: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR03 of instance: SEMC
            pub const ALT0_semc_ADDR3: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT04 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT4: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT5_COMPARE2 of instance: GPT5
            pub const ALT2_gpt5_COMPARE2: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO12 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO12: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D12 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO12: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO12 of instance: GPIO7
            pub const ALT10_gpio7_IO12: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR04 of instance: SEMC
            pub const ALT0_semc_ADDR4: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT05 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT5: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT5_COMPARE3 of instance: GPT5
            pub const ALT2_gpt5_COMPARE3: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO13 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO13: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D13 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO13: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO13 of instance: GPIO7
            pub const ALT10_gpio7_IO13: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_14 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR05 of instance: SEMC
            pub const ALT0_semc_ADDR5: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT06 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT6: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT5_CLK of instance: GPT5
            pub const ALT2_gpt5_CLK: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO14 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO14: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D14 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO14: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO14 of instance: GPIO7
            pub const ALT10_gpio7_IO14: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_15 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR06 of instance: SEMC
            pub const ALT0_semc_ADDR6: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT07 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT7: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO15 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO15: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D15 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO15: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO15 of instance: GPIO7
            pub const ALT10_gpio7_IO15: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_15
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_16 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_16 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR07 of instance: SEMC
            pub const ALT0_semc_ADDR7: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT08 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT8: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO16 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO16: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D16 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO16: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO16 of instance: GPIO7
            pub const ALT10_gpio7_IO16: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_16
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_17 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_17 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR08 of instance: SEMC
            pub const ALT0_semc_ADDR8: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWM3_A of instance: FLEXPWM4
            pub const ALT1_flexpwm4_PWMA3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: TMR1_TIMER0 of instance: TMR1
            pub const ALT2_qtimer1_TIMER0: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO17 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO17: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D17 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO17: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO17 of instance: GPIO7
            pub const ALT10_gpio7_IO17: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_17
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_18 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_18 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR09 of instance: SEMC
            pub const ALT0_semc_ADDR9: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWM3_B of instance: FLEXPWM4
            pub const ALT1_flexpwm4_PWMB3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: TMR2_TIMER0 of instance: TMR2
            pub const ALT2_qtimer2_TIMER0: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO18 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO18: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D18 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO18: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO18 of instance: GPIO7
            pub const ALT10_gpio7_IO18: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_18
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_19 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_19 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR11 of instance: SEMC
            pub const ALT0_semc_ADDR11: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWM3_A of instance: FLEXPWM2
            pub const ALT1_flexpwm2_PWMA3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: TMR3_TIMER0 of instance: TMR3
            pub const ALT2_qtimer3_TIMER0: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO19 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO19: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D19 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO19: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO19 of instance: GPIO7
            pub const ALT10_gpio7_IO19: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_19
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_20 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_20 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR12 of instance: SEMC
            pub const ALT0_semc_ADDR12: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWM3_B of instance: FLEXPWM2
            pub const ALT1_flexpwm2_PWMB3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: TMR4_TIMER0 of instance: TMR4
            pub const ALT2_qtimer4_TIMER0: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO20 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO20: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D20 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO20: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO20 of instance: GPIO7
            pub const ALT10_gpio7_IO20: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_20
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_21 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_21 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_BA0 of instance: SEMC
            pub const ALT0_semc_BA0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWM3_A of instance: FLEXPWM3
            pub const ALT1_flexpwm3_PWMA3: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO21 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO21: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D21 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO21: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO21 of instance: GPIO7
            pub const ALT10_gpio7_IO21: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_21
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_22 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_22 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_BA1 of instance: SEMC
            pub const ALT0_semc_BA1: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWM3_B of instance: FLEXPWM3
            pub const ALT1_flexpwm3_PWMB3: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO22 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO22: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D22 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO22: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO22 of instance: GPIO7
            pub const ALT10_gpio7_IO22: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_22
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_23 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_23 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR10 of instance: SEMC
            pub const ALT0_semc_ADDR10: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM0_A of instance: FLEXPWM1
            pub const ALT1_flexpwm1_PWMA0: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO23 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO23: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D23 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO23: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO23 of instance: GPIO7
            pub const ALT10_gpio7_IO23: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_23
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_24 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_24 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CAS of instance: SEMC
            pub const ALT0_semc_CAS: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM0_B of instance: FLEXPWM1
            pub const ALT1_flexpwm1_PWMB0: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO24 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO24: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D24 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO24: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO24 of instance: GPIO7
            pub const ALT10_gpio7_IO24: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_24
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_25 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_25 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_RAS of instance: SEMC
            pub const ALT0_semc_RAS: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM1_A of instance: FLEXPWM1
            pub const ALT1_flexpwm1_PWMA1: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO25 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO25: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D25 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO25: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO25 of instance: GPIO7
            pub const ALT10_gpio7_IO25: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_25
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_26 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_26 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CLK of instance: SEMC
            pub const ALT0_semc_CLK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM1_B of instance: FLEXPWM1
            pub const ALT1_flexpwm1_PWMB1: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO26 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO26: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D26 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO26: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO26 of instance: GPIO7
            pub const ALT10_gpio7_IO26: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_26
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_27 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_27 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CKE of instance: SEMC
            pub const ALT0_semc_CKE: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM2_A of instance: FLEXPWM1
            pub const ALT1_flexpwm1_PWMA2: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO27 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO27: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D27 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO27: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO27 of instance: GPIO7
            pub const ALT10_gpio7_IO27: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_27
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_28 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_28 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_WE of instance: SEMC
            pub const ALT0_semc_WE: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM2_B of instance: FLEXPWM1
            pub const ALT1_flexpwm1_PWMB2: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO28 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO28: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D28 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO28: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO28 of instance: GPIO7
            pub const ALT10_gpio7_IO28: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_28
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_29 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_29 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CS0 of instance: SEMC
            pub const ALT0_semc_CS0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWM0_A of instance: FLEXPWM3
            pub const ALT1_flexpwm3_PWMA0: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO29 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO29: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D29 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO29: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO29 of instance: GPIO7
            pub const ALT10_gpio7_IO29: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_29
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_30 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_30 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA08 of instance: SEMC
            pub const ALT0_semc_DATA8: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWM0_B of instance: FLEXPWM3
            pub const ALT1_flexpwm3_PWMB0: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO30 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO30: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D30 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO30: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO30 of instance: GPIO7
            pub const ALT10_gpio7_IO30: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_30
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_31 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_31 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA09 of instance: SEMC
            pub const ALT0_semc_DATA9: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWM1_A of instance: FLEXPWM3
            pub const ALT1_flexpwm3_PWMA1: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX1_IO31 of instance: GPIO_MUX1
            pub const ALT5_gpio_mux1_IO31: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO1_D31 of instance: FLEXIO1
            pub const ALT8_flexio1_FLEXIO31: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO7_IO31 of instance: GPIO7
            pub const ALT10_gpio7_IO31: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_31
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_32 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_32 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA10 of instance: SEMC
            pub const ALT0_semc_DATA10: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWM1_B of instance: FLEXPWM3
            pub const ALT1_flexpwm3_PWMB1: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO00 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO0: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO00 of instance: GPIO8
            pub const ALT10_gpio8_IO0: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_32
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_33 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_33 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA11 of instance: SEMC
            pub const ALT0_semc_DATA11: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWM2_A of instance: FLEXPWM3
            pub const ALT1_flexpwm3_PWMA2: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO01 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO1: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO01 of instance: GPIO8
            pub const ALT10_gpio8_IO1: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_33
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_34 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_34 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA12 of instance: SEMC
            pub const ALT0_semc_DATA12: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWM2_B of instance: FLEXPWM3
            pub const ALT1_flexpwm3_PWMB2: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO02 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO2: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO02 of instance: GPIO8
            pub const ALT10_gpio8_IO2: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_34
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_35 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_35 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA13 of instance: SEMC
            pub const ALT0_semc_DATA13: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT09 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT9: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO03 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO3: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO03 of instance: GPIO8
            pub const ALT10_gpio8_IO3: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_35
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_36 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_36 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA14 of instance: SEMC
            pub const ALT0_semc_DATA14: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT10 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT10: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO04 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO4: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO04 of instance: GPIO8
            pub const ALT10_gpio8_IO4: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_36
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_37 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_37 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA15 of instance: SEMC
            pub const ALT0_semc_DATA15: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT11 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT11: u32 = 0b0001;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO05 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO5: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO05 of instance: GPIO8
            pub const ALT10_gpio8_IO5: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_37
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_38 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_38 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DM01 of instance: SEMC
            pub const ALT0_semc_DM1: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM3_A of instance: FLEXPWM1
            pub const ALT1_flexpwm1_PWMA3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: TMR1_TIMER1 of instance: TMR1
            pub const ALT2_qtimer1_TIMER1: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO06 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO6: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO06 of instance: GPIO8
            pub const ALT10_gpio8_IO6: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_38
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_39 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_39 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DQS of instance: SEMC
            pub const ALT0_semc_DQS: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM3_B of instance: FLEXPWM1
            pub const ALT1_flexpwm1_PWMB3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: TMR2_TIMER1 of instance: TMR2
            pub const ALT2_qtimer2_TIMER1: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO07 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO7: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO07 of instance: GPIO8
            pub const ALT10_gpio8_IO7: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_39
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_40 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_40 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_RDY of instance: SEMC
            pub const ALT0_semc_RDY: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT12 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT12: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: MQS_RIGHT of instance: MQS
            pub const ALT2_mqs_RIGHT: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART6_TXD of instance: LPUART6
            pub const ALT3_lpuart6_TX: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO08 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO8: u32 = 0b0101;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_MDC of instance: ENET_1G
            pub const ALT7_enet_1g_MDC: u32 = 0b0111;

            /// 0b1001: Select mux mode: ALT9 mux port: CCM_CLKO1 of instance: CCM
            pub const ALT9_CCM_CLKO1: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO08 of instance: GPIO8
            pub const ALT10_gpio8_IO8: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_40
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B1_41 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B1_41 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CSX00 of instance: SEMC
            pub const ALT0_semc_CSX0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT13 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT13: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: MQS_LEFT of instance: MQS
            pub const ALT2_mqs_LEFT: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART6_RXD of instance: LPUART6
            pub const ALT3_lpuart6_RX: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_DATA07 of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_DATA7: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO09 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO9: u32 = 0b0101;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_MDIO of instance: ENET_1G
            pub const ALT7_enet_1g_MDIO: u32 = 0b0111;

            /// 0b1001: Select mux mode: ALT9 mux port: CCM_CLKO2 of instance: CCM
            pub const ALT9_CCM_CLKO2: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO09 of instance: GPIO8
            pub const ALT10_gpio8_IO9: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B1_41
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA16 of instance: SEMC
            pub const ALT0_semc_DATA16: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: CCM_ENET_REF_CLK_25M of instance: CCM
            pub const ALT1_CCM_ENET_REF_CLK_25M: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: TMR3_TIMER1 of instance: TMR3
            pub const ALT2_qtimer3_TIMER1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART6_CTS_B of instance: LPUART6
            pub const ALT3_lpuart6_CTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_DATA06 of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_DATA6: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO10 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO10: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT20 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT20: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_QOS_1588_EVENT1_OUT of instance: ENET_QOS
            pub const ALT7_enet_qos_1588_EVENT1_OUT: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI1_SCK of instance: LPSPI1
            pub const ALT8_lpspi1_SCK: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPI2C2_SCL of instance: LPI2C2
            pub const ALT9_lpi2c2_SCL: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO10 of instance: GPIO8
            pub const ALT10_gpio8_IO10: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM0_A of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMA0: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA17 of instance: SEMC
            pub const ALT0_semc_DATA17: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USDHC2_CD_B of instance: USDHC2
            pub const ALT1_usdhc2_CD_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: TMR4_TIMER1 of instance: TMR4
            pub const ALT2_qtimer4_TIMER1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART6_RTS_B of instance: LPUART6
            pub const ALT3_lpuart6_RTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_DATA05 of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_DATA5: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO11 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO11: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT21 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT21: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_QOS_1588_EVENT1_IN of instance: ENET_QOS
            pub const ALT7_enet_qos_1588_EVENT1_IN: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI1_PCS0 of instance: LPSPI1
            pub const ALT8_lpspi1_PCS0: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPI2C2_SDA of instance: LPI2C2
            pub const ALT9_lpi2c2_SDA: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO11 of instance: GPIO8
            pub const ALT10_gpio8_IO11: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM0_B of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMB0: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA18 of instance: SEMC
            pub const ALT0_semc_DATA18: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USDHC2_WP of instance: USDHC2
            pub const ALT1_usdhc2_WP: u32 = 0b0001;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_CSI_DATA23 of instance: VIDEO_MUX
            pub const ALT3_video_mux_CSI_DATA23: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_DATA04 of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_DATA4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO12 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO12: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT22 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT22: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_QOS_1588_EVENT1_AUX_IN of instance: ENET_QOS
            pub const ALT7_enet_qos_1588_EVENT1_AUX_IN: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI1_SOUT of instance: LPSPI1
            pub const ALT8_lpspi1_SDO: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO12 of instance: GPIO8
            pub const ALT10_gpio8_IO12: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM1_A of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMA1: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA19 of instance: SEMC
            pub const ALT0_semc_DATA19: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USDHC2_VSELECT of instance: USDHC2
            pub const ALT1_usdhc2_VSELECT: u32 = 0b0001;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_CSI_DATA22 of instance: VIDEO_MUX
            pub const ALT3_video_mux_CSI_DATA22: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_DATA03 of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_DATA3: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO13 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO13: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT23 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT23: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_TX_DATA03 of instance: ENET_1G
            pub const ALT7_ENET_1G_TX_DATA3: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI1_SIN of instance: LPSPI1
            pub const ALT8_lpspi1_SDI: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO13 of instance: GPIO8
            pub const ALT10_gpio8_IO13: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM1_B of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMB1: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA20 of instance: SEMC
            pub const ALT0_semc_DATA20: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USDHC2_RESET_B of instance: USDHC2
            pub const ALT1_usdhc2_RESET_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI2_MCLK of instance: SAI2
            pub const ALT2_sai2_MCLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_CSI_DATA21 of instance: VIDEO_MUX
            pub const ALT3_video_mux_CSI_DATA21: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_DATA02 of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_DATA2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO14 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO14: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT24 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT24: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_TX_DATA02 of instance: ENET_1G
            pub const ALT7_ENET_1G_TX_DATA2: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI3_SCK of instance: LPSPI3
            pub const ALT8_lpspi3_SCK: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO14 of instance: GPIO8
            pub const ALT10_gpio8_IO14: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM2_A of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMA2: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA21 of instance: SEMC
            pub const ALT0_semc_DATA21: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: GPT3_CLK of instance: GPT3
            pub const ALT1_gpt3_CLK: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI2_RX_SYNC of instance: SAI2
            pub const ALT2_sai2_RX_SYNC: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_CSI_DATA20 of instance: VIDEO_MUX
            pub const ALT3_video_mux_CSI_DATA20: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_DATA01 of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_DATA1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO15 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO15: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT25 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT25: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_RX_CLK of instance: ENET_1G
            pub const ALT7_enet_1g_RX_CLK: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI3_PCS0 of instance: LPSPI3
            pub const ALT8_lpspi3_PCS0: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: PIT1_TRIGGER0 of instance: PIT1
            pub const ALT9_pit1_TRIGGER0: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO15 of instance: GPIO8
            pub const ALT10_gpio8_IO15: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM2_B of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMB2: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA22 of instance: SEMC
            pub const ALT0_semc_DATA22: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: GPT3_CAPTURE1 of instance: GPT3
            pub const ALT1_gpt3_CAPTURE1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI2_RX_BCLK of instance: SAI2
            pub const ALT2_sai2_RX_BCLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_CSI_DATA19 of instance: VIDEO_MUX
            pub const ALT3_video_mux_CSI_DATA19: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_DATA00 of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_DATA0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO16 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO16: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT26 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT26: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_TX_ER of instance: ENET_1G
            pub const ALT7_enet_1g_TX_ER: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI3_SOUT of instance: LPSPI3
            pub const ALT8_lpspi3_SDO: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: PIT1_TRIGGER1 of instance: PIT1
            pub const ALT9_pit1_TRIGGER1: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO16 of instance: GPIO8
            pub const ALT10_gpio8_IO16: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM3_A of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMA3: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA23 of instance: SEMC
            pub const ALT0_semc_DATA23: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: GPT3_CAPTURE2 of instance: GPT3
            pub const ALT1_gpt3_CAPTURE2: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI2_RX_DATA of instance: SAI2
            pub const ALT2_sai2_RX_DATA: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_CSI_DATA18 of instance: VIDEO_MUX
            pub const ALT3_video_mux_CSI_DATA18: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_DQS of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_DQS: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO17 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO17: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT27 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT27: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_RX_DATA03 of instance: ENET_1G
            pub const ALT7_ENET_1G_RX_DATA3: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI3_SIN of instance: LPSPI3
            pub const ALT8_lpspi3_SDI: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: PIT1_TRIGGER2 of instance: PIT1
            pub const ALT9_pit1_TRIGGER2: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO17 of instance: GPIO8
            pub const ALT10_gpio8_IO17: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM3_B of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMB3: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DM02 of instance: SEMC
            pub const ALT0_semc_DM2: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: GPT3_COMPARE1 of instance: GPT3
            pub const ALT1_gpt3_COMPARE1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI2_TX_DATA of instance: SAI2
            pub const ALT2_sai2_TX_DATA: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_CSI_DATA17 of instance: VIDEO_MUX
            pub const ALT3_video_mux_CSI_DATA17: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_SS0_B of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_SS0_B: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO18 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO18: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT28 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT28: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_RX_DATA02 of instance: ENET_1G
            pub const ALT7_ENET_1G_RX_DATA2: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI3_PCS1 of instance: LPSPI3
            pub const ALT8_lpspi3_PCS1: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: PIT1_TRIGGER3 of instance: PIT1
            pub const ALT9_pit1_TRIGGER3: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO18 of instance: GPIO8
            pub const ALT10_gpio8_IO18: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA24 of instance: SEMC
            pub const ALT0_semc_DATA24: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: GPT3_COMPARE2 of instance: GPT3
            pub const ALT1_gpt3_COMPARE2: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI2_TX_BCLK of instance: SAI2
            pub const ALT2_sai2_TX_BCLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_CSI_DATA16 of instance: VIDEO_MUX
            pub const ALT3_video_mux_CSI_DATA16: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_B_SCLK of instance: FLEXSPI2
            pub const ALT4_flexspi2_B_SCLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO19 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO19: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT29 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT29: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_CRS of instance: ENET_1G
            pub const ALT7_enet_1g_CRS: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI3_PCS2 of instance: LPSPI3
            pub const ALT8_lpspi3_PCS2: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR1_TIMER0 of instance: TMR1
            pub const ALT9_qtimer1_TIMER0: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO19 of instance: GPIO8
            pub const ALT10_gpio8_IO19: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA25 of instance: SEMC
            pub const ALT0_semc_DATA25: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: GPT3_COMPARE3 of instance: GPT3
            pub const ALT1_gpt3_COMPARE3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI2_TX_SYNC of instance: SAI2
            pub const ALT2_sai2_TX_SYNC: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_CSI_FIELD of instance: VIDEO_MUX
            pub const ALT3_video_mux_CSI_FIELD: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_SCLK of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_SCLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO20 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO20: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT30 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT30: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1G_COL of instance: ENET_1G
            pub const ALT7_enet_1g_COL: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI3_PCS3 of instance: LPSPI3
            pub const ALT8_lpspi3_PCS3: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR1_TIMER1 of instance: TMR1
            pub const ALT9_qtimer1_TIMER1: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO20 of instance: GPIO8
            pub const ALT10_gpio8_IO20: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA26 of instance: SEMC
            pub const ALT0_semc_DATA26: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: SPDIF_IN of instance: SPDIF
            pub const ALT1_spdif_IN: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_DATA00 of instance: ENET_1G
            pub const ALT2_ENET_1G_TX_DATA0: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_RX_SYNC of instance: SAI3
            pub const ALT3_sai3_RX_SYNC: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_SS0_B of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_SS0_B: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO21 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO21: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT31 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT31: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: EMVSIM1_IO of instance: EMVSIM1
            pub const ALT8_EMVSIM1_TRXD: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR1_TIMER2 of instance: TMR1
            pub const ALT9_qtimer1_TIMER2: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO21 of instance: GPIO8
            pub const ALT10_gpio8_IO21: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA27 of instance: SEMC
            pub const ALT0_semc_DATA27: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: SPDIF_OUT of instance: SPDIF
            pub const ALT1_spdif_OUT: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_DATA01 of instance: ENET_1G
            pub const ALT2_ENET_1G_TX_DATA1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_RX_BCLK of instance: SAI3
            pub const ALT3_sai3_RX_BCLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_DQS of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_DQS: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO22 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO22: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT32 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT32: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: EMVSIM1_CLK of instance: EMVSIM1
            pub const ALT8_EMVSIM1_CLK: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR1_TIMER3 of instance: TMR1
            pub const ALT9_qtimer1_TIMER3: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO22 of instance: GPIO8
            pub const ALT10_gpio8_IO22: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA28 of instance: SEMC
            pub const ALT0_semc_DATA28: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_EN of instance: ENET_1G
            pub const ALT2_enet_1g_TX_EN: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_RX_DATA of instance: SAI3
            pub const ALT3_sai3_RX_DATA: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_DATA00 of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_DATA0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO23 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO23: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT33 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT33: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: EMVSIM1_RST of instance: EMVSIM1
            pub const ALT8_EMVSIM1_RST_B: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR2_TIMER0 of instance: TMR2
            pub const ALT9_qtimer2_TIMER0: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO23 of instance: GPIO8
            pub const ALT10_gpio8_IO23: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_14 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA29 of instance: SEMC
            pub const ALT0_semc_DATA29: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_CLK_IO of instance: ENET_1G
            pub const ALT2_enet_1g_TX_CLK_IO: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_TX_DATA of instance: SAI3
            pub const ALT3_sai3_TX_DATA: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_DATA01 of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_DATA1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO24 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO24: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT34 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT34: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: SFA_ipp_do_atx_clk_under_test of instance: sfa
            pub const ALT7_sfa_ipp_do_atx_clk_under_test: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: EMVSIM1_SVEN of instance: EMVSIM1
            pub const ALT8_EMVSIM1_SVEN: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR2_TIMER1 of instance: TMR2
            pub const ALT9_qtimer2_TIMER1: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO24 of instance: GPIO8
            pub const ALT10_gpio8_IO24: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_15 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA30 of instance: SEMC
            pub const ALT0_semc_DATA30: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_DATA00 of instance: ENET_1G
            pub const ALT2_ENET_1G_RX_DATA0: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_TX_BCLK of instance: SAI3
            pub const ALT3_sai3_TX_BCLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_DATA02 of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_DATA2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO25 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO25: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_INOUT35 of instance: XBAR1
            pub const ALT6_XBAR1_INOUT35: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: EMVSIM1_PD of instance: EMVSIM1
            pub const ALT8_EMVSIM1_PD: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR2_TIMER2 of instance: TMR2
            pub const ALT9_qtimer2_TIMER2: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO25 of instance: GPIO8
            pub const ALT10_gpio8_IO25: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_15
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_16 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_16 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA31 of instance: SEMC
            pub const ALT0_semc_DATA31: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT14 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT14: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_DATA01 of instance: ENET_1G
            pub const ALT2_ENET_1G_RX_DATA1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_TX_SYNC of instance: SAI3
            pub const ALT3_sai3_TX_SYNC: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_DATA03 of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_DATA3: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO26 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO26: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: EMVSIM1_POWER_FAIL of instance: EMVSIM1
            pub const ALT8_EMVSIM1_POWER_FAIL: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR2_TIMER3 of instance: TMR2
            pub const ALT9_qtimer2_TIMER3: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO26 of instance: GPIO8
            pub const ALT10_gpio8_IO26: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_16
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_17 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_17 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DM03 of instance: SEMC
            pub const ALT0_semc_DM3: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT15 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT15: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_EN of instance: ENET_1G
            pub const ALT2_enet_1g_RX_EN: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_MCLK of instance: SAI3
            pub const ALT3_sai3_MCLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_DATA04 of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_DATA4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO27 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO27: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: WDOG1_ANY of instance: WDOG1
            pub const ALT8_WDOG1_ANY: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR3_TIMER0 of instance: TMR3
            pub const ALT9_qtimer3_TIMER0: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO27 of instance: GPIO8
            pub const ALT10_gpio8_IO27: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_17
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_18 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_18 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DQS4 of instance: SEMC
            pub const ALT0_semc_DQS4: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT16 of instance: XBAR1
            pub const ALT1_XBAR1_INOUT16: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_ER of instance: ENET_1G
            pub const ALT2_enet_1g_RX_ER: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: EWM_OUT_B of instance: EWM
            pub const ALT3_EWM_OUT_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_DATA05 of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_DATA5: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO28 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO28: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXSPI1_A_DQS of instance: FLEXSPI1
            pub const ALT6_flexspi1_A_DQS: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: WDOG1_B of instance: WDOG1
            pub const ALT8_WDOG1_B: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR3_TIMER1 of instance: TMR3
            pub const ALT9_qtimer3_TIMER1: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO28 of instance: GPIO8
            pub const ALT10_gpio8_IO28: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_18
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_19 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_19 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CLKX00 of instance: SEMC
            pub const ALT0_semc_CLKX0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_MDC of instance: ENET
            pub const ALT1_enet_MDC: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_MDC of instance: ENET_1G
            pub const ALT2_enet_1g_MDC: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_1G_REF_CLK of instance: ENET_1G
            pub const ALT3_enet_1g_REF_CLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_DATA06 of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_DATA6: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO29 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO29: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_MDC of instance: ENET_QOS
            pub const ALT8_enet_qos_MDC: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR3_TIMER2 of instance: TMR3
            pub const ALT9_qtimer3_TIMER2: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO29 of instance: GPIO8
            pub const ALT10_gpio8_IO29: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_19
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_B2_20 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_B2_20 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CLKX01 of instance: SEMC
            pub const ALT0_semc_CLKX1: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_MDIO of instance: ENET
            pub const ALT1_enet_MDIO: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_MDIO of instance: ENET_1G
            pub const ALT2_enet_1g_MDIO: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_QOS_REF_CLK of instance: ENET_QOS
            pub const ALT3_CCM_enet_qos_clock_generate_REF_CLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPI2_A_DATA07 of instance: FLEXSPI2
            pub const ALT4_flexspi2_A_DATA7: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO30 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO30: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_MDIO of instance: ENET_QOS
            pub const ALT8_enet_qos_MDIO: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR3_TIMER3 of instance: TMR3
            pub const ALT9_qtimer3_TIMER3: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO30 of instance: GPIO8
            pub const ALT10_gpio8_IO30: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_EMC_B2_20
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: EMVSIM1_IO of instance: EMVSIM1
            pub const ALT0_EMVSIM1_TRXD: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXCAN2_TX of instance: FLEXCAN2
            pub const ALT1_can2_TX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_1588_EVENT1_IN of instance: ENET_1G
            pub const ALT2_enet_1g_1588_EVENT1_IN: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT2_CAPTURE1 of instance: GPT2
            pub const ALT3_gpt2_CAPTURE1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM1_PWM0_A of instance: FLEXPWM1
            pub const ALT4_flexpwm1_PWMA0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX2_IO31 of instance: GPIO_MUX2
            pub const ALT5_gpio_mux2_IO31: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPUART7_TXD of instance: LPUART7
            pub const ALT6_lpuart7_TX: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D00 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO0: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXSPI2_B_SS1_B of instance: FLEXSPI2
            pub const ALT9_flexspi2_B_SS1_B: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO8_IO31 of instance: GPIO8
            pub const ALT10_gpio8_IO31: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: EMVSIM1_CLK of instance: EMVSIM1
            pub const ALT0_EMVSIM1_CLK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXCAN2_RX of instance: FLEXCAN2
            pub const ALT1_can2_RX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_1588_EVENT1_OUT of instance: ENET_1G
            pub const ALT2_enet_1g_1588_EVENT1_OUT: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT2_CAPTURE2 of instance: GPT2
            pub const ALT3_gpt2_CAPTURE2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM1_PWM0_B of instance: FLEXPWM1
            pub const ALT4_flexpwm1_PWMB0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO00 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO0: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPUART7_RXD of instance: LPUART7
            pub const ALT6_lpuart7_RX: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D01 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO1: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXSPI2_A_SS1_B of instance: FLEXSPI2
            pub const ALT9_flexspi2_A_SS1_B: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO00 of instance: GPIO9
            pub const ALT10_gpio9_IO0: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: EMVSIM1_RST of instance: EMVSIM1
            pub const ALT0_EMVSIM1_RST_B: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART7_CTS_B of instance: LPUART7
            pub const ALT1_lpuart7_CTS_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_1588_EVENT2_IN of instance: ENET_1G
            pub const ALT2_enet_1g_1588_EVENT2_IN: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT2_COMPARE1 of instance: GPT2
            pub const ALT3_gpt2_COMPARE1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM1_PWM1_A of instance: FLEXPWM1
            pub const ALT4_flexpwm1_PWMA1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO01 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO1: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPUART8_TXD of instance: LPUART8
            pub const ALT6_lpuart8_TX: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D02 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO2: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: VIDEO_MUX_EXT_DCIC1 of instance: VIDEO_MUX
            pub const ALT9_video_mux_EXT_DCIC1: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO01 of instance: GPIO9
            pub const ALT10_gpio9_IO1: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: EMVSIM1_SVEN of instance: EMVSIM1
            pub const ALT0_EMVSIM1_SVEN: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART7_RTS_B of instance: LPUART7
            pub const ALT1_lpuart7_RTS_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_1588_EVENT2_OUT of instance: ENET_1G
            pub const ALT2_enet_1g_1588_EVENT2_OUT: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT2_COMPARE2 of instance: GPT2
            pub const ALT3_gpt2_COMPARE2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM1_PWM1_B of instance: FLEXPWM1
            pub const ALT4_flexpwm1_PWMB1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO02 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO2: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPUART8_RXD of instance: LPUART8
            pub const ALT6_lpuart8_RX: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D03 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO3: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: VIDEO_MUX_EXT_DCIC2 of instance: VIDEO_MUX
            pub const ALT9_video_mux_EXT_DCIC2: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO02 of instance: GPIO9
            pub const ALT10_gpio9_IO2: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: EMVSIM1_PD of instance: EMVSIM1
            pub const ALT0_EMVSIM1_PD: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART8_CTS_B of instance: LPUART8
            pub const ALT1_lpuart8_CTS_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_1588_EVENT3_IN of instance: ENET_1G
            pub const ALT2_enet_1g_1588_EVENT3_IN: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT2_COMPARE3 of instance: GPT2
            pub const ALT3_gpt2_COMPARE3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM1_PWM2_A of instance: FLEXPWM1
            pub const ALT4_flexpwm1_PWMA2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO03 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO3: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: WDOG1_B of instance: WDOG1
            pub const ALT6_WDOG1_B: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D04 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO4: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR4_TIMER0 of instance: TMR4
            pub const ALT9_qtimer4_TIMER0: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO03 of instance: GPIO9
            pub const ALT10_gpio9_IO3: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: EMVSIM1_POWER_FAIL of instance: EMVSIM1
            pub const ALT0_EMVSIM1_POWER_FAIL: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART8_RTS_B of instance: LPUART8
            pub const ALT1_lpuart8_RTS_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_1588_EVENT3_OUT of instance: ENET_1G
            pub const ALT2_enet_1g_1588_EVENT3_OUT: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT2_CLK of instance: GPT2
            pub const ALT3_gpt2_CLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM1_PWM2_B of instance: FLEXPWM1
            pub const ALT4_flexpwm1_PWMB2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO04 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO4: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: WDOG2_B of instance: WDOG2
            pub const ALT6_WDOG2_B: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D05 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO5: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR4_TIMER1 of instance: TMR4
            pub const ALT9_qtimer4_TIMER1: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO04 of instance: GPIO9
            pub const ALT10_gpio9_IO4: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG2_OC of instance: USB
            pub const ALT0_usb_OTG2_OC: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXCAN1_TX of instance: FLEXCAN1
            pub const ALT1_can1_TX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: EMVSIM2_IO of instance: EMVSIM2
            pub const ALT2_EMVSIM2_TRXD: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT3_CAPTURE1 of instance: GPT3
            pub const ALT3_gpt3_CAPTURE1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA15 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA15: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO05 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_1588_EVENT1_IN of instance: ENET
            pub const ALT6_enet_1588_EVENT1_IN: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D06 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO6: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR4_TIMER2 of instance: TMR4
            pub const ALT9_qtimer4_TIMER2: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO05 of instance: GPIO9
            pub const ALT10_gpio9_IO5: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM1_PWM0_X of instance: FLEXPWM1
            pub const ALT11_flexpwm1_PWMX0: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG2_PWR of instance: USB
            pub const ALT0_usb_OTG2_PWR: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXCAN1_RX of instance: FLEXCAN1
            pub const ALT1_can1_RX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: EMVSIM2_CLK of instance: EMVSIM2
            pub const ALT2_EMVSIM2_CLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT3_CAPTURE2 of instance: GPT3
            pub const ALT3_gpt3_CAPTURE2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA14 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA14: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO06 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO6: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_1588_EVENT1_OUT of instance: ENET
            pub const ALT6_enet_1588_EVENT1_OUT: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D07 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO7: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: TMR4_TIMER3 of instance: TMR4
            pub const ALT9_qtimer4_TIMER3: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO06 of instance: GPIO9
            pub const ALT10_gpio9_IO6: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM1_PWM1_X of instance: FLEXPWM1
            pub const ALT11_flexpwm1_PWMX1: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USBPHY2_OTG_ID of instance: USBPHY2
            pub const ALT0_usbphy2_OTG_ID: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPI2C1_SCL of instance: LPI2C1
            pub const ALT1_lpi2c1_SCL: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: EMVSIM2_RST of instance: EMVSIM2
            pub const ALT2_EMVSIM2_RST_B: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT3_COMPARE1 of instance: GPT3
            pub const ALT3_gpt3_COMPARE1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA13 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA13: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO07 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO7: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_1588_EVENT2_IN of instance: ENET
            pub const ALT6_enet_1588_EVENT2_IN: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D08 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO8: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO07 of instance: GPIO9
            pub const ALT10_gpio9_IO7: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM1_PWM2_X of instance: FLEXPWM1
            pub const ALT11_flexpwm1_PWMX2: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USBPHY1_OTG_ID of instance: USBPHY1
            pub const ALT0_usbphy1_OTG_ID: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPI2C1_SDA of instance: LPI2C1
            pub const ALT1_lpi2c1_SDA: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: EMVSIM2_SVEN of instance: EMVSIM2
            pub const ALT2_EMVSIM2_SVEN: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT3_COMPARE2 of instance: GPT3
            pub const ALT3_gpt3_COMPARE2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA12 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA12: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO08 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO8: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_1588_EVENT2_OUT of instance: ENET
            pub const ALT6_enet_1588_EVENT2_OUT: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D09 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO9: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO08 of instance: GPIO9
            pub const ALT10_gpio9_IO8: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM1_PWM3_X of instance: FLEXPWM1
            pub const ALT11_flexpwm1_PWMX3: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG1_PWR of instance: USB
            pub const ALT0_usb_OTG1_PWR: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPI2C1_SCLS of instance: LPI2C1
            pub const ALT1_lpi2c1_SCLS: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: EMVSIM2_PD of instance: EMVSIM2
            pub const ALT2_EMVSIM2_PD: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT3_COMPARE3 of instance: GPT3
            pub const ALT3_gpt3_COMPARE3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA11 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA11: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO09 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO9: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_1588_EVENT3_IN of instance: ENET
            pub const ALT6_enet_1588_EVENT3_IN: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D10 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO10: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO09 of instance: GPIO9
            pub const ALT10_gpio9_IO9: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM2_PWM0_X of instance: FLEXPWM2
            pub const ALT11_flexpwm2_PWMX0: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG1_OC of instance: USB
            pub const ALT0_usb_OTG1_OC: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPI2C1_SDAS of instance: LPI2C1
            pub const ALT1_lpi2c1_SDAS: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: EMVSIM2_POWER_FAIL of instance: EMVSIM2
            pub const ALT2_EMVSIM2_POWER_FAIL: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT3_CLK of instance: GPT3
            pub const ALT3_gpt3_CLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA10 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA10: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO10 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO10: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_1588_EVENT3_OUT of instance: ENET
            pub const ALT6_enet_1588_EVENT3_OUT: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D11 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO11: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO10 of instance: GPIO9
            pub const ALT10_gpio9_IO10: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM2_PWM1_X of instance: FLEXPWM2
            pub const ALT11_flexpwm2_PWMX1: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SPDIF_LOCK of instance: SPDIF
            pub const ALT0_spdif_LOCK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPI2C1_HREQ of instance: LPI2C1
            pub const ALT1_lpi2c1_HREQ: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT1_CAPTURE1 of instance: GPT1
            pub const ALT2_gpt1_CAPTURE1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_B_DATA03 of instance: FLEXSPI1
            pub const ALT3_flexspi1_B_DATA3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_PIXCLK of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_PIXCLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO11 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO11: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_TX_DATA03 of instance: ENET
            pub const ALT6_ENET_TX_DATA3: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D12 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO12: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: EWM_OUT_B of instance: EWM
            pub const ALT9_EWM_OUT_B: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO11 of instance: GPIO9
            pub const ALT10_gpio9_IO11: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM2_PWM2_X of instance: FLEXPWM2
            pub const ALT11_flexpwm2_PWMX2: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SPDIF_SR_CLK of instance: SPDIF
            pub const ALT0_spdif_SR_CLK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: PIT1_TRIGGER0 of instance: PIT1
            pub const ALT1_pit1_TRIGGER0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT1_CAPTURE2 of instance: GPT1
            pub const ALT2_gpt1_CAPTURE2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_B_DATA02 of instance: FLEXSPI1
            pub const ALT3_flexspi1_B_DATA2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_MCLK of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_MCLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO12 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO12: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_TX_DATA02 of instance: ENET
            pub const ALT6_ENET_TX_DATA2: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D13 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO13: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: REF_CLK_32K of instance: XTAL OSC
            pub const ALT9_anatop_32K_OUT: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO12 of instance: GPIO9
            pub const ALT10_gpio9_IO12: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM2_PWM3_X of instance: FLEXPWM2
            pub const ALT11_flexpwm2_PWMX3: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_14 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SPDIF_EXT_CLK of instance: SPDIF
            pub const ALT0_spdif_EXT_CLK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: REF_CLK_24M of instance: XTAL OSC
            pub const ALT1_anatop_24M_OUT: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT1_COMPARE1 of instance: GPT1
            pub const ALT2_gpt1_COMPARE1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_B_DATA01 of instance: FLEXSPI1
            pub const ALT3_flexspi1_B_DATA1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_VSYNC of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_VSYNC: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO13 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO13: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_RX_CLK of instance: ENET
            pub const ALT6_enet_RX_CLK: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D14 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO14: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: CCM_ENET_REF_CLK_25M of instance: CCM
            pub const ALT9_CCM_ENET_REF_CLK_25M: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO13 of instance: GPIO9
            pub const ALT10_gpio9_IO13: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM0_X of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMX0: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_15 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SPDIF_IN of instance: SPDIF
            pub const ALT0_spdif_IN: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART10_TXD of instance: LPUART10
            pub const ALT1_lpuart10_TX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT1_COMPARE2 of instance: GPT1
            pub const ALT2_gpt1_COMPARE2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_B_DATA00 of instance: FLEXSPI1
            pub const ALT3_flexspi1_B_DATA0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_HSYNC of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_HSYNC: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO14 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO14: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_TX_ER of instance: ENET
            pub const ALT6_enet_TX_ER: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D15 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO15: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO14 of instance: GPIO9
            pub const ALT10_gpio9_IO14: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM1_X of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMX1: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_15
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_16 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_16 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SPDIF_OUT of instance: SPDIF
            pub const ALT0_spdif_OUT: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART10_RXD of instance: LPUART10
            pub const ALT1_lpuart10_RX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT1_COMPARE3 of instance: GPT1
            pub const ALT2_gpt1_COMPARE3: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_B_SCLK of instance: FLEXSPI1
            pub const ALT3_flexspi1_B_SCLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA09 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA9: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO15 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO15: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_RX_DATA03 of instance: ENET
            pub const ALT6_ENET_RX_DATA3: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D16 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO16: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_1G_MDC of instance: ENET_1G
            pub const ALT9_enet_1g_MDC: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO15 of instance: GPIO9
            pub const ALT10_gpio9_IO15: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM2_X of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMX2: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_16
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_17 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_17 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SAI1_MCLK of instance: SAI1
            pub const ALT0_sai1_MCLK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ACMP1_OUT of instance: ACMP1
            pub const ALT1_ACMP1_OUT: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT1_CLK of instance: GPT1
            pub const ALT2_gpt1_CLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_A_DQS of instance: FLEXSPI1
            pub const ALT3_flexspi1_A_DQS: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA08 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA8: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO16 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO16: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_RX_DATA02 of instance: ENET
            pub const ALT6_ENET_RX_DATA2: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D17 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO17: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_1G_MDIO of instance: ENET_1G
            pub const ALT9_enet_1g_MDIO: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO16 of instance: GPIO9
            pub const ALT10_gpio9_IO16: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM3_PWM3_X of instance: FLEXPWM3
            pub const ALT11_flexpwm3_PWMX3: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_17
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_18 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_18 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SAI1_RX_SYNC of instance: SAI1
            pub const ALT0_sai1_RX_SYNC: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ACMP2_OUT of instance: ACMP2
            pub const ALT1_ACMP2_OUT: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI1_PCS1 of instance: LPSPI1
            pub const ALT2_lpspi1_PCS1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_A_SS0_B of instance: FLEXSPI1
            pub const ALT3_flexspi1_A_SS0_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA07 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA7: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO17 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO17: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_CRS of instance: ENET
            pub const ALT6_enet_CRS: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D18 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO18: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPI2C2_SCL of instance: LPI2C2
            pub const ALT9_lpi2c2_SCL: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO17 of instance: GPIO9
            pub const ALT10_gpio9_IO17: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM4_PWM0_X of instance: FLEXPWM4
            pub const ALT11_flexpwm4_PWMX0: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_18
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_19 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_19 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SAI1_RX_BCLK of instance: SAI1
            pub const ALT0_sai1_RX_BCLK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ACMP3_OUT of instance: ACMP3
            pub const ALT1_ACMP3_OUT: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI1_PCS2 of instance: LPSPI1
            pub const ALT2_lpspi1_PCS2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_A_SCLK of instance: FLEXSPI1
            pub const ALT3_flexspi1_A_SCLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA06 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA6: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO18 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO18: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_COL of instance: ENET
            pub const ALT6_enet_COL: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D19 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO19: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPI2C2_SDA of instance: LPI2C2
            pub const ALT9_lpi2c2_SDA: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO18 of instance: GPIO9
            pub const ALT10_gpio9_IO18: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM4_PWM1_X of instance: FLEXPWM4
            pub const ALT11_flexpwm4_PWMX1: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_19
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_20 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_20 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SAI1_RX_DATA00 of instance: SAI1
            pub const ALT0_sai1_RX_DATA0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ACMP4_OUT of instance: ACMP4
            pub const ALT1_ACMP4_OUT: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI1_PCS3 of instance: LPSPI1
            pub const ALT2_lpspi1_PCS3: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_A_DATA00 of instance: FLEXSPI1
            pub const ALT3_flexspi1_A_DATA0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA05 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA5: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO19 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO19: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_ROW07 of instance: KPP
            pub const ALT6_kpp_ROW7: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D20 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO20: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_1588_EVENT2_OUT of instance: ENET_QOS
            pub const ALT9_enet_qos_1588_EVENT2_OUT: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO19 of instance: GPIO9
            pub const ALT10_gpio9_IO19: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM4_PWM2_X of instance: FLEXPWM4
            pub const ALT11_flexpwm4_PWMX2: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_20
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_21 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_21 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SAI1_TX_DATA00 of instance: SAI1
            pub const ALT0_sai1_TX_DATA0: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI2_PCS1 of instance: LPSPI2
            pub const ALT2_lpspi2_PCS1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_A_DATA01 of instance: FLEXSPI1
            pub const ALT3_flexspi1_A_DATA1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA04 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO20 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO20: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_COL07 of instance: KPP
            pub const ALT6_kpp_COL7: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D21 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO21: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_1588_EVENT2_IN of instance: ENET_QOS
            pub const ALT9_enet_qos_1588_EVENT2_IN: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO20 of instance: GPIO9
            pub const ALT10_gpio9_IO20: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: FLEXPWM4_PWM3_X of instance: FLEXPWM4
            pub const ALT11_flexpwm4_PWMX3: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_21
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_22 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_22 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SAI1_TX_BCLK of instance: SAI1
            pub const ALT0_sai1_TX_BCLK: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI2_PCS2 of instance: LPSPI2
            pub const ALT2_lpspi2_PCS2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_A_DATA02 of instance: FLEXSPI1
            pub const ALT3_flexspi1_A_DATA2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA03 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA3: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO21 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO21: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_ROW06 of instance: KPP
            pub const ALT6_kpp_ROW6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D22 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO22: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_1588_EVENT3_OUT of instance: ENET_QOS
            pub const ALT9_enet_qos_1588_EVENT3_OUT: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO21 of instance: GPIO9
            pub const ALT10_gpio9_IO21: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_22
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_23 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_23 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SAI1_TX_SYNC of instance: SAI1
            pub const ALT0_sai1_TX_SYNC: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI2_PCS3 of instance: LPSPI2
            pub const ALT2_lpspi2_PCS3: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_A_DATA03 of instance: FLEXSPI1
            pub const ALT3_flexspi1_A_DATA3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: VIDEO_MUX_CSI_DATA02 of instance: VIDEO_MUX
            pub const ALT4_video_mux_CSI_DATA2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO22 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO22: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_COL06 of instance: KPP
            pub const ALT6_kpp_COL6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D23 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO23: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_1588_EVENT3_IN of instance: ENET_QOS
            pub const ALT9_enet_qos_1588_EVENT3_IN: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO22 of instance: GPIO9
            pub const ALT10_gpio9_IO22: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_23
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_24 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_24 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPUART1_TXD of instance: LPUART1
            pub const ALT0_lpuart1_TX: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI2_SCK of instance: LPSPI2
            pub const ALT1_lpspi2_SCK: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: VIDEO_MUX_CSI_DATA00 of instance: VIDEO_MUX
            pub const ALT2_video_mux_CSI_DATA0: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_EN of instance: ENET
            pub const ALT3_enet_RX_EN: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM2_PWM0_A of instance: FLEXPWM2
            pub const ALT4_flexpwm2_PWMA0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO23 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO23: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_ROW05 of instance: KPP
            pub const ALT6_kpp_ROW5: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D24 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO24: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPI2C4_SCL of instance: LPI2C4
            pub const ALT9_lpi2c4_SCL: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO23 of instance: GPIO9
            pub const ALT10_gpio9_IO23: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_24
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_25 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_25 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPUART1_RXD of instance: LPUART1
            pub const ALT0_lpuart1_RX: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI2_PCS0 of instance: LPSPI2
            pub const ALT1_lpspi2_PCS0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: VIDEO_MUX_CSI_DATA01 of instance: VIDEO_MUX
            pub const ALT2_video_mux_CSI_DATA1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_ER of instance: ENET
            pub const ALT3_enet_RX_ER: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM2_PWM0_B of instance: FLEXPWM2
            pub const ALT4_flexpwm2_PWMB0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO24 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO24: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_COL05 of instance: KPP
            pub const ALT6_kpp_COL5: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D25 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO25: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPI2C4_SDA of instance: LPI2C4
            pub const ALT9_lpi2c4_SDA: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO24 of instance: GPIO9
            pub const ALT10_gpio9_IO24: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_25
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_26 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_26 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPUART1_CTS_B of instance: LPUART1
            pub const ALT0_lpuart1_CTS_B: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI2_SOUT of instance: LPSPI2
            pub const ALT1_lpspi2_SDO: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SEMC_CSX01 of instance: SEMC
            pub const ALT2_semc_CSX1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_DATA00 of instance: ENET
            pub const ALT3_ENET_RX_DATA0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM2_PWM1_A of instance: FLEXPWM2
            pub const ALT4_flexpwm2_PWMA1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO25 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO25: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_ROW04 of instance: KPP
            pub const ALT6_kpp_ROW4: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D26 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO26: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_MDC of instance: ENET_QOS
            pub const ALT9_enet_qos_MDC: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO25 of instance: GPIO9
            pub const ALT10_gpio9_IO25: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: USDHC2_CD_B of instance: USDHC2
            pub const ALT11_usdhc2_CD_B: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_26
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_27 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_27 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPUART1_RTS_B of instance: LPUART1
            pub const ALT0_lpuart1_RTS_B: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI2_SIN of instance: LPSPI2
            pub const ALT1_lpspi2_SDI: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SEMC_CSX02 of instance: SEMC
            pub const ALT2_semc_CSX2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_DATA01 of instance: ENET
            pub const ALT3_ENET_RX_DATA1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM2_PWM1_B of instance: FLEXPWM2
            pub const ALT4_flexpwm2_PWMB1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO26 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO26: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_COL04 of instance: KPP
            pub const ALT6_kpp_COL4: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D27 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO27: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_MDIO of instance: ENET_QOS
            pub const ALT9_enet_qos_MDIO: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO26 of instance: GPIO9
            pub const ALT10_gpio9_IO26: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: USDHC2_WP of instance: USDHC2
            pub const ALT11_usdhc2_WP: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_27
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_28 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_28 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPSPI1_SCK of instance: LPSPI1
            pub const ALT0_lpspi1_SCK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART5_TXD of instance: LPUART5
            pub const ALT1_lpuart5_TX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SEMC_CSX03 of instance: SEMC
            pub const ALT2_semc_CSX3: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_EN of instance: ENET
            pub const ALT3_enet_TX_EN: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM2_PWM2_A of instance: FLEXPWM2
            pub const ALT4_flexpwm2_PWMA2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO27 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO27: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_ROW03 of instance: KPP
            pub const ALT6_kpp_ROW3: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D28 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO28: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: VIDEO_MUX_EXT_DCIC1 of instance: VIDEO_MUX
            pub const ALT9_video_mux_EXT_DCIC1: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO27 of instance: GPIO9
            pub const ALT10_gpio9_IO27: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: USDHC2_VSELECT of instance: USDHC2
            pub const ALT11_usdhc2_VSELECT: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_28
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_29 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_29 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPSPI1_PCS0 of instance: LPSPI1
            pub const ALT0_lpspi1_PCS0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART5_RXD of instance: LPUART5
            pub const ALT1_lpuart5_RX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_REF_CLK of instance: ENET
            pub const ALT2_enet_REF_CLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_CLK of instance: ENET
            pub const ALT3_enet_TX_CLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM2_PWM2_B of instance: FLEXPWM2
            pub const ALT4_flexpwm2_PWMB2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO28 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO28: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_COL03 of instance: KPP
            pub const ALT6_kpp_COL3: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D29 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO29: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: VIDEO_MUX_EXT_DCIC2 of instance: VIDEO_MUX
            pub const ALT9_video_mux_EXT_DCIC2: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO28 of instance: GPIO9
            pub const ALT10_gpio9_IO28: u32 = 0b1010;

            /// 0b1011: Select mux mode: ALT11 mux port: USDHC2_RESET_B of instance: USDHC2
            pub const ALT11_usdhc2_RESET_B: u32 = 0b1011;
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

            /// 0b1: Force input path of pad GPIO_AD_29
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_30 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_30 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPSPI1_SOUT of instance: LPSPI1
            pub const ALT0_lpspi1_SDO: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USB_OTG2_OC of instance: USB
            pub const ALT1_usb_OTG2_OC: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXCAN2_TX of instance: FLEXCAN2
            pub const ALT2_can2_TX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_DATA00 of instance: ENET
            pub const ALT3_ENET_TX_DATA0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPUART3_TXD of instance: LPUART3
            pub const ALT4_lpuart3_TX: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO29 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO29: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_ROW02 of instance: KPP
            pub const ALT6_kpp_ROW2: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D30 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO30: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: WDOG2_RESET_B_DEB of instance: WDOG2
            pub const ALT9_WDOG2_RESET_B_DEB: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO29 of instance: GPIO9
            pub const ALT10_gpio9_IO29: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_30
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_31 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_31 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPSPI1_SIN of instance: LPSPI1
            pub const ALT0_lpspi1_SDI: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USB_OTG2_PWR of instance: USB
            pub const ALT1_usb_OTG2_PWR: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXCAN2_RX of instance: FLEXCAN2
            pub const ALT2_can2_RX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_DATA01 of instance: ENET
            pub const ALT3_ENET_TX_DATA1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPUART3_RXD of instance: LPUART3
            pub const ALT4_lpuart3_RX: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO30 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO30: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_COL02 of instance: KPP
            pub const ALT6_kpp_COL2: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXIO2_D31 of instance: FLEXIO2
            pub const ALT8_flexio2_FLEXIO31: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: WDOG1_RESET_B_DEB of instance: WDOG1
            pub const ALT9_WDOG1_RESET_B_DEB: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO30 of instance: GPIO9
            pub const ALT10_gpio9_IO30: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_31
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_32 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_32 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPI2C1_SCL of instance: LPI2C1
            pub const ALT0_lpi2c1_SCL: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USBPHY2_OTG_ID of instance: USBPHY2
            pub const ALT1_usbphy2_OTG_ID: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: PGMC_PMIC_RDY of instance: pgmc
            pub const ALT2_pgmc_PMIC_RDY: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_MDC of instance: ENET
            pub const ALT3_enet_MDC: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: USDHC1_CD_B of instance: USDHC1
            pub const ALT4_usdhc1_CD_B: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX3_IO31 of instance: GPIO_MUX3
            pub const ALT5_gpio_mux3_IO31: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_ROW01 of instance: KPP
            pub const ALT6_kpp_ROW1: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: LPUART10_TXD of instance: LPUART10
            pub const ALT8_lpuart10_TX: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_1G_MDC of instance: ENET_1G
            pub const ALT9_enet_1g_MDC: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO9_IO31 of instance: GPIO9
            pub const ALT10_gpio9_IO31: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_32
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_33 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_33 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LPI2C1_SDA of instance: LPI2C1
            pub const ALT0_lpi2c1_SDA: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USBPHY1_OTG_ID of instance: USBPHY1
            pub const ALT1_usbphy1_OTG_ID: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: XBAR1_INOUT17 of instance: XBAR1
            pub const ALT2_XBAR1_INOUT17: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_MDIO of instance: ENET
            pub const ALT3_enet_MDIO: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: USDHC1_WP of instance: USDHC1
            pub const ALT4_usdhc1_WP: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO00 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO0: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_COL01 of instance: KPP
            pub const ALT6_kpp_COL1: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: LPUART10_RXD of instance: LPUART10
            pub const ALT8_lpuart10_RX: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_1G_MDIO of instance: ENET_1G
            pub const ALT9_enet_1g_MDIO: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO00 of instance: GPIO10
            pub const ALT10_gpio10_IO0: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_33
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_34 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_34 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: ENET_1G_1588_EVENT0_IN of instance: ENET_1G
            pub const ALT0_enet_1g_1588_EVENT0_IN: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USB_OTG1_PWR of instance: USB
            pub const ALT1_usb_OTG1_PWR: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: XBAR1_INOUT18 of instance: XBAR1
            pub const ALT2_XBAR1_INOUT18: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_1588_EVENT0_IN of instance: ENET
            pub const ALT3_enet_1588_EVENT0_IN: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: USDHC1_VSELECT of instance: USDHC1
            pub const ALT4_usdhc1_VSELECT: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO01 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO1: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_ROW00 of instance: KPP
            pub const ALT6_kpp_ROW0: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: LPUART10_CTS_B of instance: LPUART10
            pub const ALT8_lpuart10_CTS_B: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: WDOG1_ANY of instance: WDOG1
            pub const ALT9_WDOG1_ANY: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO01 of instance: GPIO10
            pub const ALT10_gpio10_IO1: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_34
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_35 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_35 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: ENET_1G_1588_EVENT0_OUT of instance: ENET_1G
            pub const ALT0_enet_1g_1588_EVENT0_OUT: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USB_OTG1_OC of instance: USB
            pub const ALT1_usb_OTG1_OC: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: XBAR1_INOUT19 of instance: XBAR1
            pub const ALT2_XBAR1_INOUT19: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_1588_EVENT0_OUT of instance: ENET
            pub const ALT3_enet_1588_EVENT0_OUT: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: USDHC1_RESET_B of instance: USDHC1
            pub const ALT4_usdhc1_RESET_B: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO02 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO2: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: KPP_COL00 of instance: KPP
            pub const ALT6_kpp_COL0: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: LPUART10_RTS_B of instance: LPUART10
            pub const ALT8_lpuart10_RTS_B: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXSPI1_B_SS1_B of instance: FLEXSPI1
            pub const ALT9_flexspi1_B_SS1_B: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO02 of instance: GPIO10
            pub const ALT10_gpio10_IO2: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_AD_35
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_CMD of instance: USDHC1
            pub const ALT0_usdhc1_CMD: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: XBAR1_INOUT20 of instance: XBAR1
            pub const ALT2_XBAR1_INOUT20: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT4_CAPTURE1 of instance: GPT4
            pub const ALT3_gpt4_CAPTURE1: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO03 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO3: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXSPI2_A_SS0_B of instance: FLEXSPI2
            pub const ALT6_flexspi2_A_SS0_B: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: KPP_ROW07 of instance: KPP
            pub const ALT8_kpp_ROW7: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO03 of instance: GPIO10
            pub const ALT10_gpio10_IO3: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B1_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_CLK of instance: USDHC1
            pub const ALT0_usdhc1_CLK: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: XBAR1_INOUT21 of instance: XBAR1
            pub const ALT2_XBAR1_INOUT21: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT4_CAPTURE2 of instance: GPT4
            pub const ALT3_gpt4_CAPTURE2: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO04 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO4: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXSPI2_A_SCLK of instance: FLEXSPI2
            pub const ALT6_flexspi2_A_SCLK: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: KPP_COL07 of instance: KPP
            pub const ALT8_kpp_COL7: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO04 of instance: GPIO10
            pub const ALT10_gpio10_IO4: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B1_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_DATA0 of instance: USDHC1
            pub const ALT0_usdhc1_DATA0: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: XBAR1_INOUT22 of instance: XBAR1
            pub const ALT2_XBAR1_INOUT22: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT4_COMPARE1 of instance: GPT4
            pub const ALT3_gpt4_COMPARE1: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO05 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXSPI2_A_DATA00 of instance: FLEXSPI2
            pub const ALT6_flexspi2_A_DATA0: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: KPP_ROW06 of instance: KPP
            pub const ALT8_kpp_ROW6: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXSPI1_A_SS1_B of instance: FLEXSPI1
            pub const ALT9_flexspi1_A_SS1_B: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO05 of instance: GPIO10
            pub const ALT10_gpio10_IO5: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B1_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_DATA1 of instance: USDHC1
            pub const ALT0_usdhc1_DATA1: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: XBAR1_INOUT23 of instance: XBAR1
            pub const ALT2_XBAR1_INOUT23: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT4_COMPARE2 of instance: GPT4
            pub const ALT3_gpt4_COMPARE2: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO06 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO6: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXSPI2_A_DATA01 of instance: FLEXSPI2
            pub const ALT6_flexspi2_A_DATA1: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: KPP_COL06 of instance: KPP
            pub const ALT8_kpp_COL6: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXSPI1_B_SS1_B of instance: FLEXSPI1
            pub const ALT9_flexspi1_B_SS1_B: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO06 of instance: GPIO10
            pub const ALT10_gpio10_IO6: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B1_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_DATA2 of instance: USDHC1
            pub const ALT0_usdhc1_DATA2: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: XBAR1_INOUT24 of instance: XBAR1
            pub const ALT2_XBAR1_INOUT24: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT4_COMPARE3 of instance: GPT4
            pub const ALT3_gpt4_COMPARE3: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO07 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO7: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXSPI2_A_DATA02 of instance: FLEXSPI2
            pub const ALT6_flexspi2_A_DATA2: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXSPI1_B_SS0_B of instance: FLEXSPI1
            pub const ALT8_flexspi1_B_SS0_B: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_1588_EVENT2_AUX_IN of instance: ENET_QOS
            pub const ALT9_enet_qos_1588_EVENT2_AUX_IN: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO07 of instance: GPIO10
            pub const ALT10_gpio10_IO7: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B1_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_DATA3 of instance: USDHC1
            pub const ALT0_usdhc1_DATA3: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: XBAR1_INOUT25 of instance: XBAR1
            pub const ALT2_XBAR1_INOUT25: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: GPT4_CLK of instance: GPT4
            pub const ALT3_gpt4_CLK: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO08 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO8: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXSPI2_A_DATA03 of instance: FLEXSPI2
            pub const ALT6_flexspi2_A_DATA3: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXSPI1_B_DQS of instance: FLEXSPI1
            pub const ALT8_flexspi1_B_DQS: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_1588_EVENT3_AUX_IN of instance: ENET_QOS
            pub const ALT9_enet_qos_1588_EVENT3_AUX_IN: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO08 of instance: GPIO10
            pub const ALT10_gpio10_IO8: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B1_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA3 of instance: USDHC2
            pub const ALT0_usdhc2_DATA3: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_B_DATA03 of instance: FLEXSPI1
            pub const ALT1_flexspi1_B_DATA3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_EN of instance: ENET_1G
            pub const ALT2_enet_1g_RX_EN: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART9_TXD of instance: LPUART9
            pub const ALT3_lpuart9_TX: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI4_SCK of instance: LPSPI4
            pub const ALT4_lpspi4_SCK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO09 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO9: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO09 of instance: GPIO10
            pub const ALT10_gpio10_IO9: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA2 of instance: USDHC2
            pub const ALT0_usdhc2_DATA2: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_B_DATA02 of instance: FLEXSPI1
            pub const ALT1_flexspi1_B_DATA2: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_CLK of instance: ENET_1G
            pub const ALT2_enet_1g_RX_CLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART9_RXD of instance: LPUART9
            pub const ALT3_lpuart9_RX: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI4_PCS0 of instance: LPSPI4
            pub const ALT4_lpspi4_PCS0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO10 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO10: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO10 of instance: GPIO10
            pub const ALT10_gpio10_IO10: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA1 of instance: USDHC2
            pub const ALT0_usdhc2_DATA1: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_B_DATA01 of instance: FLEXSPI1
            pub const ALT1_flexspi1_B_DATA1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_DATA00 of instance: ENET_1G
            pub const ALT2_ENET_1G_RX_DATA0: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART9_CTS_B of instance: LPUART9
            pub const ALT3_lpuart9_CTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI4_SOUT of instance: LPSPI4
            pub const ALT4_lpspi4_SDO: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO11 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO11: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO11 of instance: GPIO10
            pub const ALT10_gpio10_IO11: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA0 of instance: USDHC2
            pub const ALT0_usdhc2_DATA0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_B_DATA00 of instance: FLEXSPI1
            pub const ALT1_flexspi1_B_DATA0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_DATA01 of instance: ENET_1G
            pub const ALT2_ENET_1G_RX_DATA1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART9_RTS_B of instance: LPUART9
            pub const ALT3_lpuart9_RTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI4_SIN of instance: LPSPI4
            pub const ALT4_lpspi4_SDI: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO12 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO12: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO12 of instance: GPIO10
            pub const ALT10_gpio10_IO12: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_CLK of instance: USDHC2
            pub const ALT0_usdhc2_CLK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_B_SCLK of instance: FLEXSPI1
            pub const ALT1_flexspi1_B_SCLK: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_DATA02 of instance: ENET_1G
            pub const ALT2_ENET_1G_RX_DATA2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_A_SS1_B of instance: FLEXSPI1
            pub const ALT3_flexspi1_A_SS1_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI4_PCS1 of instance: LPSPI4
            pub const ALT4_lpspi4_PCS1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO13 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO13: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO13 of instance: GPIO10
            pub const ALT10_gpio10_IO13: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_CMD of instance: USDHC2
            pub const ALT0_usdhc2_CMD: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_A_DQS of instance: FLEXSPI1
            pub const ALT1_flexspi1_A_DQS: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_DATA03 of instance: ENET_1G
            pub const ALT2_ENET_1G_RX_DATA3: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXSPI1_B_SS0_B of instance: FLEXSPI1
            pub const ALT3_flexspi1_B_SS0_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI4_PCS2 of instance: LPSPI4
            pub const ALT4_lpspi4_PCS2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO14 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO14: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO14 of instance: GPIO10
            pub const ALT10_gpio10_IO14: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_RESET_B of instance: USDHC2
            pub const ALT0_usdhc2_RESET_B: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_A_SS0_B of instance: FLEXSPI1
            pub const ALT1_flexspi1_A_SS0_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_DATA03 of instance: ENET_1G
            pub const ALT2_ENET_1G_TX_DATA3: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI4_PCS3 of instance: LPSPI4
            pub const ALT3_lpspi4_PCS3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: GPT6_CAPTURE1 of instance: GPT6
            pub const ALT4_gpt6_CAPTURE1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO15 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO15: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO15 of instance: GPIO10
            pub const ALT10_gpio10_IO15: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_STROBE of instance: USDHC2
            pub const ALT0_usdhc2_STROBE: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_A_SCLK of instance: FLEXSPI1
            pub const ALT1_flexspi1_A_SCLK: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_DATA02 of instance: ENET_1G
            pub const ALT2_ENET_1G_TX_DATA2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART3_CTS_B of instance: LPUART3
            pub const ALT3_lpuart3_CTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: GPT6_CAPTURE2 of instance: GPT6
            pub const ALT4_gpt6_CAPTURE2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO16 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO16: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPSPI2_SCK of instance: LPSPI2
            pub const ALT6_lpspi2_SCK: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_TX_ER of instance: ENET
            pub const ALT8_enet_TX_ER: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_REF_CLK of instance: ENET_QOS
            pub const ALT9_CCM_enet_qos_clock_generate_REF_CLK: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO16 of instance: GPIO10
            pub const ALT10_gpio10_IO16: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA4 of instance: USDHC2
            pub const ALT0_usdhc2_DATA4: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_A_DATA00 of instance: FLEXSPI1
            pub const ALT1_flexspi1_A_DATA0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_DATA01 of instance: ENET_1G
            pub const ALT2_ENET_1G_TX_DATA1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART3_RTS_B of instance: LPUART3
            pub const ALT3_lpuart3_RTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: GPT6_COMPARE1 of instance: GPT6
            pub const ALT4_gpt6_COMPARE1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO17 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO17: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPSPI2_PCS0 of instance: LPSPI2
            pub const ALT6_lpspi2_PCS0: u32 = 0b0110;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO17 of instance: GPIO10
            pub const ALT10_gpio10_IO17: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA5 of instance: USDHC2
            pub const ALT0_usdhc2_DATA5: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_A_DATA01 of instance: FLEXSPI1
            pub const ALT1_flexspi1_A_DATA1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_DATA00 of instance: ENET_1G
            pub const ALT2_ENET_1G_TX_DATA0: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART5_CTS_B of instance: LPUART5
            pub const ALT3_lpuart5_CTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: GPT6_COMPARE2 of instance: GPT6
            pub const ALT4_gpt6_COMPARE2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO18 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO18: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPSPI2_SOUT of instance: LPSPI2
            pub const ALT6_lpspi2_SDO: u32 = 0b0110;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO18 of instance: GPIO10
            pub const ALT10_gpio10_IO18: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA6 of instance: USDHC2
            pub const ALT0_usdhc2_DATA6: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_A_DATA02 of instance: FLEXSPI1
            pub const ALT1_flexspi1_A_DATA2: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_EN of instance: ENET_1G
            pub const ALT2_enet_1g_TX_EN: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART5_RTS_B of instance: LPUART5
            pub const ALT3_lpuart5_RTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: GPT6_COMPARE3 of instance: GPT6
            pub const ALT4_gpt6_COMPARE3: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO19 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO19: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPSPI2_SIN of instance: LPSPI2
            pub const ALT6_lpspi2_SDI: u32 = 0b0110;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO19 of instance: GPIO10
            pub const ALT10_gpio10_IO19: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B2_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B2_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA7 of instance: USDHC2
            pub const ALT0_usdhc2_DATA7: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPI1_A_DATA03 of instance: FLEXSPI1
            pub const ALT1_flexspi1_A_DATA3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_TX_CLK_IO of instance: ENET_1G
            pub const ALT2_enet_1g_TX_CLK_IO: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_1G_REF_CLK of instance: ENET_1G
            pub const ALT3_enet_1g_REF_CLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: GPT6_CLK of instance: GPT6
            pub const ALT4_gpt6_CLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO20 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO20: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPSPI2_PCS1 of instance: LPSPI2
            pub const ALT6_lpspi2_PCS1: u32 = 0b0110;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO20 of instance: GPIO10
            pub const ALT10_gpio10_IO20: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_SD_B2_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_CLK of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_CLK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_RX_EN of instance: ENET_1G
            pub const ALT1_enet_1g_RX_EN: u32 = 0b0001;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR1_TIMER0 of instance: TMR1
            pub const ALT3_qtimer1_TIMER0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT26 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT26: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO21 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO21: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_EN of instance: ENET_QOS
            pub const ALT8_enet_qos_RX_EN: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO21 of instance: GPIO10
            pub const ALT10_gpio10_IO21: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_ENABLE of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_ENABLE: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_RX_CLK of instance: ENET_1G
            pub const ALT1_enet_1g_RX_CLK: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_RX_ER of instance: ENET_1G
            pub const ALT2_enet_1g_RX_ER: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR1_TIMER1 of instance: TMR1
            pub const ALT3_qtimer1_TIMER1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT27 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT27: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO22 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO22: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_CLK of instance: ENET_QOS
            pub const ALT8_CCM_enet_qos_clock_generate_RX_CLK: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_RX_ER of instance: ENET_QOS
            pub const ALT9_enet_qos_RX_ER: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO22 of instance: GPIO10
            pub const ALT10_gpio10_IO22: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_HSYNC of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_HSYNC: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_RX_DATA00 of instance: ENET_1G
            pub const ALT1_ENET_1G_RX_DATA0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C3_SCL of instance: LPI2C3
            pub const ALT2_lpi2c3_SCL: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR1_TIMER2 of instance: TMR1
            pub const ALT3_qtimer1_TIMER2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT28 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT28: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO23 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO23: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_DATA00 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_RX_DATA0: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPUART1_TXD of instance: LPUART1
            pub const ALT9_lpuart1_TX: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO23 of instance: GPIO10
            pub const ALT10_gpio10_IO23: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_VSYNC of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_VSYNC: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_RX_DATA01 of instance: ENET_1G
            pub const ALT1_ENET_1G_RX_DATA1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C3_SDA of instance: LPI2C3
            pub const ALT2_lpi2c3_SDA: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR2_TIMER0 of instance: TMR2
            pub const ALT3_qtimer2_TIMER0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT29 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT29: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO24 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO24: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_DATA01 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_RX_DATA1: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPUART1_RXD of instance: LPUART1
            pub const ALT9_lpuart1_RX: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO24 of instance: GPIO10
            pub const ALT10_gpio10_IO24: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA00 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_RX_DATA02 of instance: ENET_1G
            pub const ALT1_ENET_1G_RX_DATA2: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART4_RXD of instance: LPUART4
            pub const ALT2_lpuart4_RX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR2_TIMER1 of instance: TMR2
            pub const ALT3_qtimer2_TIMER1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT30 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT30: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO25 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO25: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_DATA02 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_RX_DATA2: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI3_SCK of instance: LPSPI3
            pub const ALT9_lpspi3_SCK: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO25 of instance: GPIO10
            pub const ALT10_gpio10_IO25: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA01 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA1: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_RX_DATA03 of instance: ENET_1G
            pub const ALT1_ENET_1G_RX_DATA3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART4_CTS_B of instance: LPUART4
            pub const ALT2_lpuart4_CTS_B: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR2_TIMER2 of instance: TMR2
            pub const ALT3_qtimer2_TIMER2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT31 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT31: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO26 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO26: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_DATA03 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_RX_DATA3: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI3_SIN of instance: LPSPI3
            pub const ALT9_lpspi3_SDI: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO26 of instance: GPIO10
            pub const ALT10_gpio10_IO26: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA02 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA2: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_TX_DATA03 of instance: ENET_1G
            pub const ALT1_ENET_1G_TX_DATA3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART4_TXD of instance: LPUART4
            pub const ALT2_lpuart4_TX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR3_TIMER0 of instance: TMR3
            pub const ALT3_qtimer3_TIMER0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT32 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT32: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO27 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO27: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG00 of instance: SRC
            pub const ALT6_src_BT_CFG0: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_DATA03 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_TX_DATA3: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI3_SOUT of instance: LPSPI3
            pub const ALT9_lpspi3_SDO: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO27 of instance: GPIO10
            pub const ALT10_gpio10_IO27: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA03 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA3: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_TX_DATA02 of instance: ENET_1G
            pub const ALT1_ENET_1G_TX_DATA2: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART4_RTS_B of instance: LPUART4
            pub const ALT2_lpuart4_RTS_B: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR3_TIMER1 of instance: TMR3
            pub const ALT3_qtimer3_TIMER1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT33 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT33: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO28 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO28: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG01 of instance: SRC
            pub const ALT6_src_BT_CFG1: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_DATA02 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_TX_DATA2: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI3_PCS0 of instance: LPSPI3
            pub const ALT9_lpspi3_PCS0: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO28 of instance: GPIO10
            pub const ALT10_gpio10_IO28: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA04 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA4: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_TX_DATA01 of instance: ENET_1G
            pub const ALT1_ENET_1G_TX_DATA1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: USDHC1_CD_B of instance: USDHC1
            pub const ALT2_usdhc1_CD_B: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR3_TIMER2 of instance: TMR3
            pub const ALT3_qtimer3_TIMER2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT34 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT34: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO29 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO29: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG02 of instance: SRC
            pub const ALT6_src_BT_CFG2: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_DATA01 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_TX_DATA1: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI3_PCS1 of instance: LPSPI3
            pub const ALT9_lpspi3_PCS1: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO29 of instance: GPIO10
            pub const ALT10_gpio10_IO29: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA05 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA5: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_TX_DATA00 of instance: ENET_1G
            pub const ALT1_ENET_1G_TX_DATA0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: USDHC1_WP of instance: USDHC1
            pub const ALT2_usdhc1_WP: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR4_TIMER0 of instance: TMR4
            pub const ALT3_qtimer4_TIMER0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT35 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT35: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO30 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO30: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG03 of instance: SRC
            pub const ALT6_src_BT_CFG3: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_DATA00 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_TX_DATA0: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI3_PCS2 of instance: LPSPI3
            pub const ALT9_lpspi3_PCS2: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO30 of instance: GPIO10
            pub const ALT10_gpio10_IO30: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA06 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA6: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_TX_EN of instance: ENET_1G
            pub const ALT1_enet_1g_TX_EN: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: USDHC1_RESET_B of instance: USDHC1
            pub const ALT2_usdhc1_RESET_B: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR4_TIMER1 of instance: TMR4
            pub const ALT3_qtimer4_TIMER1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT36 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT36: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX4_IO31 of instance: GPIO_MUX4
            pub const ALT5_gpio_mux4_IO31: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG04 of instance: SRC
            pub const ALT6_src_BT_CFG4: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_EN of instance: ENET_QOS
            pub const ALT8_enet_qos_TX_EN: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI3_PCS3 of instance: LPSPI3
            pub const ALT9_lpspi3_PCS3: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO10_IO31 of instance: GPIO10
            pub const ALT10_gpio10_IO31: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B1_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B1_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA07 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA7: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_1G_TX_CLK_IO of instance: ENET_1G
            pub const ALT1_enet_1g_TX_CLK_IO: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_1G_REF_CLK of instance: ENET_1G
            pub const ALT2_enet_1g_REF_CLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: TMR4_TIMER2 of instance: TMR4
            pub const ALT3_qtimer4_TIMER2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT37 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT37: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO00 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO0: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG05 of instance: SRC
            pub const ALT6_src_BT_CFG5: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_CLK of instance: ENET_QOS
            pub const ALT8_CCM_enet_qos_clock_generate_TX_CLK: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET_QOS_REF_CLK of instance: ENET_QOS
            pub const ALT9_CCM_enet_qos_clock_generate_REF_CLK: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO00 of instance: GPIO11
            pub const ALT10_gpio11_IO0: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B1_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA08 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA8: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: WDOG1_B of instance: WDOG1
            pub const ALT1_WDOG1_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: MQS_RIGHT of instance: MQS
            pub const ALT2_mqs_RIGHT: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_1G_TX_ER of instance: ENET_1G
            pub const ALT3_enet_1g_TX_ER: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_TX_DATA03 of instance: SAI1
            pub const ALT4_sai1_TX_DATA3: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO01 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO1: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG06 of instance: SRC
            pub const ALT6_src_BT_CFG6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_ER of instance: ENET_QOS
            pub const ALT8_enet_qos_TX_ER: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO01 of instance: GPIO11
            pub const ALT10_gpio11_IO1: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA09 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA9: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: USDHC1_VSELECT of instance: USDHC1
            pub const ALT1_usdhc1_VSELECT: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: MQS_LEFT of instance: MQS
            pub const ALT2_mqs_LEFT: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: WDOG2_B of instance: WDOG2
            pub const ALT3_WDOG2_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_TX_DATA02 of instance: SAI1
            pub const ALT4_sai1_TX_DATA2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO02 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO2: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG07 of instance: SRC
            pub const ALT6_src_BT_CFG7: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: EWM_OUT_B of instance: EWM
            pub const ALT8_EWM_OUT_B: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: CCM_ENET_REF_CLK_25M of instance: CCM
            pub const ALT9_CCM_ENET_REF_CLK_25M: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO02 of instance: GPIO11
            pub const ALT10_gpio11_IO2: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA10 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA10: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_TX_DATA00 of instance: ENET
            pub const ALT1_ENET_TX_DATA0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: PIT1_TRIGGER3 of instance: PIT1
            pub const ALT2_pit1_TRIGGER3: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE00 of instance: ARM
            pub const ALT3_ARM_TRACE0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_TX_DATA01 of instance: SAI1
            pub const ALT4_sai1_TX_DATA1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO03 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO3: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG08 of instance: SRC
            pub const ALT6_src_BT_CFG8: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_DATA00 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_TX_DATA0: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO03 of instance: GPIO11
            pub const ALT10_gpio11_IO3: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA11 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA11: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_TX_DATA01 of instance: ENET
            pub const ALT1_ENET_TX_DATA1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: PIT1_TRIGGER2 of instance: PIT1
            pub const ALT2_pit1_TRIGGER2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE01 of instance: ARM
            pub const ALT3_ARM_TRACE1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_MCLK of instance: SAI1
            pub const ALT4_sai1_MCLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO04 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO4: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG09 of instance: SRC
            pub const ALT6_src_BT_CFG9: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_DATA01 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_TX_DATA1: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO04 of instance: GPIO11
            pub const ALT10_gpio11_IO4: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA12 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA12: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_TX_EN of instance: ENET
            pub const ALT1_enet_TX_EN: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: PIT1_TRIGGER1 of instance: PIT1
            pub const ALT2_pit1_TRIGGER1: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE02 of instance: ARM
            pub const ALT3_ARM_TRACE2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_RX_SYNC of instance: SAI1
            pub const ALT4_sai1_RX_SYNC: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO05 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG10 of instance: SRC
            pub const ALT6_src_BT_CFG10: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_EN of instance: ENET_QOS
            pub const ALT8_enet_qos_TX_EN: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO05 of instance: GPIO11
            pub const ALT10_gpio11_IO5: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA13 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA13: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_TX_CLK of instance: ENET
            pub const ALT1_enet_TX_CLK: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_REF_CLK of instance: ENET
            pub const ALT2_enet_REF_CLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE03 of instance: ARM
            pub const ALT3_ARM_TRACE3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_RX_BCLK of instance: SAI1
            pub const ALT4_sai1_RX_BCLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO06 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO6: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BT_CFG11 of instance: SRC
            pub const ALT6_src_BT_CFG11: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_TX_CLK of instance: ENET_QOS
            pub const ALT8_CCM_enet_qos_clock_generate_TX_CLK: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO06 of instance: GPIO11
            pub const ALT10_gpio11_IO6: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA14 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA14: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_RX_DATA00 of instance: ENET
            pub const ALT1_ENET_RX_DATA0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART7_TXD of instance: LPUART7
            pub const ALT2_lpuart7_TX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE_CLK of instance: ARM
            pub const ALT3_ARM_TRACE_CLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_RX_DATA00 of instance: SAI1
            pub const ALT4_sai1_RX_DATA0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO07 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO7: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_DATA00 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_RX_DATA0: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO07 of instance: GPIO11
            pub const ALT10_gpio11_IO7: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA15 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA15: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_RX_DATA01 of instance: ENET
            pub const ALT1_ENET_RX_DATA1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART7_RXD of instance: LPUART7
            pub const ALT2_lpuart7_RX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE_SWO of instance: ARM
            pub const ALT3_ARM_TRACE_SWO: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_TX_DATA00 of instance: SAI1
            pub const ALT4_sai1_TX_DATA0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO08 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO8: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_DATA01 of instance: ENET_QOS
            pub const ALT8_ENET_QOS_RX_DATA1: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO08 of instance: GPIO11
            pub const ALT10_gpio11_IO8: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA16 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA16: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_RX_EN of instance: ENET
            pub const ALT1_enet_RX_EN: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_TXD of instance: LPUART8
            pub const ALT2_lpuart8_TX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_CM7_EVENTO of instance: CM7
            pub const ALT3_cm7_imxrt_TXEV: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_TX_BCLK of instance: SAI1
            pub const ALT4_sai1_TX_BCLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO09 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO9: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_EN of instance: ENET_QOS
            pub const ALT8_enet_qos_RX_EN: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPUART1_TXD of instance: LPUART1
            pub const ALT9_lpuart1_TX: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO09 of instance: GPIO11
            pub const ALT10_gpio11_IO9: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA17 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA17: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_RX_ER of instance: ENET
            pub const ALT1_enet_RX_ER: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_RXD of instance: LPUART8
            pub const ALT2_lpuart8_RX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_CM7_EVENTI of instance: CM7
            pub const ALT3_cm7_imxrt_RXEV: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: SAI1_TX_SYNC of instance: SAI1
            pub const ALT4_sai1_TX_SYNC: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO10 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO10: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_ER of instance: ENET_QOS
            pub const ALT8_enet_qos_RX_ER: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPUART1_RXD of instance: LPUART1
            pub const ALT9_lpuart1_RX: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO10 of instance: GPIO11
            pub const ALT10_gpio11_IO10: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA18 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA18: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: EMVSIM2_IO of instance: EMVSIM2
            pub const ALT1_EMVSIM2_TRXD: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART2_TXD of instance: LPUART2
            pub const ALT2_lpuart2_TX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: WDOG2_RESET_B_DEB of instance: WDOG2
            pub const ALT3_WDOG2_RESET_B_DEB: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT38 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT38: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO11 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO11: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPI2C3_SCL of instance: LPI2C3
            pub const ALT6_lpi2c3_SCL: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_RX_ER of instance: ENET_QOS
            pub const ALT8_enet_qos_RX_ER: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: SPDIF_IN of instance: SPDIF
            pub const ALT9_spdif_IN: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO11 of instance: GPIO11
            pub const ALT10_gpio11_IO11: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA19 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA19: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: EMVSIM2_CLK of instance: EMVSIM2
            pub const ALT1_EMVSIM2_CLK: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART2_RXD of instance: LPUART2
            pub const ALT2_lpuart2_RX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: WDOG1_RESET_B_DEB of instance: WDOG1
            pub const ALT3_WDOG1_RESET_B_DEB: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT39 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT39: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO12 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO12: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPI2C3_SDA of instance: LPI2C3
            pub const ALT6_lpi2c3_SDA: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_CRS of instance: ENET_QOS
            pub const ALT8_enet_qos_CRS: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: SPDIF_OUT of instance: SPDIF
            pub const ALT9_spdif_OUT: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO12 of instance: GPIO11
            pub const ALT10_gpio11_IO12: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA20 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA20: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: EMVSIM2_RST of instance: EMVSIM2
            pub const ALT1_EMVSIM2_RST_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXCAN1_TX of instance: FLEXCAN1
            pub const ALT2_can1_TX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART2_CTS_B of instance: LPUART2
            pub const ALT3_lpuart2_CTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: XBAR1_INOUT40 of instance: XBAR1
            pub const ALT4_XBAR1_INOUT40: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO13 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO13: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPI2C4_SCL of instance: LPI2C4
            pub const ALT6_lpi2c4_SCL: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_COL of instance: ENET_QOS
            pub const ALT8_enet_qos_COL: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI4_SCK of instance: LPSPI4
            pub const ALT9_lpspi4_SCK: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO13 of instance: GPIO11
            pub const ALT10_gpio11_IO13: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA21 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA21: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: EMVSIM2_SVEN of instance: EMVSIM2
            pub const ALT1_EMVSIM2_SVEN: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXCAN1_RX of instance: FLEXCAN1
            pub const ALT2_can1_RX: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART2_RTS_B of instance: LPUART2
            pub const ALT3_lpuart2_RTS_B: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: ENET_REF_CLK of instance: ENET
            pub const ALT4_enet_REF_CLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO14 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO14: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPI2C4_SDA of instance: LPI2C4
            pub const ALT6_lpi2c4_SDA: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_1588_EVENT0_OUT of instance: ENET_QOS
            pub const ALT8_enet_qos_1588_EVENT0_OUT: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI4_SIN of instance: LPSPI4
            pub const ALT9_lpspi4_SDI: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO14 of instance: GPIO11
            pub const ALT10_gpio11_IO14: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_14 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA22 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA22: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: EMVSIM2_PD of instance: EMVSIM2
            pub const ALT1_EMVSIM2_PD: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: WDOG2_B of instance: WDOG2
            pub const ALT2_WDOG2_B: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_EXT_DCIC1 of instance: VIDEO_MUX
            pub const ALT3_video_mux_EXT_DCIC1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: ENET_1G_REF_CLK of instance: ENET_1G
            pub const ALT4_enet_1g_REF_CLK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO15 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO15: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXCAN1_TX of instance: FLEXCAN1
            pub const ALT6_can1_TX: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_1588_EVENT0_IN of instance: ENET_QOS
            pub const ALT8_enet_qos_1588_EVENT0_IN: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI4_SOUT of instance: LPSPI4
            pub const ALT9_lpspi4_SDO: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO15 of instance: GPIO11
            pub const ALT10_gpio11_IO15: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_DISP_B2_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_DISP_B2_15 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: VIDEO_MUX_LCDIF_DATA23 of instance: VIDEO_MUX
            pub const ALT0_video_mux_LCDIF_DATA23: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: EMVSIM2_POWER_FAIL of instance: EMVSIM2
            pub const ALT1_EMVSIM2_POWER_FAIL: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: WDOG1_B of instance: WDOG1
            pub const ALT2_WDOG1_B: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: VIDEO_MUX_EXT_DCIC2 of instance: VIDEO_MUX
            pub const ALT3_video_mux_EXT_DCIC2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: PIT1_TRIGGER0 of instance: PIT1
            pub const ALT4_pit1_TRIGGER0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX5_IO16 of instance: GPIO_MUX5
            pub const ALT5_gpio_mux5_IO16: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXCAN1_RX of instance: FLEXCAN1
            pub const ALT6_can1_RX: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET_QOS_1588_EVENT0_AUX_IN of instance: ENET_QOS
            pub const ALT8_enet_qos_1588_EVENT0_AUX_IN: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: LPSPI4_PCS0 of instance: LPSPI4
            pub const ALT9_lpspi4_PCS0: u32 = 0b1001;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO11_IO16 of instance: GPIO11
            pub const ALT10_gpio11_IO16: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_DISP_B2_15
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_00 {

    /// PDRV Field
    pub mod PDRV {
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

            /// 0b0: high drive strength
            pub const PDRV_0_high_driver: u32 = 0b0;

            /// 0b1: normal drive strength
            pub const PDRV_1_normal_driver: u32 = 0b1;
        }
    }

    /// Pull Down Pull Up Field
    pub mod PULL {
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

            /// 0b00: Forbidden
            pub const PULL_0_Forbidden: u32 = 0b00;

            /// 0b01: Internal pullup resistor enabled
            pub const PULL_1_PU: u32 = 0b01;

            /// 0b10: Internal pulldown resistor enabled
            pub const PULL_2_PD: u32 = 0b10;

            /// 0b11: No Pull
            pub const PULL_3_No_Pull: u32 = 0b11;
        }
    }

    /// Open Drain Field
    pub mod ODE {
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

            /// 0b0: Disabled
            pub const ODE_0_Disabled: u32 = 0b0;

            /// 0b1: Enabled
            pub const ODE_1_Enabled: u32 = 0b1;
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

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_16 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_16 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_17 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_17 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_18 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_18 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_19 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_19 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_20 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_20 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_21 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_21 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_22 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_22 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_23 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_23 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_24 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_24 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_25 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_25 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_26 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_26 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_27 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_27 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_28 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_28 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_29 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_29 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_30 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_30 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_31 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_31 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_32 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_32 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_33 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_33 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_34 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_34 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_35 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_35 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_36 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_36 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_37 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_37 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_38 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_38 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_39 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_39 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_40 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_40 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B1_41 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B1_41 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_16 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_16 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_17 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_17 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_18 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_18 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_19 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_19 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_B2_20 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_B2_20 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_AD_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_00 {

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

            /// 0b0: normal drive strength
            pub const DSE_0_normal_driver: u32 = 0b0;

            /// 0b1: high drive strength
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

            /// 0b0: Pull Disable, Highz
            pub const PUE_0_Pull_Disable__Highz: u32 = 0b0;

            /// 0b1: Pull Enable
            pub const PUE_1_Pull_Enable: u32 = 0b1;
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

    /// Open Drain Field
    pub mod ODE {
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

            /// 0b0: Disabled
            pub const ODE_0_Disabled: u32 = 0b0;

            /// 0b1: Enabled
            pub const ODE_1_Enabled: u32 = 0b1;
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

/// SW_PAD_CTL_PAD_GPIO_AD_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_16 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_16 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_17 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_17 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_18 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_18 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_19 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_19 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_20 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_20 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_21 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_21 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_22 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_22 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_23 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_23 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_24 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_24 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_25 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_25 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_26 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_26 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_27 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_27 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_28 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_28 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_29 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_29 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_30 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_30 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_31 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_31 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_32 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_32 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_33 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_33 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_34 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_34 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_35 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_35 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B2_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B2_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B1_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B1_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PDRV;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_B1_00::PULL;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_DISP_B2_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_DISP_B2_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_00::SRE;
}

/// FLEXCAN1_RX_SELECT_INPUT DAISY Register
pub mod FLEXCAN1_RX_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_AD_07 for Mode: ALT1
            pub const SELECT_GPIO_AD_07_ALT1: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_DISP_B2_13 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B2_13_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B2_15 for Mode: ALT6
            pub const SELECT_GPIO_DISP_B2_15_ALT6: u32 = 0b10;
        }
    }
}

/// FLEXCAN2_RX_SELECT_INPUT DAISY Register
pub mod FLEXCAN2_RX_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_01 for Mode: ALT1
            pub const SELECT_GPIO_AD_01_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_31 for Mode: ALT2
            pub const SELECT_GPIO_AD_31_ALT2: u32 = 0b1;
        }
    }
}

/// CCM_ENET_QOS_REF_CLK_SELECT_INPUT DAISY Register
pub mod CCM_ENET_QOS_REF_CLK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT3
            pub const SELECT_GPIO_EMC_B2_20_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B2_07 for Mode: ALT9
            pub const SELECT_GPIO_SD_B2_07_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_11 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_11_ALT9: u32 = 0b10;
        }
    }
}

/// CCM_ENET_QOS_TX_CLK_SELECT_INPUT DAISY Register
pub mod CCM_ENET_QOS_TX_CLK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_DISP_B1_11 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B1_11_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_05 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B2_05_ALT8: u32 = 0b1;
        }
    }
}

/// ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register
pub mod ENET_IPG_CLK_RMII_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_AD_29 for Mode: ALT2
            pub const SELECT_GPIO_AD_29_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_DISP_B2_05 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B2_05_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B2_13 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B2_13_ALT4: u32 = 0b10;
        }
    }
}

/// ENET_MAC0_MDIO_SELECT_INPUT DAISY Register
pub mod ENET_MAC0_MDIO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B2_20_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_33 for Mode: ALT3
            pub const SELECT_GPIO_AD_33_ALT3: u32 = 0b1;
        }
    }
}

/// ENET_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register
pub mod ENET_MAC0_RXDATA_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_26 for Mode: ALT3
            pub const SELECT_GPIO_AD_26_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_06 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B2_06_ALT1: u32 = 0b1;
        }
    }
}

/// ENET_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register
pub mod ENET_MAC0_RXDATA_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_27 for Mode: ALT3
            pub const SELECT_GPIO_AD_27_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_07 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B2_07_ALT1: u32 = 0b1;
        }
    }
}

/// ENET_MAC0_RXEN_SELECT_INPUT DAISY Register
pub mod ENET_MAC0_RXEN_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_24 for Mode: ALT3
            pub const SELECT_GPIO_AD_24_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_08 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B2_08_ALT1: u32 = 0b1;
        }
    }
}

/// ENET_MAC0_RXERR_SELECT_INPUT DAISY Register
pub mod ENET_MAC0_RXERR_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_25 for Mode: ALT3
            pub const SELECT_GPIO_AD_25_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_09 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B2_09_ALT1: u32 = 0b1;
        }
    }
}

/// ENET_MAC0_TXCLK_SELECT_INPUT DAISY Register
pub mod ENET_MAC0_TXCLK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_29 for Mode: ALT3
            pub const SELECT_GPIO_AD_29_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_05 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B2_05_ALT1: u32 = 0b1;
        }
    }
}

/// ENET_1G_IPG_CLK_RMII_SELECT_INPUT DAISY Register
pub mod ENET_1G_IPG_CLK_RMII_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT3
            pub const SELECT_GPIO_EMC_B2_19_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B2_11 for Mode: ALT3
            pub const SELECT_GPIO_SD_B2_11_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_11 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B1_11_ALT2: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_DISP_B2_14 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B2_14_ALT4: u32 = 0b11;
        }
    }
}

/// ENET_1G_MAC0_MDIO_SELECT_INPUT DAISY Register
pub mod ENET_1G_MAC0_MDIO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT7
            pub const SELECT_GPIO_EMC_B1_41_ALT7: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B2_20_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_17 for Mode: ALT9
            pub const SELECT_GPIO_AD_17_ALT9: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_AD_33 for Mode: ALT9
            pub const SELECT_GPIO_AD_33_ALT9: u32 = 0b11;
        }
    }
}

/// ENET_1G_MAC0_RXCLK_SELECT_INPUT DAISY Register
pub mod ENET_1G_MAC0_RXCLK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT7
            pub const SELECT_GPIO_EMC_B2_05_ALT7: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B2_01 for Mode: ALT2
            pub const SELECT_GPIO_SD_B2_01_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_01 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B1_01_ALT1: u32 = 0b10;
        }
    }
}

/// ENET_1G_MAC0_RXDATA_0_SELECT_INPUT DAISY Register
pub mod ENET_1G_MAC0_RXDATA_0_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B2_15_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B2_02 for Mode: ALT2
            pub const SELECT_GPIO_SD_B2_02_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_02 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B1_02_ALT1: u32 = 0b10;
        }
    }
}

/// ENET_1G_MAC0_RXDATA_1_SELECT_INPUT DAISY Register
pub mod ENET_1G_MAC0_RXDATA_1_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B2_16_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B2_03 for Mode: ALT2
            pub const SELECT_GPIO_SD_B2_03_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_03 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B1_03_ALT1: u32 = 0b10;
        }
    }
}

/// ENET_1G_MAC0_RXDATA_2_SELECT_INPUT DAISY Register
pub mod ENET_1G_MAC0_RXDATA_2_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT7
            pub const SELECT_GPIO_EMC_B2_08_ALT7: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B2_04 for Mode: ALT2
            pub const SELECT_GPIO_SD_B2_04_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_04 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B1_04_ALT1: u32 = 0b10;
        }
    }
}

/// ENET_1G_MAC0_RXDATA_3_SELECT_INPUT DAISY Register
pub mod ENET_1G_MAC0_RXDATA_3_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_07 for Mode: ALT7
            pub const SELECT_GPIO_EMC_B2_07_ALT7: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B2_05 for Mode: ALT2
            pub const SELECT_GPIO_SD_B2_05_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_05 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B1_05_ALT1: u32 = 0b10;
        }
    }
}

/// ENET_1G_MAC0_RXEN_SELECT_INPUT DAISY Register
pub mod ENET_1G_MAC0_RXEN_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B2_17_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B2_00 for Mode: ALT2
            pub const SELECT_GPIO_SD_B2_00_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_00 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B1_00_ALT1: u32 = 0b10;
        }
    }
}

/// ENET_1G_MAC0_RXERR_SELECT_INPUT DAISY Register
pub mod ENET_1G_MAC0_RXERR_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B2_18_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_01 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B1_01_ALT2: u32 = 0b1;
        }
    }
}

/// ENET_1G_MAC0_TXCLK_SELECT_INPUT DAISY Register
pub mod ENET_1G_MAC0_TXCLK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_14 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B2_14_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B2_11 for Mode: ALT2
            pub const SELECT_GPIO_SD_B2_11_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_11 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B1_11_ALT1: u32 = 0b10;
        }
    }
}

/// ENET_QOS_GMII_MDI_I_SELECT_INPUT DAISY Register
pub mod ENET_QOS_GMII_MDI_I_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_20_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_27 for Mode: ALT9
            pub const SELECT_GPIO_AD_27_ALT9: u32 = 0b1;
        }
    }
}

/// ENET_QOS_PHY_RXD_I_SELECT_INPUT_0 DAISY Register
pub mod ENET_QOS_PHY_RXD_I_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_DISP_B1_02 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B1_02_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_06 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B2_06_ALT8: u32 = 0b1;
        }
    }
}

/// ENET_QOS_PHY_RXD_I_SELECT_INPUT_1 DAISY Register
pub mod ENET_QOS_PHY_RXD_I_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_DISP_B1_03 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B1_03_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_07 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B2_07_ALT8: u32 = 0b1;
        }
    }
}

/// ENET_QOS_PHY_RXDV_I_SELECT_INPUT DAISY Register
pub mod ENET_QOS_PHY_RXDV_I_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_DISP_B1_00 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B1_00_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_08 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B2_08_ALT8: u32 = 0b1;
        }
    }
}

/// ENET_QOS_PHY_RXER_I_SELECT_INPUT DAISY Register
pub mod ENET_QOS_PHY_RXER_I_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_DISP_B1_01 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_01_ALT9: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_DISP_B2_09 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B2_09_ALT8: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B2_10 for Mode: ALT8
            pub const SELECT_GPIO_DISP_B2_10_ALT8: u32 = 0b10;
        }
    }
}

/// FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM1_PWMA_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_23 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_23_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_00 for Mode: ALT4
            pub const SELECT_GPIO_AD_00_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM1_PWMA_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_25 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_25_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_02 for Mode: ALT4
            pub const SELECT_GPIO_AD_02_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM1_PWMA_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_27 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_27_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_04 for Mode: ALT4
            pub const SELECT_GPIO_AD_04_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM1_PWMB_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_24 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_24_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_01 for Mode: ALT4
            pub const SELECT_GPIO_AD_01_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM1_PWMB_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_26 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_26_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_03 for Mode: ALT4
            pub const SELECT_GPIO_AD_03_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM1_PWMB_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_28 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_28_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_05 for Mode: ALT4
            pub const SELECT_GPIO_AD_05_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMA_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM2_PWMA_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_06 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_06_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_24 for Mode: ALT4
            pub const SELECT_GPIO_AD_24_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMA_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM2_PWMA_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_08 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_08_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_26 for Mode: ALT4
            pub const SELECT_GPIO_AD_26_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMA_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM2_PWMA_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_10 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_10_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_28 for Mode: ALT4
            pub const SELECT_GPIO_AD_28_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMB_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM2_PWMB_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_07 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_07_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_25 for Mode: ALT4
            pub const SELECT_GPIO_AD_25_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMB_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM2_PWMB_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_09 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_09_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_27 for Mode: ALT4
            pub const SELECT_GPIO_AD_27_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMB_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM2_PWMB_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_11 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_11_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_29 for Mode: ALT4
            pub const SELECT_GPIO_AD_29_ALT4: u32 = 0b1;
        }
    }
}

/// FLEXPWM3_PWMA_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM3_PWMA_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_29_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT11
            pub const SELECT_GPIO_EMC_B2_00_ALT11: u32 = 0b1;
        }
    }
}

/// FLEXPWM3_PWMA_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM3_PWMA_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_31_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT11
            pub const SELECT_GPIO_EMC_B2_02_ALT11: u32 = 0b1;
        }
    }
}

/// FLEXPWM3_PWMA_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM3_PWMA_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_33_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_B2_04 for Mode: ALT11
            pub const SELECT_GPIO_EMC_B2_04_ALT11: u32 = 0b1;
        }
    }
}

/// FLEXPWM3_PWMA_SELECT_INPUT_3 DAISY Register
pub mod FLEXPWM3_PWMA_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_21 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_21_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT11
            pub const SELECT_GPIO_EMC_B2_06_ALT11: u32 = 0b1;
        }
    }
}

/// FLEXPWM3_PWMB_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM3_PWMB_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_30_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT11
            pub const SELECT_GPIO_EMC_B2_01_ALT11: u32 = 0b1;
        }
    }
}

/// FLEXPWM3_PWMB_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM3_PWMB_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_32_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT11
            pub const SELECT_GPIO_EMC_B2_03_ALT11: u32 = 0b1;
        }
    }
}

/// FLEXPWM3_PWMB_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM3_PWMB_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_34 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_34_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT11
            pub const SELECT_GPIO_EMC_B2_05_ALT11: u32 = 0b1;
        }
    }
}

/// FLEXPWM3_PWMB_SELECT_INPUT_3 DAISY Register
pub mod FLEXPWM3_PWMB_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B1_22 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B1_22_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_B2_07 for Mode: ALT11
            pub const SELECT_GPIO_EMC_B2_07_ALT11: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_DQS_FA_SELECT_INPUT DAISY Register
pub mod FLEXSPI1_I_DQS_FA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_18_ALT6: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_17 for Mode: ALT3
            pub const SELECT_GPIO_AD_17_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_SD_B2_05 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_05_ALT1: u32 = 0b10;
        }
    }
}

/// FLEXSPI1_I_IO_FA_SELECT_INPUT_0 DAISY Register
pub mod FLEXSPI1_I_IO_FA_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_20 for Mode: ALT3
            pub const SELECT_GPIO_AD_20_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_08 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_08_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_IO_FA_SELECT_INPUT_1 DAISY Register
pub mod FLEXSPI1_I_IO_FA_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_21 for Mode: ALT3
            pub const SELECT_GPIO_AD_21_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_09 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_09_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_IO_FA_SELECT_INPUT_2 DAISY Register
pub mod FLEXSPI1_I_IO_FA_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_22 for Mode: ALT3
            pub const SELECT_GPIO_AD_22_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_10 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_10_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_IO_FA_SELECT_INPUT_3 DAISY Register
pub mod FLEXSPI1_I_IO_FA_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_23 for Mode: ALT3
            pub const SELECT_GPIO_AD_23_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_11 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_11_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_IO_FB_SELECT_INPUT_0 DAISY Register
pub mod FLEXSPI1_I_IO_FB_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_15 for Mode: ALT3
            pub const SELECT_GPIO_AD_15_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_03 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_03_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_IO_FB_SELECT_INPUT_1 DAISY Register
pub mod FLEXSPI1_I_IO_FB_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_14 for Mode: ALT3
            pub const SELECT_GPIO_AD_14_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_02 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_02_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_IO_FB_SELECT_INPUT_2 DAISY Register
pub mod FLEXSPI1_I_IO_FB_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_13 for Mode: ALT3
            pub const SELECT_GPIO_AD_13_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_01 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_01_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_IO_FB_SELECT_INPUT_3 DAISY Register
pub mod FLEXSPI1_I_IO_FB_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_12 for Mode: ALT3
            pub const SELECT_GPIO_AD_12_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_00 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_00_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_SCK_FA_SELECT_INPUT DAISY Register
pub mod FLEXSPI1_I_SCK_FA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_19 for Mode: ALT3
            pub const SELECT_GPIO_AD_19_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_07 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_07_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI1_I_SCK_FB_SELECT_INPUT DAISY Register
pub mod FLEXSPI1_I_SCK_FB_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_16 for Mode: ALT3
            pub const SELECT_GPIO_AD_16_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_04 for Mode: ALT1
            pub const SELECT_GPIO_SD_B2_04_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI2_I_IO_FA_SELECT_INPUT_0 DAISY Register
pub mod FLEXSPI2_I_IO_FA_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_13 for Mode: ALT4
            pub const SELECT_GPIO_EMC_B2_13_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_02 for Mode: ALT6
            pub const SELECT_GPIO_SD_B1_02_ALT6: u32 = 0b1;
        }
    }
}

/// FLEXSPI2_I_IO_FA_SELECT_INPUT_1 DAISY Register
pub mod FLEXSPI2_I_IO_FA_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_14 for Mode: ALT4
            pub const SELECT_GPIO_EMC_B2_14_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT6
            pub const SELECT_GPIO_SD_B1_03_ALT6: u32 = 0b1;
        }
    }
}

/// FLEXSPI2_I_IO_FA_SELECT_INPUT_2 DAISY Register
pub mod FLEXSPI2_I_IO_FA_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT4
            pub const SELECT_GPIO_EMC_B2_15_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_04 for Mode: ALT6
            pub const SELECT_GPIO_SD_B1_04_ALT6: u32 = 0b1;
        }
    }
}

/// FLEXSPI2_I_IO_FA_SELECT_INPUT_3 DAISY Register
pub mod FLEXSPI2_I_IO_FA_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT4
            pub const SELECT_GPIO_EMC_B2_16_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_05 for Mode: ALT6
            pub const SELECT_GPIO_SD_B1_05_ALT6: u32 = 0b1;
        }
    }
}

/// FLEXSPI2_I_SCK_FA_SELECT_INPUT DAISY Register
pub mod FLEXSPI2_I_SCK_FA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT4
            pub const SELECT_GPIO_EMC_B2_10_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_01 for Mode: ALT6
            pub const SELECT_GPIO_SD_B1_01_ALT6: u32 = 0b1;
        }
    }
}

/// GPT3_CAPIN1_SELECT_INPUT DAISY Register
pub mod GPT3_CAPIN1_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B2_06_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_06 for Mode: ALT3
            pub const SELECT_GPIO_AD_06_ALT3: u32 = 0b1;
        }
    }
}

/// GPT3_CAPIN2_SELECT_INPUT DAISY Register
pub mod GPT3_CAPIN2_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_07 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B2_07_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_07 for Mode: ALT3
            pub const SELECT_GPIO_AD_07_ALT3: u32 = 0b1;
        }
    }
}

/// GPT3_CLKIN_SELECT_INPUT DAISY Register
pub mod GPT3_CLKIN_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B2_05_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_11 for Mode: ALT3
            pub const SELECT_GPIO_AD_11_ALT3: u32 = 0b1;
        }
    }
}

/// KPP_COL_SELECT_INPUT_6 DAISY Register
pub mod KPP_COL_SELECT_INPUT_6 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_23 for Mode: ALT6
            pub const SELECT_GPIO_AD_23_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT8
            pub const SELECT_GPIO_SD_B1_03_ALT8: u32 = 0b1;
        }
    }
}

/// KPP_COL_SELECT_INPUT_7 DAISY Register
pub mod KPP_COL_SELECT_INPUT_7 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_21 for Mode: ALT6
            pub const SELECT_GPIO_AD_21_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_01 for Mode: ALT8
            pub const SELECT_GPIO_SD_B1_01_ALT8: u32 = 0b1;
        }
    }
}

/// KPP_ROW_SELECT_INPUT_6 DAISY Register
pub mod KPP_ROW_SELECT_INPUT_6 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_22 for Mode: ALT6
            pub const SELECT_GPIO_AD_22_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_02 for Mode: ALT8
            pub const SELECT_GPIO_SD_B1_02_ALT8: u32 = 0b1;
        }
    }
}

/// KPP_ROW_SELECT_INPUT_7 DAISY Register
pub mod KPP_ROW_SELECT_INPUT_7 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_20 for Mode: ALT6
            pub const SELECT_GPIO_AD_20_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_00 for Mode: ALT8
            pub const SELECT_GPIO_SD_B1_00_ALT8: u32 = 0b1;
        }
    }
}

/// LPI2C1_LPI2C_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C1_LPI2C_SCL_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_08 for Mode: ALT1
            pub const SELECT_GPIO_AD_08_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_32 for Mode: ALT0
            pub const SELECT_GPIO_AD_32_ALT0: u32 = 0b1;
        }
    }
}

/// LPI2C1_LPI2C_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C1_LPI2C_SDA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_09 for Mode: ALT1
            pub const SELECT_GPIO_AD_09_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_33 for Mode: ALT0
            pub const SELECT_GPIO_AD_33_ALT0: u32 = 0b1;
        }
    }
}

/// LPI2C2_LPI2C_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C2_LPI2C_SCL_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_00_ALT9: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_18 for Mode: ALT9
            pub const SELECT_GPIO_AD_18_ALT9: u32 = 0b1;
        }
    }
}

/// LPI2C2_LPI2C_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C2_LPI2C_SDA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_01_ALT9: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_19 for Mode: ALT9
            pub const SELECT_GPIO_AD_19_ALT9: u32 = 0b1;
        }
    }
}

/// LPI2C3_LPI2C_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C3_LPI2C_SCL_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_DISP_B1_02 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B1_02_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_10 for Mode: ALT6
            pub const SELECT_GPIO_DISP_B2_10_ALT6: u32 = 0b1;
        }
    }
}

/// LPI2C3_LPI2C_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C3_LPI2C_SDA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_DISP_B1_03 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B1_03_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_11 for Mode: ALT6
            pub const SELECT_GPIO_DISP_B2_11_ALT6: u32 = 0b1;
        }
    }
}

/// LPI2C4_LPI2C_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C4_LPI2C_SCL_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_24 for Mode: ALT9
            pub const SELECT_GPIO_AD_24_ALT9: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_12 for Mode: ALT6
            pub const SELECT_GPIO_DISP_B2_12_ALT6: u32 = 0b1;
        }
    }
}

/// LPI2C4_LPI2C_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C4_LPI2C_SDA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_25 for Mode: ALT9
            pub const SELECT_GPIO_AD_25_ALT9: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_13 for Mode: ALT6
            pub const SELECT_GPIO_DISP_B2_13_ALT6: u32 = 0b1;
        }
    }
}

/// LPSPI1_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
pub mod LPSPI1_LPSPI_PCS_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_01_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_29 for Mode: ALT0
            pub const SELECT_GPIO_AD_29_ALT0: u32 = 0b1;
        }
    }
}

/// LPSPI1_LPSPI_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI1_LPSPI_SCK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_00_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_28 for Mode: ALT0
            pub const SELECT_GPIO_AD_28_ALT0: u32 = 0b1;
        }
    }
}

/// LPSPI1_LPSPI_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI1_LPSPI_SDI_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_03_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_31 for Mode: ALT0
            pub const SELECT_GPIO_AD_31_ALT0: u32 = 0b1;
        }
    }
}

/// LPSPI1_LPSPI_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI1_LPSPI_SDO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_02_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_30 for Mode: ALT0
            pub const SELECT_GPIO_AD_30_ALT0: u32 = 0b1;
        }
    }
}

/// LPSPI2_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
pub mod LPSPI2_LPSPI_PCS_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_25 for Mode: ALT1
            pub const SELECT_GPIO_AD_25_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_08 for Mode: ALT6
            pub const SELECT_GPIO_SD_B2_08_ALT6: u32 = 0b1;
        }
    }
}

/// LPSPI2_LPSPI_PCS_SELECT_INPUT_1 DAISY Register
pub mod LPSPI2_LPSPI_PCS_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_21 for Mode: ALT2
            pub const SELECT_GPIO_AD_21_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_11 for Mode: ALT6
            pub const SELECT_GPIO_SD_B2_11_ALT6: u32 = 0b1;
        }
    }
}

/// LPSPI2_LPSPI_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI2_LPSPI_SCK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_24 for Mode: ALT1
            pub const SELECT_GPIO_AD_24_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_07 for Mode: ALT6
            pub const SELECT_GPIO_SD_B2_07_ALT6: u32 = 0b1;
        }
    }
}

/// LPSPI2_LPSPI_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI2_LPSPI_SDI_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_27 for Mode: ALT1
            pub const SELECT_GPIO_AD_27_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_10 for Mode: ALT6
            pub const SELECT_GPIO_SD_B2_10_ALT6: u32 = 0b1;
        }
    }
}

/// LPSPI2_LPSPI_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI2_LPSPI_SDO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_26 for Mode: ALT1
            pub const SELECT_GPIO_AD_26_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B2_09 for Mode: ALT6
            pub const SELECT_GPIO_SD_B2_09_ALT6: u32 = 0b1;
        }
    }
}

/// LPSPI3_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
pub mod LPSPI3_LPSPI_PCS_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_05_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_07 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_07_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI3_LPSPI_PCS_SELECT_INPUT_1 DAISY Register
pub mod LPSPI3_LPSPI_PCS_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_08_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_08 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_08_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI3_LPSPI_PCS_SELECT_INPUT_2 DAISY Register
pub mod LPSPI3_LPSPI_PCS_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_09_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_09 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_09_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI3_LPSPI_PCS_SELECT_INPUT_3 DAISY Register
pub mod LPSPI3_LPSPI_PCS_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_10_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_10 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_10_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI3_LPSPI_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI3_LPSPI_SCK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_04 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_04_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_04 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_04_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI3_LPSPI_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI3_LPSPI_SDI_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_07 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_07_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_05 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_05_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI3_LPSPI_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI3_LPSPI_SDO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_06_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_06 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_06_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI4_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
pub mod LPSPI4_LPSPI_PCS_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_SD_B2_01 for Mode: ALT4
            pub const SELECT_GPIO_SD_B2_01_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_15 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B2_15_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI4_LPSPI_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI4_LPSPI_SCK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_SD_B2_00 for Mode: ALT4
            pub const SELECT_GPIO_SD_B2_00_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_12 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B2_12_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI4_LPSPI_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI4_LPSPI_SDI_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_SD_B2_03 for Mode: ALT4
            pub const SELECT_GPIO_SD_B2_03_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_13 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B2_13_ALT9: u32 = 0b1;
        }
    }
}

/// LPSPI4_LPSPI_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI4_LPSPI_SDO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_SD_B2_02 for Mode: ALT4
            pub const SELECT_GPIO_SD_B2_02_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_14 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B2_14_ALT9: u32 = 0b1;
        }
    }
}

/// LPUART1_LPUART_RXD_SELECT_INPUT DAISY Register
pub mod LPUART1_LPUART_RXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_AD_25 for Mode: ALT0
            pub const SELECT_GPIO_AD_25_ALT0: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_DISP_B1_03 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_03_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B2_09 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B2_09_ALT9: u32 = 0b10;
        }
    }
}

/// LPUART1_LPUART_TXD_SELECT_INPUT DAISY Register
pub mod LPUART1_LPUART_TXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_AD_24 for Mode: ALT0
            pub const SELECT_GPIO_AD_24_ALT0: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_DISP_B1_02 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B1_02_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B2_08 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B2_08_ALT9: u32 = 0b10;
        }
    }
}

/// LPUART10_LPUART_RXD_SELECT_INPUT DAISY Register
pub mod LPUART10_LPUART_RXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_16 for Mode: ALT1
            pub const SELECT_GPIO_AD_16_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_33 for Mode: ALT8
            pub const SELECT_GPIO_AD_33_ALT8: u32 = 0b1;
        }
    }
}

/// LPUART10_LPUART_TXD_SELECT_INPUT DAISY Register
pub mod LPUART10_LPUART_TXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_15 for Mode: ALT1
            pub const SELECT_GPIO_AD_15_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_32 for Mode: ALT8
            pub const SELECT_GPIO_AD_32_ALT8: u32 = 0b1;
        }
    }
}

/// LPUART7_LPUART_RXD_SELECT_INPUT DAISY Register
pub mod LPUART7_LPUART_RXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_01 for Mode: ALT6
            pub const SELECT_GPIO_AD_01_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_07 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B2_07_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART7_LPUART_TXD_SELECT_INPUT DAISY Register
pub mod LPUART7_LPUART_TXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_00 for Mode: ALT6
            pub const SELECT_GPIO_AD_00_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_06 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B2_06_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART8_LPUART_RXD_SELECT_INPUT DAISY Register
pub mod LPUART8_LPUART_RXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_03 for Mode: ALT6
            pub const SELECT_GPIO_AD_03_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_09 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B2_09_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART8_LPUART_TXD_SELECT_INPUT DAISY Register
pub mod LPUART8_LPUART_TXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_02 for Mode: ALT6
            pub const SELECT_GPIO_AD_02_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_08 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B2_08_ALT2: u32 = 0b1;
        }
    }
}

/// QTIMER1_TMR0_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER1_TMR0_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B1_17 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B1_17_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_09_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_00 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_00_ALT3: u32 = 0b10;
        }
    }
}

/// QTIMER1_TMR1_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER1_TMR1_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B1_38 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B1_38_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_10_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_01 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_01_ALT3: u32 = 0b10;
        }
    }
}

/// QTIMER1_TMR2_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER1_TMR2_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_11_ALT9: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_02 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_02_ALT3: u32 = 0b1;
        }
    }
}

/// QTIMER2_TMR0_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER2_TMR0_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B1_18 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B1_18_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_B2_13 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_13_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_03 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_03_ALT3: u32 = 0b10;
        }
    }
}

/// QTIMER2_TMR1_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER2_TMR1_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B1_39 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B1_39_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_B2_14 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_14_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_04 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_04_ALT3: u32 = 0b10;
        }
    }
}

/// QTIMER2_TMR2_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER2_TMR2_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_15_ALT9: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_05 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_05_ALT3: u32 = 0b1;
        }
    }
}

/// QTIMER3_TMR0_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER3_TMR0_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B1_19 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B1_19_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_17_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_06 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_06_ALT3: u32 = 0b10;
        }
    }
}

/// QTIMER3_TMR1_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER3_TMR1_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B2_00_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_18_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_07 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_07_ALT3: u32 = 0b10;
        }
    }
}

/// QTIMER3_TMR2_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER3_TMR2_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT9
            pub const SELECT_GPIO_EMC_B2_19_ALT9: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_08 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_08_ALT3: u32 = 0b1;
        }
    }
}

/// QTIMER4_TMR0_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER4_TMR0_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B1_20 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B1_20_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_04 for Mode: ALT9
            pub const SELECT_GPIO_AD_04_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_09 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_09_ALT3: u32 = 0b10;
        }
    }
}

/// QTIMER4_TMR1_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER4_TMR1_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT2
            pub const SELECT_GPIO_EMC_B2_01_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_05 for Mode: ALT9
            pub const SELECT_GPIO_AD_05_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B1_10 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_10_ALT3: u32 = 0b10;
        }
    }
}

/// QTIMER4_TMR2_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER4_TMR2_INPUT_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_06 for Mode: ALT9
            pub const SELECT_GPIO_AD_06_ALT9: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_11 for Mode: ALT3
            pub const SELECT_GPIO_DISP_B1_11_ALT3: u32 = 0b1;
        }
    }
}

/// SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT DAISY Register
pub mod SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_17 for Mode: ALT0
            pub const SELECT_GPIO_AD_17_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_03 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B2_03_ALT4: u32 = 0b1;
        }
    }
}

/// SAI1_SAI_RXBCLK_SELECT_INPUT DAISY Register
pub mod SAI1_SAI_RXBCLK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_19 for Mode: ALT0
            pub const SELECT_GPIO_AD_19_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_05 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B2_05_ALT4: u32 = 0b1;
        }
    }
}

/// SAI1_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
pub mod SAI1_SAI_RXDATA_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_20 for Mode: ALT0
            pub const SELECT_GPIO_AD_20_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_06 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B2_06_ALT4: u32 = 0b1;
        }
    }
}

/// SAI1_SAI_RXSYNC_SELECT_INPUT DAISY Register
pub mod SAI1_SAI_RXSYNC_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_18 for Mode: ALT0
            pub const SELECT_GPIO_AD_18_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_04 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B2_04_ALT4: u32 = 0b1;
        }
    }
}

/// SAI1_SAI_TXBCLK_SELECT_INPUT DAISY Register
pub mod SAI1_SAI_TXBCLK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_22 for Mode: ALT0
            pub const SELECT_GPIO_AD_22_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_08 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B2_08_ALT4: u32 = 0b1;
        }
    }
}

/// SAI1_SAI_TXSYNC_SELECT_INPUT DAISY Register
pub mod SAI1_SAI_TXSYNC_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_23 for Mode: ALT0
            pub const SELECT_GPIO_AD_23_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_09 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B2_09_ALT4: u32 = 0b1;
        }
    }
}

/// EMVSIM1_SIO_SELECT_INPUT DAISY Register
pub mod EMVSIM1_SIO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_11_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_00 for Mode: ALT0
            pub const SELECT_GPIO_AD_00_ALT0: u32 = 0b1;
        }
    }
}

/// EMVSIM1_IPP_SIMPD_SELECT_INPUT DAISY Register
pub mod EMVSIM1_IPP_SIMPD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_15_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_04 for Mode: ALT0
            pub const SELECT_GPIO_AD_04_ALT0: u32 = 0b1;
        }
    }
}

/// EMVSIM1_POWER_FAIL_SELECT_INPUT DAISY Register
pub mod EMVSIM1_POWER_FAIL_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT8
            pub const SELECT_GPIO_EMC_B2_16_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_05 for Mode: ALT0
            pub const SELECT_GPIO_AD_05_ALT0: u32 = 0b1;
        }
    }
}

/// EMVSIM2_SIO_SELECT_INPUT DAISY Register
pub mod EMVSIM2_SIO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_06 for Mode: ALT2
            pub const SELECT_GPIO_AD_06_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_10 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B2_10_ALT1: u32 = 0b1;
        }
    }
}

/// EMVSIM2_IPP_SIMPD_SELECT_INPUT DAISY Register
pub mod EMVSIM2_IPP_SIMPD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_10 for Mode: ALT2
            pub const SELECT_GPIO_AD_10_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_14 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B2_14_ALT1: u32 = 0b1;
        }
    }
}

/// EMVSIM2_POWER_FAIL_SELECT_INPUT DAISY Register
pub mod EMVSIM2_POWER_FAIL_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_11 for Mode: ALT2
            pub const SELECT_GPIO_AD_11_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B2_15 for Mode: ALT1
            pub const SELECT_GPIO_DISP_B2_15_ALT1: u32 = 0b1;
        }
    }
}

/// SPDIF_SPDIF_IN1_SELECT_INPUT DAISY Register
pub mod SPDIF_SPDIF_IN1_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b00: Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B2_11_ALT1: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_15 for Mode: ALT0
            pub const SELECT_GPIO_AD_15_ALT0: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_DISP_B2_10 for Mode: ALT9
            pub const SELECT_GPIO_DISP_B2_10_ALT9: u32 = 0b10;
        }
    }
}

/// USB_OTG2_OC_SELECT_INPUT DAISY Register
pub mod USB_OTG2_OC_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_06 for Mode: ALT0
            pub const SELECT_GPIO_AD_06_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_30 for Mode: ALT1
            pub const SELECT_GPIO_AD_30_ALT1: u32 = 0b1;
        }
    }
}

/// USB_OTG_OC_SELECT_INPUT DAISY Register
pub mod USB_OTG_OC_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_11 for Mode: ALT0
            pub const SELECT_GPIO_AD_11_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_35 for Mode: ALT1
            pub const SELECT_GPIO_AD_35_ALT1: u32 = 0b1;
        }
    }
}

/// USBPHY1_USB_ID_SELECT_INPUT DAISY Register
pub mod USBPHY1_USB_ID_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_09 for Mode: ALT0
            pub const SELECT_GPIO_AD_09_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_33 for Mode: ALT1
            pub const SELECT_GPIO_AD_33_ALT1: u32 = 0b1;
        }
    }
}

/// USBPHY2_USB_ID_SELECT_INPUT DAISY Register
pub mod USBPHY2_USB_ID_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_08 for Mode: ALT0
            pub const SELECT_GPIO_AD_08_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_32 for Mode: ALT1
            pub const SELECT_GPIO_AD_32_ALT1: u32 = 0b1;
        }
    }
}

/// USDHC1_IPP_CARD_DET_SELECT_INPUT DAISY Register
pub mod USDHC1_IPP_CARD_DET_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_32 for Mode: ALT4
            pub const SELECT_GPIO_AD_32_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_08 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B1_08_ALT2: u32 = 0b1;
        }
    }
}

/// USDHC1_IPP_WP_ON_SELECT_INPUT DAISY Register
pub mod USDHC1_IPP_WP_ON_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_AD_33 for Mode: ALT4
            pub const SELECT_GPIO_AD_33_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_09 for Mode: ALT2
            pub const SELECT_GPIO_DISP_B1_09_ALT2: u32 = 0b1;
        }
    }
}

/// USDHC2_IPP_CARD_DET_SELECT_INPUT DAISY Register
pub mod USDHC2_IPP_CARD_DET_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B2_01_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_26 for Mode: ALT11
            pub const SELECT_GPIO_AD_26_ALT11: u32 = 0b1;
        }
    }
}

/// USDHC2_IPP_WP_ON_SELECT_INPUT DAISY Register
pub mod USDHC2_IPP_WP_ON_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT1
            pub const SELECT_GPIO_EMC_B2_02_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_27 for Mode: ALT11
            pub const SELECT_GPIO_AD_27_ALT11: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_20 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_20 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_00_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_00 for Mode: ALT2
            pub const SELECT_GPIO_SD_B1_00_ALT2: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_21 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_21 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_01_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2
            pub const SELECT_GPIO_SD_B1_01_ALT2: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_22 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_22 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_02_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_02 for Mode: ALT2
            pub const SELECT_GPIO_SD_B1_02_ALT2: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_23 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_23 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_03_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT2
            pub const SELECT_GPIO_SD_B1_03_ALT2: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_24 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_24 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_04 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_04_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_04 for Mode: ALT2
            pub const SELECT_GPIO_SD_B1_04_ALT2: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_25 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_25 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_05_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_05 for Mode: ALT2
            pub const SELECT_GPIO_SD_B1_05_ALT2: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_26 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_26 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_06_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_00 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_00_ALT4: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_27 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_27 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_07 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_07_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_01 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_01_ALT4: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_28 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_28 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_08_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_02 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_02_ALT4: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_29 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_29 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_09_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_03 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_03_ALT4: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_30 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_30 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_10_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_04 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_04_ALT4: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_31 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_31 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_11_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_05 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_05_ALT4: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_32 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_32 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_12_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_06 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_06_ALT4: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_33 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_33 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_13 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_13_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_07 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_07_ALT4: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_34 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_34 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_14 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_14_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_08 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_08_ALT4: u32 = 0b1;
        }
    }
}

/// XBAR1_IN_SELECT_INPUT_35 DAISY Register
pub mod XBAR1_IN_SELECT_INPUT_35 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b0: Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT6
            pub const SELECT_GPIO_EMC_B2_15_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_DISP_B1_09 for Mode: ALT4
            pub const SELECT_GPIO_DISP_B1_09_ALT4: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 4],

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_15: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_16 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_16: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_17 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_17: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_18 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_18: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_19 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_19: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_20 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_20: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_21 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_21: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_22 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_22: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_23 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_23: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_24 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_24: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_25 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_25: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_26 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_26: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_27 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_27: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_28 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_28: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_29 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_29: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_30 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_30: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_31 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_31: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_32 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_32: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_33 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_33: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_34 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_34: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_35 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_35: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_36 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_36: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_37 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_37: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_38 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_38: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_39 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_39: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_40 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_40: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B1_41 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_41: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_15: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_16 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_16: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_17 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_17: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_18 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_18: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_19 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_19: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_B2_20 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_20: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_15: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_16 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_16: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_17 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_17: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_18 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_18: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_19 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_19: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_20 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_20: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_21 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_21: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_22 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_22: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_23 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_23: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_24 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_24: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_25 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_25: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_26 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_26: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_27 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_27: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_28 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_28: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_29 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_29: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_30 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_30: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_31 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_31: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_32 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_32: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_33 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_33: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_34 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_34: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_35 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_35: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B2_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B1_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_DISP_B2_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_15: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_15: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_16 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_16: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_17 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_17: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_18 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_18: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_19 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_19: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_20 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_20: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_21 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_21: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_22 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_22: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_23 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_23: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_24 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_24: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_25 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_25: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_26 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_26: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_27 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_27: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_28 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_28: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_29 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_29: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_30 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_30: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_31 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_31: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_32 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_32: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_33 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_33: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_34 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_34: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_35 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_35: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_36 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_36: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_37 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_37: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_38 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_38: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_39 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_39: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_40 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_40: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B1_41 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_41: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_15: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_16 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_16: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_17 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_17: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_18 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_18: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_19 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_19: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_B2_20 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_20: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_15: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_16 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_16: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_17 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_17: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_18 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_18: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_19 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_19: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_20 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_20: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_21 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_21: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_22 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_22: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_23 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_23: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_24 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_24: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_25 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_25: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_26 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_26: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_27 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_27: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_28 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_28: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_29 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_29: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_30 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_30: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_31 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_31: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_32 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_32: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_33 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_33: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_34 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_34: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_35 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_35: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B2_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B1_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_DISP_B2_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_15: RWRegister<u32>,

    /// FLEXCAN1_RX_SELECT_INPUT DAISY Register
    pub FLEXCAN1_RX_SELECT_INPUT: RWRegister<u32>,

    /// FLEXCAN2_RX_SELECT_INPUT DAISY Register
    pub FLEXCAN2_RX_SELECT_INPUT: RWRegister<u32>,

    /// CCM_ENET_QOS_REF_CLK_SELECT_INPUT DAISY Register
    pub CCM_ENET_QOS_REF_CLK_SELECT_INPUT: RWRegister<u32>,

    /// CCM_ENET_QOS_TX_CLK_SELECT_INPUT DAISY Register
    pub CCM_ENET_QOS_TX_CLK_SELECT_INPUT: RWRegister<u32>,

    /// ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register
    pub ENET_IPG_CLK_RMII_SELECT_INPUT: RWRegister<u32>,

    /// ENET_MAC0_MDIO_SELECT_INPUT DAISY Register
    pub ENET_MAC0_MDIO_SELECT_INPUT: RWRegister<u32>,

    /// ENET_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register
    pub ENET_MAC0_RXDATA_SELECT_INPUT_0: RWRegister<u32>,

    /// ENET_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register
    pub ENET_MAC0_RXDATA_SELECT_INPUT_1: RWRegister<u32>,

    /// ENET_MAC0_RXEN_SELECT_INPUT DAISY Register
    pub ENET_MAC0_RXEN_SELECT_INPUT: RWRegister<u32>,

    /// ENET_MAC0_RXERR_SELECT_INPUT DAISY Register
    pub ENET_MAC0_RXERR_SELECT_INPUT: RWRegister<u32>,

    /// ENET_MAC0_TXCLK_SELECT_INPUT DAISY Register
    pub ENET_MAC0_TXCLK_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_IPG_CLK_RMII_SELECT_INPUT DAISY Register
    pub ENET_1G_IPG_CLK_RMII_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_MAC0_MDIO_SELECT_INPUT DAISY Register
    pub ENET_1G_MAC0_MDIO_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_MAC0_RXCLK_SELECT_INPUT DAISY Register
    pub ENET_1G_MAC0_RXCLK_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_MAC0_RXDATA_0_SELECT_INPUT DAISY Register
    pub ENET_1G_MAC0_RXDATA_0_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_MAC0_RXDATA_1_SELECT_INPUT DAISY Register
    pub ENET_1G_MAC0_RXDATA_1_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_MAC0_RXDATA_2_SELECT_INPUT DAISY Register
    pub ENET_1G_MAC0_RXDATA_2_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_MAC0_RXDATA_3_SELECT_INPUT DAISY Register
    pub ENET_1G_MAC0_RXDATA_3_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_MAC0_RXEN_SELECT_INPUT DAISY Register
    pub ENET_1G_MAC0_RXEN_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_MAC0_RXERR_SELECT_INPUT DAISY Register
    pub ENET_1G_MAC0_RXERR_SELECT_INPUT: RWRegister<u32>,

    /// ENET_1G_MAC0_TXCLK_SELECT_INPUT DAISY Register
    pub ENET_1G_MAC0_TXCLK_SELECT_INPUT: RWRegister<u32>,

    /// ENET_QOS_GMII_MDI_I_SELECT_INPUT DAISY Register
    pub ENET_QOS_GMII_MDI_I_SELECT_INPUT: RWRegister<u32>,

    /// ENET_QOS_PHY_RXD_I_SELECT_INPUT_0 DAISY Register
    pub ENET_QOS_PHY_RXD_I_SELECT_INPUT_0: RWRegister<u32>,

    /// ENET_QOS_PHY_RXD_I_SELECT_INPUT_1 DAISY Register
    pub ENET_QOS_PHY_RXD_I_SELECT_INPUT_1: RWRegister<u32>,

    /// ENET_QOS_PHY_RXDV_I_SELECT_INPUT DAISY Register
    pub ENET_QOS_PHY_RXDV_I_SELECT_INPUT: RWRegister<u32>,

    /// ENET_QOS_PHY_RXER_I_SELECT_INPUT DAISY Register
    pub ENET_QOS_PHY_RXER_I_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM1_PWMA_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM1_PWMA_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM1_PWMA_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM1_PWMB_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM1_PWMB_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM1_PWMB_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM2_PWMA_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM2_PWMA_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM2_PWMA_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM2_PWMA_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM2_PWMA_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM2_PWMA_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM2_PWMB_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM2_PWMB_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM2_PWMB_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM2_PWMB_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM2_PWMB_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM2_PWMB_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM3_PWMA_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM3_PWMA_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM3_PWMA_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM3_PWMA_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM3_PWMA_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM3_PWMA_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM3_PWMA_SELECT_INPUT_3 DAISY Register
    pub FLEXPWM3_PWMA_SELECT_INPUT_3: RWRegister<u32>,

    /// FLEXPWM3_PWMB_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM3_PWMB_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM3_PWMB_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM3_PWMB_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM3_PWMB_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM3_PWMB_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM3_PWMB_SELECT_INPUT_3 DAISY Register
    pub FLEXPWM3_PWMB_SELECT_INPUT_3: RWRegister<u32>,

    /// FLEXSPI1_I_DQS_FA_SELECT_INPUT DAISY Register
    pub FLEXSPI1_I_DQS_FA_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPI1_I_IO_FA_SELECT_INPUT_0 DAISY Register
    pub FLEXSPI1_I_IO_FA_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXSPI1_I_IO_FA_SELECT_INPUT_1 DAISY Register
    pub FLEXSPI1_I_IO_FA_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXSPI1_I_IO_FA_SELECT_INPUT_2 DAISY Register
    pub FLEXSPI1_I_IO_FA_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXSPI1_I_IO_FA_SELECT_INPUT_3 DAISY Register
    pub FLEXSPI1_I_IO_FA_SELECT_INPUT_3: RWRegister<u32>,

    /// FLEXSPI1_I_IO_FB_SELECT_INPUT_0 DAISY Register
    pub FLEXSPI1_I_IO_FB_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXSPI1_I_IO_FB_SELECT_INPUT_1 DAISY Register
    pub FLEXSPI1_I_IO_FB_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXSPI1_I_IO_FB_SELECT_INPUT_2 DAISY Register
    pub FLEXSPI1_I_IO_FB_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXSPI1_I_IO_FB_SELECT_INPUT_3 DAISY Register
    pub FLEXSPI1_I_IO_FB_SELECT_INPUT_3: RWRegister<u32>,

    /// FLEXSPI1_I_SCK_FA_SELECT_INPUT DAISY Register
    pub FLEXSPI1_I_SCK_FA_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPI1_I_SCK_FB_SELECT_INPUT DAISY Register
    pub FLEXSPI1_I_SCK_FB_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPI2_I_IO_FA_SELECT_INPUT_0 DAISY Register
    pub FLEXSPI2_I_IO_FA_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXSPI2_I_IO_FA_SELECT_INPUT_1 DAISY Register
    pub FLEXSPI2_I_IO_FA_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXSPI2_I_IO_FA_SELECT_INPUT_2 DAISY Register
    pub FLEXSPI2_I_IO_FA_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXSPI2_I_IO_FA_SELECT_INPUT_3 DAISY Register
    pub FLEXSPI2_I_IO_FA_SELECT_INPUT_3: RWRegister<u32>,

    /// FLEXSPI2_I_SCK_FA_SELECT_INPUT DAISY Register
    pub FLEXSPI2_I_SCK_FA_SELECT_INPUT: RWRegister<u32>,

    /// GPT3_CAPIN1_SELECT_INPUT DAISY Register
    pub GPT3_CAPIN1_SELECT_INPUT: RWRegister<u32>,

    /// GPT3_CAPIN2_SELECT_INPUT DAISY Register
    pub GPT3_CAPIN2_SELECT_INPUT: RWRegister<u32>,

    /// GPT3_CLKIN_SELECT_INPUT DAISY Register
    pub GPT3_CLKIN_SELECT_INPUT: RWRegister<u32>,

    /// KPP_COL_SELECT_INPUT_6 DAISY Register
    pub KPP_COL_SELECT_INPUT_6: RWRegister<u32>,

    /// KPP_COL_SELECT_INPUT_7 DAISY Register
    pub KPP_COL_SELECT_INPUT_7: RWRegister<u32>,

    /// KPP_ROW_SELECT_INPUT_6 DAISY Register
    pub KPP_ROW_SELECT_INPUT_6: RWRegister<u32>,

    /// KPP_ROW_SELECT_INPUT_7 DAISY Register
    pub KPP_ROW_SELECT_INPUT_7: RWRegister<u32>,

    /// LPI2C1_LPI2C_SCL_SELECT_INPUT DAISY Register
    pub LPI2C1_LPI2C_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C1_LPI2C_SDA_SELECT_INPUT DAISY Register
    pub LPI2C1_LPI2C_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C2_LPI2C_SCL_SELECT_INPUT DAISY Register
    pub LPI2C2_LPI2C_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C2_LPI2C_SDA_SELECT_INPUT DAISY Register
    pub LPI2C2_LPI2C_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C3_LPI2C_SCL_SELECT_INPUT DAISY Register
    pub LPI2C3_LPI2C_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C3_LPI2C_SDA_SELECT_INPUT DAISY Register
    pub LPI2C3_LPI2C_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C4_LPI2C_SCL_SELECT_INPUT DAISY Register
    pub LPI2C4_LPI2C_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C4_LPI2C_SDA_SELECT_INPUT DAISY Register
    pub LPI2C4_LPI2C_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
    pub LPSPI1_LPSPI_PCS_SELECT_INPUT_0: RWRegister<u32>,

    /// LPSPI1_LPSPI_SCK_SELECT_INPUT DAISY Register
    pub LPSPI1_LPSPI_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_LPSPI_SDI_SELECT_INPUT DAISY Register
    pub LPSPI1_LPSPI_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_LPSPI_SDO_SELECT_INPUT DAISY Register
    pub LPSPI1_LPSPI_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
    pub LPSPI2_LPSPI_PCS_SELECT_INPUT_0: RWRegister<u32>,

    /// LPSPI2_LPSPI_PCS_SELECT_INPUT_1 DAISY Register
    pub LPSPI2_LPSPI_PCS_SELECT_INPUT_1: RWRegister<u32>,

    /// LPSPI2_LPSPI_SCK_SELECT_INPUT DAISY Register
    pub LPSPI2_LPSPI_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_LPSPI_SDI_SELECT_INPUT DAISY Register
    pub LPSPI2_LPSPI_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_LPSPI_SDO_SELECT_INPUT DAISY Register
    pub LPSPI2_LPSPI_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI3_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
    pub LPSPI3_LPSPI_PCS_SELECT_INPUT_0: RWRegister<u32>,

    /// LPSPI3_LPSPI_PCS_SELECT_INPUT_1 DAISY Register
    pub LPSPI3_LPSPI_PCS_SELECT_INPUT_1: RWRegister<u32>,

    /// LPSPI3_LPSPI_PCS_SELECT_INPUT_2 DAISY Register
    pub LPSPI3_LPSPI_PCS_SELECT_INPUT_2: RWRegister<u32>,

    /// LPSPI3_LPSPI_PCS_SELECT_INPUT_3 DAISY Register
    pub LPSPI3_LPSPI_PCS_SELECT_INPUT_3: RWRegister<u32>,

    /// LPSPI3_LPSPI_SCK_SELECT_INPUT DAISY Register
    pub LPSPI3_LPSPI_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI3_LPSPI_SDI_SELECT_INPUT DAISY Register
    pub LPSPI3_LPSPI_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI3_LPSPI_SDO_SELECT_INPUT DAISY Register
    pub LPSPI3_LPSPI_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI4_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
    pub LPSPI4_LPSPI_PCS_SELECT_INPUT_0: RWRegister<u32>,

    /// LPSPI4_LPSPI_SCK_SELECT_INPUT DAISY Register
    pub LPSPI4_LPSPI_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI4_LPSPI_SDI_SELECT_INPUT DAISY Register
    pub LPSPI4_LPSPI_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI4_LPSPI_SDO_SELECT_INPUT DAISY Register
    pub LPSPI4_LPSPI_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPUART1_LPUART_RXD_SELECT_INPUT DAISY Register
    pub LPUART1_LPUART_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART1_LPUART_TXD_SELECT_INPUT DAISY Register
    pub LPUART1_LPUART_TXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART10_LPUART_RXD_SELECT_INPUT DAISY Register
    pub LPUART10_LPUART_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART10_LPUART_TXD_SELECT_INPUT DAISY Register
    pub LPUART10_LPUART_TXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART7_LPUART_RXD_SELECT_INPUT DAISY Register
    pub LPUART7_LPUART_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART7_LPUART_TXD_SELECT_INPUT DAISY Register
    pub LPUART7_LPUART_TXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART8_LPUART_RXD_SELECT_INPUT DAISY Register
    pub LPUART8_LPUART_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART8_LPUART_TXD_SELECT_INPUT DAISY Register
    pub LPUART8_LPUART_TXD_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER1_TMR0_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER1_TMR0_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER1_TMR1_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER1_TMR1_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER1_TMR2_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER1_TMR2_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER2_TMR0_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER2_TMR0_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER2_TMR1_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER2_TMR1_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER2_TMR2_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER2_TMR2_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER3_TMR0_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER3_TMR0_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER3_TMR1_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER3_TMR1_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER3_TMR2_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER3_TMR2_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER4_TMR0_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER4_TMR0_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER4_TMR1_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER4_TMR1_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER4_TMR2_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER4_TMR2_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT DAISY Register
    pub SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_SAI_RXBCLK_SELECT_INPUT DAISY Register
    pub SAI1_SAI_RXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
    pub SAI1_SAI_RXDATA_SELECT_INPUT_0: RWRegister<u32>,

    /// SAI1_SAI_RXSYNC_SELECT_INPUT DAISY Register
    pub SAI1_SAI_RXSYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_SAI_TXBCLK_SELECT_INPUT DAISY Register
    pub SAI1_SAI_TXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_SAI_TXSYNC_SELECT_INPUT DAISY Register
    pub SAI1_SAI_TXSYNC_SELECT_INPUT: RWRegister<u32>,

    _reserved2: [u32; 6],

    /// EMVSIM1_SIO_SELECT_INPUT DAISY Register
    pub EMVSIM1_SIO_SELECT_INPUT: RWRegister<u32>,

    /// EMVSIM1_IPP_SIMPD_SELECT_INPUT DAISY Register
    pub EMVSIM1_IPP_SIMPD_SELECT_INPUT: RWRegister<u32>,

    /// EMVSIM1_POWER_FAIL_SELECT_INPUT DAISY Register
    pub EMVSIM1_POWER_FAIL_SELECT_INPUT: RWRegister<u32>,

    /// EMVSIM2_SIO_SELECT_INPUT DAISY Register
    pub EMVSIM2_SIO_SELECT_INPUT: RWRegister<u32>,

    /// EMVSIM2_IPP_SIMPD_SELECT_INPUT DAISY Register
    pub EMVSIM2_IPP_SIMPD_SELECT_INPUT: RWRegister<u32>,

    /// EMVSIM2_POWER_FAIL_SELECT_INPUT DAISY Register
    pub EMVSIM2_POWER_FAIL_SELECT_INPUT: RWRegister<u32>,

    /// SPDIF_SPDIF_IN1_SELECT_INPUT DAISY Register
    pub SPDIF_SPDIF_IN1_SELECT_INPUT: RWRegister<u32>,

    /// USB_OTG2_OC_SELECT_INPUT DAISY Register
    pub USB_OTG2_OC_SELECT_INPUT: RWRegister<u32>,

    /// USB_OTG_OC_SELECT_INPUT DAISY Register
    pub USB_OTG_OC_SELECT_INPUT: RWRegister<u32>,

    /// USBPHY1_USB_ID_SELECT_INPUT DAISY Register
    pub USBPHY1_USB_ID_SELECT_INPUT: RWRegister<u32>,

    /// USBPHY2_USB_ID_SELECT_INPUT DAISY Register
    pub USBPHY2_USB_ID_SELECT_INPUT: RWRegister<u32>,

    /// USDHC1_IPP_CARD_DET_SELECT_INPUT DAISY Register
    pub USDHC1_IPP_CARD_DET_SELECT_INPUT: RWRegister<u32>,

    /// USDHC1_IPP_WP_ON_SELECT_INPUT DAISY Register
    pub USDHC1_IPP_WP_ON_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_IPP_CARD_DET_SELECT_INPUT DAISY Register
    pub USDHC2_IPP_CARD_DET_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_IPP_WP_ON_SELECT_INPUT DAISY Register
    pub USDHC2_IPP_WP_ON_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_20 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_20: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_21 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_21: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_22 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_22: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_23 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_23: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_24 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_24: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_25 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_25: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_26 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_26: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_27 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_27: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_28 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_28: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_29 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_29: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_30 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_30: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_31 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_31: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_32 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_32: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_33 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_33: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_34 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_34: RWRegister<u32>,

    /// XBAR1_IN_SELECT_INPUT_35 DAISY Register
    pub XBAR1_IN_SELECT_INPUT_35: RWRegister<u32>,
}
pub struct ResetValues {
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_15: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_16: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_17: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_18: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_19: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_20: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_21: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_22: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_23: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_24: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_25: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_26: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_27: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_28: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_29: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_30: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_31: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_32: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_33: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_34: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_35: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_36: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_37: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_38: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_39: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_40: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B1_41: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_15: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_16: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_17: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_18: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_19: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_B2_20: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_15: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_16: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_17: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_18: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_19: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_20: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_21: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_22: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_23: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_24: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_25: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_26: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_27: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_28: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_29: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_30: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_31: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_32: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_33: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_34: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_35: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B2_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B1_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_DISP_B2_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_16: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_17: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_18: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_19: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_20: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_21: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_22: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_23: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_24: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_25: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_26: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_27: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_28: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_29: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_30: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_31: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_32: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_33: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_34: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_35: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_36: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_37: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_38: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_39: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_40: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B1_41: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_16: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_17: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_18: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_19: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_B2_20: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_16: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_17: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_18: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_19: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_20: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_21: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_22: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_23: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_24: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_25: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_26: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_27: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_28: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_29: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_30: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_31: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_32: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_33: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_34: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_35: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B2_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B1_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_DISP_B2_15: u32,
    pub FLEXCAN1_RX_SELECT_INPUT: u32,
    pub FLEXCAN2_RX_SELECT_INPUT: u32,
    pub CCM_ENET_QOS_REF_CLK_SELECT_INPUT: u32,
    pub CCM_ENET_QOS_TX_CLK_SELECT_INPUT: u32,
    pub ENET_IPG_CLK_RMII_SELECT_INPUT: u32,
    pub ENET_MAC0_MDIO_SELECT_INPUT: u32,
    pub ENET_MAC0_RXDATA_SELECT_INPUT_0: u32,
    pub ENET_MAC0_RXDATA_SELECT_INPUT_1: u32,
    pub ENET_MAC0_RXEN_SELECT_INPUT: u32,
    pub ENET_MAC0_RXERR_SELECT_INPUT: u32,
    pub ENET_MAC0_TXCLK_SELECT_INPUT: u32,
    pub ENET_1G_IPG_CLK_RMII_SELECT_INPUT: u32,
    pub ENET_1G_MAC0_MDIO_SELECT_INPUT: u32,
    pub ENET_1G_MAC0_RXCLK_SELECT_INPUT: u32,
    pub ENET_1G_MAC0_RXDATA_0_SELECT_INPUT: u32,
    pub ENET_1G_MAC0_RXDATA_1_SELECT_INPUT: u32,
    pub ENET_1G_MAC0_RXDATA_2_SELECT_INPUT: u32,
    pub ENET_1G_MAC0_RXDATA_3_SELECT_INPUT: u32,
    pub ENET_1G_MAC0_RXEN_SELECT_INPUT: u32,
    pub ENET_1G_MAC0_RXERR_SELECT_INPUT: u32,
    pub ENET_1G_MAC0_TXCLK_SELECT_INPUT: u32,
    pub ENET_QOS_GMII_MDI_I_SELECT_INPUT: u32,
    pub ENET_QOS_PHY_RXD_I_SELECT_INPUT_0: u32,
    pub ENET_QOS_PHY_RXD_I_SELECT_INPUT_1: u32,
    pub ENET_QOS_PHY_RXDV_I_SELECT_INPUT: u32,
    pub ENET_QOS_PHY_RXER_I_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMA_SELECT_INPUT_0: u32,
    pub FLEXPWM1_PWMA_SELECT_INPUT_1: u32,
    pub FLEXPWM1_PWMA_SELECT_INPUT_2: u32,
    pub FLEXPWM1_PWMB_SELECT_INPUT_0: u32,
    pub FLEXPWM1_PWMB_SELECT_INPUT_1: u32,
    pub FLEXPWM1_PWMB_SELECT_INPUT_2: u32,
    pub FLEXPWM2_PWMA_SELECT_INPUT_0: u32,
    pub FLEXPWM2_PWMA_SELECT_INPUT_1: u32,
    pub FLEXPWM2_PWMA_SELECT_INPUT_2: u32,
    pub FLEXPWM2_PWMB_SELECT_INPUT_0: u32,
    pub FLEXPWM2_PWMB_SELECT_INPUT_1: u32,
    pub FLEXPWM2_PWMB_SELECT_INPUT_2: u32,
    pub FLEXPWM3_PWMA_SELECT_INPUT_0: u32,
    pub FLEXPWM3_PWMA_SELECT_INPUT_1: u32,
    pub FLEXPWM3_PWMA_SELECT_INPUT_2: u32,
    pub FLEXPWM3_PWMA_SELECT_INPUT_3: u32,
    pub FLEXPWM3_PWMB_SELECT_INPUT_0: u32,
    pub FLEXPWM3_PWMB_SELECT_INPUT_1: u32,
    pub FLEXPWM3_PWMB_SELECT_INPUT_2: u32,
    pub FLEXPWM3_PWMB_SELECT_INPUT_3: u32,
    pub FLEXSPI1_I_DQS_FA_SELECT_INPUT: u32,
    pub FLEXSPI1_I_IO_FA_SELECT_INPUT_0: u32,
    pub FLEXSPI1_I_IO_FA_SELECT_INPUT_1: u32,
    pub FLEXSPI1_I_IO_FA_SELECT_INPUT_2: u32,
    pub FLEXSPI1_I_IO_FA_SELECT_INPUT_3: u32,
    pub FLEXSPI1_I_IO_FB_SELECT_INPUT_0: u32,
    pub FLEXSPI1_I_IO_FB_SELECT_INPUT_1: u32,
    pub FLEXSPI1_I_IO_FB_SELECT_INPUT_2: u32,
    pub FLEXSPI1_I_IO_FB_SELECT_INPUT_3: u32,
    pub FLEXSPI1_I_SCK_FA_SELECT_INPUT: u32,
    pub FLEXSPI1_I_SCK_FB_SELECT_INPUT: u32,
    pub FLEXSPI2_I_IO_FA_SELECT_INPUT_0: u32,
    pub FLEXSPI2_I_IO_FA_SELECT_INPUT_1: u32,
    pub FLEXSPI2_I_IO_FA_SELECT_INPUT_2: u32,
    pub FLEXSPI2_I_IO_FA_SELECT_INPUT_3: u32,
    pub FLEXSPI2_I_SCK_FA_SELECT_INPUT: u32,
    pub GPT3_CAPIN1_SELECT_INPUT: u32,
    pub GPT3_CAPIN2_SELECT_INPUT: u32,
    pub GPT3_CLKIN_SELECT_INPUT: u32,
    pub KPP_COL_SELECT_INPUT_6: u32,
    pub KPP_COL_SELECT_INPUT_7: u32,
    pub KPP_ROW_SELECT_INPUT_6: u32,
    pub KPP_ROW_SELECT_INPUT_7: u32,
    pub LPI2C1_LPI2C_SCL_SELECT_INPUT: u32,
    pub LPI2C1_LPI2C_SDA_SELECT_INPUT: u32,
    pub LPI2C2_LPI2C_SCL_SELECT_INPUT: u32,
    pub LPI2C2_LPI2C_SDA_SELECT_INPUT: u32,
    pub LPI2C3_LPI2C_SCL_SELECT_INPUT: u32,
    pub LPI2C3_LPI2C_SDA_SELECT_INPUT: u32,
    pub LPI2C4_LPI2C_SCL_SELECT_INPUT: u32,
    pub LPI2C4_LPI2C_SDA_SELECT_INPUT: u32,
    pub LPSPI1_LPSPI_PCS_SELECT_INPUT_0: u32,
    pub LPSPI1_LPSPI_SCK_SELECT_INPUT: u32,
    pub LPSPI1_LPSPI_SDI_SELECT_INPUT: u32,
    pub LPSPI1_LPSPI_SDO_SELECT_INPUT: u32,
    pub LPSPI2_LPSPI_PCS_SELECT_INPUT_0: u32,
    pub LPSPI2_LPSPI_PCS_SELECT_INPUT_1: u32,
    pub LPSPI2_LPSPI_SCK_SELECT_INPUT: u32,
    pub LPSPI2_LPSPI_SDI_SELECT_INPUT: u32,
    pub LPSPI2_LPSPI_SDO_SELECT_INPUT: u32,
    pub LPSPI3_LPSPI_PCS_SELECT_INPUT_0: u32,
    pub LPSPI3_LPSPI_PCS_SELECT_INPUT_1: u32,
    pub LPSPI3_LPSPI_PCS_SELECT_INPUT_2: u32,
    pub LPSPI3_LPSPI_PCS_SELECT_INPUT_3: u32,
    pub LPSPI3_LPSPI_SCK_SELECT_INPUT: u32,
    pub LPSPI3_LPSPI_SDI_SELECT_INPUT: u32,
    pub LPSPI3_LPSPI_SDO_SELECT_INPUT: u32,
    pub LPSPI4_LPSPI_PCS_SELECT_INPUT_0: u32,
    pub LPSPI4_LPSPI_SCK_SELECT_INPUT: u32,
    pub LPSPI4_LPSPI_SDI_SELECT_INPUT: u32,
    pub LPSPI4_LPSPI_SDO_SELECT_INPUT: u32,
    pub LPUART1_LPUART_RXD_SELECT_INPUT: u32,
    pub LPUART1_LPUART_TXD_SELECT_INPUT: u32,
    pub LPUART10_LPUART_RXD_SELECT_INPUT: u32,
    pub LPUART10_LPUART_TXD_SELECT_INPUT: u32,
    pub LPUART7_LPUART_RXD_SELECT_INPUT: u32,
    pub LPUART7_LPUART_TXD_SELECT_INPUT: u32,
    pub LPUART8_LPUART_RXD_SELECT_INPUT: u32,
    pub LPUART8_LPUART_TXD_SELECT_INPUT: u32,
    pub QTIMER1_TMR0_INPUT_SELECT_INPUT: u32,
    pub QTIMER1_TMR1_INPUT_SELECT_INPUT: u32,
    pub QTIMER1_TMR2_INPUT_SELECT_INPUT: u32,
    pub QTIMER2_TMR0_INPUT_SELECT_INPUT: u32,
    pub QTIMER2_TMR1_INPUT_SELECT_INPUT: u32,
    pub QTIMER2_TMR2_INPUT_SELECT_INPUT: u32,
    pub QTIMER3_TMR0_INPUT_SELECT_INPUT: u32,
    pub QTIMER3_TMR1_INPUT_SELECT_INPUT: u32,
    pub QTIMER3_TMR2_INPUT_SELECT_INPUT: u32,
    pub QTIMER4_TMR0_INPUT_SELECT_INPUT: u32,
    pub QTIMER4_TMR1_INPUT_SELECT_INPUT: u32,
    pub QTIMER4_TMR2_INPUT_SELECT_INPUT: u32,
    pub SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT: u32,
    pub SAI1_SAI_RXBCLK_SELECT_INPUT: u32,
    pub SAI1_SAI_RXDATA_SELECT_INPUT_0: u32,
    pub SAI1_SAI_RXSYNC_SELECT_INPUT: u32,
    pub SAI1_SAI_TXBCLK_SELECT_INPUT: u32,
    pub SAI1_SAI_TXSYNC_SELECT_INPUT: u32,
    pub EMVSIM1_SIO_SELECT_INPUT: u32,
    pub EMVSIM1_IPP_SIMPD_SELECT_INPUT: u32,
    pub EMVSIM1_POWER_FAIL_SELECT_INPUT: u32,
    pub EMVSIM2_SIO_SELECT_INPUT: u32,
    pub EMVSIM2_IPP_SIMPD_SELECT_INPUT: u32,
    pub EMVSIM2_POWER_FAIL_SELECT_INPUT: u32,
    pub SPDIF_SPDIF_IN1_SELECT_INPUT: u32,
    pub USB_OTG2_OC_SELECT_INPUT: u32,
    pub USB_OTG_OC_SELECT_INPUT: u32,
    pub USBPHY1_USB_ID_SELECT_INPUT: u32,
    pub USBPHY2_USB_ID_SELECT_INPUT: u32,
    pub USDHC1_IPP_CARD_DET_SELECT_INPUT: u32,
    pub USDHC1_IPP_WP_ON_SELECT_INPUT: u32,
    pub USDHC2_IPP_CARD_DET_SELECT_INPUT: u32,
    pub USDHC2_IPP_WP_ON_SELECT_INPUT: u32,
    pub XBAR1_IN_SELECT_INPUT_20: u32,
    pub XBAR1_IN_SELECT_INPUT_21: u32,
    pub XBAR1_IN_SELECT_INPUT_22: u32,
    pub XBAR1_IN_SELECT_INPUT_23: u32,
    pub XBAR1_IN_SELECT_INPUT_24: u32,
    pub XBAR1_IN_SELECT_INPUT_25: u32,
    pub XBAR1_IN_SELECT_INPUT_26: u32,
    pub XBAR1_IN_SELECT_INPUT_27: u32,
    pub XBAR1_IN_SELECT_INPUT_28: u32,
    pub XBAR1_IN_SELECT_INPUT_29: u32,
    pub XBAR1_IN_SELECT_INPUT_30: u32,
    pub XBAR1_IN_SELECT_INPUT_31: u32,
    pub XBAR1_IN_SELECT_INPUT_32: u32,
    pub XBAR1_IN_SELECT_INPUT_33: u32,
    pub XBAR1_IN_SELECT_INPUT_34: u32,
    pub XBAR1_IN_SELECT_INPUT_35: u32,
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
