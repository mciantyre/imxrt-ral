#[doc = "ETHERCAT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Type"]
    pub TYPE: u8,
    #[doc = "Revision"]
    pub REVISION: u8,
    #[doc = "Build"]
    pub BUILD: u16,
    #[doc = "FMMUs supported"]
    pub FMMUS_SUPPORTED: u8,
    #[doc = "SyncManagers supported"]
    pub SYNCMANAGERS_SUPPORTED: u8,
    #[doc = "RAM Size"]
    pub RAM_SIZE: u8,
    #[doc = "Port configuration"]
    pub PORT_DESCRIPTOR: u8,
    #[doc = "Register ESC Features supported"]
    pub ESC_FEATURES_SUPPORTED: u16,
    _reserved0: [u8; 0x06],
    #[doc = "Configured Station Address"]
    pub CONFIGURED_STATION_ADDRESS: u16,
    #[doc = "Configured Station Alias"]
    pub CONFIGURED_STATION_ALIAS_PDI: u16,
    _reserved1: [u8; 0x0c],
    #[doc = "Register Write Enable"]
    pub REGISTER_WRITE_ENABLE: u8,
    #[doc = "Register Write Protection"]
    pub REGISTER_WRITE_PROTECTION: u8,
    _reserved2: [u8; 0x0e],
    #[doc = "ESC Write Enable"]
    pub ESC_WRITE_ENABLE: u8,
    #[doc = "ESC Write Protection"]
    pub ESC_WRITE_PROTECTION: u8,
    _reserved3: [u8; 0x0e],
    #[doc = "ESC Reset ECAT WRITE"]
    pub ESC_RESET_ECAT_WRITE: u8,
    #[doc = "ESC Reset PDI WRITE"]
    pub ESC_RESET_PDI_WRITE_PDI: u8,
    _reserved4: [u8; 0xbe],
    #[doc = "ESC DL Control"]
    pub ESC_DL_CONTROL: u32,
    _reserved5: [u8; 0x04],
    #[doc = "Physical Read Write Offset"]
    pub PHYSICAL_READ_WRITE_OFFSET: u16,
    _reserved6: [u8; 0x06],
    #[doc = "ESC DL Status"]
    pub ESC_DL_STATUS: u16,
    _reserved7: [u8; 0x0e],
    #[doc = "AL Control"]
    pub AL_CONTROL: u16,
    _reserved8: [u8; 0x0e],
    #[doc = "AL Status"]
    pub AL_STATUS_PDI: u16,
    _reserved9: [u8; 0x02],
    #[doc = "AL Status Code"]
    pub AL_STATUS_CODE_PDI: u16,
    _reserved10: [u8; 0x02],
    #[doc = "RUN LED Override"]
    pub RUN_LED_OVERRIDE: u8,
    #[doc = "ERR LED Override"]
    pub ERR_LED_OVERRIDE: u8,
    _reserved11: [u8; 0x06],
    #[doc = "PDI Control"]
    pub PDI_CONTROL: u8,
    #[doc = "ESC Configuration"]
    pub ESC_CONFIGURATION: u8,
    _reserved12: [u8; 0x0c],
    #[doc = "PDI Information"]
    pub PDI_INFORMATION: u16,
    #[doc = "Register PDI On-chip bus configuration"]
    pub PDI_ON_CHIP_BUS_CONFIGURATION: u8,
    #[doc = "PDI Configuration Sync Latch 1 and 0 PDI Configuration"]
    pub SYNC_LATCH_1_AND_0_PDI_CONFIGURATION: u8,
    #[doc = "Register PDI On-chip bus extended configuration."]
    pub PDI_ON_CHIP_BUS_EXTENDED_CONFIGURATION: u16,
    _reserved13: [u8; 0xac],
    #[doc = "ECAT Event Mask"]
    pub ECAT_EVENT_MASK: u16,
    _reserved14: [u8; 0x02],
    #[doc = "PDI AL Event Mask"]
    pub PDI_AL_EVENT_MASK_PDI: u32,
    _reserved15: [u8; 0x08],
    #[doc = "ECAT Event Request"]
    pub ECAT_EVENT_REQUEST: u16,
    _reserved16: [u8; 0x0e],
    #[doc = "AL Event request"]
    pub AL_EVENT_REQUEST: u32,
    _reserved17: [u8; 0xe8],
    #[doc = "ECAT Processing Unit Error Counter"]
    pub ECAT_PROCESSING_UNIT_ERROR_COUNTER: u8,
    #[doc = "PDI Error counter"]
    pub PDI_ERROR_COUNTER: u8,
    #[doc = "ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER_PDI_ERROR_CODE."]
    pub ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER: u8,
    _reserved18: [u8; 0xf1],
    #[doc = "Watchdog Divider"]
    pub WATCHDOG_DIVIDER: u16,
    _reserved19: [u8; 0x0e],
    #[doc = "Register Watchdog Time PDI"]
    pub WATCHDOG_TIME_PDI: u16,
    _reserved20: [u8; 0x0e],
    #[doc = "Regsister Watchdog Time Process Data"]
    pub WATCHDOG_TIME_PROCESS_DATA: u16,
    _reserved21: [u8; 0x1e],
    #[doc = "Watchdog Status Process Data"]
    pub WATCHDOG_STATUS_PROCESS_DATA: u16,
    #[doc = "Watchdog Counter Process Data"]
    pub WATCHDOG_COUNTER_PROCESS_DATA: u8,
    #[doc = "Watchdog Counter PDI"]
    pub WATCHDOG_COUNTER_PDI: u8,
    _reserved22: [u8; 0xbc],
    #[doc = "EEPROM Configuration"]
    pub EEPROM_CONFIGURATION: u8,
    #[doc = "EEPROM PDI Access State"]
    pub REGISTER_EEPROM_PDI_ACCESS_STATE_PDI: u8,
    #[doc = "Register EEPROM Control/Status"]
    pub EEPROM_CONTROL_STATUS: u16,
    #[doc = "EEPROM Address"]
    pub EEPROM_ADDRESS: u32,
    #[doc = "EEPROM Data"]
    pub EEPROM_DATA: u64,
    #[doc = "MII Management Control/Status"]
    pub MII_MANAGEMENT_CONTROL_OR_STATUS: u16,
    #[doc = "PHY Address"]
    pub PHY_ADDRESS: u8,
    #[doc = "PHY Register Address"]
    pub PHY_REGISTER_ADDRESS: u8,
    #[doc = "PHY Data"]
    pub PHY_DATA: u16,
    #[doc = "MII Management ECAT Access State"]
    pub MII_MANAGEMENT_ECAT_ACCESS_STATE: u8,
    #[doc = "MII Management PDI Access State"]
    pub MII_MANAGEMENT_PDI_ACCESS_STATE: u8,
    #[doc = "PHY Port"]
    pub PHY_PORT_STATUS: [u8; 2usize],
    _reserved23: [u8; 0x03e6],
    #[doc = "Distributed Clocks Receive Times"]
    pub RECEIVE_TIMES: u32,
    #[doc = "Distributed Clocks Receive Time Port 1"]
    pub RECEIVE_TIME_PORT_1: u32,
    _reserved24: [u8; 0x08],
    #[doc = "Register System Time"]
    pub SYSTEM_TIME: u64,
    #[doc = "Distributed Clocks Register Receive Time ECAT Processing Unit"]
    pub RECEIVE_TIME_ECAT_PROCESSING_UNIT: u64,
    #[doc = "Register System Time Offset"]
    pub SYSTEM_TIME_OFFSET: u64,
    #[doc = "Register System Time Delay"]
    pub SYSTEM_TIME_DELAY: u32,
    #[doc = "Register System Time Difference"]
    pub SYSTEM_TIME_DIFFERENCE: u32,
    #[doc = "Register Speed Counter Start"]
    pub SPEED_COUNTER_START: u16,
    #[doc = "Register Speed Counter Diff"]
    pub SPEED_COUNTER_DIFF: u16,
    #[doc = "Register System Time Difference Filter Depth"]
    pub SYSTEM_TIME_DIFFERENCE_FILTER_DEPTH: u8,
    #[doc = "Register Speed Counter Filter Depth"]
    pub SPEED_COUNTER_FILTER_DEPTH: u8,
    _reserved25: [u8; 0x4a],
    #[doc = "Register Cyclic Unit Control"]
    pub CYCLIC_UNIT_CONTROL: u8,
    #[doc = "Register Activation register"]
    pub UNIT_ACTIVATION_REGISTER: u8,
    #[doc = "Register Pulse Length of SyncSignals"]
    pub UNI_PULSE_LENGTH_OF_SYNCSIGNALS: u16,
    #[doc = "Register Activation Status"]
    pub UNIT_ACTIVATION_STATUS: u8,
    _reserved26: [u8; 0x09],
    #[doc = "Register SYNC0 Status"]
    pub UNIT_SYNC0_STATUS: u8,
    #[doc = "Register SYNC1 Status"]
    pub UNIT_SYNC1_STATUS: u8,
    #[doc = "Register Start Time Cyclic Operation"]
    pub UNIT_START_TIME_CYCLIC_OPERATION: u64,
    #[doc = "Register Next SYNC1 Pulse"]
    pub UNIT_NEXT_SYNC1_PULSE: u64,
    #[doc = "Register SYNC0 Cycle Time"]
    pub UNIT_SYNC0_CYCLE_TIME: u32,
    #[doc = "Register SYNC1 Cycle Time"]
    pub UNIT_SYNC1_CYCLE_TIME: u32,
    #[doc = "Register Latch0 Control"]
    pub LATCH0_CONTROL: u8,
    #[doc = "Register Latch1 Control"]
    pub LATCH1_CONTROL: u8,
    _reserved27: [u8; 0x04],
    #[doc = "Register Latch0 Status"]
    pub LATCH0_STATUS: u8,
    #[doc = "Register Latch1 Status"]
    pub LATCH1_STATUS: u8,
    #[doc = "Register Latch0 Time Positive Edge"]
    pub LATCH0_TIME_POSITIVE_EDGE: u64,
    #[doc = "Register Latch0 Time Negative Edge"]
    pub LATCH0_TIME_NEGATIVE_EDGE: u64,
    #[doc = "Register Latch1 Time Positive Edge"]
    pub LATCH1_TIME_POSITIVE_EDGE: u64,
    #[doc = "Register Latch1 Time Negative Edge"]
    pub LATCH1_TIME_NEGATIVE_EDGE: u64,
    _reserved28: [u8; 0x20],
    #[doc = "Register EtherCAT Buffer Change Event Time"]
    pub ETHERCAT_BUFFER_CHANGE_EVENT_TIME: u32,
    _reserved29: [u8; 0x04],
    #[doc = "Register PDI Buffer Start Event Time"]
    pub PDI_BUFFER_START_EVENT_TIME: u32,
    #[doc = "Register PDI Buffer Change Event Time"]
    pub PDI_BUFFER_CHANGE_EVENT_TIME: u32,
    _reserved30: [u8; 0x0400],
    #[doc = "Register Product ID IP Core"]
    pub PRODUCT_ID_IP_CORE: u64,
    #[doc = "Register Vendor ID IP Core"]
    pub VENDOR_ID_IP_CORE: u64,
    _reserved31: [u8; 0x0100],
    #[doc = "Register General Purpose Outputs"]
    pub GENERAL_PURPOSE_OUTPUTS: u16,
    _reserved32: [u8; 0x06],
    #[doc = "Register General Purpose Inputs"]
    pub GENERAL_PURPOSE_INPUTS: u16,
}
#[doc = "Type"]
pub mod TYPE {
    pub use crate::RO as access;
    #[doc = "Type of EtherCAT controller"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Revision"]
pub mod REVISION {
    pub use crate::RO as access;
    #[doc = "Revision of EtherCAT controller."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Build"]
pub mod BUILD {
    pub use crate::RO as access;
    #[doc = "Build of EtherCAT controller"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "FMMUs supported"]
pub mod FMMUS_SUPPORTED {
    pub use crate::RO as access;
    #[doc = "Number of supported FMMU channels (or entities)"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SyncManagers supported"]
pub mod SYNCMANAGERS_SUPPORTED {
    pub use crate::RO as access;
    #[doc = "Number of supported SyncManager channels (or entities)"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "RAM Size"]
pub mod RAM_SIZE {
    pub use crate::RO as access;
    #[doc = "Process Data RAM size supported in KByte"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port configuration"]
pub mod PORT_DESCRIPTOR {
    pub use crate::RO as access;
    #[doc = "Port 0"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Port 1"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register ESC Features supported"]
pub mod ESC_FEATURES_SUPPORTED {
    pub use crate::RO as access;
    #[doc = "FMMU Operation:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bit oriented"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Byte oriented"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Unused register access:"]
    pub mod BF1 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "allowed"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "not supported"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Distributed Clocks:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not available"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Available"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Distributed Clocks (width):"]
    pub mod BF3 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "32 bit"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "64 bit"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enhanced Link Detection MII:"]
    pub mod BF6 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not available"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Available"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Separate Handling of FCS Errors:"]
    pub mod BF7 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not supported"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enhanced DC SYNC Activation"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not available"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Available"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EtherCAT LRW command support:"]
    pub mod BF9 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Supported"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Not supported"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EtherCAT read/write command support (BRW, APRW, FPRW):"]
    pub mod BF10 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Supported"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Not supported"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Fixed FMMU/SyncManager configuration"]
    pub mod BF11 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Variable configuration"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Fixed configuration (refer to documentation of supporting ESCs)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Configured Station Address"]
pub mod CONFIGURED_STATION_ADDRESS {
    pub use crate::RW as access;
    #[doc = "Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands)."]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Configured Station Alias"]
pub mod CONFIGURED_STATION_ALIAS_PDI {
    pub use crate::RW as access;
    #[doc = "Alias Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands)."]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Write Enable"]
pub mod REGISTER_WRITE_ENABLE {
    pub use crate::RW as access;
    #[doc = "Register Write Enable."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Write Protection"]
pub mod REGISTER_WRITE_PROTECTION {
    pub use crate::RW as access;
    #[doc = "Register write protection."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Protection disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Protection enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ESC Write Enable"]
pub mod ESC_WRITE_ENABLE {
    pub use crate::RW as access;
    #[doc = "ESC Write Enable"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ESC Write Protection"]
pub mod ESC_WRITE_PROTECTION {
    pub use crate::RW as access;
    #[doc = "Write protect:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Protection disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Protection enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ESC Reset ECAT WRITE"]
pub mod ESC_RESET_ECAT_WRITE {
    pub use crate::RW as access;
    #[doc = "A reset is asserted after writing 0x52 ('R'), 0x45 ('E') and 0x53 ('S') in this register with 3 consecutive frames."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ESC Reset PDI WRITE"]
pub mod ESC_RESET_PDI_WRITE_PDI {
    pub use crate::RW as access;
    #[doc = "A reset is asserted after writing 0x52 ('R'), 0x45 ('E') and 0x53 ('S') in this register with 3 consecutive commands."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ESC DL Control"]
pub mod ESC_DL_CONTROL {
    pub use crate::RW as access;
    #[doc = "Forwarding Rule"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "EtherCAT frames are processed, non-EtherCAT frames are forwarded without processing or modification."]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "EtherCAT frames are processed, non-EtherCAT frames are destroyed."]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Temporary use of settings in 0x0100:0x0103\\[8:15\\]:"]
    pub mod BF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "permanent use"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "use for about 1 second, then revert to previous settings"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Port 0:"]
    pub mod BF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Auto Close"]
            pub const BF_VAL_1: u32 = 0x01;
            #[doc = "Open"]
            pub const BF_VAL_2: u32 = 0x02;
            #[doc = "Closed"]
            pub const BF_VAL_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Port 1:"]
    pub mod BF10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Auto Close"]
            pub const BF_VAL_1: u32 = 0x01;
            #[doc = "Open"]
            pub const BF_VAL_2: u32 = 0x02;
            #[doc = "Closed"]
            pub const BF_VAL_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RX FIFO Size (ESC delays start of forwarding until FIFO is at least half full)."]
    pub mod BF16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Station alias:"]
    pub mod BF24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Ignore Station Alias"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Alias can be used for all configured address command types (FPRD, FPWR, ...)"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Physical Read Write Offset"]
pub mod PHYSICAL_READ_WRITE_OFFSET {
    pub use crate::RW as access;
    #[doc = "This register is used for ReadWrite commands in Device Addressing mode (FPRW, APRW, BRW)."]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ESC DL Status"]
pub mod ESC_DL_STATUS {
    pub use crate::RO as access;
    #[doc = "Register ESC DL Status"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PDI operational/EEPROM loaded correctly:"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "EEPROM loaded correctly, PDI operational (access to Process Data RAM)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PDI Watchdog Status:"]
    pub mod BF1 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Watchdog expired"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Watchdog reloaded"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enhanced Link detection:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Deactivated for all ports"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Activated for at least one port"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Physical link on Port 0:"]
    pub mod BF4 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No link"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Link detected"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Physical link on Port 1:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No link"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Link detected"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Port 0:"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Open"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Closed"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Communication on Port 0:"]
    pub mod BF9 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No stable communication"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Communication established"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Port 1"]
    pub mod BF10 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Open"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Closed"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Communication on Port 1:"]
    pub mod BF11 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No stable communication"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Communication established"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "AL Control"]
pub mod AL_CONTROL {
    pub use crate::RW as access;
    #[doc = "Initiate State Transition of the Device State Machine:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Error Ind Ack"]
    pub mod BF4 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Ack of Error Ind in AL status register"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Ack of Error Ind in AL status register"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Device Identification:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No request"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Device Identification request"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "AL Status"]
pub mod AL_STATUS_PDI {
    pub use crate::RW as access;
    #[doc = "Actual State of the Device State Machine:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Error Ind:"]
    pub mod BF4 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Device is in State as requested or Flag cleared by command"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Device has not entered requested State or changed State as result of a local action"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Device Identification:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Device Identification not valid"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Device Identification loaded"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "AL Status Code"]
pub mod AL_STATUS_CODE_PDI {
    pub use crate::RW as access;
    #[doc = "AL Status Code"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "RUN LED Override"]
pub mod RUN_LED_OVERRIDE {
    pub use crate::RW as access;
    #[doc = "LED code and AL Status"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Override:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Override disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Override enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ERR LED Override"]
pub mod ERR_LED_OVERRIDE {
    pub use crate::RW as access;
    #[doc = "LED code"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Override:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Override disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Override enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PDI Control"]
pub mod PDI_CONTROL {
    pub use crate::RO as access;
    #[doc = "Process data interface:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interface deactivated (no PDI)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "4 Digital Input"]
            pub const BF_VAL_1: u8 = 0x01;
            #[doc = "4 Digital Output"]
            pub const BF_VAL_2: u8 = 0x02;
            #[doc = "2 Digital Input and 2 Digital Output"]
            pub const BF_VAL_3: u8 = 0x03;
            #[doc = "Digital I/O"]
            pub const BF_VAL_4: u8 = 0x04;
            #[doc = "SPI Slave"]
            pub const BF_VAL_5: u8 = 0x05;
            #[doc = "Oversampling I/O"]
            pub const BF_VAL_6: u8 = 0x06;
            #[doc = "16 Bit asynchronous Microcontroller interface"]
            pub const BF_VAL_8: u8 = 0x08;
            #[doc = "8 Bit asynchronous Microcontroller interface"]
            pub const BF_VAL_9: u8 = 0x09;
            #[doc = "16 Bit synchronous Microcontroller interface"]
            pub const BF_VAL_10: u8 = 0x0a;
            #[doc = "8 Bit synchronous Microcontroller interface"]
            pub const BF_VAL_11: u8 = 0x0b;
            #[doc = "32 Digital Input and 0 Digital Output"]
            pub const BF_VAL_12: u8 = 0x10;
            #[doc = "24 Digital Input and 8 Digital Output"]
            pub const BF_VAL_13: u8 = 0x11;
            #[doc = "16 Digital Input and 16 Digital Output"]
            pub const BF_VAL_14: u8 = 0x12;
            #[doc = "8 Digital Input and 24 Digital Output"]
            pub const BF_VAL_15: u8 = 0x13;
            #[doc = "0 Digital Input and 32 Digital Output"]
            pub const BF_VAL_16: u8 = 0x14;
            #[doc = "On-chip bus."]
            pub const BF_VAL_17: u8 = 0x80;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ESC Configuration"]
pub mod ESC_CONFIGURATION {
    pub use crate::RO as access;
    #[doc = "Device emulation (control of AL status):"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "AL status register has to be set by PDI"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "AL status register will be set to value written to AL control register"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enhanced Link detection all ports:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "disabled (if bits \\[7:4\\]=0)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled at all ports (overrides bits \\[7:4\\])"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Distributed Clocks SYNC Out Unit:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "disabled (power saving)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Distributed Clocks Latch In Unit:"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "disabled (power saving)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enhanced Link port 0:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "disabled (if bit 1=0)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enhanced Link port 1:"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "disabled (if bit 1=0)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PDI Information"]
pub mod PDI_INFORMATION {
    pub use crate::RO as access;
    #[doc = "PDI function"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Enabled"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ESC configuration area loaded from EEPROM:"]
    pub mod BF1 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "not loaded"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "loaded"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PDI active:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PDI not active"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "PDI active"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PDI configuration invalid:"]
    pub mod BF3 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PDI configuration ok"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "PDI configuration invalid"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register PDI On-chip bus configuration"]
pub mod PDI_ON_CHIP_BUS_CONFIGURATION {
    pub use crate::RO as access;
    #[doc = "On-chip bus clock:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "asynchronous"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_1: u8 = 0x01;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_2: u8 = 0x02;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_3: u8 = 0x03;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_4: u8 = 0x04;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_5: u8 = 0x05;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_6: u8 = 0x06;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_7: u8 = 0x07;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_8: u8 = 0x08;
            #[doc = "synchronous multiplication factor (N * 25 MHz)"]
            pub const BF_VAL_1_9: u8 = 0x09;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "On-chip bus"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Intel Avalon"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "AXI 010: Xilinx PLB v4.6"]
            pub const BF_VAL_1: u8 = 0x01;
            #[doc = "Xilinx OPB"]
            pub const BF_VAL_2: u8 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PDI Configuration Sync Latch 1 and 0 PDI Configuration"]
pub mod SYNC_LATCH_1_AND_0_PDI_CONFIGURATION {
    pub use crate::RO as access;
    #[doc = "SYNC0 output driver/polarity:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Push-Pull active low"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Open Drain (active low)"]
            pub const BF_VAL_1: u8 = 0x01;
            #[doc = "Push-Pull active high"]
            pub const BF_VAL_2: u8 = 0x02;
            #[doc = "Open Source (active high)"]
            pub const BF_VAL_3: u8 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYNC0/LATCH0 configuration*:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LATCH0 Input"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "SYNC0 Output"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYNC0 mapped to AL Event Request register 0x0220\\[2\\]:"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYNC1 output driver/polarity:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Push-Pull active low"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Open Drain (active low)"]
            pub const BF_VAL_1: u8 = 0x01;
            #[doc = "Push-Pull active high"]
            pub const BF_VAL_2: u8 = 0x02;
            #[doc = "Open Source (active high)"]
            pub const BF_VAL_3: u8 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYNC1/LATCH1 configuration*"]
    pub mod BF6 {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LATCH1 input"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "SYNC1 output"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYNC1 mapped to AL Event Request register 0x0220\\[3\\]:"]
    pub mod BF7 {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Enabled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register PDI On-chip bus extended configuration."]
pub mod PDI_ON_CHIP_BUS_EXTENDED_CONFIGURATION {
    pub use crate::RO as access;
    #[doc = "Read prefetch size (in cycles of PDI width):"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4 cycles"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "1 cycle (typical)"]
            pub const BF_VAL_1: u16 = 0x01;
            #[doc = "2 cycles"]
            pub const BF_VAL_2: u16 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "On-chip bus sub-type for AXI:"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "AXI3"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "AXI4"]
            pub const BF_VAL_1: u16 = 0x01;
            #[doc = "AXI4 LITE"]
            pub const BF_VAL_2: u16 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ECAT Event Mask"]
pub mod ECAT_EVENT_MASK {
    pub use crate::RW as access;
    #[doc = "ECAT Event masking of the ECAT Event Request Events for mapping into ECAT event field of EtherCAT frames:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Corresponding ECAT Event Request register bit is mapped"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PDI AL Event Mask"]
pub mod PDI_AL_EVENT_MASK_PDI {
    pub use crate::RW as access;
    #[doc = "AL Event masking of the AL Event Request register Events for mapping to PDI IRQ signal:"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Corresponding AL Event Request register bit is not mapped"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Corresponding AL Event Request register bit is mapped"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ECAT Event Request"]
pub mod ECAT_EVENT_REQUEST {
    pub use crate::RO as access;
    #[doc = "DC Latch event:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change on DC Latch Inputs"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "At least one change on DC Latch Inputs"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DL Status event:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change in DL Status"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "DL Status change"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AL Status event:"]
    pub mod BF3 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change in AL Status"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "AL Status change"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF4 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Sync Channel 0 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 0 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Sync Channel 1 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 1 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF6 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Sync Channel 2 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 2 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF7 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Sync Channel 3 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 3 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Sync Channel 4 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 4 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF9 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Sync Channel 5 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 5 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF10 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Sync Channel 6 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 6 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    pub mod BF11 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Sync Channel 7 event"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Sync Channel 7 event pending"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "AL Event request"]
pub mod AL_EVENT_REQUEST {
    pub use crate::RO as access;
    #[doc = "AL Control event:"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No AL Control Register change"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "AL Control Register has been written3"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DC Latch event:"]
    pub mod BF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change on DC Latch Inputs"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "At least one change on DC Latch Inputs"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "State of DC SYNC0 (if register 0x0151\\[3\\]=1):"]
    pub mod BF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "State of DC SYNC1 (if register 0x0151\\[7\\]=1):"]
    pub mod BF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SyncManager activation register (SyncManager register offset 0x6) changed:"]
    pub mod BF4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change in any SyncManager"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "At least one SyncManager changed"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EEPROM Emulation:"]
    pub mod BF5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No command pending"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "EEPROM command pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Watchdog Process Data:"]
    pub mod BF6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Has not expired"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Has expired"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SyncManager interrupts (SyncManager register offset 0x5, bit \\[0\\] or \\[1\\]):"]
    pub mod BF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No SyncManager 0 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 0 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No SyncManager 1 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 1 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No SyncManager2 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 2 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No SyncManager 3 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 3 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No SyncManager4 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 4 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No SyncManager 5 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 5 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No SyncManager 6 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager6 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No SyncManager 7 interrupt"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "SyncManager 7 interrupt pending"]
            pub const BF_VAL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ECAT Processing Unit Error Counter"]
pub mod ECAT_PROCESSING_UNIT_ERROR_COUNTER {
    pub use crate::RW as access;
    #[doc = "ECAT Processing Unit error counter (counting is stopped when 0xFF is reached). Counts errors of frames passing the Processing Unit."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PDI Error counter"]
pub mod PDI_ERROR_COUNTER {
    pub use crate::RW as access;
    #[doc = "PDI Error counter (counting is stopped when 0xFF is reached). Counts if a PDI access has an interface error."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER_PDI_ERROR_CODE."]
pub mod ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER {
    pub use crate::RO as access;
    #[doc = "Busy violation during read access"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "no error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "error detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Busy violation during write access"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "no error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "error detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Addressing error for a read access (odd address without BHE)"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "no error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "error detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Addressing error for a write access (odd address without BHE)"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "no error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "error detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Watchdog Divider"]
pub mod WATCHDOG_DIVIDER {
    pub use crate::RW as access;
    #[doc = "Watchdog divider: Number of 25 MHz tics (minus 2) that represent the basic watchdog increment. (Default value is 100us = 2498)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Watchdog Time PDI"]
pub mod WATCHDOG_TIME_PDI {
    pub use crate::RW as access;
    #[doc = "Watchdog Time PDI: number or basic watchdog increments (Default value with Watchdog divider 100us means 100ms Watchdog)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Regsister Watchdog Time Process Data"]
pub mod WATCHDOG_TIME_PROCESS_DATA {
    pub use crate::RW as access;
    #[doc = "Watchdog Time Process Data"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Watchdog Status Process Data"]
pub mod WATCHDOG_STATUS_PROCESS_DATA {
    pub use crate::RO as access;
    #[doc = "Watchdog Status of Process Data (triggered by SyncManagers)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Watchdog Process Data expired"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Watchdog Process Data is active or disabled"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Watchdog Counter Process Data"]
pub mod WATCHDOG_COUNTER_PROCESS_DATA {
    pub use crate::RW as access;
    #[doc = "Watchdog Counter Process Data (counting is stopped when 0xFF is reached). Counts if Process Data Watchdog expires."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Watchdog Counter PDI"]
pub mod WATCHDOG_COUNTER_PDI {
    pub use crate::RW as access;
    #[doc = "Watchdog PDI counter (counting is stopped when 0xFF is reached)."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "EEPROM Configuration"]
pub mod EEPROM_CONFIGURATION {
    pub use crate::RW as access;
    #[doc = "EEPROM control is offered to PDI"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "no"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "yes (PDI has EEPROM control)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Force ECAT access"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not change Bit 0x0501\\[0\\]"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Reset Bit 0x0501\\[0\\] to 0"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "EEPROM PDI Access State"]
pub mod REGISTER_EEPROM_PDI_ACCESS_STATE_PDI {
    pub use crate::RW as access;
    #[doc = "Access to EEPROM:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PDI releases EEPROM access"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI takes EEPROM access (PDI has EEPROM control)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register EEPROM Control/Status"]
pub mod EEPROM_CONTROL_STATUS {
    pub use crate::RW as access;
    #[doc = "ECAT write enable2"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write requests are disabled"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Write requests are enabled"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EEPROM emulation:"]
    pub mod BF5 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Normal operation (I2C interface used)"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "PDI emulates EEPROM (I2C not used)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Supported number of EEPROM read bytes:"]
    pub mod BF6 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4 Bytes"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "8 Bytes"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Selected EEPROM Algorithm:"]
    pub mod BF7 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1 address byte (1Kbit to 16Kbit EEPROMs)"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "2 address bytes (32Kbit to 4 Mbit EEPROMs)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command register"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No command/EEPROM idle (clear error bits)"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Read"]
            pub const BF_VAL_1: u16 = 0x01;
            #[doc = "Write"]
            pub const BF_VAL_2: u16 = 0x02;
            #[doc = "Reload"]
            pub const BF_VAL_3: u16 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Checksum Error in ESC Configuration Area:"]
    pub mod BF11 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Checksum ok"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Checksum error"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EEPROM loading status:"]
    pub mod BF12 {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "EEPROM loaded, device information ok"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Error Acknowledge/Command3:"]
    pub mod BF13 {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No error"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Missing EEPROM acknowledge or invalid command"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Error Write Enable3:"]
    pub mod BF14 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No error"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Write Command without Write enable"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Busy:"]
    pub mod BF15 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "EEPROM Interface is idle"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "EEPROM Interface is busy"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "EEPROM Address"]
pub mod EEPROM_ADDRESS {
    pub use crate::RW as access;
    #[doc = "EEPROM Address"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "EEPROM Data"]
pub mod EEPROM_DATA {
    pub use crate::RW as access;
    #[doc = "EEPROM Write data (data to be written to EEPROM) or EEPROM Read data (data read from EEPROM,. lower bytes)"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EEPROM Read data (data read from EEPROM, higher bytes)"]
    pub mod BF16 {
        pub const offset: u64 = 16;
        pub const mask: u64 = 0xffff_ffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "MII Management Control/Status"]
pub mod MII_MANAGEMENT_CONTROL_OR_STATUS {
    pub use crate::RW as access;
    #[doc = "Write enable*:"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write disabled"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Write enabled"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Management Interface can be controlled by PDI (registers 0x0516-0x0517):"]
    pub mod BF1 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only ECAT control"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "PDI control possible"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MI link detection and configuration:"]
    pub mod BF2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled for all ports"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Enabled for at least one MII port, refer to PHY Port Status (0x0518 ff.) for details"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PHY address of port 0"]
    pub mod BF3 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command register*:"]
    pub mod BF8 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No command/MI idle (clear error bits)"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Read"]
            pub const BF_VAL_1: u16 = 0x01;
            #[doc = "Write"]
            pub const BF_VAL_2: u16 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read error:"]
    pub mod BF13 {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No read error"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Read error occurred (PHY or register not available)"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command error:"]
    pub mod BF14 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Last Command was successful"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "Invalid command or write command without Write Enable"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Busy:"]
    pub mod BF15 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "MII Management Interface is idle"]
            pub const BF_VAL_0: u16 = 0;
            #[doc = "MII Management Interface is busy"]
            pub const BF_VAL_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PHY Address"]
pub mod PHY_ADDRESS {
    pub use crate::RW as access;
    #[doc = "PHY Address"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Show configured PHY address of port 0-3 in register 0x0510\\[7:3\\]. This is used if the PHY addresses are not consecutive."]
    pub mod BF7 {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Register 0x0510\\[7:3\\] shows PHY address of port 0 (this is also the PHY address offset, if the PHY addresses are consecutive)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Register 0x0510\\[7:3\\] shows PHY address of port 0x0512\\[4:0\\] (valid values 0-3)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PHY Register Address"]
pub mod PHY_REGISTER_ADDRESS {
    pub use crate::RW as access;
    #[doc = "Address of PHY Register that shall be read/written"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PHY Data"]
pub mod PHY_DATA {
    pub use crate::RW as access;
    #[doc = "PHY Read/Write Data"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "MII Management ECAT Access State"]
pub mod MII_MANAGEMENT_ECAT_ACCESS_STATE {
    pub use crate::RW as access;
    #[doc = "Access to MII management:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ECAT enables PDI takeover of MII management interface"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "ECAT claims exclusive access to MII management interface"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "MII Management PDI Access State"]
pub mod MII_MANAGEMENT_PDI_ACCESS_STATE {
    pub use crate::RW as access;
    #[doc = "Access to MII management:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ECAT has access to MII management"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI has access to MII management"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Force PDI Access State:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not change Bit 0x0517\\[0\\]"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Reset Bit 0x0517\\[0\\] to 0"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PHY Port"]
pub mod PHY_PORT_STATUS {
    pub use crate::RW as access;
    #[doc = "Physical link status (PHY status register 1.2):"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No physical link"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Physical link detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Link status (100 Mbit/s, Full Duplex, Auto negotiation):"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No link"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Link detected"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Link status error:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No error"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Link error, link inhibited"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read error:"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No read error occurred"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "A read error has occurred"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Link partner error:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No error detected"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Link partner error"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PHY configuration updated"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No update"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PHY configuration was updated"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Distributed Clocks Receive Times"]
pub mod RECEIVE_TIMES {
    pub use crate::RW as access;
    #[doc = "Write"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Local time at the beginning of the last receive frame containing a write access to register 0x0900."]
    pub mod BF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Distributed Clocks Receive Time Port 1"]
pub mod RECEIVE_TIME_PORT_1 {
    pub use crate::RO as access;
    #[doc = "Local time at the beginning of a frame (start first bit of preamble) received at port 1 containing a BWR or FPWR to register 0x0900."]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register System Time"]
pub mod SYSTEM_TIME {
    pub use crate::RW as access;
    #[doc = "ECAT read access"]
    pub mod BF0ECAT {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Distributed Clocks Register Receive Time ECAT Processing Unit"]
pub mod RECEIVE_TIME_ECAT_PROCESSING_UNIT {
    pub use crate::RO as access;
    #[doc = "Local time at the beginning of a frame (start first bit of preamble) received at the ECAT Processing Unit containing a write access to register 0x0900"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register System Time Offset"]
pub mod SYSTEM_TIME_OFFSET {
    pub use crate::RW as access;
    #[doc = "Difference between local time and System Time. Offset is added to the local time."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register System Time Delay"]
pub mod SYSTEM_TIME_DELAY {
    pub use crate::RW as access;
    #[doc = "Delay between Reference Clock and the eCAT"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register System Time Difference"]
pub mod SYSTEM_TIME_DIFFERENCE {
    pub use crate::RO as access;
    #[doc = "Mean difference between local copy of system Time and received System Time values"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit field access for ECAT: r/-"]
    pub mod BF31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Local copy of System Time less than received System Time"]
            pub const BF_VAL_0: u32 = 0;
            #[doc = "Local copy of System Time greater than or equal to received System Time"]
            pub const BF_VAL_3: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Speed Counter Start"]
pub mod SPEED_COUNTER_START {
    pub use crate::RW as access;
    #[doc = "Bandwidth for adjustment of local copy of system Time (larger values -> smaller bandwidth and smoother adjustment)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x7fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Speed Counter Diff"]
pub mod SPEED_COUNTER_DIFF {
    pub use crate::RO as access;
    #[doc = "Representation of the deviation between local clock period and Reference Clock's clock period (representation: two's complement)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register System Time Difference Filter Depth"]
pub mod SYSTEM_TIME_DIFFERENCE_FILTER_DEPTH {
    pub use crate::RW as access;
    #[doc = "Filter depth for averaging the received System Time deviation"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Speed Counter Filter Depth"]
pub mod SPEED_COUNTER_FILTER_DEPTH {
    pub use crate::RW as access;
    #[doc = "Filter depth for averaging the clock period deviation"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Cyclic Unit Control"]
pub mod CYCLIC_UNIT_CONTROL {
    pub use crate::RW as access;
    #[doc = "SYNC out unit control:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ECAT-controlled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI-controlled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Latch In unit 0:"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ECAT-controlled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI-controlled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Latch In unit 1:"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ECAT-controlled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "PDI-controlled"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Activation register"]
pub mod UNIT_ACTIVATION_REGISTER {
    pub use crate::RW as access;
    #[doc = "Sync Out Unit activation:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Deactivated"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Activated"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYNC0 generation:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Deactivated"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "SYNC0 pulse is generated"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYNC1 generation:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Deactivated"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "SYNC1 pulse is generated"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Auto-activation by writing Start Time Cyclic Operation (0x0990:0x0997):"]
    pub mod BF3 {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Auto-activation enabled. 0x0981\\[0\\] is set automatically after Start Time is written"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Extension of Start Time Cyclic Operation (0x0990:0x0993):"]
    pub mod BF4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No extension"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Extend 32 bit written Start Time to 64 bit"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Start Time plausibility check:"]
    pub mod BF5 {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled. SyncSignal generation if Start Time is reached."]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Immediate SyncSignal generation if Start Time is outside near future (see 0x0981\\[6\\])"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Near future configuration (approx.):"]
    pub mod BF6 {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1/2 DC width future (2^31ns or 2^63ns)"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "~2.1 sec. future (2^31ns)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SyncSignal debug pulse (Vasily bit):"]
    pub mod BF7 {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Deactivated"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Immediately generate one ping only on SYNC0-1 according to 0x0981\\[2:1\\] for debugging."]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Pulse Length of SyncSignals"]
pub mod UNI_PULSE_LENGTH_OF_SYNCSIGNALS {
    pub use crate::RO as access;
    #[doc = "Pulse length of SyncSignals (in Units of 10ns)"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Acknowledge mode: SyncSignal will be cleared by reading SYNC\\[1:0\\] Status register"]
            pub const BF_VAL_0: u16 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Activation Status"]
pub mod UNIT_ACTIVATION_STATUS {
    pub use crate::RO as access;
    #[doc = "SYNC0 activation state:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "First SYNC0 pulse is not pending"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "First SYNC0 pulse is pending"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYNC1 activation state:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "First SYNC1 pulse is not pending"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "First SYNC1 pulse is pending"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Start Time Cyclic Operation (0x0990:0x0997) plausibility check result when Sync Out Unit was activated:"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Start Time was within near future"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Start Time was out of near future (0x0981\\[6\\])"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register SYNC0 Status"]
pub mod UNIT_SYNC0_STATUS {
    pub use crate::RO as access;
    #[doc = "SYNC0 state for Acknowledge mode."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register SYNC1 Status"]
pub mod UNIT_SYNC1_STATUS {
    pub use crate::RO as access;
    #[doc = "SYNC1 state for Acknowledge mode."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Start Time Cyclic Operation"]
pub mod UNIT_START_TIME_CYCLIC_OPERATION {
    pub use crate::RW as access;
    #[doc = "Write: Start time (System time) of cyclic operation in ns"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Next SYNC1 Pulse"]
pub mod UNIT_NEXT_SYNC1_PULSE {
    pub use crate::RO as access;
    #[doc = "System time of next SYNC1 pulse in ns"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register SYNC0 Cycle Time"]
pub mod UNIT_SYNC0_CYCLE_TIME {
    pub use crate::RW as access;
    #[doc = "Time between two consecutive SYNC0 pulses in ns."]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single shot mode, generate only one SYNC0 pulse."]
            pub const BF_VAL_0: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register SYNC1 Cycle Time"]
pub mod UNIT_SYNC1_CYCLE_TIME {
    pub use crate::RW as access;
    #[doc = "Time between SYNC0 pulse and SYNC1 pulse in ns"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Latch0 Control"]
pub mod LATCH0_CONTROL {
    pub use crate::RW as access;
    #[doc = "Latch0 positive edge:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Continuous Latch active"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Single event (only first event active)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Latch0 negative edge:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Continuous Latch active"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Single event (only first event active)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Latch1 Control"]
pub mod LATCH1_CONTROL {
    pub use crate::RW as access;
    #[doc = "Latch1 positive edge:"]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Continuous Latch active"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Single event (only first event active)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Latch1 negative edge:"]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Continuous Latch active"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Single event (only first event active)"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Latch0 Status"]
pub mod LATCH0_STATUS {
    pub use crate::RO as access;
    #[doc = "Event Latch0 positive edge."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Positive edge not detected or continuous mode"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Positive edge detected in single event mode only."]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Event Latch0 negative edge."]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Negative edge not detected or continuous mode"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Negative edge detected in single event mode only"]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Latch0 pin state"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Latch1 Status"]
pub mod LATCH1_STATUS {
    pub use crate::RO as access;
    #[doc = "Event Latch1 positive edge."]
    pub mod BF0 {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Positive edge not detected or continuous mode"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Positive edge detected in single event mode only."]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Event Latch1 negative edge."]
    pub mod BF1 {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Negative edge not detected or continuous mode"]
            pub const BF_VAL_0: u8 = 0;
            #[doc = "Negative edge detected in single event mode only."]
            pub const BF_VAL_1: u8 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Latch1 pin state"]
    pub mod BF2 {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Latch0 Time Positive Edge"]
pub mod LATCH0_TIME_POSITIVE_EDGE {
    pub use crate::RO as access;
    #[doc = "System time at the positive edge of the Latch0 signal."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Latch0 Time Negative Edge"]
pub mod LATCH0_TIME_NEGATIVE_EDGE {
    pub use crate::RO as access;
    #[doc = "System time at the negative edge of the Latch0 signal."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Latch1 Time Positive Edge"]
pub mod LATCH1_TIME_POSITIVE_EDGE {
    pub use crate::RO as access;
    #[doc = "System time at the positive edge of the Latch1 signal."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Latch1 Time Negative Edge"]
pub mod LATCH1_TIME_NEGATIVE_EDGE {
    pub use crate::RO as access;
    #[doc = "System time at the negative edge of the Latch1 signal."]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register EtherCAT Buffer Change Event Time"]
pub mod ETHERCAT_BUFFER_CHANGE_EVENT_TIME {
    pub use crate::RO as access;
    #[doc = "Local time at the beginning of the frame which causes at least one SyncManager to assert an ECAT event"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register PDI Buffer Start Event Time"]
pub mod PDI_BUFFER_START_EVENT_TIME {
    pub use crate::RO as access;
    #[doc = "Local time when at least one SyncManager asserts a PDI buffer start event"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register PDI Buffer Change Event Time"]
pub mod PDI_BUFFER_CHANGE_EVENT_TIME {
    pub use crate::RO as access;
    #[doc = "Local time when at least one SyncManager asserts a PDI buffer change event"]
    pub mod BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Product ID IP Core"]
pub mod PRODUCT_ID_IP_CORE {
    pub use crate::RO as access;
    #[doc = "Product ID"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register Vendor ID IP Core"]
pub mod VENDOR_ID_IP_CORE {
    pub use crate::RO as access;
    #[doc = "Vendor ID"]
    pub mod BF0 {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register General Purpose Outputs"]
pub mod GENERAL_PURPOSE_OUTPUTS {
    pub use crate::RW as access;
    #[doc = "General Purpose Output Data"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Register General Purpose Inputs"]
pub mod GENERAL_PURPOSE_INPUTS {
    pub use crate::RO as access;
    #[doc = "General Purpose Input Data"]
    pub mod BF0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
