#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CAN
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::can::Instance;
pub use crate::imxrt117::peripherals::can::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::can::{
    CBT, CRCR, CS0, CS1, CS10, CS11, CS12, CS13, CS14, CS15, CS16, CS17, CS18, CS19, CS2, CS20,
    CS21, CS22, CS23, CS24, CS25, CS26, CS27, CS28, CS29, CS3, CS30, CS31, CS32, CS33, CS34, CS35,
    CS36, CS37, CS38, CS39, CS4, CS40, CS41, CS42, CS43, CS44, CS45, CS46, CS47, CS48, CS49, CS5,
    CS50, CS51, CS52, CS53, CS54, CS55, CS56, CS57, CS58, CS59, CS6, CS60, CS61, CS62, CS63, CS7,
    CS8, CS9, CTRL1, CTRL2, ECR, ERRIAR, ERRIDPR, ERRIPPR, ERRSR, ESR1, ESR2, FDCBT, FDCRC, FDCTRL,
    ID0, ID1, ID10, ID11, ID12, ID13, ID14, ID15, ID16, ID17, ID18, ID19, ID2, ID20, ID21, ID22,
    ID23, ID24, ID25, ID26, ID27, ID28, ID29, ID3, ID30, ID31, ID32, ID33, ID34, ID35, ID36, ID37,
    ID38, ID39, ID4, ID40, ID41, ID42, ID43, ID44, ID45, ID46, ID47, ID48, ID49, ID5, ID50, ID51,
    ID52, ID53, ID54, ID55, ID56, ID57, ID58, ID59, ID6, ID60, ID61, ID62, ID63, ID7, ID8, ID9,
    IFLAG1, IFLAG2, IMASK1, IMASK2, MB, MB0, MB0_16B_WORD0_H, MB0_16B_WORD1_H, MB0_16B_WORD1_L,
    MB0_32B_WORD4_H, MB0_32B_WORD5_H, MB0_32B_WORD5_L, MB0_64B_WORD12_H, MB0_64B_WORD12_L,
    MB0_64B_WORD13_H, MB0_64B_WORD13_L, MB0_64B_WORD8_H, MB0_64B_WORD8_L, MB0_64B_WORD9_H,
    MB0_64B_WORD9_L, MB1, MB10_16B_WORD0_H, MB10_16B_WORD0_L, MB10_16B_WORD1_H, MB10_16B_WORD1_L,
    MB10_32B_WORD0_H, MB10_32B_WORD0_L, MB10_32B_WORD1_H, MB10_32B_WORD1_L, MB10_32B_WORD4_H,
    MB10_32B_WORD4_L, MB10_32B_WORD5_H, MB10_32B_WORD5_L, MB10_8B_WORD0, MB10_8B_WORD1,
    MB11_16B_CS_H, MB11_16B_CS_L, MB11_16B_ID_H, MB11_16B_ID_L, MB11_16B_WORD2_H, MB11_16B_WORD2_L,
    MB11_16B_WORD3_H, MB11_16B_WORD3_L, MB11_32B_CS_H, MB11_32B_CS_L, MB11_32B_ID_H, MB11_32B_ID_L,
    MB11_32B_WORD2_H, MB11_32B_WORD2_L, MB11_32B_WORD3_H, MB11_32B_WORD3_L, MB11_32B_WORD6_H,
    MB11_32B_WORD6_L, MB11_32B_WORD7_H, MB11_32B_WORD7_L, MB11_8B_WORD0, MB11_8B_WORD1,
    MB12_16B_WORD0_H, MB12_16B_WORD0_L, MB12_16B_WORD1_H, MB12_16B_WORD1_L, MB12_8B_WORD0,
    MB12_8B_WORD1, MB13_16B_CS_H, MB13_16B_CS_L, MB13_16B_ID_H, MB13_16B_ID_L, MB13_16B_WORD2_H,
    MB13_16B_WORD2_L, MB13_16B_WORD3_H, MB13_16B_WORD3_L, MB13_8B_WORD0, MB13_8B_WORD1,
    MB14_16B_WORD0_H, MB14_16B_WORD0_L, MB14_16B_WORD1_H, MB14_16B_WORD1_L, MB14_8B_WORD0,
    MB14_8B_WORD1, MB15_16B_CS_H, MB15_16B_CS_L, MB15_16B_ID_H, MB15_16B_ID_L, MB15_16B_WORD2_H,
    MB15_16B_WORD2_L, MB15_16B_WORD3_H, MB15_16B_WORD3_L, MB16_16B_WORD0_H, MB16_16B_WORD0_L,
    MB16_16B_WORD1_H, MB16_16B_WORD1_L, MB1_32B_WORD6_H, MB1_32B_WORD7_H, MB1_32B_WORD7_L,
    MB1_64B_WORD10_H, MB1_64B_WORD10_L, MB1_64B_WORD11_H, MB1_64B_WORD11_L, MB1_64B_WORD14_H,
    MB1_64B_WORD14_L, MB1_64B_WORD15_H, MB1_64B_WORD15_L, MB1_64B_WORD2_H, MB1_64B_WORD2_L,
    MB1_64B_WORD3_H, MB1_64B_WORD3_L, MB1_64B_WORD6_H, MB1_64B_WORD6_L, MB1_64B_WORD7_H,
    MB1_64B_WORD7_L, MB20_16B_WORD0_H, MB20_16B_WORD0_L, MB20_16B_WORD1_H, MB20_16B_WORD1_L,
    MB2_64B_WORD0_H, MB2_64B_WORD0_L, MB2_64B_WORD12_H, MB2_64B_WORD13_H, MB2_64B_WORD1_H,
    MB2_64B_WORD1_L, MB2_64B_WORD4_H, MB2_64B_WORD5_H, MB2_64B_WORD8_H, MB2_64B_WORD9_H,
    MB31_8B_WORD0, MB31_8B_WORD1, MB3_64B_CS_H, MB3_64B_ID_H, MB3_64B_WORD2_H, MB3_64B_WORD3_H,
    MB63_8B_WORD0, MB63_8B_WORD1, MCR, MECR, RERRAR, RERRDR, RERRSYNR, RX14MASK, RX15MASK,
    RXFGMASK, RXFIR, RXIMR0, RXIMR1, RXIMR10, RXIMR11, RXIMR12, RXIMR13, RXIMR14, RXIMR15, RXIMR16,
    RXIMR17, RXIMR18, RXIMR19, RXIMR2, RXIMR20, RXIMR21, RXIMR22, RXIMR23, RXIMR24, RXIMR25,
    RXIMR26, RXIMR27, RXIMR28, RXIMR29, RXIMR3, RXIMR30, RXIMR31, RXIMR32, RXIMR33, RXIMR34,
    RXIMR35, RXIMR36, RXIMR37, RXIMR38, RXIMR39, RXIMR4, RXIMR40, RXIMR41, RXIMR42, RXIMR43,
    RXIMR44, RXIMR45, RXIMR46, RXIMR47, RXIMR48, RXIMR49, RXIMR5, RXIMR50, RXIMR51, RXIMR52,
    RXIMR53, RXIMR54, RXIMR55, RXIMR56, RXIMR57, RXIMR58, RXIMR59, RXIMR6, RXIMR60, RXIMR61,
    RXIMR62, RXIMR63, RXIMR7, RXIMR8, RXIMR9, RXMGMASK, TIMER,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CAN1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CAN1 = Instance<1>;

/// The CAN1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CAN1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct CAN1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CAN1 {}
impl crate::Valid for CAN1 {
    fn take() -> Option<Self> {
        <CAN1>::take()
    }
    fn release(self) {
        <CAN1>::release(self);
    }
    unsafe fn steal() -> Self {
        <CAN1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CAN1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CAN1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CAN1 {
    const INSTANCE: Self = Self {
        addr: 0x400c4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CAN1, crate::interrupt::CAN1_ERROR],
    };

    /// Reset values for each field in CAN1
    pub const reset: ResetValues = ResetValues {
        MCR: 0x5980000F,
        CTRL1: 0x00000000,
        TIMER: 0x00000000,
        RXMGMASK: 0x00000000,
        RX14MASK: 0x00000000,
        RX15MASK: 0x00000000,
        ECR: 0x00000000,
        ESR1: 0x00000000,
        IMASK2: 0x00000000,
        IMASK1: 0x00000000,
        IFLAG2: 0x00000000,
        IFLAG1: 0x00000000,
        CTRL2: 0x00800000,
        ESR2: 0x00000000,
        CRCR: 0x00000000,
        RXFGMASK: 0x00000000,
        RXFIR: 0x00000000,
        CBT: 0x00000000,
        CS0: 0x00000000,
        ID0: 0x00000000,
        MB0: 0x00000000,
        MB0_16B_WORD1_L: 0x00000000,
        CS1: 0x00000000,
        ID1: 0x00000000,
        MB: 0x00000000,
        MB0_32B_WORD5_L: 0x00000000,
        CS2: 0x00000000,
        ID2: 0x00000000,
        MB0_64B_WORD8_L: 0x00000000,
        MB0_64B_WORD9_L: 0x00000000,
        CS3: 0x00000000,
        ID3: 0x00000000,
        MB0_64B_WORD12_L: 0x00000000,
        MB0_64B_WORD13_L: 0x00000000,
        CS4: 0x00000000,
        ID4: 0x00000000,
        MB1: 0x00000000,
        MB1_32B_WORD7_L: 0x00000000,
        CS5: 0x00000000,
        ID5: 0x00000000,
        MB1_64B_WORD2_L: 0x00000000,
        MB1_64B_WORD3_L: 0x00000000,
        CS6: 0x00000000,
        ID6: 0x00000000,
        MB1_64B_WORD6_L: 0x00000000,
        MB1_64B_WORD7_L: 0x00000000,
        CS7: 0x00000000,
        ID7: 0x00000000,
        MB1_64B_WORD10_L: 0x00000000,
        MB1_64B_WORD11_L: 0x00000000,
        CS8: 0x00000000,
        ID8: 0x00000000,
        MB1_64B_WORD14_L: 0x00000000,
        MB1_64B_WORD15_L: 0x00000000,
        CS9: 0x00000000,
        ID9: 0x00000000,
        MB2_64B_WORD0_L: 0x00000000,
        MB2_64B_WORD1_L: 0x00000000,
        CS10: 0x00000000,
        ID10: 0x00000000,
        MB10_8B_WORD0: 0x00000000,
        MB10_8B_WORD1: 0x00000000,
        CS11: 0x00000000,
        ID11: 0x00000000,
        MB11_8B_WORD0: 0x00000000,
        MB11_8B_WORD1: 0x00000000,
        CS12: 0x00000000,
        ID12: 0x00000000,
        MB12_8B_WORD0: 0x00000000,
        MB12_8B_WORD1: 0x00000000,
        CS13: 0x00000000,
        ID13: 0x00000000,
        MB13_8B_WORD0: 0x00000000,
        MB13_8B_WORD1: 0x00000000,
        CS14: 0x00000000,
        ID14: 0x00000000,
        MB14_8B_WORD0: 0x00000000,
        MB14_8B_WORD1: 0x00000000,
        CS15: 0x00000000,
        ID15: 0x00000000,
        MB10_16B_WORD0_L: 0x00000000,
        MB10_16B_WORD1_L: 0x00000000,
        CS16: 0x00000000,
        ID16: 0x00000000,
        MB11_16B_CS_L: 0x00000000,
        MB11_16B_ID_L: 0x00000000,
        CS17: 0x00000000,
        ID17: 0x00000000,
        MB11_16B_WORD2_L: 0x00000000,
        MB11_16B_WORD3_L: 0x00000000,
        CS18: 0x00000000,
        ID18: 0x00000000,
        MB12_16B_WORD0_L: 0x00000000,
        MB12_16B_WORD1_L: 0x00000000,
        CS19: 0x00000000,
        ID19: 0x00000000,
        MB13_16B_CS_L: 0x00000000,
        MB13_16B_ID_L: 0x00000000,
        CS20: 0x00000000,
        ID20: 0x00000000,
        MB13_16B_WORD2_L: 0x00000000,
        MB13_16B_WORD3_L: 0x00000000,
        CS21: 0x00000000,
        ID21: 0x00000000,
        MB14_16B_WORD0_L: 0x00000000,
        MB14_16B_WORD1_L: 0x00000000,
        CS22: 0x00000000,
        ID22: 0x00000000,
        MB15_16B_CS_L: 0x00000000,
        MB15_16B_ID_L: 0x00000000,
        CS23: 0x00000000,
        ID23: 0x00000000,
        MB15_16B_WORD2_L: 0x00000000,
        MB15_16B_WORD3_L: 0x00000000,
        CS24: 0x00000000,
        ID24: 0x00000000,
        MB16_16B_WORD0_L: 0x00000000,
        MB16_16B_WORD1_L: 0x00000000,
        CS25: 0x00000000,
        ID25: 0x00000000,
        MB10_32B_WORD0_L: 0x00000000,
        MB10_32B_WORD1_L: 0x00000000,
        CS26: 0x00000000,
        ID26: 0x00000000,
        MB10_32B_WORD4_L: 0x00000000,
        MB10_32B_WORD5_L: 0x00000000,
        CS27: 0x00000000,
        ID27: 0x00000000,
        MB11_32B_CS_L: 0x00000000,
        MB11_32B_ID_L: 0x00000000,
        CS28: 0x00000000,
        ID28: 0x00000000,
        MB11_32B_WORD2_L: 0x00000000,
        MB11_32B_WORD3_L: 0x00000000,
        CS29: 0x00000000,
        ID29: 0x00000000,
        MB11_32B_WORD6_L: 0x00000000,
        MB11_32B_WORD7_L: 0x00000000,
        CS30: 0x00000000,
        ID30: 0x00000000,
        MB20_16B_WORD0_L: 0x00000000,
        MB20_16B_WORD1_L: 0x00000000,
        CS31: 0x00000000,
        ID31: 0x00000000,
        MB31_8B_WORD0: 0x00000000,
        MB31_8B_WORD1: 0x00000000,
        CS32: 0x00000000,
        ID32: 0x00000000,
        MB0_16B_WORD0_H: 0x00000000,
        MB0_16B_WORD1_H: 0x00000000,
        CS33: 0x00000000,
        ID33: 0x00000000,
        MB0_32B_WORD4_H: 0x00000000,
        MB0_32B_WORD5_H: 0x00000000,
        CS34: 0x00000000,
        ID34: 0x00000000,
        MB0_64B_WORD8_H: 0x00000000,
        MB0_64B_WORD9_H: 0x00000000,
        CS35: 0x00000000,
        ID35: 0x00000000,
        MB0_64B_WORD12_H: 0x00000000,
        MB0_64B_WORD13_H: 0x00000000,
        CS36: 0x00000000,
        ID36: 0x00000000,
        MB1_32B_WORD6_H: 0x00000000,
        MB1_32B_WORD7_H: 0x00000000,
        CS37: 0x00000000,
        ID37: 0x00000000,
        MB1_64B_WORD2_H: 0x00000000,
        MB1_64B_WORD3_H: 0x00000000,
        CS38: 0x00000000,
        ID38: 0x00000000,
        MB1_64B_WORD6_H: 0x00000000,
        MB1_64B_WORD7_H: 0x00000000,
        CS39: 0x00000000,
        ID39: 0x00000000,
        MB1_64B_WORD10_H: 0x00000000,
        MB1_64B_WORD11_H: 0x00000000,
        CS40: 0x00000000,
        ID40: 0x00000000,
        MB1_64B_WORD14_H: 0x00000000,
        MB1_64B_WORD15_H: 0x00000000,
        CS41: 0x00000000,
        ID41: 0x00000000,
        MB2_64B_WORD0_H: 0x00000000,
        MB2_64B_WORD1_H: 0x00000000,
        CS42: 0x00000000,
        ID42: 0x00000000,
        MB2_64B_WORD4_H: 0x00000000,
        MB2_64B_WORD5_H: 0x00000000,
        CS43: 0x00000000,
        ID43: 0x00000000,
        MB2_64B_WORD8_H: 0x00000000,
        MB2_64B_WORD9_H: 0x00000000,
        CS44: 0x00000000,
        ID44: 0x00000000,
        MB2_64B_WORD12_H: 0x00000000,
        MB2_64B_WORD13_H: 0x00000000,
        CS45: 0x00000000,
        ID45: 0x00000000,
        MB3_64B_CS_H: 0x00000000,
        MB3_64B_ID_H: 0x00000000,
        CS46: 0x00000000,
        ID46: 0x00000000,
        MB3_64B_WORD2_H: 0x00000000,
        MB3_64B_WORD3_H: 0x00000000,
        CS47: 0x00000000,
        ID47: 0x00000000,
        MB10_16B_WORD0_H: 0x00000000,
        MB10_16B_WORD1_H: 0x00000000,
        CS48: 0x00000000,
        ID48: 0x00000000,
        MB11_16B_CS_H: 0x00000000,
        MB11_16B_ID_H: 0x00000000,
        CS49: 0x00000000,
        ID49: 0x00000000,
        MB11_16B_WORD2_H: 0x00000000,
        MB11_16B_WORD3_H: 0x00000000,
        CS50: 0x00000000,
        ID50: 0x00000000,
        MB12_16B_WORD0_H: 0x00000000,
        MB12_16B_WORD1_H: 0x00000000,
        CS51: 0x00000000,
        ID51: 0x00000000,
        MB13_16B_CS_H: 0x00000000,
        MB13_16B_ID_H: 0x00000000,
        CS52: 0x00000000,
        ID52: 0x00000000,
        MB13_16B_WORD2_H: 0x00000000,
        MB13_16B_WORD3_H: 0x00000000,
        CS53: 0x00000000,
        ID53: 0x00000000,
        MB14_16B_WORD0_H: 0x00000000,
        MB14_16B_WORD1_H: 0x00000000,
        CS54: 0x00000000,
        ID54: 0x00000000,
        MB15_16B_CS_H: 0x00000000,
        MB15_16B_ID_H: 0x00000000,
        CS55: 0x00000000,
        ID55: 0x00000000,
        MB15_16B_WORD2_H: 0x00000000,
        MB15_16B_WORD3_H: 0x00000000,
        CS56: 0x00000000,
        ID56: 0x00000000,
        MB16_16B_WORD0_H: 0x00000000,
        MB16_16B_WORD1_H: 0x00000000,
        CS57: 0x00000000,
        ID57: 0x00000000,
        MB10_32B_WORD0_H: 0x00000000,
        MB10_32B_WORD1_H: 0x00000000,
        CS58: 0x00000000,
        ID58: 0x00000000,
        MB10_32B_WORD4_H: 0x00000000,
        MB10_32B_WORD5_H: 0x00000000,
        CS59: 0x00000000,
        ID59: 0x00000000,
        MB11_32B_CS_H: 0x00000000,
        MB11_32B_ID_H: 0x00000000,
        CS60: 0x00000000,
        ID60: 0x00000000,
        MB11_32B_WORD2_H: 0x00000000,
        MB11_32B_WORD3_H: 0x00000000,
        CS61: 0x00000000,
        ID61: 0x00000000,
        MB11_32B_WORD6_H: 0x00000000,
        MB11_32B_WORD7_H: 0x00000000,
        CS62: 0x00000000,
        ID62: 0x00000000,
        MB20_16B_WORD0_H: 0x00000000,
        MB20_16B_WORD1_H: 0x00000000,
        CS63: 0x00000000,
        ID63: 0x00000000,
        MB63_8B_WORD0: 0x00000000,
        MB63_8B_WORD1: 0x00000000,
        RXIMR0: 0x00000000,
        RXIMR1: 0x00000000,
        RXIMR2: 0x00000000,
        RXIMR3: 0x00000000,
        RXIMR4: 0x00000000,
        RXIMR5: 0x00000000,
        RXIMR6: 0x00000000,
        RXIMR7: 0x00000000,
        RXIMR8: 0x00000000,
        RXIMR9: 0x00000000,
        RXIMR10: 0x00000000,
        RXIMR11: 0x00000000,
        RXIMR12: 0x00000000,
        RXIMR13: 0x00000000,
        RXIMR14: 0x00000000,
        RXIMR15: 0x00000000,
        RXIMR16: 0x00000000,
        RXIMR17: 0x00000000,
        RXIMR18: 0x00000000,
        RXIMR19: 0x00000000,
        RXIMR20: 0x00000000,
        RXIMR21: 0x00000000,
        RXIMR22: 0x00000000,
        RXIMR23: 0x00000000,
        RXIMR24: 0x00000000,
        RXIMR25: 0x00000000,
        RXIMR26: 0x00000000,
        RXIMR27: 0x00000000,
        RXIMR28: 0x00000000,
        RXIMR29: 0x00000000,
        RXIMR30: 0x00000000,
        RXIMR31: 0x00000000,
        RXIMR32: 0x00000000,
        RXIMR33: 0x00000000,
        RXIMR34: 0x00000000,
        RXIMR35: 0x00000000,
        RXIMR36: 0x00000000,
        RXIMR37: 0x00000000,
        RXIMR38: 0x00000000,
        RXIMR39: 0x00000000,
        RXIMR40: 0x00000000,
        RXIMR41: 0x00000000,
        RXIMR42: 0x00000000,
        RXIMR43: 0x00000000,
        RXIMR44: 0x00000000,
        RXIMR45: 0x00000000,
        RXIMR46: 0x00000000,
        RXIMR47: 0x00000000,
        RXIMR48: 0x00000000,
        RXIMR49: 0x00000000,
        RXIMR50: 0x00000000,
        RXIMR51: 0x00000000,
        RXIMR52: 0x00000000,
        RXIMR53: 0x00000000,
        RXIMR54: 0x00000000,
        RXIMR55: 0x00000000,
        RXIMR56: 0x00000000,
        RXIMR57: 0x00000000,
        RXIMR58: 0x00000000,
        RXIMR59: 0x00000000,
        RXIMR60: 0x00000000,
        RXIMR61: 0x00000000,
        RXIMR62: 0x00000000,
        RXIMR63: 0x00000000,
        MECR: 0x800C0080,
        ERRIAR: 0x00000000,
        ERRIDPR: 0x00000000,
        ERRIPPR: 0x00000000,
        RERRAR: 0x00000000,
        RERRDR: 0x00000000,
        RERRSYNR: 0x00000000,
        ERRSR: 0x00000000,
        FDCTRL: 0x80000100,
        FDCBT: 0x00000000,
        FDCRC: 0x00000000,
    };

    /// Safe access to CAN1
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
        let taken = CAN1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CAN1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CAN1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CAN1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CAN1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CAN1 {
    /// The interrupts associated with CAN1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] =
        [crate::interrupt::CAN1, crate::interrupt::CAN1_ERROR];

    /// The interrupts associated with CAN1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CAN1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN1: *const RegisterBlock = 0x400c4000 as *const _;

/// The CAN2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CAN2 = Instance<2>;

/// The CAN2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CAN2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct CAN2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CAN2 {}
impl crate::Valid for CAN2 {
    fn take() -> Option<Self> {
        <CAN2>::take()
    }
    fn release(self) {
        <CAN2>::release(self);
    }
    unsafe fn steal() -> Self {
        <CAN2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CAN2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CAN2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CAN2 {
    const INSTANCE: Self = Self {
        addr: 0x400c8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CAN2, crate::interrupt::CAN2_ERROR],
    };

    /// Reset values for each field in CAN2
    pub const reset: ResetValues = ResetValues {
        MCR: 0x5980000F,
        CTRL1: 0x00000000,
        TIMER: 0x00000000,
        RXMGMASK: 0x00000000,
        RX14MASK: 0x00000000,
        RX15MASK: 0x00000000,
        ECR: 0x00000000,
        ESR1: 0x00000000,
        IMASK2: 0x00000000,
        IMASK1: 0x00000000,
        IFLAG2: 0x00000000,
        IFLAG1: 0x00000000,
        CTRL2: 0x00800000,
        ESR2: 0x00000000,
        CRCR: 0x00000000,
        RXFGMASK: 0x00000000,
        RXFIR: 0x00000000,
        CBT: 0x00000000,
        CS0: 0x00000000,
        ID0: 0x00000000,
        MB0: 0x00000000,
        MB0_16B_WORD1_L: 0x00000000,
        CS1: 0x00000000,
        ID1: 0x00000000,
        MB: 0x00000000,
        MB0_32B_WORD5_L: 0x00000000,
        CS2: 0x00000000,
        ID2: 0x00000000,
        MB0_64B_WORD8_L: 0x00000000,
        MB0_64B_WORD9_L: 0x00000000,
        CS3: 0x00000000,
        ID3: 0x00000000,
        MB0_64B_WORD12_L: 0x00000000,
        MB0_64B_WORD13_L: 0x00000000,
        CS4: 0x00000000,
        ID4: 0x00000000,
        MB1: 0x00000000,
        MB1_32B_WORD7_L: 0x00000000,
        CS5: 0x00000000,
        ID5: 0x00000000,
        MB1_64B_WORD2_L: 0x00000000,
        MB1_64B_WORD3_L: 0x00000000,
        CS6: 0x00000000,
        ID6: 0x00000000,
        MB1_64B_WORD6_L: 0x00000000,
        MB1_64B_WORD7_L: 0x00000000,
        CS7: 0x00000000,
        ID7: 0x00000000,
        MB1_64B_WORD10_L: 0x00000000,
        MB1_64B_WORD11_L: 0x00000000,
        CS8: 0x00000000,
        ID8: 0x00000000,
        MB1_64B_WORD14_L: 0x00000000,
        MB1_64B_WORD15_L: 0x00000000,
        CS9: 0x00000000,
        ID9: 0x00000000,
        MB2_64B_WORD0_L: 0x00000000,
        MB2_64B_WORD1_L: 0x00000000,
        CS10: 0x00000000,
        ID10: 0x00000000,
        MB10_8B_WORD0: 0x00000000,
        MB10_8B_WORD1: 0x00000000,
        CS11: 0x00000000,
        ID11: 0x00000000,
        MB11_8B_WORD0: 0x00000000,
        MB11_8B_WORD1: 0x00000000,
        CS12: 0x00000000,
        ID12: 0x00000000,
        MB12_8B_WORD0: 0x00000000,
        MB12_8B_WORD1: 0x00000000,
        CS13: 0x00000000,
        ID13: 0x00000000,
        MB13_8B_WORD0: 0x00000000,
        MB13_8B_WORD1: 0x00000000,
        CS14: 0x00000000,
        ID14: 0x00000000,
        MB14_8B_WORD0: 0x00000000,
        MB14_8B_WORD1: 0x00000000,
        CS15: 0x00000000,
        ID15: 0x00000000,
        MB10_16B_WORD0_L: 0x00000000,
        MB10_16B_WORD1_L: 0x00000000,
        CS16: 0x00000000,
        ID16: 0x00000000,
        MB11_16B_CS_L: 0x00000000,
        MB11_16B_ID_L: 0x00000000,
        CS17: 0x00000000,
        ID17: 0x00000000,
        MB11_16B_WORD2_L: 0x00000000,
        MB11_16B_WORD3_L: 0x00000000,
        CS18: 0x00000000,
        ID18: 0x00000000,
        MB12_16B_WORD0_L: 0x00000000,
        MB12_16B_WORD1_L: 0x00000000,
        CS19: 0x00000000,
        ID19: 0x00000000,
        MB13_16B_CS_L: 0x00000000,
        MB13_16B_ID_L: 0x00000000,
        CS20: 0x00000000,
        ID20: 0x00000000,
        MB13_16B_WORD2_L: 0x00000000,
        MB13_16B_WORD3_L: 0x00000000,
        CS21: 0x00000000,
        ID21: 0x00000000,
        MB14_16B_WORD0_L: 0x00000000,
        MB14_16B_WORD1_L: 0x00000000,
        CS22: 0x00000000,
        ID22: 0x00000000,
        MB15_16B_CS_L: 0x00000000,
        MB15_16B_ID_L: 0x00000000,
        CS23: 0x00000000,
        ID23: 0x00000000,
        MB15_16B_WORD2_L: 0x00000000,
        MB15_16B_WORD3_L: 0x00000000,
        CS24: 0x00000000,
        ID24: 0x00000000,
        MB16_16B_WORD0_L: 0x00000000,
        MB16_16B_WORD1_L: 0x00000000,
        CS25: 0x00000000,
        ID25: 0x00000000,
        MB10_32B_WORD0_L: 0x00000000,
        MB10_32B_WORD1_L: 0x00000000,
        CS26: 0x00000000,
        ID26: 0x00000000,
        MB10_32B_WORD4_L: 0x00000000,
        MB10_32B_WORD5_L: 0x00000000,
        CS27: 0x00000000,
        ID27: 0x00000000,
        MB11_32B_CS_L: 0x00000000,
        MB11_32B_ID_L: 0x00000000,
        CS28: 0x00000000,
        ID28: 0x00000000,
        MB11_32B_WORD2_L: 0x00000000,
        MB11_32B_WORD3_L: 0x00000000,
        CS29: 0x00000000,
        ID29: 0x00000000,
        MB11_32B_WORD6_L: 0x00000000,
        MB11_32B_WORD7_L: 0x00000000,
        CS30: 0x00000000,
        ID30: 0x00000000,
        MB20_16B_WORD0_L: 0x00000000,
        MB20_16B_WORD1_L: 0x00000000,
        CS31: 0x00000000,
        ID31: 0x00000000,
        MB31_8B_WORD0: 0x00000000,
        MB31_8B_WORD1: 0x00000000,
        CS32: 0x00000000,
        ID32: 0x00000000,
        MB0_16B_WORD0_H: 0x00000000,
        MB0_16B_WORD1_H: 0x00000000,
        CS33: 0x00000000,
        ID33: 0x00000000,
        MB0_32B_WORD4_H: 0x00000000,
        MB0_32B_WORD5_H: 0x00000000,
        CS34: 0x00000000,
        ID34: 0x00000000,
        MB0_64B_WORD8_H: 0x00000000,
        MB0_64B_WORD9_H: 0x00000000,
        CS35: 0x00000000,
        ID35: 0x00000000,
        MB0_64B_WORD12_H: 0x00000000,
        MB0_64B_WORD13_H: 0x00000000,
        CS36: 0x00000000,
        ID36: 0x00000000,
        MB1_32B_WORD6_H: 0x00000000,
        MB1_32B_WORD7_H: 0x00000000,
        CS37: 0x00000000,
        ID37: 0x00000000,
        MB1_64B_WORD2_H: 0x00000000,
        MB1_64B_WORD3_H: 0x00000000,
        CS38: 0x00000000,
        ID38: 0x00000000,
        MB1_64B_WORD6_H: 0x00000000,
        MB1_64B_WORD7_H: 0x00000000,
        CS39: 0x00000000,
        ID39: 0x00000000,
        MB1_64B_WORD10_H: 0x00000000,
        MB1_64B_WORD11_H: 0x00000000,
        CS40: 0x00000000,
        ID40: 0x00000000,
        MB1_64B_WORD14_H: 0x00000000,
        MB1_64B_WORD15_H: 0x00000000,
        CS41: 0x00000000,
        ID41: 0x00000000,
        MB2_64B_WORD0_H: 0x00000000,
        MB2_64B_WORD1_H: 0x00000000,
        CS42: 0x00000000,
        ID42: 0x00000000,
        MB2_64B_WORD4_H: 0x00000000,
        MB2_64B_WORD5_H: 0x00000000,
        CS43: 0x00000000,
        ID43: 0x00000000,
        MB2_64B_WORD8_H: 0x00000000,
        MB2_64B_WORD9_H: 0x00000000,
        CS44: 0x00000000,
        ID44: 0x00000000,
        MB2_64B_WORD12_H: 0x00000000,
        MB2_64B_WORD13_H: 0x00000000,
        CS45: 0x00000000,
        ID45: 0x00000000,
        MB3_64B_CS_H: 0x00000000,
        MB3_64B_ID_H: 0x00000000,
        CS46: 0x00000000,
        ID46: 0x00000000,
        MB3_64B_WORD2_H: 0x00000000,
        MB3_64B_WORD3_H: 0x00000000,
        CS47: 0x00000000,
        ID47: 0x00000000,
        MB10_16B_WORD0_H: 0x00000000,
        MB10_16B_WORD1_H: 0x00000000,
        CS48: 0x00000000,
        ID48: 0x00000000,
        MB11_16B_CS_H: 0x00000000,
        MB11_16B_ID_H: 0x00000000,
        CS49: 0x00000000,
        ID49: 0x00000000,
        MB11_16B_WORD2_H: 0x00000000,
        MB11_16B_WORD3_H: 0x00000000,
        CS50: 0x00000000,
        ID50: 0x00000000,
        MB12_16B_WORD0_H: 0x00000000,
        MB12_16B_WORD1_H: 0x00000000,
        CS51: 0x00000000,
        ID51: 0x00000000,
        MB13_16B_CS_H: 0x00000000,
        MB13_16B_ID_H: 0x00000000,
        CS52: 0x00000000,
        ID52: 0x00000000,
        MB13_16B_WORD2_H: 0x00000000,
        MB13_16B_WORD3_H: 0x00000000,
        CS53: 0x00000000,
        ID53: 0x00000000,
        MB14_16B_WORD0_H: 0x00000000,
        MB14_16B_WORD1_H: 0x00000000,
        CS54: 0x00000000,
        ID54: 0x00000000,
        MB15_16B_CS_H: 0x00000000,
        MB15_16B_ID_H: 0x00000000,
        CS55: 0x00000000,
        ID55: 0x00000000,
        MB15_16B_WORD2_H: 0x00000000,
        MB15_16B_WORD3_H: 0x00000000,
        CS56: 0x00000000,
        ID56: 0x00000000,
        MB16_16B_WORD0_H: 0x00000000,
        MB16_16B_WORD1_H: 0x00000000,
        CS57: 0x00000000,
        ID57: 0x00000000,
        MB10_32B_WORD0_H: 0x00000000,
        MB10_32B_WORD1_H: 0x00000000,
        CS58: 0x00000000,
        ID58: 0x00000000,
        MB10_32B_WORD4_H: 0x00000000,
        MB10_32B_WORD5_H: 0x00000000,
        CS59: 0x00000000,
        ID59: 0x00000000,
        MB11_32B_CS_H: 0x00000000,
        MB11_32B_ID_H: 0x00000000,
        CS60: 0x00000000,
        ID60: 0x00000000,
        MB11_32B_WORD2_H: 0x00000000,
        MB11_32B_WORD3_H: 0x00000000,
        CS61: 0x00000000,
        ID61: 0x00000000,
        MB11_32B_WORD6_H: 0x00000000,
        MB11_32B_WORD7_H: 0x00000000,
        CS62: 0x00000000,
        ID62: 0x00000000,
        MB20_16B_WORD0_H: 0x00000000,
        MB20_16B_WORD1_H: 0x00000000,
        CS63: 0x00000000,
        ID63: 0x00000000,
        MB63_8B_WORD0: 0x00000000,
        MB63_8B_WORD1: 0x00000000,
        RXIMR0: 0x00000000,
        RXIMR1: 0x00000000,
        RXIMR2: 0x00000000,
        RXIMR3: 0x00000000,
        RXIMR4: 0x00000000,
        RXIMR5: 0x00000000,
        RXIMR6: 0x00000000,
        RXIMR7: 0x00000000,
        RXIMR8: 0x00000000,
        RXIMR9: 0x00000000,
        RXIMR10: 0x00000000,
        RXIMR11: 0x00000000,
        RXIMR12: 0x00000000,
        RXIMR13: 0x00000000,
        RXIMR14: 0x00000000,
        RXIMR15: 0x00000000,
        RXIMR16: 0x00000000,
        RXIMR17: 0x00000000,
        RXIMR18: 0x00000000,
        RXIMR19: 0x00000000,
        RXIMR20: 0x00000000,
        RXIMR21: 0x00000000,
        RXIMR22: 0x00000000,
        RXIMR23: 0x00000000,
        RXIMR24: 0x00000000,
        RXIMR25: 0x00000000,
        RXIMR26: 0x00000000,
        RXIMR27: 0x00000000,
        RXIMR28: 0x00000000,
        RXIMR29: 0x00000000,
        RXIMR30: 0x00000000,
        RXIMR31: 0x00000000,
        RXIMR32: 0x00000000,
        RXIMR33: 0x00000000,
        RXIMR34: 0x00000000,
        RXIMR35: 0x00000000,
        RXIMR36: 0x00000000,
        RXIMR37: 0x00000000,
        RXIMR38: 0x00000000,
        RXIMR39: 0x00000000,
        RXIMR40: 0x00000000,
        RXIMR41: 0x00000000,
        RXIMR42: 0x00000000,
        RXIMR43: 0x00000000,
        RXIMR44: 0x00000000,
        RXIMR45: 0x00000000,
        RXIMR46: 0x00000000,
        RXIMR47: 0x00000000,
        RXIMR48: 0x00000000,
        RXIMR49: 0x00000000,
        RXIMR50: 0x00000000,
        RXIMR51: 0x00000000,
        RXIMR52: 0x00000000,
        RXIMR53: 0x00000000,
        RXIMR54: 0x00000000,
        RXIMR55: 0x00000000,
        RXIMR56: 0x00000000,
        RXIMR57: 0x00000000,
        RXIMR58: 0x00000000,
        RXIMR59: 0x00000000,
        RXIMR60: 0x00000000,
        RXIMR61: 0x00000000,
        RXIMR62: 0x00000000,
        RXIMR63: 0x00000000,
        MECR: 0x800C0080,
        ERRIAR: 0x00000000,
        ERRIDPR: 0x00000000,
        ERRIPPR: 0x00000000,
        RERRAR: 0x00000000,
        RERRDR: 0x00000000,
        RERRSYNR: 0x00000000,
        ERRSR: 0x00000000,
        FDCTRL: 0x80000100,
        FDCBT: 0x00000000,
        FDCRC: 0x00000000,
    };

    /// Safe access to CAN2
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
        let taken = CAN2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CAN2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CAN2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CAN2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CAN2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CAN2 {
    /// The interrupts associated with CAN2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] =
        [crate::interrupt::CAN2, crate::interrupt::CAN2_ERROR];

    /// The interrupts associated with CAN2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CAN2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN2: *const RegisterBlock = 0x400c8000 as *const _;

/// The CAN3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CAN3 = Instance<3>;

/// The CAN3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CAN3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct CAN3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CAN3 {}
impl crate::Valid for CAN3 {
    fn take() -> Option<Self> {
        <CAN3>::take()
    }
    fn release(self) {
        <CAN3>::release(self);
    }
    unsafe fn steal() -> Self {
        <CAN3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CAN3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CAN3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CAN3 {
    const INSTANCE: Self = Self {
        addr: 0x40c3c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CAN3, crate::interrupt::CAN3_ERROR],
    };

    /// Reset values for each field in CAN3
    pub const reset: ResetValues = ResetValues {
        MCR: 0x5980000F,
        CTRL1: 0x00000000,
        TIMER: 0x00000000,
        RXMGMASK: 0x00000000,
        RX14MASK: 0x00000000,
        RX15MASK: 0x00000000,
        ECR: 0x00000000,
        ESR1: 0x00000000,
        IMASK2: 0x00000000,
        IMASK1: 0x00000000,
        IFLAG2: 0x00000000,
        IFLAG1: 0x00000000,
        CTRL2: 0x00800000,
        ESR2: 0x00000000,
        CRCR: 0x00000000,
        RXFGMASK: 0x00000000,
        RXFIR: 0x00000000,
        CBT: 0x00000000,
        CS0: 0x00000000,
        ID0: 0x00000000,
        MB0: 0x00000000,
        MB0_16B_WORD1_L: 0x00000000,
        CS1: 0x00000000,
        ID1: 0x00000000,
        MB: 0x00000000,
        MB0_32B_WORD5_L: 0x00000000,
        CS2: 0x00000000,
        ID2: 0x00000000,
        MB0_64B_WORD8_L: 0x00000000,
        MB0_64B_WORD9_L: 0x00000000,
        CS3: 0x00000000,
        ID3: 0x00000000,
        MB0_64B_WORD12_L: 0x00000000,
        MB0_64B_WORD13_L: 0x00000000,
        CS4: 0x00000000,
        ID4: 0x00000000,
        MB1: 0x00000000,
        MB1_32B_WORD7_L: 0x00000000,
        CS5: 0x00000000,
        ID5: 0x00000000,
        MB1_64B_WORD2_L: 0x00000000,
        MB1_64B_WORD3_L: 0x00000000,
        CS6: 0x00000000,
        ID6: 0x00000000,
        MB1_64B_WORD6_L: 0x00000000,
        MB1_64B_WORD7_L: 0x00000000,
        CS7: 0x00000000,
        ID7: 0x00000000,
        MB1_64B_WORD10_L: 0x00000000,
        MB1_64B_WORD11_L: 0x00000000,
        CS8: 0x00000000,
        ID8: 0x00000000,
        MB1_64B_WORD14_L: 0x00000000,
        MB1_64B_WORD15_L: 0x00000000,
        CS9: 0x00000000,
        ID9: 0x00000000,
        MB2_64B_WORD0_L: 0x00000000,
        MB2_64B_WORD1_L: 0x00000000,
        CS10: 0x00000000,
        ID10: 0x00000000,
        MB10_8B_WORD0: 0x00000000,
        MB10_8B_WORD1: 0x00000000,
        CS11: 0x00000000,
        ID11: 0x00000000,
        MB11_8B_WORD0: 0x00000000,
        MB11_8B_WORD1: 0x00000000,
        CS12: 0x00000000,
        ID12: 0x00000000,
        MB12_8B_WORD0: 0x00000000,
        MB12_8B_WORD1: 0x00000000,
        CS13: 0x00000000,
        ID13: 0x00000000,
        MB13_8B_WORD0: 0x00000000,
        MB13_8B_WORD1: 0x00000000,
        CS14: 0x00000000,
        ID14: 0x00000000,
        MB14_8B_WORD0: 0x00000000,
        MB14_8B_WORD1: 0x00000000,
        CS15: 0x00000000,
        ID15: 0x00000000,
        MB10_16B_WORD0_L: 0x00000000,
        MB10_16B_WORD1_L: 0x00000000,
        CS16: 0x00000000,
        ID16: 0x00000000,
        MB11_16B_CS_L: 0x00000000,
        MB11_16B_ID_L: 0x00000000,
        CS17: 0x00000000,
        ID17: 0x00000000,
        MB11_16B_WORD2_L: 0x00000000,
        MB11_16B_WORD3_L: 0x00000000,
        CS18: 0x00000000,
        ID18: 0x00000000,
        MB12_16B_WORD0_L: 0x00000000,
        MB12_16B_WORD1_L: 0x00000000,
        CS19: 0x00000000,
        ID19: 0x00000000,
        MB13_16B_CS_L: 0x00000000,
        MB13_16B_ID_L: 0x00000000,
        CS20: 0x00000000,
        ID20: 0x00000000,
        MB13_16B_WORD2_L: 0x00000000,
        MB13_16B_WORD3_L: 0x00000000,
        CS21: 0x00000000,
        ID21: 0x00000000,
        MB14_16B_WORD0_L: 0x00000000,
        MB14_16B_WORD1_L: 0x00000000,
        CS22: 0x00000000,
        ID22: 0x00000000,
        MB15_16B_CS_L: 0x00000000,
        MB15_16B_ID_L: 0x00000000,
        CS23: 0x00000000,
        ID23: 0x00000000,
        MB15_16B_WORD2_L: 0x00000000,
        MB15_16B_WORD3_L: 0x00000000,
        CS24: 0x00000000,
        ID24: 0x00000000,
        MB16_16B_WORD0_L: 0x00000000,
        MB16_16B_WORD1_L: 0x00000000,
        CS25: 0x00000000,
        ID25: 0x00000000,
        MB10_32B_WORD0_L: 0x00000000,
        MB10_32B_WORD1_L: 0x00000000,
        CS26: 0x00000000,
        ID26: 0x00000000,
        MB10_32B_WORD4_L: 0x00000000,
        MB10_32B_WORD5_L: 0x00000000,
        CS27: 0x00000000,
        ID27: 0x00000000,
        MB11_32B_CS_L: 0x00000000,
        MB11_32B_ID_L: 0x00000000,
        CS28: 0x00000000,
        ID28: 0x00000000,
        MB11_32B_WORD2_L: 0x00000000,
        MB11_32B_WORD3_L: 0x00000000,
        CS29: 0x00000000,
        ID29: 0x00000000,
        MB11_32B_WORD6_L: 0x00000000,
        MB11_32B_WORD7_L: 0x00000000,
        CS30: 0x00000000,
        ID30: 0x00000000,
        MB20_16B_WORD0_L: 0x00000000,
        MB20_16B_WORD1_L: 0x00000000,
        CS31: 0x00000000,
        ID31: 0x00000000,
        MB31_8B_WORD0: 0x00000000,
        MB31_8B_WORD1: 0x00000000,
        CS32: 0x00000000,
        ID32: 0x00000000,
        MB0_16B_WORD0_H: 0x00000000,
        MB0_16B_WORD1_H: 0x00000000,
        CS33: 0x00000000,
        ID33: 0x00000000,
        MB0_32B_WORD4_H: 0x00000000,
        MB0_32B_WORD5_H: 0x00000000,
        CS34: 0x00000000,
        ID34: 0x00000000,
        MB0_64B_WORD8_H: 0x00000000,
        MB0_64B_WORD9_H: 0x00000000,
        CS35: 0x00000000,
        ID35: 0x00000000,
        MB0_64B_WORD12_H: 0x00000000,
        MB0_64B_WORD13_H: 0x00000000,
        CS36: 0x00000000,
        ID36: 0x00000000,
        MB1_32B_WORD6_H: 0x00000000,
        MB1_32B_WORD7_H: 0x00000000,
        CS37: 0x00000000,
        ID37: 0x00000000,
        MB1_64B_WORD2_H: 0x00000000,
        MB1_64B_WORD3_H: 0x00000000,
        CS38: 0x00000000,
        ID38: 0x00000000,
        MB1_64B_WORD6_H: 0x00000000,
        MB1_64B_WORD7_H: 0x00000000,
        CS39: 0x00000000,
        ID39: 0x00000000,
        MB1_64B_WORD10_H: 0x00000000,
        MB1_64B_WORD11_H: 0x00000000,
        CS40: 0x00000000,
        ID40: 0x00000000,
        MB1_64B_WORD14_H: 0x00000000,
        MB1_64B_WORD15_H: 0x00000000,
        CS41: 0x00000000,
        ID41: 0x00000000,
        MB2_64B_WORD0_H: 0x00000000,
        MB2_64B_WORD1_H: 0x00000000,
        CS42: 0x00000000,
        ID42: 0x00000000,
        MB2_64B_WORD4_H: 0x00000000,
        MB2_64B_WORD5_H: 0x00000000,
        CS43: 0x00000000,
        ID43: 0x00000000,
        MB2_64B_WORD8_H: 0x00000000,
        MB2_64B_WORD9_H: 0x00000000,
        CS44: 0x00000000,
        ID44: 0x00000000,
        MB2_64B_WORD12_H: 0x00000000,
        MB2_64B_WORD13_H: 0x00000000,
        CS45: 0x00000000,
        ID45: 0x00000000,
        MB3_64B_CS_H: 0x00000000,
        MB3_64B_ID_H: 0x00000000,
        CS46: 0x00000000,
        ID46: 0x00000000,
        MB3_64B_WORD2_H: 0x00000000,
        MB3_64B_WORD3_H: 0x00000000,
        CS47: 0x00000000,
        ID47: 0x00000000,
        MB10_16B_WORD0_H: 0x00000000,
        MB10_16B_WORD1_H: 0x00000000,
        CS48: 0x00000000,
        ID48: 0x00000000,
        MB11_16B_CS_H: 0x00000000,
        MB11_16B_ID_H: 0x00000000,
        CS49: 0x00000000,
        ID49: 0x00000000,
        MB11_16B_WORD2_H: 0x00000000,
        MB11_16B_WORD3_H: 0x00000000,
        CS50: 0x00000000,
        ID50: 0x00000000,
        MB12_16B_WORD0_H: 0x00000000,
        MB12_16B_WORD1_H: 0x00000000,
        CS51: 0x00000000,
        ID51: 0x00000000,
        MB13_16B_CS_H: 0x00000000,
        MB13_16B_ID_H: 0x00000000,
        CS52: 0x00000000,
        ID52: 0x00000000,
        MB13_16B_WORD2_H: 0x00000000,
        MB13_16B_WORD3_H: 0x00000000,
        CS53: 0x00000000,
        ID53: 0x00000000,
        MB14_16B_WORD0_H: 0x00000000,
        MB14_16B_WORD1_H: 0x00000000,
        CS54: 0x00000000,
        ID54: 0x00000000,
        MB15_16B_CS_H: 0x00000000,
        MB15_16B_ID_H: 0x00000000,
        CS55: 0x00000000,
        ID55: 0x00000000,
        MB15_16B_WORD2_H: 0x00000000,
        MB15_16B_WORD3_H: 0x00000000,
        CS56: 0x00000000,
        ID56: 0x00000000,
        MB16_16B_WORD0_H: 0x00000000,
        MB16_16B_WORD1_H: 0x00000000,
        CS57: 0x00000000,
        ID57: 0x00000000,
        MB10_32B_WORD0_H: 0x00000000,
        MB10_32B_WORD1_H: 0x00000000,
        CS58: 0x00000000,
        ID58: 0x00000000,
        MB10_32B_WORD4_H: 0x00000000,
        MB10_32B_WORD5_H: 0x00000000,
        CS59: 0x00000000,
        ID59: 0x00000000,
        MB11_32B_CS_H: 0x00000000,
        MB11_32B_ID_H: 0x00000000,
        CS60: 0x00000000,
        ID60: 0x00000000,
        MB11_32B_WORD2_H: 0x00000000,
        MB11_32B_WORD3_H: 0x00000000,
        CS61: 0x00000000,
        ID61: 0x00000000,
        MB11_32B_WORD6_H: 0x00000000,
        MB11_32B_WORD7_H: 0x00000000,
        CS62: 0x00000000,
        ID62: 0x00000000,
        MB20_16B_WORD0_H: 0x00000000,
        MB20_16B_WORD1_H: 0x00000000,
        CS63: 0x00000000,
        ID63: 0x00000000,
        MB63_8B_WORD0: 0x00000000,
        MB63_8B_WORD1: 0x00000000,
        RXIMR0: 0x00000000,
        RXIMR1: 0x00000000,
        RXIMR2: 0x00000000,
        RXIMR3: 0x00000000,
        RXIMR4: 0x00000000,
        RXIMR5: 0x00000000,
        RXIMR6: 0x00000000,
        RXIMR7: 0x00000000,
        RXIMR8: 0x00000000,
        RXIMR9: 0x00000000,
        RXIMR10: 0x00000000,
        RXIMR11: 0x00000000,
        RXIMR12: 0x00000000,
        RXIMR13: 0x00000000,
        RXIMR14: 0x00000000,
        RXIMR15: 0x00000000,
        RXIMR16: 0x00000000,
        RXIMR17: 0x00000000,
        RXIMR18: 0x00000000,
        RXIMR19: 0x00000000,
        RXIMR20: 0x00000000,
        RXIMR21: 0x00000000,
        RXIMR22: 0x00000000,
        RXIMR23: 0x00000000,
        RXIMR24: 0x00000000,
        RXIMR25: 0x00000000,
        RXIMR26: 0x00000000,
        RXIMR27: 0x00000000,
        RXIMR28: 0x00000000,
        RXIMR29: 0x00000000,
        RXIMR30: 0x00000000,
        RXIMR31: 0x00000000,
        RXIMR32: 0x00000000,
        RXIMR33: 0x00000000,
        RXIMR34: 0x00000000,
        RXIMR35: 0x00000000,
        RXIMR36: 0x00000000,
        RXIMR37: 0x00000000,
        RXIMR38: 0x00000000,
        RXIMR39: 0x00000000,
        RXIMR40: 0x00000000,
        RXIMR41: 0x00000000,
        RXIMR42: 0x00000000,
        RXIMR43: 0x00000000,
        RXIMR44: 0x00000000,
        RXIMR45: 0x00000000,
        RXIMR46: 0x00000000,
        RXIMR47: 0x00000000,
        RXIMR48: 0x00000000,
        RXIMR49: 0x00000000,
        RXIMR50: 0x00000000,
        RXIMR51: 0x00000000,
        RXIMR52: 0x00000000,
        RXIMR53: 0x00000000,
        RXIMR54: 0x00000000,
        RXIMR55: 0x00000000,
        RXIMR56: 0x00000000,
        RXIMR57: 0x00000000,
        RXIMR58: 0x00000000,
        RXIMR59: 0x00000000,
        RXIMR60: 0x00000000,
        RXIMR61: 0x00000000,
        RXIMR62: 0x00000000,
        RXIMR63: 0x00000000,
        MECR: 0x800C0080,
        ERRIAR: 0x00000000,
        ERRIDPR: 0x00000000,
        ERRIPPR: 0x00000000,
        RERRAR: 0x00000000,
        RERRDR: 0x00000000,
        RERRSYNR: 0x00000000,
        ERRSR: 0x00000000,
        FDCTRL: 0x80000100,
        FDCBT: 0x00000000,
        FDCRC: 0x00000000,
    };

    /// Safe access to CAN3
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
        let taken = CAN3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CAN3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CAN3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CAN3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CAN3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CAN3 {
    /// The interrupts associated with CAN3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] =
        [crate::interrupt::CAN3, crate::interrupt::CAN3_ERROR];

    /// The interrupts associated with CAN3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CAN3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN3: *const RegisterBlock = 0x40c3c000 as *const _;
