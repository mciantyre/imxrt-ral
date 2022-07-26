#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBPHY
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// USB PHY Power-Down Register
pub mod PWD {

    /// TXPWDFS
    pub mod TXPWDFS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the drivers into high-impedance output
            pub const PWR_DOWN: u32 = 0b1;
        }
    }

    /// TXPWDIBIAS
    pub mod TXPWDIBIAS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the USB is in suspend mode. This effectively powers down the entire USB transmit path
            pub const PWR_DOWN: u32 = 0b1;
        }
    }

    /// TXPWDV2I
    pub mod TXPWDV2I {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Power-down the USB PHY transmit V-to-I converter and the current mirror
            pub const PWR_DOWN: u32 = 0b1;
        }
    }

    /// RXPWDENV
    pub mod RXPWDENV {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Power-down the USB high-speed receiver envelope detector (squelch signal)
            pub const PWR_DOWN: u32 = 0b1;
        }
    }

    /// RXPWD1PT1
    pub mod RXPWD1PT1 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Power-down the USB full-speed differential receiver.
            pub const PWR_DOWN: u32 = 0b1;
        }
    }

    /// RXPWDDIFF
    pub mod RXPWDDIFF {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Power-down the USB high-speed differential receiver
            pub const PWR_DOWN: u32 = 0b1;
        }
    }

    /// RXPWDRX
    pub mod RXPWDRX {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Power-down the entire USB PHY receiver block except for the full-speed differential receiver
            pub const PWR_DOWN: u32 = 0b1;
        }
    }
}

/// USB PHY Power-Down Register
pub mod PWD_SET {

    /// TXPWDFS
    pub mod TXPWDFS {
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

    /// TXPWDIBIAS
    pub mod TXPWDIBIAS {
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

    /// TXPWDV2I
    pub mod TXPWDV2I {
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

    /// RXPWDENV
    pub mod RXPWDENV {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXPWD1PT1
    pub mod RXPWD1PT1 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXPWDDIFF
    pub mod RXPWDDIFF {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXPWDRX
    pub mod RXPWDRX {
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

/// USB PHY Power-Down Register
pub mod PWD_CLR {
    pub use super::PWD_SET::RXPWD1PT1;
    pub use super::PWD_SET::RXPWDDIFF;
    pub use super::PWD_SET::RXPWDENV;
    pub use super::PWD_SET::RXPWDRX;
    pub use super::PWD_SET::TXPWDFS;
    pub use super::PWD_SET::TXPWDIBIAS;
    pub use super::PWD_SET::TXPWDV2I;
}

/// USB PHY Power-Down Register
pub mod PWD_TOG {
    pub use super::PWD_SET::RXPWD1PT1;
    pub use super::PWD_SET::RXPWDDIFF;
    pub use super::PWD_SET::RXPWDENV;
    pub use super::PWD_SET::RXPWDRX;
    pub use super::PWD_SET::TXPWDFS;
    pub use super::PWD_SET::TXPWDIBIAS;
    pub use super::PWD_SET::TXPWDV2I;
}

/// USB PHY Transmitter Control Register
pub mod TX {

    /// D_CAL
    pub mod D_CAL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Maximum current, approximately 19% above nominal.
            pub const MAX: u32 = 0b0000;

            /// 0b0111: Nominal
            pub const NOMINAL: u32 = 0b0111;

            /// 0b1111: Minimum current, approximately 19% below nominal.
            pub const MIN: u32 = 0b1111;
        }
    }

    /// TXCAL45DN
    pub mod TXCAL45DN {
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

    /// TXCAL45DP
    pub mod TXCAL45DP {
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
}

/// USB PHY Transmitter Control Register
pub mod TX_SET {

    /// D_CAL
    pub mod D_CAL {
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

    /// TXCAL45DN
    pub mod TXCAL45DN {
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

    /// TXCAL45DP
    pub mod TXCAL45DP {
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
}

/// USB PHY Transmitter Control Register
pub mod TX_CLR {
    pub use super::TX_SET::D_CAL;
    pub use super::TX_SET::TXCAL45DN;
    pub use super::TX_SET::TXCAL45DP;
}

/// USB PHY Transmitter Control Register
pub mod TX_TOG {
    pub use super::TX_SET::D_CAL;
    pub use super::TX_SET::TXCAL45DN;
    pub use super::TX_SET::TXCAL45DP;
}

/// USB PHY Receiver Control Register
pub mod RX {

    /// ENVADJ
    pub mod ENVADJ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Trip-Level Voltage is 0.1000 V
            pub const LVL_P1: u32 = 0b000;

            /// 0b001: Trip-Level Voltage is 0.1125 V
            pub const LVL_P1125: u32 = 0b001;

            /// 0b010: Trip-Level Voltage is 0.1250 V
            pub const LVL_P1250: u32 = 0b010;

            /// 0b011: Trip-Level Voltage is 0.0875 V
            pub const LVL_P0875: u32 = 0b011;
        }
    }

    /// DISCONADJ
    pub mod DISCONADJ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Trip-Level Voltage is 0.56875 V
            pub const LVL_P56875: u32 = 0b000;

            /// 0b001: Trip-Level Voltage is 0.55000 V
            pub const LVL_P55: u32 = 0b001;

            /// 0b010: Trip-Level Voltage is 0.58125 V
            pub const LVL_P58125: u32 = 0b010;

            /// 0b011: Trip-Level Voltage is 0.60000 V
            pub const LVL_P6: u32 = 0b011;
        }
    }

    /// RXDBYPASS
    pub mod RXDBYPASS {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver
            pub const OUT_SINGLE_END: u32 = 0b1;
        }
    }
}

/// USB PHY Receiver Control Register
pub mod RX_SET {

    /// ENVADJ
    pub mod ENVADJ {
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

    /// DISCONADJ
    pub mod DISCONADJ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXDBYPASS
    pub mod RXDBYPASS {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB PHY Receiver Control Register
pub mod RX_CLR {
    pub use super::RX_SET::DISCONADJ;
    pub use super::RX_SET::ENVADJ;
    pub use super::RX_SET::RXDBYPASS;
}

/// USB PHY Receiver Control Register
pub mod RX_TOG {
    pub use super::RX_SET::DISCONADJ;
    pub use super::RX_SET::ENVADJ;
    pub use super::RX_SET::RXDBYPASS;
}

/// USB PHY General Control Register
pub mod CTRL {

    /// ENOTG_ID_CHG_IRQ
    pub mod ENOTG_ID_CHG_IRQ {
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

    /// ENHOSTDISCONDETECT
    pub mod ENHOSTDISCONDETECT {
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

    /// ENIRQHOSTDISCON
    pub mod ENIRQHOSTDISCON {
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

    /// HOSTDISCONDETECT_IRQ
    pub mod HOSTDISCONDETECT_IRQ {
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

    /// Enables non-standard resistive plugged-in detection
    pub mod ENDEVPLUGINDETECT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disables 200kohm pullup resistors on DP and DN pins
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enables 200kohm pullup resistors on DP and DN pins
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DEVPLUGIN_POLARITY
    pub mod DEVPLUGIN_POLARITY {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTG_ID_CHG_IRQ
    pub mod OTG_ID_CHG_IRQ {
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

    /// ENOTGIDDETECT
    pub mod ENOTGIDDETECT {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RESUMEIRQSTICKY
    pub mod RESUMEIRQSTICKY {
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

    /// ENIRQRESUMEDETECT
    pub mod ENIRQRESUMEDETECT {
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

    /// RESUME_IRQ
    pub mod RESUME_IRQ {
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

    /// ENIRQDEVPLUGIN
    pub mod ENIRQDEVPLUGIN {
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

    /// DEVPLUGIN_IRQ
    pub mod DEVPLUGIN_IRQ {
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

    /// ENUTMILEVEL2
    pub mod ENUTMILEVEL2 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENUTMILEVEL3
    pub mod ENUTMILEVEL3 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENIRQWAKEUP
    pub mod ENIRQWAKEUP {
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

    /// WAKEUP_IRQ
    pub mod WAKEUP_IRQ {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AUTORESUME_EN
    pub mod AUTORESUME_EN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENAUTOCLR_CLKGATE
    pub mod ENAUTOCLR_CLKGATE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENAUTOCLR_PHY_PWD
    pub mod ENAUTOCLR_PHY_PWD {
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

    /// ENDPDMCHG_WKUP
    pub mod ENDPDMCHG_WKUP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENIDCHG_WKUP
    pub mod ENIDCHG_WKUP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENVBUSCHG_WKUP
    pub mod ENVBUSCHG_WKUP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FSDLL_RST_EN
    pub mod FSDLL_RST_EN {
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

    /// OTG_ID_VALUE
    pub mod OTG_ID_VALUE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HOST_FORCE_LS_SE0
    pub mod HOST_FORCE_LS_SE0 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UTMI_SUSPENDM
    pub mod UTMI_SUSPENDM {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CLKGATE
    pub mod CLKGATE {
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

    /// SFTRST
    pub mod SFTRST {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB PHY General Control Register
pub mod CTRL_SET {

    /// ENOTG_ID_CHG_IRQ
    pub mod ENOTG_ID_CHG_IRQ {
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

    /// ENHOSTDISCONDETECT
    pub mod ENHOSTDISCONDETECT {
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

    /// ENIRQHOSTDISCON
    pub mod ENIRQHOSTDISCON {
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

    /// HOSTDISCONDETECT_IRQ
    pub mod HOSTDISCONDETECT_IRQ {
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

    /// Enables non-standard resistive plugged-in detection
    pub mod ENDEVPLUGINDETECT {
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

    /// DEVPLUGIN_POLARITY
    pub mod DEVPLUGIN_POLARITY {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTG_ID_CHG_IRQ
    pub mod OTG_ID_CHG_IRQ {
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

    /// ENOTGIDDETECT
    pub mod ENOTGIDDETECT {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RESUMEIRQSTICKY
    pub mod RESUMEIRQSTICKY {
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

    /// ENIRQRESUMEDETECT
    pub mod ENIRQRESUMEDETECT {
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

    /// RESUME_IRQ
    pub mod RESUME_IRQ {
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

    /// ENIRQDEVPLUGIN
    pub mod ENIRQDEVPLUGIN {
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

    /// DEVPLUGIN_IRQ
    pub mod DEVPLUGIN_IRQ {
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

    /// ENUTMILEVEL2
    pub mod ENUTMILEVEL2 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENUTMILEVEL3
    pub mod ENUTMILEVEL3 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENIRQWAKEUP
    pub mod ENIRQWAKEUP {
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

    /// WAKEUP_IRQ
    pub mod WAKEUP_IRQ {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AUTORESUME_EN
    pub mod AUTORESUME_EN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENAUTOCLR_CLKGATE
    pub mod ENAUTOCLR_CLKGATE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENAUTOCLR_PHY_PWD
    pub mod ENAUTOCLR_PHY_PWD {
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

    /// ENDPDMCHG_WKUP
    pub mod ENDPDMCHG_WKUP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENIDCHG_WKUP
    pub mod ENIDCHG_WKUP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENVBUSCHG_WKUP
    pub mod ENVBUSCHG_WKUP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FSDLL_RST_EN
    pub mod FSDLL_RST_EN {
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

    /// OTG_ID_VALUE
    pub mod OTG_ID_VALUE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HOST_FORCE_LS_SE0
    pub mod HOST_FORCE_LS_SE0 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UTMI_SUSPENDM
    pub mod UTMI_SUSPENDM {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CLKGATE
    pub mod CLKGATE {
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

    /// SFTRST
    pub mod SFTRST {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB PHY General Control Register
pub mod CTRL_CLR {
    pub use super::CTRL_SET::AUTORESUME_EN;
    pub use super::CTRL_SET::CLKGATE;
    pub use super::CTRL_SET::DEVPLUGIN_IRQ;
    pub use super::CTRL_SET::DEVPLUGIN_POLARITY;
    pub use super::CTRL_SET::ENAUTOCLR_CLKGATE;
    pub use super::CTRL_SET::ENAUTOCLR_PHY_PWD;
    pub use super::CTRL_SET::ENDEVPLUGINDETECT;
    pub use super::CTRL_SET::ENDPDMCHG_WKUP;
    pub use super::CTRL_SET::ENHOSTDISCONDETECT;
    pub use super::CTRL_SET::ENIDCHG_WKUP;
    pub use super::CTRL_SET::ENIRQDEVPLUGIN;
    pub use super::CTRL_SET::ENIRQHOSTDISCON;
    pub use super::CTRL_SET::ENIRQRESUMEDETECT;
    pub use super::CTRL_SET::ENIRQWAKEUP;
    pub use super::CTRL_SET::ENOTGIDDETECT;
    pub use super::CTRL_SET::ENOTG_ID_CHG_IRQ;
    pub use super::CTRL_SET::ENUTMILEVEL2;
    pub use super::CTRL_SET::ENUTMILEVEL3;
    pub use super::CTRL_SET::ENVBUSCHG_WKUP;
    pub use super::CTRL_SET::FSDLL_RST_EN;
    pub use super::CTRL_SET::HOSTDISCONDETECT_IRQ;
    pub use super::CTRL_SET::HOST_FORCE_LS_SE0;
    pub use super::CTRL_SET::OTG_ID_CHG_IRQ;
    pub use super::CTRL_SET::OTG_ID_VALUE;
    pub use super::CTRL_SET::RESUMEIRQSTICKY;
    pub use super::CTRL_SET::RESUME_IRQ;
    pub use super::CTRL_SET::SFTRST;
    pub use super::CTRL_SET::UTMI_SUSPENDM;
    pub use super::CTRL_SET::WAKEUP_IRQ;
}

/// USB PHY General Control Register
pub mod CTRL_TOG {
    pub use super::CTRL_SET::AUTORESUME_EN;
    pub use super::CTRL_SET::CLKGATE;
    pub use super::CTRL_SET::DEVPLUGIN_IRQ;
    pub use super::CTRL_SET::DEVPLUGIN_POLARITY;
    pub use super::CTRL_SET::ENAUTOCLR_CLKGATE;
    pub use super::CTRL_SET::ENAUTOCLR_PHY_PWD;
    pub use super::CTRL_SET::ENDEVPLUGINDETECT;
    pub use super::CTRL_SET::ENDPDMCHG_WKUP;
    pub use super::CTRL_SET::ENHOSTDISCONDETECT;
    pub use super::CTRL_SET::ENIDCHG_WKUP;
    pub use super::CTRL_SET::ENIRQDEVPLUGIN;
    pub use super::CTRL_SET::ENIRQHOSTDISCON;
    pub use super::CTRL_SET::ENIRQRESUMEDETECT;
    pub use super::CTRL_SET::ENIRQWAKEUP;
    pub use super::CTRL_SET::ENOTGIDDETECT;
    pub use super::CTRL_SET::ENOTG_ID_CHG_IRQ;
    pub use super::CTRL_SET::ENUTMILEVEL2;
    pub use super::CTRL_SET::ENUTMILEVEL3;
    pub use super::CTRL_SET::ENVBUSCHG_WKUP;
    pub use super::CTRL_SET::FSDLL_RST_EN;
    pub use super::CTRL_SET::HOSTDISCONDETECT_IRQ;
    pub use super::CTRL_SET::HOST_FORCE_LS_SE0;
    pub use super::CTRL_SET::OTG_ID_CHG_IRQ;
    pub use super::CTRL_SET::OTG_ID_VALUE;
    pub use super::CTRL_SET::RESUMEIRQSTICKY;
    pub use super::CTRL_SET::RESUME_IRQ;
    pub use super::CTRL_SET::SFTRST;
    pub use super::CTRL_SET::UTMI_SUSPENDM;
    pub use super::CTRL_SET::WAKEUP_IRQ;
}

/// USB PHY Status Register
pub mod STATUS {

    /// HOSTDISCONDETECT_STATUS
    pub mod HOSTDISCONDETECT_STATUS {
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

            /// 0b0: USB cable disconnect has not been detected at the local host
            pub const NOT_DET: u32 = 0b0;

            /// 0b1: USB cable disconnect has been detected at the local host
            pub const DET: u32 = 0b1;
        }
    }

    /// Status indicator for non-standard resistive plugged-in detection
    pub mod DEVPLUGIN_STATUS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No attachment to a USB host is detected
            pub const NO_ATTACH: u32 = 0b0;

            /// 0b1: Cable attachment to a USB host is detected
            pub const ATTACH: u32 = 0b1;
        }
    }

    /// OTGID_STATUS
    pub mod OTGID_STATUS {
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

    /// RESUME_STATUS
    pub mod RESUME_STATUS {
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
}

/// USB PHY Debug Register
pub mod DEBUG {

    /// OTGIDPIOLOCK
    pub mod OTGIDPIOLOCK {
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

    /// DEBUG_INTERFACE_HOLD
    pub mod DEBUG_INTERFACE_HOLD {
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

    /// HSTPULLDOWN
    pub mod HSTPULLDOWN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENHSTPULLDOWN
    pub mod ENHSTPULLDOWN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX2RXCOUNT
    pub mod TX2RXCOUNT {
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

    /// ENTX2RXCOUNT
    pub mod ENTX2RXCOUNT {
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

    /// SQUELCHRESETCOUNT
    pub mod SQUELCHRESETCOUNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENSQUELCHRESET
    pub mod ENSQUELCHRESET {
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

    /// SQUELCHRESETLENGTH
    pub mod SQUELCHRESETLENGTH {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (4 bits: 0b1111 << 25)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HOST_RESUME_DEBUG
    pub mod HOST_RESUME_DEBUG {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CLKGATE
    pub mod CLKGATE {
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

/// USB PHY Debug Register
pub mod DEBUG_SET {
    pub use super::DEBUG::CLKGATE;
    pub use super::DEBUG::DEBUG_INTERFACE_HOLD;
    pub use super::DEBUG::ENHSTPULLDOWN;
    pub use super::DEBUG::ENSQUELCHRESET;
    pub use super::DEBUG::ENTX2RXCOUNT;
    pub use super::DEBUG::HOST_RESUME_DEBUG;
    pub use super::DEBUG::HSTPULLDOWN;
    pub use super::DEBUG::OTGIDPIOLOCK;
    pub use super::DEBUG::SQUELCHRESETCOUNT;
    pub use super::DEBUG::SQUELCHRESETLENGTH;
    pub use super::DEBUG::TX2RXCOUNT;
}

/// USB PHY Debug Register
pub mod DEBUG_CLR {
    pub use super::DEBUG::CLKGATE;
    pub use super::DEBUG::DEBUG_INTERFACE_HOLD;
    pub use super::DEBUG::ENHSTPULLDOWN;
    pub use super::DEBUG::ENSQUELCHRESET;
    pub use super::DEBUG::ENTX2RXCOUNT;
    pub use super::DEBUG::HOST_RESUME_DEBUG;
    pub use super::DEBUG::HSTPULLDOWN;
    pub use super::DEBUG::OTGIDPIOLOCK;
    pub use super::DEBUG::SQUELCHRESETCOUNT;
    pub use super::DEBUG::SQUELCHRESETLENGTH;
    pub use super::DEBUG::TX2RXCOUNT;
}

/// USB PHY Debug Register
pub mod DEBUG_TOG {
    pub use super::DEBUG::CLKGATE;
    pub use super::DEBUG::DEBUG_INTERFACE_HOLD;
    pub use super::DEBUG::ENHSTPULLDOWN;
    pub use super::DEBUG::ENSQUELCHRESET;
    pub use super::DEBUG::ENTX2RXCOUNT;
    pub use super::DEBUG::HOST_RESUME_DEBUG;
    pub use super::DEBUG::HSTPULLDOWN;
    pub use super::DEBUG::OTGIDPIOLOCK;
    pub use super::DEBUG::SQUELCHRESETCOUNT;
    pub use super::DEBUG::SQUELCHRESETLENGTH;
    pub use super::DEBUG::TX2RXCOUNT;
}

/// UTMI Debug Status Register 0
pub mod DEBUG0_STATUS {

    /// LOOP_BACK_FAIL_COUNT
    pub mod LOOP_BACK_FAIL_COUNT {
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

    /// UTMI_RXERROR_FAIL_COUNT
    pub mod UTMI_RXERROR_FAIL_COUNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SQUELCH_COUNT
    pub mod SQUELCH_COUNT {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (6 bits: 0x3f << 26)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// UTMI Debug Status Register 1
pub mod DEBUG1 {

    /// ENTAILADJVD
    pub mod ENTAILADJVD {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Delay is nominal
            pub const NOM_DELAY: u32 = 0b00;

            /// 0b01: Delay is +20%
            pub const DELAY_20_P: u32 = 0b01;

            /// 0b10: Delay is -20%
            pub const DELAY_20_N: u32 = 0b10;

            /// 0b11: Delay is -40%
            pub const DELAY_40_N: u32 = 0b11;
        }
    }

    /// Set to 1 to disable self bias, 100 us after power up refbias(usb2_refbias_pwd).This can reduce noise on power.
    pub mod USB2_REFBIAS_SELFBIASOFF {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Powers down the bandgap detect logic, will affect vbgup on misc1 register.
    pub mod USB2_REFBIAS_PWDVBGUP {
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

    /// to be added
    pub mod USB2_REFBIAS_LOWPWR {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Adjustment bits on bandgap
    pub mod USB2_REFBIAS_VBGADJ {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bias current control for usb2_phy
    pub mod USB2_REFBIAS_TST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// UTMI Debug Status Register 1
pub mod DEBUG1_SET {

    /// ENTAILADJVD
    pub mod ENTAILADJVD {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set to 1 to disable self bias, 100 us after power up refbias(usb2_refbias_pwd).This can reduce noise on power.
    pub mod USB2_REFBIAS_SELFBIASOFF {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Powers down the bandgap detect logic, will affect vbgup on misc1 register.
    pub mod USB2_REFBIAS_PWDVBGUP {
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

    /// to be added
    pub mod USB2_REFBIAS_LOWPWR {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Adjustment bits on bandgap
    pub mod USB2_REFBIAS_VBGADJ {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bias current control for usb2_phy
    pub mod USB2_REFBIAS_TST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// UTMI Debug Status Register 1
pub mod DEBUG1_CLR {
    pub use super::DEBUG1_SET::ENTAILADJVD;
    pub use super::DEBUG1_SET::USB2_REFBIAS_LOWPWR;
    pub use super::DEBUG1_SET::USB2_REFBIAS_PWDVBGUP;
    pub use super::DEBUG1_SET::USB2_REFBIAS_SELFBIASOFF;
    pub use super::DEBUG1_SET::USB2_REFBIAS_TST;
    pub use super::DEBUG1_SET::USB2_REFBIAS_VBGADJ;
}

/// UTMI Debug Status Register 1
pub mod DEBUG1_TOG {
    pub use super::DEBUG1_SET::ENTAILADJVD;
    pub use super::DEBUG1_SET::USB2_REFBIAS_LOWPWR;
    pub use super::DEBUG1_SET::USB2_REFBIAS_PWDVBGUP;
    pub use super::DEBUG1_SET::USB2_REFBIAS_SELFBIASOFF;
    pub use super::DEBUG1_SET::USB2_REFBIAS_TST;
    pub use super::DEBUG1_SET::USB2_REFBIAS_VBGADJ;
}

/// UTMI RTL Version
pub mod VERSION {

    /// STEP
    pub mod STEP {
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

    /// MINOR
    pub mod MINOR {
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

    /// MAJOR
    pub mod MAJOR {
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

/// USB PHY PLL Control/Status Register
pub mod PLL_SIC {

    /// PLL_POSTDIV
    pub mod PLL_POSTDIV {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_EN_USB_CLKS
    pub mod PLL_EN_USB_CLKS {
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

    /// PLL_POWER
    pub mod PLL_POWER {
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

    /// PLL_ENABLE
    pub mod PLL_ENABLE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_BYPASS
    pub mod PLL_BYPASS {
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

    /// REFBIAS_PWD_SEL
    pub mod REFBIAS_PWD_SEL {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selects PLL_POWER to control the reference bias
            pub const PLL_PWR: u32 = 0b0;

            /// 0b1: Selects REFBIAS_PWD to control the reference bias.
            pub const REFBIAS_PWD: u32 = 0b1;
        }
    }

    /// Power down the reference bias
    pub mod REFBIAS_PWD {
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

    /// PLL_REG_ENABLE
    pub mod PLL_REG_ENABLE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_DIV_SEL
    pub mod PLL_DIV_SEL {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (3 bits: 0b111 << 22)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Divide by 13
            pub const DIV_BY_13: u32 = 0b000;

            /// 0b001: Divide by 15
            pub const DIV_BY_15: u32 = 0b001;

            /// 0b010: Divide by 16
            pub const DIV_BY_16: u32 = 0b010;

            /// 0b011: Divide by 20
            pub const DIV_BY_20: u32 = 0b011;

            /// 0b100: Divide by 22
            pub const DIV_BY_22: u32 = 0b100;

            /// 0b101: Divide by 25
            pub const DIV_BY_25: u32 = 0b101;

            /// 0b110: Divide by 30
            pub const DIV_BY_30: u32 = 0b110;

            /// 0b111: Divide by 240
            pub const DIV_BY_240: u32 = 0b111;
        }
    }

    /// PLL_LOCK
    pub mod PLL_LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLL is not currently locked
            pub const NOT_LOCKED: u32 = 0b0;

            /// 0b1: PLL is currently locked
            pub const LOCKED: u32 = 0b1;
        }
    }
}

/// USB PHY PLL Control/Status Register
pub mod PLL_SIC_SET {

    /// PLL_POSTDIV
    pub mod PLL_POSTDIV {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_EN_USB_CLKS
    pub mod PLL_EN_USB_CLKS {
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

    /// PLL_POWER
    pub mod PLL_POWER {
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

    /// PLL_ENABLE
    pub mod PLL_ENABLE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_BYPASS
    pub mod PLL_BYPASS {
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

    /// REFBIAS_PWD_SEL
    pub mod REFBIAS_PWD_SEL {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power down the reference bias
    pub mod REFBIAS_PWD {
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

    /// PLL_REG_ENABLE
    pub mod PLL_REG_ENABLE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_DIV_SEL
    pub mod PLL_DIV_SEL {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (3 bits: 0b111 << 22)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_LOCK
    pub mod PLL_LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB PHY PLL Control/Status Register
pub mod PLL_SIC_CLR {
    pub use super::PLL_SIC_SET::PLL_BYPASS;
    pub use super::PLL_SIC_SET::PLL_DIV_SEL;
    pub use super::PLL_SIC_SET::PLL_ENABLE;
    pub use super::PLL_SIC_SET::PLL_EN_USB_CLKS;
    pub use super::PLL_SIC_SET::PLL_LOCK;
    pub use super::PLL_SIC_SET::PLL_POSTDIV;
    pub use super::PLL_SIC_SET::PLL_POWER;
    pub use super::PLL_SIC_SET::PLL_REG_ENABLE;
    pub use super::PLL_SIC_SET::REFBIAS_PWD;
    pub use super::PLL_SIC_SET::REFBIAS_PWD_SEL;
}

/// USB PHY PLL Control/Status Register
pub mod PLL_SIC_TOG {
    pub use super::PLL_SIC_SET::PLL_BYPASS;
    pub use super::PLL_SIC_SET::PLL_DIV_SEL;
    pub use super::PLL_SIC_SET::PLL_ENABLE;
    pub use super::PLL_SIC_SET::PLL_EN_USB_CLKS;
    pub use super::PLL_SIC_SET::PLL_LOCK;
    pub use super::PLL_SIC_SET::PLL_POSTDIV;
    pub use super::PLL_SIC_SET::PLL_POWER;
    pub use super::PLL_SIC_SET::PLL_REG_ENABLE;
    pub use super::PLL_SIC_SET::REFBIAS_PWD;
    pub use super::PLL_SIC_SET::REFBIAS_PWD_SEL;
}

/// USB PHY VBUS Detect Control Register
pub mod USB1_VBUS_DETECT {

    /// VBUSVALID_THRESH
    pub mod VBUSVALID_THRESH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 4.0 V
            pub const VOLT_4: u32 = 0b000;

            /// 0b001: 4.1 V
            pub const VOLT_4P1: u32 = 0b001;

            /// 0b010: 4.2 V
            pub const VOLT_4P2: u32 = 0b010;

            /// 0b011: 4.3 V
            pub const VOLT_4P3: u32 = 0b011;

            /// 0b100: 4.4 V (Default)
            pub const VOLT_4P4: u32 = 0b100;

            /// 0b101: 4.5 V
            pub const VOLT_4P5: u32 = 0b101;

            /// 0b110: 4.6 V
            pub const VOLT_4P6: u32 = 0b110;

            /// 0b111: 4.7 V
            pub const VOLT_4P7: u32 = 0b111;
        }
    }

    /// VBUS detect signal override enable
    pub mod VBUS_OVERRIDE_EN {
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

            /// 0b0: Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)
            pub const INTERNAL: u32 = 0b0;

            /// 0b1: Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND
            pub const OVERRIDE: u32 = 0b1;
        }
    }

    /// Override value for SESSEND
    pub mod SESSEND_OVERRIDE {
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

    /// Override value for B-Device Session Valid
    pub mod BVALID_OVERRIDE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Override value for A-Device Session Valid
    pub mod AVALID_OVERRIDE {
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

    /// Override value for VBUS_VALID signal sent to USB controller
    pub mod VBUSVALID_OVERRIDE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Selects the source of the VBUS_VALID signal reported to the USB controller
    pub mod VBUSVALID_SEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)
            pub const COMP: u32 = 0b0;

            /// 0b1: Use the VBUS_VALID_3V detector results for signal reported to the USB controller
            pub const DET_3V: u32 = 0b1;
        }
    }

    /// Selects the source of the VBUS_VALID signal reported to the USB controller
    pub mod VBUS_SOURCE_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)
            pub const VBUS_VALID_COMP: u32 = 0b00;

            /// 0b01: Use the Session Valid comparator results for signal reported to the USB controller
            pub const SESSION_VALID_COMP: u32 = 0b01;

            /// 0b10: Use the Session Valid comparator results for signal reported to the USB controller
            pub const SESSION_VALID_COMP_1: u32 = 0b10;
        }
    }

    /// TBA
    pub mod ID_OVERRIDE_EN {
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

    /// TBA
    pub mod ID_OVERRIDE {
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

    /// Selects the comparator used for VBUS_VALID
    pub mod VBUSVALID_TO_SESSVALID {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Use the VBUS_VALID comparator for VBUS_VALID results
            pub const VBUS_VALID: u32 = 0b0;

            /// 0b1: Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V.
            pub const SESSION_VALID: u32 = 0b1;
        }
    }

    /// Enables the VBUS_VALID comparator
    pub mod PWRUP_CMPS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Powers down the VBUS_VALID comparator
            pub const DISABLE: u32 = 0b000;

            /// 0b001: Enables the SESS_VALID comparator (default)
            pub const ENABLE: u32 = 0b001;

            /// 0b010: Enables the 3Vdetect (default)
            pub const VDETECT: u32 = 0b010;
        }
    }

    /// Controls VBUS discharge resistor
    pub mod DISCHARGE_VBUS {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VBUS discharge resistor is disabled (Default)
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VBUS discharge resistor is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enables resistors used for an older method of resistive battery charger detection
    pub mod EN_CHARGER_RESISTOR {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable resistive charger detection resistors on DP and DP
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable resistive charger detection resistors on DP and DP
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// USB PHY VBUS Detect Control Register
pub mod USB1_VBUS_DETECT_SET {

    /// VBUSVALID_THRESH
    pub mod VBUSVALID_THRESH {
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

    /// VBUS detect signal override enable
    pub mod VBUS_OVERRIDE_EN {
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

    /// Override value for SESSEND
    pub mod SESSEND_OVERRIDE {
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

    /// Override value for B-Device Session Valid
    pub mod BVALID_OVERRIDE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Override value for A-Device Session Valid
    pub mod AVALID_OVERRIDE {
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

    /// Override value for VBUS_VALID signal sent to USB controller
    pub mod VBUSVALID_OVERRIDE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Selects the source of the VBUS_VALID signal reported to the USB controller
    pub mod VBUSVALID_SEL {
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

    /// Selects the source of the VBUS_VALID signal reported to the USB controller
    pub mod VBUS_SOURCE_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TBA
    pub mod ID_OVERRIDE_EN {
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

    /// TBA
    pub mod ID_OVERRIDE {
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

    /// Selects the comparator used for VBUS_VALID
    pub mod VBUSVALID_TO_SESSVALID {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enables the VBUS_VALID comparator
    pub mod PWRUP_CMPS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Controls VBUS discharge resistor
    pub mod DISCHARGE_VBUS {
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

    /// Enables resistors used for an older method of resistive battery charger detection
    pub mod EN_CHARGER_RESISTOR {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB PHY VBUS Detect Control Register
pub mod USB1_VBUS_DETECT_CLR {
    pub use super::USB1_VBUS_DETECT_SET::AVALID_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::BVALID_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::DISCHARGE_VBUS;
    pub use super::USB1_VBUS_DETECT_SET::EN_CHARGER_RESISTOR;
    pub use super::USB1_VBUS_DETECT_SET::ID_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::ID_OVERRIDE_EN;
    pub use super::USB1_VBUS_DETECT_SET::PWRUP_CMPS;
    pub use super::USB1_VBUS_DETECT_SET::SESSEND_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::VBUSVALID_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::VBUSVALID_SEL;
    pub use super::USB1_VBUS_DETECT_SET::VBUSVALID_THRESH;
    pub use super::USB1_VBUS_DETECT_SET::VBUSVALID_TO_SESSVALID;
    pub use super::USB1_VBUS_DETECT_SET::VBUS_OVERRIDE_EN;
    pub use super::USB1_VBUS_DETECT_SET::VBUS_SOURCE_SEL;
}

/// USB PHY VBUS Detect Control Register
pub mod USB1_VBUS_DETECT_TOG {
    pub use super::USB1_VBUS_DETECT_SET::AVALID_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::BVALID_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::DISCHARGE_VBUS;
    pub use super::USB1_VBUS_DETECT_SET::EN_CHARGER_RESISTOR;
    pub use super::USB1_VBUS_DETECT_SET::ID_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::ID_OVERRIDE_EN;
    pub use super::USB1_VBUS_DETECT_SET::PWRUP_CMPS;
    pub use super::USB1_VBUS_DETECT_SET::SESSEND_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::VBUSVALID_OVERRIDE;
    pub use super::USB1_VBUS_DETECT_SET::VBUSVALID_SEL;
    pub use super::USB1_VBUS_DETECT_SET::VBUSVALID_THRESH;
    pub use super::USB1_VBUS_DETECT_SET::VBUSVALID_TO_SESSVALID;
    pub use super::USB1_VBUS_DETECT_SET::VBUS_OVERRIDE_EN;
    pub use super::USB1_VBUS_DETECT_SET::VBUS_SOURCE_SEL;
}

/// USB PHY VBUS Detector Status Register
pub mod USB1_VBUS_DET_STAT {

    /// Session End indicator
    pub mod SESSEND {
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

            /// 0b0: The VBUS voltage is above the Session Valid threshold
            pub const ABOVE: u32 = 0b0;

            /// 0b1: The VBUS voltage is below the Session Valid threshold
            pub const BELOW: u32 = 0b1;
        }
    }

    /// B-Device Session Valid status
    pub mod BVALID {
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

            /// 0b0: The VBUS voltage is below the Session Valid threshold
            pub const BELOW: u32 = 0b0;

            /// 0b1: The VBUS voltage is above the Session Valid threshold
            pub const ABOVE: u32 = 0b1;
        }
    }

    /// A-Device Session Valid status
    pub mod AVALID {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BVALID::RW;
    }

    /// VBUS voltage status
    pub mod VBUS_VALID {
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

            /// 0b0: VBUS is below the comparator threshold
            pub const BELOW: u32 = 0b0;

            /// 0b1: VBUS is above the comparator threshold
            pub const ABOVE: u32 = 0b1;
        }
    }

    /// VBUS_VALID_3V detector status
    pub mod VBUS_VALID_3V {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VBUS voltage is below VBUS_VALID_3V threshold
            pub const BELOW: u32 = 0b0;

            /// 0b1: VBUS voltage is above VBUS_VALID_3V threshold
            pub const ABOVE: u32 = 0b1;
        }
    }
}

/// USB PHY Charger Detect Control Register
pub mod USB1_CHRG_DETECT {

    /// PULLUP_DP
    pub mod PULLUP_DP {
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

    /// BGR_BIAS
    pub mod BGR_BIAS {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Use local bias powered from USB1_VBUS for 10uA reference (Default)
            pub const LOCAL_BIAS: u32 = 0b0;

            /// 0b1: Use bandgap bias powered from VREGIN0/VREGIN1 for 10uA reference
            pub const BANDGAP: u32 = 0b1;
        }
    }
}

/// USB PHY Charger Detect Control Register
pub mod USB1_CHRG_DETECT_SET {

    /// PULLUP_DP
    pub mod PULLUP_DP {
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

    /// BGR_BIAS
    pub mod BGR_BIAS {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB PHY Charger Detect Control Register
pub mod USB1_CHRG_DETECT_CLR {
    pub use super::USB1_CHRG_DETECT_SET::BGR_BIAS;
    pub use super::USB1_CHRG_DETECT_SET::PULLUP_DP;
}

/// USB PHY Charger Detect Control Register
pub mod USB1_CHRG_DETECT_TOG {
    pub use super::USB1_CHRG_DETECT_SET::BGR_BIAS;
    pub use super::USB1_CHRG_DETECT_SET::PULLUP_DP;
}

/// USB PHY Charger Detect Status Register
pub mod USB1_CHRG_DET_STAT {

    /// Battery Charging Data Contact Detection phase output
    pub mod PLUG_CONTACT {
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

            /// 0b0: No USB cable attachment has been detected
            pub const NO_ATTACH: u32 = 0b0;

            /// 0b1: A USB cable attachment between the device and host has been detected
            pub const ATTACH: u32 = 0b1;
        }
    }

    /// Battery Charging Primary Detection phase output
    pub mod CHRG_DETECTED {
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

            /// 0b0: Standard Downstream Port (SDP) has been detected
            pub const SDP: u32 = 0b0;

            /// 0b1: Charging Port has been detected
            pub const CHRG_PORT: u32 = 0b1;
        }
    }

    /// DN_STATE
    pub mod DN_STATE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DN pin voltage is < 0.8V
            pub const BELOW_P8: u32 = 0b0;

            /// 0b1: DN pin voltage is > 2.0V
            pub const ABOVE_2: u32 = 0b1;
        }
    }

    /// DP_STATE
    pub mod DP_STATE {
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

            /// 0b0: DP pin voltage is < 0.8V
            pub const BELOW_P8: u32 = 0b0;

            /// 0b1: DP pin voltage is > 2.0V
            pub const ABOVE_2: u32 = 0b1;
        }
    }

    /// Battery Charging Secondary Detection phase output
    pub mod SECDET_DCP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Charging Downstream Port (CDP) has been detected
            pub const CDP: u32 = 0b0;

            /// 0b1: Downstream Charging Port (DCP) has been detected
            pub const DCP: u32 = 0b1;
        }
    }
}

/// USB PHY Analog Control Register
pub mod ANACTRL {

    /// DEV_PULLDOWN
    pub mod DEV_PULLDOWN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The 15kohm nominal pulldowns on the DP and DN pinsare disabled in device mode.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The 15kohm nominal pulldowns on the DP and DN pinsare enabled in device mode.
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// USB PHY Analog Control Register
pub mod ANACTRL_SET {

    /// DEV_PULLDOWN
    pub mod DEV_PULLDOWN {
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
}

/// USB PHY Analog Control Register
pub mod ANACTRL_CLR {
    pub use super::ANACTRL_SET::DEV_PULLDOWN;
}

/// USB PHY Analog Control Register
pub mod ANACTRL_TOG {
    pub use super::ANACTRL_SET::DEV_PULLDOWN;
}

/// USB PHY Loopback Control/Status Register
pub mod USB1_LOOPBACK {

    /// UTMI_TESTSTART
    pub mod UTMI_TESTSTART {
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

    /// UTMI_DIG_TST0
    pub mod UTMI_DIG_TST0 {
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

    /// UTMI_DIG_TST1
    pub mod UTMI_DIG_TST1 {
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

    /// TSTI_TX_HS_MODE
    pub mod TSTI_TX_HS_MODE {
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

    /// TSTI_TX_LS_MODE
    pub mod TSTI_TX_LS_MODE {
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

    /// TSTI_TX_EN
    pub mod TSTI_TX_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TSTI_TX_HIZ
    pub mod TSTI_TX_HIZ {
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

    /// UTMO_DIG_TST0
    pub mod UTMO_DIG_TST0 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UTMO_DIG_TST1
    pub mod UTMO_DIG_TST1 {
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

    /// TSTI_HSFS_MODE_EN
    pub mod TSTI_HSFS_MODE_EN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TSTPKT
    pub mod TSTPKT {
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
}

/// USB PHY Loopback Control/Status Register
pub mod USB1_LOOPBACK_SET {
    pub use super::USB1_LOOPBACK::TSTI_HSFS_MODE_EN;
    pub use super::USB1_LOOPBACK::TSTI_TX_EN;
    pub use super::USB1_LOOPBACK::TSTI_TX_HIZ;
    pub use super::USB1_LOOPBACK::TSTI_TX_HS_MODE;
    pub use super::USB1_LOOPBACK::TSTI_TX_LS_MODE;
    pub use super::USB1_LOOPBACK::TSTPKT;
    pub use super::USB1_LOOPBACK::UTMI_DIG_TST0;
    pub use super::USB1_LOOPBACK::UTMI_DIG_TST1;
    pub use super::USB1_LOOPBACK::UTMI_TESTSTART;
    pub use super::USB1_LOOPBACK::UTMO_DIG_TST0;
    pub use super::USB1_LOOPBACK::UTMO_DIG_TST1;
}

/// USB PHY Loopback Control/Status Register
pub mod USB1_LOOPBACK_CLR {
    pub use super::USB1_LOOPBACK::TSTI_HSFS_MODE_EN;
    pub use super::USB1_LOOPBACK::TSTI_TX_EN;
    pub use super::USB1_LOOPBACK::TSTI_TX_HIZ;
    pub use super::USB1_LOOPBACK::TSTI_TX_HS_MODE;
    pub use super::USB1_LOOPBACK::TSTI_TX_LS_MODE;
    pub use super::USB1_LOOPBACK::TSTPKT;
    pub use super::USB1_LOOPBACK::UTMI_DIG_TST0;
    pub use super::USB1_LOOPBACK::UTMI_DIG_TST1;
    pub use super::USB1_LOOPBACK::UTMI_TESTSTART;
    pub use super::USB1_LOOPBACK::UTMO_DIG_TST0;
    pub use super::USB1_LOOPBACK::UTMO_DIG_TST1;
}

/// USB PHY Loopback Control/Status Register
pub mod USB1_LOOPBACK_TOG {
    pub use super::USB1_LOOPBACK::TSTI_HSFS_MODE_EN;
    pub use super::USB1_LOOPBACK::TSTI_TX_EN;
    pub use super::USB1_LOOPBACK::TSTI_TX_HIZ;
    pub use super::USB1_LOOPBACK::TSTI_TX_HS_MODE;
    pub use super::USB1_LOOPBACK::TSTI_TX_LS_MODE;
    pub use super::USB1_LOOPBACK::TSTPKT;
    pub use super::USB1_LOOPBACK::UTMI_DIG_TST0;
    pub use super::USB1_LOOPBACK::UTMI_DIG_TST1;
    pub use super::USB1_LOOPBACK::UTMI_TESTSTART;
    pub use super::USB1_LOOPBACK::UTMO_DIG_TST0;
    pub use super::USB1_LOOPBACK::UTMO_DIG_TST1;
}

/// USB PHY Loopback Packet Number Select Register
pub mod USB1_LOOPBACK_HSFSCNT {

    /// TSTI_HS_NUMBER
    pub mod TSTI_HS_NUMBER {
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

    /// TSTI_FS_NUMBER
    pub mod TSTI_FS_NUMBER {
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

/// USB PHY Loopback Packet Number Select Register
pub mod USB1_LOOPBACK_HSFSCNT_SET {
    pub use super::USB1_LOOPBACK_HSFSCNT::TSTI_FS_NUMBER;
    pub use super::USB1_LOOPBACK_HSFSCNT::TSTI_HS_NUMBER;
}

/// USB PHY Loopback Packet Number Select Register
pub mod USB1_LOOPBACK_HSFSCNT_CLR {
    pub use super::USB1_LOOPBACK_HSFSCNT::TSTI_FS_NUMBER;
    pub use super::USB1_LOOPBACK_HSFSCNT::TSTI_HS_NUMBER;
}

/// USB PHY Loopback Packet Number Select Register
pub mod USB1_LOOPBACK_HSFSCNT_TOG {
    pub use super::USB1_LOOPBACK_HSFSCNT::TSTI_FS_NUMBER;
    pub use super::USB1_LOOPBACK_HSFSCNT::TSTI_HS_NUMBER;
}

/// USB PHY Trim Override Enable Register
pub mod TRIM_OVERRIDE_EN {

    /// TRIM_DIV_SEL_OVERRIDE
    pub mod TRIM_DIV_SEL_OVERRIDE {
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

    /// TRIM_ENV_TAIL_ADJ_VD_OVERRIDE
    pub mod TRIM_ENV_TAIL_ADJ_VD_OVERRIDE {
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

    /// TRIM_TX_D_CAL_OVERRIDE
    pub mod TRIM_TX_D_CAL_OVERRIDE {
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

    /// TRIM_TX_CAL45DP_OVERRIDE
    pub mod TRIM_TX_CAL45DP_OVERRIDE {
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

    /// TRIM_TX_CAL45DN_OVERRIDE
    pub mod TRIM_TX_CAL45DN_OVERRIDE {
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

    /// Override enable for bandgap adjustment.
    pub mod TRIM_REFBIAS_VBGADJ_OVERRIDE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Override enable for bias current control
    pub mod TRIM_REFBIAS_TST_OVERRIDE {
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

    /// TRIM_USB2_REFBIAS_VBGADJ
    pub mod TRIM_USB2_REFBIAS_VBGADJ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIM_USB2_REFBIAS_TST
    pub mod TRIM_USB2_REFBIAS_TST {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIM_PLL_CTRL0_DIV_SEL
    pub mod TRIM_PLL_CTRL0_DIV_SEL {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIM_USB_REG_ENV_TAIL_ADJ_VD
    pub mod TRIM_USB_REG_ENV_TAIL_ADJ_VD {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIM_USBPHY_TX_D_CAL
    pub mod TRIM_USBPHY_TX_D_CAL {
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

    /// TRIM_USBPHY_TX_CAL45DP
    pub mod TRIM_USBPHY_TX_CAL45DP {
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

    /// TRIM_USBPHY_TX_CAL45DN
    pub mod TRIM_USBPHY_TX_CAL45DN {
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

/// USB PHY Trim Override Enable Register
pub mod TRIM_OVERRIDE_EN_SET {
    pub use super::TRIM_OVERRIDE_EN::TRIM_DIV_SEL_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_ENV_TAIL_ADJ_VD_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_PLL_CTRL0_DIV_SEL;
    pub use super::TRIM_OVERRIDE_EN::TRIM_REFBIAS_TST_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_REFBIAS_VBGADJ_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_TX_CAL45DN_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_TX_CAL45DP_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_TX_D_CAL_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USB2_REFBIAS_TST;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USB2_REFBIAS_VBGADJ;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USBPHY_TX_CAL45DN;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USBPHY_TX_CAL45DP;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USBPHY_TX_D_CAL;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USB_REG_ENV_TAIL_ADJ_VD;
}

/// USB PHY Trim Override Enable Register
pub mod TRIM_OVERRIDE_EN_CLR {
    pub use super::TRIM_OVERRIDE_EN::TRIM_DIV_SEL_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_ENV_TAIL_ADJ_VD_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_PLL_CTRL0_DIV_SEL;
    pub use super::TRIM_OVERRIDE_EN::TRIM_REFBIAS_TST_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_REFBIAS_VBGADJ_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_TX_CAL45DN_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_TX_CAL45DP_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_TX_D_CAL_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USB2_REFBIAS_TST;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USB2_REFBIAS_VBGADJ;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USBPHY_TX_CAL45DN;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USBPHY_TX_CAL45DP;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USBPHY_TX_D_CAL;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USB_REG_ENV_TAIL_ADJ_VD;
}

/// USB PHY Trim Override Enable Register
pub mod TRIM_OVERRIDE_EN_TOG {
    pub use super::TRIM_OVERRIDE_EN::TRIM_DIV_SEL_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_ENV_TAIL_ADJ_VD_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_PLL_CTRL0_DIV_SEL;
    pub use super::TRIM_OVERRIDE_EN::TRIM_REFBIAS_TST_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_REFBIAS_VBGADJ_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_TX_CAL45DN_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_TX_CAL45DP_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_TX_D_CAL_OVERRIDE;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USB2_REFBIAS_TST;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USB2_REFBIAS_VBGADJ;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USBPHY_TX_CAL45DN;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USBPHY_TX_CAL45DP;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USBPHY_TX_D_CAL;
    pub use super::TRIM_OVERRIDE_EN::TRIM_USB_REG_ENV_TAIL_ADJ_VD;
}
#[repr(C)]
pub struct RegisterBlock {
    /// USB PHY Power-Down Register
    pub PWD: RWRegister<u32>,

    /// USB PHY Power-Down Register
    pub PWD_SET: RWRegister<u32>,

    /// USB PHY Power-Down Register
    pub PWD_CLR: RWRegister<u32>,

    /// USB PHY Power-Down Register
    pub PWD_TOG: RWRegister<u32>,

    /// USB PHY Transmitter Control Register
    pub TX: RWRegister<u32>,

    /// USB PHY Transmitter Control Register
    pub TX_SET: RWRegister<u32>,

    /// USB PHY Transmitter Control Register
    pub TX_CLR: RWRegister<u32>,

    /// USB PHY Transmitter Control Register
    pub TX_TOG: RWRegister<u32>,

    /// USB PHY Receiver Control Register
    pub RX: RWRegister<u32>,

    /// USB PHY Receiver Control Register
    pub RX_SET: RWRegister<u32>,

    /// USB PHY Receiver Control Register
    pub RX_CLR: RWRegister<u32>,

    /// USB PHY Receiver Control Register
    pub RX_TOG: RWRegister<u32>,

    /// USB PHY General Control Register
    pub CTRL: RWRegister<u32>,

    /// USB PHY General Control Register
    pub CTRL_SET: RWRegister<u32>,

    /// USB PHY General Control Register
    pub CTRL_CLR: RWRegister<u32>,

    /// USB PHY General Control Register
    pub CTRL_TOG: RWRegister<u32>,

    /// USB PHY Status Register
    pub STATUS: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// USB PHY Debug Register
    pub DEBUG: RWRegister<u32>,

    /// USB PHY Debug Register
    pub DEBUG_SET: RWRegister<u32>,

    /// USB PHY Debug Register
    pub DEBUG_CLR: RWRegister<u32>,

    /// USB PHY Debug Register
    pub DEBUG_TOG: RWRegister<u32>,

    /// UTMI Debug Status Register 0
    pub DEBUG0_STATUS: RORegister<u32>,

    _reserved2: [u32; 3],

    /// UTMI Debug Status Register 1
    pub DEBUG1: RWRegister<u32>,

    /// UTMI Debug Status Register 1
    pub DEBUG1_SET: RWRegister<u32>,

    /// UTMI Debug Status Register 1
    pub DEBUG1_CLR: RWRegister<u32>,

    /// UTMI Debug Status Register 1
    pub DEBUG1_TOG: RWRegister<u32>,

    /// UTMI RTL Version
    pub VERSION: RORegister<u32>,

    _reserved3: [u32; 7],

    /// USB PHY PLL Control/Status Register
    pub PLL_SIC: RWRegister<u32>,

    /// USB PHY PLL Control/Status Register
    pub PLL_SIC_SET: RWRegister<u32>,

    /// USB PHY PLL Control/Status Register
    pub PLL_SIC_CLR: RWRegister<u32>,

    /// USB PHY PLL Control/Status Register
    pub PLL_SIC_TOG: RWRegister<u32>,

    _reserved4: [u32; 4],

    /// USB PHY VBUS Detect Control Register
    pub USB1_VBUS_DETECT: RWRegister<u32>,

    /// USB PHY VBUS Detect Control Register
    pub USB1_VBUS_DETECT_SET: RWRegister<u32>,

    /// USB PHY VBUS Detect Control Register
    pub USB1_VBUS_DETECT_CLR: RWRegister<u32>,

    /// USB PHY VBUS Detect Control Register
    pub USB1_VBUS_DETECT_TOG: RWRegister<u32>,

    /// USB PHY VBUS Detector Status Register
    pub USB1_VBUS_DET_STAT: RORegister<u32>,

    _reserved5: [u32; 3],

    /// USB PHY Charger Detect Control Register
    pub USB1_CHRG_DETECT: RWRegister<u32>,

    /// USB PHY Charger Detect Control Register
    pub USB1_CHRG_DETECT_SET: RWRegister<u32>,

    /// USB PHY Charger Detect Control Register
    pub USB1_CHRG_DETECT_CLR: RWRegister<u32>,

    /// USB PHY Charger Detect Control Register
    pub USB1_CHRG_DETECT_TOG: RWRegister<u32>,

    /// USB PHY Charger Detect Status Register
    pub USB1_CHRG_DET_STAT: RORegister<u32>,

    _reserved6: [u32; 3],

    /// USB PHY Analog Control Register
    pub ANACTRL: RWRegister<u32>,

    /// USB PHY Analog Control Register
    pub ANACTRL_SET: RWRegister<u32>,

    /// USB PHY Analog Control Register
    pub ANACTRL_CLR: RWRegister<u32>,

    /// USB PHY Analog Control Register
    pub ANACTRL_TOG: RWRegister<u32>,

    /// USB PHY Loopback Control/Status Register
    pub USB1_LOOPBACK: RWRegister<u32>,

    /// USB PHY Loopback Control/Status Register
    pub USB1_LOOPBACK_SET: RWRegister<u32>,

    /// USB PHY Loopback Control/Status Register
    pub USB1_LOOPBACK_CLR: RWRegister<u32>,

    /// USB PHY Loopback Control/Status Register
    pub USB1_LOOPBACK_TOG: RWRegister<u32>,

    /// USB PHY Loopback Packet Number Select Register
    pub USB1_LOOPBACK_HSFSCNT: RWRegister<u32>,

    /// USB PHY Loopback Packet Number Select Register
    pub USB1_LOOPBACK_HSFSCNT_SET: RWRegister<u32>,

    /// USB PHY Loopback Packet Number Select Register
    pub USB1_LOOPBACK_HSFSCNT_CLR: RWRegister<u32>,

    /// USB PHY Loopback Packet Number Select Register
    pub USB1_LOOPBACK_HSFSCNT_TOG: RWRegister<u32>,

    /// USB PHY Trim Override Enable Register
    pub TRIM_OVERRIDE_EN: RWRegister<u32>,

    /// USB PHY Trim Override Enable Register
    pub TRIM_OVERRIDE_EN_SET: RWRegister<u32>,

    /// USB PHY Trim Override Enable Register
    pub TRIM_OVERRIDE_EN_CLR: RWRegister<u32>,

    /// USB PHY Trim Override Enable Register
    pub TRIM_OVERRIDE_EN_TOG: RWRegister<u32>,
}
pub struct ResetValues {
    pub PWD: u32,
    pub PWD_SET: u32,
    pub PWD_CLR: u32,
    pub PWD_TOG: u32,
    pub TX: u32,
    pub TX_SET: u32,
    pub TX_CLR: u32,
    pub TX_TOG: u32,
    pub RX: u32,
    pub RX_SET: u32,
    pub RX_CLR: u32,
    pub RX_TOG: u32,
    pub CTRL: u32,
    pub CTRL_SET: u32,
    pub CTRL_CLR: u32,
    pub CTRL_TOG: u32,
    pub STATUS: u32,
    pub DEBUG: u32,
    pub DEBUG_SET: u32,
    pub DEBUG_CLR: u32,
    pub DEBUG_TOG: u32,
    pub DEBUG0_STATUS: u32,
    pub DEBUG1: u32,
    pub DEBUG1_SET: u32,
    pub DEBUG1_CLR: u32,
    pub DEBUG1_TOG: u32,
    pub VERSION: u32,
    pub PLL_SIC: u32,
    pub PLL_SIC_SET: u32,
    pub PLL_SIC_CLR: u32,
    pub PLL_SIC_TOG: u32,
    pub USB1_VBUS_DETECT: u32,
    pub USB1_VBUS_DETECT_SET: u32,
    pub USB1_VBUS_DETECT_CLR: u32,
    pub USB1_VBUS_DETECT_TOG: u32,
    pub USB1_VBUS_DET_STAT: u32,
    pub USB1_CHRG_DETECT: u32,
    pub USB1_CHRG_DETECT_SET: u32,
    pub USB1_CHRG_DETECT_CLR: u32,
    pub USB1_CHRG_DETECT_TOG: u32,
    pub USB1_CHRG_DET_STAT: u32,
    pub ANACTRL: u32,
    pub ANACTRL_SET: u32,
    pub ANACTRL_CLR: u32,
    pub ANACTRL_TOG: u32,
    pub USB1_LOOPBACK: u32,
    pub USB1_LOOPBACK_SET: u32,
    pub USB1_LOOPBACK_CLR: u32,
    pub USB1_LOOPBACK_TOG: u32,
    pub USB1_LOOPBACK_HSFSCNT: u32,
    pub USB1_LOOPBACK_HSFSCNT_SET: u32,
    pub USB1_LOOPBACK_HSFSCNT_CLR: u32,
    pub USB1_LOOPBACK_HSFSCNT_TOG: u32,
    pub TRIM_OVERRIDE_EN: u32,
    pub TRIM_OVERRIDE_EN_SET: u32,
    pub TRIM_OVERRIDE_EN_CLR: u32,
    pub TRIM_OVERRIDE_EN_TOG: u32,
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
