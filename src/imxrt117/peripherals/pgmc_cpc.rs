#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGMC_CPC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// CPC Authentication Control
pub mod CPC_AUTHEN_CTRL {

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

/// CPC Core Mode
pub mod CPC_CORE_MODE {

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
        }
    }
}

/// CPC core power control
pub mod CPC_CORE_POWER_CTRL {

    /// Power off when domain enters WAIT mode
    pub mod PWR_OFF_AT_WAIT {
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

    /// Power off when domain enters STOP mode
    pub mod PWR_OFF_AT_STOP {
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

    /// Power off when domain enters SUSPEND mode
    pub mod PWR_OFF_AT_SUSPEND {
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

    /// Software isolation on trigger
    pub mod ISO_ON_SOFT {
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

    /// Software power off trigger
    pub mod PSW_OFF_SOFT {
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

    /// Software power on trigger
    pub mod PSW_ON_SOFT {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software isolation off trigger
    pub mod ISO_OFF_SOFT {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CPC flag
pub mod CPC_FLAG {

    /// set to 1 after core power switch off, cleared by writing 1
    pub mod CORE_PDN_FLAG {
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
}

/// CPC Cache Mode
pub mod CPC_CACHE_MODE {

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

            /// 0b10: Controlled by Setpoint
            pub const CTRL_MODE_2: u32 = 0b10;
        }
    }
}

/// CPC cache CPU mode control
pub mod CPC_CACHE_CM_CTRL {

    /// Memory Low Power Level (MLPL) at RUN mode
    pub mod MLPL_AT_RUN {
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

    /// Memory Low Power Level (MLPL) at WAIT mode. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_WAIT {
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

    /// Memory Low Power Level (MLPL) at STOP mode. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_STOP {
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

    /// Memory Low Power Level (MLPL) at SUSPEND mode. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SUSPEND {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Low Power Level (MLPL) software change request, keep 1 until MLPL transition complete
    pub mod MLPL_SOFT {
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
}

/// CPC cache Setpoint control 0
pub mod CPC_CACHE_SP_CTRL_0 {

    /// Memory Low Power Level (MLPL) at Setpoint 0. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP0 {
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

    /// Memory Low Power Level (MLPL) at Setpoint 1. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP1 {
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

    /// Memory Low Power Level (MLPL) at Setpoint 2. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP2 {
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

    /// Memory Low Power Level (MLPL) at Setpoint 3. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Low Power Level (MLPL) at Setpoint 4. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP4 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Low Power Level (MLPL) at Setpoint 5. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Low Power Level (MLPL) at Setpoint 6. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP6 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Low Power Level (MLPL) at Setpoint 7. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP7 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CPC cache Setpoint control 1
pub mod CPC_CACHE_SP_CTRL_1 {

    /// Memory Low Power Level (MLPL) at Setpoint 8. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP8 {
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

    /// Memory Low Power Level (MLPL) at Setpoint 9. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP9 {
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

    /// Memory Low Power Level (MLPL) at Setpoint 10. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP10 {
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

    /// Memory Low Power Level (MLPL) at Setpoint 11. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP11 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Low Power Level (MLPL) at Setpoint 12. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP12 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Low Power Level (MLPL) at Setpoint 13. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP13 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Low Power Level (MLPL) at Setpoint 14. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP14 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Low Power Level (MLPL) at Setpoint 15. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod MLPL_AT_SP15 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CPC local memory Mode
pub mod CPC_LMEM_MODE {
    pub use super::CPC_CACHE_MODE::CTRL_MODE;
}

/// CPC local memory CPU mode control
pub mod CPC_LMEM_CM_CTRL {
    pub use super::CPC_CACHE_CM_CTRL::MLPL_AT_RUN;
    pub use super::CPC_CACHE_CM_CTRL::MLPL_AT_STOP;
    pub use super::CPC_CACHE_CM_CTRL::MLPL_AT_SUSPEND;
    pub use super::CPC_CACHE_CM_CTRL::MLPL_AT_WAIT;
    pub use super::CPC_CACHE_CM_CTRL::MLPL_SOFT;
}

/// CPC local memory Setpoint control 0
pub mod CPC_LMEM_SP_CTRL_0 {
    pub use super::CPC_CACHE_SP_CTRL_0::MLPL_AT_SP0;
    pub use super::CPC_CACHE_SP_CTRL_0::MLPL_AT_SP1;
    pub use super::CPC_CACHE_SP_CTRL_0::MLPL_AT_SP2;
    pub use super::CPC_CACHE_SP_CTRL_0::MLPL_AT_SP3;
    pub use super::CPC_CACHE_SP_CTRL_0::MLPL_AT_SP4;
    pub use super::CPC_CACHE_SP_CTRL_0::MLPL_AT_SP5;
    pub use super::CPC_CACHE_SP_CTRL_0::MLPL_AT_SP6;
    pub use super::CPC_CACHE_SP_CTRL_0::MLPL_AT_SP7;
}

/// CPC local memory Setpoint control 1
pub mod CPC_LMEM_SP_CTRL_1 {
    pub use super::CPC_CACHE_SP_CTRL_1::MLPL_AT_SP10;
    pub use super::CPC_CACHE_SP_CTRL_1::MLPL_AT_SP11;
    pub use super::CPC_CACHE_SP_CTRL_1::MLPL_AT_SP12;
    pub use super::CPC_CACHE_SP_CTRL_1::MLPL_AT_SP13;
    pub use super::CPC_CACHE_SP_CTRL_1::MLPL_AT_SP14;
    pub use super::CPC_CACHE_SP_CTRL_1::MLPL_AT_SP15;
    pub use super::CPC_CACHE_SP_CTRL_1::MLPL_AT_SP8;
    pub use super::CPC_CACHE_SP_CTRL_1::MLPL_AT_SP9;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 1],

    /// CPC Authentication Control
    pub CPC_AUTHEN_CTRL: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// CPC Core Mode
    pub CPC_CORE_MODE: RWRegister<u32>,

    /// CPC core power control
    pub CPC_CORE_POWER_CTRL: RWRegister<u32>,

    _reserved3: [u32; 5],

    /// CPC flag
    pub CPC_FLAG: RWRegister<u32>,

    _reserved4: [u32; 4],

    /// CPC Cache Mode
    pub CPC_CACHE_MODE: RWRegister<u32>,

    /// CPC cache CPU mode control
    pub CPC_CACHE_CM_CTRL: RWRegister<u32>,

    /// CPC cache Setpoint control 0
    pub CPC_CACHE_SP_CTRL_0: RWRegister<u32>,

    /// CPC cache Setpoint control 1
    pub CPC_CACHE_SP_CTRL_1: RWRegister<u32>,

    _reserved5: [u32; 28],

    /// CPC local memory Mode
    pub CPC_LMEM_MODE: RWRegister<u32>,

    /// CPC local memory CPU mode control
    pub CPC_LMEM_CM_CTRL: RWRegister<u32>,

    /// CPC local memory Setpoint control 0
    pub CPC_LMEM_SP_CTRL_0: RWRegister<u32>,

    /// CPC local memory Setpoint control 1
    pub CPC_LMEM_SP_CTRL_1: RWRegister<u32>,
}
pub struct ResetValues {
    pub CPC_AUTHEN_CTRL: u32,
    pub CPC_CORE_MODE: u32,
    pub CPC_CORE_POWER_CTRL: u32,
    pub CPC_FLAG: u32,
    pub CPC_CACHE_MODE: u32,
    pub CPC_CACHE_CM_CTRL: u32,
    pub CPC_CACHE_SP_CTRL_0: u32,
    pub CPC_CACHE_SP_CTRL_1: u32,
    pub CPC_LMEM_MODE: u32,
    pub CPC_LMEM_CM_CTRL: u32,
    pub CPC_LMEM_SP_CTRL_0: u32,
    pub CPC_LMEM_SP_CTRL_1: u32,
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
