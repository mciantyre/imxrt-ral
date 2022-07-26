#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IEE
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::iee__iee_rt1170::Instance;
pub use crate::imxrt117::peripherals::iee__iee_rt1170::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::iee__iee_rt1170::{
    AESVID, AES_TST_DB0, AES_TST_DB1, AES_TST_DB10, AES_TST_DB11, AES_TST_DB12, AES_TST_DB13,
    AES_TST_DB14, AES_TST_DB15, AES_TST_DB16, AES_TST_DB17, AES_TST_DB18, AES_TST_DB19,
    AES_TST_DB2, AES_TST_DB20, AES_TST_DB21, AES_TST_DB22, AES_TST_DB23, AES_TST_DB24,
    AES_TST_DB25, AES_TST_DB26, AES_TST_DB27, AES_TST_DB28, AES_TST_DB29, AES_TST_DB3,
    AES_TST_DB30, AES_TST_DB31, AES_TST_DB4, AES_TST_DB5, AES_TST_DB6, AES_TST_DB7, AES_TST_DB8,
    AES_TST_DB9, DPAMS, GCFG, PC_BLK_DEC, PC_BLK_ENC, PC_MR_TBC_L, PC_MR_TBC_U, PC_MR_TLAT_L,
    PC_MR_TLAT_U, PC_MR_TLGTT, PC_MR_TRANS, PC_MW_TBC_L, PC_MW_TBC_U, PC_MW_TLAT_L, PC_MW_TLAT_U,
    PC_MW_TLGTT, PC_MW_TRANS, PC_M_LT, PC_M_MBR, PC_SR_TBC_L, PC_SR_TBC_U, PC_SR_TLAT_L,
    PC_SR_TLAT_U, PC_SR_TLGTT, PC_SR_TNRT_L, PC_SR_TNRT_U, PC_SR_TRANS, PC_SW_TBC_L, PC_SW_TBC_U,
    PC_SW_TLAT_L, PC_SW_TLAT_U, PC_SW_TLGTT, PC_SW_TNRT_L, PC_SW_TNRT_U, PC_SW_TRANS, PC_S_LT,
    REGATTR_0, REGATTR_1, REGATTR_2, REGATTR_3, REGATTR_4, REGATTR_5, REGATTR_6, REGATTR_7,
    REGKEY1_0_0, REGKEY1_0_1, REGKEY1_0_2, REGKEY1_0_3, REGKEY1_0_4, REGKEY1_0_5, REGKEY1_0_6,
    REGKEY1_0_7, REGKEY1_1_0, REGKEY1_1_1, REGKEY1_1_2, REGKEY1_1_3, REGKEY1_1_4, REGKEY1_1_5,
    REGKEY1_1_6, REGKEY1_1_7, REGKEY1_2_0, REGKEY1_2_1, REGKEY1_2_2, REGKEY1_2_3, REGKEY1_2_4,
    REGKEY1_2_5, REGKEY1_2_6, REGKEY1_2_7, REGKEY1_3_0, REGKEY1_3_1, REGKEY1_3_2, REGKEY1_3_3,
    REGKEY1_3_4, REGKEY1_3_5, REGKEY1_3_6, REGKEY1_3_7, REGKEY1_4_0, REGKEY1_4_1, REGKEY1_4_2,
    REGKEY1_4_3, REGKEY1_4_4, REGKEY1_4_5, REGKEY1_4_6, REGKEY1_4_7, REGKEY1_5_0, REGKEY1_5_1,
    REGKEY1_5_2, REGKEY1_5_3, REGKEY1_5_4, REGKEY1_5_5, REGKEY1_5_6, REGKEY1_5_7, REGKEY1_6_0,
    REGKEY1_6_1, REGKEY1_6_2, REGKEY1_6_3, REGKEY1_6_4, REGKEY1_6_5, REGKEY1_6_6, REGKEY1_6_7,
    REGKEY1_7_0, REGKEY1_7_1, REGKEY1_7_2, REGKEY1_7_3, REGKEY1_7_4, REGKEY1_7_5, REGKEY1_7_6,
    REGKEY1_7_7, REGKEY2_0_0, REGKEY2_0_1, REGKEY2_0_2, REGKEY2_0_3, REGKEY2_0_4, REGKEY2_0_5,
    REGKEY2_0_6, REGKEY2_0_7, REGKEY2_1_0, REGKEY2_1_1, REGKEY2_1_2, REGKEY2_1_3, REGKEY2_1_4,
    REGKEY2_1_5, REGKEY2_1_6, REGKEY2_1_7, REGKEY2_2_0, REGKEY2_2_1, REGKEY2_2_2, REGKEY2_2_3,
    REGKEY2_2_4, REGKEY2_2_5, REGKEY2_2_6, REGKEY2_2_7, REGKEY2_3_0, REGKEY2_3_1, REGKEY2_3_2,
    REGKEY2_3_3, REGKEY2_3_4, REGKEY2_3_5, REGKEY2_3_6, REGKEY2_3_7, REGKEY2_4_0, REGKEY2_4_1,
    REGKEY2_4_2, REGKEY2_4_3, REGKEY2_4_4, REGKEY2_4_5, REGKEY2_4_6, REGKEY2_4_7, REGKEY2_5_0,
    REGKEY2_5_1, REGKEY2_5_2, REGKEY2_5_3, REGKEY2_5_4, REGKEY2_5_5, REGKEY2_5_6, REGKEY2_5_7,
    REGKEY2_6_0, REGKEY2_6_1, REGKEY2_6_2, REGKEY2_6_3, REGKEY2_6_4, REGKEY2_6_5, REGKEY2_6_6,
    REGKEY2_6_7, REGKEY2_7_0, REGKEY2_7_1, REGKEY2_7_2, REGKEY2_7_3, REGKEY2_7_4, REGKEY2_7_5,
    REGKEY2_7_6, REGKEY2_7_7, REGPO_0, REGPO_1, REGPO_2, REGPO_3, REGPO_4, REGPO_5, REGPO_6,
    REGPO_7, STA, TSTMD, VIDR1,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The IEE__IEE_RT1170 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type IEE__IEE_RT1170 = Instance<0>;

/// The IEE__IEE_RT1170 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type IEE__IEE_RT1170 = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct IEE__IEE_RT1170 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for IEE__IEE_RT1170 {}
impl crate::Valid for IEE__IEE_RT1170 {
    fn take() -> Option<Self> {
        <IEE__IEE_RT1170>::take()
    }
    fn release(self) {
        <IEE__IEE_RT1170>::release(self);
    }
    unsafe fn steal() -> Self {
        <IEE__IEE_RT1170>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static IEE__IEE_RT1170_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the IEE__IEE_RT1170 peripheral instance
#[cfg(not(feature = "nosync"))]
impl IEE__IEE_RT1170 {
    const INSTANCE: Self = Self {
        addr: 0x4006c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in IEE__IEE_RT1170
    pub const reset: ResetValues = ResetValues {
        GCFG: 0x00000000,
        STA: 0x00000001,
        TSTMD: 0x00000000,
        DPAMS: 0x00000000,
        PC_S_LT: 0x00000000,
        PC_M_LT: 0x00000000,
        PC_BLK_ENC: 0x00000000,
        PC_BLK_DEC: 0x00000000,
        PC_SR_TRANS: 0x00000000,
        PC_SW_TRANS: 0x00000000,
        PC_MR_TRANS: 0x00000000,
        PC_MW_TRANS: 0x00000000,
        PC_M_MBR: 0x00000000,
        PC_SR_TBC_U: 0x00000000,
        PC_SR_TBC_L: 0x00000000,
        PC_SW_TBC_U: 0x00000000,
        PC_SW_TBC_L: 0x00000000,
        PC_MR_TBC_U: 0x00000000,
        PC_MR_TBC_L: 0x00000000,
        PC_MW_TBC_U: 0x00000000,
        PC_MW_TBC_L: 0x00000000,
        PC_SR_TLGTT: 0x00000000,
        PC_SW_TLGTT: 0x00000000,
        PC_MR_TLGTT: 0x00000000,
        PC_MW_TLGTT: 0x00000000,
        PC_SR_TLAT_U: 0x00000000,
        PC_SR_TLAT_L: 0x00000000,
        PC_SW_TLAT_U: 0x00000000,
        PC_SW_TLAT_L: 0x00000000,
        PC_MR_TLAT_U: 0x00000000,
        PC_MR_TLAT_L: 0x00000000,
        PC_MW_TLAT_U: 0x00000000,
        PC_MW_TLAT_L: 0x00000000,
        PC_SR_TNRT_U: 0x00000000,
        PC_SR_TNRT_L: 0x00000000,
        PC_SW_TNRT_U: 0x00000000,
        PC_SW_TNRT_L: 0x00000000,
        VIDR1: 0x00340102,
        AESVID: 0x00000020,
        AES_TST_DB0: 0x00000000,
        AES_TST_DB1: 0x00000000,
        AES_TST_DB2: 0x00000000,
        AES_TST_DB3: 0x00000000,
        AES_TST_DB4: 0x00000000,
        AES_TST_DB5: 0x00000000,
        AES_TST_DB6: 0x00000000,
        AES_TST_DB7: 0x00000000,
        AES_TST_DB8: 0x00000000,
        AES_TST_DB9: 0x00000000,
        AES_TST_DB10: 0x00000000,
        AES_TST_DB11: 0x00000000,
        AES_TST_DB12: 0x00000000,
        AES_TST_DB13: 0x00000000,
        AES_TST_DB14: 0x00000000,
        AES_TST_DB15: 0x00000000,
        AES_TST_DB16: 0x00000000,
        AES_TST_DB17: 0x00000000,
        AES_TST_DB18: 0x00000000,
        AES_TST_DB19: 0x00000000,
        AES_TST_DB20: 0x00000000,
        AES_TST_DB21: 0x00000000,
        AES_TST_DB22: 0x00000000,
        AES_TST_DB23: 0x00000000,
        AES_TST_DB24: 0x00000000,
        AES_TST_DB25: 0x00000000,
        AES_TST_DB26: 0x00000000,
        AES_TST_DB27: 0x00000000,
        AES_TST_DB28: 0x00000000,
        AES_TST_DB29: 0x00000000,
        AES_TST_DB30: 0x00000000,
        AES_TST_DB31: 0x00000000,
        REGATTR_0: 0x00000000,
        REGPO_0: 0x00000000,
        REGKEY1_0_0: 0x00000000,
        REGKEY1_1_0: 0x00000000,
        REGKEY1_2_0: 0x00000000,
        REGKEY1_3_0: 0x00000000,
        REGKEY1_4_0: 0x00000000,
        REGKEY1_5_0: 0x00000000,
        REGKEY1_6_0: 0x00000000,
        REGKEY1_7_0: 0x00000000,
        REGKEY2_0_0: 0x00000000,
        REGKEY2_1_0: 0x00000000,
        REGKEY2_2_0: 0x00000000,
        REGKEY2_3_0: 0x00000000,
        REGKEY2_4_0: 0x00000000,
        REGKEY2_5_0: 0x00000000,
        REGKEY2_6_0: 0x00000000,
        REGKEY2_7_0: 0x00000000,
        REGATTR_1: 0x00000000,
        REGPO_1: 0x00000000,
        REGKEY1_0_1: 0x00000000,
        REGKEY1_1_1: 0x00000000,
        REGKEY1_2_1: 0x00000000,
        REGKEY1_3_1: 0x00000000,
        REGKEY1_4_1: 0x00000000,
        REGKEY1_5_1: 0x00000000,
        REGKEY1_6_1: 0x00000000,
        REGKEY1_7_1: 0x00000000,
        REGKEY2_0_1: 0x00000000,
        REGKEY2_1_1: 0x00000000,
        REGKEY2_2_1: 0x00000000,
        REGKEY2_3_1: 0x00000000,
        REGKEY2_4_1: 0x00000000,
        REGKEY2_5_1: 0x00000000,
        REGKEY2_6_1: 0x00000000,
        REGKEY2_7_1: 0x00000000,
        REGATTR_2: 0x00000000,
        REGPO_2: 0x00000000,
        REGKEY1_0_2: 0x00000000,
        REGKEY1_1_2: 0x00000000,
        REGKEY1_2_2: 0x00000000,
        REGKEY1_3_2: 0x00000000,
        REGKEY1_4_2: 0x00000000,
        REGKEY1_5_2: 0x00000000,
        REGKEY1_6_2: 0x00000000,
        REGKEY1_7_2: 0x00000000,
        REGKEY2_0_2: 0x00000000,
        REGKEY2_1_2: 0x00000000,
        REGKEY2_2_2: 0x00000000,
        REGKEY2_3_2: 0x00000000,
        REGKEY2_4_2: 0x00000000,
        REGKEY2_5_2: 0x00000000,
        REGKEY2_6_2: 0x00000000,
        REGKEY2_7_2: 0x00000000,
        REGATTR_3: 0x00000000,
        REGPO_3: 0x00000000,
        REGKEY1_0_3: 0x00000000,
        REGKEY1_1_3: 0x00000000,
        REGKEY1_2_3: 0x00000000,
        REGKEY1_3_3: 0x00000000,
        REGKEY1_4_3: 0x00000000,
        REGKEY1_5_3: 0x00000000,
        REGKEY1_6_3: 0x00000000,
        REGKEY1_7_3: 0x00000000,
        REGKEY2_0_3: 0x00000000,
        REGKEY2_1_3: 0x00000000,
        REGKEY2_2_3: 0x00000000,
        REGKEY2_3_3: 0x00000000,
        REGKEY2_4_3: 0x00000000,
        REGKEY2_5_3: 0x00000000,
        REGKEY2_6_3: 0x00000000,
        REGKEY2_7_3: 0x00000000,
        REGATTR_4: 0x00000000,
        REGPO_4: 0x00000000,
        REGKEY1_0_4: 0x00000000,
        REGKEY1_1_4: 0x00000000,
        REGKEY1_2_4: 0x00000000,
        REGKEY1_3_4: 0x00000000,
        REGKEY1_4_4: 0x00000000,
        REGKEY1_5_4: 0x00000000,
        REGKEY1_6_4: 0x00000000,
        REGKEY1_7_4: 0x00000000,
        REGKEY2_0_4: 0x00000000,
        REGKEY2_1_4: 0x00000000,
        REGKEY2_2_4: 0x00000000,
        REGKEY2_3_4: 0x00000000,
        REGKEY2_4_4: 0x00000000,
        REGKEY2_5_4: 0x00000000,
        REGKEY2_6_4: 0x00000000,
        REGKEY2_7_4: 0x00000000,
        REGATTR_5: 0x00000000,
        REGPO_5: 0x00000000,
        REGKEY1_0_5: 0x00000000,
        REGKEY1_1_5: 0x00000000,
        REGKEY1_2_5: 0x00000000,
        REGKEY1_3_5: 0x00000000,
        REGKEY1_4_5: 0x00000000,
        REGKEY1_5_5: 0x00000000,
        REGKEY1_6_5: 0x00000000,
        REGKEY1_7_5: 0x00000000,
        REGKEY2_0_5: 0x00000000,
        REGKEY2_1_5: 0x00000000,
        REGKEY2_2_5: 0x00000000,
        REGKEY2_3_5: 0x00000000,
        REGKEY2_4_5: 0x00000000,
        REGKEY2_5_5: 0x00000000,
        REGKEY2_6_5: 0x00000000,
        REGKEY2_7_5: 0x00000000,
        REGATTR_6: 0x00000000,
        REGPO_6: 0x00000000,
        REGKEY1_0_6: 0x00000000,
        REGKEY1_1_6: 0x00000000,
        REGKEY1_2_6: 0x00000000,
        REGKEY1_3_6: 0x00000000,
        REGKEY1_4_6: 0x00000000,
        REGKEY1_5_6: 0x00000000,
        REGKEY1_6_6: 0x00000000,
        REGKEY1_7_6: 0x00000000,
        REGKEY2_0_6: 0x00000000,
        REGKEY2_1_6: 0x00000000,
        REGKEY2_2_6: 0x00000000,
        REGKEY2_3_6: 0x00000000,
        REGKEY2_4_6: 0x00000000,
        REGKEY2_5_6: 0x00000000,
        REGKEY2_6_6: 0x00000000,
        REGKEY2_7_6: 0x00000000,
        REGATTR_7: 0x00000000,
        REGPO_7: 0x00000000,
        REGKEY1_0_7: 0x00000000,
        REGKEY1_1_7: 0x00000000,
        REGKEY1_2_7: 0x00000000,
        REGKEY1_3_7: 0x00000000,
        REGKEY1_4_7: 0x00000000,
        REGKEY1_5_7: 0x00000000,
        REGKEY1_6_7: 0x00000000,
        REGKEY1_7_7: 0x00000000,
        REGKEY2_0_7: 0x00000000,
        REGKEY2_1_7: 0x00000000,
        REGKEY2_2_7: 0x00000000,
        REGKEY2_3_7: 0x00000000,
        REGKEY2_4_7: 0x00000000,
        REGKEY2_5_7: 0x00000000,
        REGKEY2_6_7: 0x00000000,
        REGKEY2_7_7: 0x00000000,
    };

    /// Safe access to IEE__IEE_RT1170
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
        let taken = IEE__IEE_RT1170_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to IEE__IEE_RT1170
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = IEE__IEE_RT1170_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal IEE__IEE_RT1170
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        IEE__IEE_RT1170_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl IEE__IEE_RT1170 {
    /// The interrupts associated with IEE__IEE_RT1170
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with IEE__IEE_RT1170
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to IEE__IEE_RT1170
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IEE__IEE_RT1170: *const RegisterBlock = 0x4006c000 as *const _;
