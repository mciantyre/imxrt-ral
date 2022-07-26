#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGMC_PPC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// PPC Authentication Control
pub mod PPC_AUTHEN_CTRL {

    /// Allow user mode access
    pub mod USER {
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

    /// Allow non-secure mode access
    pub mod NONSECURE {
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

    /// Lock NONSECURE and USER
    pub mod LOCK_SETTING {
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

    /// Domain ID white list
    pub mod WHITE_LIST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// White list lock
    pub mod LOCK_LIST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configuration lock
    pub mod LOCK_CFG {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPC Mode
pub mod PPC_MODE {

    /// Control mode. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod CTRL_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Not affected by any low power mode
            pub const CTRL_MODE_0: u32 = 0b00;

            /// 0b01: Controlled by CPU power mode of the domain
            pub const CTRL_MODE_1: u32 = 0b01;

            /// 0b10: Controlled by Setpoint and system standby
            pub const CTRL_MODE_2: u32 = 0b10;
        }
    }

    /// Domain assignment of the BPC
    pub mod DOMAIN_ASSIGN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Domain 0
            pub const d0: u32 = 0b00;

            /// 0b01: Domain 1
            pub const d1: u32 = 0b01;

            /// 0b10: Domain 2
            pub const d2: u32 = 0b10;

            /// 0b11: Domain 3
            pub const d3: u32 = 0b11;
        }
    }
}

/// PPC standby CPU mode control
pub mod PPC_STBY_CM_CTRL {

    /// PMIC Standby on when domain enters WAIT mode. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod STBY_ON_AT_WAIT {
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

    /// PMIC Standby on when domain enters STOP mode. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod STBY_ON_AT_STOP {
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

    /// PMIC Standby on when domain enters SUSPEND mode. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod STBY_ON_AT_SUSPEND {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software PMIC standby on trigger
    pub mod STBY_ON_SOFT {
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

    /// Software PMIC standby off trigger
    pub mod STBY_OFF_SOFT {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPC standby Setpoint control
pub mod PPC_STBY_SP_CTRL {

    /// PMIC standby on when system enters Setpoint number. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod STBY_ON_AT_SP_ACTIVE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PMIC standby on when system enters Setpoint number and system is in standby mode. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod STBY_ON_AT_SP_SLEEP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
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
    _reserved1: [u32; 1],

    /// PPC Authentication Control
    pub PPC_AUTHEN_CTRL: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// PPC Mode
    pub PPC_MODE: RWRegister<u32>,

    /// PPC standby CPU mode control
    pub PPC_STBY_CM_CTRL: RWRegister<u32>,

    /// PPC standby Setpoint control
    pub PPC_STBY_SP_CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub PPC_AUTHEN_CTRL: u32,
    pub PPC_MODE: u32,
    pub PPC_STBY_CM_CTRL: u32,
    pub PPC_STBY_SP_CTRL: u32,
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
