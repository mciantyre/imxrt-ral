#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PWM
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};

/// Output Enable Register
pub mod OUTEN {

    /// PWM_X Output Enables
    pub mod PWMX_EN {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_X output disabled.
            pub const PWMX_EN_0: u16 = 0b0000;

            /// 0b0001: PWM_X output enabled.
            pub const PWMX_EN_1: u16 = 0b0001;
        }
    }

    /// PWM_B Output Enables
    pub mod PWMB_EN {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_B output disabled.
            pub const PWMB_EN_0: u16 = 0b0000;

            /// 0b0001: PWM_B output enabled.
            pub const PWMB_EN_1: u16 = 0b0001;
        }
    }

    /// PWM_A Output Enables
    pub mod PWMA_EN {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_A output disabled.
            pub const PWMA_EN_0: u16 = 0b0000;

            /// 0b0001: PWM_A output enabled.
            pub const PWMA_EN_1: u16 = 0b0001;
        }
    }
}

/// Mask Register
pub mod MASK {

    /// PWM_X Masks
    pub mod MASKX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_X output normal.
            pub const MASKX_0: u16 = 0b0000;

            /// 0b0001: PWM_X output masked.
            pub const MASKX_1: u16 = 0b0001;
        }
    }

    /// PWM_B Masks
    pub mod MASKB {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_B output normal.
            pub const MASKB_0: u16 = 0b0000;

            /// 0b0001: PWM_B output masked.
            pub const MASKB_1: u16 = 0b0001;
        }
    }

    /// PWM_A Masks
    pub mod MASKA {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_A output normal.
            pub const MASKA_0: u16 = 0b0000;

            /// 0b0001: PWM_A output masked.
            pub const MASKA_1: u16 = 0b0001;
        }
    }

    /// Update Mask Bits Immediately
    pub mod UPDATE_MASK {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule.
            pub const UPDATE_MASK_0: u16 = 0b0000;

            /// 0b0001: Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit.
            pub const UPDATE_MASK_1: u16 = 0b0001;
        }
    }
}

/// Software Controlled Output Register
pub mod SWCOUT {

    /// Submodule 0 Software Controlled Output 45
    pub mod SM0OUT45 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45.
            pub const SM0OUT45_0: u16 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45.
            pub const SM0OUT45_1: u16 = 0b1;
        }
    }

    /// Submodule 0 Software Controlled Output 23
    pub mod SM0OUT23 {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23.
            pub const SM0OUT23_0: u16 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23.
            pub const SM0OUT23_1: u16 = 0b1;
        }
    }

    /// Submodule 1 Software Controlled Output 45
    pub mod SM1OUT45 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45.
            pub const SM1OUT45_0: u16 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45.
            pub const SM1OUT45_1: u16 = 0b1;
        }
    }

    /// Submodule 1 Software Controlled Output 23
    pub mod SM1OUT23 {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23.
            pub const SM1OUT23_0: u16 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23.
            pub const SM1OUT23_1: u16 = 0b1;
        }
    }

    /// Submodule 2 Software Controlled Output 45
    pub mod SM2OUT45 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45.
            pub const SM2OUT45_0: u16 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45.
            pub const SM2OUT45_1: u16 = 0b1;
        }
    }

    /// Submodule 2 Software Controlled Output 23
    pub mod SM2OUT23 {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23.
            pub const SM2OUT23_0: u16 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23.
            pub const SM2OUT23_1: u16 = 0b1;
        }
    }

    /// Submodule 3 Software Controlled Output 45
    pub mod SM3OUT45 {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45.
            pub const SM3OUT45_0: u16 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45.
            pub const SM3OUT45_1: u16 = 0b1;
        }
    }

    /// Submodule 3 Software Controlled Output 23
    pub mod SM3OUT23 {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23.
            pub const SM3OUT23_0: u16 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23.
            pub const SM3OUT23_1: u16 = 0b1;
        }
    }
}

/// PWM Source Select Register
pub mod DTSRCSEL {

    /// Submodule 0 PWM45 Control Select
    pub mod SM0SEL45 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM0PWM45 signal is used by the deadtime logic.
            pub const SM0SEL45_0: u16 = 0b00;

            /// 0b01: Inverted generated SM0PWM45 signal is used by the deadtime logic.
            pub const SM0SEL45_1: u16 = 0b01;

            /// 0b10: SWCOUT\[SM0OUT45\] is used by the deadtime logic.
            pub const SM0SEL45_2: u16 = 0b10;

            /// 0b11: PWM0_EXTB signal is used by the deadtime logic.
            pub const SM0SEL45_3: u16 = 0b11;
        }
    }

    /// Submodule 0 PWM23 Control Select
    pub mod SM0SEL23 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM0PWM23 signal is used by the deadtime logic.
            pub const SM0SEL23_0: u16 = 0b00;

            /// 0b01: Inverted generated SM0PWM23 signal is used by the deadtime logic.
            pub const SM0SEL23_1: u16 = 0b01;

            /// 0b10: SWCOUT\[SM0OUT23\] is used by the deadtime logic.
            pub const SM0SEL23_2: u16 = 0b10;

            /// 0b11: PWM0_EXTA signal is used by the deadtime logic.
            pub const SM0SEL23_3: u16 = 0b11;
        }
    }

    /// Submodule 1 PWM45 Control Select
    pub mod SM1SEL45 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM1PWM45 signal is used by the deadtime logic.
            pub const SM1SEL45_0: u16 = 0b00;

            /// 0b01: Inverted generated SM1PWM45 signal is used by the deadtime logic.
            pub const SM1SEL45_1: u16 = 0b01;

            /// 0b10: SWCOUT\[SM1OUT45\] is used by the deadtime logic.
            pub const SM1SEL45_2: u16 = 0b10;

            /// 0b11: PWM1_EXTB signal is used by the deadtime logic.
            pub const SM1SEL45_3: u16 = 0b11;
        }
    }

    /// Submodule 1 PWM23 Control Select
    pub mod SM1SEL23 {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM1PWM23 signal is used by the deadtime logic.
            pub const SM1SEL23_0: u16 = 0b00;

            /// 0b01: Inverted generated SM1PWM23 signal is used by the deadtime logic.
            pub const SM1SEL23_1: u16 = 0b01;

            /// 0b10: SWCOUT\[SM1OUT23\] is used by the deadtime logic.
            pub const SM1SEL23_2: u16 = 0b10;

            /// 0b11: PWM1_EXTA signal is used by the deadtime logic.
            pub const SM1SEL23_3: u16 = 0b11;
        }
    }

    /// Submodule 2 PWM45 Control Select
    pub mod SM2SEL45 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM2PWM45 signal is used by the deadtime logic.
            pub const SM2SEL45_0: u16 = 0b00;

            /// 0b01: Inverted generated SM2PWM45 signal is used by the deadtime logic.
            pub const SM2SEL45_1: u16 = 0b01;

            /// 0b10: SWCOUT\[SM2OUT45\] is used by the deadtime logic.
            pub const SM2SEL45_2: u16 = 0b10;

            /// 0b11: PWM2_EXTB signal is used by the deadtime logic.
            pub const SM2SEL45_3: u16 = 0b11;
        }
    }

    /// Submodule 2 PWM23 Control Select
    pub mod SM2SEL23 {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM2PWM23 signal is used by the deadtime logic.
            pub const SM2SEL23_0: u16 = 0b00;

            /// 0b01: Inverted generated SM2PWM23 signal is used by the deadtime logic.
            pub const SM2SEL23_1: u16 = 0b01;

            /// 0b10: SWCOUT\[SM2OUT23\] is used by the deadtime logic.
            pub const SM2SEL23_2: u16 = 0b10;

            /// 0b11: PWM2_EXTA signal is used by the deadtime logic.
            pub const SM2SEL23_3: u16 = 0b11;
        }
    }

    /// Submodule 3 PWM45 Control Select
    pub mod SM3SEL45 {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM3PWM45 signal is used by the deadtime logic.
            pub const SM3SEL45_0: u16 = 0b00;

            /// 0b01: Inverted generated SM3PWM45 signal is used by the deadtime logic.
            pub const SM3SEL45_1: u16 = 0b01;

            /// 0b10: SWCOUT\[SM3OUT45\] is used by the deadtime logic.
            pub const SM3SEL45_2: u16 = 0b10;

            /// 0b11: PWM3_EXTB signal is used by the deadtime logic.
            pub const SM3SEL45_3: u16 = 0b11;
        }
    }

    /// Submodule 3 PWM23 Control Select
    pub mod SM3SEL23 {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM3PWM23 signal is used by the deadtime logic.
            pub const SM3SEL23_0: u16 = 0b00;

            /// 0b01: Inverted generated SM3PWM23 signal is used by the deadtime logic.
            pub const SM3SEL23_1: u16 = 0b01;

            /// 0b10: SWCOUT\[SM3OUT23\] is used by the deadtime logic.
            pub const SM3SEL23_2: u16 = 0b10;

            /// 0b11: PWM3_EXTA signal is used by the deadtime logic.
            pub const SM3SEL23_3: u16 = 0b11;
        }
    }
}

/// Master Control Register
pub mod MCTRL {

    /// Load Okay
    pub mod LDOK {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Do not load new values.
            pub const LDOK_0: u16 = 0b0000;

            /// 0b0001: Load prescaler, modulus, and PWM values of the corresponding submodule.
            pub const LDOK_1: u16 = 0b0001;
        }
    }

    /// Clear Load Okay
    pub mod CLDOK {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run
    pub mod RUN {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM generator is disabled in the corresponding submodule.
            pub const RUN_0: u16 = 0b0000;

            /// 0b0001: PWM generator is enabled in the corresponding submodule.
            pub const RUN_1: u16 = 0b0001;
        }
    }

    /// Current Polarity
    pub mod IPOL {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM23 is used to generate complementary PWM pair in the corresponding submodule.
            pub const IPOL_0: u16 = 0b0000;

            /// 0b0001: PWM45 is used to generate complementary PWM pair in the corresponding submodule.
            pub const IPOL_1: u16 = 0b0001;
        }
    }
}

/// Master Control 2 Register
pub mod MCTRL2 {

    /// Monitor PLL State
    pub mod MONPLL {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software.
            pub const MONPLL_0: u16 = 0b00;

            /// 0b01: Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems.
            pub const MONPLL_1: u16 = 0b01;

            /// 0b10: Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset.
            pub const MONPLL_2: u16 = 0b10;

            /// 0b11: Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset.
            pub const MONPLL_3: u16 = 0b11;
        }
    }
}

/// Fault Control Register
pub mod FCTRL0 {

    /// Fault Interrupt Enables
    pub mod FIE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: FAULTx CPU interrupt requests disabled.
            pub const FIE_0: u16 = 0b0000;

            /// 0b0001: FAULTx CPU interrupt requests enabled.
            pub const FIE_1: u16 = 0b0001;
        }
    }

    /// Fault Safety Mode
    pub mod FSAFE {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\[FFLAGx\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\[FFULL\] without regard to the state of FSTS\[FFPINx\]. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn).
            pub const FSAFE_0: u16 = 0b0000;

            /// 0b0001: Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\[FFLAGx\] is clear and FSTS\[FFPINx\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\[FFULL\].
            pub const FSAFE_1: u16 = 0b0001;
        }
    }

    /// Automatic Fault Clearing
    pub mod FAUTO {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\[FFLAGx\] is clear at the start of a half cycle or full cycle depending the state of FSTS\[FFULL\]. This is further controlled by FCTRL\[FSAFE\].
            pub const FAUTO_0: u16 = 0b0000;

            /// 0b0001: Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\[FFPINx\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\[FFULL\] without regard to the state of FSTS\[FFLAGx\].
            pub const FAUTO_1: u16 = 0b0001;
        }
    }

    /// Fault Level
    pub mod FLVL {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: A logic 0 on the fault input indicates a fault condition.
            pub const FLVL_0: u16 = 0b0000;

            /// 0b0001: A logic 1 on the fault input indicates a fault condition.
            pub const FLVL_1: u16 = 0b0001;
        }
    }
}

/// Fault Status Register
pub mod FSTS0 {

    /// Fault Flags
    pub mod FFLAG {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No fault on the FAULTx pin.
            pub const FFLAG_0: u16 = 0b0000;

            /// 0b0001: Fault on the FAULTx pin.
            pub const FFLAG_1: u16 = 0b0001;
        }
    }

    /// Full Cycle
    pub mod FFULL {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM outputs are not re-enabled at the start of a full cycle
            pub const FFULL_0: u16 = 0b0000;

            /// 0b0001: PWM outputs are re-enabled at the start of a full cycle
            pub const FFULL_1: u16 = 0b0001;
        }
    }

    /// Filtered Fault Pins
    pub mod FFPIN {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half Cycle Fault Recovery
    pub mod FHALF {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM outputs are not re-enabled at the start of a half cycle.
            pub const FHALF_0: u16 = 0b0000;

            /// 0b0001: PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0).
            pub const FHALF_1: u16 = 0b0001;
        }
    }
}

/// Fault Filter Register
pub mod FFILT0 {

    /// Fault Filter Period
    pub mod FILT_PER {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fault Filter Count
    pub mod FILT_CNT {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fault Glitch Stretch Enable
    pub mod GSTR {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fault input glitch stretching is disabled.
            pub const GSTR_0: u16 = 0b0;

            /// 0b1: Input fault signals will be stretched to at least 2 IPBus clock cycles.
            pub const GSTR_1: u16 = 0b1;
        }
    }
}

/// Fault Test Register
pub mod FTST0 {

    /// Fault Test
    pub mod FTEST {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No fault
            pub const FTEST_0: u16 = 0b0;

            /// 0b1: Cause a simulated fault
            pub const FTEST_1: u16 = 0b1;
        }
    }
}

/// Fault Control 2 Register
pub mod FCTRL20 {

    /// No Combinational Path From Fault Input To PWM Output
    pub mod NOCOMB {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs.
            pub const NOCOMB_0: u16 = 0b0000;

            /// 0b0001: The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs.
            pub const NOCOMB_1: u16 = 0b0001;
        }
    }
}

/// Counter Register
pub mod SMCNT_0 {

    /// Counter Register Bits
    pub mod CNT {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Initial Count Register
pub mod SMINIT_0 {

    /// Initial Count Register Bits
    pub mod INIT {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Control 2 Register
pub mod SMCTRL2_0 {

    /// Clock Source Select
    pub mod CLK_SEL {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The IPBus clock is used as the clock for the local prescaler and counter.
            pub const CLK_SEL_0: u16 = 0b00;

            /// 0b01: EXT_CLK is used as the clock for the local prescaler and counter.
            pub const CLK_SEL_1: u16 = 0b01;

            /// 0b10: Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0.
            pub const CLK_SEL_2: u16 = 0b10;
        }
    }

    /// Reload Source Select
    pub mod RELOAD_SEL {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The local RELOAD signal is used to reload registers.
            pub const RELOAD_SEL_0: u16 = 0b0;

            /// 0b1: The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0.
            pub const RELOAD_SEL_1: u16 = 0b1;
        }
    }

    /// This read/write bit determines the source of the FORCE OUTPUT signal for this submodule.
    pub mod FORCE_SEL {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: The local force signal, CTRL2\[FORCE\], from this submodule is used to force updates.
            pub const FORCE_SEL_0: u16 = 0b000;

            /// 0b001: The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0.
            pub const FORCE_SEL_1: u16 = 0b001;

            /// 0b010: The local reload signal from this submodule is used to force updates without regard to the state of LDOK.
            pub const FORCE_SEL_2: u16 = 0b010;

            /// 0b011: The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0.
            pub const FORCE_SEL_3: u16 = 0b011;

            /// 0b100: The local sync signal from this submodule is used to force updates.
            pub const FORCE_SEL_4: u16 = 0b100;

            /// 0b101: The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0.
            pub const FORCE_SEL_5: u16 = 0b101;

            /// 0b110: The external force signal, EXT_FORCE, from outside the PWM module causes updates.
            pub const FORCE_SEL_6: u16 = 0b110;

            /// 0b111: The external sync signal, EXT_SYNC, from outside the PWM module causes updates.
            pub const FORCE_SEL_7: u16 = 0b111;
        }
    }

    /// Force Initialization
    pub mod FORCE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FRCEN
    pub mod FRCEN {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Initialization from a FORCE_OUT is disabled.
            pub const FRCEN_0: u16 = 0b0;

            /// 0b1: Initialization from a FORCE_OUT is enabled.
            pub const FRCEN_1: u16 = 0b1;
        }
    }

    /// Initialization Control Select
    pub mod INIT_SEL {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Local sync (PWM_X) causes initialization.
            pub const INIT_SEL_0: u16 = 0b00;

            /// 0b01: Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs.
            pub const INIT_SEL_1: u16 = 0b01;

            /// 0b10: Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0.
            pub const INIT_SEL_2: u16 = 0b10;

            /// 0b11: EXT_SYNC causes initialization.
            pub const INIT_SEL_3: u16 = 0b11;
        }
    }

    /// PWM_X Initial Value
    pub mod PWMX_INIT {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM45 Initial Value
    pub mod PWM45_INIT {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM23 Initial Value
    pub mod PWM23_INIT {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Independent or Complementary Pair Operation
    pub mod INDEP {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PWM_A and PWM_B form a complementary PWM pair.
            pub const INDEP_0: u16 = 0b0;

            /// 0b1: PWM_A and PWM_B outputs are independent PWMs.
            pub const INDEP_1: u16 = 0b1;
        }
    }

    /// WAIT Enable
    pub mod WAITEN {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Debug Enable
    pub mod DBGEN {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Control Register
pub mod SMCTRL_0 {

    /// Double Switching Enable
    pub mod DBLEN {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Double switching disabled.
            pub const DBLEN_0: u16 = 0b0;

            /// 0b1: Double switching enabled.
            pub const DBLEN_1: u16 = 0b1;
        }
    }

    /// PWMX Double Switching Enable
    pub mod DBLX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PWMX double pulse disabled.
            pub const DBLX_0: u16 = 0b0;

            /// 0b1: PWMX double pulse enabled.
            pub const DBLX_1: u16 = 0b1;
        }
    }

    /// Load Mode Select
    pub mod LDMOD {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\[LDOK\] is set.
            pub const LDMOD_0: u16 = 0b0;

            /// 0b1: Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\[LDOK\] being set. In this case it is not necessary to set CTRL\[FULL\] or CTRL\[HALF\].
            pub const LDMOD_1: u16 = 0b1;
        }
    }

    /// Split the DBLPWM signal to PWMA and PWMB
    pub mod SPLIT {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DBLPWM is not split. PWMA and PWMB each have double pulses.
            pub const SPLIT_0: u16 = 0b0;

            /// 0b1: DBLPWM is split to PWMA and PWMB.
            pub const SPLIT_1: u16 = 0b1;
        }
    }

    /// Prescaler
    pub mod PRSC {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: PWM clock frequency = fclk
            pub const PRSC_0: u16 = 0b000;

            /// 0b001: PWM clock frequency = fclk/2
            pub const PRSC_1: u16 = 0b001;

            /// 0b010: PWM clock frequency = fclk/4
            pub const PRSC_2: u16 = 0b010;

            /// 0b011: PWM clock frequency = fclk/8
            pub const PRSC_3: u16 = 0b011;

            /// 0b100: PWM clock frequency = fclk/16
            pub const PRSC_4: u16 = 0b100;

            /// 0b101: PWM clock frequency = fclk/32
            pub const PRSC_5: u16 = 0b101;

            /// 0b110: PWM clock frequency = fclk/64
            pub const PRSC_6: u16 = 0b110;

            /// 0b111: PWM clock frequency = fclk/128
            pub const PRSC_7: u16 = 0b111;
        }
    }

    /// Compare Mode
    pub mod COMPMODE {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The VAL* registers and the PWM counter are compared using an "equal to" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period.
            pub const COMPMODE_0: u16 = 0b0;

            /// 0b1: The VAL* registers and the PWM counter are compared using an "equal to or greater than" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value.
            pub const COMPMODE_1: u16 = 0b1;
        }
    }

    /// Deadtime
    pub mod DT {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Full Cycle Reload
    pub mod FULL {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Full-cycle reloads disabled.
            pub const FULL_0: u16 = 0b0;

            /// 0b1: Full-cycle reloads enabled.
            pub const FULL_1: u16 = 0b1;
        }
    }

    /// Half Cycle Reload
    pub mod HALF {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Half-cycle reloads disabled.
            pub const HALF_0: u16 = 0b0;

            /// 0b1: Half-cycle reloads enabled.
            pub const HALF_1: u16 = 0b1;
        }
    }

    /// Load Frequency
    pub mod LDFQ {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Every PWM opportunity
            pub const LDFQ_0: u16 = 0b0000;

            /// 0b0001: Every 2 PWM opportunities
            pub const LDFQ_1: u16 = 0b0001;

            /// 0b0010: Every 3 PWM opportunities
            pub const LDFQ_2: u16 = 0b0010;

            /// 0b0011: Every 4 PWM opportunities
            pub const LDFQ_3: u16 = 0b0011;

            /// 0b0100: Every 5 PWM opportunities
            pub const LDFQ_4: u16 = 0b0100;

            /// 0b0101: Every 6 PWM opportunities
            pub const LDFQ_5: u16 = 0b0101;

            /// 0b0110: Every 7 PWM opportunities
            pub const LDFQ_6: u16 = 0b0110;

            /// 0b0111: Every 8 PWM opportunities
            pub const LDFQ_7: u16 = 0b0111;

            /// 0b1000: Every 9 PWM opportunities
            pub const LDFQ_8: u16 = 0b1000;

            /// 0b1001: Every 10 PWM opportunities
            pub const LDFQ_9: u16 = 0b1001;

            /// 0b1010: Every 11 PWM opportunities
            pub const LDFQ_10: u16 = 0b1010;

            /// 0b1011: Every 12 PWM opportunities
            pub const LDFQ_11: u16 = 0b1011;

            /// 0b1100: Every 13 PWM opportunities
            pub const LDFQ_12: u16 = 0b1100;

            /// 0b1101: Every 14 PWM opportunities
            pub const LDFQ_13: u16 = 0b1101;

            /// 0b1110: Every 15 PWM opportunities
            pub const LDFQ_14: u16 = 0b1110;

            /// 0b1111: Every 16 PWM opportunities
            pub const LDFQ_15: u16 = 0b1111;
        }
    }
}

/// Value Register 0
pub mod SMVAL0_0 {

    /// Value Register 0
    pub mod VAL0 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fractional Value Register 1
pub mod SMFRACVAL1_0 {

    /// Fractional Value 1 Register
    pub mod FRACVAL1 {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 1
pub mod SMVAL1_0 {

    /// Value Register 1
    pub mod VAL1 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fractional Value Register 2
pub mod SMFRACVAL2_0 {

    /// Fractional Value 2
    pub mod FRACVAL2 {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 2
pub mod SMVAL2_0 {

    /// Value Register 2
    pub mod VAL2 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fractional Value Register 3
pub mod SMFRACVAL3_0 {

    /// Fractional Value 3
    pub mod FRACVAL3 {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 3
pub mod SMVAL3_0 {

    /// Value Register 3
    pub mod VAL3 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fractional Value Register 4
pub mod SMFRACVAL4_0 {

    /// Fractional Value 4
    pub mod FRACVAL4 {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 4
pub mod SMVAL4_0 {

    /// Value Register 4
    pub mod VAL4 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fractional Value Register 5
pub mod SMFRACVAL5_0 {

    /// Fractional Value 5
    pub mod FRACVAL5 {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 5
pub mod SMVAL5_0 {

    /// Value Register 5
    pub mod VAL5 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fractional Control Register
pub mod SMFRCTRL_0 {

    /// Fractional Cycle PWM Period Enable
    pub mod FRAC1_EN {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable fractional cycle length for the PWM period.
            pub const FRAC1_EN_0: u16 = 0b0;

            /// 0b1: Enable fractional cycle length for the PWM period.
            pub const FRAC1_EN_1: u16 = 0b1;
        }
    }

    /// Fractional Cycle Placement Enable for PWM_A
    pub mod FRAC23_EN {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable fractional cycle placement for PWM_A.
            pub const FRAC23_EN_0: u16 = 0b0;

            /// 0b1: Enable fractional cycle placement for PWM_A.
            pub const FRAC23_EN_1: u16 = 0b1;
        }
    }

    /// Fractional Cycle Placement Enable for PWM_B
    pub mod FRAC45_EN {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable fractional cycle placement for PWM_B.
            pub const FRAC45_EN_0: u16 = 0b0;

            /// 0b1: Enable fractional cycle placement for PWM_B.
            pub const FRAC45_EN_1: u16 = 0b1;
        }
    }

    /// Fractional Delay Circuit Power Up
    pub mod FRAC_PU {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Turn off fractional delay logic.
            pub const FRAC_PU_0: u16 = 0b0;

            /// 0b1: Power up fractional delay logic.
            pub const FRAC_PU_1: u16 = 0b1;
        }
    }

    /// Test Status Bit
    pub mod TEST {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Output Control Register
pub mod SMOCTRL_0 {

    /// PWM_X Fault State
    pub mod PWMXFS {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Output is forced to logic 0 state prior to consideration of output polarity control.
            pub const PWMXFS_0: u16 = 0b00;

            /// 0b01: Output is forced to logic 1 state prior to consideration of output polarity control.
            pub const PWMXFS_1: u16 = 0b01;

            /// 0b10: Output is tristated.
            pub const PWMXFS_2: u16 = 0b10;

            /// 0b11: Output is tristated.
            pub const PWMXFS_3: u16 = 0b11;
        }
    }

    /// PWM_B Fault State
    pub mod PWMBFS {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Output is forced to logic 0 state prior to consideration of output polarity control.
            pub const PWMBFS_0: u16 = 0b00;

            /// 0b01: Output is forced to logic 1 state prior to consideration of output polarity control.
            pub const PWMBFS_1: u16 = 0b01;

            /// 0b10: Output is tristated.
            pub const PWMBFS_2: u16 = 0b10;

            /// 0b11: Output is tristated.
            pub const PWMBFS_3: u16 = 0b11;
        }
    }

    /// PWM_A Fault State
    pub mod PWMAFS {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Output is forced to logic 0 state prior to consideration of output polarity control.
            pub const PWMAFS_0: u16 = 0b00;

            /// 0b01: Output is forced to logic 1 state prior to consideration of output polarity control.
            pub const PWMAFS_1: u16 = 0b01;

            /// 0b10: Output is tristated.
            pub const PWMAFS_2: u16 = 0b10;

            /// 0b11: Output is tristated.
            pub const PWMAFS_3: u16 = 0b11;
        }
    }

    /// PWM_X Output Polarity
    pub mod POLX {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PWM_X output not inverted. A high level on the PWM_X pin represents the "on" or "active" state.
            pub const POLX_0: u16 = 0b0;

            /// 0b1: PWM_X output inverted. A low level on the PWM_X pin represents the "on" or "active" state.
            pub const POLX_1: u16 = 0b1;
        }
    }

    /// PWM_B Output Polarity
    pub mod POLB {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PWM_B output not inverted. A high level on the PWM_B pin represents the "on" or "active" state.
            pub const POLB_0: u16 = 0b0;

            /// 0b1: PWM_B output inverted. A low level on the PWM_B pin represents the "on" or "active" state.
            pub const POLB_1: u16 = 0b1;
        }
    }

    /// PWM_A Output Polarity
    pub mod POLA {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PWM_A output not inverted. A high level on the PWM_A pin represents the "on" or "active" state.
            pub const POLA_0: u16 = 0b0;

            /// 0b1: PWM_A output inverted. A low level on the PWM_A pin represents the "on" or "active" state.
            pub const POLA_1: u16 = 0b1;
        }
    }

    /// PWM_X Input
    pub mod PWMX_IN {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_B Input
    pub mod PWMB_IN {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_A Input
    pub mod PWMA_IN {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status Register
pub mod SMSTS_0 {

    /// Compare Flags
    pub mod CMPF {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: No compare event has occurred for a particular VALx value.
            pub const CMPF_0: u16 = 0b000000;

            /// 0b000001: A compare event has occurred for a particular VALx value.
            pub const CMPF_1: u16 = 0b000001;
        }
    }

    /// Capture Flag X0
    pub mod CFX0 {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture Flag X1
    pub mod CFX1 {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture Flag B0
    pub mod CFB0 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture Flag B1
    pub mod CFB1 {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture Flag A0
    pub mod CFA0 {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture Flag A1
    pub mod CFA1 {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reload Flag
    pub mod RF {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No new reload cycle since last STS\[RF\] clearing
            pub const RF_0: u16 = 0b0;

            /// 0b1: New reload cycle since last STS\[RF\] clearing
            pub const RF_1: u16 = 0b1;
        }
    }

    /// Reload Error Flag
    pub mod REF {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No reload error occurred.
            pub const REF_0: u16 = 0b0;

            /// 0b1: Reload signal occurred with non-coherent data and MCTRL\[LDOK\] = 0.
            pub const REF_1: u16 = 0b1;
        }
    }

    /// Registers Updated Flag
    pub mod RUF {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No register update has occurred since last reload.
            pub const RUF_0: u16 = 0b0;

            /// 0b1: At least one of the double buffered registers has been updated since the last reload.
            pub const RUF_1: u16 = 0b1;
        }
    }
}

/// Interrupt Enable Register
pub mod SMINTEN_0 {

    /// Compare Interrupt Enables
    pub mod CMPIE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: The corresponding STS\[CMPF\] bit will not cause an interrupt request.
            pub const CMPIE_0: u16 = 0b000000;

            /// 0b000001: The corresponding STS\[CMPF\] bit will cause an interrupt request.
            pub const CMPIE_1: u16 = 0b000001;
        }
    }

    /// Capture X 0 Interrupt Enable
    pub mod CX0IE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFX0\].
            pub const CX0IE_0: u16 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFX0\].
            pub const CX0IE_1: u16 = 0b1;
        }
    }

    /// Capture X 1 Interrupt Enable
    pub mod CX1IE {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFX1\].
            pub const CX1IE_0: u16 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFX1\].
            pub const CX1IE_1: u16 = 0b1;
        }
    }

    /// Capture B 0 Interrupt Enable
    pub mod CB0IE {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFB0\].
            pub const CB0IE_0: u16 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFB0\].
            pub const CB0IE_1: u16 = 0b1;
        }
    }

    /// Capture B 1 Interrupt Enable
    pub mod CB1IE {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFB1\].
            pub const CB1IE_0: u16 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFB1\].
            pub const CB1IE_1: u16 = 0b1;
        }
    }

    /// Capture A 0 Interrupt Enable
    pub mod CA0IE {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFA0\].
            pub const CA0IE_0: u16 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFA0\].
            pub const CA0IE_1: u16 = 0b1;
        }
    }

    /// Capture A 1 Interrupt Enable
    pub mod CA1IE {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFA1\].
            pub const CA1IE_0: u16 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFA1\].
            pub const CA1IE_1: u16 = 0b1;
        }
    }

    /// Reload Interrupt Enable
    pub mod RIE {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: STS\[RF\] CPU interrupt requests disabled
            pub const RIE_0: u16 = 0b0;

            /// 0b1: STS\[RF\] CPU interrupt requests enabled
            pub const RIE_1: u16 = 0b1;
        }
    }

    /// Reload Error Interrupt Enable
    pub mod REIE {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: STS\[REF\] CPU interrupt requests disabled
            pub const REIE_0: u16 = 0b0;

            /// 0b1: STS\[REF\] CPU interrupt requests enabled
            pub const REIE_1: u16 = 0b1;
        }
    }
}

/// DMA Enable Register
pub mod SMDMAEN_0 {

    /// Capture X0 FIFO DMA Enable
    pub mod CX0DE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture X1 FIFO DMA Enable
    pub mod CX1DE {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture B0 FIFO DMA Enable
    pub mod CB0DE {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture B1 FIFO DMA Enable
    pub mod CB1DE {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture A0 FIFO DMA Enable
    pub mod CA0DE {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture A1 FIFO DMA Enable
    pub mod CA1DE {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture DMA Enable Source Select
    pub mod CAPTDE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Read DMA requests disabled.
            pub const CAPTDE_0: u16 = 0b00;

            /// 0b01: Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\[CA1DE\], DMAEN\[CA0DE\], DMAEN\[CB1DE\], DMAEN\[CB0DE\], DMAEN\[CX1DE\], or DMAEN\[CX0DE\] to also be set in order to determine to which watermark(s) the DMA request is sensitive.
            pub const CAPTDE_1: u16 = 0b01;

            /// 0b10: A local sync (VAL1 matches counter) sets the read DMA request.
            pub const CAPTDE_2: u16 = 0b10;

            /// 0b11: A local reload (STS\[RF\] being set) sets the read DMA request.
            pub const CAPTDE_3: u16 = 0b11;
        }
    }

    /// FIFO Watermark AND Control
    pub mod FAND {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selected FIFO watermarks are OR'ed together.
            pub const FAND_0: u16 = 0b0;

            /// 0b1: Selected FIFO watermarks are AND'ed together.
            pub const FAND_1: u16 = 0b1;
        }
    }

    /// Value Registers DMA Enable
    pub mod VALDE {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA write requests disabled
            pub const VALDE_0: u16 = 0b0;

            /// 0b1: DMA write requests for the VALx and FRACVALx registers enabled
            pub const VALDE_1: u16 = 0b1;
        }
    }
}

/// Output Trigger Control Register
pub mod SMTCTRL_0 {

    /// Output Trigger Enables
    pub mod OUT_TRIG_EN {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: PWM_OUT_TRIGx will not set when the counter value matches the VALx value.
            pub const OUT_TRIG_EN_0: u16 = 0b000000;

            /// 0b000001: PWM_OUT_TRIGx will set when the counter value matches the VALx value.
            pub const OUT_TRIG_EN_1: u16 = 0b000001;
        }
    }

    /// Trigger frequency
    pub mod TRGFRQ {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\[LDFQ\] being non-zero.
            pub const TRGFRQ_0: u16 = 0b0;

            /// 0b1: Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\[LDFQ\] being non-zero.
            pub const TRGFRQ_1: u16 = 0b1;
        }
    }

    /// Output Trigger 1 Source Select
    pub mod PWBOT1 {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port.
            pub const PWBOT1_0: u16 = 0b0;

            /// 0b1: Route the PWMB output to the PWM_OUT_TRIG1 port.
            pub const PWBOT1_1: u16 = 0b1;
        }
    }

    /// Output Trigger 0 Source Select
    pub mod PWAOT0 {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port.
            pub const PWAOT0_0: u16 = 0b0;

            /// 0b1: Route the PWMA output to the PWM_OUT_TRIG0 port.
            pub const PWAOT0_1: u16 = 0b1;
        }
    }
}

/// Fault Disable Mapping Register 0
pub mod SMDISMAP0_0 {

    /// PWM_A Fault Disable Mask 0
    pub mod DIS0A {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_B Fault Disable Mask 0
    pub mod DIS0B {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_X Fault Disable Mask 0
    pub mod DIS0X {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fault Disable Mapping Register 1
pub mod SMDISMAP1_0 {

    /// PWM_A Fault Disable Mask 1
    pub mod DIS1A {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_B Fault Disable Mask 1
    pub mod DIS1B {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_X Fault Disable Mask 1
    pub mod DIS1X {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Deadtime Count Register 0
pub mod SMDTCNT0_0 {

    /// DTCNT0
    pub mod DTCNT0 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Deadtime Count Register 1
pub mod SMDTCNT1_0 {

    /// DTCNT1
    pub mod DTCNT1 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Control A Register
pub mod SMCAPTCTRLA_0 {

    /// Arm A
    pub mod ARMA {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input capture operation is disabled.
            pub const ARMA_0: u16 = 0b0;

            /// 0b1: Input capture operation as specified by CAPTCTRLA\[EDGAx\] is enabled.
            pub const ARMA_1: u16 = 0b1;
        }
    }

    /// One Shot Mode A
    pub mod ONESHOTA {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\[ARMA\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit.
            pub const ONESHOTA_0: u16 = 0b0;

            /// 0b1: One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\[ARMA\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLA\[ARMA\] is cleared. No further captures will be performed until CAPTCTRLA\[ARMA\] is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLA\[ARMA\] is then cleared.
            pub const ONESHOTA_1: u16 = 0b1;
        }
    }

    /// Edge A 0
    pub mod EDGA0 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Disabled
            pub const EDGA0_0: u16 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGA0_1: u16 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGA0_2: u16 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGA0_3: u16 = 0b11;
        }
    }

    /// Edge A 1
    pub mod EDGA1 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Disabled
            pub const EDGA1_0: u16 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGA1_1: u16 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGA1_2: u16 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGA1_3: u16 = 0b11;
        }
    }

    /// Input Select A
    pub mod INP_SELA {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Raw PWM_A input signal selected as source.
            pub const INP_SELA_0: u16 = 0b0;

            /// 0b1: Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLA\[EDGA0\] and CAPTCTRLA\[EDGA1\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRA\[EDGA0\] and/or CAPTCTRLA\[EDGA1\] fields in order to enable one or both of the capture registers.
            pub const INP_SELA_1: u16 = 0b1;
        }
    }

    /// Edge Counter A Enable
    pub mod EDGCNTA_EN {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Edge counter disabled and held in reset
            pub const EDGCNTA_EN_0: u16 = 0b0;

            /// 0b1: Edge counter enabled
            pub const EDGCNTA_EN_1: u16 = 0b1;
        }
    }

    /// Capture A FIFOs Water Mark
    pub mod CFAWM {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture A0 FIFO Word Count
    pub mod CA0CNT {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture A1 FIFO Word Count
    pub mod CA1CNT {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Compare A Register
pub mod SMCAPTCOMPA_0 {

    /// Edge Compare A
    pub mod EDGCMPA {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Edge Counter A
    pub mod EDGCNTA {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Control B Register
pub mod SMCAPTCTRLB_0 {

    /// Arm B
    pub mod ARMB {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input capture operation is disabled.
            pub const ARMB_0: u16 = 0b0;

            /// 0b1: Input capture operation as specified by CAPTCTRLB\[EDGBx\] is enabled.
            pub const ARMB_1: u16 = 0b1;
        }
    }

    /// One Shot Mode B
    pub mod ONESHOTB {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\[ARMB\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit.
            pub const ONESHOTB_0: u16 = 0b0;

            /// 0b1: One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\[ARMB\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLB\[ARMB\] is cleared. No further captures will be performed until CAPTCTRLB\[ARMB\] is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLB\[ARMB\] is then cleared.
            pub const ONESHOTB_1: u16 = 0b1;
        }
    }

    /// Edge B 0
    pub mod EDGB0 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Disabled
            pub const EDGB0_0: u16 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGB0_1: u16 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGB0_2: u16 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGB0_3: u16 = 0b11;
        }
    }

    /// Edge B 1
    pub mod EDGB1 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Disabled
            pub const EDGB1_0: u16 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGB1_1: u16 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGB1_2: u16 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGB1_3: u16 = 0b11;
        }
    }

    /// Input Select B
    pub mod INP_SELB {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Raw PWM_B input signal selected as source.
            pub const INP_SELB_0: u16 = 0b0;

            /// 0b1: Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLB\[EDGB0\] and CAPTCTRLB\[EDGB1\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRB\[EDGB0\] and/or CAPTCTRLB\[EDGB1\] fields in order to enable one or both of the capture registers.
            pub const INP_SELB_1: u16 = 0b1;
        }
    }

    /// Edge Counter B Enable
    pub mod EDGCNTB_EN {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Edge counter disabled and held in reset
            pub const EDGCNTB_EN_0: u16 = 0b0;

            /// 0b1: Edge counter enabled
            pub const EDGCNTB_EN_1: u16 = 0b1;
        }
    }

    /// Capture B FIFOs Water Mark
    pub mod CFBWM {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture B0 FIFO Word Count
    pub mod CB0CNT {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture B1 FIFO Word Count
    pub mod CB1CNT {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Compare B Register
pub mod SMCAPTCOMPB_0 {

    /// Edge Compare B
    pub mod EDGCMPB {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Edge Counter B
    pub mod EDGCNTB {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Control X Register
pub mod SMCAPTCTRLX_0 {

    /// Arm X
    pub mod ARMX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input capture operation is disabled.
            pub const ARMX_0: u16 = 0b0;

            /// 0b1: Input capture operation as specified by CAPTCTRLX\[EDGXx\] is enabled.
            pub const ARMX_1: u16 = 0b1;
        }
    }

    /// One Shot Mode Aux
    pub mod ONESHOTX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit.
            pub const ONESHOTX_0: u16 = 0b0;

            /// 0b1: One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and the ARMX bit is cleared. No further captures will be performed until the ARMX bit is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and the ARMX bit is then cleared.
            pub const ONESHOTX_1: u16 = 0b1;
        }
    }

    /// Edge X 0
    pub mod EDGX0 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Disabled
            pub const EDGX0_0: u16 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGX0_1: u16 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGX0_2: u16 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGX0_3: u16 = 0b11;
        }
    }

    /// Edge X 1
    pub mod EDGX1 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Disabled
            pub const EDGX1_0: u16 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGX1_1: u16 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGX1_2: u16 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGX1_3: u16 = 0b11;
        }
    }

    /// Input Select X
    pub mod INP_SELX {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Raw PWM_X input signal selected as source.
            pub const INP_SELX_0: u16 = 0b0;

            /// 0b1: Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLX\[EDGX0\] and CAPTCTRLX\[EDGX1\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRX\[EDGX0\] and/or CAPTCTRLX\[EDGX1\] fields in order to enable one or both of the capture registers.
            pub const INP_SELX_1: u16 = 0b1;
        }
    }

    /// Edge Counter X Enable
    pub mod EDGCNTX_EN {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Edge counter disabled and held in reset
            pub const EDGCNTX_EN_0: u16 = 0b0;

            /// 0b1: Edge counter enabled
            pub const EDGCNTX_EN_1: u16 = 0b1;
        }
    }

    /// Capture X FIFOs Water Mark
    pub mod CFXWM {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture X0 FIFO Word Count
    pub mod CX0CNT {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture X1 FIFO Word Count
    pub mod CX1CNT {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Compare X Register
pub mod SMCAPTCOMPX_0 {

    /// Edge Compare X
    pub mod EDGCMPX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Edge Counter X
    pub mod EDGCNTX {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 0 Register
pub mod SMCVAL0_0 {

    /// CAPTVAL0
    pub mod CAPTVAL0 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 0 Cycle Register
pub mod SMCVAL0CYC_0 {

    /// CVAL0CYC
    pub mod CVAL0CYC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 1 Register
pub mod SMCVAL1_0 {

    /// CAPTVAL1
    pub mod CAPTVAL1 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 1 Cycle Register
pub mod SMCVAL1CYC_0 {

    /// CVAL1CYC
    pub mod CVAL1CYC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 2 Register
pub mod SMCVAL2_0 {

    /// CAPTVAL2
    pub mod CAPTVAL2 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 2 Cycle Register
pub mod SMCVAL2CYC_0 {

    /// CVAL2CYC
    pub mod CVAL2CYC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 3 Register
pub mod SMCVAL3_0 {

    /// CAPTVAL3
    pub mod CAPTVAL3 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 3 Cycle Register
pub mod SMCVAL3CYC_0 {

    /// CVAL3CYC
    pub mod CVAL3CYC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 4 Register
pub mod SMCVAL4_0 {

    /// CAPTVAL4
    pub mod CAPTVAL4 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 4 Cycle Register
pub mod SMCVAL4CYC_0 {

    /// CVAL4CYC
    pub mod CVAL4CYC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 5 Register
pub mod SMCVAL5_0 {

    /// CAPTVAL5
    pub mod CAPTVAL5 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 5 Cycle Register
pub mod SMCVAL5CYC_0 {

    /// CVAL5CYC
    pub mod CVAL5CYC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Counter Register
pub mod SMCNT_1 {
    pub use super::SMCNT_0::CNT;
}

/// Initial Count Register
pub mod SMINIT_1 {
    pub use super::SMINIT_0::INIT;
}

/// Control 2 Register
pub mod SMCTRL2_1 {
    pub use super::SMCTRL2_0::CLK_SEL;
    pub use super::SMCTRL2_0::DBGEN;
    pub use super::SMCTRL2_0::FORCE;
    pub use super::SMCTRL2_0::FORCE_SEL;
    pub use super::SMCTRL2_0::FRCEN;
    pub use super::SMCTRL2_0::INDEP;
    pub use super::SMCTRL2_0::INIT_SEL;
    pub use super::SMCTRL2_0::PWM23_INIT;
    pub use super::SMCTRL2_0::PWM45_INIT;
    pub use super::SMCTRL2_0::PWMX_INIT;
    pub use super::SMCTRL2_0::RELOAD_SEL;
    pub use super::SMCTRL2_0::WAITEN;
}

/// Control Register
pub mod SMCTRL_1 {
    pub use super::SMCTRL_0::COMPMODE;
    pub use super::SMCTRL_0::DBLEN;
    pub use super::SMCTRL_0::DBLX;
    pub use super::SMCTRL_0::DT;
    pub use super::SMCTRL_0::FULL;
    pub use super::SMCTRL_0::HALF;
    pub use super::SMCTRL_0::LDFQ;
    pub use super::SMCTRL_0::LDMOD;
    pub use super::SMCTRL_0::PRSC;
    pub use super::SMCTRL_0::SPLIT;
}

/// Value Register 0
pub mod SMVAL0_1 {
    pub use super::SMVAL0_0::VAL0;
}

/// Fractional Value Register 1
pub mod SMFRACVAL1_1 {
    pub use super::SMFRACVAL1_0::FRACVAL1;
}

/// Value Register 1
pub mod SMVAL1_1 {
    pub use super::SMVAL1_0::VAL1;
}

/// Fractional Value Register 2
pub mod SMFRACVAL2_1 {
    pub use super::SMFRACVAL2_0::FRACVAL2;
}

/// Value Register 2
pub mod SMVAL2_1 {
    pub use super::SMVAL2_0::VAL2;
}

/// Fractional Value Register 3
pub mod SMFRACVAL3_1 {
    pub use super::SMFRACVAL3_0::FRACVAL3;
}

/// Value Register 3
pub mod SMVAL3_1 {
    pub use super::SMVAL3_0::VAL3;
}

/// Fractional Value Register 4
pub mod SMFRACVAL4_1 {
    pub use super::SMFRACVAL4_0::FRACVAL4;
}

/// Value Register 4
pub mod SMVAL4_1 {
    pub use super::SMVAL4_0::VAL4;
}

/// Fractional Value Register 5
pub mod SMFRACVAL5_1 {
    pub use super::SMFRACVAL5_0::FRACVAL5;
}

/// Value Register 5
pub mod SMVAL5_1 {
    pub use super::SMVAL5_0::VAL5;
}

/// Fractional Control Register
pub mod SMFRCTRL_1 {
    pub use super::SMFRCTRL_0::FRAC1_EN;
    pub use super::SMFRCTRL_0::FRAC23_EN;
    pub use super::SMFRCTRL_0::FRAC45_EN;
    pub use super::SMFRCTRL_0::FRAC_PU;
    pub use super::SMFRCTRL_0::TEST;
}

/// Output Control Register
pub mod SMOCTRL_1 {
    pub use super::SMOCTRL_0::POLA;
    pub use super::SMOCTRL_0::POLB;
    pub use super::SMOCTRL_0::POLX;
    pub use super::SMOCTRL_0::PWMAFS;
    pub use super::SMOCTRL_0::PWMA_IN;
    pub use super::SMOCTRL_0::PWMBFS;
    pub use super::SMOCTRL_0::PWMB_IN;
    pub use super::SMOCTRL_0::PWMXFS;
    pub use super::SMOCTRL_0::PWMX_IN;
}

/// Status Register
pub mod SMSTS_1 {
    pub use super::SMSTS_0::CFA0;
    pub use super::SMSTS_0::CFA1;
    pub use super::SMSTS_0::CFB0;
    pub use super::SMSTS_0::CFB1;
    pub use super::SMSTS_0::CFX0;
    pub use super::SMSTS_0::CFX1;
    pub use super::SMSTS_0::CMPF;
    pub use super::SMSTS_0::REF;
    pub use super::SMSTS_0::RF;
    pub use super::SMSTS_0::RUF;
}

/// Interrupt Enable Register
pub mod SMINTEN_1 {
    pub use super::SMINTEN_0::CA0IE;
    pub use super::SMINTEN_0::CA1IE;
    pub use super::SMINTEN_0::CB0IE;
    pub use super::SMINTEN_0::CB1IE;
    pub use super::SMINTEN_0::CMPIE;
    pub use super::SMINTEN_0::CX0IE;
    pub use super::SMINTEN_0::CX1IE;
    pub use super::SMINTEN_0::REIE;
    pub use super::SMINTEN_0::RIE;
}

/// DMA Enable Register
pub mod SMDMAEN_1 {
    pub use super::SMDMAEN_0::CA0DE;
    pub use super::SMDMAEN_0::CA1DE;
    pub use super::SMDMAEN_0::CAPTDE;
    pub use super::SMDMAEN_0::CB0DE;
    pub use super::SMDMAEN_0::CB1DE;
    pub use super::SMDMAEN_0::CX0DE;
    pub use super::SMDMAEN_0::CX1DE;
    pub use super::SMDMAEN_0::FAND;
    pub use super::SMDMAEN_0::VALDE;
}

/// Output Trigger Control Register
pub mod SMTCTRL_1 {
    pub use super::SMTCTRL_0::OUT_TRIG_EN;
    pub use super::SMTCTRL_0::PWAOT0;
    pub use super::SMTCTRL_0::PWBOT1;
    pub use super::SMTCTRL_0::TRGFRQ;
}

/// Fault Disable Mapping Register 0
pub mod SMDISMAP0_1 {
    pub use super::SMDISMAP0_0::DIS0A;
    pub use super::SMDISMAP0_0::DIS0B;
    pub use super::SMDISMAP0_0::DIS0X;
}

/// Fault Disable Mapping Register 1
pub mod SMDISMAP1_1 {
    pub use super::SMDISMAP1_0::DIS1A;
    pub use super::SMDISMAP1_0::DIS1B;
    pub use super::SMDISMAP1_0::DIS1X;
}

/// Deadtime Count Register 0
pub mod SMDTCNT0_1 {
    pub use super::SMDTCNT0_0::DTCNT0;
}

/// Deadtime Count Register 1
pub mod SMDTCNT1_1 {
    pub use super::SMDTCNT1_0::DTCNT1;
}

/// Capture Control A Register
pub mod SMCAPTCTRLA_1 {
    pub use super::SMCAPTCTRLA_0::ARMA;
    pub use super::SMCAPTCTRLA_0::CA0CNT;
    pub use super::SMCAPTCTRLA_0::CA1CNT;
    pub use super::SMCAPTCTRLA_0::CFAWM;
    pub use super::SMCAPTCTRLA_0::EDGA0;
    pub use super::SMCAPTCTRLA_0::EDGA1;
    pub use super::SMCAPTCTRLA_0::EDGCNTA_EN;
    pub use super::SMCAPTCTRLA_0::INP_SELA;
    pub use super::SMCAPTCTRLA_0::ONESHOTA;
}

/// Capture Compare A Register
pub mod SMCAPTCOMPA_1 {
    pub use super::SMCAPTCOMPA_0::EDGCMPA;
    pub use super::SMCAPTCOMPA_0::EDGCNTA;
}

/// Capture Control B Register
pub mod SMCAPTCTRLB_1 {
    pub use super::SMCAPTCTRLB_0::ARMB;
    pub use super::SMCAPTCTRLB_0::CB0CNT;
    pub use super::SMCAPTCTRLB_0::CB1CNT;
    pub use super::SMCAPTCTRLB_0::CFBWM;
    pub use super::SMCAPTCTRLB_0::EDGB0;
    pub use super::SMCAPTCTRLB_0::EDGB1;
    pub use super::SMCAPTCTRLB_0::EDGCNTB_EN;
    pub use super::SMCAPTCTRLB_0::INP_SELB;
    pub use super::SMCAPTCTRLB_0::ONESHOTB;
}

/// Capture Compare B Register
pub mod SMCAPTCOMPB_1 {
    pub use super::SMCAPTCOMPB_0::EDGCMPB;
    pub use super::SMCAPTCOMPB_0::EDGCNTB;
}

/// Capture Control X Register
pub mod SMCAPTCTRLX_1 {
    pub use super::SMCAPTCTRLX_0::ARMX;
    pub use super::SMCAPTCTRLX_0::CFXWM;
    pub use super::SMCAPTCTRLX_0::CX0CNT;
    pub use super::SMCAPTCTRLX_0::CX1CNT;
    pub use super::SMCAPTCTRLX_0::EDGCNTX_EN;
    pub use super::SMCAPTCTRLX_0::EDGX0;
    pub use super::SMCAPTCTRLX_0::EDGX1;
    pub use super::SMCAPTCTRLX_0::INP_SELX;
    pub use super::SMCAPTCTRLX_0::ONESHOTX;
}

/// Capture Compare X Register
pub mod SMCAPTCOMPX_1 {
    pub use super::SMCAPTCOMPX_0::EDGCMPX;
    pub use super::SMCAPTCOMPX_0::EDGCNTX;
}

/// Capture Value 0 Register
pub mod SMCVAL0_1 {
    pub use super::SMCVAL0_0::CAPTVAL0;
}

/// Capture Value 0 Cycle Register
pub mod SMCVAL0CYC_1 {
    pub use super::SMCVAL0CYC_0::CVAL0CYC;
}

/// Capture Value 1 Register
pub mod SMCVAL1_1 {
    pub use super::SMCVAL1_0::CAPTVAL1;
}

/// Capture Value 1 Cycle Register
pub mod SMCVAL1CYC_1 {
    pub use super::SMCVAL1CYC_0::CVAL1CYC;
}

/// Capture Value 2 Register
pub mod SMCVAL2_1 {
    pub use super::SMCVAL2_0::CAPTVAL2;
}

/// Capture Value 2 Cycle Register
pub mod SMCVAL2CYC_1 {
    pub use super::SMCVAL2CYC_0::CVAL2CYC;
}

/// Capture Value 3 Register
pub mod SMCVAL3_1 {
    pub use super::SMCVAL3_0::CAPTVAL3;
}

/// Capture Value 3 Cycle Register
pub mod SMCVAL3CYC_1 {
    pub use super::SMCVAL3CYC_0::CVAL3CYC;
}

/// Capture Value 4 Register
pub mod SMCVAL4_1 {
    pub use super::SMCVAL4_0::CAPTVAL4;
}

/// Capture Value 4 Cycle Register
pub mod SMCVAL4CYC_1 {
    pub use super::SMCVAL4CYC_0::CVAL4CYC;
}

/// Capture Value 5 Register
pub mod SMCVAL5_1 {
    pub use super::SMCVAL5_0::CAPTVAL5;
}

/// Capture Value 5 Cycle Register
pub mod SMCVAL5CYC_1 {
    pub use super::SMCVAL5CYC_0::CVAL5CYC;
}

/// Counter Register
pub mod SMCNT_2 {
    pub use super::SMCNT_0::CNT;
}

/// Initial Count Register
pub mod SMINIT_2 {
    pub use super::SMINIT_0::INIT;
}

/// Control 2 Register
pub mod SMCTRL2_2 {
    pub use super::SMCTRL2_0::CLK_SEL;
    pub use super::SMCTRL2_0::DBGEN;
    pub use super::SMCTRL2_0::FORCE;
    pub use super::SMCTRL2_0::FORCE_SEL;
    pub use super::SMCTRL2_0::FRCEN;
    pub use super::SMCTRL2_0::INDEP;
    pub use super::SMCTRL2_0::INIT_SEL;
    pub use super::SMCTRL2_0::PWM23_INIT;
    pub use super::SMCTRL2_0::PWM45_INIT;
    pub use super::SMCTRL2_0::PWMX_INIT;
    pub use super::SMCTRL2_0::RELOAD_SEL;
    pub use super::SMCTRL2_0::WAITEN;
}

/// Control Register
pub mod SMCTRL_2 {
    pub use super::SMCTRL_0::COMPMODE;
    pub use super::SMCTRL_0::DBLEN;
    pub use super::SMCTRL_0::DBLX;
    pub use super::SMCTRL_0::DT;
    pub use super::SMCTRL_0::FULL;
    pub use super::SMCTRL_0::HALF;
    pub use super::SMCTRL_0::LDFQ;
    pub use super::SMCTRL_0::LDMOD;
    pub use super::SMCTRL_0::PRSC;
    pub use super::SMCTRL_0::SPLIT;
}

/// Value Register 0
pub mod SMVAL0_2 {
    pub use super::SMVAL0_0::VAL0;
}

/// Fractional Value Register 1
pub mod SMFRACVAL1_2 {
    pub use super::SMFRACVAL1_0::FRACVAL1;
}

/// Value Register 1
pub mod SMVAL1_2 {
    pub use super::SMVAL1_0::VAL1;
}

/// Fractional Value Register 2
pub mod SMFRACVAL2_2 {
    pub use super::SMFRACVAL2_0::FRACVAL2;
}

/// Value Register 2
pub mod SMVAL2_2 {
    pub use super::SMVAL2_0::VAL2;
}

/// Fractional Value Register 3
pub mod SMFRACVAL3_2 {
    pub use super::SMFRACVAL3_0::FRACVAL3;
}

/// Value Register 3
pub mod SMVAL3_2 {
    pub use super::SMVAL3_0::VAL3;
}

/// Fractional Value Register 4
pub mod SMFRACVAL4_2 {
    pub use super::SMFRACVAL4_0::FRACVAL4;
}

/// Value Register 4
pub mod SMVAL4_2 {
    pub use super::SMVAL4_0::VAL4;
}

/// Fractional Value Register 5
pub mod SMFRACVAL5_2 {
    pub use super::SMFRACVAL5_0::FRACVAL5;
}

/// Value Register 5
pub mod SMVAL5_2 {
    pub use super::SMVAL5_0::VAL5;
}

/// Fractional Control Register
pub mod SMFRCTRL_2 {
    pub use super::SMFRCTRL_0::FRAC1_EN;
    pub use super::SMFRCTRL_0::FRAC23_EN;
    pub use super::SMFRCTRL_0::FRAC45_EN;
    pub use super::SMFRCTRL_0::FRAC_PU;
    pub use super::SMFRCTRL_0::TEST;
}

/// Output Control Register
pub mod SMOCTRL_2 {
    pub use super::SMOCTRL_0::POLA;
    pub use super::SMOCTRL_0::POLB;
    pub use super::SMOCTRL_0::POLX;
    pub use super::SMOCTRL_0::PWMAFS;
    pub use super::SMOCTRL_0::PWMA_IN;
    pub use super::SMOCTRL_0::PWMBFS;
    pub use super::SMOCTRL_0::PWMB_IN;
    pub use super::SMOCTRL_0::PWMXFS;
    pub use super::SMOCTRL_0::PWMX_IN;
}

/// Status Register
pub mod SMSTS_2 {
    pub use super::SMSTS_0::CFA0;
    pub use super::SMSTS_0::CFA1;
    pub use super::SMSTS_0::CFB0;
    pub use super::SMSTS_0::CFB1;
    pub use super::SMSTS_0::CFX0;
    pub use super::SMSTS_0::CFX1;
    pub use super::SMSTS_0::CMPF;
    pub use super::SMSTS_0::REF;
    pub use super::SMSTS_0::RF;
    pub use super::SMSTS_0::RUF;
}

/// Interrupt Enable Register
pub mod SMINTEN_2 {
    pub use super::SMINTEN_0::CA0IE;
    pub use super::SMINTEN_0::CA1IE;
    pub use super::SMINTEN_0::CB0IE;
    pub use super::SMINTEN_0::CB1IE;
    pub use super::SMINTEN_0::CMPIE;
    pub use super::SMINTEN_0::CX0IE;
    pub use super::SMINTEN_0::CX1IE;
    pub use super::SMINTEN_0::REIE;
    pub use super::SMINTEN_0::RIE;
}

/// DMA Enable Register
pub mod SMDMAEN_2 {
    pub use super::SMDMAEN_0::CA0DE;
    pub use super::SMDMAEN_0::CA1DE;
    pub use super::SMDMAEN_0::CAPTDE;
    pub use super::SMDMAEN_0::CB0DE;
    pub use super::SMDMAEN_0::CB1DE;
    pub use super::SMDMAEN_0::CX0DE;
    pub use super::SMDMAEN_0::CX1DE;
    pub use super::SMDMAEN_0::FAND;
    pub use super::SMDMAEN_0::VALDE;
}

/// Output Trigger Control Register
pub mod SMTCTRL_2 {
    pub use super::SMTCTRL_0::OUT_TRIG_EN;
    pub use super::SMTCTRL_0::PWAOT0;
    pub use super::SMTCTRL_0::PWBOT1;
    pub use super::SMTCTRL_0::TRGFRQ;
}

/// Fault Disable Mapping Register 0
pub mod SMDISMAP0_2 {
    pub use super::SMDISMAP0_0::DIS0A;
    pub use super::SMDISMAP0_0::DIS0B;
    pub use super::SMDISMAP0_0::DIS0X;
}

/// Fault Disable Mapping Register 1
pub mod SMDISMAP1_2 {
    pub use super::SMDISMAP1_0::DIS1A;
    pub use super::SMDISMAP1_0::DIS1B;
    pub use super::SMDISMAP1_0::DIS1X;
}

/// Deadtime Count Register 0
pub mod SMDTCNT0_2 {
    pub use super::SMDTCNT0_0::DTCNT0;
}

/// Deadtime Count Register 1
pub mod SMDTCNT1_2 {
    pub use super::SMDTCNT1_0::DTCNT1;
}

/// Capture Control A Register
pub mod SMCAPTCTRLA_2 {
    pub use super::SMCAPTCTRLA_0::ARMA;
    pub use super::SMCAPTCTRLA_0::CA0CNT;
    pub use super::SMCAPTCTRLA_0::CA1CNT;
    pub use super::SMCAPTCTRLA_0::CFAWM;
    pub use super::SMCAPTCTRLA_0::EDGA0;
    pub use super::SMCAPTCTRLA_0::EDGA1;
    pub use super::SMCAPTCTRLA_0::EDGCNTA_EN;
    pub use super::SMCAPTCTRLA_0::INP_SELA;
    pub use super::SMCAPTCTRLA_0::ONESHOTA;
}

/// Capture Compare A Register
pub mod SMCAPTCOMPA_2 {
    pub use super::SMCAPTCOMPA_0::EDGCMPA;
    pub use super::SMCAPTCOMPA_0::EDGCNTA;
}

/// Capture Control B Register
pub mod SMCAPTCTRLB_2 {
    pub use super::SMCAPTCTRLB_0::ARMB;
    pub use super::SMCAPTCTRLB_0::CB0CNT;
    pub use super::SMCAPTCTRLB_0::CB1CNT;
    pub use super::SMCAPTCTRLB_0::CFBWM;
    pub use super::SMCAPTCTRLB_0::EDGB0;
    pub use super::SMCAPTCTRLB_0::EDGB1;
    pub use super::SMCAPTCTRLB_0::EDGCNTB_EN;
    pub use super::SMCAPTCTRLB_0::INP_SELB;
    pub use super::SMCAPTCTRLB_0::ONESHOTB;
}

/// Capture Compare B Register
pub mod SMCAPTCOMPB_2 {
    pub use super::SMCAPTCOMPB_0::EDGCMPB;
    pub use super::SMCAPTCOMPB_0::EDGCNTB;
}

/// Capture Control X Register
pub mod SMCAPTCTRLX_2 {
    pub use super::SMCAPTCTRLX_0::ARMX;
    pub use super::SMCAPTCTRLX_0::CFXWM;
    pub use super::SMCAPTCTRLX_0::CX0CNT;
    pub use super::SMCAPTCTRLX_0::CX1CNT;
    pub use super::SMCAPTCTRLX_0::EDGCNTX_EN;
    pub use super::SMCAPTCTRLX_0::EDGX0;
    pub use super::SMCAPTCTRLX_0::EDGX1;
    pub use super::SMCAPTCTRLX_0::INP_SELX;
    pub use super::SMCAPTCTRLX_0::ONESHOTX;
}

/// Capture Compare X Register
pub mod SMCAPTCOMPX_2 {
    pub use super::SMCAPTCOMPX_0::EDGCMPX;
    pub use super::SMCAPTCOMPX_0::EDGCNTX;
}

/// Capture Value 0 Register
pub mod SMCVAL0_2 {
    pub use super::SMCVAL0_0::CAPTVAL0;
}

/// Capture Value 0 Cycle Register
pub mod SMCVAL0CYC_2 {
    pub use super::SMCVAL0CYC_0::CVAL0CYC;
}

/// Capture Value 1 Register
pub mod SMCVAL1_2 {
    pub use super::SMCVAL1_0::CAPTVAL1;
}

/// Capture Value 1 Cycle Register
pub mod SMCVAL1CYC_2 {
    pub use super::SMCVAL1CYC_0::CVAL1CYC;
}

/// Capture Value 2 Register
pub mod SMCVAL2_2 {
    pub use super::SMCVAL2_0::CAPTVAL2;
}

/// Capture Value 2 Cycle Register
pub mod SMCVAL2CYC_2 {
    pub use super::SMCVAL2CYC_0::CVAL2CYC;
}

/// Capture Value 3 Register
pub mod SMCVAL3_2 {
    pub use super::SMCVAL3_0::CAPTVAL3;
}

/// Capture Value 3 Cycle Register
pub mod SMCVAL3CYC_2 {
    pub use super::SMCVAL3CYC_0::CVAL3CYC;
}

/// Capture Value 4 Register
pub mod SMCVAL4_2 {
    pub use super::SMCVAL4_0::CAPTVAL4;
}

/// Capture Value 4 Cycle Register
pub mod SMCVAL4CYC_2 {
    pub use super::SMCVAL4CYC_0::CVAL4CYC;
}

/// Capture Value 5 Register
pub mod SMCVAL5_2 {
    pub use super::SMCVAL5_0::CAPTVAL5;
}

/// Capture Value 5 Cycle Register
pub mod SMCVAL5CYC_2 {
    pub use super::SMCVAL5CYC_0::CVAL5CYC;
}

/// Counter Register
pub mod SMCNT_3 {
    pub use super::SMCNT_0::CNT;
}

/// Initial Count Register
pub mod SMINIT_3 {
    pub use super::SMINIT_0::INIT;
}

/// Control 2 Register
pub mod SMCTRL2_3 {
    pub use super::SMCTRL2_0::CLK_SEL;
    pub use super::SMCTRL2_0::DBGEN;
    pub use super::SMCTRL2_0::FORCE;
    pub use super::SMCTRL2_0::FORCE_SEL;
    pub use super::SMCTRL2_0::FRCEN;
    pub use super::SMCTRL2_0::INDEP;
    pub use super::SMCTRL2_0::INIT_SEL;
    pub use super::SMCTRL2_0::PWM23_INIT;
    pub use super::SMCTRL2_0::PWM45_INIT;
    pub use super::SMCTRL2_0::PWMX_INIT;
    pub use super::SMCTRL2_0::RELOAD_SEL;
    pub use super::SMCTRL2_0::WAITEN;
}

/// Control Register
pub mod SMCTRL_3 {
    pub use super::SMCTRL_0::COMPMODE;
    pub use super::SMCTRL_0::DBLEN;
    pub use super::SMCTRL_0::DBLX;
    pub use super::SMCTRL_0::DT;
    pub use super::SMCTRL_0::FULL;
    pub use super::SMCTRL_0::HALF;
    pub use super::SMCTRL_0::LDFQ;
    pub use super::SMCTRL_0::LDMOD;
    pub use super::SMCTRL_0::PRSC;
    pub use super::SMCTRL_0::SPLIT;
}

/// Value Register 0
pub mod SMVAL0_3 {
    pub use super::SMVAL0_0::VAL0;
}

/// Fractional Value Register 1
pub mod SMFRACVAL1_3 {
    pub use super::SMFRACVAL1_0::FRACVAL1;
}

/// Value Register 1
pub mod SMVAL1_3 {
    pub use super::SMVAL1_0::VAL1;
}

/// Fractional Value Register 2
pub mod SMFRACVAL2_3 {
    pub use super::SMFRACVAL2_0::FRACVAL2;
}

/// Value Register 2
pub mod SMVAL2_3 {
    pub use super::SMVAL2_0::VAL2;
}

/// Fractional Value Register 3
pub mod SMFRACVAL3_3 {
    pub use super::SMFRACVAL3_0::FRACVAL3;
}

/// Value Register 3
pub mod SMVAL3_3 {
    pub use super::SMVAL3_0::VAL3;
}

/// Fractional Value Register 4
pub mod SMFRACVAL4_3 {
    pub use super::SMFRACVAL4_0::FRACVAL4;
}

/// Value Register 4
pub mod SMVAL4_3 {
    pub use super::SMVAL4_0::VAL4;
}

/// Fractional Value Register 5
pub mod SMFRACVAL5_3 {
    pub use super::SMFRACVAL5_0::FRACVAL5;
}

/// Value Register 5
pub mod SMVAL5_3 {
    pub use super::SMVAL5_0::VAL5;
}

/// Fractional Control Register
pub mod SMFRCTRL_3 {
    pub use super::SMFRCTRL_0::FRAC1_EN;
    pub use super::SMFRCTRL_0::FRAC23_EN;
    pub use super::SMFRCTRL_0::FRAC45_EN;
    pub use super::SMFRCTRL_0::FRAC_PU;
    pub use super::SMFRCTRL_0::TEST;
}

/// Output Control Register
pub mod SMOCTRL_3 {
    pub use super::SMOCTRL_0::POLA;
    pub use super::SMOCTRL_0::POLB;
    pub use super::SMOCTRL_0::POLX;
    pub use super::SMOCTRL_0::PWMAFS;
    pub use super::SMOCTRL_0::PWMA_IN;
    pub use super::SMOCTRL_0::PWMBFS;
    pub use super::SMOCTRL_0::PWMB_IN;
    pub use super::SMOCTRL_0::PWMXFS;
    pub use super::SMOCTRL_0::PWMX_IN;
}

/// Status Register
pub mod SMSTS_3 {
    pub use super::SMSTS_0::CFA0;
    pub use super::SMSTS_0::CFA1;
    pub use super::SMSTS_0::CFB0;
    pub use super::SMSTS_0::CFB1;
    pub use super::SMSTS_0::CFX0;
    pub use super::SMSTS_0::CFX1;
    pub use super::SMSTS_0::CMPF;
    pub use super::SMSTS_0::REF;
    pub use super::SMSTS_0::RF;
    pub use super::SMSTS_0::RUF;
}

/// Interrupt Enable Register
pub mod SMINTEN_3 {
    pub use super::SMINTEN_0::CA0IE;
    pub use super::SMINTEN_0::CA1IE;
    pub use super::SMINTEN_0::CB0IE;
    pub use super::SMINTEN_0::CB1IE;
    pub use super::SMINTEN_0::CMPIE;
    pub use super::SMINTEN_0::CX0IE;
    pub use super::SMINTEN_0::CX1IE;
    pub use super::SMINTEN_0::REIE;
    pub use super::SMINTEN_0::RIE;
}

/// DMA Enable Register
pub mod SMDMAEN_3 {
    pub use super::SMDMAEN_0::CA0DE;
    pub use super::SMDMAEN_0::CA1DE;
    pub use super::SMDMAEN_0::CAPTDE;
    pub use super::SMDMAEN_0::CB0DE;
    pub use super::SMDMAEN_0::CB1DE;
    pub use super::SMDMAEN_0::CX0DE;
    pub use super::SMDMAEN_0::CX1DE;
    pub use super::SMDMAEN_0::FAND;
    pub use super::SMDMAEN_0::VALDE;
}

/// Output Trigger Control Register
pub mod SMTCTRL_3 {
    pub use super::SMTCTRL_0::OUT_TRIG_EN;
    pub use super::SMTCTRL_0::PWAOT0;
    pub use super::SMTCTRL_0::PWBOT1;
    pub use super::SMTCTRL_0::TRGFRQ;
}

/// Fault Disable Mapping Register 0
pub mod SMDISMAP0_3 {
    pub use super::SMDISMAP0_0::DIS0A;
    pub use super::SMDISMAP0_0::DIS0B;
    pub use super::SMDISMAP0_0::DIS0X;
}

/// Fault Disable Mapping Register 1
pub mod SMDISMAP1_3 {
    pub use super::SMDISMAP1_0::DIS1A;
    pub use super::SMDISMAP1_0::DIS1B;
    pub use super::SMDISMAP1_0::DIS1X;
}

/// Deadtime Count Register 0
pub mod SMDTCNT0_3 {
    pub use super::SMDTCNT0_0::DTCNT0;
}

/// Deadtime Count Register 1
pub mod SMDTCNT1_3 {
    pub use super::SMDTCNT1_0::DTCNT1;
}

/// Capture Control A Register
pub mod SMCAPTCTRLA_3 {
    pub use super::SMCAPTCTRLA_0::ARMA;
    pub use super::SMCAPTCTRLA_0::CA0CNT;
    pub use super::SMCAPTCTRLA_0::CA1CNT;
    pub use super::SMCAPTCTRLA_0::CFAWM;
    pub use super::SMCAPTCTRLA_0::EDGA0;
    pub use super::SMCAPTCTRLA_0::EDGA1;
    pub use super::SMCAPTCTRLA_0::EDGCNTA_EN;
    pub use super::SMCAPTCTRLA_0::INP_SELA;
    pub use super::SMCAPTCTRLA_0::ONESHOTA;
}

/// Capture Compare A Register
pub mod SMCAPTCOMPA_3 {
    pub use super::SMCAPTCOMPA_0::EDGCMPA;
    pub use super::SMCAPTCOMPA_0::EDGCNTA;
}

/// Capture Control B Register
pub mod SMCAPTCTRLB_3 {
    pub use super::SMCAPTCTRLB_0::ARMB;
    pub use super::SMCAPTCTRLB_0::CB0CNT;
    pub use super::SMCAPTCTRLB_0::CB1CNT;
    pub use super::SMCAPTCTRLB_0::CFBWM;
    pub use super::SMCAPTCTRLB_0::EDGB0;
    pub use super::SMCAPTCTRLB_0::EDGB1;
    pub use super::SMCAPTCTRLB_0::EDGCNTB_EN;
    pub use super::SMCAPTCTRLB_0::INP_SELB;
    pub use super::SMCAPTCTRLB_0::ONESHOTB;
}

/// Capture Compare B Register
pub mod SMCAPTCOMPB_3 {
    pub use super::SMCAPTCOMPB_0::EDGCMPB;
    pub use super::SMCAPTCOMPB_0::EDGCNTB;
}

/// Capture Control X Register
pub mod SMCAPTCTRLX_3 {
    pub use super::SMCAPTCTRLX_0::ARMX;
    pub use super::SMCAPTCTRLX_0::CFXWM;
    pub use super::SMCAPTCTRLX_0::CX0CNT;
    pub use super::SMCAPTCTRLX_0::CX1CNT;
    pub use super::SMCAPTCTRLX_0::EDGCNTX_EN;
    pub use super::SMCAPTCTRLX_0::EDGX0;
    pub use super::SMCAPTCTRLX_0::EDGX1;
    pub use super::SMCAPTCTRLX_0::INP_SELX;
    pub use super::SMCAPTCTRLX_0::ONESHOTX;
}

/// Capture Compare X Register
pub mod SMCAPTCOMPX_3 {
    pub use super::SMCAPTCOMPX_0::EDGCMPX;
    pub use super::SMCAPTCOMPX_0::EDGCNTX;
}

/// Capture Value 0 Register
pub mod SMCVAL0_3 {
    pub use super::SMCVAL0_0::CAPTVAL0;
}

/// Capture Value 0 Cycle Register
pub mod SMCVAL0CYC_3 {
    pub use super::SMCVAL0CYC_0::CVAL0CYC;
}

/// Capture Value 1 Register
pub mod SMCVAL1_3 {
    pub use super::SMCVAL1_0::CAPTVAL1;
}

/// Capture Value 1 Cycle Register
pub mod SMCVAL1CYC_3 {
    pub use super::SMCVAL1CYC_0::CVAL1CYC;
}

/// Capture Value 2 Register
pub mod SMCVAL2_3 {
    pub use super::SMCVAL2_0::CAPTVAL2;
}

/// Capture Value 2 Cycle Register
pub mod SMCVAL2CYC_3 {
    pub use super::SMCVAL2CYC_0::CVAL2CYC;
}

/// Capture Value 3 Register
pub mod SMCVAL3_3 {
    pub use super::SMCVAL3_0::CAPTVAL3;
}

/// Capture Value 3 Cycle Register
pub mod SMCVAL3CYC_3 {
    pub use super::SMCVAL3CYC_0::CVAL3CYC;
}

/// Capture Value 4 Register
pub mod SMCVAL4_3 {
    pub use super::SMCVAL4_0::CAPTVAL4;
}

/// Capture Value 4 Cycle Register
pub mod SMCVAL4CYC_3 {
    pub use super::SMCVAL4CYC_0::CVAL4CYC;
}

/// Capture Value 5 Register
pub mod SMCVAL5_3 {
    pub use super::SMCVAL5_0::CAPTVAL5;
}

/// Capture Value 5 Cycle Register
pub mod SMCVAL5CYC_3 {
    pub use super::SMCVAL5CYC_0::CVAL5CYC;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Counter Register
    pub SMCNT_0: RORegister<u16>,

    /// Initial Count Register
    pub SMINIT_0: RWRegister<u16>,

    /// Control 2 Register
    pub SMCTRL2_0: RWRegister<u16>,

    /// Control Register
    pub SMCTRL_0: RWRegister<u16>,

    _reserved1: [u16; 1],

    /// Value Register 0
    pub SMVAL0_0: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SMFRACVAL1_0: RWRegister<u16>,

    /// Value Register 1
    pub SMVAL1_0: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SMFRACVAL2_0: RWRegister<u16>,

    /// Value Register 2
    pub SMVAL2_0: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SMFRACVAL3_0: RWRegister<u16>,

    /// Value Register 3
    pub SMVAL3_0: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SMFRACVAL4_0: RWRegister<u16>,

    /// Value Register 4
    pub SMVAL4_0: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SMFRACVAL5_0: RWRegister<u16>,

    /// Value Register 5
    pub SMVAL5_0: RWRegister<u16>,

    /// Fractional Control Register
    pub SMFRCTRL_0: RWRegister<u16>,

    /// Output Control Register
    pub SMOCTRL_0: RWRegister<u16>,

    /// Status Register
    pub SMSTS_0: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SMINTEN_0: RWRegister<u16>,

    /// DMA Enable Register
    pub SMDMAEN_0: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SMTCTRL_0: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SMDISMAP0_0: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SMDISMAP1_0: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SMDTCNT0_0: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SMDTCNT1_0: RWRegister<u16>,

    /// Capture Control A Register
    pub SMCAPTCTRLA_0: RWRegister<u16>,

    /// Capture Compare A Register
    pub SMCAPTCOMPA_0: RWRegister<u16>,

    /// Capture Control B Register
    pub SMCAPTCTRLB_0: RWRegister<u16>,

    /// Capture Compare B Register
    pub SMCAPTCOMPB_0: RWRegister<u16>,

    /// Capture Control X Register
    pub SMCAPTCTRLX_0: RWRegister<u16>,

    /// Capture Compare X Register
    pub SMCAPTCOMPX_0: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SMCVAL0_0: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SMCVAL0CYC_0: RORegister<u16>,

    /// Capture Value 1 Register
    pub SMCVAL1_0: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SMCVAL1CYC_0: RORegister<u16>,

    /// Capture Value 2 Register
    pub SMCVAL2_0: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SMCVAL2CYC_0: RORegister<u16>,

    /// Capture Value 3 Register
    pub SMCVAL3_0: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SMCVAL3CYC_0: RORegister<u16>,

    /// Capture Value 4 Register
    pub SMCVAL4_0: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SMCVAL4CYC_0: RORegister<u16>,

    /// Capture Value 5 Register
    pub SMCVAL5_0: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SMCVAL5CYC_0: RORegister<u16>,

    _reserved2: [u32; 2],

    /// Counter Register
    pub SMCNT_1: RORegister<u16>,

    /// Initial Count Register
    pub SMINIT_1: RWRegister<u16>,

    /// Control 2 Register
    pub SMCTRL2_1: RWRegister<u16>,

    /// Control Register
    pub SMCTRL_1: RWRegister<u16>,

    _reserved3: [u16; 1],

    /// Value Register 0
    pub SMVAL0_1: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SMFRACVAL1_1: RWRegister<u16>,

    /// Value Register 1
    pub SMVAL1_1: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SMFRACVAL2_1: RWRegister<u16>,

    /// Value Register 2
    pub SMVAL2_1: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SMFRACVAL3_1: RWRegister<u16>,

    /// Value Register 3
    pub SMVAL3_1: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SMFRACVAL4_1: RWRegister<u16>,

    /// Value Register 4
    pub SMVAL4_1: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SMFRACVAL5_1: RWRegister<u16>,

    /// Value Register 5
    pub SMVAL5_1: RWRegister<u16>,

    /// Fractional Control Register
    pub SMFRCTRL_1: RWRegister<u16>,

    /// Output Control Register
    pub SMOCTRL_1: RWRegister<u16>,

    /// Status Register
    pub SMSTS_1: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SMINTEN_1: RWRegister<u16>,

    /// DMA Enable Register
    pub SMDMAEN_1: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SMTCTRL_1: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SMDISMAP0_1: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SMDISMAP1_1: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SMDTCNT0_1: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SMDTCNT1_1: RWRegister<u16>,

    /// Capture Control A Register
    pub SMCAPTCTRLA_1: RWRegister<u16>,

    /// Capture Compare A Register
    pub SMCAPTCOMPA_1: RWRegister<u16>,

    /// Capture Control B Register
    pub SMCAPTCTRLB_1: RWRegister<u16>,

    /// Capture Compare B Register
    pub SMCAPTCOMPB_1: RWRegister<u16>,

    /// Capture Control X Register
    pub SMCAPTCTRLX_1: RWRegister<u16>,

    /// Capture Compare X Register
    pub SMCAPTCOMPX_1: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SMCVAL0_1: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SMCVAL0CYC_1: RORegister<u16>,

    /// Capture Value 1 Register
    pub SMCVAL1_1: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SMCVAL1CYC_1: RORegister<u16>,

    /// Capture Value 2 Register
    pub SMCVAL2_1: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SMCVAL2CYC_1: RORegister<u16>,

    /// Capture Value 3 Register
    pub SMCVAL3_1: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SMCVAL3CYC_1: RORegister<u16>,

    /// Capture Value 4 Register
    pub SMCVAL4_1: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SMCVAL4CYC_1: RORegister<u16>,

    /// Capture Value 5 Register
    pub SMCVAL5_1: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SMCVAL5CYC_1: RORegister<u16>,

    _reserved4: [u32; 2],

    /// Counter Register
    pub SMCNT_2: RORegister<u16>,

    /// Initial Count Register
    pub SMINIT_2: RWRegister<u16>,

    /// Control 2 Register
    pub SMCTRL2_2: RWRegister<u16>,

    /// Control Register
    pub SMCTRL_2: RWRegister<u16>,

    _reserved5: [u16; 1],

    /// Value Register 0
    pub SMVAL0_2: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SMFRACVAL1_2: RWRegister<u16>,

    /// Value Register 1
    pub SMVAL1_2: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SMFRACVAL2_2: RWRegister<u16>,

    /// Value Register 2
    pub SMVAL2_2: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SMFRACVAL3_2: RWRegister<u16>,

    /// Value Register 3
    pub SMVAL3_2: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SMFRACVAL4_2: RWRegister<u16>,

    /// Value Register 4
    pub SMVAL4_2: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SMFRACVAL5_2: RWRegister<u16>,

    /// Value Register 5
    pub SMVAL5_2: RWRegister<u16>,

    /// Fractional Control Register
    pub SMFRCTRL_2: RWRegister<u16>,

    /// Output Control Register
    pub SMOCTRL_2: RWRegister<u16>,

    /// Status Register
    pub SMSTS_2: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SMINTEN_2: RWRegister<u16>,

    /// DMA Enable Register
    pub SMDMAEN_2: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SMTCTRL_2: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SMDISMAP0_2: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SMDISMAP1_2: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SMDTCNT0_2: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SMDTCNT1_2: RWRegister<u16>,

    /// Capture Control A Register
    pub SMCAPTCTRLA_2: RWRegister<u16>,

    /// Capture Compare A Register
    pub SMCAPTCOMPA_2: RWRegister<u16>,

    /// Capture Control B Register
    pub SMCAPTCTRLB_2: RWRegister<u16>,

    /// Capture Compare B Register
    pub SMCAPTCOMPB_2: RWRegister<u16>,

    /// Capture Control X Register
    pub SMCAPTCTRLX_2: RWRegister<u16>,

    /// Capture Compare X Register
    pub SMCAPTCOMPX_2: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SMCVAL0_2: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SMCVAL0CYC_2: RORegister<u16>,

    /// Capture Value 1 Register
    pub SMCVAL1_2: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SMCVAL1CYC_2: RORegister<u16>,

    /// Capture Value 2 Register
    pub SMCVAL2_2: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SMCVAL2CYC_2: RORegister<u16>,

    /// Capture Value 3 Register
    pub SMCVAL3_2: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SMCVAL3CYC_2: RORegister<u16>,

    /// Capture Value 4 Register
    pub SMCVAL4_2: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SMCVAL4CYC_2: RORegister<u16>,

    /// Capture Value 5 Register
    pub SMCVAL5_2: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SMCVAL5CYC_2: RORegister<u16>,

    _reserved6: [u32; 2],

    /// Counter Register
    pub SMCNT_3: RORegister<u16>,

    /// Initial Count Register
    pub SMINIT_3: RWRegister<u16>,

    /// Control 2 Register
    pub SMCTRL2_3: RWRegister<u16>,

    /// Control Register
    pub SMCTRL_3: RWRegister<u16>,

    _reserved7: [u16; 1],

    /// Value Register 0
    pub SMVAL0_3: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SMFRACVAL1_3: RWRegister<u16>,

    /// Value Register 1
    pub SMVAL1_3: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SMFRACVAL2_3: RWRegister<u16>,

    /// Value Register 2
    pub SMVAL2_3: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SMFRACVAL3_3: RWRegister<u16>,

    /// Value Register 3
    pub SMVAL3_3: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SMFRACVAL4_3: RWRegister<u16>,

    /// Value Register 4
    pub SMVAL4_3: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SMFRACVAL5_3: RWRegister<u16>,

    /// Value Register 5
    pub SMVAL5_3: RWRegister<u16>,

    /// Fractional Control Register
    pub SMFRCTRL_3: RWRegister<u16>,

    /// Output Control Register
    pub SMOCTRL_3: RWRegister<u16>,

    /// Status Register
    pub SMSTS_3: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SMINTEN_3: RWRegister<u16>,

    /// DMA Enable Register
    pub SMDMAEN_3: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SMTCTRL_3: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SMDISMAP0_3: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SMDISMAP1_3: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SMDTCNT0_3: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SMDTCNT1_3: RWRegister<u16>,

    /// Capture Control A Register
    pub SMCAPTCTRLA_3: RWRegister<u16>,

    /// Capture Compare A Register
    pub SMCAPTCOMPA_3: RWRegister<u16>,

    /// Capture Control B Register
    pub SMCAPTCTRLB_3: RWRegister<u16>,

    /// Capture Compare B Register
    pub SMCAPTCOMPB_3: RWRegister<u16>,

    /// Capture Control X Register
    pub SMCAPTCTRLX_3: RWRegister<u16>,

    /// Capture Compare X Register
    pub SMCAPTCOMPX_3: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SMCVAL0_3: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SMCVAL0CYC_3: RORegister<u16>,

    /// Capture Value 1 Register
    pub SMCVAL1_3: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SMCVAL1CYC_3: RORegister<u16>,

    /// Capture Value 2 Register
    pub SMCVAL2_3: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SMCVAL2CYC_3: RORegister<u16>,

    /// Capture Value 3 Register
    pub SMCVAL3_3: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SMCVAL3CYC_3: RORegister<u16>,

    /// Capture Value 4 Register
    pub SMCVAL4_3: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SMCVAL4CYC_3: RORegister<u16>,

    /// Capture Value 5 Register
    pub SMCVAL5_3: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SMCVAL5CYC_3: RORegister<u16>,

    _reserved8: [u32; 2],

    /// Output Enable Register
    pub OUTEN: RWRegister<u16>,

    /// Mask Register
    pub MASK: RWRegister<u16>,

    /// Software Controlled Output Register
    pub SWCOUT: RWRegister<u16>,

    /// PWM Source Select Register
    pub DTSRCSEL: RWRegister<u16>,

    /// Master Control Register
    pub MCTRL: RWRegister<u16>,

    /// Master Control 2 Register
    pub MCTRL2: RWRegister<u16>,

    /// Fault Control Register
    pub FCTRL0: RWRegister<u16>,

    /// Fault Status Register
    pub FSTS0: RWRegister<u16>,

    /// Fault Filter Register
    pub FFILT0: RWRegister<u16>,

    /// Fault Test Register
    pub FTST0: RWRegister<u16>,

    /// Fault Control 2 Register
    pub FCTRL20: RWRegister<u16>,
}
pub struct ResetValues {
    pub SMCNT_0: u16,
    pub SMINIT_0: u16,
    pub SMCTRL2_0: u16,
    pub SMCTRL_0: u16,
    pub SMVAL0_0: u16,
    pub SMFRACVAL1_0: u16,
    pub SMVAL1_0: u16,
    pub SMFRACVAL2_0: u16,
    pub SMVAL2_0: u16,
    pub SMFRACVAL3_0: u16,
    pub SMVAL3_0: u16,
    pub SMFRACVAL4_0: u16,
    pub SMVAL4_0: u16,
    pub SMFRACVAL5_0: u16,
    pub SMVAL5_0: u16,
    pub SMFRCTRL_0: u16,
    pub SMOCTRL_0: u16,
    pub SMSTS_0: u16,
    pub SMINTEN_0: u16,
    pub SMDMAEN_0: u16,
    pub SMTCTRL_0: u16,
    pub SMDISMAP0_0: u16,
    pub SMDISMAP1_0: u16,
    pub SMDTCNT0_0: u16,
    pub SMDTCNT1_0: u16,
    pub SMCAPTCTRLA_0: u16,
    pub SMCAPTCOMPA_0: u16,
    pub SMCAPTCTRLB_0: u16,
    pub SMCAPTCOMPB_0: u16,
    pub SMCAPTCTRLX_0: u16,
    pub SMCAPTCOMPX_0: u16,
    pub SMCVAL0_0: u16,
    pub SMCVAL0CYC_0: u16,
    pub SMCVAL1_0: u16,
    pub SMCVAL1CYC_0: u16,
    pub SMCVAL2_0: u16,
    pub SMCVAL2CYC_0: u16,
    pub SMCVAL3_0: u16,
    pub SMCVAL3CYC_0: u16,
    pub SMCVAL4_0: u16,
    pub SMCVAL4CYC_0: u16,
    pub SMCVAL5_0: u16,
    pub SMCVAL5CYC_0: u16,
    pub SMCNT_1: u16,
    pub SMINIT_1: u16,
    pub SMCTRL2_1: u16,
    pub SMCTRL_1: u16,
    pub SMVAL0_1: u16,
    pub SMFRACVAL1_1: u16,
    pub SMVAL1_1: u16,
    pub SMFRACVAL2_1: u16,
    pub SMVAL2_1: u16,
    pub SMFRACVAL3_1: u16,
    pub SMVAL3_1: u16,
    pub SMFRACVAL4_1: u16,
    pub SMVAL4_1: u16,
    pub SMFRACVAL5_1: u16,
    pub SMVAL5_1: u16,
    pub SMFRCTRL_1: u16,
    pub SMOCTRL_1: u16,
    pub SMSTS_1: u16,
    pub SMINTEN_1: u16,
    pub SMDMAEN_1: u16,
    pub SMTCTRL_1: u16,
    pub SMDISMAP0_1: u16,
    pub SMDISMAP1_1: u16,
    pub SMDTCNT0_1: u16,
    pub SMDTCNT1_1: u16,
    pub SMCAPTCTRLA_1: u16,
    pub SMCAPTCOMPA_1: u16,
    pub SMCAPTCTRLB_1: u16,
    pub SMCAPTCOMPB_1: u16,
    pub SMCAPTCTRLX_1: u16,
    pub SMCAPTCOMPX_1: u16,
    pub SMCVAL0_1: u16,
    pub SMCVAL0CYC_1: u16,
    pub SMCVAL1_1: u16,
    pub SMCVAL1CYC_1: u16,
    pub SMCVAL2_1: u16,
    pub SMCVAL2CYC_1: u16,
    pub SMCVAL3_1: u16,
    pub SMCVAL3CYC_1: u16,
    pub SMCVAL4_1: u16,
    pub SMCVAL4CYC_1: u16,
    pub SMCVAL5_1: u16,
    pub SMCVAL5CYC_1: u16,
    pub SMCNT_2: u16,
    pub SMINIT_2: u16,
    pub SMCTRL2_2: u16,
    pub SMCTRL_2: u16,
    pub SMVAL0_2: u16,
    pub SMFRACVAL1_2: u16,
    pub SMVAL1_2: u16,
    pub SMFRACVAL2_2: u16,
    pub SMVAL2_2: u16,
    pub SMFRACVAL3_2: u16,
    pub SMVAL3_2: u16,
    pub SMFRACVAL4_2: u16,
    pub SMVAL4_2: u16,
    pub SMFRACVAL5_2: u16,
    pub SMVAL5_2: u16,
    pub SMFRCTRL_2: u16,
    pub SMOCTRL_2: u16,
    pub SMSTS_2: u16,
    pub SMINTEN_2: u16,
    pub SMDMAEN_2: u16,
    pub SMTCTRL_2: u16,
    pub SMDISMAP0_2: u16,
    pub SMDISMAP1_2: u16,
    pub SMDTCNT0_2: u16,
    pub SMDTCNT1_2: u16,
    pub SMCAPTCTRLA_2: u16,
    pub SMCAPTCOMPA_2: u16,
    pub SMCAPTCTRLB_2: u16,
    pub SMCAPTCOMPB_2: u16,
    pub SMCAPTCTRLX_2: u16,
    pub SMCAPTCOMPX_2: u16,
    pub SMCVAL0_2: u16,
    pub SMCVAL0CYC_2: u16,
    pub SMCVAL1_2: u16,
    pub SMCVAL1CYC_2: u16,
    pub SMCVAL2_2: u16,
    pub SMCVAL2CYC_2: u16,
    pub SMCVAL3_2: u16,
    pub SMCVAL3CYC_2: u16,
    pub SMCVAL4_2: u16,
    pub SMCVAL4CYC_2: u16,
    pub SMCVAL5_2: u16,
    pub SMCVAL5CYC_2: u16,
    pub SMCNT_3: u16,
    pub SMINIT_3: u16,
    pub SMCTRL2_3: u16,
    pub SMCTRL_3: u16,
    pub SMVAL0_3: u16,
    pub SMFRACVAL1_3: u16,
    pub SMVAL1_3: u16,
    pub SMFRACVAL2_3: u16,
    pub SMVAL2_3: u16,
    pub SMFRACVAL3_3: u16,
    pub SMVAL3_3: u16,
    pub SMFRACVAL4_3: u16,
    pub SMVAL4_3: u16,
    pub SMFRACVAL5_3: u16,
    pub SMVAL5_3: u16,
    pub SMFRCTRL_3: u16,
    pub SMOCTRL_3: u16,
    pub SMSTS_3: u16,
    pub SMINTEN_3: u16,
    pub SMDMAEN_3: u16,
    pub SMTCTRL_3: u16,
    pub SMDISMAP0_3: u16,
    pub SMDISMAP1_3: u16,
    pub SMDTCNT0_3: u16,
    pub SMDTCNT1_3: u16,
    pub SMCAPTCTRLA_3: u16,
    pub SMCAPTCOMPA_3: u16,
    pub SMCAPTCTRLB_3: u16,
    pub SMCAPTCOMPB_3: u16,
    pub SMCAPTCTRLX_3: u16,
    pub SMCAPTCOMPX_3: u16,
    pub SMCVAL0_3: u16,
    pub SMCVAL0CYC_3: u16,
    pub SMCVAL1_3: u16,
    pub SMCVAL1CYC_3: u16,
    pub SMCVAL2_3: u16,
    pub SMCVAL2CYC_3: u16,
    pub SMCVAL3_3: u16,
    pub SMCVAL3CYC_3: u16,
    pub SMCVAL4_3: u16,
    pub SMCVAL4CYC_3: u16,
    pub SMCVAL5_3: u16,
    pub SMCVAL5CYC_3: u16,
    pub OUTEN: u16,
    pub MASK: u16,
    pub SWCOUT: u16,
    pub DTSRCSEL: u16,
    pub MCTRL: u16,
    pub MCTRL2: u16,
    pub FCTRL0: u16,
    pub FSTS0: u16,
    pub FFILT0: u16,
    pub FTST0: u16,
    pub FCTRL20: u16,
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
