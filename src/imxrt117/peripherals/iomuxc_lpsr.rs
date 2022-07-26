#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC LPSR
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// SW_MUX_CTL_PAD_GPIO_LPSR_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_00 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXCAN3_TX of instance: FLEXCAN3
            pub const ALT0_can3_TX: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: MIC_CLK of instance: MIC
            pub const ALT1_mic_CLK: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: MQS_RIGHT of instance: MQS
            pub const ALT2_mqs_RIGHT: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_CM4_EVENTO of instance: CM4
            pub const ALT3_ARM_CM4_EVENTO: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO00 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO0: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPUART12_TXD of instance: LPUART12
            pub const ALT6_lpuart12_TX: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: SAI4_MCLK of instance: SAI4
            pub const ALT7_sai4_MCLK: u32 = 0b0111;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO00 of instance: GPIO12
            pub const ALT10_gpio12_IO0: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_01 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXCAN3_RX of instance: FLEXCAN3
            pub const ALT0_can3_RX: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: MIC_BITSTREAM0 of instance: MIC
            pub const ALT1_mic_BITSTREAM0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: MQS_LEFT of instance: MQS
            pub const ALT2_mqs_LEFT: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_CM4_EVENTI of instance: CM4
            pub const ALT3_ARM_CM4_EVENTI: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO01 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO1: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPUART12_RXD of instance: LPUART12
            pub const ALT6_lpuart12_RX: u32 = 0b0110;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO01 of instance: GPIO12
            pub const ALT10_gpio12_IO1: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_02 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: SRC_BOOT_MODE00 of instance: SRC
            pub const ALT0_src_BOOT_MODE0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI5_SCK of instance: LPSPI5
            pub const ALT1_lpspi5_SCK: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI4_TX_DATA of instance: SAI4
            pub const ALT2_sai4_TX_DATA: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: MQS_RIGHT of instance: MQS
            pub const ALT3_mqs_RIGHT: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO02 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO2: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO02 of instance: GPIO12
            pub const ALT10_gpio12_IO2: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_03 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: SRC_BOOT_MODE01 of instance: SRC
            pub const ALT0_src_BOOT_MODE1: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI5_PCS0 of instance: LPSPI5
            pub const ALT1_lpspi5_PCS0: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI4_TX_SYNC of instance: SAI4
            pub const ALT2_sai4_TX_SYNC: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: MQS_LEFT of instance: MQS
            pub const ALT3_mqs_LEFT: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO03 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO3: u32 = 0b0101;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO03 of instance: GPIO12
            pub const ALT10_gpio12_IO3: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_04 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: LPI2C5_SDA of instance: LPI2C5
            pub const ALT0_lpi2c5_SDA: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI5_SOUT of instance: LPSPI5
            pub const ALT1_lpspi5_SDO: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI4_TX_BCLK of instance: SAI4
            pub const ALT2_sai4_TX_BCLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART12_RTS_B of instance: LPUART12
            pub const ALT3_lpuart12_RTS_B: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO04 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO4: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPUART11_TXD of instance: LPUART11
            pub const ALT6_lpuart11_TX: u32 = 0b0110;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO04 of instance: GPIO12
            pub const ALT10_gpio12_IO4: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_05 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: LPI2C5_SCL of instance: LPI2C5
            pub const ALT0_lpi2c5_SCL: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI5_SIN of instance: LPSPI5
            pub const ALT1_lpspi5_SDI: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI4_MCLK of instance: SAI4
            pub const ALT2_sai4_MCLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART12_CTS_B of instance: LPUART12
            pub const ALT3_lpuart12_CTS_B: u32 = 0b0011;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO05 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPUART11_RXD of instance: LPUART11
            pub const ALT6_lpuart11_RX: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: NMI_GLUE_NMI of instance: nmi_glue
            pub const ALT7_nmi_glue_NMI: u32 = 0b0111;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO05 of instance: GPIO12
            pub const ALT10_gpio12_IO5: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_06 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: LPI2C6_SDA of instance: LPI2C6
            pub const ALT0_lpi2c6_SDA: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI4_RX_DATA of instance: SAI4
            pub const ALT2_sai4_RX_DATA: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART12_TXD of instance: LPUART12
            pub const ALT3_lpuart12_TX: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI6_PCS3 of instance: LPSPI6
            pub const ALT4_lpspi6_PCS3: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO06 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO6: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXCAN3_TX of instance: FLEXCAN3
            pub const ALT6_can3_TX: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: PIT2_TRIGGER3 of instance: PIT2
            pub const ALT7_pit2_TRIGGER3: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI5_PCS1 of instance: LPSPI5
            pub const ALT8_lpspi5_PCS1: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO06 of instance: GPIO12
            pub const ALT10_gpio12_IO6: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_07 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: LPI2C6_SCL of instance: LPI2C6
            pub const ALT0_lpi2c6_SCL: u32 = 0b0000;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI4_RX_BCLK of instance: SAI4
            pub const ALT2_sai4_RX_BCLK: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART12_RXD of instance: LPUART12
            pub const ALT3_lpuart12_RX: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI6_PCS2 of instance: LPSPI6
            pub const ALT4_lpspi6_PCS2: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO07 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO7: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXCAN3_RX of instance: FLEXCAN3
            pub const ALT6_can3_RX: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: PIT2_TRIGGER2 of instance: PIT2
            pub const ALT7_pit2_TRIGGER2: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI5_PCS2 of instance: LPSPI5
            pub const ALT8_lpspi5_PCS2: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO07 of instance: GPIO12
            pub const ALT10_gpio12_IO7: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_08 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: LPUART11_TXD of instance: LPUART11
            pub const ALT0_lpuart11_TX: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXCAN3_TX of instance: FLEXCAN3
            pub const ALT1_can3_TX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI4_RX_SYNC of instance: SAI4
            pub const ALT2_sai4_RX_SYNC: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: MIC_CLK of instance: MIC
            pub const ALT3_mic_CLK: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI6_PCS1 of instance: LPSPI6
            pub const ALT4_lpspi6_PCS1: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO08 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO8: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPI2C5_SDA of instance: LPI2C5
            pub const ALT6_lpi2c5_SDA: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: PIT2_TRIGGER1 of instance: PIT2
            pub const ALT7_pit2_TRIGGER1: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI5_PCS3 of instance: LPSPI5
            pub const ALT8_lpspi5_PCS3: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO08 of instance: GPIO12
            pub const ALT10_gpio12_IO8: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_09 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: LPUART11_RXD of instance: LPUART11
            pub const ALT0_lpuart11_RX: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXCAN3_RX of instance: FLEXCAN3
            pub const ALT1_can3_RX: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: PIT2_TRIGGER0 of instance: PIT2
            pub const ALT2_pit2_TRIGGER0: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: MIC_BITSTREAM0 of instance: MIC
            pub const ALT3_mic_BITSTREAM0: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI6_PCS0 of instance: LPSPI6
            pub const ALT4_lpspi6_PCS0: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO09 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO9: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPI2C5_SCL of instance: LPI2C5
            pub const ALT6_lpi2c5_SCL: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: SAI4_TX_DATA of instance: SAI4
            pub const ALT7_sai4_TX_DATA: u32 = 0b0111;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO09 of instance: GPIO12
            pub const ALT10_gpio12_IO9: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_10 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: JTAG_MUX_TRSTB of instance: JTAG_MUX
            pub const ALT0_jtag_mux_TRSTB: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART11_CTS_B of instance: LPUART11
            pub const ALT1_lpuart11_CTS_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C6_SDA of instance: LPI2C6
            pub const ALT2_lpi2c6_SDA: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: MIC_BITSTREAM1 of instance: MIC
            pub const ALT3_mic_BITSTREAM1: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI6_SCK of instance: LPSPI6
            pub const ALT4_lpspi6_SCK: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO10 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO10: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPI2C5_SCLS of instance: LPI2C5
            pub const ALT6_lpi2c5_SCLS: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: SAI4_TX_SYNC of instance: SAI4
            pub const ALT7_sai4_TX_SYNC: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPUART12_TXD of instance: LPUART12
            pub const ALT8_lpuart12_TX: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO10 of instance: GPIO12
            pub const ALT10_gpio12_IO10: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_11 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: JTAG_MUX_TDO of instance: JTAG_MUX
            pub const ALT0_jtag_mux_TDO: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART11_RTS_B of instance: LPUART11
            pub const ALT1_lpuart11_RTS_B: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C6_SCL of instance: LPI2C6
            pub const ALT2_lpi2c6_SCL: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: MIC_BITSTREAM2 of instance: MIC
            pub const ALT3_mic_BITSTREAM2: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI6_SOUT of instance: LPSPI6
            pub const ALT4_lpspi6_SDO: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO11 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO11: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPI2C5_SDAS of instance: LPI2C5
            pub const ALT6_lpi2c5_SDAS: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ARM_TRACE_SWO of instance: ARM
            pub const ALT7_ARM_TRACE_SWO: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPUART12_RXD of instance: LPUART12
            pub const ALT8_lpuart12_RX: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO11 of instance: GPIO12
            pub const ALT10_gpio12_IO11: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_12 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: JTAG_MUX_TDI of instance: JTAG_MUX
            pub const ALT0_jtag_mux_TDI: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: PIT2_TRIGGER0 of instance: PIT2
            pub const ALT1_pit2_TRIGGER0: u32 = 0b0001;

            /// 0b0011: Select mux mode: ALT3 mux port: MIC_BITSTREAM3 of instance: MIC
            pub const ALT3_mic_BITSTREAM3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI6_SIN of instance: LPSPI6
            pub const ALT4_lpspi6_SDI: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO12 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO12: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPI2C5_HREQ of instance: LPI2C5
            pub const ALT6_lpi2c5_HREQ: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: SAI4_TX_BCLK of instance: SAI4
            pub const ALT7_sai4_TX_BCLK: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI5_SCK of instance: LPSPI5
            pub const ALT8_lpspi5_SCK: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO12 of instance: GPIO12
            pub const ALT10_gpio12_IO12: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_13 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: JTAG_MUX_MOD of instance: JTAG_MUX
            pub const ALT0_jtag_mux_MOD: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: MIC_BITSTREAM1 of instance: MIC
            pub const ALT1_mic_BITSTREAM1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: PIT2_TRIGGER1 of instance: PIT2
            pub const ALT2_pit2_TRIGGER1: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO13 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO13: u32 = 0b0101;

            /// 0b0111: Select mux mode: ALT7 mux port: SAI4_RX_DATA of instance: SAI4
            pub const ALT7_sai4_RX_DATA: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI5_PCS0 of instance: LPSPI5
            pub const ALT8_lpspi5_PCS0: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO13 of instance: GPIO12
            pub const ALT10_gpio12_IO13: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_14 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: JTAG_MUX_TCK of instance: JTAG_MUX/SWD_CLK
            pub const ALT0_jtag_mux_TCK: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: MIC_BITSTREAM2 of instance: MIC
            pub const ALT1_mic_BITSTREAM2: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: PIT2_TRIGGER2 of instance: PIT2
            pub const ALT2_pit2_TRIGGER2: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO14 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO14: u32 = 0b0101;

            /// 0b0111: Select mux mode: ALT7 mux port: SAI4_RX_BCLK of instance: SAI4
            pub const ALT7_sai4_RX_BCLK: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI5_SOUT of instance: LPSPI5
            pub const ALT8_lpspi5_SDO: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO14 of instance: GPIO12
            pub const ALT10_gpio12_IO14: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_LPSR_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_LPSR_15 {

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

            /// 0b0000: Select mux mode: ALT0 mux port: JTAG_MUX_TMS of instance: JTAG_MUX/SWD_DIO
            pub const ALT0_jtag_mux_TMS: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: MIC_BITSTREAM3 of instance: MIC
            pub const ALT1_mic_BITSTREAM3: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: PIT2_TRIGGER3 of instance: PIT2
            pub const ALT2_pit2_TRIGGER3: u32 = 0b0010;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO_MUX6_IO15 of instance: GPIO_MUX6
            pub const ALT5_gpio_mux6_IO15: u32 = 0b0101;

            /// 0b0111: Select mux mode: ALT7 mux port: SAI4_RX_SYNC of instance: SAI4
            pub const ALT7_sai4_RX_SYNC: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: LPSPI5_SIN of instance: LPSPI5
            pub const ALT8_lpspi5_SDI: u32 = 0b1000;

            /// 0b1010: Select mux mode: ALT10 mux port: GPIO12_IO15 of instance: GPIO12
            pub const ALT10_gpio12_IO15: u32 = 0b1010;
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

            /// 0b1: Force input path of pad GPIO_LPSR_15
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_00 {

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

    /// Open Drain LPSR Field
    pub mod ODE_LPSR {
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

            /// 0b0: Disabled
            pub const ODE_LPSR_0_Disabled: u32 = 0b0;

            /// 0b1: Enabled
            pub const ODE_LPSR_1_Enabled: u32 = 0b1;
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

/// SW_PAD_CTL_PAD_GPIO_LPSR_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_LPSR_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_LPSR_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::DWP_LOCK;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::ODE_LPSR;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_LPSR_00::SRE;
}

/// CAN3_IPP_IND_CANRX_SELECT_INPUT DAISY Register
pub mod CAN3_IPP_IND_CANRX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_LPSR_01 for Mode: ALT0
            pub const SELECT_GPIO_LPSR_01_ALT0: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_LPSR_07 for Mode: ALT6
            pub const SELECT_GPIO_LPSR_07_ALT6: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_LPSR_09 for Mode: ALT1
            pub const SELECT_GPIO_LPSR_09_ALT1: u32 = 0b10;
        }
    }
}

/// LPI2C5_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C5_IPP_IND_LPI2C_SCL_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_05 for Mode: ALT0
            pub const SELECT_GPIO_LPSR_05_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_09 for Mode: ALT6
            pub const SELECT_GPIO_LPSR_09_ALT6: u32 = 0b1;
        }
    }
}

/// LPI2C5_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C5_IPP_IND_LPI2C_SDA_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_04 for Mode: ALT0
            pub const SELECT_GPIO_LPSR_04_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_08 for Mode: ALT6
            pub const SELECT_GPIO_LPSR_08_ALT6: u32 = 0b1;
        }
    }
}

/// LPI2C6_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C6_IPP_IND_LPI2C_SCL_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_07 for Mode: ALT0
            pub const SELECT_GPIO_LPSR_07_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_11 for Mode: ALT2
            pub const SELECT_GPIO_LPSR_11_ALT2: u32 = 0b1;
        }
    }
}

/// LPI2C6_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C6_IPP_IND_LPI2C_SDA_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_06 for Mode: ALT0
            pub const SELECT_GPIO_LPSR_06_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_10 for Mode: ALT2
            pub const SELECT_GPIO_LPSR_10_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
pub mod LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_03 for Mode: ALT1
            pub const SELECT_GPIO_LPSR_03_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_13 for Mode: ALT8
            pub const SELECT_GPIO_LPSR_13_ALT8: u32 = 0b1;
        }
    }
}

/// LPSPI5_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI5_IPP_IND_LPSPI_SCK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_02 for Mode: ALT1
            pub const SELECT_GPIO_LPSR_02_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_12 for Mode: ALT8
            pub const SELECT_GPIO_LPSR_12_ALT8: u32 = 0b1;
        }
    }
}

/// LPSPI5_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI5_IPP_IND_LPSPI_SDI_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_05 for Mode: ALT1
            pub const SELECT_GPIO_LPSR_05_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_15 for Mode: ALT8
            pub const SELECT_GPIO_LPSR_15_ALT8: u32 = 0b1;
        }
    }
}

/// LPSPI5_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI5_IPP_IND_LPSPI_SDO_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_04 for Mode: ALT1
            pub const SELECT_GPIO_LPSR_04_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_14 for Mode: ALT8
            pub const SELECT_GPIO_LPSR_14_ALT8: u32 = 0b1;
        }
    }
}

/// LPUART11_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
pub mod LPUART11_IPP_IND_LPUART_RXD_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_05 for Mode: ALT6
            pub const SELECT_GPIO_LPSR_05_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_09 for Mode: ALT0
            pub const SELECT_GPIO_LPSR_09_ALT0: u32 = 0b1;
        }
    }
}

/// LPUART11_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
pub mod LPUART11_IPP_IND_LPUART_TXD_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_04 for Mode: ALT6
            pub const SELECT_GPIO_LPSR_04_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_08 for Mode: ALT0
            pub const SELECT_GPIO_LPSR_08_ALT0: u32 = 0b1;
        }
    }
}

/// LPUART12_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
pub mod LPUART12_IPP_IND_LPUART_RXD_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_LPSR_01 for Mode: ALT6
            pub const SELECT_GPIO_LPSR_01_ALT6: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_LPSR_07 for Mode: ALT3
            pub const SELECT_GPIO_LPSR_07_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_LPSR_11 for Mode: ALT8
            pub const SELECT_GPIO_LPSR_11_ALT8: u32 = 0b10;
        }
    }
}

/// LPUART12_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
pub mod LPUART12_IPP_IND_LPUART_TXD_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_LPSR_00 for Mode: ALT6
            pub const SELECT_GPIO_LPSR_00_ALT6: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_LPSR_06 for Mode: ALT3
            pub const SELECT_GPIO_LPSR_06_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_LPSR_10 for Mode: ALT8
            pub const SELECT_GPIO_LPSR_10_ALT8: u32 = 0b10;
        }
    }
}

/// MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_0 DAISY Register
pub mod MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_0 {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_01 for Mode: ALT1
            pub const SELECT_GPIO_LPSR_01_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_09 for Mode: ALT3
            pub const SELECT_GPIO_LPSR_09_ALT3: u32 = 0b1;
        }
    }
}

/// MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_1 DAISY Register
pub mod MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_1 {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_10 for Mode: ALT3
            pub const SELECT_GPIO_LPSR_10_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_13 for Mode: ALT1
            pub const SELECT_GPIO_LPSR_13_ALT1: u32 = 0b1;
        }
    }
}

/// MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_2 DAISY Register
pub mod MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_2 {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_11 for Mode: ALT3
            pub const SELECT_GPIO_LPSR_11_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_14 for Mode: ALT1
            pub const SELECT_GPIO_LPSR_14_ALT1: u32 = 0b1;
        }
    }
}

/// MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_3 DAISY Register
pub mod MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_3 {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_12 for Mode: ALT3
            pub const SELECT_GPIO_LPSR_12_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_15 for Mode: ALT1
            pub const SELECT_GPIO_LPSR_15_ALT1: u32 = 0b1;
        }
    }
}

/// NMI_GLUE_IPP_IND_NMI_SELECT_INPUT DAISY Register
pub mod NMI_GLUE_IPP_IND_NMI_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_05 for Mode: ALT7
            pub const SELECT_GPIO_LPSR_05_ALT7: u32 = 0b0;

            /// 0b1: Selecting Pad: WAKEUP_DIG for Mode: ALT7
            pub const SELECT_WAKEUP_DIG_ALT7: u32 = 0b1;
        }
    }
}

/// SAI4_IPG_CLK_SAI_MCLK_SELECT_INPUT DAISY Register
pub mod SAI4_IPG_CLK_SAI_MCLK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_00 for Mode: ALT7
            pub const SELECT_GPIO_LPSR_00_ALT7: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_05 for Mode: ALT2
            pub const SELECT_GPIO_LPSR_05_ALT2: u32 = 0b1;
        }
    }
}

/// SAI4_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register
pub mod SAI4_IPP_IND_SAI_RXBCLK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_07 for Mode: ALT2
            pub const SELECT_GPIO_LPSR_07_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_14 for Mode: ALT7
            pub const SELECT_GPIO_LPSR_14_ALT7: u32 = 0b1;
        }
    }
}

/// SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
pub mod SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_06 for Mode: ALT2
            pub const SELECT_GPIO_LPSR_06_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_13 for Mode: ALT7
            pub const SELECT_GPIO_LPSR_13_ALT7: u32 = 0b1;
        }
    }
}

/// SAI4_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register
pub mod SAI4_IPP_IND_SAI_RXSYNC_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_08 for Mode: ALT2
            pub const SELECT_GPIO_LPSR_08_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_15 for Mode: ALT7
            pub const SELECT_GPIO_LPSR_15_ALT7: u32 = 0b1;
        }
    }
}

/// SAI4_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register
pub mod SAI4_IPP_IND_SAI_TXBCLK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_04 for Mode: ALT2
            pub const SELECT_GPIO_LPSR_04_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_12 for Mode: ALT7
            pub const SELECT_GPIO_LPSR_12_ALT7: u32 = 0b1;
        }
    }
}

/// SAI4_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register
pub mod SAI4_IPP_IND_SAI_TXSYNC_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_LPSR_03 for Mode: ALT2
            pub const SELECT_GPIO_LPSR_03_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_LPSR_10 for Mode: ALT7
            pub const SELECT_GPIO_LPSR_10_ALT7: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// SW_MUX_CTL_PAD_GPIO_LPSR_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_LPSR_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_LPSR_15: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_LPSR_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_LPSR_15: RWRegister<u32>,

    /// CAN3_IPP_IND_CANRX_SELECT_INPUT DAISY Register
    pub CAN3_IPP_IND_CANRX_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C5_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register
    pub LPI2C5_IPP_IND_LPI2C_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C5_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register
    pub LPI2C5_IPP_IND_LPI2C_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C6_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register
    pub LPI2C6_IPP_IND_LPI2C_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C6_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register
    pub LPI2C6_IPP_IND_LPI2C_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
    pub LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: RWRegister<u32>,

    /// LPSPI5_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register
    pub LPSPI5_IPP_IND_LPSPI_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI5_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register
    pub LPSPI5_IPP_IND_LPSPI_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI5_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register
    pub LPSPI5_IPP_IND_LPSPI_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPUART11_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
    pub LPUART11_IPP_IND_LPUART_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART11_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
    pub LPUART11_IPP_IND_LPUART_TXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART12_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
    pub LPUART12_IPP_IND_LPUART_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART12_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
    pub LPUART12_IPP_IND_LPUART_TXD_SELECT_INPUT: RWRegister<u32>,

    /// MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_0 DAISY Register
    pub MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_0: RWRegister<u32>,

    /// MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_1 DAISY Register
    pub MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_1: RWRegister<u32>,

    /// MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_2 DAISY Register
    pub MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_2: RWRegister<u32>,

    /// MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_3 DAISY Register
    pub MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_3: RWRegister<u32>,

    /// NMI_GLUE_IPP_IND_NMI_SELECT_INPUT DAISY Register
    pub NMI_GLUE_IPP_IND_NMI_SELECT_INPUT: RWRegister<u32>,

    /// SAI4_IPG_CLK_SAI_MCLK_SELECT_INPUT DAISY Register
    pub SAI4_IPG_CLK_SAI_MCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI4_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register
    pub SAI4_IPP_IND_SAI_RXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
    pub SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: RWRegister<u32>,

    /// SAI4_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register
    pub SAI4_IPP_IND_SAI_RXSYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI4_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register
    pub SAI4_IPP_IND_SAI_TXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI4_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register
    pub SAI4_IPP_IND_SAI_TXSYNC_SELECT_INPUT: RWRegister<u32>,
}
pub struct ResetValues {
    pub SW_MUX_CTL_PAD_GPIO_LPSR_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_LPSR_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_LPSR_15: u32,
    pub CAN3_IPP_IND_CANRX_SELECT_INPUT: u32,
    pub LPI2C5_IPP_IND_LPI2C_SCL_SELECT_INPUT: u32,
    pub LPI2C5_IPP_IND_LPI2C_SDA_SELECT_INPUT: u32,
    pub LPI2C6_IPP_IND_LPI2C_SCL_SELECT_INPUT: u32,
    pub LPI2C6_IPP_IND_LPI2C_SDA_SELECT_INPUT: u32,
    pub LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: u32,
    pub LPSPI5_IPP_IND_LPSPI_SCK_SELECT_INPUT: u32,
    pub LPSPI5_IPP_IND_LPSPI_SDI_SELECT_INPUT: u32,
    pub LPSPI5_IPP_IND_LPSPI_SDO_SELECT_INPUT: u32,
    pub LPUART11_IPP_IND_LPUART_RXD_SELECT_INPUT: u32,
    pub LPUART11_IPP_IND_LPUART_TXD_SELECT_INPUT: u32,
    pub LPUART12_IPP_IND_LPUART_RXD_SELECT_INPUT: u32,
    pub LPUART12_IPP_IND_LPUART_TXD_SELECT_INPUT: u32,
    pub MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_0: u32,
    pub MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_1: u32,
    pub MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_2: u32,
    pub MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_3: u32,
    pub NMI_GLUE_IPP_IND_NMI_SELECT_INPUT: u32,
    pub SAI4_IPG_CLK_SAI_MCLK_SELECT_INPUT: u32,
    pub SAI4_IPP_IND_SAI_RXBCLK_SELECT_INPUT: u32,
    pub SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: u32,
    pub SAI4_IPP_IND_SAI_RXSYNC_SELECT_INPUT: u32,
    pub SAI4_IPP_IND_SAI_TXBCLK_SELECT_INPUT: u32,
    pub SAI4_IPP_IND_SAI_TXSYNC_SELECT_INPUT: u32,
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
