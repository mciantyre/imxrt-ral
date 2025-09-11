#[doc = "USB Analog"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01a0],
    #[doc = "USB VBUS Detect Register"]
    pub USB1_VBUS_DETECT: u32,
    #[doc = "USB VBUS Detect Register"]
    pub USB1_VBUS_DETECT_SET: u32,
    #[doc = "USB VBUS Detect Register"]
    pub USB1_VBUS_DETECT_CLR: u32,
    #[doc = "USB VBUS Detect Register"]
    pub USB1_VBUS_DETECT_TOG: u32,
    #[doc = "USB Charger Detect Register"]
    pub USB1_CHRG_DETECT: u32,
    #[doc = "USB Charger Detect Register"]
    pub USB1_CHRG_DETECT_SET: u32,
    #[doc = "USB Charger Detect Register"]
    pub USB1_CHRG_DETECT_CLR: u32,
    #[doc = "USB Charger Detect Register"]
    pub USB1_CHRG_DETECT_TOG: u32,
    #[doc = "USB VBUS Detect Status Register"]
    pub USB1_VBUS_DETECT_STAT: u32,
    _reserved1: [u8; 0x0c],
    #[doc = "USB Charger Detect Status Register"]
    pub USB1_CHRG_DETECT_STAT: u32,
    _reserved2: [u8; 0x0c],
    #[doc = "USB Loopback Test Register"]
    pub USB1_LOOPBACK: u32,
    #[doc = "USB Loopback Test Register"]
    pub USB1_LOOPBACK_SET: u32,
    #[doc = "USB Loopback Test Register"]
    pub USB1_LOOPBACK_CLR: u32,
    #[doc = "USB Loopback Test Register"]
    pub USB1_LOOPBACK_TOG: u32,
    #[doc = "USB Misc Register"]
    pub USB1_MISC: u32,
    #[doc = "USB Misc Register"]
    pub USB1_MISC_SET: u32,
    #[doc = "USB Misc Register"]
    pub USB1_MISC_CLR: u32,
    #[doc = "USB Misc Register"]
    pub USB1_MISC_TOG: u32,
    _reserved3: [u8; 0x60],
    #[doc = "Chip Silicon Version"]
    pub DIGPROG: u32,
}
#[doc = "USB VBUS Detect Register"]
pub mod USB1_VBUS_DETECT {
    pub use crate::RW as access;
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4.0V"]
            pub const _4V0: u32 = 0;
            #[doc = "4.1V"]
            pub const _4V1: u32 = 0x01;
            #[doc = "4.2V"]
            pub const _4V2: u32 = 0x02;
            #[doc = "4.3V"]
            pub const _4V3: u32 = 0x03;
            #[doc = "4.4V (default)"]
            pub const _4V4: u32 = 0x04;
            #[doc = "4.5V"]
            pub const _4V5: u32 = 0x05;
            #[doc = "4.6V"]
            pub const _4V6: u32 = 0x06;
            #[doc = "4.7V"]
            pub const _4V7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USB OTG discharge VBUS."]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USB OTG charge VBUS."]
    pub mod CHARGE_VBUS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB VBUS Detect Register"]
pub mod USB1_VBUS_DETECT_SET {
    pub use crate::RW as access;
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4.0V"]
            pub const _4V0: u32 = 0;
            #[doc = "4.1V"]
            pub const _4V1: u32 = 0x01;
            #[doc = "4.2V"]
            pub const _4V2: u32 = 0x02;
            #[doc = "4.3V"]
            pub const _4V3: u32 = 0x03;
            #[doc = "4.4V (default)"]
            pub const _4V4: u32 = 0x04;
            #[doc = "4.5V"]
            pub const _4V5: u32 = 0x05;
            #[doc = "4.6V"]
            pub const _4V6: u32 = 0x06;
            #[doc = "4.7V"]
            pub const _4V7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USB OTG discharge VBUS."]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USB OTG charge VBUS."]
    pub mod CHARGE_VBUS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB VBUS Detect Register"]
pub mod USB1_VBUS_DETECT_CLR {
    pub use crate::RW as access;
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4.0V"]
            pub const _4V0: u32 = 0;
            #[doc = "4.1V"]
            pub const _4V1: u32 = 0x01;
            #[doc = "4.2V"]
            pub const _4V2: u32 = 0x02;
            #[doc = "4.3V"]
            pub const _4V3: u32 = 0x03;
            #[doc = "4.4V (default)"]
            pub const _4V4: u32 = 0x04;
            #[doc = "4.5V"]
            pub const _4V5: u32 = 0x05;
            #[doc = "4.6V"]
            pub const _4V6: u32 = 0x06;
            #[doc = "4.7V"]
            pub const _4V7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USB OTG discharge VBUS."]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USB OTG charge VBUS."]
    pub mod CHARGE_VBUS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB VBUS Detect Register"]
pub mod USB1_VBUS_DETECT_TOG {
    pub use crate::RW as access;
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4.0V"]
            pub const _4V0: u32 = 0;
            #[doc = "4.1V"]
            pub const _4V1: u32 = 0x01;
            #[doc = "4.2V"]
            pub const _4V2: u32 = 0x02;
            #[doc = "4.3V"]
            pub const _4V3: u32 = 0x03;
            #[doc = "4.4V (default)"]
            pub const _4V4: u32 = 0x04;
            #[doc = "4.5V"]
            pub const _4V5: u32 = 0x05;
            #[doc = "4.6V"]
            pub const _4V6: u32 = 0x06;
            #[doc = "4.7V"]
            pub const _4V7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USB OTG discharge VBUS."]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "USB OTG charge VBUS."]
    pub mod CHARGE_VBUS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Charger Detect Register"]
pub mod USB1_CHRG_DETECT {
    pub use crate::RW as access;
    #[doc = "Check the contact of USB plug"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not check the contact of USB plug."]
            pub const NO_CHECK: u32 = 0;
            #[doc = "Check whether the USB plug has been in contact with each other"]
            pub const CHECK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Check the charger connection"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
            pub const CHECK: u32 = 0;
            #[doc = "Do not check whether a charger is connected to the USB port."]
            pub const NO_CHECK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control the charger detector."]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable the charger detector."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the charger detector."]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Charger Detect Register"]
pub mod USB1_CHRG_DETECT_SET {
    pub use crate::RW as access;
    #[doc = "Check the contact of USB plug"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not check the contact of USB plug."]
            pub const NO_CHECK: u32 = 0;
            #[doc = "Check whether the USB plug has been in contact with each other"]
            pub const CHECK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Check the charger connection"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
            pub const CHECK: u32 = 0;
            #[doc = "Do not check whether a charger is connected to the USB port."]
            pub const NO_CHECK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control the charger detector."]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable the charger detector."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the charger detector."]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Charger Detect Register"]
pub mod USB1_CHRG_DETECT_CLR {
    pub use crate::RW as access;
    #[doc = "Check the contact of USB plug"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not check the contact of USB plug."]
            pub const NO_CHECK: u32 = 0;
            #[doc = "Check whether the USB plug has been in contact with each other"]
            pub const CHECK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Check the charger connection"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
            pub const CHECK: u32 = 0;
            #[doc = "Do not check whether a charger is connected to the USB port."]
            pub const NO_CHECK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control the charger detector."]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable the charger detector."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the charger detector."]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Charger Detect Register"]
pub mod USB1_CHRG_DETECT_TOG {
    pub use crate::RW as access;
    #[doc = "Check the contact of USB plug"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not check the contact of USB plug."]
            pub const NO_CHECK: u32 = 0;
            #[doc = "Check whether the USB plug has been in contact with each other"]
            pub const CHECK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Check the charger connection"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
            pub const CHECK: u32 = 0;
            #[doc = "Do not check whether a charger is connected to the USB port."]
            pub const NO_CHECK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control the charger detector."]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable the charger detector."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the charger detector."]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB VBUS Detect Status Register"]
pub mod USB1_VBUS_DETECT_STAT {
    pub use crate::RO as access;
    #[doc = "Session End for USB OTG"]
    pub mod SESSEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    pub mod BVALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    pub mod AVALID {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "VBus valid for USB OTG"]
    pub mod VBUS_VALID {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Charger Detect Status Register"]
pub mod USB1_CHRG_DETECT_STAT {
    pub use crate::RO as access;
    #[doc = "State of the USB plug contact detector."]
    pub mod PLUG_CONTACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The USB plug has not made contact."]
            pub const NO_CONTACT: u32 = 0;
            #[doc = "The USB plug has made good contact."]
            pub const GOOD_CONTACT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    pub mod CHRG_DETECTED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The USB port is not connected to a charger."]
            pub const CHARGER_NOT_PRESENT: u32 = 0;
            #[doc = "A charger (either a dedicated charger or a host charger) is connected to the USB port."]
            pub const CHARGER_PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DM line state output of the charger detector."]
    pub mod DM_STATE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DP line state output of the charger detector."]
    pub mod DP_STATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Loopback Test Register"]
pub mod USB1_LOOPBACK {
    pub use crate::RW as access;
    #[doc = "Setting this bit can enable 1"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Loopback Test Register"]
pub mod USB1_LOOPBACK_SET {
    pub use crate::RW as access;
    #[doc = "Setting this bit can enable 1"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Loopback Test Register"]
pub mod USB1_LOOPBACK_CLR {
    pub use crate::RW as access;
    #[doc = "Setting this bit can enable 1"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Loopback Test Register"]
pub mod USB1_LOOPBACK_TOG {
    pub use crate::RW as access;
    #[doc = "Setting this bit can enable 1"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Misc Register"]
pub mod USB1_MISC {
    pub use crate::RW as access;
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    pub mod HS_USE_EXTERNAL_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    pub mod EN_DEGLITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the clk to the UTMI block."]
    pub mod EN_CLK_UTMI {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Misc Register"]
pub mod USB1_MISC_SET {
    pub use crate::RW as access;
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    pub mod HS_USE_EXTERNAL_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    pub mod EN_DEGLITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the clk to the UTMI block."]
    pub mod EN_CLK_UTMI {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Misc Register"]
pub mod USB1_MISC_CLR {
    pub use crate::RW as access;
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    pub mod HS_USE_EXTERNAL_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    pub mod EN_DEGLITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the clk to the UTMI block."]
    pub mod EN_CLK_UTMI {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "USB Misc Register"]
pub mod USB1_MISC_TOG {
    pub use crate::RW as access;
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    pub mod HS_USE_EXTERNAL_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    pub mod EN_DEGLITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the clk to the UTMI block."]
    pub mod EN_CLK_UTMI {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Chip Silicon Version"]
pub mod DIGPROG {
    pub use crate::RO as access;
    #[doc = "Chip silicon revision"]
    pub mod SILICON_REVISION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Silicon revision 1.0"]
            pub const SILICON_REVISION_7012352: u32 = 0x006b_0000;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
