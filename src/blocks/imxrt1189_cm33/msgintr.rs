#[doc = "MSGINTR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Message Signaled Interrupt Index Register 0"]
    pub MSIIR0: u32,
    #[doc = "Message Signaled Interrupt Register 0"]
    pub MSIR0: u32,
    #[doc = "Message Signaled Interrupt Index Register 1"]
    pub MSIIR1: u32,
    #[doc = "Message Signaled Interrupt Register 1"]
    pub MSIR1: u32,
    #[doc = "Message Signaled Interrupt Index Register 2"]
    pub MSIIR2: u32,
    #[doc = "Message Signaled Interrupt Register 2"]
    pub MSIR2: u32,
}
#[doc = "Message Signaled Interrupt Index Register 0"]
pub mod MSIIR0 {
    pub use crate::WO as access;
    #[doc = "Interrupt Bit Select"]
    pub mod IBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Message Signaled Interrupt Register 0"]
pub mod MSIR0 {
    pub use crate::RO as access;
    #[doc = "Message sharer n has a pending interrupt."]
    pub mod SHN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Message Signaled Interrupt Index Register 1"]
pub mod MSIIR1 {
    pub use crate::WO as access;
    #[doc = "Interrupt Bit Select"]
    pub mod IBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Message Signaled Interrupt Register 1"]
pub mod MSIR1 {
    pub use crate::RO as access;
    #[doc = "Message sharer n has a pending interrupt."]
    pub mod SHN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Message Signaled Interrupt Index Register 2"]
pub mod MSIIR2 {
    pub use crate::WO as access;
    #[doc = "Interrupt Bit Select"]
    pub mod IBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Message Signaled Interrupt Register 2"]
pub mod MSIR2 {
    pub use crate::RO as access;
    #[doc = "Message sharer n has a pending interrupt."]
    pub mod SHN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
