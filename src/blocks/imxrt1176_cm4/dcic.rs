#[doc = "DCIC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "DCIC Control Register"]
    pub DCICC: u32,
    #[doc = "DCIC Interrupt Control Register"]
    pub DCICIC: u32,
    #[doc = "DCIC Status Register"]
    pub DCICS: u32,
    _reserved0: [u8; 0x04],
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC0: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS0: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS0: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS0: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC1: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS1: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS1: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS1: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC2: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS2: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS2: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS2: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC3: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS3: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS3: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS3: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC4: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS4: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS4: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS4: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC5: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS5: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS5: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS5: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC6: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS6: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS6: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS6: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC7: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS7: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS7: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS7: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC8: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS8: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS8: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS8: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC9: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS9: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS9: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS9: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC10: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS10: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS10: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS10: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC11: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS11: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS11: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS11: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC12: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS12: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS12: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS12: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC13: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS13: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS13: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS13: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC14: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS14: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS14: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS14: u32,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC15: u32,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS15: u32,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS15: u32,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS15: u32,
}
#[doc = "DCIC Control Register"]
pub mod DCICC {
    pub use crate::RW as access;
    #[doc = "Integrity Check enable. Main enable switch."]
    pub mod IC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const IC_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const IC_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DATA_EN_IN signal polarity."]
    pub mod DE_POL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Active High."]
            pub const DE_POL_0: u32 = 0;
            #[doc = "Active Low."]
            pub const DE_POL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "HSYNC_IN signal polarity."]
    pub mod HSYNC_POL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Active High."]
            pub const HSYNC_POL_0: u32 = 0;
            #[doc = "Active Low."]
            pub const HSYNC_POL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "VSYNC_IN signal polarity."]
    pub mod VSYNC_POL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Active High."]
            pub const VSYNC_POL_0: u32 = 0;
            #[doc = "Active Low."]
            pub const VSYNC_POL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DISP_CLK signal polarity."]
    pub mod CLK_POL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not inverted (default)."]
            pub const CLK_POL_0: u32 = 0;
            #[doc = "Inverted."]
            pub const CLK_POL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC Interrupt Control Register"]
pub mod DCICIC {
    pub use crate::RW as access;
    #[doc = "Error Interrupt mask. Can be changed only while FREEZE_MASK = 0."]
    pub mod EI_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Mask disabled - Interrupt assertion enabled"]
            pub const EI_MASK_0: u32 = 0;
            #[doc = "Mask enabled - Interrupt assertion disabled"]
            pub const EI_MASK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Functional Interrupt mask. Can be changed only while FREEZE_MASK = 0."]
    pub mod FI_MASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Mask disabled - Interrupt assertion enabled"]
            pub const FI_MASK_0: u32 = 0;
            #[doc = "Mask enabled - Interrupt assertion disabled"]
            pub const FI_MASK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable change of interrupt masks. \"Sticky\" bit which can be set once and cleared by reset only."]
    pub mod FREEZE_MASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Masks change allowed"]
            pub const FREEZE_MASK_0: u32 = 0;
            #[doc = "Masks are frozen"]
            pub const FREEZE_MASK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External controller mismatch indication signal."]
    pub mod EXT_SIG_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const EXT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const EXT_SIG_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC Status Register"]
pub mod DCICS {
    pub use crate::RW as access;
    #[doc = "Each set bit of this field indicates there was a mismatch at the appropriate ROIs signature during the last frame"]
    pub mod ROI_MATCH_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI calculated CRC matches expected signature"]
            pub const ROI_MATCH_STAT_0: u32 = 0;
            #[doc = "Mismatch at ROI calculated CRC"]
            pub const ROI_MATCH_STAT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Error Interrupt status"]
    pub mod EI_STAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No pending Interrupt"]
            pub const EI_STAT_0: u32 = 0;
            #[doc = "Pending Interrupt"]
            pub const EI_STAT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Functional Interrupt status. Write \"1\" to clear."]
    pub mod FI_STAT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No pending Interrupt"]
            pub const FI_STAT_0: u32 = 0;
            #[doc = "Pending Interrupt"]
            pub const FI_STAT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC0 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS0 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS0 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS0 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC1 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS1 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS1 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS1 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC2 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS2 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS2 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS2 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC3 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS3 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS3 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS3 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC4 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS4 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS4 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS4 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC5 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS5 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS5 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS5 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC6 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS6 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS6 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS6 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC7 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS7 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS7 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS7 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC8 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS8 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS8 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS8 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC9 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS9 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS9 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS9 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC10 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS10 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS10 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS10 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC11 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS11 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS11 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS11 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC12 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS12 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS12 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS12 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC13 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS13 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS13 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS13 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC14 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS14 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS14 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS14 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC15 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS15 {
    pub use crate::RW as access;
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS15 {
    pub use crate::RW as access;
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS15 {
    pub use crate::RO as access;
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
