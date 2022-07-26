#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// PMU_LDO_PLL_REGISTER
pub mod PMU_LDO_PLL {

    /// LDO_PLL_ENABLE
    pub mod LDO_PLL_ENABLE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LDO_PLL_CONTROL_MODE
    pub mod LDO_PLL_CONTROL_MODE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SW Control
            pub const sw: u32 = 0b0;

            /// 0b1: HW Control
            pub const hw: u32 = 0b1;
        }
    }

    /// ldo_pll_ai_toggle
    pub mod LDO_PLL_AI_TOGGLE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ldo_pll_busy
    pub mod LDO_PLL_AI_BUSY {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PMU_BIAS_CTRL_REGISTER
pub mod PMU_BIAS_CTRL {

    /// wb_cfg_1p8
    pub mod WB_CFG_1P8 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// wb_vdd_sel_1p8
    pub mod WB_VDD_SEL_1P8 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VDD_LV1
            pub const lv1: u32 = 0b0;

            /// 0b1: VDD_LV2
            pub const lv2: u32 = 0b1;
        }
    }
}

/// PMU_BIAS_CTRL2_REGISTER
pub mod PMU_BIAS_CTRL2 {

    /// TMOD_wb_tst_md_1p8
    pub mod WB_TST_MD {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (9 bits: 0x1ff << 1)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MODSEL_wb_tst_md_1p8
    pub mod WB_PWR_SW_EN_1P8 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b001: No BB
            pub const bb0: u32 = 0b001;

            /// 0b010: BB
            pub const bb1: u32 = 0b010;

            /// 0b100: BB
            pub const bb2: u32 = 0b100;
        }
    }

    /// wb_adj_1p8
    pub mod WB_ADJ_1P8 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (8 bits: 0xff << 13)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Cref= 0fF Cspl= 0fF DeltaC= 0fF
            pub const wb00000000: u32 = 0b00000000;

            /// 0b00000001: Cref= 0fF Cspl= 30fF DeltaC= -30fF
            pub const wb00000001: u32 = 0b00000001;

            /// 0b00000010: Cref= 0fF Cspl= 43fF DeltaC= -43fF
            pub const wb00000010: u32 = 0b00000010;

            /// 0b00000011: Cref= 0fF Cspl= 62fF DeltaC=-62fF
            pub const wb00000011: u32 = 0b00000011;

            /// 0b00000100: Cref= 0fF Cspl=105fF DeltaC=-105fF
            pub const wb00000100: u32 = 0b00000100;

            /// 0b00000101: Cref= 30fF Cspl= 0fF DeltaC= 30fF
            pub const wb00000101: u32 = 0b00000101;

            /// 0b00000110: Cref= 30fF Cspl= 43fF DeltaC= -12fF
            pub const wb00000110: u32 = 0b00000110;

            /// 0b00000111: Cref= 30fF Cspl=105fF DeltaC= -75fF
            pub const wb00000111: u32 = 0b00000111;

            /// 0b00001000: Cref= 43fF Cspl= 0fF DeltaC= 43fF
            pub const wb00001000: u32 = 0b00001000;

            /// 0b00001001: Cref= 43fF Cspl= 30fF DeltaC= 13fF
            pub const wb00001001: u32 = 0b00001001;

            /// 0b00001010: Cref= 43fF Cspl= 62fF DeltaC= -19fF
            pub const wb00001010: u32 = 0b00001010;

            /// 0b00001011: Cref= 62fF Cspl= 0fF DeltaC= 62fF
            pub const wb00001011: u32 = 0b00001011;

            /// 0b00001100: Cref= 62fF Cspl= 43fF DeltaC= 19fF
            pub const wb00001100: u32 = 0b00001100;

            /// 0b00001101: Cref=105fF Cspl= 0fF DeltaC= 105fF
            pub const wb00001101: u32 = 0b00001101;

            /// 0b00001110: Cref=105fF Cspl=30fF DeltaC= 75fF
            pub const wb00001110: u32 = 0b00001110;

            /// 0b00001111: Cref=0fF Cspl=0fF DeltaC= 0fF
            pub const wb00001111: u32 = 0b00001111;
        }
    }

    /// FBB_M7_CONTROL_MODE
    pub mod FBB_M7_CONTROL_MODE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SW Control
            pub const sw: u32 = 0b0;

            /// 0b1: HW Control
            pub const hw: u32 = 0b1;
        }
    }

    /// RBB_SOC_CONTROL_MODE
    pub mod RBB_SOC_CONTROL_MODE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FBB_M7_CONTROL_MODE::RW;
    }

    /// RBB_LPSR_CONTROL_MODE
    pub mod RBB_LPSR_CONTROL_MODE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FBB_M7_CONTROL_MODE::RW;
    }

    /// wb_en
    pub mod WB_EN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Digital output
    pub mod WB_TST_DIG_OUT {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Digital Output pin.
    pub mod WB_OK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PMU_REF_CTRL_REGISTER
pub mod PMU_REF_CTRL {

    /// ref_ai_toggle
    pub mod REF_AI_TOGGLE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ref_ai_busy
    pub mod REF_AI_BUSY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REF_ENABLE
    pub mod REF_ENABLE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REF_CONTROL_MODE
    pub mod REF_CONTROL_MODE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SW Control
            pub const sw: u32 = 0b0;

            /// 0b1: HW Control
            pub const hw: u32 = 0b1;
        }
    }

    /// en_pll_vol_ref_buffer
    pub mod EN_PLL_VOL_REF_BUFFER {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PMU_POWER_DETECT_CTRL_REGISTER
pub mod PMU_POWER_DETECT_CTRL {

    /// ckgb_lpsr1p0
    pub mod CKGB_LPSR1P0 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LDO_PLL_ENABLE_SP_REGISTER
pub mod LDO_PLL_ENABLE_SP {

    /// ON_OFF_SETPOINT0
    pub mod ON_OFF_SETPOINT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ON
            pub const s0: u32 = 0b0;

            /// 0b1: OFF
            pub const s1: u32 = 0b1;
        }
    }

    /// ON_OFF_SETPOINT1
    pub mod ON_OFF_SETPOINT1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT2
    pub mod ON_OFF_SETPOINT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT3
    pub mod ON_OFF_SETPOINT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT4
    pub mod ON_OFF_SETPOINT4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT5
    pub mod ON_OFF_SETPOINT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT6
    pub mod ON_OFF_SETPOINT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT7
    pub mod ON_OFF_SETPOINT7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT8
    pub mod ON_OFF_SETPOINT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT9
    pub mod ON_OFF_SETPOINT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT10
    pub mod ON_OFF_SETPOINT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT11
    pub mod ON_OFF_SETPOINT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT12
    pub mod ON_OFF_SETPOINT12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT13
    pub mod ON_OFF_SETPOINT13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT14
    pub mod ON_OFF_SETPOINT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }

    /// ON_OFF_SETPOINT15
    pub mod ON_OFF_SETPOINT15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ON_OFF_SETPOINT0::RW;
    }
}

/// LDO_LPSR_ANA_ENABLE_SP_REGISTER
pub mod LDO_LPSR_ANA_ENABLE_SP {
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT0;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT1;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT10;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT11;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT12;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT13;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT14;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT15;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT2;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT3;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT4;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT5;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT6;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT7;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT8;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT9;
}

/// LDO_LPSR_ANA_LP_MODE_SP_REGISTER
pub mod LDO_LPSR_ANA_LP_MODE_SP {

    /// LP_MODE_SETPOINT0
    pub mod LP_MODE_SETPOINT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LP
            pub const lp0: u32 = 0b0;

            /// 0b1: HP
            pub const hp1: u32 = 0b1;
        }
    }

    /// LP_MODE_SETPOINT1
    pub mod LP_MODE_SETPOINT1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT2
    pub mod LP_MODE_SETPONIT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT3
    pub mod LP_MODE_SETPONIT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT4
    pub mod LP_MODE_SETPONIT4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT5
    pub mod LP_MODE_SETPONIT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT6
    pub mod LP_MODE_SETPONIT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT7
    pub mod LP_MODE_SETPONIT7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT8
    pub mod LP_MODE_SETPONIT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT9
    pub mod LP_MODE_SETPONIT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT10
    pub mod LP_MODE_SETPONIT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT11
    pub mod LP_MODE_SETPONIT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT12
    pub mod LP_MODE_SETPONIT12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT13
    pub mod LP_MODE_SETPONIT13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT14
    pub mod LP_MODE_SETPONIT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT15
    pub mod LP_MODE_SETPONIT15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }
}

/// LDO_LPSR_ANA_TRACKING_EN_SP_REGISTER
pub mod LDO_LPSR_ANA_TRACKING_EN_SP {

    /// TRACKING_EN_SETPOINT0
    pub mod TRACKING_EN_SETPOINT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const track0: u32 = 0b0;

            /// 0b1: Enabled
            pub const track1: u32 = 0b1;
        }
    }

    /// TRACKING_EN_SETPOINT1
    pub mod TRACKING_EN_SETPOINT1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT2
    pub mod TRACKING_EN_SETPOINT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT3
    pub mod TRACKING_EN_SETPOINT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT4
    pub mod TRACKING_EN_SETPOINT4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT5
    pub mod TRACKING_EN_SETPOINT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT6
    pub mod TRACKING_EN_SETPOINT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT7
    pub mod TRACKING_EN_SETPOINT7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT8
    pub mod TRACKING_EN_SETPOINT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT9
    pub mod TRACKING_EN_SETPOINT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT10
    pub mod TRACKING_EN_SETPOINT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT11
    pub mod TRACKING_EN_SETPOINT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT12
    pub mod TRACKING_EN_SETPOINT12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT13
    pub mod TRACKING_EN_SETPOINT13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT14
    pub mod TRACKING_EN_SETPOINT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }

    /// TRACKING_EN_SETPOINT15
    pub mod TRACKING_EN_SETPOINT15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRACKING_EN_SETPOINT0::RW;
    }
}

/// LDO_LPSR_ANA_BYPASS_EN_SP_REGISTER
pub mod LDO_LPSR_ANA_BYPASS_EN_SP {

    /// BYPASS_EN_SETPOINT0
    pub mod BYPASS_EN_SETPOINT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const byp0: u32 = 0b0;

            /// 0b1: Enabled
            pub const byp1: u32 = 0b1;
        }
    }

    /// BYPASS_EN_SETPOINT1
    pub mod BYPASS_EN_SETPOINT1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT2
    pub mod BYPASS_EN_SETPOINT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT3
    pub mod BYPASS_EN_SETPOINT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT4
    pub mod BYPASS_EN_SETPOINT4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT5
    pub mod BYPASS_EN_SETPOINT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT6
    pub mod BYPASS_EN_SETPOINT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT7
    pub mod BYPASS_EN_SETPOINT7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT
    pub mod BYPASS_EN_SETPOINT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT9
    pub mod BYPASS_EN_SETPOINT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT10
    pub mod BYPASS_EN_SETPOINT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT11
    pub mod BYPASS_EN_SETPOINT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT12
    pub mod BYPASS_EN_SETPOINT12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT13
    pub mod BYPASS_EN_SETPOINT13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT14
    pub mod BYPASS_EN_SETPOINT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }

    /// BYPASS_EN_SETPOINT15
    pub mod BYPASS_EN_SETPOINT15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BYPASS_EN_SETPOINT0::RW;
    }
}

/// LDO_LPSR_ANA_STBY_EN_SP_REGISTER
pub mod LDO_LPSR_ANA_STBY_EN_SP {

    /// STBY_EN_SETPOINT0
    pub mod STBY_EN_SETPOINT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const b0: u32 = 0b0;

            /// 0b1: Enabled
            pub const b1: u32 = 0b1;
        }
    }

    /// STBY_EN_SETPOINT1
    pub mod STBY_EN_SETPOINT1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT2
    pub mod STBY_EN_SETPOINT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT3
    pub mod STBY_EN_SETPOINT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT4
    pub mod STBY_EN_SETPOINT4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT5
    pub mod STBY_EN_SETPOINT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT6
    pub mod STBY_EN_SETPOINT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT7
    pub mod STBY_EN_SETPOINT7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT8
    pub mod STBY_EN_SETPOINT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT9
    pub mod STBY_EN_SETPOINT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT10
    pub mod STBY_EN_SETPOINT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT11
    pub mod STBY_EN_SETPOINT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT12
    pub mod STBY_EN_SETPOINT12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT13
    pub mod STBY_EN_SETPOINT13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT14
    pub mod STBY_EN_SETPOINT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }

    /// STBY_EN_SETPOINT15
    pub mod STBY_EN_SETPOINT15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STBY_EN_SETPOINT0::RW;
    }
}

/// LDO_LPSR_DIG_ENABLE_SP_REGISTER
pub mod LDO_LPSR_DIG_ENABLE_SP {
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT0;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT1;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT10;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT11;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT12;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT13;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT14;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT15;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT2;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT3;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT4;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT5;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT6;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT7;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT8;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT9;
}

/// LDO_LPSR_DIG_TRG_SP0_REGISTER
pub mod LDO_LPSR_DIG_TRG_SP0 {

    /// VOLTAGE_SETPOINT0
    pub mod VOLTAGE_SETPOINT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT1
    pub mod VOLTAGE_SETPOINT1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT2
    pub mod VOLTAGE_SETPOINT2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT3
    pub mod VOLTAGE_SETPOINT3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LDO_LPSR_DIG_TRG_SP1_REGISTER
pub mod LDO_LPSR_DIG_TRG_SP1 {

    /// VOLTAGE_SETPOINT4
    pub mod VOLTAGE_SETPOINT4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT5
    pub mod VOLTAGE_SETPOINT5 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT6
    pub mod VOLTAGE_SETPOINT6 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT7
    pub mod VOLTAGE_SETPOINT7 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LDO_LPSR_DIG_TRG_SP2_REGISTER
pub mod LDO_LPSR_DIG_TRG_SP2 {

    /// VOLTAGE_SETPOINT8
    pub mod VOLTAGE_SETPOINT8 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT9
    pub mod VOLTAGE_SETPOINT9 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT10
    pub mod VOLTAGE_SETPOINT10 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT11
    pub mod VOLTAGE_SETPOINT11 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LDO_LPSR_DIG_TRG_SP3_REGISTER
pub mod LDO_LPSR_DIG_TRG_SP3 {

    /// VOLTAGE_SETPOINT12
    pub mod VOLTAGE_SETPOINT12 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT13
    pub mod VOLTAGE_SETPOINT13 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT14
    pub mod VOLTAGE_SETPOINT14 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SETPOINT15
    pub mod VOLTAGE_SETPOINT15 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LDO_LPSR_DIG_LP_MODE_SP_REGISTER
pub mod LDO_LPSR_DIG_LP_MODE_SP {

    /// LP_MODE_SETPOINT0
    pub mod LP_MODE_SETPOINT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LP
            pub const lp0: u32 = 0b0;

            /// 0b1: HP
            pub const hp1: u32 = 0b1;
        }
    }

    /// LP_MODE_SETPOINT1
    pub mod LP_MODE_SETPOINT1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT2
    pub mod LP_MODE_SETPOINT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT3
    pub mod LP_MODE_SETPOINT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT4
    pub mod LP_MODE_SETPOINT4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT5
    pub mod LP_MODE_SETPOINT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT6
    pub mod LP_MODE_SETPOINT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT7
    pub mod LP_MODE_SETPOINT7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT8
    pub mod LP_MODE_SETPOINT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT9
    pub mod LP_MODE_SETPOINT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT10
    pub mod LP_MODE_SETPOINT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT11
    pub mod LP_MODE_SETPOINT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT12
    pub mod LP_MODE_SETPOINT12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT13
    pub mod LP_MODE_SETPOINT13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT14
    pub mod LP_MODE_SETPOINT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }

    /// LP_MODE_SETPOINT15
    pub mod LP_MODE_SETPOINT15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LP_MODE_SETPOINT0::RW;
    }
}

/// LDO_LPSR_DIG_TRACKING_EN_SP_REGISTER
pub mod LDO_LPSR_DIG_TRACKING_EN_SP {
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT0;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT1;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT10;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT11;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT12;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT13;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT14;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT15;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT2;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT3;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT4;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT5;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT6;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT7;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT8;
    pub use super::LDO_LPSR_ANA_TRACKING_EN_SP::TRACKING_EN_SETPOINT9;
}

/// LDO_LPSR_DIG_BYPASS_EN_SP_REGISTER
pub mod LDO_LPSR_DIG_BYPASS_EN_SP {
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT0;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT1;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT10;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT11;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT12;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT13;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT14;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT15;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT2;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT3;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT4;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT5;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT6;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT7;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT8;
    pub use super::LDO_LPSR_ANA_BYPASS_EN_SP::BYPASS_EN_SETPOINT9;
}

/// LDO_LPSR_DIG_STBY_EN_SP_REGISTER
pub mod LDO_LPSR_DIG_STBY_EN_SP {
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT0;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT1;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT10;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT11;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT12;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT13;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT14;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT15;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT2;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT3;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT4;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT5;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT6;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT7;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT8;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT9;
}

/// BANDGAP_ENABLE_SP_REGISTER
pub mod BANDGAP_ENABLE_SP {
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT0;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT1;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT10;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT11;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT12;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT13;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT14;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT15;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT2;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT3;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT4;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT5;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT6;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT7;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT8;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT9;
}

/// FBB_M7_ENABLE_SP_REGISTER
pub mod FBB_M7_ENABLE_SP {
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT0;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT1;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT10;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT11;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT12;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT13;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT14;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT15;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT2;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT3;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT4;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT5;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT6;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT7;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT8;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT9;
}

/// RBB_SOC_ENABLE_SP_REGISTER
pub mod RBB_SOC_ENABLE_SP {
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT0;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT1;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT10;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT11;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT12;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT13;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT14;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT15;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT2;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT3;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT4;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT5;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT6;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT7;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT8;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT9;
}

/// RBB_LPSR_ENABLE_SP_REGISTER
pub mod RBB_LPSR_ENABLE_SP {
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT0;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT1;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT10;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT11;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT12;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT13;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT14;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT15;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT2;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT3;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT4;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT5;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT6;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT7;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT8;
    pub use super::LDO_PLL_ENABLE_SP::ON_OFF_SETPOINT9;
}

/// BANDGAP_STBY_EN_SP_REGISTER
pub mod BANDGAP_STBY_EN_SP {
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT0;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT1;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT10;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT11;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT12;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT13;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT14;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT15;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT2;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT3;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT4;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT5;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT6;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT7;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT8;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT9;
}

/// PLL_LDO_STBY_EN_SP_REGISTER
pub mod PLL_LDO_STBY_EN_SP {
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT0;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT1;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT10;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT11;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT12;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT13;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT14;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT15;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT2;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT3;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT4;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT5;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT6;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT7;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT8;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT9;
}

/// FBB_M7_STBY_EN_SP_REGISTER
pub mod FBB_M7_STBY_EN_SP {
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT0;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT1;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT10;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT11;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT12;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT13;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT14;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT15;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT2;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT3;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT4;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT5;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT6;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT7;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT8;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT9;
}

/// RBB_SOC_STBY_EN_SP_REGISTER
pub mod RBB_SOC_STBY_EN_SP {
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT0;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT1;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT10;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT11;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT12;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT13;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT14;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT15;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT2;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT3;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT4;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT5;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT6;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT7;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT8;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT9;
}

/// RBB_LPSR_STBY_EN_SP_REGISTER
pub mod RBB_LPSR_STBY_EN_SP {
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT0;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT1;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT10;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT11;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT12;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT13;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT14;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT15;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT2;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT3;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT4;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT5;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT6;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT7;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT8;
    pub use super::LDO_LPSR_ANA_STBY_EN_SP::STBY_EN_SETPOINT9;
}

/// FBB_M7_CONFIGURE_REGISTER
pub mod FBB_M7_CONFIGURE {

    /// wb_cfg_pw
    pub mod WB_CFG_PW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// wb_cfg_nw
    pub mod WB_CFG_NW {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// oscillator_bits
    pub mod OSCILLATOR_BITS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// regulator_strength
    pub mod REGULATOR_STRENGTH {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RBB_LPSR_CONFIGURE_REGISTER
pub mod RBB_LPSR_CONFIGURE {
    pub use super::FBB_M7_CONFIGURE::OSCILLATOR_BITS;
    pub use super::FBB_M7_CONFIGURE::REGULATOR_STRENGTH;
    pub use super::FBB_M7_CONFIGURE::WB_CFG_NW;
    pub use super::FBB_M7_CONFIGURE::WB_CFG_PW;
}

/// RBB_SOC_CONFIGURE_REGISTER
pub mod RBB_SOC_CONFIGURE {
    pub use super::FBB_M7_CONFIGURE::OSCILLATOR_BITS;
    pub use super::FBB_M7_CONFIGURE::REGULATOR_STRENGTH;
    pub use super::FBB_M7_CONFIGURE::WB_CFG_NW;
    pub use super::FBB_M7_CONFIGURE::WB_CFG_PW;
}

/// REFTOP_OTP_TRIM_VALUE_REGISTER
pub mod REFTOP_OTP_TRIM_VALUE {

    /// REFTOP_IBZTCADJ
    pub mod REFTOP_IBZTCADJ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REFTOP_VBGADJ
    pub mod REFTOP_VBGADJ {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REFTOP_TRIM_EN
    pub mod REFTOP_TRIM_EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LPSR_1P8_LDO_OTP_TRIM_VALUE_REGISTER
pub mod LPSR_1P8_LDO_OTP_TRIM_VALUE {

    /// LPSR_LDO_1P8_TRIM
    pub mod LPSR_LDO_1P8_TRIM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPSR_LDO_1P8_TRIM_EN
    pub mod LPSR_LDO_1P8_TRIM_EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 320],

    /// PMU_LDO_PLL_REGISTER
    pub PMU_LDO_PLL: RWRegister<u32>,

    _reserved2: [u32; 19],

    /// PMU_BIAS_CTRL_REGISTER
    pub PMU_BIAS_CTRL: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// PMU_BIAS_CTRL2_REGISTER
    pub PMU_BIAS_CTRL2: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// PMU_REF_CTRL_REGISTER
    pub PMU_REF_CTRL: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// PMU_POWER_DETECT_CTRL_REGISTER
    pub PMU_POWER_DETECT_CTRL: RWRegister<u32>,

    _reserved6: [u32; 31],

    /// LDO_PLL_ENABLE_SP_REGISTER
    pub LDO_PLL_ENABLE_SP: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// LDO_LPSR_ANA_ENABLE_SP_REGISTER
    pub LDO_LPSR_ANA_ENABLE_SP: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// LDO_LPSR_ANA_LP_MODE_SP_REGISTER
    pub LDO_LPSR_ANA_LP_MODE_SP: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// LDO_LPSR_ANA_TRACKING_EN_SP_REGISTER
    pub LDO_LPSR_ANA_TRACKING_EN_SP: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// LDO_LPSR_ANA_BYPASS_EN_SP_REGISTER
    pub LDO_LPSR_ANA_BYPASS_EN_SP: RWRegister<u32>,

    _reserved11: [u32; 3],

    /// LDO_LPSR_ANA_STBY_EN_SP_REGISTER
    pub LDO_LPSR_ANA_STBY_EN_SP: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// LDO_LPSR_DIG_ENABLE_SP_REGISTER
    pub LDO_LPSR_DIG_ENABLE_SP: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// LDO_LPSR_DIG_TRG_SP0_REGISTER
    pub LDO_LPSR_DIG_TRG_SP0: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// LDO_LPSR_DIG_TRG_SP1_REGISTER
    pub LDO_LPSR_DIG_TRG_SP1: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// LDO_LPSR_DIG_TRG_SP2_REGISTER
    pub LDO_LPSR_DIG_TRG_SP2: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// LDO_LPSR_DIG_TRG_SP3_REGISTER
    pub LDO_LPSR_DIG_TRG_SP3: RWRegister<u32>,

    _reserved17: [u32; 3],

    /// LDO_LPSR_DIG_LP_MODE_SP_REGISTER
    pub LDO_LPSR_DIG_LP_MODE_SP: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// LDO_LPSR_DIG_TRACKING_EN_SP_REGISTER
    pub LDO_LPSR_DIG_TRACKING_EN_SP: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// LDO_LPSR_DIG_BYPASS_EN_SP_REGISTER
    pub LDO_LPSR_DIG_BYPASS_EN_SP: RWRegister<u32>,

    _reserved20: [u32; 3],

    /// LDO_LPSR_DIG_STBY_EN_SP_REGISTER
    pub LDO_LPSR_DIG_STBY_EN_SP: RWRegister<u32>,

    _reserved21: [u32; 3],

    /// BANDGAP_ENABLE_SP_REGISTER
    pub BANDGAP_ENABLE_SP: RWRegister<u32>,

    _reserved22: [u32; 3],

    /// FBB_M7_ENABLE_SP_REGISTER
    pub FBB_M7_ENABLE_SP: RWRegister<u32>,

    _reserved23: [u32; 3],

    /// RBB_SOC_ENABLE_SP_REGISTER
    pub RBB_SOC_ENABLE_SP: RWRegister<u32>,

    _reserved24: [u32; 3],

    /// RBB_LPSR_ENABLE_SP_REGISTER
    pub RBB_LPSR_ENABLE_SP: RWRegister<u32>,

    _reserved25: [u32; 3],

    /// BANDGAP_STBY_EN_SP_REGISTER
    pub BANDGAP_STBY_EN_SP: RWRegister<u32>,

    _reserved26: [u32; 3],

    /// PLL_LDO_STBY_EN_SP_REGISTER
    pub PLL_LDO_STBY_EN_SP: RWRegister<u32>,

    _reserved27: [u32; 3],

    /// FBB_M7_STBY_EN_SP_REGISTER
    pub FBB_M7_STBY_EN_SP: RWRegister<u32>,

    _reserved28: [u32; 3],

    /// RBB_SOC_STBY_EN_SP_REGISTER
    pub RBB_SOC_STBY_EN_SP: RWRegister<u32>,

    _reserved29: [u32; 3],

    /// RBB_LPSR_STBY_EN_SP_REGISTER
    pub RBB_LPSR_STBY_EN_SP: RWRegister<u32>,

    _reserved30: [u32; 3],

    /// FBB_M7_CONFIGURE_REGISTER
    pub FBB_M7_CONFIGURE: RWRegister<u32>,

    _reserved31: [u32; 3],

    /// RBB_LPSR_CONFIGURE_REGISTER
    pub RBB_LPSR_CONFIGURE: RWRegister<u32>,

    _reserved32: [u32; 3],

    /// RBB_SOC_CONFIGURE_REGISTER
    pub RBB_SOC_CONFIGURE: RWRegister<u32>,

    _reserved33: [u32; 3],

    /// REFTOP_OTP_TRIM_VALUE_REGISTER
    pub REFTOP_OTP_TRIM_VALUE: RORegister<u32>,

    _reserved34: [u32; 7],

    /// LPSR_1P8_LDO_OTP_TRIM_VALUE_REGISTER
    pub LPSR_1P8_LDO_OTP_TRIM_VALUE: RORegister<u32>,
}
pub struct ResetValues {
    pub PMU_LDO_PLL: u32,
    pub PMU_BIAS_CTRL: u32,
    pub PMU_BIAS_CTRL2: u32,
    pub PMU_REF_CTRL: u32,
    pub PMU_POWER_DETECT_CTRL: u32,
    pub LDO_PLL_ENABLE_SP: u32,
    pub LDO_LPSR_ANA_ENABLE_SP: u32,
    pub LDO_LPSR_ANA_LP_MODE_SP: u32,
    pub LDO_LPSR_ANA_TRACKING_EN_SP: u32,
    pub LDO_LPSR_ANA_BYPASS_EN_SP: u32,
    pub LDO_LPSR_ANA_STBY_EN_SP: u32,
    pub LDO_LPSR_DIG_ENABLE_SP: u32,
    pub LDO_LPSR_DIG_TRG_SP0: u32,
    pub LDO_LPSR_DIG_TRG_SP1: u32,
    pub LDO_LPSR_DIG_TRG_SP2: u32,
    pub LDO_LPSR_DIG_TRG_SP3: u32,
    pub LDO_LPSR_DIG_LP_MODE_SP: u32,
    pub LDO_LPSR_DIG_TRACKING_EN_SP: u32,
    pub LDO_LPSR_DIG_BYPASS_EN_SP: u32,
    pub LDO_LPSR_DIG_STBY_EN_SP: u32,
    pub BANDGAP_ENABLE_SP: u32,
    pub FBB_M7_ENABLE_SP: u32,
    pub RBB_SOC_ENABLE_SP: u32,
    pub RBB_LPSR_ENABLE_SP: u32,
    pub BANDGAP_STBY_EN_SP: u32,
    pub PLL_LDO_STBY_EN_SP: u32,
    pub FBB_M7_STBY_EN_SP: u32,
    pub RBB_SOC_STBY_EN_SP: u32,
    pub RBB_LPSR_STBY_EN_SP: u32,
    pub FBB_M7_CONFIGURE: u32,
    pub RBB_LPSR_CONFIGURE: u32,
    pub RBB_SOC_CONFIGURE: u32,
    pub REFTOP_OTP_TRIM_VALUE: u32,
    pub LPSR_1P8_LDO_OTP_TRIM_VALUE: u32,
}
pub struct Instance<const N: u8> {
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) addr: u32,
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) intrs: &'static [crate::Interrupt],
}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> ::core::ops::Deref for Instance<N> {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

#[cfg(not(feature = "nosync"))]
unsafe impl<const N: u8> Send for Instance<N> {}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> Instance<N> {
    /// Return the interrupt signals associated with this
    /// peripheral instance
    ///
    /// Collection may be empty if there is no interrupt signal
    /// associated with the peripheral. There's no guarantee for
    /// interrupt signal ordering in the collection.
    #[inline(always)]
    pub const fn interrupts<'a>(&'a self) -> &'a [crate::Interrupt] {
        self.intrs
    }
}
