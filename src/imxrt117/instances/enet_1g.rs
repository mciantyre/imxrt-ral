#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ENET
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::enet_1g::Instance;
pub use crate::imxrt117::peripherals::enet_1g::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::enet_1g::{
    ATCOR, ATCR, ATINC, ATOFF, ATPER, ATSTMP, ATVR, DMACFG1, DMACFG2, ECR, EIMR, EIR, FTRL, GALR,
    GAUR, IALR, IAUR, IEEE_R_ALIGN, IEEE_R_CRC, IEEE_R_DROP, IEEE_R_FDXFC, IEEE_R_FRAME_OK,
    IEEE_R_MACERR, IEEE_R_OCTETS_OK, IEEE_T_1COL, IEEE_T_CSERR, IEEE_T_DEF, IEEE_T_DROP,
    IEEE_T_EXCOL, IEEE_T_FDXFC, IEEE_T_FRAME_OK, IEEE_T_LCOL, IEEE_T_MACERR, IEEE_T_MCOL,
    IEEE_T_OCTETS_OK, IEEE_T_SQE, MIBC, MMFR, MRBR, MRBR1, MRBR2, MSCR, OPD, PALR, PAUR, QOS, RACC,
    RAEM, RAFL, RCMR1, RCMR2, RCR, RDAR, RDAR1, RDAR2, RDSR, RDSR1, RDSR2, RMON_R_BC_PKT,
    RMON_R_CRC_ALIGN, RMON_R_FRAG, RMON_R_JAB, RMON_R_MC_PKT, RMON_R_OCTETS, RMON_R_OVERSIZE,
    RMON_R_P1024TO2047, RMON_R_P128TO255, RMON_R_P256TO511, RMON_R_P512TO1023, RMON_R_P64,
    RMON_R_P65TO127, RMON_R_PACKETS, RMON_R_P_GTE2048, RMON_R_UNDERSIZE, RMON_T_BC_PKT, RMON_T_COL,
    RMON_T_CRC_ALIGN, RMON_T_FRAG, RMON_T_JAB, RMON_T_MC_PKT, RMON_T_OCTETS, RMON_T_OVERSIZE,
    RMON_T_P1024TO2047, RMON_T_P128TO255, RMON_T_P256TO511, RMON_T_P512TO1023, RMON_T_P64,
    RMON_T_P65TO127, RMON_T_PACKETS, RMON_T_P_GTE2048, RMON_T_UNDERSIZE, RSEM, RSFL, RXIC0, RXIC1,
    RXIC2, TACC, TAEM, TAFL, TCCR0, TCCR1, TCCR2, TCCR3, TCR, TCSR0, TCSR1, TCSR2, TCSR3, TDAR,
    TDAR1, TDAR2, TDSR, TDSR1, TDSR2, TFWR, TGSR, TIPG, TSEM, TXIC0, TXIC1, TXIC2,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ENET_1G peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ENET_1G = Instance<0>;

/// The ENET_1G peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ENET_1G = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct ENET_1G {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ENET_1G {}
impl crate::Valid for ENET_1G {
    fn take() -> Option<Self> {
        <ENET_1G>::take()
    }
    fn release(self) {
        <ENET_1G>::release(self);
    }
    unsafe fn steal() -> Self {
        <ENET_1G>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ENET_1G_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ENET_1G peripheral instance
#[cfg(not(feature = "nosync"))]
impl ENET_1G {
    const INSTANCE: Self = Self {
        addr: 0x40420000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::ENET_1G_MAC0_Tx_Rx_1,
            crate::interrupt::ENET_1G_MAC0_Tx_Rx_2,
            crate::interrupt::ENET_1G,
            crate::interrupt::ENET_1G_1588_Timer,
        ],
    };

    /// Reset values for each field in ENET_1G
    pub const reset: ResetValues = ResetValues {
        EIR: 0x00000000,
        EIMR: 0x00000000,
        RDAR: 0x00000000,
        TDAR: 0x00000000,
        ECR: 0x70000000,
        MMFR: 0x00000000,
        MSCR: 0x00000000,
        MIBC: 0xC0000000,
        RCR: 0x05EE0001,
        TCR: 0x00000000,
        PALR: 0x00000000,
        PAUR: 0x00008808,
        OPD: 0x00010000,
        TXIC0: 0x00000000,
        TXIC1: 0x00000000,
        TXIC2: 0x00000000,
        RXIC0: 0x00000000,
        RXIC1: 0x00000000,
        RXIC2: 0x00000000,
        IAUR: 0x00000000,
        IALR: 0x00000000,
        GAUR: 0x00000000,
        GALR: 0x00000000,
        TFWR: 0x00000000,
        RDSR1: 0x00000000,
        TDSR1: 0x00000000,
        MRBR1: 0x00000000,
        RDSR2: 0x00000000,
        TDSR2: 0x00000000,
        MRBR2: 0x00000000,
        RDSR: 0x00000000,
        TDSR: 0x00000000,
        MRBR: 0x00000000,
        RSFL: 0x00000000,
        RSEM: 0x00000000,
        RAEM: 0x00000004,
        RAFL: 0x00000004,
        TSEM: 0x00000000,
        TAEM: 0x00000004,
        TAFL: 0x00000008,
        TIPG: 0x0000000C,
        FTRL: 0x000007FF,
        TACC: 0x00000000,
        RACC: 0x00000000,
        RCMR1: 0x00000000,
        RCMR2: 0x00000000,
        DMACFG1: 0x00000000,
        DMACFG2: 0x00000000,
        RDAR1: 0x00000000,
        TDAR1: 0x00000000,
        RDAR2: 0x00000000,
        TDAR2: 0x00000000,
        QOS: 0x00000000,
        RMON_T_PACKETS: 0x00000000,
        RMON_T_BC_PKT: 0x00000000,
        RMON_T_MC_PKT: 0x00000000,
        RMON_T_CRC_ALIGN: 0x00000000,
        RMON_T_UNDERSIZE: 0x00000000,
        RMON_T_OVERSIZE: 0x00000000,
        RMON_T_FRAG: 0x00000000,
        RMON_T_JAB: 0x00000000,
        RMON_T_COL: 0x00000000,
        RMON_T_P64: 0x00000000,
        RMON_T_P65TO127: 0x00000000,
        RMON_T_P128TO255: 0x00000000,
        RMON_T_P256TO511: 0x00000000,
        RMON_T_P512TO1023: 0x00000000,
        RMON_T_P1024TO2047: 0x00000000,
        RMON_T_P_GTE2048: 0x00000000,
        RMON_T_OCTETS: 0x00000000,
        IEEE_T_DROP: 0x00000000,
        IEEE_T_FRAME_OK: 0x00000000,
        IEEE_T_1COL: 0x00000000,
        IEEE_T_MCOL: 0x00000000,
        IEEE_T_DEF: 0x00000000,
        IEEE_T_LCOL: 0x00000000,
        IEEE_T_EXCOL: 0x00000000,
        IEEE_T_MACERR: 0x00000000,
        IEEE_T_CSERR: 0x00000000,
        IEEE_T_SQE: 0x00000000,
        IEEE_T_FDXFC: 0x00000000,
        IEEE_T_OCTETS_OK: 0x00000000,
        RMON_R_PACKETS: 0x00000000,
        RMON_R_BC_PKT: 0x00000000,
        RMON_R_MC_PKT: 0x00000000,
        RMON_R_CRC_ALIGN: 0x00000000,
        RMON_R_UNDERSIZE: 0x00000000,
        RMON_R_OVERSIZE: 0x00000000,
        RMON_R_FRAG: 0x00000000,
        RMON_R_JAB: 0x00000000,
        RMON_R_P64: 0x00000000,
        RMON_R_P65TO127: 0x00000000,
        RMON_R_P128TO255: 0x00000000,
        RMON_R_P256TO511: 0x00000000,
        RMON_R_P512TO1023: 0x00000000,
        RMON_R_P1024TO2047: 0x00000000,
        RMON_R_P_GTE2048: 0x00000000,
        RMON_R_OCTETS: 0x00000000,
        IEEE_R_DROP: 0x00000000,
        IEEE_R_FRAME_OK: 0x00000000,
        IEEE_R_CRC: 0x00000000,
        IEEE_R_ALIGN: 0x00000000,
        IEEE_R_MACERR: 0x00000000,
        IEEE_R_FDXFC: 0x00000000,
        IEEE_R_OCTETS_OK: 0x00000000,
        ATCR: 0x00000000,
        ATVR: 0x00000000,
        ATOFF: 0x00000000,
        ATPER: 0x3B9ACA00,
        ATCOR: 0x00000000,
        ATINC: 0x00000000,
        ATSTMP: 0x00000000,
        TGSR: 0x00000000,
        TCSR0: 0x00000000,
        TCCR0: 0x00000000,
        TCSR1: 0x00000000,
        TCCR1: 0x00000000,
        TCSR2: 0x00000000,
        TCCR2: 0x00000000,
        TCSR3: 0x00000000,
        TCCR3: 0x00000000,
    };

    /// Safe access to ENET_1G
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
        let taken = ENET_1G_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ENET_1G
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ENET_1G_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ENET_1G
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ENET_1G_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ENET_1G {
    /// The interrupts associated with ENET_1G
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 4] = [
        crate::interrupt::ENET_1G_MAC0_Tx_Rx_1,
        crate::interrupt::ENET_1G_MAC0_Tx_Rx_2,
        crate::interrupt::ENET_1G,
        crate::interrupt::ENET_1G_1588_Timer,
    ];

    /// The interrupts associated with ENET_1G
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ENET_1G
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ENET_1G: *const RegisterBlock = 0x40420000 as *const _;
