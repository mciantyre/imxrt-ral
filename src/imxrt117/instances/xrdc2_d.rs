#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! XRDC2
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::xrdc2_d::Instance;
pub use crate::imxrt117::peripherals::xrdc2_d::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::xrdc2_d::{
    MCR, MSC_MSAC_W0_0, MSC_MSAC_W0_1, MSC_MSAC_W0_10, MSC_MSAC_W0_100, MSC_MSAC_W0_101,
    MSC_MSAC_W0_102, MSC_MSAC_W0_103, MSC_MSAC_W0_104, MSC_MSAC_W0_105, MSC_MSAC_W0_106,
    MSC_MSAC_W0_107, MSC_MSAC_W0_108, MSC_MSAC_W0_109, MSC_MSAC_W0_11, MSC_MSAC_W0_110,
    MSC_MSAC_W0_111, MSC_MSAC_W0_112, MSC_MSAC_W0_113, MSC_MSAC_W0_114, MSC_MSAC_W0_115,
    MSC_MSAC_W0_116, MSC_MSAC_W0_117, MSC_MSAC_W0_118, MSC_MSAC_W0_119, MSC_MSAC_W0_12,
    MSC_MSAC_W0_120, MSC_MSAC_W0_121, MSC_MSAC_W0_122, MSC_MSAC_W0_123, MSC_MSAC_W0_124,
    MSC_MSAC_W0_125, MSC_MSAC_W0_126, MSC_MSAC_W0_127, MSC_MSAC_W0_13, MSC_MSAC_W0_14,
    MSC_MSAC_W0_15, MSC_MSAC_W0_16, MSC_MSAC_W0_17, MSC_MSAC_W0_18, MSC_MSAC_W0_19, MSC_MSAC_W0_2,
    MSC_MSAC_W0_20, MSC_MSAC_W0_21, MSC_MSAC_W0_22, MSC_MSAC_W0_23, MSC_MSAC_W0_24, MSC_MSAC_W0_25,
    MSC_MSAC_W0_26, MSC_MSAC_W0_27, MSC_MSAC_W0_28, MSC_MSAC_W0_29, MSC_MSAC_W0_3, MSC_MSAC_W0_30,
    MSC_MSAC_W0_31, MSC_MSAC_W0_32, MSC_MSAC_W0_33, MSC_MSAC_W0_34, MSC_MSAC_W0_35, MSC_MSAC_W0_36,
    MSC_MSAC_W0_37, MSC_MSAC_W0_38, MSC_MSAC_W0_39, MSC_MSAC_W0_4, MSC_MSAC_W0_40, MSC_MSAC_W0_41,
    MSC_MSAC_W0_42, MSC_MSAC_W0_43, MSC_MSAC_W0_44, MSC_MSAC_W0_45, MSC_MSAC_W0_46, MSC_MSAC_W0_47,
    MSC_MSAC_W0_48, MSC_MSAC_W0_49, MSC_MSAC_W0_5, MSC_MSAC_W0_50, MSC_MSAC_W0_51, MSC_MSAC_W0_52,
    MSC_MSAC_W0_53, MSC_MSAC_W0_54, MSC_MSAC_W0_55, MSC_MSAC_W0_56, MSC_MSAC_W0_57, MSC_MSAC_W0_58,
    MSC_MSAC_W0_59, MSC_MSAC_W0_6, MSC_MSAC_W0_60, MSC_MSAC_W0_61, MSC_MSAC_W0_62, MSC_MSAC_W0_63,
    MSC_MSAC_W0_64, MSC_MSAC_W0_65, MSC_MSAC_W0_66, MSC_MSAC_W0_67, MSC_MSAC_W0_68, MSC_MSAC_W0_69,
    MSC_MSAC_W0_7, MSC_MSAC_W0_70, MSC_MSAC_W0_71, MSC_MSAC_W0_72, MSC_MSAC_W0_73, MSC_MSAC_W0_74,
    MSC_MSAC_W0_75, MSC_MSAC_W0_76, MSC_MSAC_W0_77, MSC_MSAC_W0_78, MSC_MSAC_W0_79, MSC_MSAC_W0_8,
    MSC_MSAC_W0_80, MSC_MSAC_W0_81, MSC_MSAC_W0_82, MSC_MSAC_W0_83, MSC_MSAC_W0_84, MSC_MSAC_W0_85,
    MSC_MSAC_W0_86, MSC_MSAC_W0_87, MSC_MSAC_W0_88, MSC_MSAC_W0_89, MSC_MSAC_W0_9, MSC_MSAC_W0_90,
    MSC_MSAC_W0_91, MSC_MSAC_W0_92, MSC_MSAC_W0_93, MSC_MSAC_W0_94, MSC_MSAC_W0_95, MSC_MSAC_W0_96,
    MSC_MSAC_W0_97, MSC_MSAC_W0_98, MSC_MSAC_W0_99, MSC_MSAC_W1_0, MSC_MSAC_W1_1, MSC_MSAC_W1_10,
    MSC_MSAC_W1_100, MSC_MSAC_W1_101, MSC_MSAC_W1_102, MSC_MSAC_W1_103, MSC_MSAC_W1_104,
    MSC_MSAC_W1_105, MSC_MSAC_W1_106, MSC_MSAC_W1_107, MSC_MSAC_W1_108, MSC_MSAC_W1_109,
    MSC_MSAC_W1_11, MSC_MSAC_W1_110, MSC_MSAC_W1_111, MSC_MSAC_W1_112, MSC_MSAC_W1_113,
    MSC_MSAC_W1_114, MSC_MSAC_W1_115, MSC_MSAC_W1_116, MSC_MSAC_W1_117, MSC_MSAC_W1_118,
    MSC_MSAC_W1_119, MSC_MSAC_W1_12, MSC_MSAC_W1_120, MSC_MSAC_W1_121, MSC_MSAC_W1_122,
    MSC_MSAC_W1_123, MSC_MSAC_W1_124, MSC_MSAC_W1_125, MSC_MSAC_W1_126, MSC_MSAC_W1_127,
    MSC_MSAC_W1_13, MSC_MSAC_W1_14, MSC_MSAC_W1_15, MSC_MSAC_W1_16, MSC_MSAC_W1_17, MSC_MSAC_W1_18,
    MSC_MSAC_W1_19, MSC_MSAC_W1_2, MSC_MSAC_W1_20, MSC_MSAC_W1_21, MSC_MSAC_W1_22, MSC_MSAC_W1_23,
    MSC_MSAC_W1_24, MSC_MSAC_W1_25, MSC_MSAC_W1_26, MSC_MSAC_W1_27, MSC_MSAC_W1_28, MSC_MSAC_W1_29,
    MSC_MSAC_W1_3, MSC_MSAC_W1_30, MSC_MSAC_W1_31, MSC_MSAC_W1_32, MSC_MSAC_W1_33, MSC_MSAC_W1_34,
    MSC_MSAC_W1_35, MSC_MSAC_W1_36, MSC_MSAC_W1_37, MSC_MSAC_W1_38, MSC_MSAC_W1_39, MSC_MSAC_W1_4,
    MSC_MSAC_W1_40, MSC_MSAC_W1_41, MSC_MSAC_W1_42, MSC_MSAC_W1_43, MSC_MSAC_W1_44, MSC_MSAC_W1_45,
    MSC_MSAC_W1_46, MSC_MSAC_W1_47, MSC_MSAC_W1_48, MSC_MSAC_W1_49, MSC_MSAC_W1_5, MSC_MSAC_W1_50,
    MSC_MSAC_W1_51, MSC_MSAC_W1_52, MSC_MSAC_W1_53, MSC_MSAC_W1_54, MSC_MSAC_W1_55, MSC_MSAC_W1_56,
    MSC_MSAC_W1_57, MSC_MSAC_W1_58, MSC_MSAC_W1_59, MSC_MSAC_W1_6, MSC_MSAC_W1_60, MSC_MSAC_W1_61,
    MSC_MSAC_W1_62, MSC_MSAC_W1_63, MSC_MSAC_W1_64, MSC_MSAC_W1_65, MSC_MSAC_W1_66, MSC_MSAC_W1_67,
    MSC_MSAC_W1_68, MSC_MSAC_W1_69, MSC_MSAC_W1_7, MSC_MSAC_W1_70, MSC_MSAC_W1_71, MSC_MSAC_W1_72,
    MSC_MSAC_W1_73, MSC_MSAC_W1_74, MSC_MSAC_W1_75, MSC_MSAC_W1_76, MSC_MSAC_W1_77, MSC_MSAC_W1_78,
    MSC_MSAC_W1_79, MSC_MSAC_W1_8, MSC_MSAC_W1_80, MSC_MSAC_W1_81, MSC_MSAC_W1_82, MSC_MSAC_W1_83,
    MSC_MSAC_W1_84, MSC_MSAC_W1_85, MSC_MSAC_W1_86, MSC_MSAC_W1_87, MSC_MSAC_W1_88, MSC_MSAC_W1_89,
    MSC_MSAC_W1_9, MSC_MSAC_W1_90, MSC_MSAC_W1_91, MSC_MSAC_W1_92, MSC_MSAC_W1_93, MSC_MSAC_W1_94,
    MSC_MSAC_W1_95, MSC_MSAC_W1_96, MSC_MSAC_W1_97, MSC_MSAC_W1_98, MSC_MSAC_W1_99, SR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The XRDC2_D0 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type XRDC2_D0 = Instance<0>;

/// The XRDC2_D0 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type XRDC2_D0 = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct XRDC2_D0 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for XRDC2_D0 {}
impl crate::Valid for XRDC2_D0 {
    fn take() -> Option<Self> {
        <XRDC2_D0>::take()
    }
    fn release(self) {
        <XRDC2_D0>::release(self);
    }
    unsafe fn steal() -> Self {
        <XRDC2_D0>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static XRDC2_D0_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the XRDC2_D0 peripheral instance
#[cfg(not(feature = "nosync"))]
impl XRDC2_D0 {
    const INSTANCE: Self = Self {
        addr: 0x40ce0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in XRDC2_D0
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00000000,
        SR: 0x00000010,
        MSC_MSAC_W0_0: 0x00000000,
        MSC_MSAC_W1_0: 0x00000000,
        MSC_MSAC_W0_1: 0x00000000,
        MSC_MSAC_W1_1: 0x00000000,
        MSC_MSAC_W0_2: 0x00000000,
        MSC_MSAC_W1_2: 0x00000000,
        MSC_MSAC_W0_3: 0x00000000,
        MSC_MSAC_W1_3: 0x00000000,
        MSC_MSAC_W0_4: 0x00000000,
        MSC_MSAC_W1_4: 0x00000000,
        MSC_MSAC_W0_5: 0x00000000,
        MSC_MSAC_W1_5: 0x00000000,
        MSC_MSAC_W0_6: 0x00000000,
        MSC_MSAC_W1_6: 0x00000000,
        MSC_MSAC_W0_7: 0x00000000,
        MSC_MSAC_W1_7: 0x00000000,
        MSC_MSAC_W0_8: 0x00000000,
        MSC_MSAC_W1_8: 0x00000000,
        MSC_MSAC_W0_9: 0x00000000,
        MSC_MSAC_W1_9: 0x00000000,
        MSC_MSAC_W0_10: 0x00000000,
        MSC_MSAC_W1_10: 0x00000000,
        MSC_MSAC_W0_11: 0x00000000,
        MSC_MSAC_W1_11: 0x00000000,
        MSC_MSAC_W0_12: 0x00000000,
        MSC_MSAC_W1_12: 0x00000000,
        MSC_MSAC_W0_13: 0x00000000,
        MSC_MSAC_W1_13: 0x00000000,
        MSC_MSAC_W0_14: 0x00000000,
        MSC_MSAC_W1_14: 0x00000000,
        MSC_MSAC_W0_15: 0x00000000,
        MSC_MSAC_W1_15: 0x00000000,
        MSC_MSAC_W0_16: 0x00000000,
        MSC_MSAC_W1_16: 0x00000000,
        MSC_MSAC_W0_17: 0x00000000,
        MSC_MSAC_W1_17: 0x00000000,
        MSC_MSAC_W0_18: 0x00000000,
        MSC_MSAC_W1_18: 0x00000000,
        MSC_MSAC_W0_19: 0x00000000,
        MSC_MSAC_W1_19: 0x00000000,
        MSC_MSAC_W0_20: 0x00000000,
        MSC_MSAC_W1_20: 0x00000000,
        MSC_MSAC_W0_21: 0x00000000,
        MSC_MSAC_W1_21: 0x00000000,
        MSC_MSAC_W0_22: 0x00000000,
        MSC_MSAC_W1_22: 0x00000000,
        MSC_MSAC_W0_23: 0x00000000,
        MSC_MSAC_W1_23: 0x00000000,
        MSC_MSAC_W0_24: 0x00000000,
        MSC_MSAC_W1_24: 0x00000000,
        MSC_MSAC_W0_25: 0x00000000,
        MSC_MSAC_W1_25: 0x00000000,
        MSC_MSAC_W0_26: 0x00000000,
        MSC_MSAC_W1_26: 0x00000000,
        MSC_MSAC_W0_27: 0x00000000,
        MSC_MSAC_W1_27: 0x00000000,
        MSC_MSAC_W0_28: 0x00000000,
        MSC_MSAC_W1_28: 0x00000000,
        MSC_MSAC_W0_29: 0x00000000,
        MSC_MSAC_W1_29: 0x00000000,
        MSC_MSAC_W0_30: 0x00000000,
        MSC_MSAC_W1_30: 0x00000000,
        MSC_MSAC_W0_31: 0x00000000,
        MSC_MSAC_W1_31: 0x00000000,
        MSC_MSAC_W0_32: 0x00000000,
        MSC_MSAC_W1_32: 0x00000000,
        MSC_MSAC_W0_33: 0x00000000,
        MSC_MSAC_W1_33: 0x00000000,
        MSC_MSAC_W0_34: 0x00000000,
        MSC_MSAC_W1_34: 0x00000000,
        MSC_MSAC_W0_35: 0x00000000,
        MSC_MSAC_W1_35: 0x00000000,
        MSC_MSAC_W0_36: 0x00000000,
        MSC_MSAC_W1_36: 0x00000000,
        MSC_MSAC_W0_37: 0x00000000,
        MSC_MSAC_W1_37: 0x00000000,
        MSC_MSAC_W0_38: 0x00000000,
        MSC_MSAC_W1_38: 0x00000000,
        MSC_MSAC_W0_39: 0x00000000,
        MSC_MSAC_W1_39: 0x00000000,
        MSC_MSAC_W0_40: 0x00000000,
        MSC_MSAC_W1_40: 0x00000000,
        MSC_MSAC_W0_41: 0x00000000,
        MSC_MSAC_W1_41: 0x00000000,
        MSC_MSAC_W0_42: 0x00000000,
        MSC_MSAC_W1_42: 0x00000000,
        MSC_MSAC_W0_43: 0x00000000,
        MSC_MSAC_W1_43: 0x00000000,
        MSC_MSAC_W0_44: 0x00000000,
        MSC_MSAC_W1_44: 0x00000000,
        MSC_MSAC_W0_45: 0x00000000,
        MSC_MSAC_W1_45: 0x00000000,
        MSC_MSAC_W0_46: 0x00000000,
        MSC_MSAC_W1_46: 0x00000000,
        MSC_MSAC_W0_47: 0x00000000,
        MSC_MSAC_W1_47: 0x00000000,
        MSC_MSAC_W0_48: 0x00000000,
        MSC_MSAC_W1_48: 0x00000000,
        MSC_MSAC_W0_49: 0x00000000,
        MSC_MSAC_W1_49: 0x00000000,
        MSC_MSAC_W0_50: 0x00000000,
        MSC_MSAC_W1_50: 0x00000000,
        MSC_MSAC_W0_51: 0x00000000,
        MSC_MSAC_W1_51: 0x00000000,
        MSC_MSAC_W0_52: 0x00000000,
        MSC_MSAC_W1_52: 0x00000000,
        MSC_MSAC_W0_53: 0x00000000,
        MSC_MSAC_W1_53: 0x00000000,
        MSC_MSAC_W0_54: 0x00000000,
        MSC_MSAC_W1_54: 0x00000000,
        MSC_MSAC_W0_55: 0x00000000,
        MSC_MSAC_W1_55: 0x00000000,
        MSC_MSAC_W0_56: 0x00000000,
        MSC_MSAC_W1_56: 0x00000000,
        MSC_MSAC_W0_57: 0x00000000,
        MSC_MSAC_W1_57: 0x00000000,
        MSC_MSAC_W0_58: 0x00000000,
        MSC_MSAC_W1_58: 0x00000000,
        MSC_MSAC_W0_59: 0x00000000,
        MSC_MSAC_W1_59: 0x00000000,
        MSC_MSAC_W0_60: 0x00000000,
        MSC_MSAC_W1_60: 0x00000000,
        MSC_MSAC_W0_61: 0x00000000,
        MSC_MSAC_W1_61: 0x00000000,
        MSC_MSAC_W0_62: 0x00000000,
        MSC_MSAC_W1_62: 0x00000000,
        MSC_MSAC_W0_63: 0x00000000,
        MSC_MSAC_W1_63: 0x00000000,
        MSC_MSAC_W0_64: 0x00000000,
        MSC_MSAC_W1_64: 0x00000000,
        MSC_MSAC_W0_65: 0x00000000,
        MSC_MSAC_W1_65: 0x00000000,
        MSC_MSAC_W0_66: 0x00000000,
        MSC_MSAC_W1_66: 0x00000000,
        MSC_MSAC_W0_67: 0x00000000,
        MSC_MSAC_W1_67: 0x00000000,
        MSC_MSAC_W0_68: 0x00000000,
        MSC_MSAC_W1_68: 0x00000000,
        MSC_MSAC_W0_69: 0x00000000,
        MSC_MSAC_W1_69: 0x00000000,
        MSC_MSAC_W0_70: 0x00000000,
        MSC_MSAC_W1_70: 0x00000000,
        MSC_MSAC_W0_71: 0x00000000,
        MSC_MSAC_W1_71: 0x00000000,
        MSC_MSAC_W0_72: 0x00000000,
        MSC_MSAC_W1_72: 0x00000000,
        MSC_MSAC_W0_73: 0x00000000,
        MSC_MSAC_W1_73: 0x00000000,
        MSC_MSAC_W0_74: 0x00000000,
        MSC_MSAC_W1_74: 0x00000000,
        MSC_MSAC_W0_75: 0x00000000,
        MSC_MSAC_W1_75: 0x00000000,
        MSC_MSAC_W0_76: 0x00000000,
        MSC_MSAC_W1_76: 0x00000000,
        MSC_MSAC_W0_77: 0x00000000,
        MSC_MSAC_W1_77: 0x00000000,
        MSC_MSAC_W0_78: 0x00000000,
        MSC_MSAC_W1_78: 0x00000000,
        MSC_MSAC_W0_79: 0x00000000,
        MSC_MSAC_W1_79: 0x00000000,
        MSC_MSAC_W0_80: 0x00000000,
        MSC_MSAC_W1_80: 0x00000000,
        MSC_MSAC_W0_81: 0x00000000,
        MSC_MSAC_W1_81: 0x00000000,
        MSC_MSAC_W0_82: 0x00000000,
        MSC_MSAC_W1_82: 0x00000000,
        MSC_MSAC_W0_83: 0x00000000,
        MSC_MSAC_W1_83: 0x00000000,
        MSC_MSAC_W0_84: 0x00000000,
        MSC_MSAC_W1_84: 0x00000000,
        MSC_MSAC_W0_85: 0x00000000,
        MSC_MSAC_W1_85: 0x00000000,
        MSC_MSAC_W0_86: 0x00000000,
        MSC_MSAC_W1_86: 0x00000000,
        MSC_MSAC_W0_87: 0x00000000,
        MSC_MSAC_W1_87: 0x00000000,
        MSC_MSAC_W0_88: 0x00000000,
        MSC_MSAC_W1_88: 0x00000000,
        MSC_MSAC_W0_89: 0x00000000,
        MSC_MSAC_W1_89: 0x00000000,
        MSC_MSAC_W0_90: 0x00000000,
        MSC_MSAC_W1_90: 0x00000000,
        MSC_MSAC_W0_91: 0x00000000,
        MSC_MSAC_W1_91: 0x00000000,
        MSC_MSAC_W0_92: 0x00000000,
        MSC_MSAC_W1_92: 0x00000000,
        MSC_MSAC_W0_93: 0x00000000,
        MSC_MSAC_W1_93: 0x00000000,
        MSC_MSAC_W0_94: 0x00000000,
        MSC_MSAC_W1_94: 0x00000000,
        MSC_MSAC_W0_95: 0x00000000,
        MSC_MSAC_W1_95: 0x00000000,
        MSC_MSAC_W0_96: 0x00000000,
        MSC_MSAC_W1_96: 0x00000000,
        MSC_MSAC_W0_97: 0x00000000,
        MSC_MSAC_W1_97: 0x00000000,
        MSC_MSAC_W0_98: 0x00000000,
        MSC_MSAC_W1_98: 0x00000000,
        MSC_MSAC_W0_99: 0x00000000,
        MSC_MSAC_W1_99: 0x00000000,
        MSC_MSAC_W0_100: 0x00000000,
        MSC_MSAC_W1_100: 0x00000000,
        MSC_MSAC_W0_101: 0x00000000,
        MSC_MSAC_W1_101: 0x00000000,
        MSC_MSAC_W0_102: 0x00000000,
        MSC_MSAC_W1_102: 0x00000000,
        MSC_MSAC_W0_103: 0x00000000,
        MSC_MSAC_W1_103: 0x00000000,
        MSC_MSAC_W0_104: 0x00000000,
        MSC_MSAC_W1_104: 0x00000000,
        MSC_MSAC_W0_105: 0x00000000,
        MSC_MSAC_W1_105: 0x00000000,
        MSC_MSAC_W0_106: 0x00000000,
        MSC_MSAC_W1_106: 0x00000000,
        MSC_MSAC_W0_107: 0x00000000,
        MSC_MSAC_W1_107: 0x00000000,
        MSC_MSAC_W0_108: 0x00000000,
        MSC_MSAC_W1_108: 0x00000000,
        MSC_MSAC_W0_109: 0x00000000,
        MSC_MSAC_W1_109: 0x00000000,
        MSC_MSAC_W0_110: 0x00000000,
        MSC_MSAC_W1_110: 0x00000000,
        MSC_MSAC_W0_111: 0x00000000,
        MSC_MSAC_W1_111: 0x00000000,
        MSC_MSAC_W0_112: 0x00000000,
        MSC_MSAC_W1_112: 0x00000000,
        MSC_MSAC_W0_113: 0x00000000,
        MSC_MSAC_W1_113: 0x00000000,
        MSC_MSAC_W0_114: 0x00000000,
        MSC_MSAC_W1_114: 0x00000000,
        MSC_MSAC_W0_115: 0x00000000,
        MSC_MSAC_W1_115: 0x00000000,
        MSC_MSAC_W0_116: 0x00000000,
        MSC_MSAC_W1_116: 0x00000000,
        MSC_MSAC_W0_117: 0x00000000,
        MSC_MSAC_W1_117: 0x00000000,
        MSC_MSAC_W0_118: 0x00000000,
        MSC_MSAC_W1_118: 0x00000000,
        MSC_MSAC_W0_119: 0x00000000,
        MSC_MSAC_W1_119: 0x00000000,
        MSC_MSAC_W0_120: 0x00000000,
        MSC_MSAC_W1_120: 0x00000000,
        MSC_MSAC_W0_121: 0x00000000,
        MSC_MSAC_W1_121: 0x00000000,
        MSC_MSAC_W0_122: 0x00000000,
        MSC_MSAC_W1_122: 0x00000000,
        MSC_MSAC_W0_123: 0x00000000,
        MSC_MSAC_W1_123: 0x00000000,
        MSC_MSAC_W0_124: 0x00000000,
        MSC_MSAC_W1_124: 0x00000000,
        MSC_MSAC_W0_125: 0x00000000,
        MSC_MSAC_W1_125: 0x00000000,
        MSC_MSAC_W0_126: 0x00000000,
        MSC_MSAC_W1_126: 0x00000000,
        MSC_MSAC_W0_127: 0x00000000,
        MSC_MSAC_W1_127: 0x00000000,
    };

    /// Safe access to XRDC2_D0
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
        let taken = XRDC2_D0_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to XRDC2_D0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = XRDC2_D0_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal XRDC2_D0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        XRDC2_D0_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl XRDC2_D0 {
    /// The interrupts associated with XRDC2_D0
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with XRDC2_D0
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to XRDC2_D0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XRDC2_D0: *const RegisterBlock = 0x40ce0000 as *const _;

/// The XRDC2_D1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type XRDC2_D1 = Instance<1>;

/// The XRDC2_D1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type XRDC2_D1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct XRDC2_D1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for XRDC2_D1 {}
impl crate::Valid for XRDC2_D1 {
    fn take() -> Option<Self> {
        <XRDC2_D1>::take()
    }
    fn release(self) {
        <XRDC2_D1>::release(self);
    }
    unsafe fn steal() -> Self {
        <XRDC2_D1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static XRDC2_D1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the XRDC2_D1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl XRDC2_D1 {
    const INSTANCE: Self = Self {
        addr: 0x40cd0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in XRDC2_D1
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00000000,
        SR: 0x00000010,
        MSC_MSAC_W0_0: 0x00000000,
        MSC_MSAC_W1_0: 0x00000000,
        MSC_MSAC_W0_1: 0x00000000,
        MSC_MSAC_W1_1: 0x00000000,
        MSC_MSAC_W0_2: 0x00000000,
        MSC_MSAC_W1_2: 0x00000000,
        MSC_MSAC_W0_3: 0x00000000,
        MSC_MSAC_W1_3: 0x00000000,
        MSC_MSAC_W0_4: 0x00000000,
        MSC_MSAC_W1_4: 0x00000000,
        MSC_MSAC_W0_5: 0x00000000,
        MSC_MSAC_W1_5: 0x00000000,
        MSC_MSAC_W0_6: 0x00000000,
        MSC_MSAC_W1_6: 0x00000000,
        MSC_MSAC_W0_7: 0x00000000,
        MSC_MSAC_W1_7: 0x00000000,
        MSC_MSAC_W0_8: 0x00000000,
        MSC_MSAC_W1_8: 0x00000000,
        MSC_MSAC_W0_9: 0x00000000,
        MSC_MSAC_W1_9: 0x00000000,
        MSC_MSAC_W0_10: 0x00000000,
        MSC_MSAC_W1_10: 0x00000000,
        MSC_MSAC_W0_11: 0x00000000,
        MSC_MSAC_W1_11: 0x00000000,
        MSC_MSAC_W0_12: 0x00000000,
        MSC_MSAC_W1_12: 0x00000000,
        MSC_MSAC_W0_13: 0x00000000,
        MSC_MSAC_W1_13: 0x00000000,
        MSC_MSAC_W0_14: 0x00000000,
        MSC_MSAC_W1_14: 0x00000000,
        MSC_MSAC_W0_15: 0x00000000,
        MSC_MSAC_W1_15: 0x00000000,
        MSC_MSAC_W0_16: 0x00000000,
        MSC_MSAC_W1_16: 0x00000000,
        MSC_MSAC_W0_17: 0x00000000,
        MSC_MSAC_W1_17: 0x00000000,
        MSC_MSAC_W0_18: 0x00000000,
        MSC_MSAC_W1_18: 0x00000000,
        MSC_MSAC_W0_19: 0x00000000,
        MSC_MSAC_W1_19: 0x00000000,
        MSC_MSAC_W0_20: 0x00000000,
        MSC_MSAC_W1_20: 0x00000000,
        MSC_MSAC_W0_21: 0x00000000,
        MSC_MSAC_W1_21: 0x00000000,
        MSC_MSAC_W0_22: 0x00000000,
        MSC_MSAC_W1_22: 0x00000000,
        MSC_MSAC_W0_23: 0x00000000,
        MSC_MSAC_W1_23: 0x00000000,
        MSC_MSAC_W0_24: 0x00000000,
        MSC_MSAC_W1_24: 0x00000000,
        MSC_MSAC_W0_25: 0x00000000,
        MSC_MSAC_W1_25: 0x00000000,
        MSC_MSAC_W0_26: 0x00000000,
        MSC_MSAC_W1_26: 0x00000000,
        MSC_MSAC_W0_27: 0x00000000,
        MSC_MSAC_W1_27: 0x00000000,
        MSC_MSAC_W0_28: 0x00000000,
        MSC_MSAC_W1_28: 0x00000000,
        MSC_MSAC_W0_29: 0x00000000,
        MSC_MSAC_W1_29: 0x00000000,
        MSC_MSAC_W0_30: 0x00000000,
        MSC_MSAC_W1_30: 0x00000000,
        MSC_MSAC_W0_31: 0x00000000,
        MSC_MSAC_W1_31: 0x00000000,
        MSC_MSAC_W0_32: 0x00000000,
        MSC_MSAC_W1_32: 0x00000000,
        MSC_MSAC_W0_33: 0x00000000,
        MSC_MSAC_W1_33: 0x00000000,
        MSC_MSAC_W0_34: 0x00000000,
        MSC_MSAC_W1_34: 0x00000000,
        MSC_MSAC_W0_35: 0x00000000,
        MSC_MSAC_W1_35: 0x00000000,
        MSC_MSAC_W0_36: 0x00000000,
        MSC_MSAC_W1_36: 0x00000000,
        MSC_MSAC_W0_37: 0x00000000,
        MSC_MSAC_W1_37: 0x00000000,
        MSC_MSAC_W0_38: 0x00000000,
        MSC_MSAC_W1_38: 0x00000000,
        MSC_MSAC_W0_39: 0x00000000,
        MSC_MSAC_W1_39: 0x00000000,
        MSC_MSAC_W0_40: 0x00000000,
        MSC_MSAC_W1_40: 0x00000000,
        MSC_MSAC_W0_41: 0x00000000,
        MSC_MSAC_W1_41: 0x00000000,
        MSC_MSAC_W0_42: 0x00000000,
        MSC_MSAC_W1_42: 0x00000000,
        MSC_MSAC_W0_43: 0x00000000,
        MSC_MSAC_W1_43: 0x00000000,
        MSC_MSAC_W0_44: 0x00000000,
        MSC_MSAC_W1_44: 0x00000000,
        MSC_MSAC_W0_45: 0x00000000,
        MSC_MSAC_W1_45: 0x00000000,
        MSC_MSAC_W0_46: 0x00000000,
        MSC_MSAC_W1_46: 0x00000000,
        MSC_MSAC_W0_47: 0x00000000,
        MSC_MSAC_W1_47: 0x00000000,
        MSC_MSAC_W0_48: 0x00000000,
        MSC_MSAC_W1_48: 0x00000000,
        MSC_MSAC_W0_49: 0x00000000,
        MSC_MSAC_W1_49: 0x00000000,
        MSC_MSAC_W0_50: 0x00000000,
        MSC_MSAC_W1_50: 0x00000000,
        MSC_MSAC_W0_51: 0x00000000,
        MSC_MSAC_W1_51: 0x00000000,
        MSC_MSAC_W0_52: 0x00000000,
        MSC_MSAC_W1_52: 0x00000000,
        MSC_MSAC_W0_53: 0x00000000,
        MSC_MSAC_W1_53: 0x00000000,
        MSC_MSAC_W0_54: 0x00000000,
        MSC_MSAC_W1_54: 0x00000000,
        MSC_MSAC_W0_55: 0x00000000,
        MSC_MSAC_W1_55: 0x00000000,
        MSC_MSAC_W0_56: 0x00000000,
        MSC_MSAC_W1_56: 0x00000000,
        MSC_MSAC_W0_57: 0x00000000,
        MSC_MSAC_W1_57: 0x00000000,
        MSC_MSAC_W0_58: 0x00000000,
        MSC_MSAC_W1_58: 0x00000000,
        MSC_MSAC_W0_59: 0x00000000,
        MSC_MSAC_W1_59: 0x00000000,
        MSC_MSAC_W0_60: 0x00000000,
        MSC_MSAC_W1_60: 0x00000000,
        MSC_MSAC_W0_61: 0x00000000,
        MSC_MSAC_W1_61: 0x00000000,
        MSC_MSAC_W0_62: 0x00000000,
        MSC_MSAC_W1_62: 0x00000000,
        MSC_MSAC_W0_63: 0x00000000,
        MSC_MSAC_W1_63: 0x00000000,
        MSC_MSAC_W0_64: 0x00000000,
        MSC_MSAC_W1_64: 0x00000000,
        MSC_MSAC_W0_65: 0x00000000,
        MSC_MSAC_W1_65: 0x00000000,
        MSC_MSAC_W0_66: 0x00000000,
        MSC_MSAC_W1_66: 0x00000000,
        MSC_MSAC_W0_67: 0x00000000,
        MSC_MSAC_W1_67: 0x00000000,
        MSC_MSAC_W0_68: 0x00000000,
        MSC_MSAC_W1_68: 0x00000000,
        MSC_MSAC_W0_69: 0x00000000,
        MSC_MSAC_W1_69: 0x00000000,
        MSC_MSAC_W0_70: 0x00000000,
        MSC_MSAC_W1_70: 0x00000000,
        MSC_MSAC_W0_71: 0x00000000,
        MSC_MSAC_W1_71: 0x00000000,
        MSC_MSAC_W0_72: 0x00000000,
        MSC_MSAC_W1_72: 0x00000000,
        MSC_MSAC_W0_73: 0x00000000,
        MSC_MSAC_W1_73: 0x00000000,
        MSC_MSAC_W0_74: 0x00000000,
        MSC_MSAC_W1_74: 0x00000000,
        MSC_MSAC_W0_75: 0x00000000,
        MSC_MSAC_W1_75: 0x00000000,
        MSC_MSAC_W0_76: 0x00000000,
        MSC_MSAC_W1_76: 0x00000000,
        MSC_MSAC_W0_77: 0x00000000,
        MSC_MSAC_W1_77: 0x00000000,
        MSC_MSAC_W0_78: 0x00000000,
        MSC_MSAC_W1_78: 0x00000000,
        MSC_MSAC_W0_79: 0x00000000,
        MSC_MSAC_W1_79: 0x00000000,
        MSC_MSAC_W0_80: 0x00000000,
        MSC_MSAC_W1_80: 0x00000000,
        MSC_MSAC_W0_81: 0x00000000,
        MSC_MSAC_W1_81: 0x00000000,
        MSC_MSAC_W0_82: 0x00000000,
        MSC_MSAC_W1_82: 0x00000000,
        MSC_MSAC_W0_83: 0x00000000,
        MSC_MSAC_W1_83: 0x00000000,
        MSC_MSAC_W0_84: 0x00000000,
        MSC_MSAC_W1_84: 0x00000000,
        MSC_MSAC_W0_85: 0x00000000,
        MSC_MSAC_W1_85: 0x00000000,
        MSC_MSAC_W0_86: 0x00000000,
        MSC_MSAC_W1_86: 0x00000000,
        MSC_MSAC_W0_87: 0x00000000,
        MSC_MSAC_W1_87: 0x00000000,
        MSC_MSAC_W0_88: 0x00000000,
        MSC_MSAC_W1_88: 0x00000000,
        MSC_MSAC_W0_89: 0x00000000,
        MSC_MSAC_W1_89: 0x00000000,
        MSC_MSAC_W0_90: 0x00000000,
        MSC_MSAC_W1_90: 0x00000000,
        MSC_MSAC_W0_91: 0x00000000,
        MSC_MSAC_W1_91: 0x00000000,
        MSC_MSAC_W0_92: 0x00000000,
        MSC_MSAC_W1_92: 0x00000000,
        MSC_MSAC_W0_93: 0x00000000,
        MSC_MSAC_W1_93: 0x00000000,
        MSC_MSAC_W0_94: 0x00000000,
        MSC_MSAC_W1_94: 0x00000000,
        MSC_MSAC_W0_95: 0x00000000,
        MSC_MSAC_W1_95: 0x00000000,
        MSC_MSAC_W0_96: 0x00000000,
        MSC_MSAC_W1_96: 0x00000000,
        MSC_MSAC_W0_97: 0x00000000,
        MSC_MSAC_W1_97: 0x00000000,
        MSC_MSAC_W0_98: 0x00000000,
        MSC_MSAC_W1_98: 0x00000000,
        MSC_MSAC_W0_99: 0x00000000,
        MSC_MSAC_W1_99: 0x00000000,
        MSC_MSAC_W0_100: 0x00000000,
        MSC_MSAC_W1_100: 0x00000000,
        MSC_MSAC_W0_101: 0x00000000,
        MSC_MSAC_W1_101: 0x00000000,
        MSC_MSAC_W0_102: 0x00000000,
        MSC_MSAC_W1_102: 0x00000000,
        MSC_MSAC_W0_103: 0x00000000,
        MSC_MSAC_W1_103: 0x00000000,
        MSC_MSAC_W0_104: 0x00000000,
        MSC_MSAC_W1_104: 0x00000000,
        MSC_MSAC_W0_105: 0x00000000,
        MSC_MSAC_W1_105: 0x00000000,
        MSC_MSAC_W0_106: 0x00000000,
        MSC_MSAC_W1_106: 0x00000000,
        MSC_MSAC_W0_107: 0x00000000,
        MSC_MSAC_W1_107: 0x00000000,
        MSC_MSAC_W0_108: 0x00000000,
        MSC_MSAC_W1_108: 0x00000000,
        MSC_MSAC_W0_109: 0x00000000,
        MSC_MSAC_W1_109: 0x00000000,
        MSC_MSAC_W0_110: 0x00000000,
        MSC_MSAC_W1_110: 0x00000000,
        MSC_MSAC_W0_111: 0x00000000,
        MSC_MSAC_W1_111: 0x00000000,
        MSC_MSAC_W0_112: 0x00000000,
        MSC_MSAC_W1_112: 0x00000000,
        MSC_MSAC_W0_113: 0x00000000,
        MSC_MSAC_W1_113: 0x00000000,
        MSC_MSAC_W0_114: 0x00000000,
        MSC_MSAC_W1_114: 0x00000000,
        MSC_MSAC_W0_115: 0x00000000,
        MSC_MSAC_W1_115: 0x00000000,
        MSC_MSAC_W0_116: 0x00000000,
        MSC_MSAC_W1_116: 0x00000000,
        MSC_MSAC_W0_117: 0x00000000,
        MSC_MSAC_W1_117: 0x00000000,
        MSC_MSAC_W0_118: 0x00000000,
        MSC_MSAC_W1_118: 0x00000000,
        MSC_MSAC_W0_119: 0x00000000,
        MSC_MSAC_W1_119: 0x00000000,
        MSC_MSAC_W0_120: 0x00000000,
        MSC_MSAC_W1_120: 0x00000000,
        MSC_MSAC_W0_121: 0x00000000,
        MSC_MSAC_W1_121: 0x00000000,
        MSC_MSAC_W0_122: 0x00000000,
        MSC_MSAC_W1_122: 0x00000000,
        MSC_MSAC_W0_123: 0x00000000,
        MSC_MSAC_W1_123: 0x00000000,
        MSC_MSAC_W0_124: 0x00000000,
        MSC_MSAC_W1_124: 0x00000000,
        MSC_MSAC_W0_125: 0x00000000,
        MSC_MSAC_W1_125: 0x00000000,
        MSC_MSAC_W0_126: 0x00000000,
        MSC_MSAC_W1_126: 0x00000000,
        MSC_MSAC_W0_127: 0x00000000,
        MSC_MSAC_W1_127: 0x00000000,
    };

    /// Safe access to XRDC2_D1
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
        let taken = XRDC2_D1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to XRDC2_D1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = XRDC2_D1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal XRDC2_D1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        XRDC2_D1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl XRDC2_D1 {
    /// The interrupts associated with XRDC2_D1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with XRDC2_D1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to XRDC2_D1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XRDC2_D1: *const RegisterBlock = 0x40cd0000 as *const _;
