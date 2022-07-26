#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CSI
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::csi::Instance;
pub use crate::imxrt117::peripherals::csi::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::csi::{
    CR1, CR100, CR101, CR102, CR103, CR104, CR105, CR106, CR107, CR108, CR109, CR110, CR111, CR112,
    CR113, CR114, CR115, CR116, CR117, CR118, CR119, CR120, CR121, CR122, CR123, CR124, CR125,
    CR126, CR127, CR128, CR129, CR130, CR131, CR132, CR133, CR134, CR135, CR136, CR137, CR138,
    CR139, CR140, CR141, CR142, CR143, CR144, CR145, CR146, CR147, CR148, CR149, CR150, CR151,
    CR152, CR153, CR154, CR155, CR156, CR157, CR158, CR159, CR160, CR161, CR162, CR163, CR164,
    CR165, CR166, CR167, CR168, CR169, CR170, CR171, CR172, CR173, CR174, CR175, CR176, CR177,
    CR178, CR179, CR18, CR180, CR181, CR182, CR183, CR184, CR185, CR186, CR187, CR188, CR189, CR19,
    CR190, CR191, CR192, CR193, CR194, CR195, CR196, CR197, CR198, CR199, CR2, CR20, CR200, CR201,
    CR202, CR203, CR204, CR205, CR206, CR207, CR208, CR209, CR21, CR210, CR211, CR212, CR213,
    CR214, CR215, CR216, CR217, CR218, CR219, CR22, CR220, CR221, CR222, CR223, CR224, CR225,
    CR226, CR227, CR228, CR229, CR23, CR230, CR231, CR232, CR233, CR234, CR235, CR236, CR237,
    CR238, CR239, CR24, CR240, CR241, CR242, CR243, CR244, CR245, CR246, CR247, CR248, CR249, CR25,
    CR250, CR251, CR252, CR253, CR254, CR255, CR256, CR257, CR258, CR259, CR26, CR260, CR261,
    CR262, CR263, CR264, CR265, CR266, CR267, CR268, CR269, CR27, CR270, CR271, CR272, CR273,
    CR274, CR275, CR276, CR28, CR29, CR3, CR30, CR31, CR32, CR33, CR34, CR35, CR36, CR37, CR38,
    CR39, CR40, CR41, CR42, CR43, CR44, CR45, CR46, CR47, CR48, CR49, CR50, CR51, CR52, CR53, CR54,
    CR55, CR56, CR57, CR58, CR59, CR60, CR61, CR62, CR63, CR64, CR65, CR66, CR67, CR68, CR69, CR70,
    CR71, CR72, CR73, CR74, CR75, CR76, CR77, CR78, CR79, CR80, CR81, CR82, CR83, CR84, CR85, CR86,
    CR87, CR88, CR89, CR90, CR91, CR92, CR93, CR94, CR95, CR96, CR97, CR98, CR99, DMASA_FB1,
    DMASA_FB2, DMASA_STATFIFO, DMATS_STATFIFO, FBUF_PARA, IMAG_PARA, RFIFO, RXCNT, SR, STATFIFO,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CSI peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CSI = Instance<0>;

/// The CSI peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CSI = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct CSI {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CSI {}
impl crate::Valid for CSI {
    fn take() -> Option<Self> {
        <CSI>::take()
    }
    fn release(self) {
        <CSI>::release(self);
    }
    unsafe fn steal() -> Self {
        <CSI>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CSI_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CSI peripheral instance
#[cfg(not(feature = "nosync"))]
impl CSI {
    const INSTANCE: Self = Self {
        addr: 0x40800000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CSI],
    };

    /// Reset values for each field in CSI
    pub const reset: ResetValues = ResetValues {
        CR1: 0x40000800,
        CR2: 0x00000000,
        CR3: 0x00000000,
        STATFIFO: 0x00000000,
        RFIFO: 0x00000000,
        RXCNT: 0x00009600,
        SR: 0x80004000,
        DMASA_STATFIFO: 0x00000000,
        DMATS_STATFIFO: 0x00000000,
        DMASA_FB1: 0x00000000,
        DMASA_FB2: 0x00000000,
        FBUF_PARA: 0x00000000,
        IMAG_PARA: 0x00000000,
        CR18: 0x0002D000,
        CR19: 0x00000000,
        CR20: 0x00000000,
        CR21: 0x00000000,
        CR22: 0x00000000,
        CR23: 0x00000000,
        CR24: 0x00000000,
        CR25: 0x00000000,
        CR26: 0x00000000,
        CR27: 0x00000000,
        CR28: 0x00000000,
        CR29: 0x00000000,
        CR30: 0x00000000,
        CR31: 0x00000000,
        CR32: 0x00000000,
        CR33: 0x00000000,
        CR34: 0x00000000,
        CR35: 0x00000000,
        CR36: 0x00000000,
        CR37: 0x00000000,
        CR38: 0x00000000,
        CR39: 0x00000000,
        CR40: 0x00000000,
        CR41: 0x00000000,
        CR42: 0x00000000,
        CR43: 0x00000000,
        CR44: 0x00000000,
        CR45: 0x00000000,
        CR46: 0x00000000,
        CR47: 0x00000000,
        CR48: 0x00000000,
        CR49: 0x00000000,
        CR50: 0x00000000,
        CR51: 0x00000000,
        CR52: 0x00000000,
        CR53: 0x00000000,
        CR54: 0x00000000,
        CR55: 0x00000000,
        CR56: 0x00000000,
        CR57: 0x00000000,
        CR58: 0x00000000,
        CR59: 0x00000000,
        CR60: 0x00000000,
        CR61: 0x00000000,
        CR62: 0x00000000,
        CR63: 0x00000000,
        CR64: 0x00000000,
        CR65: 0x00000000,
        CR66: 0x00000000,
        CR67: 0x00000000,
        CR68: 0x00000000,
        CR69: 0x00000000,
        CR70: 0x00000000,
        CR71: 0x00000000,
        CR72: 0x00000000,
        CR73: 0x00000000,
        CR74: 0x00000000,
        CR75: 0x00000000,
        CR76: 0x00000000,
        CR77: 0x00000000,
        CR78: 0x00000000,
        CR79: 0x00000000,
        CR80: 0x00000000,
        CR81: 0x00000000,
        CR82: 0x00000000,
        CR83: 0x00000000,
        CR84: 0x00000000,
        CR85: 0x00000000,
        CR86: 0x00000000,
        CR87: 0x00000000,
        CR88: 0x00000000,
        CR89: 0x00000000,
        CR90: 0x00000000,
        CR91: 0x00000000,
        CR92: 0x00000000,
        CR93: 0x00000000,
        CR94: 0x00000000,
        CR95: 0x00000000,
        CR96: 0x00000000,
        CR97: 0x00000000,
        CR98: 0x00000000,
        CR99: 0x00000000,
        CR100: 0x00000000,
        CR101: 0x00000000,
        CR102: 0x00000000,
        CR103: 0x00000000,
        CR104: 0x00000000,
        CR105: 0x00000000,
        CR106: 0x00000000,
        CR107: 0x00000000,
        CR108: 0x00000000,
        CR109: 0x00000000,
        CR110: 0x00000000,
        CR111: 0x00000000,
        CR112: 0x00000000,
        CR113: 0x00000000,
        CR114: 0x00000000,
        CR115: 0x00000000,
        CR116: 0x00000000,
        CR117: 0x00000000,
        CR118: 0x00000000,
        CR119: 0x00000000,
        CR120: 0x00000000,
        CR121: 0x00000000,
        CR122: 0x00000000,
        CR123: 0x00000000,
        CR124: 0x00000000,
        CR125: 0x00000000,
        CR126: 0x00000000,
        CR127: 0x00000000,
        CR128: 0x00000000,
        CR129: 0x00000000,
        CR130: 0x00000000,
        CR131: 0x00000000,
        CR132: 0x00000000,
        CR133: 0x00000000,
        CR134: 0x00000000,
        CR135: 0x00000000,
        CR136: 0x00000000,
        CR137: 0x00000000,
        CR138: 0x00000000,
        CR139: 0x00000000,
        CR140: 0x00000000,
        CR141: 0x00000000,
        CR142: 0x00000000,
        CR143: 0x00000000,
        CR144: 0x00000000,
        CR145: 0x00000000,
        CR146: 0x00000000,
        CR147: 0x00000000,
        CR148: 0x00000000,
        CR149: 0x00000000,
        CR150: 0x00000000,
        CR151: 0x00000000,
        CR152: 0x00000000,
        CR153: 0x00000000,
        CR154: 0x00000000,
        CR155: 0x00000000,
        CR156: 0x00000000,
        CR157: 0x00000000,
        CR158: 0x00000000,
        CR159: 0x00000000,
        CR160: 0x00000000,
        CR161: 0x00000000,
        CR162: 0x00000000,
        CR163: 0x00000000,
        CR164: 0x00000000,
        CR165: 0x00000000,
        CR166: 0x00000000,
        CR167: 0x00000000,
        CR168: 0x00000000,
        CR169: 0x00000000,
        CR170: 0x00000000,
        CR171: 0x00000000,
        CR172: 0x00000000,
        CR173: 0x00000000,
        CR174: 0x00000000,
        CR175: 0x00000000,
        CR176: 0x00000000,
        CR177: 0x00000000,
        CR178: 0x00000000,
        CR179: 0x00000000,
        CR180: 0x00000000,
        CR181: 0x00000000,
        CR182: 0x00000000,
        CR183: 0x00000000,
        CR184: 0x00000000,
        CR185: 0x00000000,
        CR186: 0x00000000,
        CR187: 0x00000000,
        CR188: 0x00000000,
        CR189: 0x00000000,
        CR190: 0x00000000,
        CR191: 0x00000000,
        CR192: 0x00000000,
        CR193: 0x00000000,
        CR194: 0x00000000,
        CR195: 0x00000000,
        CR196: 0x00000000,
        CR197: 0x00000000,
        CR198: 0x00000000,
        CR199: 0x00000000,
        CR200: 0x00000000,
        CR201: 0x00000000,
        CR202: 0x00000000,
        CR203: 0x00000000,
        CR204: 0x00000000,
        CR205: 0x00000000,
        CR206: 0x00000000,
        CR207: 0x00000000,
        CR208: 0x00000000,
        CR209: 0x00000000,
        CR210: 0x00000000,
        CR211: 0x00000000,
        CR212: 0x00000000,
        CR213: 0x00000000,
        CR214: 0x00000000,
        CR215: 0x00000000,
        CR216: 0x00000000,
        CR217: 0x00000000,
        CR218: 0x00000000,
        CR219: 0x00000000,
        CR220: 0x00000000,
        CR221: 0x00000000,
        CR222: 0x00000000,
        CR223: 0x00000000,
        CR224: 0x00000000,
        CR225: 0x00000000,
        CR226: 0x00000000,
        CR227: 0x00000000,
        CR228: 0x00000000,
        CR229: 0x00000000,
        CR230: 0x00000000,
        CR231: 0x00000000,
        CR232: 0x00000000,
        CR233: 0x00000000,
        CR234: 0x00000000,
        CR235: 0x00000000,
        CR236: 0x00000000,
        CR237: 0x00000000,
        CR238: 0x00000000,
        CR239: 0x00000000,
        CR240: 0x00000000,
        CR241: 0x00000000,
        CR242: 0x00000000,
        CR243: 0x00000000,
        CR244: 0x00000000,
        CR245: 0x00000000,
        CR246: 0x00000000,
        CR247: 0x00000000,
        CR248: 0x00000000,
        CR249: 0x00000000,
        CR250: 0x00000000,
        CR251: 0x00000000,
        CR252: 0x00000000,
        CR253: 0x00000000,
        CR254: 0x00000000,
        CR255: 0x00000000,
        CR256: 0x00000000,
        CR257: 0x00000000,
        CR258: 0x00000000,
        CR259: 0x00000000,
        CR260: 0x00000000,
        CR261: 0x00000000,
        CR262: 0x00000000,
        CR263: 0x00000000,
        CR264: 0x00000000,
        CR265: 0x00000000,
        CR266: 0x00000000,
        CR267: 0x00000000,
        CR268: 0x00000000,
        CR269: 0x00000000,
        CR270: 0x00000000,
        CR271: 0x00000000,
        CR272: 0x00000000,
        CR273: 0x00000000,
        CR274: 0x00000000,
        CR275: 0x00000000,
        CR276: 0x00000000,
    };

    /// Safe access to CSI
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
        let taken = CSI_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CSI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CSI_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CSI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CSI_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CSI {
    /// The interrupts associated with CSI
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::CSI];

    /// The interrupts associated with CSI
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CSI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CSI: *const RegisterBlock = 0x40800000 as *const _;
