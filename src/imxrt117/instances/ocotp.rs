#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::ocotp::Instance;
pub use crate::imxrt117::peripherals::ocotp::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::ocotp::{
    BIT_LOCK, CTRL, CTRL_CLR, CTRL_SET, CTRL_TOG, DATA, FUSE_0, FUSE_1, FUSE_10, FUSE_100,
    FUSE_101, FUSE_102, FUSE_103, FUSE_104, FUSE_105, FUSE_106, FUSE_107, FUSE_108, FUSE_109,
    FUSE_11, FUSE_110, FUSE_111, FUSE_112, FUSE_113, FUSE_114, FUSE_115, FUSE_116, FUSE_117,
    FUSE_118, FUSE_119, FUSE_12, FUSE_120, FUSE_121, FUSE_122, FUSE_123, FUSE_124, FUSE_125,
    FUSE_126, FUSE_127, FUSE_128, FUSE_129, FUSE_13, FUSE_130, FUSE_131, FUSE_132, FUSE_133,
    FUSE_134, FUSE_135, FUSE_136, FUSE_137, FUSE_138, FUSE_139, FUSE_14, FUSE_140, FUSE_141,
    FUSE_142, FUSE_143, FUSE_15, FUSE_16, FUSE_17, FUSE_18, FUSE_19, FUSE_2, FUSE_20, FUSE_21,
    FUSE_22, FUSE_23, FUSE_24, FUSE_25, FUSE_26, FUSE_27, FUSE_28, FUSE_29, FUSE_3, FUSE_30,
    FUSE_31, FUSE_32, FUSE_33, FUSE_34, FUSE_35, FUSE_36, FUSE_37, FUSE_38, FUSE_39, FUSE_4,
    FUSE_40, FUSE_41, FUSE_42, FUSE_43, FUSE_44, FUSE_45, FUSE_46, FUSE_47, FUSE_48, FUSE_49,
    FUSE_5, FUSE_50, FUSE_51, FUSE_52, FUSE_53, FUSE_54, FUSE_55, FUSE_56, FUSE_57, FUSE_58,
    FUSE_59, FUSE_6, FUSE_60, FUSE_61, FUSE_62, FUSE_63, FUSE_64, FUSE_65, FUSE_66, FUSE_67,
    FUSE_68, FUSE_69, FUSE_7, FUSE_70, FUSE_71, FUSE_72, FUSE_73, FUSE_74, FUSE_75, FUSE_76,
    FUSE_77, FUSE_78, FUSE_79, FUSE_8, FUSE_80, FUSE_81, FUSE_82, FUSE_83, FUSE_84, FUSE_85,
    FUSE_86, FUSE_87, FUSE_88, FUSE_89, FUSE_9, FUSE_90, FUSE_91, FUSE_92, FUSE_93, FUSE_94,
    FUSE_95, FUSE_96, FUSE_97, FUSE_98, FUSE_99, LOCKED0, LOCKED1, LOCKED2, LOCKED3, LOCKED4,
    OUT_STATUS, OUT_STATUS_CLR, OUT_STATUS_SET, OUT_STATUS_TOG, PDN, READ_CTRL, READ_FUSE_DATA0,
    READ_FUSE_DATA1, READ_FUSE_DATA2, READ_FUSE_DATA3, SW_LOCK, VERSION,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The OCOTP peripheral instance.
#[cfg(not(feature = "doc"))]
pub type OCOTP = Instance<0>;

/// The OCOTP peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type OCOTP = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct OCOTP {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for OCOTP {}
impl crate::Valid for OCOTP {
    fn take() -> Option<Self> {
        <OCOTP>::take()
    }
    fn release(self) {
        <OCOTP>::release(self);
    }
    unsafe fn steal() -> Self {
        <OCOTP>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static OCOTP_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the OCOTP peripheral instance
#[cfg(not(feature = "nosync"))]
impl OCOTP {
    const INSTANCE: Self = Self {
        addr: 0x40cac000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::OCOTP_READ_FUSE_ERROR,
            crate::interrupt::OCOTP_READ_DONE_ERROR,
        ],
    };

    /// Reset values for each field in OCOTP
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        CTRL_SET: 0x00000000,
        CTRL_CLR: 0x00000000,
        CTRL_TOG: 0x00000000,
        PDN: 0x00000000,
        DATA: 0x00000000,
        READ_CTRL: 0x00000000,
        OUT_STATUS: 0x00000000,
        OUT_STATUS_SET: 0x00000000,
        OUT_STATUS_CLR: 0x00000000,
        OUT_STATUS_TOG: 0x00000000,
        VERSION: 0x0A000000,
        READ_FUSE_DATA0: 0x00000000,
        READ_FUSE_DATA1: 0x00000000,
        READ_FUSE_DATA2: 0x00000000,
        READ_FUSE_DATA3: 0x00000000,
        SW_LOCK: 0x00000000,
        BIT_LOCK: 0x00000000,
        LOCKED0: 0x00000000,
        LOCKED1: 0x00000000,
        LOCKED2: 0x00000000,
        LOCKED3: 0x00000000,
        LOCKED4: 0x00000000,
        FUSE_0: 0x00000000,
        FUSE_1: 0x00000000,
        FUSE_2: 0x00000000,
        FUSE_3: 0x00000000,
        FUSE_4: 0x00000000,
        FUSE_5: 0x00000000,
        FUSE_6: 0x00000000,
        FUSE_7: 0x00000000,
        FUSE_8: 0x00000000,
        FUSE_9: 0x00000000,
        FUSE_10: 0x00000000,
        FUSE_11: 0x00000000,
        FUSE_12: 0x00000000,
        FUSE_13: 0x00000000,
        FUSE_14: 0x00000000,
        FUSE_15: 0x00000000,
        FUSE_16: 0x00000000,
        FUSE_17: 0x00000000,
        FUSE_18: 0x00000000,
        FUSE_19: 0x00000000,
        FUSE_20: 0x00000000,
        FUSE_21: 0x00000000,
        FUSE_22: 0x00000000,
        FUSE_23: 0x00000000,
        FUSE_24: 0x00000000,
        FUSE_25: 0x00000000,
        FUSE_26: 0x00000000,
        FUSE_27: 0x00000000,
        FUSE_28: 0x00000000,
        FUSE_29: 0x00000000,
        FUSE_30: 0x00000000,
        FUSE_31: 0x00000000,
        FUSE_32: 0x00000000,
        FUSE_33: 0x00000000,
        FUSE_34: 0x00000000,
        FUSE_35: 0x00000000,
        FUSE_36: 0x00000000,
        FUSE_37: 0x00000000,
        FUSE_38: 0x00000000,
        FUSE_39: 0x00000000,
        FUSE_40: 0x00000000,
        FUSE_41: 0x00000000,
        FUSE_42: 0x00000000,
        FUSE_43: 0x00000000,
        FUSE_44: 0x00000000,
        FUSE_45: 0x00000000,
        FUSE_46: 0x00000000,
        FUSE_47: 0x00000000,
        FUSE_48: 0x00000000,
        FUSE_49: 0x00000000,
        FUSE_50: 0x00000000,
        FUSE_51: 0x00000000,
        FUSE_52: 0x00000000,
        FUSE_53: 0x00000000,
        FUSE_54: 0x00000000,
        FUSE_55: 0x00000000,
        FUSE_56: 0x00000000,
        FUSE_57: 0x00000000,
        FUSE_58: 0x00000000,
        FUSE_59: 0x00000000,
        FUSE_60: 0x00000000,
        FUSE_61: 0x00000000,
        FUSE_62: 0x00000000,
        FUSE_63: 0x00000000,
        FUSE_64: 0x00000000,
        FUSE_65: 0x00000000,
        FUSE_66: 0x00000000,
        FUSE_67: 0x00000000,
        FUSE_68: 0x00000000,
        FUSE_69: 0x00000000,
        FUSE_70: 0x00000000,
        FUSE_71: 0x00000000,
        FUSE_72: 0x00000000,
        FUSE_73: 0x00000000,
        FUSE_74: 0x00000000,
        FUSE_75: 0x00000000,
        FUSE_76: 0x00000000,
        FUSE_77: 0x00000000,
        FUSE_78: 0x00000000,
        FUSE_79: 0x00000000,
        FUSE_80: 0x00000000,
        FUSE_81: 0x00000000,
        FUSE_82: 0x00000000,
        FUSE_83: 0x00000000,
        FUSE_84: 0x00000000,
        FUSE_85: 0x00000000,
        FUSE_86: 0x00000000,
        FUSE_87: 0x00000000,
        FUSE_88: 0x00000000,
        FUSE_89: 0x00000000,
        FUSE_90: 0x00000000,
        FUSE_91: 0x00000000,
        FUSE_92: 0x00000000,
        FUSE_93: 0x00000000,
        FUSE_94: 0x00000000,
        FUSE_95: 0x00000000,
        FUSE_96: 0x00000000,
        FUSE_97: 0x00000000,
        FUSE_98: 0x00000000,
        FUSE_99: 0x00000000,
        FUSE_100: 0x00000000,
        FUSE_101: 0x00000000,
        FUSE_102: 0x00000000,
        FUSE_103: 0x00000000,
        FUSE_104: 0x00000000,
        FUSE_105: 0x00000000,
        FUSE_106: 0x00000000,
        FUSE_107: 0x00000000,
        FUSE_108: 0x00000000,
        FUSE_109: 0x00000000,
        FUSE_110: 0x00000000,
        FUSE_111: 0x00000000,
        FUSE_112: 0x00000000,
        FUSE_113: 0x00000000,
        FUSE_114: 0x00000000,
        FUSE_115: 0x00000000,
        FUSE_116: 0x00000000,
        FUSE_117: 0x00000000,
        FUSE_118: 0x00000000,
        FUSE_119: 0x00000000,
        FUSE_120: 0x00000000,
        FUSE_121: 0x00000000,
        FUSE_122: 0x00000000,
        FUSE_123: 0x00000000,
        FUSE_124: 0x00000000,
        FUSE_125: 0x00000000,
        FUSE_126: 0x00000000,
        FUSE_127: 0x00000000,
        FUSE_128: 0x00000000,
        FUSE_129: 0x00000000,
        FUSE_130: 0x00000000,
        FUSE_131: 0x00000000,
        FUSE_132: 0x00000000,
        FUSE_133: 0x00000000,
        FUSE_134: 0x00000000,
        FUSE_135: 0x00000000,
        FUSE_136: 0x00000000,
        FUSE_137: 0x00000000,
        FUSE_138: 0x00000000,
        FUSE_139: 0x00000000,
        FUSE_140: 0x00000000,
        FUSE_141: 0x00000000,
        FUSE_142: 0x00000000,
        FUSE_143: 0x00000000,
    };

    /// Safe access to OCOTP
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
        let taken = OCOTP_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to OCOTP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = OCOTP_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal OCOTP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        OCOTP_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl OCOTP {
    /// The interrupts associated with OCOTP
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::OCOTP_READ_FUSE_ERROR,
        crate::interrupt::OCOTP_READ_DONE_ERROR,
    ];

    /// The interrupts associated with OCOTP
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to OCOTP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OCOTP: *const RegisterBlock = 0x40cac000 as *const _;
