#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0: u32,
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0_SET: u32,
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0_CLR: u32,
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0_TOG: u32,
    #[doc = "Analog Control Register CTRL1"]
    pub CTRL1: u32,
    #[doc = "Analog Control Register CTRL1"]
    pub CTRL1_SET: u32,
    #[doc = "Analog Control Register CTRL1"]
    pub CTRL1_CLR: u32,
    #[doc = "Analog Control Register CTRL1"]
    pub CTRL1_TOG: u32,
    #[doc = "Analog Control Register CTRL2"]
    pub CTRL2: u32,
    #[doc = "Analog Control Register CTRL2"]
    pub CTRL2_SET: u32,
    #[doc = "Analog Control Register CTRL2"]
    pub CTRL2_CLR: u32,
    #[doc = "Analog Control Register CTRL2"]
    pub CTRL2_TOG: u32,
    #[doc = "Analog Control Register CTRL3"]
    pub CTRL3: u32,
    #[doc = "Analog Control Register CTRL3"]
    pub CTRL3_SET: u32,
    #[doc = "Analog Control Register CTRL3"]
    pub CTRL3_CLR: u32,
    #[doc = "Analog Control Register CTRL3"]
    pub CTRL3_TOG: u32,
    _reserved0: [u8; 0x10],
    #[doc = "Analog Status Register STAT0"]
    pub STAT0: u32,
    _reserved1: [u8; 0x0c],
    #[doc = "Analog Status Register STAT1"]
    pub STAT1: u32,
    _reserved2: [u8; 0x0c],
    #[doc = "Analog Status Register STAT2"]
    pub STAT2: u32,
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0 {
    pub use crate::RW as access;
    #[doc = "Divide value for ref_clk to generate slow_clk (used inside this IP)"]
    pub mod REF_CLK_DIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_SET {
    pub use crate::RW as access;
    #[doc = "Divide value for ref_clk to generate slow_clk (used inside this IP)"]
    pub mod REF_CLK_DIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_CLR {
    pub use crate::RW as access;
    #[doc = "Divide value for ref_clk to generate slow_clk (used inside this IP)"]
    pub mod REF_CLK_DIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_TOG {
    pub use crate::RW as access;
    #[doc = "Divide value for ref_clk to generate slow_clk (used inside this IP)"]
    pub mod REF_CLK_DIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL1"]
pub mod CTRL1 {
    pub use crate::RW as access;
    #[doc = "Negative hysteresis value for the tuned clock"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Positive hysteresis value for the tuned clock"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Target count for the fast clock"]
    pub mod TARGET_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL1"]
pub mod CTRL1_SET {
    pub use crate::RW as access;
    #[doc = "Negative hysteresis value for the tuned clock"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Positive hysteresis value for the tuned clock"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Target count for the fast clock"]
    pub mod TARGET_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL1"]
pub mod CTRL1_CLR {
    pub use crate::RW as access;
    #[doc = "Negative hysteresis value for the tuned clock"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Positive hysteresis value for the tuned clock"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Target count for the fast clock"]
    pub mod TARGET_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL1"]
pub mod CTRL1_TOG {
    pub use crate::RW as access;
    #[doc = "Negative hysteresis value for the tuned clock"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Positive hysteresis value for the tuned clock"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Target count for the fast clock"]
    pub mod TARGET_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL2"]
pub mod CTRL2 {
    pub use crate::RW as access;
    #[doc = "Inverse tuning direction."]
    pub mod TUNE_INV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the tuning logic"]
    pub mod TUNE_BYP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    pub mod TUNE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Start/Stop tuning."]
    pub mod TUNE_START {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Program the oscillator frequency."]
    pub mod OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL2"]
pub mod CTRL2_SET {
    pub use crate::RW as access;
    #[doc = "Inverse tuning direction."]
    pub mod TUNE_INV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the tuning logic"]
    pub mod TUNE_BYP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    pub mod TUNE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Start/Stop tuning."]
    pub mod TUNE_START {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Program the oscillator frequency."]
    pub mod OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL2"]
pub mod CTRL2_CLR {
    pub use crate::RW as access;
    #[doc = "Inverse tuning direction."]
    pub mod TUNE_INV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the tuning logic"]
    pub mod TUNE_BYP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    pub mod TUNE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Start/Stop tuning."]
    pub mod TUNE_START {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Program the oscillator frequency."]
    pub mod OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL2"]
pub mod CTRL2_TOG {
    pub use crate::RW as access;
    #[doc = "Inverse tuning direction."]
    pub mod TUNE_INV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the tuning logic"]
    pub mod TUNE_BYP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    pub mod TUNE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Start/Stop tuning."]
    pub mod TUNE_START {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Program the oscillator frequency."]
    pub mod OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL3"]
pub mod CTRL3 {
    pub use crate::RW as access;
    #[doc = "Clear the error flag CLK1M_ERR"]
    pub mod CLR_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect"]
            pub const CLR_ERR_0: u32 = 0;
            #[doc = "Clears the error flag CLK1M_ERR in status register STAT0"]
            pub const CLR_ERR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1: Disable clk_1m_out."]
    pub mod EN_1M_CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select free/locked 1MHz output"]
    pub mod MUX_1M_CLK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count for the locked clk_1m_out."]
    pub mod COUNT_1M_CLK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL3"]
pub mod CTRL3_SET {
    pub use crate::RW as access;
    #[doc = "Clear the error flag CLK1M_ERR"]
    pub mod CLR_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable 1MHz output Clock"]
    pub mod EN_1M_CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select free/locked 1MHz output"]
    pub mod MUX_1M_CLK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count for the locked clk_1m_out"]
    pub mod COUNT_1M_CLK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL3"]
pub mod CTRL3_CLR {
    pub use crate::RW as access;
    #[doc = "Clear the error flag CLK1M_ERR"]
    pub mod CLR_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable 1MHz output Clock"]
    pub mod EN_1M_CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select free/locked 1MHz output"]
    pub mod MUX_1M_CLK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count for the locked clk_1m_out"]
    pub mod COUNT_1M_CLK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL3"]
pub mod CTRL3_TOG {
    pub use crate::RW as access;
    #[doc = "Clear the error flag CLK1M_ERR"]
    pub mod CLR_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable 1MHz output Clock"]
    pub mod EN_1M_CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select free/locked 1MHz output"]
    pub mod MUX_1M_CLK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count for the locked clk_1m_out"]
    pub mod COUNT_1M_CLK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0 {
    pub use crate::RO as access;
    #[doc = "Error flag for clk_1m_locked"]
    pub mod CLK1M_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect"]
            pub const CLK1M_ERR_0: u32 = 0;
            #[doc = "The count value has been reached within one divided ref_clk period"]
            pub const CLK1M_ERR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Status Register STAT1"]
pub mod STAT1 {
    pub use crate::RO as access;
    #[doc = "Current count for the fast clock"]
    pub mod CURR_COUNT_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Status Register STAT2"]
pub mod STAT2 {
    pub use crate::RO as access;
    #[doc = "Current tuning value used by oscillator"]
    pub mod CURR_OSC_TUNE_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
