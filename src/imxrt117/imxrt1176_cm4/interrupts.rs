#[cfg(feature = "rt")]
extern "C" {
    fn DMA0_DMA16();
    fn DMA1_DMA17();
    fn DMA2_DMA18();
    fn DMA3_DMA19();
    fn DMA4_DMA20();
    fn DMA5_DMA21();
    fn DMA6_DMA22();
    fn DMA7_DMA23();
    fn DMA8_DMA24();
    fn DMA9_DMA25();
    fn DMA10_DMA26();
    fn DMA11_DMA27();
    fn DMA12_DMA28();
    fn DMA13_DMA29();
    fn DMA14_DMA30();
    fn DMA15_DMA31();
    fn DMA_ERROR();
    fn Reserved33();
    fn Reserved34();
    fn CORE();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn LPUART5();
    fn LPUART6();
    fn LPUART7();
    fn LPUART8();
    fn LPUART9();
    fn LPUART10();
    fn LPUART11();
    fn LPUART12();
    fn LPI2C1();
    fn LPI2C2();
    fn LPI2C3();
    fn LPI2C4();
    fn LPI2C5();
    fn LPI2C6();
    fn LPSPI1();
    fn LPSPI2();
    fn LPSPI3();
    fn LPSPI4();
    fn LPSPI5();
    fn LPSPI6();
    fn CAN1();
    fn CAN1_ERROR();
    fn CAN2();
    fn CAN2_ERROR();
    fn CAN3();
    fn CAN3_ERROR();
    fn Reserved66();
    fn KPP();
    fn Reserved68();
    fn GPR_IRQ();
    fn eLCDIF();
    fn LCDIFv2();
    fn CSI();
    fn PXP();
    fn MIPI_CSI();
    fn MIPI_DSI();
    fn GPU2D();
    fn GPIO12_Combined_0_15();
    fn GPIO12_Combined_16_31();
    fn DAC();
    fn KEY_MANAGER();
    fn WDOG2();
    fn SNVS_HP_NON_TZ();
    fn SNVS_HP_TZ();
    fn SNVS_PULSE_EVENT();
    fn CAAM_IRQ0();
    fn CAAM_IRQ1();
    fn CAAM_IRQ2();
    fn CAAM_IRQ3();
    fn CAAM_RECORVE_ERRPR();
    fn CAAM_RTIC();
    fn CDOG();
    fn SAI1();
    fn SAI2();
    fn SAI3_RX();
    fn SAI3_TX();
    fn SAI4_RX();
    fn SAI4_TX();
    fn SPDIF();
    fn TMPSNS_INT();
    fn TMPSNS_LOW_HIGH();
    fn TMPSNS_PANIC();
    fn LPSR_LP8_BROWNOUT();
    fn LPSR_LP0_BROWNOUT();
    fn ADC1();
    fn ADC2();
    fn USBPHY1();
    fn USBPHY2();
    fn RDC();
    fn GPIO13_Combined_0_31();
    fn DCIC1();
    fn DCIC2();
    fn ASRC();
    fn FLEXRAM_ECC();
    fn GPIO7_8_9_10_11();
    fn GPIO1_Combined_0_15();
    fn GPIO1_Combined_16_31();
    fn GPIO2_Combined_0_15();
    fn GPIO2_Combined_16_31();
    fn GPIO3_Combined_0_15();
    fn GPIO3_Combined_16_31();
    fn GPIO4_Combined_0_15();
    fn GPIO4_Combined_16_31();
    fn GPIO5_Combined_0_15();
    fn GPIO5_Combined_16_31();
    fn FLEXIO1();
    fn FLEXIO2();
    fn WDOG1();
    fn RTWDOG4();
    fn EWM();
    fn OCOTP_READ_FUSE_ERROR();
    fn OCOTP_READ_DONE_ERROR();
    fn GPC();
    fn MUB();
    fn GPT1();
    fn GPT2();
    fn GPT3();
    fn GPT4();
    fn GPT5();
    fn GPT6();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn PWM1_FAULT();
    fn FLEXSPI1();
    fn FLEXSPI2();
    fn SEMC();
    fn USDHC1();
    fn USDHC2();
    fn USB_OTG2();
    fn USB_OTG1();
    fn ENET();
    fn ENET_1588_Timer();
    fn ENET_1G_MAC0_Tx_Rx_1();
    fn ENET_1G_MAC0_Tx_Rx_2();
    fn ENET_1G();
    fn ENET_1G_1588_Timer();
    fn XBAR1_IRQ_0_1();
    fn XBAR1_IRQ_2_3();
    fn ADC_ETC_IRQ0();
    fn ADC_ETC_IRQ1();
    fn ADC_ETC_IRQ2();
    fn ADC_ETC_IRQ3();
    fn ADC_ETC_ERROR_IRQ();
    fn PIT1();
    fn PIT2();
    fn ACMP1();
    fn ACMP2();
    fn ACMP3();
    fn ACMP4();
    fn Reserved177();
    fn Reserved178();
    fn Reserved179();
    fn Reserved180();
    fn ENC1();
    fn ENC2();
    fn ENC3();
    fn ENC4();
    fn Reserved185();
    fn Reserved186();
    fn TMR1();
    fn TMR2();
    fn TMR3();
    fn TMR4();
    fn SEMA4_CP0();
    fn SEMA4_CP1();
    fn PWM2_0();
    fn PWM2_1();
    fn PWM2_2();
    fn PWM2_3();
    fn PWM2_FAULT();
    fn PWM3_0();
    fn PWM3_1();
    fn PWM3_2();
    fn PWM3_3();
    fn PWM3_FAULT();
    fn PWM4_0();
    fn PWM4_1();
    fn PWM4_2();
    fn PWM4_3();
    fn PWM4_FAULT();
    fn Reserved208();
    fn Reserved209();
    fn Reserved210();
    fn Reserved211();
    fn Reserved212();
    fn Reserved213();
    fn Reserved214();
    fn Reserved215();
    fn PDM_HWVAD_EVENT();
    fn PDM_HWVAD_ERROR();
    fn PDM_EVENT();
    fn PDM_ERROR();
    fn EMVSIM1();
    fn EMVSIM2();
    fn MECC1_INT();
    fn MECC1_FATAL_INT();
    fn MECC2_INT();
    fn MECC2_FATAL_INT();
    fn XECC_FLEXSPI1_INT();
    fn XECC_FLEXSPI1_FATAL_INT();
    fn XECC_FLEXSPI2_INT();
    fn XECC_FLEXSPI2_FATAL_INT();
    fn XECC_SEMC_INT();
    fn XECC_SEMC_FATAL_INT();
    fn ENET_QOS();
    fn ENET_QOS_PMT();
}

#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

#[cfg(feature = "rt")]
#[doc(hidden)]
#[cfg_attr(target_arch = "arm", link_section = ".vector_table.interrupts")]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 218] = [
    Vector {
        _handler: DMA0_DMA16,
    },
    Vector {
        _handler: DMA1_DMA17,
    },
    Vector {
        _handler: DMA2_DMA18,
    },
    Vector {
        _handler: DMA3_DMA19,
    },
    Vector {
        _handler: DMA4_DMA20,
    },
    Vector {
        _handler: DMA5_DMA21,
    },
    Vector {
        _handler: DMA6_DMA22,
    },
    Vector {
        _handler: DMA7_DMA23,
    },
    Vector {
        _handler: DMA8_DMA24,
    },
    Vector {
        _handler: DMA9_DMA25,
    },
    Vector {
        _handler: DMA10_DMA26,
    },
    Vector {
        _handler: DMA11_DMA27,
    },
    Vector {
        _handler: DMA12_DMA28,
    },
    Vector {
        _handler: DMA13_DMA29,
    },
    Vector {
        _handler: DMA14_DMA30,
    },
    Vector {
        _handler: DMA15_DMA31,
    },
    Vector {
        _handler: DMA_ERROR,
    },
    Vector {
        _handler: Reserved33,
    },
    Vector {
        _handler: Reserved34,
    },
    Vector { _handler: CORE },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _handler: LPUART5 },
    Vector { _handler: LPUART6 },
    Vector { _handler: LPUART7 },
    Vector { _handler: LPUART8 },
    Vector { _handler: LPUART9 },
    Vector { _handler: LPUART10 },
    Vector { _handler: LPUART11 },
    Vector { _handler: LPUART12 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: LPI2C3 },
    Vector { _handler: LPI2C4 },
    Vector { _handler: LPI2C5 },
    Vector { _handler: LPI2C6 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: LPSPI3 },
    Vector { _handler: LPSPI4 },
    Vector { _handler: LPSPI5 },
    Vector { _handler: LPSPI6 },
    Vector { _handler: CAN1 },
    Vector {
        _handler: CAN1_ERROR,
    },
    Vector { _handler: CAN2 },
    Vector {
        _handler: CAN2_ERROR,
    },
    Vector { _handler: CAN3 },
    Vector {
        _handler: CAN3_ERROR,
    },
    Vector {
        _handler: Reserved66,
    },
    Vector { _handler: KPP },
    Vector {
        _handler: Reserved68,
    },
    Vector { _handler: GPR_IRQ },
    Vector { _handler: eLCDIF },
    Vector { _handler: LCDIFv2 },
    Vector { _handler: CSI },
    Vector { _handler: PXP },
    Vector { _handler: MIPI_CSI },
    Vector { _handler: MIPI_DSI },
    Vector { _handler: GPU2D },
    Vector {
        _handler: GPIO12_Combined_0_15,
    },
    Vector {
        _handler: GPIO12_Combined_16_31,
    },
    Vector { _handler: DAC },
    Vector {
        _handler: KEY_MANAGER,
    },
    Vector { _handler: WDOG2 },
    Vector {
        _handler: SNVS_HP_NON_TZ,
    },
    Vector {
        _handler: SNVS_HP_TZ,
    },
    Vector {
        _handler: SNVS_PULSE_EVENT,
    },
    Vector {
        _handler: CAAM_IRQ0,
    },
    Vector {
        _handler: CAAM_IRQ1,
    },
    Vector {
        _handler: CAAM_IRQ2,
    },
    Vector {
        _handler: CAAM_IRQ3,
    },
    Vector {
        _handler: CAAM_RECORVE_ERRPR,
    },
    Vector {
        _handler: CAAM_RTIC,
    },
    Vector { _handler: CDOG },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: SAI3_RX },
    Vector { _handler: SAI3_TX },
    Vector { _handler: SAI4_RX },
    Vector { _handler: SAI4_TX },
    Vector { _handler: SPDIF },
    Vector {
        _handler: TMPSNS_INT,
    },
    Vector {
        _handler: TMPSNS_LOW_HIGH,
    },
    Vector {
        _handler: TMPSNS_PANIC,
    },
    Vector {
        _handler: LPSR_LP8_BROWNOUT,
    },
    Vector {
        _handler: LPSR_LP0_BROWNOUT,
    },
    Vector { _handler: ADC1 },
    Vector { _handler: ADC2 },
    Vector { _handler: USBPHY1 },
    Vector { _handler: USBPHY2 },
    Vector { _handler: RDC },
    Vector {
        _handler: GPIO13_Combined_0_31,
    },
    Vector { _reserved: 0 },
    Vector { _handler: DCIC1 },
    Vector { _handler: DCIC2 },
    Vector { _handler: ASRC },
    Vector {
        _handler: FLEXRAM_ECC,
    },
    Vector {
        _handler: GPIO7_8_9_10_11,
    },
    Vector {
        _handler: GPIO1_Combined_0_15,
    },
    Vector {
        _handler: GPIO1_Combined_16_31,
    },
    Vector {
        _handler: GPIO2_Combined_0_15,
    },
    Vector {
        _handler: GPIO2_Combined_16_31,
    },
    Vector {
        _handler: GPIO3_Combined_0_15,
    },
    Vector {
        _handler: GPIO3_Combined_16_31,
    },
    Vector {
        _handler: GPIO4_Combined_0_15,
    },
    Vector {
        _handler: GPIO4_Combined_16_31,
    },
    Vector {
        _handler: GPIO5_Combined_0_15,
    },
    Vector {
        _handler: GPIO5_Combined_16_31,
    },
    Vector { _handler: FLEXIO1 },
    Vector { _handler: FLEXIO2 },
    Vector { _handler: WDOG1 },
    Vector { _handler: RTWDOG4 },
    Vector { _handler: EWM },
    Vector {
        _handler: OCOTP_READ_FUSE_ERROR,
    },
    Vector {
        _handler: OCOTP_READ_DONE_ERROR,
    },
    Vector { _handler: GPC },
    Vector { _handler: MUB },
    Vector { _handler: GPT1 },
    Vector { _handler: GPT2 },
    Vector { _handler: GPT3 },
    Vector { _handler: GPT4 },
    Vector { _handler: GPT5 },
    Vector { _handler: GPT6 },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector {
        _handler: PWM1_FAULT,
    },
    Vector { _handler: FLEXSPI1 },
    Vector { _handler: FLEXSPI2 },
    Vector { _handler: SEMC },
    Vector { _handler: USDHC1 },
    Vector { _handler: USDHC2 },
    Vector { _handler: USB_OTG2 },
    Vector { _handler: USB_OTG1 },
    Vector { _handler: ENET },
    Vector {
        _handler: ENET_1588_Timer,
    },
    Vector {
        _handler: ENET_1G_MAC0_Tx_Rx_1,
    },
    Vector {
        _handler: ENET_1G_MAC0_Tx_Rx_2,
    },
    Vector { _handler: ENET_1G },
    Vector {
        _handler: ENET_1G_1588_Timer,
    },
    Vector {
        _handler: XBAR1_IRQ_0_1,
    },
    Vector {
        _handler: XBAR1_IRQ_2_3,
    },
    Vector {
        _handler: ADC_ETC_IRQ0,
    },
    Vector {
        _handler: ADC_ETC_IRQ1,
    },
    Vector {
        _handler: ADC_ETC_IRQ2,
    },
    Vector {
        _handler: ADC_ETC_IRQ3,
    },
    Vector {
        _handler: ADC_ETC_ERROR_IRQ,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PIT1 },
    Vector { _handler: PIT2 },
    Vector { _handler: ACMP1 },
    Vector { _handler: ACMP2 },
    Vector { _handler: ACMP3 },
    Vector { _handler: ACMP4 },
    Vector {
        _handler: Reserved177,
    },
    Vector {
        _handler: Reserved178,
    },
    Vector {
        _handler: Reserved179,
    },
    Vector {
        _handler: Reserved180,
    },
    Vector { _handler: ENC1 },
    Vector { _handler: ENC2 },
    Vector { _handler: ENC3 },
    Vector { _handler: ENC4 },
    Vector {
        _handler: Reserved185,
    },
    Vector {
        _handler: Reserved186,
    },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR2 },
    Vector { _handler: TMR3 },
    Vector { _handler: TMR4 },
    Vector {
        _handler: SEMA4_CP0,
    },
    Vector {
        _handler: SEMA4_CP1,
    },
    Vector { _handler: PWM2_0 },
    Vector { _handler: PWM2_1 },
    Vector { _handler: PWM2_2 },
    Vector { _handler: PWM2_3 },
    Vector {
        _handler: PWM2_FAULT,
    },
    Vector { _handler: PWM3_0 },
    Vector { _handler: PWM3_1 },
    Vector { _handler: PWM3_2 },
    Vector { _handler: PWM3_3 },
    Vector {
        _handler: PWM3_FAULT,
    },
    Vector { _handler: PWM4_0 },
    Vector { _handler: PWM4_1 },
    Vector { _handler: PWM4_2 },
    Vector { _handler: PWM4_3 },
    Vector {
        _handler: PWM4_FAULT,
    },
    Vector {
        _handler: Reserved208,
    },
    Vector {
        _handler: Reserved209,
    },
    Vector {
        _handler: Reserved210,
    },
    Vector {
        _handler: Reserved211,
    },
    Vector {
        _handler: Reserved212,
    },
    Vector {
        _handler: Reserved213,
    },
    Vector {
        _handler: Reserved214,
    },
    Vector {
        _handler: Reserved215,
    },
    Vector {
        _handler: PDM_HWVAD_EVENT,
    },
    Vector {
        _handler: PDM_HWVAD_ERROR,
    },
    Vector {
        _handler: PDM_EVENT,
    },
    Vector {
        _handler: PDM_ERROR,
    },
    Vector { _handler: EMVSIM1 },
    Vector { _handler: EMVSIM2 },
    Vector {
        _handler: MECC1_INT,
    },
    Vector {
        _handler: MECC1_FATAL_INT,
    },
    Vector {
        _handler: MECC2_INT,
    },
    Vector {
        _handler: MECC2_FATAL_INT,
    },
    Vector {
        _handler: XECC_FLEXSPI1_INT,
    },
    Vector {
        _handler: XECC_FLEXSPI1_FATAL_INT,
    },
    Vector {
        _handler: XECC_FLEXSPI2_INT,
    },
    Vector {
        _handler: XECC_FLEXSPI2_FATAL_INT,
    },
    Vector {
        _handler: XECC_SEMC_INT,
    },
    Vector {
        _handler: XECC_SEMC_FATAL_INT,
    },
    Vector { _handler: ENET_QOS },
    Vector {
        _handler: ENET_QOS_PMT,
    },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0:
    DMA0_DMA16 = 0,
    /// 1:
    DMA1_DMA17 = 1,
    /// 2:
    DMA2_DMA18 = 2,
    /// 3:
    DMA3_DMA19 = 3,
    /// 4:
    DMA4_DMA20 = 4,
    /// 5:
    DMA5_DMA21 = 5,
    /// 6:
    DMA6_DMA22 = 6,
    /// 7:
    DMA7_DMA23 = 7,
    /// 8:
    DMA8_DMA24 = 8,
    /// 9:
    DMA9_DMA25 = 9,
    /// 10:
    DMA10_DMA26 = 10,
    /// 11:
    DMA11_DMA27 = 11,
    /// 12:
    DMA12_DMA28 = 12,
    /// 13:
    DMA13_DMA29 = 13,
    /// 14:
    DMA14_DMA30 = 14,
    /// 15:
    DMA15_DMA31 = 15,
    /// 16:
    DMA_ERROR = 16,
    /// 17:
    Reserved33 = 17,
    /// 18:
    Reserved34 = 18,
    /// 19:
    CORE = 19,
    /// 20:
    LPUART1 = 20,
    /// 21:
    LPUART2 = 21,
    /// 22:
    LPUART3 = 22,
    /// 23:
    LPUART4 = 23,
    /// 24:
    LPUART5 = 24,
    /// 25:
    LPUART6 = 25,
    /// 26:
    LPUART7 = 26,
    /// 27:
    LPUART8 = 27,
    /// 28:
    LPUART9 = 28,
    /// 29:
    LPUART10 = 29,
    /// 30:
    LPUART11 = 30,
    /// 31:
    LPUART12 = 31,
    /// 32:
    LPI2C1 = 32,
    /// 33:
    LPI2C2 = 33,
    /// 34:
    LPI2C3 = 34,
    /// 35:
    LPI2C4 = 35,
    /// 36:
    LPI2C5 = 36,
    /// 37:
    LPI2C6 = 37,
    /// 38:
    LPSPI1 = 38,
    /// 39:
    LPSPI2 = 39,
    /// 40:
    LPSPI3 = 40,
    /// 41:
    LPSPI4 = 41,
    /// 42:
    LPSPI5 = 42,
    /// 43:
    LPSPI6 = 43,
    /// 44:
    CAN1 = 44,
    /// 45:
    CAN1_ERROR = 45,
    /// 46:
    CAN2 = 46,
    /// 47:
    CAN2_ERROR = 47,
    /// 48:
    CAN3 = 48,
    /// 49:
    CAN3_ERROR = 49,
    /// 50:
    Reserved66 = 50,
    /// 51:
    KPP = 51,
    /// 52:
    Reserved68 = 52,
    /// 53:
    GPR_IRQ = 53,
    /// 54:
    eLCDIF = 54,
    /// 55:
    LCDIFv2 = 55,
    /// 56:
    CSI = 56,
    /// 57:
    PXP = 57,
    /// 58:
    MIPI_CSI = 58,
    /// 59:
    MIPI_DSI = 59,
    /// 60:
    GPU2D = 60,
    /// 61:
    GPIO12_Combined_0_15 = 61,
    /// 62:
    GPIO12_Combined_16_31 = 62,
    /// 63:
    DAC = 63,
    /// 64:
    KEY_MANAGER = 64,
    /// 65:
    WDOG2 = 65,
    /// 66:
    SNVS_HP_NON_TZ = 66,
    /// 67:
    SNVS_HP_TZ = 67,
    /// 68:
    SNVS_PULSE_EVENT = 68,
    /// 69:
    CAAM_IRQ0 = 69,
    /// 70:
    CAAM_IRQ1 = 70,
    /// 71:
    CAAM_IRQ2 = 71,
    /// 72:
    CAAM_IRQ3 = 72,
    /// 73:
    CAAM_RECORVE_ERRPR = 73,
    /// 74:
    CAAM_RTIC = 74,
    /// 75:
    CDOG = 75,
    /// 76:
    SAI1 = 76,
    /// 77:
    SAI2 = 77,
    /// 78:
    SAI3_RX = 78,
    /// 79:
    SAI3_TX = 79,
    /// 80:
    SAI4_RX = 80,
    /// 81:
    SAI4_TX = 81,
    /// 82:
    SPDIF = 82,
    /// 83:
    TMPSNS_INT = 83,
    /// 84:
    TMPSNS_LOW_HIGH = 84,
    /// 85:
    TMPSNS_PANIC = 85,
    /// 86:
    LPSR_LP8_BROWNOUT = 86,
    /// 87:
    LPSR_LP0_BROWNOUT = 87,
    /// 88:
    ADC1 = 88,
    /// 89:
    ADC2 = 89,
    /// 90:
    USBPHY1 = 90,
    /// 91:
    USBPHY2 = 91,
    /// 92:
    RDC = 92,
    /// 93:
    GPIO13_Combined_0_31 = 93,
    /// 95:
    DCIC1 = 95,
    /// 96:
    DCIC2 = 96,
    /// 97:
    ASRC = 97,
    /// 98:
    FLEXRAM_ECC = 98,
    /// 99:
    GPIO7_8_9_10_11 = 99,
    /// 100:
    GPIO1_Combined_0_15 = 100,
    /// 101:
    GPIO1_Combined_16_31 = 101,
    /// 102:
    GPIO2_Combined_0_15 = 102,
    /// 103:
    GPIO2_Combined_16_31 = 103,
    /// 104:
    GPIO3_Combined_0_15 = 104,
    /// 105:
    GPIO3_Combined_16_31 = 105,
    /// 106:
    GPIO4_Combined_0_15 = 106,
    /// 107:
    GPIO4_Combined_16_31 = 107,
    /// 108:
    GPIO5_Combined_0_15 = 108,
    /// 109:
    GPIO5_Combined_16_31 = 109,
    /// 110:
    FLEXIO1 = 110,
    /// 111:
    FLEXIO2 = 111,
    /// 112:
    WDOG1 = 112,
    /// 113:
    RTWDOG4 = 113,
    /// 114:
    EWM = 114,
    /// 115:
    OCOTP_READ_FUSE_ERROR = 115,
    /// 116:
    OCOTP_READ_DONE_ERROR = 116,
    /// 117:
    GPC = 117,
    /// 118:
    MUB = 118,
    /// 119:
    GPT1 = 119,
    /// 120:
    GPT2 = 120,
    /// 121:
    GPT3 = 121,
    /// 122:
    GPT4 = 122,
    /// 123:
    GPT5 = 123,
    /// 124:
    GPT6 = 124,
    /// 125:
    PWM1_0 = 125,
    /// 126:
    PWM1_1 = 126,
    /// 127:
    PWM1_2 = 127,
    /// 128:
    PWM1_3 = 128,
    /// 129:
    PWM1_FAULT = 129,
    /// 130:
    FLEXSPI1 = 130,
    /// 131:
    FLEXSPI2 = 131,
    /// 132:
    SEMC = 132,
    /// 133:
    USDHC1 = 133,
    /// 134:
    USDHC2 = 134,
    /// 135:
    USB_OTG2 = 135,
    /// 136:
    USB_OTG1 = 136,
    /// 137:
    ENET = 137,
    /// 138:
    ENET_1588_Timer = 138,
    /// 139:
    ENET_1G_MAC0_Tx_Rx_1 = 139,
    /// 140:
    ENET_1G_MAC0_Tx_Rx_2 = 140,
    /// 141:
    ENET_1G = 141,
    /// 142:
    ENET_1G_1588_Timer = 142,
    /// 143:
    XBAR1_IRQ_0_1 = 143,
    /// 144:
    XBAR1_IRQ_2_3 = 144,
    /// 145:
    ADC_ETC_IRQ0 = 145,
    /// 146:
    ADC_ETC_IRQ1 = 146,
    /// 147:
    ADC_ETC_IRQ2 = 147,
    /// 148:
    ADC_ETC_IRQ3 = 148,
    /// 149:
    ADC_ETC_ERROR_IRQ = 149,
    /// 155:
    PIT1 = 155,
    /// 156:
    PIT2 = 156,
    /// 157:
    ACMP1 = 157,
    /// 158:
    ACMP2 = 158,
    /// 159:
    ACMP3 = 159,
    /// 160:
    ACMP4 = 160,
    /// 161:
    Reserved177 = 161,
    /// 162:
    Reserved178 = 162,
    /// 163:
    Reserved179 = 163,
    /// 164:
    Reserved180 = 164,
    /// 165:
    ENC1 = 165,
    /// 166:
    ENC2 = 166,
    /// 167:
    ENC3 = 167,
    /// 168:
    ENC4 = 168,
    /// 169:
    Reserved185 = 169,
    /// 170:
    Reserved186 = 170,
    /// 171:
    TMR1 = 171,
    /// 172:
    TMR2 = 172,
    /// 173:
    TMR3 = 173,
    /// 174:
    TMR4 = 174,
    /// 175:
    SEMA4_CP0 = 175,
    /// 176:
    SEMA4_CP1 = 176,
    /// 177:
    PWM2_0 = 177,
    /// 178:
    PWM2_1 = 178,
    /// 179:
    PWM2_2 = 179,
    /// 180:
    PWM2_3 = 180,
    /// 181:
    PWM2_FAULT = 181,
    /// 182:
    PWM3_0 = 182,
    /// 183:
    PWM3_1 = 183,
    /// 184:
    PWM3_2 = 184,
    /// 185:
    PWM3_3 = 185,
    /// 186:
    PWM3_FAULT = 186,
    /// 187:
    PWM4_0 = 187,
    /// 188:
    PWM4_1 = 188,
    /// 189:
    PWM4_2 = 189,
    /// 190:
    PWM4_3 = 190,
    /// 191:
    PWM4_FAULT = 191,
    /// 192:
    Reserved208 = 192,
    /// 193:
    Reserved209 = 193,
    /// 194:
    Reserved210 = 194,
    /// 195:
    Reserved211 = 195,
    /// 196:
    Reserved212 = 196,
    /// 197:
    Reserved213 = 197,
    /// 198:
    Reserved214 = 198,
    /// 199:
    Reserved215 = 199,
    /// 200:
    PDM_HWVAD_EVENT = 200,
    /// 201:
    PDM_HWVAD_ERROR = 201,
    /// 202:
    PDM_EVENT = 202,
    /// 203:
    PDM_ERROR = 203,
    /// 204:
    EMVSIM1 = 204,
    /// 205:
    EMVSIM2 = 205,
    /// 206:
    MECC1_INT = 206,
    /// 207:
    MECC1_FATAL_INT = 207,
    /// 208:
    MECC2_INT = 208,
    /// 209:
    MECC2_FATAL_INT = 209,
    /// 210:
    XECC_FLEXSPI1_INT = 210,
    /// 211:
    XECC_FLEXSPI1_FATAL_INT = 211,
    /// 212:
    XECC_FLEXSPI2_INT = 212,
    /// 213:
    XECC_FLEXSPI2_FATAL_INT = 213,
    /// 214:
    XECC_SEMC_INT = 214,
    /// 215:
    XECC_SEMC_FATAL_INT = 215,
    /// 216:
    ENET_QOS = 216,
    /// 217:
    ENET_QOS_PMT = 217,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline]
    fn number(self) -> u16 {
        self as u16
    }
}
