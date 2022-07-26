//! Register access layer for imxrt1176_cm4

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::mecc;
pub use super::instances::osc_rc_400m;
pub use super::instances::phy_ldo;
pub use super::instances::tmpsns;
pub use super::instances::vmbandgap;
pub use super::instances::xecc;
pub mod flexram;
pub use super::instances::ewm;
pub use super::instances::wdog;
pub mod rtwdog;
pub use super::instances::adc_etc;
pub use super::instances::aoi;
pub use super::instances::asrc;
pub use super::instances::caam;
pub use super::instances::can;
pub use super::instances::can_wrapper;
pub use super::instances::ccm_obs;
pub use super::instances::cmp;
pub use super::instances::csi;
pub use super::instances::dac;
pub use super::instances::dcic;
pub use super::instances::dsi_host;
pub use super::instances::dsi_host_apb_pkt_if;
pub use super::instances::dsi_host_dphy_intfc;
pub use super::instances::dsi_host_dpi_intfc;
pub use super::instances::emvsim;
pub use super::instances::enc;
pub use super::instances::enet;
pub use super::instances::enet_1g;
pub use super::instances::enet_qos;
pub use super::instances::flexio;
pub use super::instances::flexspi;
pub use super::instances::gpc_cpu_mode_ctrl_;
pub use super::instances::gpc_set_point_ctrl;
pub use super::instances::gpc_stby_ctrl;
pub use super::instances::gpt;
pub use super::instances::iee__iee_rt1170;
pub use super::instances::iee_apc;
pub use super::instances::iomuxc;
pub use super::instances::iomuxc_gpr;
pub use super::instances::iomuxc_lpsr;
pub use super::instances::iomuxc_lpsr_gpr;
pub use super::instances::kpp;
pub use super::instances::lcdif;
pub use super::instances::lcdifv2;
pub use super::instances::lpadc;
pub use super::instances::lpi2c;
pub use super::instances::lpspi;
pub use super::instances::lpuart;
pub use super::instances::mipi_csi2rx;
pub use super::instances::otfad;
pub use super::instances::pit;
pub use super::instances::pwm;
pub use super::instances::pxp;
pub use super::instances::sai;
pub use super::instances::sai1;
pub use super::instances::semc;
pub use super::instances::spdif;
pub use super::instances::src;
pub use super::instances::tmr;
pub use super::instances::usb_otg;
pub use super::instances::usbhsdcd;
pub use super::instances::usbnc_otg;
pub use super::instances::usbphy;
pub use super::instances::usdhc;
pub use super::instances::video_mux;
pub use super::instances::xbara1;
pub use super::instances::xbarb;
pub mod dma1;
pub mod dmamux1;
pub use super::instances::pdm;
pub use super::instances::rdc_semaphore;
pub mod mub;
pub use super::instances::anadig_ldo_snvs;
pub use super::instances::anadig_ldo_snvs_dig;
pub use super::instances::anadig_misc;
pub use super::instances::anadig_osc;
pub use super::instances::anadig_pll;
pub use super::instances::anadig_pmu;
pub use super::instances::anadig_tempsensor;
pub use super::instances::ccm;
pub use super::instances::cdog;
pub use super::instances::dcdc;
pub use super::instances::iomuxc_snvs;
pub use super::instances::iomuxc_snvs_gpr;
pub use super::instances::ips_domain;
pub use super::instances::key_manager;
pub use super::instances::key_manager__puf;
pub use super::instances::ocotp;
pub use super::instances::pgmc_bpc;
pub use super::instances::pgmc_cpc;
pub use super::instances::pgmc_cpc0_mif;
pub use super::instances::pgmc_ppc0;
pub use super::instances::rdc;
pub use super::instances::sema4;
pub use super::instances::snvs;
pub use super::instances::sram;
pub use super::instances::ssarc_hp;
pub use super::instances::ssarc_lp;
pub use super::instances::xrdc2_d;
pub mod cm7_gpio;
pub mod gpio;
pub mod lmem;
pub mod mcm;
pub mod mmcau;
pub mod nvic;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub OSC_RC_400M: osc_rc_400m::Instance<0>,
    pub PHY_LDO: phy_ldo::Instance<0>,
    pub TMPSNS: tmpsns::Instance<0>,
    pub VMBANDGAP: vmbandgap::Instance<0>,
    pub MECC1: mecc::Instance<1>,
    pub MECC2: mecc::Instance<2>,
    pub XECC_FLEXSPI1: xecc::Instance<1>,
    pub XECC_FLEXSPI2: xecc::Instance<2>,
    pub XECC_SEMC: xecc::Instance<0>,
    pub FLEXRAM: flexram::Instance<0>,
    pub EWM: ewm::Instance<0>,
    pub WDOG1: wdog::Instance<1>,
    pub WDOG2: wdog::Instance<2>,
    pub RTWDOG3: rtwdog::Instance<3>,
    pub RTWDOG4: rtwdog::Instance<4>,
    pub XBARA1: xbara1::Instance<0>,
    pub XBARB2: xbarb::Instance<2>,
    pub XBARB3: xbarb::Instance<3>,
    pub ADC_ETC: adc_etc::Instance<0>,
    pub LPADC1: lpadc::Instance<1>,
    pub LPADC2: lpadc::Instance<2>,
    pub DAC: dac::Instance<0>,
    pub IEE_APC: iee_apc::Instance<0>,
    pub IEE__IEE_RT1170: iee__iee_rt1170::Instance<0>,
    pub LPUART1: lpuart::Instance<1>,
    pub LPUART2: lpuart::Instance<2>,
    pub LPUART3: lpuart::Instance<3>,
    pub LPUART4: lpuart::Instance<4>,
    pub LPUART5: lpuart::Instance<5>,
    pub LPUART6: lpuart::Instance<6>,
    pub LPUART7: lpuart::Instance<7>,
    pub LPUART8: lpuart::Instance<8>,
    pub LPUART9: lpuart::Instance<9>,
    pub LPUART10: lpuart::Instance<10>,
    pub LPUART11: lpuart::Instance<11>,
    pub LPUART12: lpuart::Instance<12>,
    pub FLEXIO1: flexio::Instance<1>,
    pub FLEXIO2: flexio::Instance<2>,
    pub AOI1: aoi::Instance<1>,
    pub AOI2: aoi::Instance<2>,
    pub CAN1: can::Instance<1>,
    pub CAN2: can::Instance<2>,
    pub CAN3: can::Instance<3>,
    pub CAN1_WRAPPER: can_wrapper::Instance<1>,
    pub CAN2_WRAPPER: can_wrapper::Instance<2>,
    pub CAN3_WRAPPER: can_wrapper::Instance<3>,
    pub FLEXSPI1: flexspi::Instance<1>,
    pub FLEXSPI2: flexspi::Instance<2>,
    pub OTFAD1: otfad::Instance<1>,
    pub OTFAD2: otfad::Instance<2>,
    pub SEMC: semc::Instance<0>,
    pub PIT1: pit::Instance<1>,
    pub PIT2: pit::Instance<2>,
    pub KPP: kpp::Instance<0>,
    pub IOMUXC_GPR: iomuxc_gpr::Instance<0>,
    pub IOMUXC: iomuxc::Instance<0>,
    pub GPT1: gpt::Instance<1>,
    pub GPT2: gpt::Instance<2>,
    pub GPT3: gpt::Instance<3>,
    pub GPT4: gpt::Instance<4>,
    pub GPT5: gpt::Instance<5>,
    pub GPT6: gpt::Instance<6>,
    pub LPI2C1: lpi2c::Instance<1>,
    pub LPI2C2: lpi2c::Instance<2>,
    pub LPI2C3: lpi2c::Instance<3>,
    pub LPI2C4: lpi2c::Instance<4>,
    pub LPI2C5: lpi2c::Instance<5>,
    pub LPI2C6: lpi2c::Instance<6>,
    pub LPSPI1: lpspi::Instance<1>,
    pub LPSPI2: lpspi::Instance<2>,
    pub LPSPI3: lpspi::Instance<3>,
    pub LPSPI4: lpspi::Instance<4>,
    pub LPSPI5: lpspi::Instance<5>,
    pub LPSPI6: lpspi::Instance<6>,
    pub CCM_OBS: ccm_obs::Instance<0>,
    pub EMVSIM1: emvsim::Instance<1>,
    pub EMVSIM2: emvsim::Instance<2>,
    pub TMR1: tmr::Instance<1>,
    pub TMR2: tmr::Instance<2>,
    pub TMR3: tmr::Instance<3>,
    pub TMR4: tmr::Instance<4>,
    pub ENC1: enc::Instance<1>,
    pub ENC2: enc::Instance<2>,
    pub ENC3: enc::Instance<3>,
    pub ENC4: enc::Instance<4>,
    pub PWM1: pwm::Instance<1>,
    pub PWM2: pwm::Instance<2>,
    pub PWM3: pwm::Instance<3>,
    pub PWM4: pwm::Instance<4>,
    pub CMP1: cmp::Instance<1>,
    pub CMP2: cmp::Instance<2>,
    pub CMP3: cmp::Instance<3>,
    pub CMP4: cmp::Instance<4>,
    pub SPDIF: spdif::Instance<0>,
    pub SAI1: sai1::Instance<0>,
    pub SAI2: sai::Instance<2>,
    pub SAI3: sai::Instance<3>,
    pub SAI4: sai::Instance<4>,
    pub ASRC: asrc::Instance<0>,
    pub USDHC1: usdhc::Instance<1>,
    pub USDHC2: usdhc::Instance<2>,
    pub ENET_1G: enet_1g::Instance<0>,
    pub ENET: enet::Instance<0>,
    pub USB_OTG1: usb_otg::Instance<1>,
    pub USB_OTG2: usb_otg::Instance<2>,
    pub USBNC_OTG1: usbnc_otg::Instance<1>,
    pub USBNC_OTG2: usbnc_otg::Instance<2>,
    pub USBPHY1: usbphy::Instance<1>,
    pub USBPHY2: usbphy::Instance<2>,
    pub USBHSDCD1: usbhsdcd::Instance<1>,
    pub USBHSDCD2: usbhsdcd::Instance<2>,
    pub ENET_QOS: enet_qos::Instance<0>,
    pub CAAM: caam::Instance<0>,
    pub CSI: csi::Instance<0>,
    pub LCDIF: lcdif::Instance<0>,
    pub LCDIFV2: lcdifv2::Instance<0>,
    pub DSI_HOST: dsi_host::Instance<0>,
    pub DSI_HOST_DPI_INTFC: dsi_host_dpi_intfc::Instance<0>,
    pub DSI_HOST_APB_PKT_IF: dsi_host_apb_pkt_if::Instance<0>,
    pub DSI_HOST_DPHY_INTFC: dsi_host_dphy_intfc::Instance<0>,
    pub MIPI_CSI2RX: mipi_csi2rx::Instance<0>,
    pub PXP: pxp::Instance<0>,
    pub VIDEO_MUX: video_mux::Instance<0>,
    pub DCIC1: dcic::Instance<1>,
    pub DCIC2: dcic::Instance<2>,
    pub GPC_CPU_MODE_CTRL_0: gpc_cpu_mode_ctrl_::Instance<0>,
    pub GPC_CPU_MODE_CTRL_1: gpc_cpu_mode_ctrl_::Instance<1>,
    pub GPC_SET_POINT_CTRL: gpc_set_point_ctrl::Instance<0>,
    pub GPC_STBY_CTRL: gpc_stby_ctrl::Instance<0>,
    pub SRC: src::Instance<0>,
    pub IOMUXC_LPSR: iomuxc_lpsr::Instance<0>,
    pub IOMUXC_LPSR_GPR: iomuxc_lpsr_gpr::Instance<0>,
    pub DMA1: dma1::Instance<0>,
    pub DMAMUX1: dmamux1::Instance<0>,
    pub PDM: pdm::Instance<0>,
    pub RDC_SEMAPHORE1: rdc_semaphore::Instance<1>,
    pub RDC_SEMAPHORE2: rdc_semaphore::Instance<2>,
    pub MUB: mub::Instance<0>,
    pub RDC: rdc::Instance<0>,
    pub KEY_MANAGER: key_manager::Instance<0>,
    pub KEY_MANAGER__PUF: key_manager__puf::Instance<0>,
    pub ANADIG_LDO_SNVS: anadig_ldo_snvs::Instance<0>,
    pub ANADIG_LDO_SNVS_DIG: anadig_ldo_snvs_dig::Instance<0>,
    pub ANADIG_MISC: anadig_misc::Instance<0>,
    pub ANADIG_OSC: anadig_osc::Instance<0>,
    pub ANADIG_PLL: anadig_pll::Instance<0>,
    pub ANADIG_PMU: anadig_pmu::Instance<0>,
    pub ANADIG_TEMPSENSOR: anadig_tempsensor::Instance<0>,
    pub IPS_DOMAIN: ips_domain::Instance<0>,
    pub PGMC_BPC0: pgmc_bpc::Instance<1>,
    pub PGMC_BPC1: pgmc_bpc::Instance<2>,
    pub PGMC_BPC2: pgmc_bpc::Instance<3>,
    pub PGMC_BPC3: pgmc_bpc::Instance<4>,
    pub PGMC_BPC4: pgmc_bpc::Instance<5>,
    pub PGMC_BPC5: pgmc_bpc::Instance<6>,
    pub PGMC_BPC6: pgmc_bpc::Instance<7>,
    pub PGMC_BPC7: pgmc_bpc::Instance<8>,
    pub PGMC_CPC0: pgmc_cpc::Instance<1>,
    pub PGMC_CPC1: pgmc_cpc::Instance<2>,
    pub PGMC_CPC0_MIF0: pgmc_cpc0_mif::Instance<1>,
    pub PGMC_CPC0_MIF1: pgmc_cpc0_mif::Instance<2>,
    pub PGMC_CPC1_MIF0: pgmc_cpc0_mif::Instance<3>,
    pub PGMC_CPC1_MIF1: pgmc_cpc0_mif::Instance<4>,
    pub PGMC_PPC0: pgmc_ppc0::Instance<0>,
    pub SNVS: snvs::Instance<0>,
    pub IOMUXC_SNVS: iomuxc_snvs::Instance<0>,
    pub IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::Instance<0>,
    pub SRAM: sram::Instance<0>,
    pub DCDC: dcdc::Instance<0>,
    pub OCOTP: ocotp::Instance<0>,
    pub SSARC_HP: ssarc_hp::Instance<0>,
    pub SSARC_LP: ssarc_lp::Instance<0>,
    pub CCM: ccm::Instance<0>,
    pub SEMA4: sema4::Instance<0>,
    pub XRDC2_D0: xrdc2_d::Instance<0>,
    pub XRDC2_D1: xrdc2_d::Instance<1>,
    pub CDOG: cdog::Instance<0>,
    pub CM7_GPIO2: cm7_gpio::Instance<2>,
    pub CM7_GPIO3: cm7_gpio::Instance<3>,
    pub GPIO1: gpio::Instance<1>,
    pub GPIO2: gpio::Instance<2>,
    pub GPIO3: gpio::Instance<3>,
    pub GPIO4: gpio::Instance<4>,
    pub GPIO5: gpio::Instance<5>,
    pub GPIO6: gpio::Instance<6>,
    pub GPIO7: gpio::Instance<7>,
    pub GPIO8: gpio::Instance<8>,
    pub GPIO9: gpio::Instance<9>,
    pub GPIO10: gpio::Instance<10>,
    pub GPIO11: gpio::Instance<11>,
    pub GPIO12: gpio::Instance<12>,
    pub GPIO13: gpio::Instance<13>,
    pub NVIC: nvic::Instance<0>,
    pub MCM: mcm::Instance<0>,
    pub MMCAU: mmcau::Instance<0>,
    pub LMEM: lmem::Instance<0>,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            OSC_RC_400M: osc_rc_400m::OSC_RC_400M::steal(),
            PHY_LDO: phy_ldo::PHY_LDO::steal(),
            TMPSNS: tmpsns::TMPSNS::steal(),
            VMBANDGAP: vmbandgap::VMBANDGAP::steal(),
            MECC1: mecc::MECC1::steal(),
            MECC2: mecc::MECC2::steal(),
            XECC_FLEXSPI1: xecc::XECC_FLEXSPI1::steal(),
            XECC_FLEXSPI2: xecc::XECC_FLEXSPI2::steal(),
            XECC_SEMC: xecc::XECC_SEMC::steal(),
            FLEXRAM: flexram::FLEXRAM::steal(),
            EWM: ewm::EWM::steal(),
            WDOG1: wdog::WDOG1::steal(),
            WDOG2: wdog::WDOG2::steal(),
            RTWDOG3: rtwdog::RTWDOG3::steal(),
            RTWDOG4: rtwdog::RTWDOG4::steal(),
            XBARA1: xbara1::XBARA1::steal(),
            XBARB2: xbarb::XBARB2::steal(),
            XBARB3: xbarb::XBARB3::steal(),
            ADC_ETC: adc_etc::ADC_ETC::steal(),
            LPADC1: lpadc::LPADC1::steal(),
            LPADC2: lpadc::LPADC2::steal(),
            DAC: dac::DAC::steal(),
            IEE_APC: iee_apc::IEE_APC::steal(),
            IEE__IEE_RT1170: iee__iee_rt1170::IEE__IEE_RT1170::steal(),
            LPUART1: lpuart::LPUART1::steal(),
            LPUART2: lpuart::LPUART2::steal(),
            LPUART3: lpuart::LPUART3::steal(),
            LPUART4: lpuart::LPUART4::steal(),
            LPUART5: lpuart::LPUART5::steal(),
            LPUART6: lpuart::LPUART6::steal(),
            LPUART7: lpuart::LPUART7::steal(),
            LPUART8: lpuart::LPUART8::steal(),
            LPUART9: lpuart::LPUART9::steal(),
            LPUART10: lpuart::LPUART10::steal(),
            LPUART11: lpuart::LPUART11::steal(),
            LPUART12: lpuart::LPUART12::steal(),
            FLEXIO1: flexio::FLEXIO1::steal(),
            FLEXIO2: flexio::FLEXIO2::steal(),
            AOI1: aoi::AOI1::steal(),
            AOI2: aoi::AOI2::steal(),
            CAN1: can::CAN1::steal(),
            CAN2: can::CAN2::steal(),
            CAN3: can::CAN3::steal(),
            CAN1_WRAPPER: can_wrapper::CAN1_WRAPPER::steal(),
            CAN2_WRAPPER: can_wrapper::CAN2_WRAPPER::steal(),
            CAN3_WRAPPER: can_wrapper::CAN3_WRAPPER::steal(),
            FLEXSPI1: flexspi::FLEXSPI1::steal(),
            FLEXSPI2: flexspi::FLEXSPI2::steal(),
            OTFAD1: otfad::OTFAD1::steal(),
            OTFAD2: otfad::OTFAD2::steal(),
            SEMC: semc::SEMC::steal(),
            PIT1: pit::PIT1::steal(),
            PIT2: pit::PIT2::steal(),
            KPP: kpp::KPP::steal(),
            IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR::steal(),
            IOMUXC: iomuxc::IOMUXC::steal(),
            GPT1: gpt::GPT1::steal(),
            GPT2: gpt::GPT2::steal(),
            GPT3: gpt::GPT3::steal(),
            GPT4: gpt::GPT4::steal(),
            GPT5: gpt::GPT5::steal(),
            GPT6: gpt::GPT6::steal(),
            LPI2C1: lpi2c::LPI2C1::steal(),
            LPI2C2: lpi2c::LPI2C2::steal(),
            LPI2C3: lpi2c::LPI2C3::steal(),
            LPI2C4: lpi2c::LPI2C4::steal(),
            LPI2C5: lpi2c::LPI2C5::steal(),
            LPI2C6: lpi2c::LPI2C6::steal(),
            LPSPI1: lpspi::LPSPI1::steal(),
            LPSPI2: lpspi::LPSPI2::steal(),
            LPSPI3: lpspi::LPSPI3::steal(),
            LPSPI4: lpspi::LPSPI4::steal(),
            LPSPI5: lpspi::LPSPI5::steal(),
            LPSPI6: lpspi::LPSPI6::steal(),
            CCM_OBS: ccm_obs::CCM_OBS::steal(),
            EMVSIM1: emvsim::EMVSIM1::steal(),
            EMVSIM2: emvsim::EMVSIM2::steal(),
            TMR1: tmr::TMR1::steal(),
            TMR2: tmr::TMR2::steal(),
            TMR3: tmr::TMR3::steal(),
            TMR4: tmr::TMR4::steal(),
            ENC1: enc::ENC1::steal(),
            ENC2: enc::ENC2::steal(),
            ENC3: enc::ENC3::steal(),
            ENC4: enc::ENC4::steal(),
            PWM1: pwm::PWM1::steal(),
            PWM2: pwm::PWM2::steal(),
            PWM3: pwm::PWM3::steal(),
            PWM4: pwm::PWM4::steal(),
            CMP1: cmp::CMP1::steal(),
            CMP2: cmp::CMP2::steal(),
            CMP3: cmp::CMP3::steal(),
            CMP4: cmp::CMP4::steal(),
            SPDIF: spdif::SPDIF::steal(),
            SAI1: sai1::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            SAI3: sai::SAI3::steal(),
            SAI4: sai::SAI4::steal(),
            ASRC: asrc::ASRC::steal(),
            USDHC1: usdhc::USDHC1::steal(),
            USDHC2: usdhc::USDHC2::steal(),
            ENET_1G: enet_1g::ENET_1G::steal(),
            ENET: enet::ENET::steal(),
            USB_OTG1: usb_otg::USB_OTG1::steal(),
            USB_OTG2: usb_otg::USB_OTG2::steal(),
            USBNC_OTG1: usbnc_otg::USBNC_OTG1::steal(),
            USBNC_OTG2: usbnc_otg::USBNC_OTG2::steal(),
            USBPHY1: usbphy::USBPHY1::steal(),
            USBPHY2: usbphy::USBPHY2::steal(),
            USBHSDCD1: usbhsdcd::USBHSDCD1::steal(),
            USBHSDCD2: usbhsdcd::USBHSDCD2::steal(),
            ENET_QOS: enet_qos::ENET_QOS::steal(),
            CAAM: caam::CAAM::steal(),
            CSI: csi::CSI::steal(),
            LCDIF: lcdif::LCDIF::steal(),
            LCDIFV2: lcdifv2::LCDIFV2::steal(),
            DSI_HOST: dsi_host::DSI_HOST::steal(),
            DSI_HOST_DPI_INTFC: dsi_host_dpi_intfc::DSI_HOST_DPI_INTFC::steal(),
            DSI_HOST_APB_PKT_IF: dsi_host_apb_pkt_if::DSI_HOST_APB_PKT_IF::steal(),
            DSI_HOST_DPHY_INTFC: dsi_host_dphy_intfc::DSI_HOST_DPHY_INTFC::steal(),
            MIPI_CSI2RX: mipi_csi2rx::MIPI_CSI2RX::steal(),
            PXP: pxp::PXP::steal(),
            VIDEO_MUX: video_mux::VIDEO_MUX::steal(),
            DCIC1: dcic::DCIC1::steal(),
            DCIC2: dcic::DCIC2::steal(),
            GPC_CPU_MODE_CTRL_0: gpc_cpu_mode_ctrl_::GPC_CPU_MODE_CTRL_0::steal(),
            GPC_CPU_MODE_CTRL_1: gpc_cpu_mode_ctrl_::GPC_CPU_MODE_CTRL_1::steal(),
            GPC_SET_POINT_CTRL: gpc_set_point_ctrl::GPC_SET_POINT_CTRL::steal(),
            GPC_STBY_CTRL: gpc_stby_ctrl::GPC_STBY_CTRL::steal(),
            SRC: src::SRC::steal(),
            IOMUXC_LPSR: iomuxc_lpsr::IOMUXC_LPSR::steal(),
            IOMUXC_LPSR_GPR: iomuxc_lpsr_gpr::IOMUXC_LPSR_GPR::steal(),
            DMA1: dma1::DMA1::steal(),
            DMAMUX1: dmamux1::DMAMUX1::steal(),
            PDM: pdm::PDM::steal(),
            RDC_SEMAPHORE1: rdc_semaphore::RDC_SEMAPHORE1::steal(),
            RDC_SEMAPHORE2: rdc_semaphore::RDC_SEMAPHORE2::steal(),
            MUB: mub::MUB::steal(),
            RDC: rdc::RDC::steal(),
            KEY_MANAGER: key_manager::KEY_MANAGER::steal(),
            KEY_MANAGER__PUF: key_manager__puf::KEY_MANAGER__PUF::steal(),
            ANADIG_LDO_SNVS: anadig_ldo_snvs::ANADIG_LDO_SNVS::steal(),
            ANADIG_LDO_SNVS_DIG: anadig_ldo_snvs_dig::ANADIG_LDO_SNVS_DIG::steal(),
            ANADIG_MISC: anadig_misc::ANADIG_MISC::steal(),
            ANADIG_OSC: anadig_osc::ANADIG_OSC::steal(),
            ANADIG_PLL: anadig_pll::ANADIG_PLL::steal(),
            ANADIG_PMU: anadig_pmu::ANADIG_PMU::steal(),
            ANADIG_TEMPSENSOR: anadig_tempsensor::ANADIG_TEMPSENSOR::steal(),
            IPS_DOMAIN: ips_domain::IPS_DOMAIN::steal(),
            PGMC_BPC0: pgmc_bpc::PGMC_BPC0::steal(),
            PGMC_BPC1: pgmc_bpc::PGMC_BPC1::steal(),
            PGMC_BPC2: pgmc_bpc::PGMC_BPC2::steal(),
            PGMC_BPC3: pgmc_bpc::PGMC_BPC3::steal(),
            PGMC_BPC4: pgmc_bpc::PGMC_BPC4::steal(),
            PGMC_BPC5: pgmc_bpc::PGMC_BPC5::steal(),
            PGMC_BPC6: pgmc_bpc::PGMC_BPC6::steal(),
            PGMC_BPC7: pgmc_bpc::PGMC_BPC7::steal(),
            PGMC_CPC0: pgmc_cpc::PGMC_CPC0::steal(),
            PGMC_CPC1: pgmc_cpc::PGMC_CPC1::steal(),
            PGMC_CPC0_MIF0: pgmc_cpc0_mif::PGMC_CPC0_MIF0::steal(),
            PGMC_CPC0_MIF1: pgmc_cpc0_mif::PGMC_CPC0_MIF1::steal(),
            PGMC_CPC1_MIF0: pgmc_cpc0_mif::PGMC_CPC1_MIF0::steal(),
            PGMC_CPC1_MIF1: pgmc_cpc0_mif::PGMC_CPC1_MIF1::steal(),
            PGMC_PPC0: pgmc_ppc0::PGMC_PPC0::steal(),
            SNVS: snvs::SNVS::steal(),
            IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS::steal(),
            IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR::steal(),
            SRAM: sram::SRAM::steal(),
            DCDC: dcdc::DCDC::steal(),
            OCOTP: ocotp::OCOTP::steal(),
            SSARC_HP: ssarc_hp::SSARC_HP::steal(),
            SSARC_LP: ssarc_lp::SSARC_LP::steal(),
            CCM: ccm::CCM::steal(),
            SEMA4: sema4::SEMA4::steal(),
            XRDC2_D0: xrdc2_d::XRDC2_D0::steal(),
            XRDC2_D1: xrdc2_d::XRDC2_D1::steal(),
            CDOG: cdog::CDOG::steal(),
            CM7_GPIO2: cm7_gpio::CM7_GPIO2::steal(),
            CM7_GPIO3: cm7_gpio::CM7_GPIO3::steal(),
            GPIO1: gpio::GPIO1::steal(),
            GPIO2: gpio::GPIO2::steal(),
            GPIO3: gpio::GPIO3::steal(),
            GPIO4: gpio::GPIO4::steal(),
            GPIO5: gpio::GPIO5::steal(),
            GPIO6: gpio::GPIO6::steal(),
            GPIO7: gpio::GPIO7::steal(),
            GPIO8: gpio::GPIO8::steal(),
            GPIO9: gpio::GPIO9::steal(),
            GPIO10: gpio::GPIO10::steal(),
            GPIO11: gpio::GPIO11::steal(),
            GPIO12: gpio::GPIO12::steal(),
            GPIO13: gpio::GPIO13::steal(),
            NVIC: nvic::NVIC::steal(),
            MCM: mcm::MCM::steal(),
            MMCAU: mmcau::MMCAU::steal(),
            LMEM: lmem::LMEM::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
