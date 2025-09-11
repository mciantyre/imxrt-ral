#[doc = "TRDC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "TRDC Register"]
    pub TRDC_CR: u32,
    _reserved0: [u8; 0xec],
    #[doc = "TRDC Hardware Configuration Register 0"]
    pub TRDC_HWCFG0: u32,
    #[doc = "TRDC Hardware Configuration Register 1"]
    pub TRDC_HWCFG1: u32,
    #[doc = "TRDC Hardware Configuration Register 2"]
    pub TRDC_HWCFG2: u32,
    #[doc = "TRDC Hardware Configuration Register 3"]
    pub TRDC_HWCFG3: u32,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG0: u8,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG1: u8,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG2: u8,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG3: u8,
    #[doc = "Domain Assignment Configuration Register"]
    pub DACFG4: u8,
    _reserved1: [u8; 0xbb],
    #[doc = "TRDC IDAU Control Register"]
    pub TRDC_IDAU_CR: u32,
    _reserved2: [u8; 0x1c],
    #[doc = "TRDC FLW Control"]
    pub TRDC_FLW_CTL: u32,
    #[doc = "TRDC FLW Physical Base"]
    pub TRDC_FLW_PBASE: u32,
    #[doc = "TRDC FLW Array Base"]
    pub TRDC_FLW_ABASE: u32,
    #[doc = "TRDC FLW Block Count"]
    pub TRDC_FLW_BCNT: u32,
    _reserved3: [u8; 0x0c],
    #[doc = "TRDC Fault Domain ID"]
    pub TRDC_FDID: u32,
    #[doc = "TRDC Domain Error Location Register"]
    pub TRDC_DERRLOC: [u32; 16usize],
    _reserved4: [u8; 0x05c0],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_0_DFMT1: u32,
    _reserved5: [u8; 0x1c],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_1_DFMT1: u32,
    _reserved6: [u8; 0x1c],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_2_DFMT1: u32,
    _reserved7: [u8; 0x1c],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_3_DFMT1: u32,
    _reserved8: [u8; 0x1c],
    #[doc = "DAC Master Domain Assignment Register"]
    pub MDA_W0_4_DFMT1: u32,
}
#[doc = "TRDC Register"]
pub mod TRDC_CR {
    pub use crate::RW as access;
    #[doc = "Global Valid for Domain Assignment Controllers"]
    pub mod GVLDM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TRDC DACs are disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "TRDC DACs are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Revision Level"]
    pub mod HRL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Global Valid for Memory Block Checkers"]
    pub mod GVLDB {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TRDC MBCs are disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "TRDC MBCs are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Global Valid for Memory Region Checkers"]
    pub mod GVLDR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TRDC MRCs are disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "TRDC MRCs are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock Status"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The CR can be written by any secure privileged write."]
            pub const INVALID: u32 = 0;
            #[doc = "The CR is locked (read-only) until the next reset."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC Hardware Configuration Register 0"]
pub mod TRDC_HWCFG0 {
    pub use crate::RO as access;
    #[doc = "Number of domains"]
    pub mod NDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of bus masters"]
    pub mod NMSTR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of MBCs"]
    pub mod NMBC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of MRCs"]
    pub mod NMRC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Module ID"]
    pub mod MID {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC Hardware Configuration Register 1"]
pub mod TRDC_HWCFG1 {
    pub use crate::RO as access;
    #[doc = "Domain identifier number"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC Hardware Configuration Register 2"]
pub mod TRDC_HWCFG2 {
    pub use crate::RO as access;
    #[doc = "Process identifier present"]
    pub mod PIDPN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC Hardware Configuration Register 3"]
pub mod TRDC_HWCFG3 {
    pub use crate::RO as access;
    #[doc = "Process identifier present"]
    pub mod PIDPN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG0 {
    pub use crate::RO as access;
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG1 {
    pub use crate::RO as access;
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG2 {
    pub use crate::RO as access;
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG3 {
    pub use crate::RO as access;
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Domain Assignment Configuration Register"]
pub mod DACFG4 {
    pub use crate::RO as access;
    #[doc = "Number of master domain assignment registers for bus master m"]
    pub mod NMDAR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-CPU Master"]
    pub mod NCM {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bus master is a processor."]
            pub const CPU: u8 = 0;
            #[doc = "Bus master is a non-processor."]
            pub const NON_CPU: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC IDAU Control Register"]
pub mod TRDC_IDAU_CR {
    pub use crate::RW as access;
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Configure Security Extension"]
    pub mod CFGSECEXT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Armv8M Security Extension is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Armv8-M Security Extension is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Secure Memory Protection Unit Disabled"]
    pub mod MPUSDIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Secure MPU is enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Secure MPU is disabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NonSecure Memory Protection Unit Disabled"]
    pub mod MPUNSDIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Nonsecure MPU is enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Nonsecure MPU is disabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Security Attribution Unit Disable"]
    pub mod SAUDIS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SAU is enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "SAU is disabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock Secure VTOR, Application interrupt and Reset Control Registers"]
    pub mod LKSVTAIRCR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock these registers"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the VTOR_S, AIRCR\\[PRIS\\], and AIRCR\\[BFHFNMINS\\] registers"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock Nonsecure Vector Table Offset Register"]
    pub mod LKNSVTOR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock this register"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the VTOR_NS register"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock Secure MPU"]
    pub mod LKSMPU {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock these registers"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the MPU_CTRL, MPU_RNR, MPU_RBAR, MPU_RLAR, MPU_RBAR_An and MPU_RLAR_An from software or from a debug agent connected to the processor in Secure state"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock Nonsecure MPU"]
    pub mod LKNSMPU {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock these registers"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn from software or from a debug agent connected to the processor"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock SAU"]
    pub mod LKSAU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock these registers"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Disable writes to the SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers from software or from a debug agent connected to the processor"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Processor current security"]
    pub mod PCURRNS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processor is in Secure state"]
            pub const SECURE: u32 = 0;
            #[doc = "Processor is in Nonsecure state"]
            pub const NONSECURE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC FLW Control"]
pub mod TRDC_FLW_CTL {
    pub use crate::RW as access;
    #[doc = "Lock bit"]
    pub mod LK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FLW registers may be modified."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "FLW registers are locked until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Valid bit"]
    pub mod V {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FLW function is disabled."]
            pub const INVALID: u32 = 0;
            #[doc = "FLW function is enabled."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC FLW Physical Base"]
pub mod TRDC_FLW_PBASE {
    pub use crate::RO as access;
    #[doc = "Physical base address"]
    pub mod PBASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC FLW Array Base"]
pub mod TRDC_FLW_ABASE {
    pub use crate::RW as access;
    #[doc = "Array base address low"]
    pub mod ABASE_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Array base address high"]
    pub mod ABASE_H {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC FLW Block Count"]
pub mod TRDC_FLW_BCNT {
    pub use crate::RW as access;
    #[doc = "Block Count"]
    pub mod BCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC Fault Domain ID"]
pub mod TRDC_FDID {
    pub use crate::RW as access;
    #[doc = "Domain ID of Faulted Access"]
    pub mod FDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TRDC Domain Error Location Register"]
pub mod TRDC_DERRLOC {
    pub use crate::RO as access;
    #[doc = "MBC instance"]
    pub mod MBCINST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MRC instance"]
    pub mod MRCINST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_0_DFMT1 {
    pub use crate::RW as access;
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_1_DFMT1 {
    pub use crate::RW as access;
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_2_DFMT1 {
    pub use crate::RW as access;
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_3_DFMT1 {
    pub use crate::RW as access;
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DAC Master Domain Assignment Register"]
pub mod MDA_W0_4_DFMT1 {
    pub use crate::RW as access;
    #[doc = "Domain identifier"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Privileged attribute"]
    pub mod PA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to user."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to privileged."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's privileged/user attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Secure attribute"]
    pub mod SA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Force the bus attribute for this master to secure."]
            pub const ZERO: u32 = 0;
            #[doc = "Force the bus attribute for this master to nonsecure."]
            pub const ONE: u32 = 0x01;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const TWO: u32 = 0x02;
            #[doc = "Use the bus master's secure/nonsecure attribute directly."]
            pub const THREE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DID Bypass"]
    pub mod DIDB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
            pub const REG: u32 = 0;
            #[doc = "Use the DID input as the domain identifier."]
            pub const INPUT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain format"]
    pub mod DFMT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Processor-core domain assignment"]
            pub const CPU: u32 = 0;
            #[doc = "Non-processor domain assignment"]
            pub const NONCPU: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1-bit Lock"]
    pub mod LK1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Register can be written by any secure privileged write."]
            pub const UNLOCK: u32 = 0;
            #[doc = "Register is locked (read-only) until the next reset."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The Wr domain assignment is invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "The Wr domain assignment is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
