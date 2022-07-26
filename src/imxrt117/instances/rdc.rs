#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! RDC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::rdc::Instance;
pub use crate::imxrt117::peripherals::rdc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::rdc::{
    INTCTRL, INTSTAT, MDA0, MDA1, MDA10, MDA11, MDA2, MDA3, MDA4, MDA5, MDA6, MDA7, MDA8, MDA9,
    MRC0, MRC1, MRC10, MRC11, MRC12, MRC13, MRC14, MRC15, MRC16, MRC17, MRC18, MRC19, MRC2, MRC20,
    MRC21, MRC22, MRC23, MRC24, MRC25, MRC26, MRC27, MRC28, MRC29, MRC3, MRC30, MRC31, MRC32,
    MRC33, MRC34, MRC35, MRC36, MRC37, MRC38, MRC39, MRC4, MRC40, MRC41, MRC42, MRC43, MRC44,
    MRC45, MRC46, MRC47, MRC48, MRC49, MRC5, MRC50, MRC51, MRC52, MRC53, MRC54, MRC55, MRC56,
    MRC57, MRC58, MRC6, MRC7, MRC8, MRC9, MREA0, MREA1, MREA10, MREA11, MREA12, MREA13, MREA14,
    MREA15, MREA16, MREA17, MREA18, MREA19, MREA2, MREA20, MREA21, MREA22, MREA23, MREA24, MREA25,
    MREA26, MREA27, MREA28, MREA29, MREA3, MREA30, MREA31, MREA32, MREA33, MREA34, MREA35, MREA36,
    MREA37, MREA38, MREA39, MREA4, MREA40, MREA41, MREA42, MREA43, MREA44, MREA45, MREA46, MREA47,
    MREA48, MREA49, MREA5, MREA50, MREA51, MREA52, MREA53, MREA54, MREA55, MREA56, MREA57, MREA58,
    MREA6, MREA7, MREA8, MREA9, MRSA0, MRSA1, MRSA10, MRSA11, MRSA12, MRSA13, MRSA14, MRSA15,
    MRSA16, MRSA17, MRSA18, MRSA19, MRSA2, MRSA20, MRSA21, MRSA22, MRSA23, MRSA24, MRSA25, MRSA26,
    MRSA27, MRSA28, MRSA29, MRSA3, MRSA30, MRSA31, MRSA32, MRSA33, MRSA34, MRSA35, MRSA36, MRSA37,
    MRSA38, MRSA39, MRSA4, MRSA40, MRSA41, MRSA42, MRSA43, MRSA44, MRSA45, MRSA46, MRSA47, MRSA48,
    MRSA49, MRSA5, MRSA50, MRSA51, MRSA52, MRSA53, MRSA54, MRSA55, MRSA56, MRSA57, MRSA58, MRSA6,
    MRSA7, MRSA8, MRSA9, MRVS0, MRVS1, MRVS10, MRVS11, MRVS12, MRVS13, MRVS14, MRVS15, MRVS16,
    MRVS17, MRVS18, MRVS19, MRVS2, MRVS20, MRVS21, MRVS22, MRVS23, MRVS24, MRVS25, MRVS26, MRVS27,
    MRVS28, MRVS29, MRVS3, MRVS30, MRVS31, MRVS32, MRVS33, MRVS34, MRVS35, MRVS36, MRVS37, MRVS38,
    MRVS39, MRVS4, MRVS40, MRVS41, MRVS42, MRVS43, MRVS44, MRVS45, MRVS46, MRVS47, MRVS48, MRVS49,
    MRVS5, MRVS50, MRVS51, MRVS52, MRVS53, MRVS54, MRVS55, MRVS56, MRVS57, MRVS58, MRVS6, MRVS7,
    MRVS8, MRVS9, PDAP0, PDAP1, PDAP10, PDAP100, PDAP101, PDAP102, PDAP103, PDAP104, PDAP105,
    PDAP106, PDAP107, PDAP108, PDAP109, PDAP11, PDAP110, PDAP111, PDAP112, PDAP113, PDAP114,
    PDAP115, PDAP116, PDAP117, PDAP118, PDAP119, PDAP12, PDAP120, PDAP121, PDAP122, PDAP123,
    PDAP124, PDAP125, PDAP126, PDAP127, PDAP13, PDAP14, PDAP15, PDAP16, PDAP17, PDAP18, PDAP19,
    PDAP2, PDAP20, PDAP21, PDAP22, PDAP23, PDAP24, PDAP25, PDAP26, PDAP27, PDAP28, PDAP29, PDAP3,
    PDAP30, PDAP31, PDAP32, PDAP33, PDAP34, PDAP35, PDAP36, PDAP37, PDAP38, PDAP39, PDAP4, PDAP40,
    PDAP41, PDAP42, PDAP43, PDAP44, PDAP45, PDAP46, PDAP47, PDAP48, PDAP49, PDAP5, PDAP50, PDAP51,
    PDAP52, PDAP53, PDAP54, PDAP55, PDAP56, PDAP57, PDAP58, PDAP59, PDAP6, PDAP60, PDAP61, PDAP62,
    PDAP63, PDAP64, PDAP65, PDAP66, PDAP67, PDAP68, PDAP69, PDAP7, PDAP70, PDAP71, PDAP72, PDAP73,
    PDAP74, PDAP75, PDAP76, PDAP77, PDAP78, PDAP79, PDAP8, PDAP80, PDAP81, PDAP82, PDAP83, PDAP84,
    PDAP85, PDAP86, PDAP87, PDAP88, PDAP89, PDAP9, PDAP90, PDAP91, PDAP92, PDAP93, PDAP94, PDAP95,
    PDAP96, PDAP97, PDAP98, PDAP99, STAT, VIR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The RDC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type RDC = Instance<0>;

/// The RDC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type RDC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct RDC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for RDC {}
impl crate::Valid for RDC {
    fn take() -> Option<Self> {
        <RDC>::take()
    }
    fn release(self) {
        <RDC>::release(self);
    }
    unsafe fn steal() -> Self {
        <RDC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static RDC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the RDC peripheral instance
#[cfg(not(feature = "nosync"))]
impl RDC {
    const INSTANCE: Self = Self {
        addr: 0x40c78000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::RDC],
    };

    /// Reset values for each field in RDC
    pub const reset: ResetValues = ResetValues {
        VIR: 0x03B800C2,
        STAT: 0x00000100,
        INTCTRL: 0x00000000,
        INTSTAT: 0x00000000,
        MDA0: 0x00000000,
        MDA1: 0x00000000,
        MDA2: 0x00000000,
        MDA3: 0x00000000,
        MDA4: 0x00000000,
        MDA5: 0x00000000,
        MDA6: 0x00000000,
        MDA7: 0x00000000,
        MDA8: 0x00000000,
        MDA9: 0x00000000,
        MDA10: 0x00000000,
        MDA11: 0x00000000,
        PDAP0: 0x0000000F,
        PDAP1: 0x0000000F,
        PDAP2: 0x0000000F,
        PDAP3: 0x0000000F,
        PDAP4: 0x0000000F,
        PDAP5: 0x0000000F,
        PDAP6: 0x0000000F,
        PDAP7: 0x0000000F,
        PDAP8: 0x0000000F,
        PDAP9: 0x0000000F,
        PDAP10: 0x0000000F,
        PDAP11: 0x0000000F,
        PDAP12: 0x0000000F,
        PDAP13: 0x0000000F,
        PDAP14: 0x0000000F,
        PDAP15: 0x0000000F,
        PDAP16: 0x0000000F,
        PDAP17: 0x0000000F,
        PDAP18: 0x0000000F,
        PDAP19: 0x0000000F,
        PDAP20: 0x0000000F,
        PDAP21: 0x0000000F,
        PDAP22: 0x0000000F,
        PDAP23: 0x0000000F,
        PDAP24: 0x0000000F,
        PDAP25: 0x0000000F,
        PDAP26: 0x0000000F,
        PDAP27: 0x0000000F,
        PDAP28: 0x0000000F,
        PDAP29: 0x0000000F,
        PDAP30: 0x0000000F,
        PDAP31: 0x0000000F,
        PDAP32: 0x0000000F,
        PDAP33: 0x0000000F,
        PDAP34: 0x0000000F,
        PDAP35: 0x0000000F,
        PDAP36: 0x0000000F,
        PDAP37: 0x0000000F,
        PDAP38: 0x0000000F,
        PDAP39: 0x0000000F,
        PDAP40: 0x0000000F,
        PDAP41: 0x0000000F,
        PDAP42: 0x0000000F,
        PDAP43: 0x0000000F,
        PDAP44: 0x0000000F,
        PDAP45: 0x0000000F,
        PDAP46: 0x0000000F,
        PDAP47: 0x0000000F,
        PDAP48: 0x0000000F,
        PDAP49: 0x0000000F,
        PDAP50: 0x0000000F,
        PDAP51: 0x0000000F,
        PDAP52: 0x0000000F,
        PDAP53: 0x0000000F,
        PDAP54: 0x0000000F,
        PDAP55: 0x0000000F,
        PDAP56: 0x0000000F,
        PDAP57: 0x0000000F,
        PDAP58: 0x0000000F,
        PDAP59: 0x0000000F,
        PDAP60: 0x0000000F,
        PDAP61: 0x0000000F,
        PDAP62: 0x0000000F,
        PDAP63: 0x0000000F,
        PDAP64: 0x0000000F,
        PDAP65: 0x0000000F,
        PDAP66: 0x0000000F,
        PDAP67: 0x0000000F,
        PDAP68: 0x0000000F,
        PDAP69: 0x0000000F,
        PDAP70: 0x0000000F,
        PDAP71: 0x0000000F,
        PDAP72: 0x0000000F,
        PDAP73: 0x0000000F,
        PDAP74: 0x0000000F,
        PDAP75: 0x0000000F,
        PDAP76: 0x0000000F,
        PDAP77: 0x0000000F,
        PDAP78: 0x0000000F,
        PDAP79: 0x0000000F,
        PDAP80: 0x0000000F,
        PDAP81: 0x0000000F,
        PDAP82: 0x0000000F,
        PDAP83: 0x0000000F,
        PDAP84: 0x0000000F,
        PDAP85: 0x0000000F,
        PDAP86: 0x0000000F,
        PDAP87: 0x0000000F,
        PDAP88: 0x0000000F,
        PDAP89: 0x0000000F,
        PDAP90: 0x0000000F,
        PDAP91: 0x0000000F,
        PDAP92: 0x0000000F,
        PDAP93: 0x0000000F,
        PDAP94: 0x0000000F,
        PDAP95: 0x0000000F,
        PDAP96: 0x0000000F,
        PDAP97: 0x0000000F,
        PDAP98: 0x0000000F,
        PDAP99: 0x0000000F,
        PDAP100: 0x0000000F,
        PDAP101: 0x0000000F,
        PDAP102: 0x0000000F,
        PDAP103: 0x0000000F,
        PDAP104: 0x0000000F,
        PDAP105: 0x0000000F,
        PDAP106: 0x0000000F,
        PDAP107: 0x0000000F,
        PDAP108: 0x0000000F,
        PDAP109: 0x0000000F,
        PDAP110: 0x0000000F,
        PDAP111: 0x0000000F,
        PDAP112: 0x0000000F,
        PDAP113: 0x0000000F,
        PDAP114: 0x0000000F,
        PDAP115: 0x0000000F,
        PDAP116: 0x0000000F,
        PDAP117: 0x0000000F,
        PDAP118: 0x0000000F,
        PDAP119: 0x0000000F,
        PDAP120: 0x0000000F,
        PDAP121: 0x0000000F,
        PDAP122: 0x0000000F,
        PDAP123: 0x0000000F,
        PDAP124: 0x0000000F,
        PDAP125: 0x0000000F,
        PDAP126: 0x0000000F,
        PDAP127: 0x0000000F,
        MRSA0: 0x00000000,
        MRSA1: 0x00000000,
        MRSA2: 0x00000000,
        MRSA3: 0x00000000,
        MRSA4: 0x00000000,
        MRSA5: 0x00000000,
        MRSA6: 0x00000000,
        MRSA7: 0x00000000,
        MRSA8: 0x00000000,
        MRSA9: 0x00000000,
        MRSA10: 0x00000000,
        MRSA11: 0x00000000,
        MRSA12: 0x00000000,
        MRSA13: 0x00000000,
        MRSA14: 0x00000000,
        MRSA15: 0x00000000,
        MRSA16: 0x00000000,
        MRSA17: 0x00000000,
        MRSA18: 0x00000000,
        MRSA19: 0x00000000,
        MRSA20: 0x00000000,
        MRSA21: 0x00000000,
        MRSA22: 0x00000000,
        MRSA23: 0x00000000,
        MRSA24: 0x00000000,
        MRSA25: 0x00000000,
        MRSA26: 0x00000000,
        MRSA27: 0x00000000,
        MRSA28: 0x00000000,
        MRSA29: 0x00000000,
        MRSA30: 0x00000000,
        MRSA31: 0x00000000,
        MRSA32: 0x00000000,
        MRSA33: 0x00000000,
        MRSA34: 0x00000000,
        MRSA35: 0x00000000,
        MRSA36: 0x00000000,
        MRSA37: 0x00000000,
        MRSA38: 0x00000000,
        MRSA39: 0x00000000,
        MRSA40: 0x00000000,
        MRSA41: 0x00000000,
        MRSA42: 0x00000000,
        MRSA43: 0x00000000,
        MRSA44: 0x00000000,
        MRSA45: 0x00000000,
        MRSA46: 0x00000000,
        MRSA47: 0x00000000,
        MRSA48: 0x00000000,
        MRSA49: 0x00000000,
        MRSA50: 0x00000000,
        MRSA51: 0x00000000,
        MRSA52: 0x00000000,
        MRSA53: 0x00000000,
        MRSA54: 0x00000000,
        MRSA55: 0x00000000,
        MRSA56: 0x00000000,
        MRSA57: 0x00000000,
        MRSA58: 0x00000000,
        MREA0: 0x00000000,
        MREA1: 0x00000000,
        MREA2: 0x00000000,
        MREA3: 0x00000000,
        MREA4: 0x00000000,
        MREA5: 0x00000000,
        MREA6: 0x00000000,
        MREA7: 0x00000000,
        MREA8: 0x00000000,
        MREA9: 0x00000000,
        MREA10: 0x00000000,
        MREA11: 0x00000000,
        MREA12: 0x00000000,
        MREA13: 0x00000000,
        MREA14: 0x00000000,
        MREA15: 0x00000000,
        MREA16: 0x00000000,
        MREA17: 0x00000000,
        MREA18: 0x00000000,
        MREA19: 0x00000000,
        MREA20: 0x00000000,
        MREA21: 0x00000000,
        MREA22: 0x00000000,
        MREA23: 0x00000000,
        MREA24: 0x00000000,
        MREA25: 0x00000000,
        MREA26: 0x00000000,
        MREA27: 0x00000000,
        MREA28: 0x00000000,
        MREA29: 0x00000000,
        MREA30: 0x00000000,
        MREA31: 0x00000000,
        MREA32: 0x00000000,
        MREA33: 0x00000000,
        MREA34: 0x00000000,
        MREA35: 0x00000000,
        MREA36: 0x00000000,
        MREA37: 0x00000000,
        MREA38: 0x00000000,
        MREA39: 0x00000000,
        MREA40: 0x00000000,
        MREA41: 0x00000000,
        MREA42: 0x00000000,
        MREA43: 0x00000000,
        MREA44: 0x00000000,
        MREA45: 0x00000000,
        MREA46: 0x00000000,
        MREA47: 0x00000000,
        MREA48: 0x00000000,
        MREA49: 0x00000000,
        MREA50: 0x00000000,
        MREA51: 0x00000000,
        MREA52: 0x00000000,
        MREA53: 0x00000000,
        MREA54: 0x00000000,
        MREA55: 0x00000000,
        MREA56: 0x00000000,
        MREA57: 0x00000000,
        MREA58: 0x00000000,
        MRC0: 0x000000FF,
        MRC1: 0x000000FF,
        MRC2: 0x000000FF,
        MRC3: 0x000000FF,
        MRC4: 0x000000FF,
        MRC5: 0x000000FF,
        MRC6: 0x000000FF,
        MRC7: 0x000000FF,
        MRC8: 0x000000FF,
        MRC9: 0x000000FF,
        MRC10: 0x000000FF,
        MRC11: 0x000000FF,
        MRC12: 0x000000FF,
        MRC13: 0x000000FF,
        MRC14: 0x000000FF,
        MRC15: 0x000000FF,
        MRC16: 0x000000FF,
        MRC17: 0x000000FF,
        MRC18: 0x000000FF,
        MRC19: 0x000000FF,
        MRC20: 0x000000FF,
        MRC21: 0x000000FF,
        MRC22: 0x000000FF,
        MRC23: 0x000000FF,
        MRC24: 0x000000FF,
        MRC25: 0x000000FF,
        MRC26: 0x000000FF,
        MRC27: 0x000000FF,
        MRC28: 0x000000FF,
        MRC29: 0x000000FF,
        MRC30: 0x000000FF,
        MRC31: 0x000000FF,
        MRC32: 0x000000FF,
        MRC33: 0x000000FF,
        MRC34: 0x000000FF,
        MRC35: 0x000000FF,
        MRC36: 0x000000FF,
        MRC37: 0x000000FF,
        MRC38: 0x000000FF,
        MRC39: 0x000000FF,
        MRC40: 0x000000FF,
        MRC41: 0x000000FF,
        MRC42: 0x000000FF,
        MRC43: 0x000000FF,
        MRC44: 0x000000FF,
        MRC45: 0x000000FF,
        MRC46: 0x000000FF,
        MRC47: 0x000000FF,
        MRC48: 0x000000FF,
        MRC49: 0x000000FF,
        MRC50: 0x000000FF,
        MRC51: 0x000000FF,
        MRC52: 0x000000FF,
        MRC53: 0x000000FF,
        MRC54: 0x000000FF,
        MRC55: 0x000000FF,
        MRC56: 0x000000FF,
        MRC57: 0x000000FF,
        MRC58: 0x000000FF,
        MRVS0: 0x00000000,
        MRVS1: 0x00000000,
        MRVS2: 0x00000000,
        MRVS3: 0x00000000,
        MRVS4: 0x00000000,
        MRVS5: 0x00000000,
        MRVS6: 0x00000000,
        MRVS7: 0x00000000,
        MRVS8: 0x00000000,
        MRVS9: 0x00000000,
        MRVS10: 0x00000000,
        MRVS11: 0x00000000,
        MRVS12: 0x00000000,
        MRVS13: 0x00000000,
        MRVS14: 0x00000000,
        MRVS15: 0x00000000,
        MRVS16: 0x00000000,
        MRVS17: 0x00000000,
        MRVS18: 0x00000000,
        MRVS19: 0x00000000,
        MRVS20: 0x00000000,
        MRVS21: 0x00000000,
        MRVS22: 0x00000000,
        MRVS23: 0x00000000,
        MRVS24: 0x00000000,
        MRVS25: 0x00000000,
        MRVS26: 0x00000000,
        MRVS27: 0x00000000,
        MRVS28: 0x00000000,
        MRVS29: 0x00000000,
        MRVS30: 0x00000000,
        MRVS31: 0x00000000,
        MRVS32: 0x00000000,
        MRVS33: 0x00000000,
        MRVS34: 0x00000000,
        MRVS35: 0x00000000,
        MRVS36: 0x00000000,
        MRVS37: 0x00000000,
        MRVS38: 0x00000000,
        MRVS39: 0x00000000,
        MRVS40: 0x00000000,
        MRVS41: 0x00000000,
        MRVS42: 0x00000000,
        MRVS43: 0x00000000,
        MRVS44: 0x00000000,
        MRVS45: 0x00000000,
        MRVS46: 0x00000000,
        MRVS47: 0x00000000,
        MRVS48: 0x00000000,
        MRVS49: 0x00000000,
        MRVS50: 0x00000000,
        MRVS51: 0x00000000,
        MRVS52: 0x00000000,
        MRVS53: 0x00000000,
        MRVS54: 0x00000000,
        MRVS55: 0x00000000,
        MRVS56: 0x00000000,
        MRVS57: 0x00000000,
        MRVS58: 0x00000000,
    };

    /// Safe access to RDC
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
        let taken = RDC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to RDC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = RDC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal RDC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        RDC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl RDC {
    /// The interrupts associated with RDC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::RDC];

    /// The interrupts associated with RDC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to RDC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RDC: *const RegisterBlock = 0x40c78000 as *const _;
