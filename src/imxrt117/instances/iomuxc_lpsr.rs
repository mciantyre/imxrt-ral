#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC LPSR
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::iomuxc_lpsr::Instance;
pub use crate::imxrt117::peripherals::iomuxc_lpsr::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::iomuxc_lpsr::{
    CAN3_IPP_IND_CANRX_SELECT_INPUT, LPI2C5_IPP_IND_LPI2C_SCL_SELECT_INPUT,
    LPI2C5_IPP_IND_LPI2C_SDA_SELECT_INPUT, LPI2C6_IPP_IND_LPI2C_SCL_SELECT_INPUT,
    LPI2C6_IPP_IND_LPI2C_SDA_SELECT_INPUT, LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_0,
    LPSPI5_IPP_IND_LPSPI_SCK_SELECT_INPUT, LPSPI5_IPP_IND_LPSPI_SDI_SELECT_INPUT,
    LPSPI5_IPP_IND_LPSPI_SDO_SELECT_INPUT, LPUART11_IPP_IND_LPUART_RXD_SELECT_INPUT,
    LPUART11_IPP_IND_LPUART_TXD_SELECT_INPUT, LPUART12_IPP_IND_LPUART_RXD_SELECT_INPUT,
    LPUART12_IPP_IND_LPUART_TXD_SELECT_INPUT, MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_0,
    MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_1, MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_2,
    MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_3, NMI_GLUE_IPP_IND_NMI_SELECT_INPUT,
    SAI4_IPG_CLK_SAI_MCLK_SELECT_INPUT, SAI4_IPP_IND_SAI_RXBCLK_SELECT_INPUT,
    SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_0, SAI4_IPP_IND_SAI_RXSYNC_SELECT_INPUT,
    SAI4_IPP_IND_SAI_TXBCLK_SELECT_INPUT, SAI4_IPP_IND_SAI_TXSYNC_SELECT_INPUT,
    SW_MUX_CTL_PAD_GPIO_LPSR_00, SW_MUX_CTL_PAD_GPIO_LPSR_01, SW_MUX_CTL_PAD_GPIO_LPSR_02,
    SW_MUX_CTL_PAD_GPIO_LPSR_03, SW_MUX_CTL_PAD_GPIO_LPSR_04, SW_MUX_CTL_PAD_GPIO_LPSR_05,
    SW_MUX_CTL_PAD_GPIO_LPSR_06, SW_MUX_CTL_PAD_GPIO_LPSR_07, SW_MUX_CTL_PAD_GPIO_LPSR_08,
    SW_MUX_CTL_PAD_GPIO_LPSR_09, SW_MUX_CTL_PAD_GPIO_LPSR_10, SW_MUX_CTL_PAD_GPIO_LPSR_11,
    SW_MUX_CTL_PAD_GPIO_LPSR_12, SW_MUX_CTL_PAD_GPIO_LPSR_13, SW_MUX_CTL_PAD_GPIO_LPSR_14,
    SW_MUX_CTL_PAD_GPIO_LPSR_15, SW_PAD_CTL_PAD_GPIO_LPSR_00, SW_PAD_CTL_PAD_GPIO_LPSR_01,
    SW_PAD_CTL_PAD_GPIO_LPSR_02, SW_PAD_CTL_PAD_GPIO_LPSR_03, SW_PAD_CTL_PAD_GPIO_LPSR_04,
    SW_PAD_CTL_PAD_GPIO_LPSR_05, SW_PAD_CTL_PAD_GPIO_LPSR_06, SW_PAD_CTL_PAD_GPIO_LPSR_07,
    SW_PAD_CTL_PAD_GPIO_LPSR_08, SW_PAD_CTL_PAD_GPIO_LPSR_09, SW_PAD_CTL_PAD_GPIO_LPSR_10,
    SW_PAD_CTL_PAD_GPIO_LPSR_11, SW_PAD_CTL_PAD_GPIO_LPSR_12, SW_PAD_CTL_PAD_GPIO_LPSR_13,
    SW_PAD_CTL_PAD_GPIO_LPSR_14, SW_PAD_CTL_PAD_GPIO_LPSR_15,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The IOMUXC_LPSR peripheral instance.
#[cfg(not(feature = "doc"))]
pub type IOMUXC_LPSR = Instance<0>;

/// The IOMUXC_LPSR peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type IOMUXC_LPSR = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct IOMUXC_LPSR {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for IOMUXC_LPSR {}
impl crate::Valid for IOMUXC_LPSR {
    fn take() -> Option<Self> {
        <IOMUXC_LPSR>::take()
    }
    fn release(self) {
        <IOMUXC_LPSR>::release(self);
    }
    unsafe fn steal() -> Self {
        <IOMUXC_LPSR>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static IOMUXC_LPSR_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the IOMUXC_LPSR peripheral instance
#[cfg(not(feature = "nosync"))]
impl IOMUXC_LPSR {
    const INSTANCE: Self = Self {
        addr: 0x40c08000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in IOMUXC_LPSR
    pub const reset: ResetValues = ResetValues {
        SW_MUX_CTL_PAD_GPIO_LPSR_00: 0x0000000A,
        SW_MUX_CTL_PAD_GPIO_LPSR_01: 0x0000000A,
        SW_MUX_CTL_PAD_GPIO_LPSR_02: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_LPSR_03: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_LPSR_04: 0x0000000A,
        SW_MUX_CTL_PAD_GPIO_LPSR_05: 0x0000000A,
        SW_MUX_CTL_PAD_GPIO_LPSR_06: 0x0000000A,
        SW_MUX_CTL_PAD_GPIO_LPSR_07: 0x0000000A,
        SW_MUX_CTL_PAD_GPIO_LPSR_08: 0x0000000A,
        SW_MUX_CTL_PAD_GPIO_LPSR_09: 0x0000000A,
        SW_MUX_CTL_PAD_GPIO_LPSR_10: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_LPSR_11: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_LPSR_12: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_LPSR_13: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_LPSR_14: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_LPSR_15: 0x00000000,
        SW_PAD_CTL_PAD_GPIO_LPSR_00: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_01: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_02: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_03: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_04: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_05: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_06: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_07: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_08: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_09: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_10: 0x0000000E,
        SW_PAD_CTL_PAD_GPIO_LPSR_11: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_12: 0x0000000E,
        SW_PAD_CTL_PAD_GPIO_LPSR_13: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_LPSR_14: 0x00000006,
        SW_PAD_CTL_PAD_GPIO_LPSR_15: 0x0000000E,
        CAN3_IPP_IND_CANRX_SELECT_INPUT: 0x00000000,
        LPI2C5_IPP_IND_LPI2C_SCL_SELECT_INPUT: 0x00000000,
        LPI2C5_IPP_IND_LPI2C_SDA_SELECT_INPUT: 0x00000000,
        LPI2C6_IPP_IND_LPI2C_SCL_SELECT_INPUT: 0x00000000,
        LPI2C6_IPP_IND_LPI2C_SDA_SELECT_INPUT: 0x00000000,
        LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: 0x00000000,
        LPSPI5_IPP_IND_LPSPI_SCK_SELECT_INPUT: 0x00000000,
        LPSPI5_IPP_IND_LPSPI_SDI_SELECT_INPUT: 0x00000000,
        LPSPI5_IPP_IND_LPSPI_SDO_SELECT_INPUT: 0x00000000,
        LPUART11_IPP_IND_LPUART_RXD_SELECT_INPUT: 0x00000000,
        LPUART11_IPP_IND_LPUART_TXD_SELECT_INPUT: 0x00000000,
        LPUART12_IPP_IND_LPUART_RXD_SELECT_INPUT: 0x00000000,
        LPUART12_IPP_IND_LPUART_TXD_SELECT_INPUT: 0x00000000,
        MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_0: 0x00000000,
        MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_1: 0x00000000,
        MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_2: 0x00000000,
        MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_3: 0x00000000,
        NMI_GLUE_IPP_IND_NMI_SELECT_INPUT: 0x00000000,
        SAI4_IPG_CLK_SAI_MCLK_SELECT_INPUT: 0x00000000,
        SAI4_IPP_IND_SAI_RXBCLK_SELECT_INPUT: 0x00000000,
        SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: 0x00000000,
        SAI4_IPP_IND_SAI_RXSYNC_SELECT_INPUT: 0x00000000,
        SAI4_IPP_IND_SAI_TXBCLK_SELECT_INPUT: 0x00000000,
        SAI4_IPP_IND_SAI_TXSYNC_SELECT_INPUT: 0x00000000,
    };

    /// Safe access to IOMUXC_LPSR
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = IOMUXC_LPSR_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to IOMUXC_LPSR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = IOMUXC_LPSR_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal IOMUXC_LPSR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        IOMUXC_LPSR_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl IOMUXC_LPSR {
    /// The interrupts associated with IOMUXC_LPSR
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with IOMUXC_LPSR
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to IOMUXC_LPSR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC_LPSR: *const RegisterBlock = 0x40c08000 as *const _;
