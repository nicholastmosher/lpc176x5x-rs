# ! [ doc = "Pin connect block" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "Pin connect block" ]
pub const PINCONNECT: Peripheral<PINCONNECT> = unsafe { Peripheral::new(1073922048) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Pin function select register 0." ]
    pub pinsel0: PINSEL0,
    # [ doc = "0x04 - Pin function select register 1." ]
    pub pinsel1: PINSEL1,
    # [ doc = "0x08 - Pin function select register 2." ]
    pub pinsel2: PINSEL2,
    # [ doc = "0x0c - Pin function select register 3." ]
    pub pinsel3: PINSEL3,
    # [ doc = "0x10 - Pin function select register 4" ]
    pub pinsel4: PINSEL4,
    _reserved0: [u8; 8usize],
    # [ doc = "0x1c - Pin function select register 7" ]
    pub pinsel7: PINSEL7,
    _reserved1: [u8; 4usize],
    # [ doc = "0x24 - Pin function select register 9" ]
    pub pinsel9: PINSEL9,
    # [ doc = "0x28 - Pin function select register 10" ]
    pub pinsel10: PINSEL10,
    _reserved2: [u8; 20usize],
    # [ doc = "0x40 - Pin mode select register 0" ]
    pub pinmode0: PINMODE0,
    # [ doc = "0x44 - Pin mode select register 1" ]
    pub pinmode1: PINMODE1,
    # [ doc = "0x48 - Pin mode select register 2" ]
    pub pinmode2: PINMODE2,
    # [ doc = "0x4c - Pin mode select register 3." ]
    pub pinmode3: PINMODE3,
    # [ doc = "0x50 - Pin mode select register 4" ]
    pub pinmode4: PINMODE4,
    _reserved3: [u8; 8usize],
    # [ doc = "0x5c - Pin mode select register 7" ]
    pub pinmode7: PINMODE7,
    _reserved4: [u8; 4usize],
    # [ doc = "0x64 - Pin mode select register 9" ]
    pub pinmode9: PINMODE9,
    # [ doc = "0x68 - Open drain mode control register 0" ]
    pub pinmode_od0: PINMODE_OD0,
    # [ doc = "0x6c - Open drain mode control register 1" ]
    pub pinmode_od1: PINMODE_OD1,
    # [ doc = "0x70 - Open drain mode control register 2" ]
    pub pinmode_od2: PINMODE_OD2,
    # [ doc = "0x74 - Open drain mode control register 3" ]
    pub pinmode_od3: PINMODE_OD3,
    # [ doc = "0x78 - Open drain mode control register 4" ]
    pub pinmode_od4: PINMODE_OD4,
    # [ doc = "0x7c - I2C Pin Configuration register" ]
    pub i2cpadcfg: I2CPADCFG,
}
# [ doc = "Pin function select register 0." ]
pub struct PINSEL0 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin function select register 0." ]
pub mod pinsel0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINSEL0 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P0_0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_0R {
        # [ doc = "GPIO P0.0" ]
        GPIO_P0,
        # [ doc = "RD1" ]
        RD1,
        # [ doc = "TXD3" ]
        TXD3,
        # [ doc = "SDA1" ]
        SDA1,
    }
    impl P0_0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_0R::GPIO_P0 => 0,
                P0_0R::RD1 => 1,
                P0_0R::TXD3 => 2,
                P0_0R::SDA1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_0R {
            match value {
                0 => P0_0R::GPIO_P0,
                1 => P0_0R::RD1,
                2 => P0_0R::TXD3,
                3 => P0_0R::SDA1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_0R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `RD1`" ]
        # [ inline ( always ) ]
        pub fn is_rd1(&self) -> bool {
            *self == P0_0R::RD1
        }
        # [ doc = "Checks if the value of the field is `TXD3`" ]
        # [ inline ( always ) ]
        pub fn is_txd3(&self) -> bool {
            *self == P0_0R::TXD3
        }
        # [ doc = "Checks if the value of the field is `SDA1`" ]
        # [ inline ( always ) ]
        pub fn is_sda1(&self) -> bool {
            *self == P0_0R::SDA1
        }
    }
    # [ doc = "Possible values of the field `P0_1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_1R {
        # [ doc = "GPIO P0.1" ]
        GPIO_P0,
        # [ doc = "TD1" ]
        TD1,
        # [ doc = "RXD3" ]
        RXD3,
        # [ doc = "SCL1" ]
        SCL1,
    }
    impl P0_1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_1R::GPIO_P0 => 0,
                P0_1R::TD1 => 1,
                P0_1R::RXD3 => 2,
                P0_1R::SCL1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_1R {
            match value {
                0 => P0_1R::GPIO_P0,
                1 => P0_1R::TD1,
                2 => P0_1R::RXD3,
                3 => P0_1R::SCL1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_1R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `TD1`" ]
        # [ inline ( always ) ]
        pub fn is_td1(&self) -> bool {
            *self == P0_1R::TD1
        }
        # [ doc = "Checks if the value of the field is `RXD3`" ]
        # [ inline ( always ) ]
        pub fn is_rxd3(&self) -> bool {
            *self == P0_1R::RXD3
        }
        # [ doc = "Checks if the value of the field is `SCL1`" ]
        # [ inline ( always ) ]
        pub fn is_scl1(&self) -> bool {
            *self == P0_1R::SCL1
        }
    }
    # [ doc = "Possible values of the field `P0_2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_2R {
        # [ doc = "GPIO P0.2" ]
        GPIO_P0,
        # [ doc = "TXD0" ]
        TXD0,
        # [ doc = "AD0.7" ]
        AD0,
    }
    impl P0_2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_2R::GPIO_P0 => 0,
                P0_2R::TXD0 => 1,
                P0_2R::AD0 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_2R {
            match value {
                0 => P0_2R::GPIO_P0,
                1 => P0_2R::TXD0,
                2 => P0_2R::AD0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_2R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `TXD0`" ]
        # [ inline ( always ) ]
        pub fn is_txd0(&self) -> bool {
            *self == P0_2R::TXD0
        }
        # [ doc = "Checks if the value of the field is `AD0`" ]
        # [ inline ( always ) ]
        pub fn is_ad0(&self) -> bool {
            *self == P0_2R::AD0
        }
    }
    # [ doc = "Possible values of the field `P0_3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_3R {
        # [ doc = "GPIO P0.3." ]
        GPIO_P0,
        # [ doc = "RXD0" ]
        RXD0,
        # [ doc = "AD0.6" ]
        AD0,
    }
    impl P0_3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_3R::GPIO_P0 => 0,
                P0_3R::RXD0 => 1,
                P0_3R::AD0 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_3R {
            match value {
                0 => P0_3R::GPIO_P0,
                1 => P0_3R::RXD0,
                2 => P0_3R::AD0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_3R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `RXD0`" ]
        # [ inline ( always ) ]
        pub fn is_rxd0(&self) -> bool {
            *self == P0_3R::RXD0
        }
        # [ doc = "Checks if the value of the field is `AD0`" ]
        # [ inline ( always ) ]
        pub fn is_ad0(&self) -> bool {
            *self == P0_3R::AD0
        }
    }
    # [ doc = "Possible values of the field `P0_4`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_4R {
        # [ doc = "GPIO P0.4." ]
        GPIO_P0,
        # [ doc = "I2SRX_CLK" ]
        I2SRX_CLK,
        # [ doc = "RD2" ]
        RD2,
        # [ doc = "CAP2.0" ]
        CAP2,
    }
    impl P0_4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_4R::GPIO_P0 => 0,
                P0_4R::I2SRX_CLK => 1,
                P0_4R::RD2 => 2,
                P0_4R::CAP2 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_4R {
            match value {
                0 => P0_4R::GPIO_P0,
                1 => P0_4R::I2SRX_CLK,
                2 => P0_4R::RD2,
                3 => P0_4R::CAP2,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_4R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `I2SRX_CLK`" ]
        # [ inline ( always ) ]
        pub fn is_i2srx_clk(&self) -> bool {
            *self == P0_4R::I2SRX_CLK
        }
        # [ doc = "Checks if the value of the field is `RD2`" ]
        # [ inline ( always ) ]
        pub fn is_rd2(&self) -> bool {
            *self == P0_4R::RD2
        }
        # [ doc = "Checks if the value of the field is `CAP2`" ]
        # [ inline ( always ) ]
        pub fn is_cap2(&self) -> bool {
            *self == P0_4R::CAP2
        }
    }
    # [ doc = "Possible values of the field `P0_5`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_5R {
        # [ doc = "GPIO P0.5." ]
        GPIO_P0,
        # [ doc = "I2SRX_WS" ]
        I2SRX_WS,
        # [ doc = "TD2" ]
        TD2,
        # [ doc = "CAP2.1" ]
        CAP2,
    }
    impl P0_5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_5R::GPIO_P0 => 0,
                P0_5R::I2SRX_WS => 1,
                P0_5R::TD2 => 2,
                P0_5R::CAP2 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_5R {
            match value {
                0 => P0_5R::GPIO_P0,
                1 => P0_5R::I2SRX_WS,
                2 => P0_5R::TD2,
                3 => P0_5R::CAP2,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_5R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `I2SRX_WS`" ]
        # [ inline ( always ) ]
        pub fn is_i2srx_ws(&self) -> bool {
            *self == P0_5R::I2SRX_WS
        }
        # [ doc = "Checks if the value of the field is `TD2`" ]
        # [ inline ( always ) ]
        pub fn is_td2(&self) -> bool {
            *self == P0_5R::TD2
        }
        # [ doc = "Checks if the value of the field is `CAP2`" ]
        # [ inline ( always ) ]
        pub fn is_cap2(&self) -> bool {
            *self == P0_5R::CAP2
        }
    }
    # [ doc = "Possible values of the field `P0_6`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_6R {
        # [ doc = "GPIO P0.6." ]
        GPIO_P0,
        # [ doc = "I2SRX_SDA" ]
        I2SRX_SDA,
        # [ doc = "SSEL1" ]
        SSEL1,
        # [ doc = "MAT2.0" ]
        MAT2,
    }
    impl P0_6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_6R::GPIO_P0 => 0,
                P0_6R::I2SRX_SDA => 1,
                P0_6R::SSEL1 => 2,
                P0_6R::MAT2 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_6R {
            match value {
                0 => P0_6R::GPIO_P0,
                1 => P0_6R::I2SRX_SDA,
                2 => P0_6R::SSEL1,
                3 => P0_6R::MAT2,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_6R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `I2SRX_SDA`" ]
        # [ inline ( always ) ]
        pub fn is_i2srx_sda(&self) -> bool {
            *self == P0_6R::I2SRX_SDA
        }
        # [ doc = "Checks if the value of the field is `SSEL1`" ]
        # [ inline ( always ) ]
        pub fn is_ssel1(&self) -> bool {
            *self == P0_6R::SSEL1
        }
        # [ doc = "Checks if the value of the field is `MAT2`" ]
        # [ inline ( always ) ]
        pub fn is_mat2(&self) -> bool {
            *self == P0_6R::MAT2
        }
    }
    # [ doc = "Possible values of the field `P0_7`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_7R {
        # [ doc = "GPIO P0.7." ]
        GPIO_P0,
        # [ doc = "I2STX_CLK" ]
        I2STX_CLK,
        # [ doc = "SCK1" ]
        SCK1,
        # [ doc = "MAT2.1" ]
        MAT2,
    }
    impl P0_7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_7R::GPIO_P0 => 0,
                P0_7R::I2STX_CLK => 1,
                P0_7R::SCK1 => 2,
                P0_7R::MAT2 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_7R {
            match value {
                0 => P0_7R::GPIO_P0,
                1 => P0_7R::I2STX_CLK,
                2 => P0_7R::SCK1,
                3 => P0_7R::MAT2,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_7R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `I2STX_CLK`" ]
        # [ inline ( always ) ]
        pub fn is_i2stx_clk(&self) -> bool {
            *self == P0_7R::I2STX_CLK
        }
        # [ doc = "Checks if the value of the field is `SCK1`" ]
        # [ inline ( always ) ]
        pub fn is_sck1(&self) -> bool {
            *self == P0_7R::SCK1
        }
        # [ doc = "Checks if the value of the field is `MAT2`" ]
        # [ inline ( always ) ]
        pub fn is_mat2(&self) -> bool {
            *self == P0_7R::MAT2
        }
    }
    # [ doc = "Possible values of the field `P0_8`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_8R {
        # [ doc = "GPIO P0.8." ]
        GPIO_P0,
        # [ doc = "I2STX_WS" ]
        I2STX_WS,
        # [ doc = "MISO1" ]
        MISO1,
        # [ doc = "MAT2.2" ]
        MAT2,
    }
    impl P0_8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_8R::GPIO_P0 => 0,
                P0_8R::I2STX_WS => 1,
                P0_8R::MISO1 => 2,
                P0_8R::MAT2 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_8R {
            match value {
                0 => P0_8R::GPIO_P0,
                1 => P0_8R::I2STX_WS,
                2 => P0_8R::MISO1,
                3 => P0_8R::MAT2,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_8R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `I2STX_WS`" ]
        # [ inline ( always ) ]
        pub fn is_i2stx_ws(&self) -> bool {
            *self == P0_8R::I2STX_WS
        }
        # [ doc = "Checks if the value of the field is `MISO1`" ]
        # [ inline ( always ) ]
        pub fn is_miso1(&self) -> bool {
            *self == P0_8R::MISO1
        }
        # [ doc = "Checks if the value of the field is `MAT2`" ]
        # [ inline ( always ) ]
        pub fn is_mat2(&self) -> bool {
            *self == P0_8R::MAT2
        }
    }
    # [ doc = "Possible values of the field `P0_9`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_9R {
        # [ doc = "GPIO P0.9" ]
        GPIO_P0,
        # [ doc = "I2STX_SDA" ]
        I2STX_SDA,
        # [ doc = "MOSI1" ]
        MOSI1,
        # [ doc = "MAT2.3" ]
        MAT2,
    }
    impl P0_9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_9R::GPIO_P0 => 0,
                P0_9R::I2STX_SDA => 1,
                P0_9R::MOSI1 => 2,
                P0_9R::MAT2 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_9R {
            match value {
                0 => P0_9R::GPIO_P0,
                1 => P0_9R::I2STX_SDA,
                2 => P0_9R::MOSI1,
                3 => P0_9R::MAT2,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_9R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `I2STX_SDA`" ]
        # [ inline ( always ) ]
        pub fn is_i2stx_sda(&self) -> bool {
            *self == P0_9R::I2STX_SDA
        }
        # [ doc = "Checks if the value of the field is `MOSI1`" ]
        # [ inline ( always ) ]
        pub fn is_mosi1(&self) -> bool {
            *self == P0_9R::MOSI1
        }
        # [ doc = "Checks if the value of the field is `MAT2`" ]
        # [ inline ( always ) ]
        pub fn is_mat2(&self) -> bool {
            *self == P0_9R::MAT2
        }
    }
    # [ doc = "Possible values of the field `P0_10`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_10R {
        # [ doc = "GPIO P0.10" ]
        GPIO_P0,
        # [ doc = "TXD2" ]
        TXD2,
        # [ doc = "SDA2" ]
        SDA2,
        # [ doc = "MAT3.0" ]
        MAT3,
    }
    impl P0_10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_10R::GPIO_P0 => 0,
                P0_10R::TXD2 => 1,
                P0_10R::SDA2 => 2,
                P0_10R::MAT3 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_10R {
            match value {
                0 => P0_10R::GPIO_P0,
                1 => P0_10R::TXD2,
                2 => P0_10R::SDA2,
                3 => P0_10R::MAT3,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_10R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `TXD2`" ]
        # [ inline ( always ) ]
        pub fn is_txd2(&self) -> bool {
            *self == P0_10R::TXD2
        }
        # [ doc = "Checks if the value of the field is `SDA2`" ]
        # [ inline ( always ) ]
        pub fn is_sda2(&self) -> bool {
            *self == P0_10R::SDA2
        }
        # [ doc = "Checks if the value of the field is `MAT3`" ]
        # [ inline ( always ) ]
        pub fn is_mat3(&self) -> bool {
            *self == P0_10R::MAT3
        }
    }
    # [ doc = "Possible values of the field `P0_11`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_11R {
        # [ doc = "GPIO P0.11" ]
        GPIO_P0,
        # [ doc = "RXD2" ]
        RXD2,
        # [ doc = "SCL2" ]
        SCL2,
        # [ doc = "MAT3.1" ]
        MAT3,
    }
    impl P0_11R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_11R::GPIO_P0 => 0,
                P0_11R::RXD2 => 1,
                P0_11R::SCL2 => 2,
                P0_11R::MAT3 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_11R {
            match value {
                0 => P0_11R::GPIO_P0,
                1 => P0_11R::RXD2,
                2 => P0_11R::SCL2,
                3 => P0_11R::MAT3,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_11R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `RXD2`" ]
        # [ inline ( always ) ]
        pub fn is_rxd2(&self) -> bool {
            *self == P0_11R::RXD2
        }
        # [ doc = "Checks if the value of the field is `SCL2`" ]
        # [ inline ( always ) ]
        pub fn is_scl2(&self) -> bool {
            *self == P0_11R::SCL2
        }
        # [ doc = "Checks if the value of the field is `MAT3`" ]
        # [ inline ( always ) ]
        pub fn is_mat3(&self) -> bool {
            *self == P0_11R::MAT3
        }
    }
    # [ doc = "Possible values of the field `P0_15`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_15R {
        # [ doc = "GPIO P0.15" ]
        GPIO_P0,
        # [ doc = "TXD1" ]
        TXD1,
        # [ doc = "SCK0" ]
        SCK0,
        # [ doc = "SCK" ]
        SCK,
    }
    impl P0_15R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_15R::GPIO_P0 => 0,
                P0_15R::TXD1 => 1,
                P0_15R::SCK0 => 2,
                P0_15R::SCK => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_15R {
            match value {
                0 => P0_15R::GPIO_P0,
                1 => P0_15R::TXD1,
                2 => P0_15R::SCK0,
                3 => P0_15R::SCK,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_15R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `TXD1`" ]
        # [ inline ( always ) ]
        pub fn is_txd1(&self) -> bool {
            *self == P0_15R::TXD1
        }
        # [ doc = "Checks if the value of the field is `SCK0`" ]
        # [ inline ( always ) ]
        pub fn is_sck0(&self) -> bool {
            *self == P0_15R::SCK0
        }
        # [ doc = "Checks if the value of the field is `SCK`" ]
        # [ inline ( always ) ]
        pub fn is_sck(&self) -> bool {
            *self == P0_15R::SCK
        }
    }
    # [ doc = "Values that can be written to the field `P0_0`" ]
    pub enum P0_0W {
        # [ doc = "GPIO P0.0" ]
        GPIO_P0,
        # [ doc = "RD1" ]
        RD1,
        # [ doc = "TXD3" ]
        TXD3,
        # [ doc = "SDA1" ]
        SDA1,
    }
    impl P0_0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_0W::GPIO_P0 => 0,
                P0_0W::RD1 => 1,
                P0_0W::TXD3 => 2,
                P0_0W::SDA1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_0W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.0" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_0W::GPIO_P0)
        }
        # [ doc = "RD1" ]
        # [ inline ( always ) ]
        pub fn rd1(self) -> &'a mut W {
            self.variant(P0_0W::RD1)
        }
        # [ doc = "TXD3" ]
        # [ inline ( always ) ]
        pub fn txd3(self) -> &'a mut W {
            self.variant(P0_0W::TXD3)
        }
        # [ doc = "SDA1" ]
        # [ inline ( always ) ]
        pub fn sda1(self) -> &'a mut W {
            self.variant(P0_0W::SDA1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_1`" ]
    pub enum P0_1W {
        # [ doc = "GPIO P0.1" ]
        GPIO_P0,
        # [ doc = "TD1" ]
        TD1,
        # [ doc = "RXD3" ]
        RXD3,
        # [ doc = "SCL1" ]
        SCL1,
    }
    impl P0_1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_1W::GPIO_P0 => 0,
                P0_1W::TD1 => 1,
                P0_1W::RXD3 => 2,
                P0_1W::SCL1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.1" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_1W::GPIO_P0)
        }
        # [ doc = "TD1" ]
        # [ inline ( always ) ]
        pub fn td1(self) -> &'a mut W {
            self.variant(P0_1W::TD1)
        }
        # [ doc = "RXD3" ]
        # [ inline ( always ) ]
        pub fn rxd3(self) -> &'a mut W {
            self.variant(P0_1W::RXD3)
        }
        # [ doc = "SCL1" ]
        # [ inline ( always ) ]
        pub fn scl1(self) -> &'a mut W {
            self.variant(P0_1W::SCL1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_2`" ]
    pub enum P0_2W {
        # [ doc = "GPIO P0.2" ]
        GPIO_P0,
        # [ doc = "TXD0" ]
        TXD0,
        # [ doc = "AD0.7" ]
        AD0,
    }
    impl P0_2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_2W::GPIO_P0 => 0,
                P0_2W::TXD0 => 1,
                P0_2W::AD0 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_2W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P0.2" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_2W::GPIO_P0)
        }
        # [ doc = "TXD0" ]
        # [ inline ( always ) ]
        pub fn txd0(self) -> &'a mut W {
            self.variant(P0_2W::TXD0)
        }
        # [ doc = "AD0.7" ]
        # [ inline ( always ) ]
        pub fn ad0(self) -> &'a mut W {
            self.variant(P0_2W::AD0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_3`" ]
    pub enum P0_3W {
        # [ doc = "GPIO P0.3." ]
        GPIO_P0,
        # [ doc = "RXD0" ]
        RXD0,
        # [ doc = "AD0.6" ]
        AD0,
    }
    impl P0_3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_3W::GPIO_P0 => 0,
                P0_3W::RXD0 => 1,
                P0_3W::AD0 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_3W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P0.3." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_3W::GPIO_P0)
        }
        # [ doc = "RXD0" ]
        # [ inline ( always ) ]
        pub fn rxd0(self) -> &'a mut W {
            self.variant(P0_3W::RXD0)
        }
        # [ doc = "AD0.6" ]
        # [ inline ( always ) ]
        pub fn ad0(self) -> &'a mut W {
            self.variant(P0_3W::AD0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_4`" ]
    pub enum P0_4W {
        # [ doc = "GPIO P0.4." ]
        GPIO_P0,
        # [ doc = "I2SRX_CLK" ]
        I2SRX_CLK,
        # [ doc = "RD2" ]
        RD2,
        # [ doc = "CAP2.0" ]
        CAP2,
    }
    impl P0_4W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_4W::GPIO_P0 => 0,
                P0_4W::I2SRX_CLK => 1,
                P0_4W::RD2 => 2,
                P0_4W::CAP2 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_4W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.4." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_4W::GPIO_P0)
        }
        # [ doc = "I2SRX_CLK" ]
        # [ inline ( always ) ]
        pub fn i2srx_clk(self) -> &'a mut W {
            self.variant(P0_4W::I2SRX_CLK)
        }
        # [ doc = "RD2" ]
        # [ inline ( always ) ]
        pub fn rd2(self) -> &'a mut W {
            self.variant(P0_4W::RD2)
        }
        # [ doc = "CAP2.0" ]
        # [ inline ( always ) ]
        pub fn cap2(self) -> &'a mut W {
            self.variant(P0_4W::CAP2)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_5`" ]
    pub enum P0_5W {
        # [ doc = "GPIO P0.5." ]
        GPIO_P0,
        # [ doc = "I2SRX_WS" ]
        I2SRX_WS,
        # [ doc = "TD2" ]
        TD2,
        # [ doc = "CAP2.1" ]
        CAP2,
    }
    impl P0_5W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_5W::GPIO_P0 => 0,
                P0_5W::I2SRX_WS => 1,
                P0_5W::TD2 => 2,
                P0_5W::CAP2 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_5W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.5." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_5W::GPIO_P0)
        }
        # [ doc = "I2SRX_WS" ]
        # [ inline ( always ) ]
        pub fn i2srx_ws(self) -> &'a mut W {
            self.variant(P0_5W::I2SRX_WS)
        }
        # [ doc = "TD2" ]
        # [ inline ( always ) ]
        pub fn td2(self) -> &'a mut W {
            self.variant(P0_5W::TD2)
        }
        # [ doc = "CAP2.1" ]
        # [ inline ( always ) ]
        pub fn cap2(self) -> &'a mut W {
            self.variant(P0_5W::CAP2)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_6`" ]
    pub enum P0_6W {
        # [ doc = "GPIO P0.6." ]
        GPIO_P0,
        # [ doc = "I2SRX_SDA" ]
        I2SRX_SDA,
        # [ doc = "SSEL1" ]
        SSEL1,
        # [ doc = "MAT2.0" ]
        MAT2,
    }
    impl P0_6W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_6W::GPIO_P0 => 0,
                P0_6W::I2SRX_SDA => 1,
                P0_6W::SSEL1 => 2,
                P0_6W::MAT2 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_6W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.6." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_6W::GPIO_P0)
        }
        # [ doc = "I2SRX_SDA" ]
        # [ inline ( always ) ]
        pub fn i2srx_sda(self) -> &'a mut W {
            self.variant(P0_6W::I2SRX_SDA)
        }
        # [ doc = "SSEL1" ]
        # [ inline ( always ) ]
        pub fn ssel1(self) -> &'a mut W {
            self.variant(P0_6W::SSEL1)
        }
        # [ doc = "MAT2.0" ]
        # [ inline ( always ) ]
        pub fn mat2(self) -> &'a mut W {
            self.variant(P0_6W::MAT2)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_7`" ]
    pub enum P0_7W {
        # [ doc = "GPIO P0.7." ]
        GPIO_P0,
        # [ doc = "I2STX_CLK" ]
        I2STX_CLK,
        # [ doc = "SCK1" ]
        SCK1,
        # [ doc = "MAT2.1" ]
        MAT2,
    }
    impl P0_7W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_7W::GPIO_P0 => 0,
                P0_7W::I2STX_CLK => 1,
                P0_7W::SCK1 => 2,
                P0_7W::MAT2 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_7W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.7." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_7W::GPIO_P0)
        }
        # [ doc = "I2STX_CLK" ]
        # [ inline ( always ) ]
        pub fn i2stx_clk(self) -> &'a mut W {
            self.variant(P0_7W::I2STX_CLK)
        }
        # [ doc = "SCK1" ]
        # [ inline ( always ) ]
        pub fn sck1(self) -> &'a mut W {
            self.variant(P0_7W::SCK1)
        }
        # [ doc = "MAT2.1" ]
        # [ inline ( always ) ]
        pub fn mat2(self) -> &'a mut W {
            self.variant(P0_7W::MAT2)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_8`" ]
    pub enum P0_8W {
        # [ doc = "GPIO P0.8." ]
        GPIO_P0,
        # [ doc = "I2STX_WS" ]
        I2STX_WS,
        # [ doc = "MISO1" ]
        MISO1,
        # [ doc = "MAT2.2" ]
        MAT2,
    }
    impl P0_8W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_8W::GPIO_P0 => 0,
                P0_8W::I2STX_WS => 1,
                P0_8W::MISO1 => 2,
                P0_8W::MAT2 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_8W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.8." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_8W::GPIO_P0)
        }
        # [ doc = "I2STX_WS" ]
        # [ inline ( always ) ]
        pub fn i2stx_ws(self) -> &'a mut W {
            self.variant(P0_8W::I2STX_WS)
        }
        # [ doc = "MISO1" ]
        # [ inline ( always ) ]
        pub fn miso1(self) -> &'a mut W {
            self.variant(P0_8W::MISO1)
        }
        # [ doc = "MAT2.2" ]
        # [ inline ( always ) ]
        pub fn mat2(self) -> &'a mut W {
            self.variant(P0_8W::MAT2)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_9`" ]
    pub enum P0_9W {
        # [ doc = "GPIO P0.9" ]
        GPIO_P0,
        # [ doc = "I2STX_SDA" ]
        I2STX_SDA,
        # [ doc = "MOSI1" ]
        MOSI1,
        # [ doc = "MAT2.3" ]
        MAT2,
    }
    impl P0_9W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_9W::GPIO_P0 => 0,
                P0_9W::I2STX_SDA => 1,
                P0_9W::MOSI1 => 2,
                P0_9W::MAT2 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_9W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.9" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_9W::GPIO_P0)
        }
        # [ doc = "I2STX_SDA" ]
        # [ inline ( always ) ]
        pub fn i2stx_sda(self) -> &'a mut W {
            self.variant(P0_9W::I2STX_SDA)
        }
        # [ doc = "MOSI1" ]
        # [ inline ( always ) ]
        pub fn mosi1(self) -> &'a mut W {
            self.variant(P0_9W::MOSI1)
        }
        # [ doc = "MAT2.3" ]
        # [ inline ( always ) ]
        pub fn mat2(self) -> &'a mut W {
            self.variant(P0_9W::MAT2)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_10`" ]
    pub enum P0_10W {
        # [ doc = "GPIO P0.10" ]
        GPIO_P0,
        # [ doc = "TXD2" ]
        TXD2,
        # [ doc = "SDA2" ]
        SDA2,
        # [ doc = "MAT3.0" ]
        MAT3,
    }
    impl P0_10W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_10W::GPIO_P0 => 0,
                P0_10W::TXD2 => 1,
                P0_10W::SDA2 => 2,
                P0_10W::MAT3 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_10W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.10" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_10W::GPIO_P0)
        }
        # [ doc = "TXD2" ]
        # [ inline ( always ) ]
        pub fn txd2(self) -> &'a mut W {
            self.variant(P0_10W::TXD2)
        }
        # [ doc = "SDA2" ]
        # [ inline ( always ) ]
        pub fn sda2(self) -> &'a mut W {
            self.variant(P0_10W::SDA2)
        }
        # [ doc = "MAT3.0" ]
        # [ inline ( always ) ]
        pub fn mat3(self) -> &'a mut W {
            self.variant(P0_10W::MAT3)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_11`" ]
    pub enum P0_11W {
        # [ doc = "GPIO P0.11" ]
        GPIO_P0,
        # [ doc = "RXD2" ]
        RXD2,
        # [ doc = "SCL2" ]
        SCL2,
        # [ doc = "MAT3.1" ]
        MAT3,
    }
    impl P0_11W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_11W::GPIO_P0 => 0,
                P0_11W::RXD2 => 1,
                P0_11W::SCL2 => 2,
                P0_11W::MAT3 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_11W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.11" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_11W::GPIO_P0)
        }
        # [ doc = "RXD2" ]
        # [ inline ( always ) ]
        pub fn rxd2(self) -> &'a mut W {
            self.variant(P0_11W::RXD2)
        }
        # [ doc = "SCL2" ]
        # [ inline ( always ) ]
        pub fn scl2(self) -> &'a mut W {
            self.variant(P0_11W::SCL2)
        }
        # [ doc = "MAT3.1" ]
        # [ inline ( always ) ]
        pub fn mat3(self) -> &'a mut W {
            self.variant(P0_11W::MAT3)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_15`" ]
    pub enum P0_15W {
        # [ doc = "GPIO P0.15" ]
        GPIO_P0,
        # [ doc = "TXD1" ]
        TXD1,
        # [ doc = "SCK0" ]
        SCK0,
        # [ doc = "SCK" ]
        SCK,
    }
    impl P0_15W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_15W::GPIO_P0 => 0,
                P0_15W::TXD1 => 1,
                P0_15W::SCK0 => 2,
                P0_15W::SCK => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_15W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.15" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_15W::GPIO_P0)
        }
        # [ doc = "TXD1" ]
        # [ inline ( always ) ]
        pub fn txd1(self) -> &'a mut W {
            self.variant(P0_15W::TXD1)
        }
        # [ doc = "SCK0" ]
        # [ inline ( always ) ]
        pub fn sck0(self) -> &'a mut W {
            self.variant(P0_15W::SCK0)
        }
        # [ doc = "SCK" ]
        # [ inline ( always ) ]
        pub fn sck(self) -> &'a mut W {
            self.variant(P0_15W::SCK)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Pin function select P0.0." ]
        # [ inline ( always ) ]
        pub fn p0_0(&self) -> P0_0R {
            P0_0R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 0;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 2:3 - Pin function select P0.1." ]
        # [ inline ( always ) ]
        pub fn p0_1(&self) -> P0_1R {
            P0_1R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 2;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 4:5 - Pin function select P0.2." ]
        # [ inline ( always ) ]
        pub fn p0_2(&self) -> P0_2R {
            P0_2R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 4;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 6:7 - Pin function select P0.3." ]
        # [ inline ( always ) ]
        pub fn p0_3(&self) -> P0_3R {
            P0_3R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 6;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 8:9 - Pin function select P0.4." ]
        # [ inline ( always ) ]
        pub fn p0_4(&self) -> P0_4R {
            P0_4R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 8;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 10:11 - Pin function select P0.5." ]
        # [ inline ( always ) ]
        pub fn p0_5(&self) -> P0_5R {
            P0_5R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 10;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 12:13 - Pin function select P0.6." ]
        # [ inline ( always ) ]
        pub fn p0_6(&self) -> P0_6R {
            P0_6R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 12;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 14:15 - Pin function select P0.7." ]
        # [ inline ( always ) ]
        pub fn p0_7(&self) -> P0_7R {
            P0_7R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 14;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 16:17 - Pin function select P0.8." ]
        # [ inline ( always ) ]
        pub fn p0_8(&self) -> P0_8R {
            P0_8R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 16;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 18:19 - Pin function select P0.9." ]
        # [ inline ( always ) ]
        pub fn p0_9(&self) -> P0_9R {
            P0_9R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 18;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 20:21 - Pin function select P0.10." ]
        # [ inline ( always ) ]
        pub fn p0_10(&self) -> P0_10R {
            P0_10R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 20;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 22:23 - Pin function select P0.11." ]
        # [ inline ( always ) ]
        pub fn p0_11(&self) -> P0_11R {
            P0_11R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 22;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 30:31 - Pin function select P0.15." ]
        # [ inline ( always ) ]
        pub fn p0_15(&self) -> P0_15R {
            P0_15R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 30;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Pin function select P0.0." ]
        # [ inline ( always ) ]
        pub fn p0_0(&mut self) -> _P0_0W {
            _P0_0W { w: self }
        }
        # [ doc = "Bits 2:3 - Pin function select P0.1." ]
        # [ inline ( always ) ]
        pub fn p0_1(&mut self) -> _P0_1W {
            _P0_1W { w: self }
        }
        # [ doc = "Bits 4:5 - Pin function select P0.2." ]
        # [ inline ( always ) ]
        pub fn p0_2(&mut self) -> _P0_2W {
            _P0_2W { w: self }
        }
        # [ doc = "Bits 6:7 - Pin function select P0.3." ]
        # [ inline ( always ) ]
        pub fn p0_3(&mut self) -> _P0_3W {
            _P0_3W { w: self }
        }
        # [ doc = "Bits 8:9 - Pin function select P0.4." ]
        # [ inline ( always ) ]
        pub fn p0_4(&mut self) -> _P0_4W {
            _P0_4W { w: self }
        }
        # [ doc = "Bits 10:11 - Pin function select P0.5." ]
        # [ inline ( always ) ]
        pub fn p0_5(&mut self) -> _P0_5W {
            _P0_5W { w: self }
        }
        # [ doc = "Bits 12:13 - Pin function select P0.6." ]
        # [ inline ( always ) ]
        pub fn p0_6(&mut self) -> _P0_6W {
            _P0_6W { w: self }
        }
        # [ doc = "Bits 14:15 - Pin function select P0.7." ]
        # [ inline ( always ) ]
        pub fn p0_7(&mut self) -> _P0_7W {
            _P0_7W { w: self }
        }
        # [ doc = "Bits 16:17 - Pin function select P0.8." ]
        # [ inline ( always ) ]
        pub fn p0_8(&mut self) -> _P0_8W {
            _P0_8W { w: self }
        }
        # [ doc = "Bits 18:19 - Pin function select P0.9." ]
        # [ inline ( always ) ]
        pub fn p0_9(&mut self) -> _P0_9W {
            _P0_9W { w: self }
        }
        # [ doc = "Bits 20:21 - Pin function select P0.10." ]
        # [ inline ( always ) ]
        pub fn p0_10(&mut self) -> _P0_10W {
            _P0_10W { w: self }
        }
        # [ doc = "Bits 22:23 - Pin function select P0.11." ]
        # [ inline ( always ) ]
        pub fn p0_11(&mut self) -> _P0_11W {
            _P0_11W { w: self }
        }
        # [ doc = "Bits 30:31 - Pin function select P0.15." ]
        # [ inline ( always ) ]
        pub fn p0_15(&mut self) -> _P0_15W {
            _P0_15W { w: self }
        }
    }
}
# [ doc = "Pin function select register 1." ]
pub struct PINSEL1 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin function select register 1." ]
pub mod pinsel1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINSEL1 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P0_16`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_16R {
        # [ doc = "GPIO P0.16" ]
        GPIO_P0,
        # [ doc = "RXD1" ]
        RXD1,
        # [ doc = "SSEL0" ]
        SSEL0,
        # [ doc = "SSEL" ]
        SSEL,
    }
    impl P0_16R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_16R::GPIO_P0 => 0,
                P0_16R::RXD1 => 1,
                P0_16R::SSEL0 => 2,
                P0_16R::SSEL => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_16R {
            match value {
                0 => P0_16R::GPIO_P0,
                1 => P0_16R::RXD1,
                2 => P0_16R::SSEL0,
                3 => P0_16R::SSEL,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_16R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `RXD1`" ]
        # [ inline ( always ) ]
        pub fn is_rxd1(&self) -> bool {
            *self == P0_16R::RXD1
        }
        # [ doc = "Checks if the value of the field is `SSEL0`" ]
        # [ inline ( always ) ]
        pub fn is_ssel0(&self) -> bool {
            *self == P0_16R::SSEL0
        }
        # [ doc = "Checks if the value of the field is `SSEL`" ]
        # [ inline ( always ) ]
        pub fn is_ssel(&self) -> bool {
            *self == P0_16R::SSEL
        }
    }
    # [ doc = "Possible values of the field `P0_17`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_17R {
        # [ doc = "GPIO P0.17" ]
        GPIO_P0,
        # [ doc = "CTS1" ]
        CTS1,
        # [ doc = "MISO0" ]
        MISO0,
        # [ doc = "MISO" ]
        MISO,
    }
    impl P0_17R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_17R::GPIO_P0 => 0,
                P0_17R::CTS1 => 1,
                P0_17R::MISO0 => 2,
                P0_17R::MISO => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_17R {
            match value {
                0 => P0_17R::GPIO_P0,
                1 => P0_17R::CTS1,
                2 => P0_17R::MISO0,
                3 => P0_17R::MISO,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_17R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `CTS1`" ]
        # [ inline ( always ) ]
        pub fn is_cts1(&self) -> bool {
            *self == P0_17R::CTS1
        }
        # [ doc = "Checks if the value of the field is `MISO0`" ]
        # [ inline ( always ) ]
        pub fn is_miso0(&self) -> bool {
            *self == P0_17R::MISO0
        }
        # [ doc = "Checks if the value of the field is `MISO`" ]
        # [ inline ( always ) ]
        pub fn is_miso(&self) -> bool {
            *self == P0_17R::MISO
        }
    }
    # [ doc = "Possible values of the field `P0_18`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_18R {
        # [ doc = "GPIO P0.18" ]
        GPIO_P0,
        # [ doc = "DCD1" ]
        DCD1,
        # [ doc = "MOSI0" ]
        MOSI0,
        # [ doc = "MOSI" ]
        MOSI,
    }
    impl P0_18R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_18R::GPIO_P0 => 0,
                P0_18R::DCD1 => 1,
                P0_18R::MOSI0 => 2,
                P0_18R::MOSI => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_18R {
            match value {
                0 => P0_18R::GPIO_P0,
                1 => P0_18R::DCD1,
                2 => P0_18R::MOSI0,
                3 => P0_18R::MOSI,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_18R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `DCD1`" ]
        # [ inline ( always ) ]
        pub fn is_dcd1(&self) -> bool {
            *self == P0_18R::DCD1
        }
        # [ doc = "Checks if the value of the field is `MOSI0`" ]
        # [ inline ( always ) ]
        pub fn is_mosi0(&self) -> bool {
            *self == P0_18R::MOSI0
        }
        # [ doc = "Checks if the value of the field is `MOSI`" ]
        # [ inline ( always ) ]
        pub fn is_mosi(&self) -> bool {
            *self == P0_18R::MOSI
        }
    }
    # [ doc = "Possible values of the field `P0_19`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_19R {
        # [ doc = "GPIO P0.19." ]
        GPIO_P0,
        # [ doc = "DSR1" ]
        DSR1,
        # [ doc = "SDA1" ]
        SDA1,
    }
    impl P0_19R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_19R::GPIO_P0 => 0,
                P0_19R::DSR1 => 1,
                P0_19R::SDA1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_19R {
            match value {
                0 => P0_19R::GPIO_P0,
                1 => P0_19R::DSR1,
                3 => P0_19R::SDA1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_19R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `DSR1`" ]
        # [ inline ( always ) ]
        pub fn is_dsr1(&self) -> bool {
            *self == P0_19R::DSR1
        }
        # [ doc = "Checks if the value of the field is `SDA1`" ]
        # [ inline ( always ) ]
        pub fn is_sda1(&self) -> bool {
            *self == P0_19R::SDA1
        }
    }
    # [ doc = "Possible values of the field `P0_20`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_20R {
        # [ doc = "GPIO P0.20." ]
        GPIO_P0,
        # [ doc = "DTR1" ]
        DTR1,
        # [ doc = "SCL1" ]
        SCL1,
    }
    impl P0_20R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_20R::GPIO_P0 => 0,
                P0_20R::DTR1 => 1,
                P0_20R::SCL1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_20R {
            match value {
                0 => P0_20R::GPIO_P0,
                1 => P0_20R::DTR1,
                3 => P0_20R::SCL1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_20R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `DTR1`" ]
        # [ inline ( always ) ]
        pub fn is_dtr1(&self) -> bool {
            *self == P0_20R::DTR1
        }
        # [ doc = "Checks if the value of the field is `SCL1`" ]
        # [ inline ( always ) ]
        pub fn is_scl1(&self) -> bool {
            *self == P0_20R::SCL1
        }
    }
    # [ doc = "Possible values of the field `P0_21`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_21R {
        # [ doc = "GPIO Port 0.21." ]
        GPIO_PORT_0,
        # [ doc = "RI1" ]
        RI1,
        # [ doc = "RD1" ]
        RD1,
    }
    impl P0_21R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_21R::GPIO_PORT_0 => 0,
                P0_21R::RI1 => 1,
                P0_21R::RD1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_21R {
            match value {
                0 => P0_21R::GPIO_PORT_0,
                1 => P0_21R::RI1,
                3 => P0_21R::RD1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_PORT_0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_port_0(&self) -> bool {
            *self == P0_21R::GPIO_PORT_0
        }
        # [ doc = "Checks if the value of the field is `RI1`" ]
        # [ inline ( always ) ]
        pub fn is_ri1(&self) -> bool {
            *self == P0_21R::RI1
        }
        # [ doc = "Checks if the value of the field is `RD1`" ]
        # [ inline ( always ) ]
        pub fn is_rd1(&self) -> bool {
            *self == P0_21R::RD1
        }
    }
    # [ doc = "Possible values of the field `P0_22`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_22R {
        # [ doc = "GPIO P0.22." ]
        GPIO_P0,
        # [ doc = "RTS1" ]
        RTS1,
        # [ doc = "TD1" ]
        TD1,
    }
    impl P0_22R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_22R::GPIO_P0 => 0,
                P0_22R::RTS1 => 1,
                P0_22R::TD1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_22R {
            match value {
                0 => P0_22R::GPIO_P0,
                1 => P0_22R::RTS1,
                3 => P0_22R::TD1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_22R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `RTS1`" ]
        # [ inline ( always ) ]
        pub fn is_rts1(&self) -> bool {
            *self == P0_22R::RTS1
        }
        # [ doc = "Checks if the value of the field is `TD1`" ]
        # [ inline ( always ) ]
        pub fn is_td1(&self) -> bool {
            *self == P0_22R::TD1
        }
    }
    # [ doc = "Possible values of the field `P0_23`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_23R {
        # [ doc = "GPIO P0.23." ]
        GPIO_P0,
        # [ doc = "AD0.0" ]
        AD0,
        # [ doc = "I2SRX_CLK" ]
        I2SRX_CLK,
        # [ doc = "CAP3.0" ]
        CAP3,
    }
    impl P0_23R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_23R::GPIO_P0 => 0,
                P0_23R::AD0 => 1,
                P0_23R::I2SRX_CLK => 2,
                P0_23R::CAP3 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_23R {
            match value {
                0 => P0_23R::GPIO_P0,
                1 => P0_23R::AD0,
                2 => P0_23R::I2SRX_CLK,
                3 => P0_23R::CAP3,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_23R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `AD0`" ]
        # [ inline ( always ) ]
        pub fn is_ad0(&self) -> bool {
            *self == P0_23R::AD0
        }
        # [ doc = "Checks if the value of the field is `I2SRX_CLK`" ]
        # [ inline ( always ) ]
        pub fn is_i2srx_clk(&self) -> bool {
            *self == P0_23R::I2SRX_CLK
        }
        # [ doc = "Checks if the value of the field is `CAP3`" ]
        # [ inline ( always ) ]
        pub fn is_cap3(&self) -> bool {
            *self == P0_23R::CAP3
        }
    }
    # [ doc = "Possible values of the field `P0_24`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_24R {
        # [ doc = "GPIO P0.24." ]
        GPIO_P0,
        # [ doc = "AD0.1" ]
        AD0,
        # [ doc = "I2SRX_WS" ]
        I2SRX_WS,
        # [ doc = "CAP3.1" ]
        CAP3,
    }
    impl P0_24R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_24R::GPIO_P0 => 0,
                P0_24R::AD0 => 1,
                P0_24R::I2SRX_WS => 2,
                P0_24R::CAP3 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_24R {
            match value {
                0 => P0_24R::GPIO_P0,
                1 => P0_24R::AD0,
                2 => P0_24R::I2SRX_WS,
                3 => P0_24R::CAP3,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_24R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `AD0`" ]
        # [ inline ( always ) ]
        pub fn is_ad0(&self) -> bool {
            *self == P0_24R::AD0
        }
        # [ doc = "Checks if the value of the field is `I2SRX_WS`" ]
        # [ inline ( always ) ]
        pub fn is_i2srx_ws(&self) -> bool {
            *self == P0_24R::I2SRX_WS
        }
        # [ doc = "Checks if the value of the field is `CAP3`" ]
        # [ inline ( always ) ]
        pub fn is_cap3(&self) -> bool {
            *self == P0_24R::CAP3
        }
    }
    # [ doc = "Possible values of the field `P0_25`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_25R {
        # [ doc = "GPIO P0.25" ]
        GPIO_P0,
        # [ doc = "AD0.2" ]
        AD0,
        # [ doc = "I2SRX_SDA" ]
        I2SRX_SDA,
        # [ doc = "TXD3" ]
        TXD3,
    }
    impl P0_25R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_25R::GPIO_P0 => 0,
                P0_25R::AD0 => 1,
                P0_25R::I2SRX_SDA => 2,
                P0_25R::TXD3 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_25R {
            match value {
                0 => P0_25R::GPIO_P0,
                1 => P0_25R::AD0,
                2 => P0_25R::I2SRX_SDA,
                3 => P0_25R::TXD3,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_25R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `AD0`" ]
        # [ inline ( always ) ]
        pub fn is_ad0(&self) -> bool {
            *self == P0_25R::AD0
        }
        # [ doc = "Checks if the value of the field is `I2SRX_SDA`" ]
        # [ inline ( always ) ]
        pub fn is_i2srx_sda(&self) -> bool {
            *self == P0_25R::I2SRX_SDA
        }
        # [ doc = "Checks if the value of the field is `TXD3`" ]
        # [ inline ( always ) ]
        pub fn is_txd3(&self) -> bool {
            *self == P0_25R::TXD3
        }
    }
    # [ doc = "Possible values of the field `P0_26`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_26R {
        # [ doc = "GPIO P0.26" ]
        GPIO_P0,
        # [ doc = "AD0.3" ]
        AD0,
        # [ doc = "AOUT" ]
        AOUT,
        # [ doc = "RXD3" ]
        RXD3,
    }
    impl P0_26R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_26R::GPIO_P0 => 0,
                P0_26R::AD0 => 1,
                P0_26R::AOUT => 2,
                P0_26R::RXD3 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_26R {
            match value {
                0 => P0_26R::GPIO_P0,
                1 => P0_26R::AD0,
                2 => P0_26R::AOUT,
                3 => P0_26R::RXD3,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_26R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `AD0`" ]
        # [ inline ( always ) ]
        pub fn is_ad0(&self) -> bool {
            *self == P0_26R::AD0
        }
        # [ doc = "Checks if the value of the field is `AOUT`" ]
        # [ inline ( always ) ]
        pub fn is_aout(&self) -> bool {
            *self == P0_26R::AOUT
        }
        # [ doc = "Checks if the value of the field is `RXD3`" ]
        # [ inline ( always ) ]
        pub fn is_rxd3(&self) -> bool {
            *self == P0_26R::RXD3
        }
    }
    # [ doc = "Possible values of the field `P0_27`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_27R {
        # [ doc = "GPIO P0.27" ]
        GPIO_P0,
        # [ doc = "SDA0" ]
        SDA0,
        # [ doc = "USB_SDA" ]
        USB_SDA,
    }
    impl P0_27R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_27R::GPIO_P0 => 0,
                P0_27R::SDA0 => 1,
                P0_27R::USB_SDA => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_27R {
            match value {
                0 => P0_27R::GPIO_P0,
                1 => P0_27R::SDA0,
                2 => P0_27R::USB_SDA,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_27R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `SDA0`" ]
        # [ inline ( always ) ]
        pub fn is_sda0(&self) -> bool {
            *self == P0_27R::SDA0
        }
        # [ doc = "Checks if the value of the field is `USB_SDA`" ]
        # [ inline ( always ) ]
        pub fn is_usb_sda(&self) -> bool {
            *self == P0_27R::USB_SDA
        }
    }
    # [ doc = "Possible values of the field `P0_28`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_28R {
        # [ doc = "GPIO P0.28" ]
        GPIO_P0,
        # [ doc = "SCL0" ]
        SCL0,
        # [ doc = "USB_SCL" ]
        USB_SCL,
    }
    impl P0_28R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_28R::GPIO_P0 => 0,
                P0_28R::SCL0 => 1,
                P0_28R::USB_SCL => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_28R {
            match value {
                0 => P0_28R::GPIO_P0,
                1 => P0_28R::SCL0,
                2 => P0_28R::USB_SCL,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_28R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `SCL0`" ]
        # [ inline ( always ) ]
        pub fn is_scl0(&self) -> bool {
            *self == P0_28R::SCL0
        }
        # [ doc = "Checks if the value of the field is `USB_SCL`" ]
        # [ inline ( always ) ]
        pub fn is_usb_scl(&self) -> bool {
            *self == P0_28R::USB_SCL
        }
    }
    # [ doc = "Possible values of the field `P0_29`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_29R {
        # [ doc = "GPIO P0.29" ]
        GPIO_P0,
        # [ doc = "USB_D+" ]
        USB_DP,
    }
    impl P0_29R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_29R::GPIO_P0 => 0,
                P0_29R::USB_DP => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_29R {
            match value {
                0 => P0_29R::GPIO_P0,
                1 => P0_29R::USB_DP,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_29R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `USB_DP`" ]
        # [ inline ( always ) ]
        pub fn is_usb_dp(&self) -> bool {
            *self == P0_29R::USB_DP
        }
    }
    # [ doc = "Possible values of the field `P0_30`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_30R {
        # [ doc = "GPIO P0.30" ]
        GPIO_P0,
        # [ doc = "USB_D-" ]
        USB_DM,
    }
    impl P0_30R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_30R::GPIO_P0 => 0,
                P0_30R::USB_DM => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_30R {
            match value {
                0 => P0_30R::GPIO_P0,
                1 => P0_30R::USB_DM,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P0`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p0(&self) -> bool {
            *self == P0_30R::GPIO_P0
        }
        # [ doc = "Checks if the value of the field is `USB_DM`" ]
        # [ inline ( always ) ]
        pub fn is_usb_dm(&self) -> bool {
            *self == P0_30R::USB_DM
        }
    }
    # [ doc = "Values that can be written to the field `P0_16`" ]
    pub enum P0_16W {
        # [ doc = "GPIO P0.16" ]
        GPIO_P0,
        # [ doc = "RXD1" ]
        RXD1,
        # [ doc = "SSEL0" ]
        SSEL0,
        # [ doc = "SSEL" ]
        SSEL,
    }
    impl P0_16W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_16W::GPIO_P0 => 0,
                P0_16W::RXD1 => 1,
                P0_16W::SSEL0 => 2,
                P0_16W::SSEL => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_16W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_16W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.16" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_16W::GPIO_P0)
        }
        # [ doc = "RXD1" ]
        # [ inline ( always ) ]
        pub fn rxd1(self) -> &'a mut W {
            self.variant(P0_16W::RXD1)
        }
        # [ doc = "SSEL0" ]
        # [ inline ( always ) ]
        pub fn ssel0(self) -> &'a mut W {
            self.variant(P0_16W::SSEL0)
        }
        # [ doc = "SSEL" ]
        # [ inline ( always ) ]
        pub fn ssel(self) -> &'a mut W {
            self.variant(P0_16W::SSEL)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_17`" ]
    pub enum P0_17W {
        # [ doc = "GPIO P0.17" ]
        GPIO_P0,
        # [ doc = "CTS1" ]
        CTS1,
        # [ doc = "MISO0" ]
        MISO0,
        # [ doc = "MISO" ]
        MISO,
    }
    impl P0_17W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_17W::GPIO_P0 => 0,
                P0_17W::CTS1 => 1,
                P0_17W::MISO0 => 2,
                P0_17W::MISO => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_17W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_17W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_17W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.17" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_17W::GPIO_P0)
        }
        # [ doc = "CTS1" ]
        # [ inline ( always ) ]
        pub fn cts1(self) -> &'a mut W {
            self.variant(P0_17W::CTS1)
        }
        # [ doc = "MISO0" ]
        # [ inline ( always ) ]
        pub fn miso0(self) -> &'a mut W {
            self.variant(P0_17W::MISO0)
        }
        # [ doc = "MISO" ]
        # [ inline ( always ) ]
        pub fn miso(self) -> &'a mut W {
            self.variant(P0_17W::MISO)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_18`" ]
    pub enum P0_18W {
        # [ doc = "GPIO P0.18" ]
        GPIO_P0,
        # [ doc = "DCD1" ]
        DCD1,
        # [ doc = "MOSI0" ]
        MOSI0,
        # [ doc = "MOSI" ]
        MOSI,
    }
    impl P0_18W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_18W::GPIO_P0 => 0,
                P0_18W::DCD1 => 1,
                P0_18W::MOSI0 => 2,
                P0_18W::MOSI => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_18W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_18W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_18W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.18" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_18W::GPIO_P0)
        }
        # [ doc = "DCD1" ]
        # [ inline ( always ) ]
        pub fn dcd1(self) -> &'a mut W {
            self.variant(P0_18W::DCD1)
        }
        # [ doc = "MOSI0" ]
        # [ inline ( always ) ]
        pub fn mosi0(self) -> &'a mut W {
            self.variant(P0_18W::MOSI0)
        }
        # [ doc = "MOSI" ]
        # [ inline ( always ) ]
        pub fn mosi(self) -> &'a mut W {
            self.variant(P0_18W::MOSI)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_19`" ]
    pub enum P0_19W {
        # [ doc = "GPIO P0.19." ]
        GPIO_P0,
        # [ doc = "DSR1" ]
        DSR1,
        # [ doc = "SDA1" ]
        SDA1,
    }
    impl P0_19W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_19W::GPIO_P0 => 0,
                P0_19W::DSR1 => 1,
                P0_19W::SDA1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_19W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_19W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_19W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P0.19." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_19W::GPIO_P0)
        }
        # [ doc = "DSR1" ]
        # [ inline ( always ) ]
        pub fn dsr1(self) -> &'a mut W {
            self.variant(P0_19W::DSR1)
        }
        # [ doc = "SDA1" ]
        # [ inline ( always ) ]
        pub fn sda1(self) -> &'a mut W {
            self.variant(P0_19W::SDA1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_20`" ]
    pub enum P0_20W {
        # [ doc = "GPIO P0.20." ]
        GPIO_P0,
        # [ doc = "DTR1" ]
        DTR1,
        # [ doc = "SCL1" ]
        SCL1,
    }
    impl P0_20W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_20W::GPIO_P0 => 0,
                P0_20W::DTR1 => 1,
                P0_20W::SCL1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_20W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_20W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_20W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P0.20." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_20W::GPIO_P0)
        }
        # [ doc = "DTR1" ]
        # [ inline ( always ) ]
        pub fn dtr1(self) -> &'a mut W {
            self.variant(P0_20W::DTR1)
        }
        # [ doc = "SCL1" ]
        # [ inline ( always ) ]
        pub fn scl1(self) -> &'a mut W {
            self.variant(P0_20W::SCL1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_21`" ]
    pub enum P0_21W {
        # [ doc = "GPIO Port 0.21." ]
        GPIO_PORT_0,
        # [ doc = "RI1" ]
        RI1,
        # [ doc = "RD1" ]
        RD1,
    }
    impl P0_21W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_21W::GPIO_PORT_0 => 0,
                P0_21W::RI1 => 1,
                P0_21W::RD1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_21W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_21W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_21W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO Port 0.21." ]
        # [ inline ( always ) ]
        pub fn gpio_port_0(self) -> &'a mut W {
            self.variant(P0_21W::GPIO_PORT_0)
        }
        # [ doc = "RI1" ]
        # [ inline ( always ) ]
        pub fn ri1(self) -> &'a mut W {
            self.variant(P0_21W::RI1)
        }
        # [ doc = "RD1" ]
        # [ inline ( always ) ]
        pub fn rd1(self) -> &'a mut W {
            self.variant(P0_21W::RD1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_22`" ]
    pub enum P0_22W {
        # [ doc = "GPIO P0.22." ]
        GPIO_P0,
        # [ doc = "RTS1" ]
        RTS1,
        # [ doc = "TD1" ]
        TD1,
    }
    impl P0_22W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_22W::GPIO_P0 => 0,
                P0_22W::RTS1 => 1,
                P0_22W::TD1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_22W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_22W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_22W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P0.22." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_22W::GPIO_P0)
        }
        # [ doc = "RTS1" ]
        # [ inline ( always ) ]
        pub fn rts1(self) -> &'a mut W {
            self.variant(P0_22W::RTS1)
        }
        # [ doc = "TD1" ]
        # [ inline ( always ) ]
        pub fn td1(self) -> &'a mut W {
            self.variant(P0_22W::TD1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_23`" ]
    pub enum P0_23W {
        # [ doc = "GPIO P0.23." ]
        GPIO_P0,
        # [ doc = "AD0.0" ]
        AD0,
        # [ doc = "I2SRX_CLK" ]
        I2SRX_CLK,
        # [ doc = "CAP3.0" ]
        CAP3,
    }
    impl P0_23W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_23W::GPIO_P0 => 0,
                P0_23W::AD0 => 1,
                P0_23W::I2SRX_CLK => 2,
                P0_23W::CAP3 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_23W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_23W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_23W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.23." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_23W::GPIO_P0)
        }
        # [ doc = "AD0.0" ]
        # [ inline ( always ) ]
        pub fn ad0(self) -> &'a mut W {
            self.variant(P0_23W::AD0)
        }
        # [ doc = "I2SRX_CLK" ]
        # [ inline ( always ) ]
        pub fn i2srx_clk(self) -> &'a mut W {
            self.variant(P0_23W::I2SRX_CLK)
        }
        # [ doc = "CAP3.0" ]
        # [ inline ( always ) ]
        pub fn cap3(self) -> &'a mut W {
            self.variant(P0_23W::CAP3)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_24`" ]
    pub enum P0_24W {
        # [ doc = "GPIO P0.24." ]
        GPIO_P0,
        # [ doc = "AD0.1" ]
        AD0,
        # [ doc = "I2SRX_WS" ]
        I2SRX_WS,
        # [ doc = "CAP3.1" ]
        CAP3,
    }
    impl P0_24W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_24W::GPIO_P0 => 0,
                P0_24W::AD0 => 1,
                P0_24W::I2SRX_WS => 2,
                P0_24W::CAP3 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_24W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_24W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_24W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.24." ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_24W::GPIO_P0)
        }
        # [ doc = "AD0.1" ]
        # [ inline ( always ) ]
        pub fn ad0(self) -> &'a mut W {
            self.variant(P0_24W::AD0)
        }
        # [ doc = "I2SRX_WS" ]
        # [ inline ( always ) ]
        pub fn i2srx_ws(self) -> &'a mut W {
            self.variant(P0_24W::I2SRX_WS)
        }
        # [ doc = "CAP3.1" ]
        # [ inline ( always ) ]
        pub fn cap3(self) -> &'a mut W {
            self.variant(P0_24W::CAP3)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_25`" ]
    pub enum P0_25W {
        # [ doc = "GPIO P0.25" ]
        GPIO_P0,
        # [ doc = "AD0.2" ]
        AD0,
        # [ doc = "I2SRX_SDA" ]
        I2SRX_SDA,
        # [ doc = "TXD3" ]
        TXD3,
    }
    impl P0_25W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_25W::GPIO_P0 => 0,
                P0_25W::AD0 => 1,
                P0_25W::I2SRX_SDA => 2,
                P0_25W::TXD3 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_25W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_25W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_25W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.25" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_25W::GPIO_P0)
        }
        # [ doc = "AD0.2" ]
        # [ inline ( always ) ]
        pub fn ad0(self) -> &'a mut W {
            self.variant(P0_25W::AD0)
        }
        # [ doc = "I2SRX_SDA" ]
        # [ inline ( always ) ]
        pub fn i2srx_sda(self) -> &'a mut W {
            self.variant(P0_25W::I2SRX_SDA)
        }
        # [ doc = "TXD3" ]
        # [ inline ( always ) ]
        pub fn txd3(self) -> &'a mut W {
            self.variant(P0_25W::TXD3)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_26`" ]
    pub enum P0_26W {
        # [ doc = "GPIO P0.26" ]
        GPIO_P0,
        # [ doc = "AD0.3" ]
        AD0,
        # [ doc = "AOUT" ]
        AOUT,
        # [ doc = "RXD3" ]
        RXD3,
    }
    impl P0_26W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_26W::GPIO_P0 => 0,
                P0_26W::AD0 => 1,
                P0_26W::AOUT => 2,
                P0_26W::RXD3 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_26W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_26W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_26W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P0.26" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_26W::GPIO_P0)
        }
        # [ doc = "AD0.3" ]
        # [ inline ( always ) ]
        pub fn ad0(self) -> &'a mut W {
            self.variant(P0_26W::AD0)
        }
        # [ doc = "AOUT" ]
        # [ inline ( always ) ]
        pub fn aout(self) -> &'a mut W {
            self.variant(P0_26W::AOUT)
        }
        # [ doc = "RXD3" ]
        # [ inline ( always ) ]
        pub fn rxd3(self) -> &'a mut W {
            self.variant(P0_26W::RXD3)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_27`" ]
    pub enum P0_27W {
        # [ doc = "GPIO P0.27" ]
        GPIO_P0,
        # [ doc = "SDA0" ]
        SDA0,
        # [ doc = "USB_SDA" ]
        USB_SDA,
    }
    impl P0_27W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_27W::GPIO_P0 => 0,
                P0_27W::SDA0 => 1,
                P0_27W::USB_SDA => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_27W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_27W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_27W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P0.27" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_27W::GPIO_P0)
        }
        # [ doc = "SDA0" ]
        # [ inline ( always ) ]
        pub fn sda0(self) -> &'a mut W {
            self.variant(P0_27W::SDA0)
        }
        # [ doc = "USB_SDA" ]
        # [ inline ( always ) ]
        pub fn usb_sda(self) -> &'a mut W {
            self.variant(P0_27W::USB_SDA)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_28`" ]
    pub enum P0_28W {
        # [ doc = "GPIO P0.28" ]
        GPIO_P0,
        # [ doc = "SCL0" ]
        SCL0,
        # [ doc = "USB_SCL" ]
        USB_SCL,
    }
    impl P0_28W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_28W::GPIO_P0 => 0,
                P0_28W::SCL0 => 1,
                P0_28W::USB_SCL => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_28W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_28W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_28W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P0.28" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_28W::GPIO_P0)
        }
        # [ doc = "SCL0" ]
        # [ inline ( always ) ]
        pub fn scl0(self) -> &'a mut W {
            self.variant(P0_28W::SCL0)
        }
        # [ doc = "USB_SCL" ]
        # [ inline ( always ) ]
        pub fn usb_scl(self) -> &'a mut W {
            self.variant(P0_28W::USB_SCL)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_29`" ]
    pub enum P0_29W {
        # [ doc = "GPIO P0.29" ]
        GPIO_P0,
        # [ doc = "USB_D+" ]
        USB_DP,
    }
    impl P0_29W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_29W::GPIO_P0 => 0,
                P0_29W::USB_DP => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_29W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_29W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_29W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P0.29" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_29W::GPIO_P0)
        }
        # [ doc = "USB_D+" ]
        # [ inline ( always ) ]
        pub fn usb_dp(self) -> &'a mut W {
            self.variant(P0_29W::USB_DP)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_30`" ]
    pub enum P0_30W {
        # [ doc = "GPIO P0.30" ]
        GPIO_P0,
        # [ doc = "USB_D-" ]
        USB_DM,
    }
    impl P0_30W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_30W::GPIO_P0 => 0,
                P0_30W::USB_DM => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_30W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_30W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_30W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P0.30" ]
        # [ inline ( always ) ]
        pub fn gpio_p0(self) -> &'a mut W {
            self.variant(P0_30W::GPIO_P0)
        }
        # [ doc = "USB_D-" ]
        # [ inline ( always ) ]
        pub fn usb_dm(self) -> &'a mut W {
            self.variant(P0_30W::USB_DM)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Pin function select P0.16." ]
        # [ inline ( always ) ]
        pub fn p0_16(&self) -> P0_16R {
            P0_16R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 2:3 - Pin function select P0.17." ]
        # [ inline ( always ) ]
        pub fn p0_17(&self) -> P0_17R {
            P0_17R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 2;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 4:5 - Pin function select P0.18." ]
        # [ inline ( always ) ]
        pub fn p0_18(&self) -> P0_18R {
            P0_18R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 4;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 6:7 - Pin function select P019." ]
        # [ inline ( always ) ]
        pub fn p0_19(&self) -> P0_19R {
            P0_19R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 6;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 8:9 - Pin function select P0.20." ]
        # [ inline ( always ) ]
        pub fn p0_20(&self) -> P0_20R {
            P0_20R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 8;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 10:11 - Pin function select P0.21." ]
        # [ inline ( always ) ]
        pub fn p0_21(&self) -> P0_21R {
            P0_21R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 10;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 12:13 - Pin function select P022" ]
        # [ inline ( always ) ]
        pub fn p0_22(&self) -> P0_22R {
            P0_22R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 12;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 14:15 - Pin function select P023." ]
        # [ inline ( always ) ]
        pub fn p0_23(&self) -> P0_23R {
            P0_23R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 14;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 16:17 - Pin function select P0.24." ]
        # [ inline ( always ) ]
        pub fn p0_24(&self) -> P0_24R {
            P0_24R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 16;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 18:19 - Pin function select P0.25." ]
        # [ inline ( always ) ]
        pub fn p0_25(&self) -> P0_25R {
            P0_25R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 18;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 20:21 - Pin function select P0.26." ]
        # [ inline ( always ) ]
        pub fn p0_26(&self) -> P0_26R {
            P0_26R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 20;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 22:23 - Pin function select P0.27." ]
        # [ inline ( always ) ]
        pub fn p0_27(&self) -> P0_27R {
            P0_27R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 22;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 24:25 - Pin function select P0.28." ]
        # [ inline ( always ) ]
        pub fn p0_28(&self) -> P0_28R {
            P0_28R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 24;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 26:27 - Pin function select P0.29" ]
        # [ inline ( always ) ]
        pub fn p0_29(&self) -> P0_29R {
            P0_29R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 26;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 28:29 - Pin function select P0.30." ]
        # [ inline ( always ) ]
        pub fn p0_30(&self) -> P0_30R {
            P0_30R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 28;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Pin function select P0.16." ]
        # [ inline ( always ) ]
        pub fn p0_16(&mut self) -> _P0_16W {
            _P0_16W { w: self }
        }
        # [ doc = "Bits 2:3 - Pin function select P0.17." ]
        # [ inline ( always ) ]
        pub fn p0_17(&mut self) -> _P0_17W {
            _P0_17W { w: self }
        }
        # [ doc = "Bits 4:5 - Pin function select P0.18." ]
        # [ inline ( always ) ]
        pub fn p0_18(&mut self) -> _P0_18W {
            _P0_18W { w: self }
        }
        # [ doc = "Bits 6:7 - Pin function select P019." ]
        # [ inline ( always ) ]
        pub fn p0_19(&mut self) -> _P0_19W {
            _P0_19W { w: self }
        }
        # [ doc = "Bits 8:9 - Pin function select P0.20." ]
        # [ inline ( always ) ]
        pub fn p0_20(&mut self) -> _P0_20W {
            _P0_20W { w: self }
        }
        # [ doc = "Bits 10:11 - Pin function select P0.21." ]
        # [ inline ( always ) ]
        pub fn p0_21(&mut self) -> _P0_21W {
            _P0_21W { w: self }
        }
        # [ doc = "Bits 12:13 - Pin function select P022" ]
        # [ inline ( always ) ]
        pub fn p0_22(&mut self) -> _P0_22W {
            _P0_22W { w: self }
        }
        # [ doc = "Bits 14:15 - Pin function select P023." ]
        # [ inline ( always ) ]
        pub fn p0_23(&mut self) -> _P0_23W {
            _P0_23W { w: self }
        }
        # [ doc = "Bits 16:17 - Pin function select P0.24." ]
        # [ inline ( always ) ]
        pub fn p0_24(&mut self) -> _P0_24W {
            _P0_24W { w: self }
        }
        # [ doc = "Bits 18:19 - Pin function select P0.25." ]
        # [ inline ( always ) ]
        pub fn p0_25(&mut self) -> _P0_25W {
            _P0_25W { w: self }
        }
        # [ doc = "Bits 20:21 - Pin function select P0.26." ]
        # [ inline ( always ) ]
        pub fn p0_26(&mut self) -> _P0_26W {
            _P0_26W { w: self }
        }
        # [ doc = "Bits 22:23 - Pin function select P0.27." ]
        # [ inline ( always ) ]
        pub fn p0_27(&mut self) -> _P0_27W {
            _P0_27W { w: self }
        }
        # [ doc = "Bits 24:25 - Pin function select P0.28." ]
        # [ inline ( always ) ]
        pub fn p0_28(&mut self) -> _P0_28W {
            _P0_28W { w: self }
        }
        # [ doc = "Bits 26:27 - Pin function select P0.29" ]
        # [ inline ( always ) ]
        pub fn p0_29(&mut self) -> _P0_29W {
            _P0_29W { w: self }
        }
        # [ doc = "Bits 28:29 - Pin function select P0.30." ]
        # [ inline ( always ) ]
        pub fn p0_30(&mut self) -> _P0_30W {
            _P0_30W { w: self }
        }
    }
}
# [ doc = "Pin function select register 2." ]
pub struct PINSEL2 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin function select register 2." ]
pub mod pinsel2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINSEL2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P1_0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_0R {
        # [ doc = "GPIO P1.0" ]
        GPIO_P1,
        # [ doc = "ENET_TXD0" ]
        ENET_TXD0,
    }
    impl P1_0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_0R::GPIO_P1 => 0,
                P1_0R::ENET_TXD0 => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_0R {
            match value {
                0 => P1_0R::GPIO_P1,
                1 => P1_0R::ENET_TXD0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_0R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `ENET_TXD0`" ]
        # [ inline ( always ) ]
        pub fn is_enet_txd0(&self) -> bool {
            *self == P1_0R::ENET_TXD0
        }
    }
    # [ doc = "Possible values of the field `P1_1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_1R {
        # [ doc = "GPIO P1.1" ]
        GPIO_P1,
        # [ doc = "ENET_TXD1" ]
        ENET_TXD1,
    }
    impl P1_1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_1R::GPIO_P1 => 0,
                P1_1R::ENET_TXD1 => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_1R {
            match value {
                0 => P1_1R::GPIO_P1,
                1 => P1_1R::ENET_TXD1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_1R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `ENET_TXD1`" ]
        # [ inline ( always ) ]
        pub fn is_enet_txd1(&self) -> bool {
            *self == P1_1R::ENET_TXD1
        }
    }
    # [ doc = "Possible values of the field `P1_4`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_4R {
        # [ doc = "GPIO P1.4." ]
        GPIO_P1,
        # [ doc = "ENET_TX_EN" ]
        ENET_TX_EN,
    }
    impl P1_4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_4R::GPIO_P1 => 0,
                P1_4R::ENET_TX_EN => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_4R {
            match value {
                0 => P1_4R::GPIO_P1,
                1 => P1_4R::ENET_TX_EN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_4R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `ENET_TX_EN`" ]
        # [ inline ( always ) ]
        pub fn is_enet_tx_en(&self) -> bool {
            *self == P1_4R::ENET_TX_EN
        }
    }
    # [ doc = "Possible values of the field `P1_8`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_8R {
        # [ doc = "GPIO P1.8." ]
        GPIO_P1,
        # [ doc = "ENET_CRS" ]
        ENET_CRS,
    }
    impl P1_8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_8R::GPIO_P1 => 0,
                P1_8R::ENET_CRS => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_8R {
            match value {
                0 => P1_8R::GPIO_P1,
                1 => P1_8R::ENET_CRS,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_8R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `ENET_CRS`" ]
        # [ inline ( always ) ]
        pub fn is_enet_crs(&self) -> bool {
            *self == P1_8R::ENET_CRS
        }
    }
    # [ doc = "Possible values of the field `P1_9`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_9R {
        # [ doc = "GPIO Port 1.9" ]
        GPIO_PORT_1,
        # [ doc = "ENET_RXD0" ]
        ENET_RXD0,
    }
    impl P1_9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_9R::GPIO_PORT_1 => 0,
                P1_9R::ENET_RXD0 => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_9R {
            match value {
                0 => P1_9R::GPIO_PORT_1,
                1 => P1_9R::ENET_RXD0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_PORT_1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_port_1(&self) -> bool {
            *self == P1_9R::GPIO_PORT_1
        }
        # [ doc = "Checks if the value of the field is `ENET_RXD0`" ]
        # [ inline ( always ) ]
        pub fn is_enet_rxd0(&self) -> bool {
            *self == P1_9R::ENET_RXD0
        }
    }
    # [ doc = "Possible values of the field `P1_10`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_10R {
        # [ doc = "GPIO P1.10" ]
        GPIO_P1,
        # [ doc = "ENET_RXD1" ]
        ENET_RXD1,
    }
    impl P1_10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_10R::GPIO_P1 => 0,
                P1_10R::ENET_RXD1 => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_10R {
            match value {
                0 => P1_10R::GPIO_P1,
                1 => P1_10R::ENET_RXD1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_10R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `ENET_RXD1`" ]
        # [ inline ( always ) ]
        pub fn is_enet_rxd1(&self) -> bool {
            *self == P1_10R::ENET_RXD1
        }
    }
    # [ doc = "Possible values of the field `P1_14`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_14R {
        # [ doc = "GPIO P1.14" ]
        GPIO_P1,
        # [ doc = "ENET_RX_ER" ]
        ENET_RX_ER,
    }
    impl P1_14R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_14R::GPIO_P1 => 0,
                P1_14R::ENET_RX_ER => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_14R {
            match value {
                0 => P1_14R::GPIO_P1,
                1 => P1_14R::ENET_RX_ER,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_14R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `ENET_RX_ER`" ]
        # [ inline ( always ) ]
        pub fn is_enet_rx_er(&self) -> bool {
            *self == P1_14R::ENET_RX_ER
        }
    }
    # [ doc = "Possible values of the field `P1_15`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_15R {
        # [ doc = "GPIO P1.15" ]
        GPIO_P1,
        # [ doc = "ENET_REF_CLK" ]
        ENET_REF_CLK,
    }
    impl P1_15R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_15R::GPIO_P1 => 0,
                P1_15R::ENET_REF_CLK => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_15R {
            match value {
                0 => P1_15R::GPIO_P1,
                1 => P1_15R::ENET_REF_CLK,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_15R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `ENET_REF_CLK`" ]
        # [ inline ( always ) ]
        pub fn is_enet_ref_clk(&self) -> bool {
            *self == P1_15R::ENET_REF_CLK
        }
    }
    # [ doc = "Values that can be written to the field `P1_0`" ]
    pub enum P1_0W {
        # [ doc = "GPIO P1.0" ]
        GPIO_P1,
        # [ doc = "ENET_TXD0" ]
        ENET_TXD0,
    }
    impl P1_0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_0W::GPIO_P1 => 0,
                P1_0W::ENET_TXD0 => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_0W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.0" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_0W::GPIO_P1)
        }
        # [ doc = "ENET_TXD0" ]
        # [ inline ( always ) ]
        pub fn enet_txd0(self) -> &'a mut W {
            self.variant(P1_0W::ENET_TXD0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_1`" ]
    pub enum P1_1W {
        # [ doc = "GPIO P1.1" ]
        GPIO_P1,
        # [ doc = "ENET_TXD1" ]
        ENET_TXD1,
    }
    impl P1_1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_1W::GPIO_P1 => 0,
                P1_1W::ENET_TXD1 => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_1W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.1" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_1W::GPIO_P1)
        }
        # [ doc = "ENET_TXD1" ]
        # [ inline ( always ) ]
        pub fn enet_txd1(self) -> &'a mut W {
            self.variant(P1_1W::ENET_TXD1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_4`" ]
    pub enum P1_4W {
        # [ doc = "GPIO P1.4." ]
        GPIO_P1,
        # [ doc = "ENET_TX_EN" ]
        ENET_TX_EN,
    }
    impl P1_4W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_4W::GPIO_P1 => 0,
                P1_4W::ENET_TX_EN => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_4W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.4." ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_4W::GPIO_P1)
        }
        # [ doc = "ENET_TX_EN" ]
        # [ inline ( always ) ]
        pub fn enet_tx_en(self) -> &'a mut W {
            self.variant(P1_4W::ENET_TX_EN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_8`" ]
    pub enum P1_8W {
        # [ doc = "GPIO P1.8." ]
        GPIO_P1,
        # [ doc = "ENET_CRS" ]
        ENET_CRS,
    }
    impl P1_8W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_8W::GPIO_P1 => 0,
                P1_8W::ENET_CRS => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_8W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.8." ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_8W::GPIO_P1)
        }
        # [ doc = "ENET_CRS" ]
        # [ inline ( always ) ]
        pub fn enet_crs(self) -> &'a mut W {
            self.variant(P1_8W::ENET_CRS)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_9`" ]
    pub enum P1_9W {
        # [ doc = "GPIO Port 1.9" ]
        GPIO_PORT_1,
        # [ doc = "ENET_RXD0" ]
        ENET_RXD0,
    }
    impl P1_9W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_9W::GPIO_PORT_1 => 0,
                P1_9W::ENET_RXD0 => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_9W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO Port 1.9" ]
        # [ inline ( always ) ]
        pub fn gpio_port_1(self) -> &'a mut W {
            self.variant(P1_9W::GPIO_PORT_1)
        }
        # [ doc = "ENET_RXD0" ]
        # [ inline ( always ) ]
        pub fn enet_rxd0(self) -> &'a mut W {
            self.variant(P1_9W::ENET_RXD0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_10`" ]
    pub enum P1_10W {
        # [ doc = "GPIO P1.10" ]
        GPIO_P1,
        # [ doc = "ENET_RXD1" ]
        ENET_RXD1,
    }
    impl P1_10W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_10W::GPIO_P1 => 0,
                P1_10W::ENET_RXD1 => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_10W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.10" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_10W::GPIO_P1)
        }
        # [ doc = "ENET_RXD1" ]
        # [ inline ( always ) ]
        pub fn enet_rxd1(self) -> &'a mut W {
            self.variant(P1_10W::ENET_RXD1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_14`" ]
    pub enum P1_14W {
        # [ doc = "GPIO P1.14" ]
        GPIO_P1,
        # [ doc = "ENET_RX_ER" ]
        ENET_RX_ER,
    }
    impl P1_14W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_14W::GPIO_P1 => 0,
                P1_14W::ENET_RX_ER => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_14W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.14" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_14W::GPIO_P1)
        }
        # [ doc = "ENET_RX_ER" ]
        # [ inline ( always ) ]
        pub fn enet_rx_er(self) -> &'a mut W {
            self.variant(P1_14W::ENET_RX_ER)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_15`" ]
    pub enum P1_15W {
        # [ doc = "GPIO P1.15" ]
        GPIO_P1,
        # [ doc = "ENET_REF_CLK" ]
        ENET_REF_CLK,
    }
    impl P1_15W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_15W::GPIO_P1 => 0,
                P1_15W::ENET_REF_CLK => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_15W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.15" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_15W::GPIO_P1)
        }
        # [ doc = "ENET_REF_CLK" ]
        # [ inline ( always ) ]
        pub fn enet_ref_clk(self) -> &'a mut W {
            self.variant(P1_15W::ENET_REF_CLK)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Pin function select P1.0." ]
        # [ inline ( always ) ]
        pub fn p1_0(&self) -> P1_0R {
            P1_0R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 0;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 2:3 - Pin function select P1.1." ]
        # [ inline ( always ) ]
        pub fn p1_1(&self) -> P1_1R {
            P1_1R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 2;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 8:9 - Pin function select P1.4." ]
        # [ inline ( always ) ]
        pub fn p1_4(&self) -> P1_4R {
            P1_4R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 8;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 16:17 - Pin function select P1.8." ]
        # [ inline ( always ) ]
        pub fn p1_8(&self) -> P1_8R {
            P1_8R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 16;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 18:19 - Pin function select P1.9." ]
        # [ inline ( always ) ]
        pub fn p1_9(&self) -> P1_9R {
            P1_9R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 18;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 20:21 - Pin function select P1.10." ]
        # [ inline ( always ) ]
        pub fn p1_10(&self) -> P1_10R {
            P1_10R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 20;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 22:23 - Pin function select P1.14." ]
        # [ inline ( always ) ]
        pub fn p1_14(&self) -> P1_14R {
            P1_14R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 22;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 30:31 - Pin function select P1.15." ]
        # [ inline ( always ) ]
        pub fn p1_15(&self) -> P1_15R {
            P1_15R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 30;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Pin function select P1.0." ]
        # [ inline ( always ) ]
        pub fn p1_0(&mut self) -> _P1_0W {
            _P1_0W { w: self }
        }
        # [ doc = "Bits 2:3 - Pin function select P1.1." ]
        # [ inline ( always ) ]
        pub fn p1_1(&mut self) -> _P1_1W {
            _P1_1W { w: self }
        }
        # [ doc = "Bits 8:9 - Pin function select P1.4." ]
        # [ inline ( always ) ]
        pub fn p1_4(&mut self) -> _P1_4W {
            _P1_4W { w: self }
        }
        # [ doc = "Bits 16:17 - Pin function select P1.8." ]
        # [ inline ( always ) ]
        pub fn p1_8(&mut self) -> _P1_8W {
            _P1_8W { w: self }
        }
        # [ doc = "Bits 18:19 - Pin function select P1.9." ]
        # [ inline ( always ) ]
        pub fn p1_9(&mut self) -> _P1_9W {
            _P1_9W { w: self }
        }
        # [ doc = "Bits 20:21 - Pin function select P1.10." ]
        # [ inline ( always ) ]
        pub fn p1_10(&mut self) -> _P1_10W {
            _P1_10W { w: self }
        }
        # [ doc = "Bits 22:23 - Pin function select P1.14." ]
        # [ inline ( always ) ]
        pub fn p1_14(&mut self) -> _P1_14W {
            _P1_14W { w: self }
        }
        # [ doc = "Bits 30:31 - Pin function select P1.15." ]
        # [ inline ( always ) ]
        pub fn p1_15(&mut self) -> _P1_15W {
            _P1_15W { w: self }
        }
    }
}
# [ doc = "Pin function select register 3." ]
pub struct PINSEL3 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin function select register 3." ]
pub mod pinsel3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINSEL3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P1_16`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_16R {
        # [ doc = "GPIO P1.16" ]
        GPIO_P1,
        # [ doc = "ENET_MDC" ]
        ENET_MDC,
    }
    impl P1_16R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_16R::GPIO_P1 => 0,
                P1_16R::ENET_MDC => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_16R {
            match value {
                0 => P1_16R::GPIO_P1,
                1 => P1_16R::ENET_MDC,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_16R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `ENET_MDC`" ]
        # [ inline ( always ) ]
        pub fn is_enet_mdc(&self) -> bool {
            *self == P1_16R::ENET_MDC
        }
    }
    # [ doc = "Possible values of the field `P1_17`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_17R {
        # [ doc = "GPIO P1.17" ]
        GPIO_P1,
        # [ doc = "ENET_MDIO" ]
        ENET_MDIO,
    }
    impl P1_17R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_17R::GPIO_P1 => 0,
                P1_17R::ENET_MDIO => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_17R {
            match value {
                0 => P1_17R::GPIO_P1,
                1 => P1_17R::ENET_MDIO,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_17R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `ENET_MDIO`" ]
        # [ inline ( always ) ]
        pub fn is_enet_mdio(&self) -> bool {
            *self == P1_17R::ENET_MDIO
        }
    }
    # [ doc = "Possible values of the field `P1_18`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_18R {
        # [ doc = "GPIO P1.18" ]
        GPIO_P1,
        # [ doc = "USB_UP_LED" ]
        USB_UP_LED,
        # [ doc = "PWM1.1" ]
        PWM1,
        # [ doc = "CAP1.0" ]
        CAP1,
    }
    impl P1_18R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_18R::GPIO_P1 => 0,
                P1_18R::USB_UP_LED => 1,
                P1_18R::PWM1 => 2,
                P1_18R::CAP1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_18R {
            match value {
                0 => P1_18R::GPIO_P1,
                1 => P1_18R::USB_UP_LED,
                2 => P1_18R::PWM1,
                3 => P1_18R::CAP1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_18R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `USB_UP_LED`" ]
        # [ inline ( always ) ]
        pub fn is_usb_up_led(&self) -> bool {
            *self == P1_18R::USB_UP_LED
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P1_18R::PWM1
        }
        # [ doc = "Checks if the value of the field is `CAP1`" ]
        # [ inline ( always ) ]
        pub fn is_cap1(&self) -> bool {
            *self == P1_18R::CAP1
        }
    }
    # [ doc = "Possible values of the field `P1_19`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_19R {
        # [ doc = "GPIO P1.19." ]
        GPIO_P1,
        # [ doc = "MCOA0" ]
        MCOA0,
        # [ doc = "USB_PPWR" ]
        USB_PPWR,
        # [ doc = "CAP1.1" ]
        CAP1,
    }
    impl P1_19R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_19R::GPIO_P1 => 0,
                P1_19R::MCOA0 => 1,
                P1_19R::USB_PPWR => 2,
                P1_19R::CAP1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_19R {
            match value {
                0 => P1_19R::GPIO_P1,
                1 => P1_19R::MCOA0,
                2 => P1_19R::USB_PPWR,
                3 => P1_19R::CAP1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_19R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCOA0`" ]
        # [ inline ( always ) ]
        pub fn is_mcoa0(&self) -> bool {
            *self == P1_19R::MCOA0
        }
        # [ doc = "Checks if the value of the field is `USB_PPWR`" ]
        # [ inline ( always ) ]
        pub fn is_usb_ppwr(&self) -> bool {
            *self == P1_19R::USB_PPWR
        }
        # [ doc = "Checks if the value of the field is `CAP1`" ]
        # [ inline ( always ) ]
        pub fn is_cap1(&self) -> bool {
            *self == P1_19R::CAP1
        }
    }
    # [ doc = "Possible values of the field `P1_20`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_20R {
        # [ doc = "GPIO P1.20." ]
        GPIO_P1,
        # [ doc = "MCI0" ]
        MCI0,
        # [ doc = "PWM1.2" ]
        PWM1,
        # [ doc = "SCK0" ]
        SCK0,
    }
    impl P1_20R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_20R::GPIO_P1 => 0,
                P1_20R::MCI0 => 1,
                P1_20R::PWM1 => 2,
                P1_20R::SCK0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_20R {
            match value {
                0 => P1_20R::GPIO_P1,
                1 => P1_20R::MCI0,
                2 => P1_20R::PWM1,
                3 => P1_20R::SCK0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_20R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCI0`" ]
        # [ inline ( always ) ]
        pub fn is_mci0(&self) -> bool {
            *self == P1_20R::MCI0
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P1_20R::PWM1
        }
        # [ doc = "Checks if the value of the field is `SCK0`" ]
        # [ inline ( always ) ]
        pub fn is_sck0(&self) -> bool {
            *self == P1_20R::SCK0
        }
    }
    # [ doc = "Possible values of the field `P1_21`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_21R {
        # [ doc = "GPIO P1.21." ]
        GPIO_P1,
        # [ doc = "MCABORT" ]
        MCABORT,
        # [ doc = "PWM1.3" ]
        PWM1,
        # [ doc = "SSEL0" ]
        SSEL0,
    }
    impl P1_21R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_21R::GPIO_P1 => 0,
                P1_21R::MCABORT => 1,
                P1_21R::PWM1 => 2,
                P1_21R::SSEL0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_21R {
            match value {
                0 => P1_21R::GPIO_P1,
                1 => P1_21R::MCABORT,
                2 => P1_21R::PWM1,
                3 => P1_21R::SSEL0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_21R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCABORT`" ]
        # [ inline ( always ) ]
        pub fn is_mcabort(&self) -> bool {
            *self == P1_21R::MCABORT
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P1_21R::PWM1
        }
        # [ doc = "Checks if the value of the field is `SSEL0`" ]
        # [ inline ( always ) ]
        pub fn is_ssel0(&self) -> bool {
            *self == P1_21R::SSEL0
        }
    }
    # [ doc = "Possible values of the field `P1_22`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_22R {
        # [ doc = "GPIO P1.22." ]
        GPIO_P1,
        # [ doc = "MCOB0" ]
        MCOB0,
        # [ doc = "USB_PWRD" ]
        USB_PWRD,
        # [ doc = "MAT1.0" ]
        MAT1,
    }
    impl P1_22R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_22R::GPIO_P1 => 0,
                P1_22R::MCOB0 => 1,
                P1_22R::USB_PWRD => 2,
                P1_22R::MAT1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_22R {
            match value {
                0 => P1_22R::GPIO_P1,
                1 => P1_22R::MCOB0,
                2 => P1_22R::USB_PWRD,
                3 => P1_22R::MAT1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_22R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCOB0`" ]
        # [ inline ( always ) ]
        pub fn is_mcob0(&self) -> bool {
            *self == P1_22R::MCOB0
        }
        # [ doc = "Checks if the value of the field is `USB_PWRD`" ]
        # [ inline ( always ) ]
        pub fn is_usb_pwrd(&self) -> bool {
            *self == P1_22R::USB_PWRD
        }
        # [ doc = "Checks if the value of the field is `MAT1`" ]
        # [ inline ( always ) ]
        pub fn is_mat1(&self) -> bool {
            *self == P1_22R::MAT1
        }
    }
    # [ doc = "Possible values of the field `P1_23`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_23R {
        # [ doc = "GPIO P1.23." ]
        GPIO_P1,
        # [ doc = "MCI1" ]
        MCI1,
        # [ doc = "PWM1.4" ]
        PWM1,
        # [ doc = "MISO0" ]
        MISO0,
    }
    impl P1_23R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_23R::GPIO_P1 => 0,
                P1_23R::MCI1 => 1,
                P1_23R::PWM1 => 2,
                P1_23R::MISO0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_23R {
            match value {
                0 => P1_23R::GPIO_P1,
                1 => P1_23R::MCI1,
                2 => P1_23R::PWM1,
                3 => P1_23R::MISO0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_23R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCI1`" ]
        # [ inline ( always ) ]
        pub fn is_mci1(&self) -> bool {
            *self == P1_23R::MCI1
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P1_23R::PWM1
        }
        # [ doc = "Checks if the value of the field is `MISO0`" ]
        # [ inline ( always ) ]
        pub fn is_miso0(&self) -> bool {
            *self == P1_23R::MISO0
        }
    }
    # [ doc = "Possible values of the field `P1_24`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_24R {
        # [ doc = "GPIO P1.24." ]
        GPIO_P1,
        # [ doc = "MCI2" ]
        MCI2,
        # [ doc = "PWM1.5" ]
        PWM1,
        # [ doc = "MOSI0" ]
        MOSI0,
    }
    impl P1_24R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_24R::GPIO_P1 => 0,
                P1_24R::MCI2 => 1,
                P1_24R::PWM1 => 2,
                P1_24R::MOSI0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_24R {
            match value {
                0 => P1_24R::GPIO_P1,
                1 => P1_24R::MCI2,
                2 => P1_24R::PWM1,
                3 => P1_24R::MOSI0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_24R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCI2`" ]
        # [ inline ( always ) ]
        pub fn is_mci2(&self) -> bool {
            *self == P1_24R::MCI2
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P1_24R::PWM1
        }
        # [ doc = "Checks if the value of the field is `MOSI0`" ]
        # [ inline ( always ) ]
        pub fn is_mosi0(&self) -> bool {
            *self == P1_24R::MOSI0
        }
    }
    # [ doc = "Possible values of the field `P1_25`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_25R {
        # [ doc = "GPIO P1.25" ]
        GPIO_P1,
        # [ doc = "MCOA1" ]
        MCOA1,
        # [ doc = "MAT1.1" ]
        MAT1,
    }
    impl P1_25R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_25R::GPIO_P1 => 0,
                P1_25R::MCOA1 => 1,
                P1_25R::MAT1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_25R {
            match value {
                0 => P1_25R::GPIO_P1,
                1 => P1_25R::MCOA1,
                3 => P1_25R::MAT1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_25R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCOA1`" ]
        # [ inline ( always ) ]
        pub fn is_mcoa1(&self) -> bool {
            *self == P1_25R::MCOA1
        }
        # [ doc = "Checks if the value of the field is `MAT1`" ]
        # [ inline ( always ) ]
        pub fn is_mat1(&self) -> bool {
            *self == P1_25R::MAT1
        }
    }
    # [ doc = "Possible values of the field `P1_26`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_26R {
        # [ doc = "GPIO P1.26" ]
        GPIO_P1,
        # [ doc = "MCOB1" ]
        MCOB1,
        # [ doc = "PWM1.6" ]
        PWM1,
        # [ doc = "CAP0.0" ]
        CAP0,
    }
    impl P1_26R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_26R::GPIO_P1 => 0,
                P1_26R::MCOB1 => 1,
                P1_26R::PWM1 => 2,
                P1_26R::CAP0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_26R {
            match value {
                0 => P1_26R::GPIO_P1,
                1 => P1_26R::MCOB1,
                2 => P1_26R::PWM1,
                3 => P1_26R::CAP0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_26R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCOB1`" ]
        # [ inline ( always ) ]
        pub fn is_mcob1(&self) -> bool {
            *self == P1_26R::MCOB1
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P1_26R::PWM1
        }
        # [ doc = "Checks if the value of the field is `CAP0`" ]
        # [ inline ( always ) ]
        pub fn is_cap0(&self) -> bool {
            *self == P1_26R::CAP0
        }
    }
    # [ doc = "Possible values of the field `P1_27`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_27R {
        # [ doc = "GPIO P1.27" ]
        GPIO_P1,
        # [ doc = "CLKOUT" ]
        CLKOUT,
        # [ doc = "USB_OVRCR" ]
        USB_OVRCR,
        # [ doc = "CAP0.1" ]
        CAP0,
    }
    impl P1_27R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_27R::GPIO_P1 => 0,
                P1_27R::CLKOUT => 1,
                P1_27R::USB_OVRCR => 2,
                P1_27R::CAP0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_27R {
            match value {
                0 => P1_27R::GPIO_P1,
                1 => P1_27R::CLKOUT,
                2 => P1_27R::USB_OVRCR,
                3 => P1_27R::CAP0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_27R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `CLKOUT`" ]
        # [ inline ( always ) ]
        pub fn is_clkout(&self) -> bool {
            *self == P1_27R::CLKOUT
        }
        # [ doc = "Checks if the value of the field is `USB_OVRCR`" ]
        # [ inline ( always ) ]
        pub fn is_usb_ovrcr(&self) -> bool {
            *self == P1_27R::USB_OVRCR
        }
        # [ doc = "Checks if the value of the field is `CAP0`" ]
        # [ inline ( always ) ]
        pub fn is_cap0(&self) -> bool {
            *self == P1_27R::CAP0
        }
    }
    # [ doc = "Possible values of the field `P1_28`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_28R {
        # [ doc = "GPIO P1.28" ]
        GPIO_P1,
        # [ doc = "MCOA2" ]
        MCOA2,
        # [ doc = "PCAP1.0" ]
        PCAP1,
        # [ doc = "MAT0.0" ]
        MAT0,
    }
    impl P1_28R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_28R::GPIO_P1 => 0,
                P1_28R::MCOA2 => 1,
                P1_28R::PCAP1 => 2,
                P1_28R::MAT0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_28R {
            match value {
                0 => P1_28R::GPIO_P1,
                1 => P1_28R::MCOA2,
                2 => P1_28R::PCAP1,
                3 => P1_28R::MAT0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_28R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCOA2`" ]
        # [ inline ( always ) ]
        pub fn is_mcoa2(&self) -> bool {
            *self == P1_28R::MCOA2
        }
        # [ doc = "Checks if the value of the field is `PCAP1`" ]
        # [ inline ( always ) ]
        pub fn is_pcap1(&self) -> bool {
            *self == P1_28R::PCAP1
        }
        # [ doc = "Checks if the value of the field is `MAT0`" ]
        # [ inline ( always ) ]
        pub fn is_mat0(&self) -> bool {
            *self == P1_28R::MAT0
        }
    }
    # [ doc = "Possible values of the field `P1_29`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_29R {
        # [ doc = "GPIO P1.29" ]
        GPIO_P1,
        # [ doc = "MCOB2" ]
        MCOB2,
        # [ doc = "PCAP1.1" ]
        PCAP1,
        # [ doc = "MAT0.1" ]
        MAT0,
    }
    impl P1_29R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_29R::GPIO_P1 => 0,
                P1_29R::MCOB2 => 1,
                P1_29R::PCAP1 => 2,
                P1_29R::MAT0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_29R {
            match value {
                0 => P1_29R::GPIO_P1,
                1 => P1_29R::MCOB2,
                2 => P1_29R::PCAP1,
                3 => P1_29R::MAT0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_29R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `MCOB2`" ]
        # [ inline ( always ) ]
        pub fn is_mcob2(&self) -> bool {
            *self == P1_29R::MCOB2
        }
        # [ doc = "Checks if the value of the field is `PCAP1`" ]
        # [ inline ( always ) ]
        pub fn is_pcap1(&self) -> bool {
            *self == P1_29R::PCAP1
        }
        # [ doc = "Checks if the value of the field is `MAT0`" ]
        # [ inline ( always ) ]
        pub fn is_mat0(&self) -> bool {
            *self == P1_29R::MAT0
        }
    }
    # [ doc = "Possible values of the field `P1_30`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_30R {
        # [ doc = "GPIO P1.30" ]
        GPIO_P1,
        # [ doc = "VBUS" ]
        VBUS,
        # [ doc = "AD0.4" ]
        AD0,
    }
    impl P1_30R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_30R::GPIO_P1 => 0,
                P1_30R::VBUS => 2,
                P1_30R::AD0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_30R {
            match value {
                0 => P1_30R::GPIO_P1,
                2 => P1_30R::VBUS,
                3 => P1_30R::AD0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p1(&self) -> bool {
            *self == P1_30R::GPIO_P1
        }
        # [ doc = "Checks if the value of the field is `VBUS`" ]
        # [ inline ( always ) ]
        pub fn is_vbus(&self) -> bool {
            *self == P1_30R::VBUS
        }
        # [ doc = "Checks if the value of the field is `AD0`" ]
        # [ inline ( always ) ]
        pub fn is_ad0(&self) -> bool {
            *self == P1_30R::AD0
        }
    }
    # [ doc = "Possible values of the field `P1_31`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_31R {
        # [ doc = "GPIO Port 1.31" ]
        GPIO_PORT_1,
        # [ doc = "SCK1" ]
        SCK1,
        # [ doc = "AD0.5" ]
        AD0,
    }
    impl P1_31R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_31R::GPIO_PORT_1 => 0,
                P1_31R::SCK1 => 2,
                P1_31R::AD0 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_31R {
            match value {
                0 => P1_31R::GPIO_PORT_1,
                2 => P1_31R::SCK1,
                3 => P1_31R::AD0,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_PORT_1`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_port_1(&self) -> bool {
            *self == P1_31R::GPIO_PORT_1
        }
        # [ doc = "Checks if the value of the field is `SCK1`" ]
        # [ inline ( always ) ]
        pub fn is_sck1(&self) -> bool {
            *self == P1_31R::SCK1
        }
        # [ doc = "Checks if the value of the field is `AD0`" ]
        # [ inline ( always ) ]
        pub fn is_ad0(&self) -> bool {
            *self == P1_31R::AD0
        }
    }
    # [ doc = "Values that can be written to the field `P1_16`" ]
    pub enum P1_16W {
        # [ doc = "GPIO P1.16" ]
        GPIO_P1,
        # [ doc = "ENET_MDC" ]
        ENET_MDC,
    }
    impl P1_16W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_16W::GPIO_P1 => 0,
                P1_16W::ENET_MDC => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_16W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_16W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.16" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_16W::GPIO_P1)
        }
        # [ doc = "ENET_MDC" ]
        # [ inline ( always ) ]
        pub fn enet_mdc(self) -> &'a mut W {
            self.variant(P1_16W::ENET_MDC)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_17`" ]
    pub enum P1_17W {
        # [ doc = "GPIO P1.17" ]
        GPIO_P1,
        # [ doc = "ENET_MDIO" ]
        ENET_MDIO,
    }
    impl P1_17W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_17W::GPIO_P1 => 0,
                P1_17W::ENET_MDIO => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_17W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_17W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_17W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.17" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_17W::GPIO_P1)
        }
        # [ doc = "ENET_MDIO" ]
        # [ inline ( always ) ]
        pub fn enet_mdio(self) -> &'a mut W {
            self.variant(P1_17W::ENET_MDIO)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_18`" ]
    pub enum P1_18W {
        # [ doc = "GPIO P1.18" ]
        GPIO_P1,
        # [ doc = "USB_UP_LED" ]
        USB_UP_LED,
        # [ doc = "PWM1.1" ]
        PWM1,
        # [ doc = "CAP1.0" ]
        CAP1,
    }
    impl P1_18W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_18W::GPIO_P1 => 0,
                P1_18W::USB_UP_LED => 1,
                P1_18W::PWM1 => 2,
                P1_18W::CAP1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_18W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_18W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_18W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.18" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_18W::GPIO_P1)
        }
        # [ doc = "USB_UP_LED" ]
        # [ inline ( always ) ]
        pub fn usb_up_led(self) -> &'a mut W {
            self.variant(P1_18W::USB_UP_LED)
        }
        # [ doc = "PWM1.1" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P1_18W::PWM1)
        }
        # [ doc = "CAP1.0" ]
        # [ inline ( always ) ]
        pub fn cap1(self) -> &'a mut W {
            self.variant(P1_18W::CAP1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_19`" ]
    pub enum P1_19W {
        # [ doc = "GPIO P1.19." ]
        GPIO_P1,
        # [ doc = "MCOA0" ]
        MCOA0,
        # [ doc = "USB_PPWR" ]
        USB_PPWR,
        # [ doc = "CAP1.1" ]
        CAP1,
    }
    impl P1_19W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_19W::GPIO_P1 => 0,
                P1_19W::MCOA0 => 1,
                P1_19W::USB_PPWR => 2,
                P1_19W::CAP1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_19W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_19W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_19W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.19." ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_19W::GPIO_P1)
        }
        # [ doc = "MCOA0" ]
        # [ inline ( always ) ]
        pub fn mcoa0(self) -> &'a mut W {
            self.variant(P1_19W::MCOA0)
        }
        # [ doc = "USB_PPWR" ]
        # [ inline ( always ) ]
        pub fn usb_ppwr(self) -> &'a mut W {
            self.variant(P1_19W::USB_PPWR)
        }
        # [ doc = "CAP1.1" ]
        # [ inline ( always ) ]
        pub fn cap1(self) -> &'a mut W {
            self.variant(P1_19W::CAP1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_20`" ]
    pub enum P1_20W {
        # [ doc = "GPIO P1.20." ]
        GPIO_P1,
        # [ doc = "MCI0" ]
        MCI0,
        # [ doc = "PWM1.2" ]
        PWM1,
        # [ doc = "SCK0" ]
        SCK0,
    }
    impl P1_20W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_20W::GPIO_P1 => 0,
                P1_20W::MCI0 => 1,
                P1_20W::PWM1 => 2,
                P1_20W::SCK0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_20W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_20W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_20W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.20." ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_20W::GPIO_P1)
        }
        # [ doc = "MCI0" ]
        # [ inline ( always ) ]
        pub fn mci0(self) -> &'a mut W {
            self.variant(P1_20W::MCI0)
        }
        # [ doc = "PWM1.2" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P1_20W::PWM1)
        }
        # [ doc = "SCK0" ]
        # [ inline ( always ) ]
        pub fn sck0(self) -> &'a mut W {
            self.variant(P1_20W::SCK0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_21`" ]
    pub enum P1_21W {
        # [ doc = "GPIO P1.21." ]
        GPIO_P1,
        # [ doc = "MCABORT" ]
        MCABORT,
        # [ doc = "PWM1.3" ]
        PWM1,
        # [ doc = "SSEL0" ]
        SSEL0,
    }
    impl P1_21W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_21W::GPIO_P1 => 0,
                P1_21W::MCABORT => 1,
                P1_21W::PWM1 => 2,
                P1_21W::SSEL0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_21W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_21W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_21W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.21." ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_21W::GPIO_P1)
        }
        # [ doc = "MCABORT" ]
        # [ inline ( always ) ]
        pub fn mcabort(self) -> &'a mut W {
            self.variant(P1_21W::MCABORT)
        }
        # [ doc = "PWM1.3" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P1_21W::PWM1)
        }
        # [ doc = "SSEL0" ]
        # [ inline ( always ) ]
        pub fn ssel0(self) -> &'a mut W {
            self.variant(P1_21W::SSEL0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_22`" ]
    pub enum P1_22W {
        # [ doc = "GPIO P1.22." ]
        GPIO_P1,
        # [ doc = "MCOB0" ]
        MCOB0,
        # [ doc = "USB_PWRD" ]
        USB_PWRD,
        # [ doc = "MAT1.0" ]
        MAT1,
    }
    impl P1_22W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_22W::GPIO_P1 => 0,
                P1_22W::MCOB0 => 1,
                P1_22W::USB_PWRD => 2,
                P1_22W::MAT1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_22W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_22W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_22W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.22." ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_22W::GPIO_P1)
        }
        # [ doc = "MCOB0" ]
        # [ inline ( always ) ]
        pub fn mcob0(self) -> &'a mut W {
            self.variant(P1_22W::MCOB0)
        }
        # [ doc = "USB_PWRD" ]
        # [ inline ( always ) ]
        pub fn usb_pwrd(self) -> &'a mut W {
            self.variant(P1_22W::USB_PWRD)
        }
        # [ doc = "MAT1.0" ]
        # [ inline ( always ) ]
        pub fn mat1(self) -> &'a mut W {
            self.variant(P1_22W::MAT1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_23`" ]
    pub enum P1_23W {
        # [ doc = "GPIO P1.23." ]
        GPIO_P1,
        # [ doc = "MCI1" ]
        MCI1,
        # [ doc = "PWM1.4" ]
        PWM1,
        # [ doc = "MISO0" ]
        MISO0,
    }
    impl P1_23W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_23W::GPIO_P1 => 0,
                P1_23W::MCI1 => 1,
                P1_23W::PWM1 => 2,
                P1_23W::MISO0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_23W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_23W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_23W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.23." ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_23W::GPIO_P1)
        }
        # [ doc = "MCI1" ]
        # [ inline ( always ) ]
        pub fn mci1(self) -> &'a mut W {
            self.variant(P1_23W::MCI1)
        }
        # [ doc = "PWM1.4" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P1_23W::PWM1)
        }
        # [ doc = "MISO0" ]
        # [ inline ( always ) ]
        pub fn miso0(self) -> &'a mut W {
            self.variant(P1_23W::MISO0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_24`" ]
    pub enum P1_24W {
        # [ doc = "GPIO P1.24." ]
        GPIO_P1,
        # [ doc = "MCI2" ]
        MCI2,
        # [ doc = "PWM1.5" ]
        PWM1,
        # [ doc = "MOSI0" ]
        MOSI0,
    }
    impl P1_24W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_24W::GPIO_P1 => 0,
                P1_24W::MCI2 => 1,
                P1_24W::PWM1 => 2,
                P1_24W::MOSI0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_24W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_24W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_24W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.24." ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_24W::GPIO_P1)
        }
        # [ doc = "MCI2" ]
        # [ inline ( always ) ]
        pub fn mci2(self) -> &'a mut W {
            self.variant(P1_24W::MCI2)
        }
        # [ doc = "PWM1.5" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P1_24W::PWM1)
        }
        # [ doc = "MOSI0" ]
        # [ inline ( always ) ]
        pub fn mosi0(self) -> &'a mut W {
            self.variant(P1_24W::MOSI0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_25`" ]
    pub enum P1_25W {
        # [ doc = "GPIO P1.25" ]
        GPIO_P1,
        # [ doc = "MCOA1" ]
        MCOA1,
        # [ doc = "MAT1.1" ]
        MAT1,
    }
    impl P1_25W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_25W::GPIO_P1 => 0,
                P1_25W::MCOA1 => 1,
                P1_25W::MAT1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_25W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_25W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_25W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.25" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_25W::GPIO_P1)
        }
        # [ doc = "MCOA1" ]
        # [ inline ( always ) ]
        pub fn mcoa1(self) -> &'a mut W {
            self.variant(P1_25W::MCOA1)
        }
        # [ doc = "MAT1.1" ]
        # [ inline ( always ) ]
        pub fn mat1(self) -> &'a mut W {
            self.variant(P1_25W::MAT1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_26`" ]
    pub enum P1_26W {
        # [ doc = "GPIO P1.26" ]
        GPIO_P1,
        # [ doc = "MCOB1" ]
        MCOB1,
        # [ doc = "PWM1.6" ]
        PWM1,
        # [ doc = "CAP0.0" ]
        CAP0,
    }
    impl P1_26W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_26W::GPIO_P1 => 0,
                P1_26W::MCOB1 => 1,
                P1_26W::PWM1 => 2,
                P1_26W::CAP0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_26W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_26W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_26W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.26" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_26W::GPIO_P1)
        }
        # [ doc = "MCOB1" ]
        # [ inline ( always ) ]
        pub fn mcob1(self) -> &'a mut W {
            self.variant(P1_26W::MCOB1)
        }
        # [ doc = "PWM1.6" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P1_26W::PWM1)
        }
        # [ doc = "CAP0.0" ]
        # [ inline ( always ) ]
        pub fn cap0(self) -> &'a mut W {
            self.variant(P1_26W::CAP0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_27`" ]
    pub enum P1_27W {
        # [ doc = "GPIO P1.27" ]
        GPIO_P1,
        # [ doc = "CLKOUT" ]
        CLKOUT,
        # [ doc = "USB_OVRCR" ]
        USB_OVRCR,
        # [ doc = "CAP0.1" ]
        CAP0,
    }
    impl P1_27W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_27W::GPIO_P1 => 0,
                P1_27W::CLKOUT => 1,
                P1_27W::USB_OVRCR => 2,
                P1_27W::CAP0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_27W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_27W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_27W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.27" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_27W::GPIO_P1)
        }
        # [ doc = "CLKOUT" ]
        # [ inline ( always ) ]
        pub fn clkout(self) -> &'a mut W {
            self.variant(P1_27W::CLKOUT)
        }
        # [ doc = "USB_OVRCR" ]
        # [ inline ( always ) ]
        pub fn usb_ovrcr(self) -> &'a mut W {
            self.variant(P1_27W::USB_OVRCR)
        }
        # [ doc = "CAP0.1" ]
        # [ inline ( always ) ]
        pub fn cap0(self) -> &'a mut W {
            self.variant(P1_27W::CAP0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_28`" ]
    pub enum P1_28W {
        # [ doc = "GPIO P1.28" ]
        GPIO_P1,
        # [ doc = "MCOA2" ]
        MCOA2,
        # [ doc = "PCAP1.0" ]
        PCAP1,
        # [ doc = "MAT0.0" ]
        MAT0,
    }
    impl P1_28W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_28W::GPIO_P1 => 0,
                P1_28W::MCOA2 => 1,
                P1_28W::PCAP1 => 2,
                P1_28W::MAT0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_28W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_28W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_28W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.28" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_28W::GPIO_P1)
        }
        # [ doc = "MCOA2" ]
        # [ inline ( always ) ]
        pub fn mcoa2(self) -> &'a mut W {
            self.variant(P1_28W::MCOA2)
        }
        # [ doc = "PCAP1.0" ]
        # [ inline ( always ) ]
        pub fn pcap1(self) -> &'a mut W {
            self.variant(P1_28W::PCAP1)
        }
        # [ doc = "MAT0.0" ]
        # [ inline ( always ) ]
        pub fn mat0(self) -> &'a mut W {
            self.variant(P1_28W::MAT0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_29`" ]
    pub enum P1_29W {
        # [ doc = "GPIO P1.29" ]
        GPIO_P1,
        # [ doc = "MCOB2" ]
        MCOB2,
        # [ doc = "PCAP1.1" ]
        PCAP1,
        # [ doc = "MAT0.1" ]
        MAT0,
    }
    impl P1_29W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_29W::GPIO_P1 => 0,
                P1_29W::MCOB2 => 1,
                P1_29W::PCAP1 => 2,
                P1_29W::MAT0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_29W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_29W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_29W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P1.29" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_29W::GPIO_P1)
        }
        # [ doc = "MCOB2" ]
        # [ inline ( always ) ]
        pub fn mcob2(self) -> &'a mut W {
            self.variant(P1_29W::MCOB2)
        }
        # [ doc = "PCAP1.1" ]
        # [ inline ( always ) ]
        pub fn pcap1(self) -> &'a mut W {
            self.variant(P1_29W::PCAP1)
        }
        # [ doc = "MAT0.1" ]
        # [ inline ( always ) ]
        pub fn mat0(self) -> &'a mut W {
            self.variant(P1_29W::MAT0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_30`" ]
    pub enum P1_30W {
        # [ doc = "GPIO P1.30" ]
        GPIO_P1,
        # [ doc = "VBUS" ]
        VBUS,
        # [ doc = "AD0.4" ]
        AD0,
    }
    impl P1_30W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_30W::GPIO_P1 => 0,
                P1_30W::VBUS => 2,
                P1_30W::AD0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_30W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_30W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_30W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P1.30" ]
        # [ inline ( always ) ]
        pub fn gpio_p1(self) -> &'a mut W {
            self.variant(P1_30W::GPIO_P1)
        }
        # [ doc = "VBUS" ]
        # [ inline ( always ) ]
        pub fn vbus(self) -> &'a mut W {
            self.variant(P1_30W::VBUS)
        }
        # [ doc = "AD0.4" ]
        # [ inline ( always ) ]
        pub fn ad0(self) -> &'a mut W {
            self.variant(P1_30W::AD0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_31`" ]
    pub enum P1_31W {
        # [ doc = "GPIO Port 1.31" ]
        GPIO_PORT_1,
        # [ doc = "SCK1" ]
        SCK1,
        # [ doc = "AD0.5" ]
        AD0,
    }
    impl P1_31W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_31W::GPIO_PORT_1 => 0,
                P1_31W::SCK1 => 2,
                P1_31W::AD0 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_31W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_31W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_31W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO Port 1.31" ]
        # [ inline ( always ) ]
        pub fn gpio_port_1(self) -> &'a mut W {
            self.variant(P1_31W::GPIO_PORT_1)
        }
        # [ doc = "SCK1" ]
        # [ inline ( always ) ]
        pub fn sck1(self) -> &'a mut W {
            self.variant(P1_31W::SCK1)
        }
        # [ doc = "AD0.5" ]
        # [ inline ( always ) ]
        pub fn ad0(self) -> &'a mut W {
            self.variant(P1_31W::AD0)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Pin function select P1.16." ]
        # [ inline ( always ) ]
        pub fn p1_16(&self) -> P1_16R {
            P1_16R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 2:3 - Pin function select P1.17." ]
        # [ inline ( always ) ]
        pub fn p1_17(&self) -> P1_17R {
            P1_17R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 2;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 4:5 - Pin function select P1.18." ]
        # [ inline ( always ) ]
        pub fn p1_18(&self) -> P1_18R {
            P1_18R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 4;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 6:7 - Pin function select P1.19." ]
        # [ inline ( always ) ]
        pub fn p1_19(&self) -> P1_19R {
            P1_19R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 6;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 8:9 - Pin function select P1.20." ]
        # [ inline ( always ) ]
        pub fn p1_20(&self) -> P1_20R {
            P1_20R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 8;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 10:11 - Pin function select P1.21." ]
        # [ inline ( always ) ]
        pub fn p1_21(&self) -> P1_21R {
            P1_21R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 10;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 12:13 - Pin function select P1.22" ]
        # [ inline ( always ) ]
        pub fn p1_22(&self) -> P1_22R {
            P1_22R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 12;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 14:15 - Pin function select P1.23." ]
        # [ inline ( always ) ]
        pub fn p1_23(&self) -> P1_23R {
            P1_23R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 14;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 16:17 - Pin function select P1.24." ]
        # [ inline ( always ) ]
        pub fn p1_24(&self) -> P1_24R {
            P1_24R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 16;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 18:19 - Pin function select P1.25." ]
        # [ inline ( always ) ]
        pub fn p1_25(&self) -> P1_25R {
            P1_25R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 18;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 20:21 - Pin function select P1.26." ]
        # [ inline ( always ) ]
        pub fn p1_26(&self) -> P1_26R {
            P1_26R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 20;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 22:23 - Pin function select P1.27." ]
        # [ inline ( always ) ]
        pub fn p1_27(&self) -> P1_27R {
            P1_27R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 22;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 24:25 - Pin function select P1.28." ]
        # [ inline ( always ) ]
        pub fn p1_28(&self) -> P1_28R {
            P1_28R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 24;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 26:27 - Pin function select P1.29" ]
        # [ inline ( always ) ]
        pub fn p1_29(&self) -> P1_29R {
            P1_29R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 26;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 28:29 - Pin function select P1.30." ]
        # [ inline ( always ) ]
        pub fn p1_30(&self) -> P1_30R {
            P1_30R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 28;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 30:31 - Pin function select P1.31." ]
        # [ inline ( always ) ]
        pub fn p1_31(&self) -> P1_31R {
            P1_31R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 30;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Pin function select P1.16." ]
        # [ inline ( always ) ]
        pub fn p1_16(&mut self) -> _P1_16W {
            _P1_16W { w: self }
        }
        # [ doc = "Bits 2:3 - Pin function select P1.17." ]
        # [ inline ( always ) ]
        pub fn p1_17(&mut self) -> _P1_17W {
            _P1_17W { w: self }
        }
        # [ doc = "Bits 4:5 - Pin function select P1.18." ]
        # [ inline ( always ) ]
        pub fn p1_18(&mut self) -> _P1_18W {
            _P1_18W { w: self }
        }
        # [ doc = "Bits 6:7 - Pin function select P1.19." ]
        # [ inline ( always ) ]
        pub fn p1_19(&mut self) -> _P1_19W {
            _P1_19W { w: self }
        }
        # [ doc = "Bits 8:9 - Pin function select P1.20." ]
        # [ inline ( always ) ]
        pub fn p1_20(&mut self) -> _P1_20W {
            _P1_20W { w: self }
        }
        # [ doc = "Bits 10:11 - Pin function select P1.21." ]
        # [ inline ( always ) ]
        pub fn p1_21(&mut self) -> _P1_21W {
            _P1_21W { w: self }
        }
        # [ doc = "Bits 12:13 - Pin function select P1.22" ]
        # [ inline ( always ) ]
        pub fn p1_22(&mut self) -> _P1_22W {
            _P1_22W { w: self }
        }
        # [ doc = "Bits 14:15 - Pin function select P1.23." ]
        # [ inline ( always ) ]
        pub fn p1_23(&mut self) -> _P1_23W {
            _P1_23W { w: self }
        }
        # [ doc = "Bits 16:17 - Pin function select P1.24." ]
        # [ inline ( always ) ]
        pub fn p1_24(&mut self) -> _P1_24W {
            _P1_24W { w: self }
        }
        # [ doc = "Bits 18:19 - Pin function select P1.25." ]
        # [ inline ( always ) ]
        pub fn p1_25(&mut self) -> _P1_25W {
            _P1_25W { w: self }
        }
        # [ doc = "Bits 20:21 - Pin function select P1.26." ]
        # [ inline ( always ) ]
        pub fn p1_26(&mut self) -> _P1_26W {
            _P1_26W { w: self }
        }
        # [ doc = "Bits 22:23 - Pin function select P1.27." ]
        # [ inline ( always ) ]
        pub fn p1_27(&mut self) -> _P1_27W {
            _P1_27W { w: self }
        }
        # [ doc = "Bits 24:25 - Pin function select P1.28." ]
        # [ inline ( always ) ]
        pub fn p1_28(&mut self) -> _P1_28W {
            _P1_28W { w: self }
        }
        # [ doc = "Bits 26:27 - Pin function select P1.29" ]
        # [ inline ( always ) ]
        pub fn p1_29(&mut self) -> _P1_29W {
            _P1_29W { w: self }
        }
        # [ doc = "Bits 28:29 - Pin function select P1.30." ]
        # [ inline ( always ) ]
        pub fn p1_30(&mut self) -> _P1_30W {
            _P1_30W { w: self }
        }
        # [ doc = "Bits 30:31 - Pin function select P1.31." ]
        # [ inline ( always ) ]
        pub fn p1_31(&mut self) -> _P1_31W {
            _P1_31W { w: self }
        }
    }
}
# [ doc = "Pin function select register 4" ]
pub struct PINSEL4 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin function select register 4" ]
pub mod pinsel4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINSEL4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P2_0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_0R {
        # [ doc = "GPIO P2.0" ]
        GPIO_P2,
        # [ doc = "PWM1.1" ]
        PWM1,
        # [ doc = "TXD1" ]
        TXD1,
    }
    impl P2_0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_0R::GPIO_P2 => 0,
                P2_0R::PWM1 => 1,
                P2_0R::TXD1 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_0R {
            match value {
                0 => P2_0R::GPIO_P2,
                1 => P2_0R::PWM1,
                2 => P2_0R::TXD1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_0R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P2_0R::PWM1
        }
        # [ doc = "Checks if the value of the field is `TXD1`" ]
        # [ inline ( always ) ]
        pub fn is_txd1(&self) -> bool {
            *self == P2_0R::TXD1
        }
    }
    # [ doc = "Possible values of the field `P2_1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_1R {
        # [ doc = "GPIO P2.1" ]
        GPIO_P2,
        # [ doc = "PWM1.2" ]
        PWM1,
        # [ doc = "RXD1" ]
        RXD1,
    }
    impl P2_1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_1R::GPIO_P2 => 0,
                P2_1R::PWM1 => 1,
                P2_1R::RXD1 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_1R {
            match value {
                0 => P2_1R::GPIO_P2,
                1 => P2_1R::PWM1,
                2 => P2_1R::RXD1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_1R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P2_1R::PWM1
        }
        # [ doc = "Checks if the value of the field is `RXD1`" ]
        # [ inline ( always ) ]
        pub fn is_rxd1(&self) -> bool {
            *self == P2_1R::RXD1
        }
    }
    # [ doc = "Possible values of the field `P2_2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_2R {
        # [ doc = "GPIO P2.2" ]
        GPIO_P2,
        # [ doc = "PWM1.3" ]
        PWM1,
        # [ doc = "CTS1" ]
        CTS1,
    }
    impl P2_2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_2R::GPIO_P2 => 0,
                P2_2R::PWM1 => 1,
                P2_2R::CTS1 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_2R {
            match value {
                0 => P2_2R::GPIO_P2,
                1 => P2_2R::PWM1,
                2 => P2_2R::CTS1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_2R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P2_2R::PWM1
        }
        # [ doc = "Checks if the value of the field is `CTS1`" ]
        # [ inline ( always ) ]
        pub fn is_cts1(&self) -> bool {
            *self == P2_2R::CTS1
        }
    }
    # [ doc = "Possible values of the field `P2_3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_3R {
        # [ doc = "GPIO P2.3." ]
        GPIO_P2,
        # [ doc = "PWM1.4" ]
        PWM1,
        # [ doc = "DCD1" ]
        DCD1,
    }
    impl P2_3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_3R::GPIO_P2 => 0,
                P2_3R::PWM1 => 1,
                P2_3R::DCD1 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_3R {
            match value {
                0 => P2_3R::GPIO_P2,
                1 => P2_3R::PWM1,
                2 => P2_3R::DCD1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_3R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P2_3R::PWM1
        }
        # [ doc = "Checks if the value of the field is `DCD1`" ]
        # [ inline ( always ) ]
        pub fn is_dcd1(&self) -> bool {
            *self == P2_3R::DCD1
        }
    }
    # [ doc = "Possible values of the field `P2_4`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_4R {
        # [ doc = "GPIO P2.4." ]
        GPIO_P2,
        # [ doc = "PWM1.5" ]
        PWM1,
        # [ doc = "DSR1" ]
        DSR1,
    }
    impl P2_4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_4R::GPIO_P2 => 0,
                P2_4R::PWM1 => 1,
                P2_4R::DSR1 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_4R {
            match value {
                0 => P2_4R::GPIO_P2,
                1 => P2_4R::PWM1,
                2 => P2_4R::DSR1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_4R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P2_4R::PWM1
        }
        # [ doc = "Checks if the value of the field is `DSR1`" ]
        # [ inline ( always ) ]
        pub fn is_dsr1(&self) -> bool {
            *self == P2_4R::DSR1
        }
    }
    # [ doc = "Possible values of the field `P2_5`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_5R {
        # [ doc = "GPIO P2.5." ]
        GPIO_P2,
        # [ doc = "PWM1.6" ]
        PWM1,
        # [ doc = "DTR1" ]
        DTR1,
    }
    impl P2_5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_5R::GPIO_P2 => 0,
                P2_5R::PWM1 => 1,
                P2_5R::DTR1 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_5R {
            match value {
                0 => P2_5R::GPIO_P2,
                1 => P2_5R::PWM1,
                2 => P2_5R::DTR1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_5R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P2_5R::PWM1
        }
        # [ doc = "Checks if the value of the field is `DTR1`" ]
        # [ inline ( always ) ]
        pub fn is_dtr1(&self) -> bool {
            *self == P2_5R::DTR1
        }
    }
    # [ doc = "Possible values of the field `P2_6`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_6R {
        # [ doc = "GPIO P2.6." ]
        GPIO_P2,
        # [ doc = "PCAP1.0" ]
        PCAP1,
        # [ doc = "RI1" ]
        RI1,
    }
    impl P2_6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_6R::GPIO_P2 => 0,
                P2_6R::PCAP1 => 1,
                P2_6R::RI1 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_6R {
            match value {
                0 => P2_6R::GPIO_P2,
                1 => P2_6R::PCAP1,
                2 => P2_6R::RI1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_6R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `PCAP1`" ]
        # [ inline ( always ) ]
        pub fn is_pcap1(&self) -> bool {
            *self == P2_6R::PCAP1
        }
        # [ doc = "Checks if the value of the field is `RI1`" ]
        # [ inline ( always ) ]
        pub fn is_ri1(&self) -> bool {
            *self == P2_6R::RI1
        }
    }
    # [ doc = "Possible values of the field `P2_7`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_7R {
        # [ doc = "GPIO P2.7." ]
        GPIO_P2,
        # [ doc = "RD2" ]
        RD2,
        # [ doc = "RTS1" ]
        RTS1,
    }
    impl P2_7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_7R::GPIO_P2 => 0,
                P2_7R::RD2 => 1,
                P2_7R::RTS1 => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_7R {
            match value {
                0 => P2_7R::GPIO_P2,
                1 => P2_7R::RD2,
                2 => P2_7R::RTS1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_7R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `RD2`" ]
        # [ inline ( always ) ]
        pub fn is_rd2(&self) -> bool {
            *self == P2_7R::RD2
        }
        # [ doc = "Checks if the value of the field is `RTS1`" ]
        # [ inline ( always ) ]
        pub fn is_rts1(&self) -> bool {
            *self == P2_7R::RTS1
        }
    }
    # [ doc = "Possible values of the field `P2_8`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_8R {
        # [ doc = "GPIO P2.8." ]
        GPIO_P2,
        # [ doc = "TD2" ]
        TD2,
        # [ doc = "TXD2" ]
        TXD2,
        # [ doc = "ENET_MDC" ]
        ENET_MDC,
    }
    impl P2_8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_8R::GPIO_P2 => 0,
                P2_8R::TD2 => 1,
                P2_8R::TXD2 => 2,
                P2_8R::ENET_MDC => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_8R {
            match value {
                0 => P2_8R::GPIO_P2,
                1 => P2_8R::TD2,
                2 => P2_8R::TXD2,
                3 => P2_8R::ENET_MDC,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_8R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `TD2`" ]
        # [ inline ( always ) ]
        pub fn is_td2(&self) -> bool {
            *self == P2_8R::TD2
        }
        # [ doc = "Checks if the value of the field is `TXD2`" ]
        # [ inline ( always ) ]
        pub fn is_txd2(&self) -> bool {
            *self == P2_8R::TXD2
        }
        # [ doc = "Checks if the value of the field is `ENET_MDC`" ]
        # [ inline ( always ) ]
        pub fn is_enet_mdc(&self) -> bool {
            *self == P2_8R::ENET_MDC
        }
    }
    # [ doc = "Possible values of the field `P2_9`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_9R {
        # [ doc = "GPIO P2.9" ]
        GPIO_P2,
        # [ doc = "USB_CONNECT" ]
        USB_CONNECT,
        # [ doc = "RXD2" ]
        RXD2,
        # [ doc = "ENET_MDIO" ]
        ENET_MDIO,
    }
    impl P2_9R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_9R::GPIO_P2 => 0,
                P2_9R::USB_CONNECT => 1,
                P2_9R::RXD2 => 2,
                P2_9R::ENET_MDIO => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_9R {
            match value {
                0 => P2_9R::GPIO_P2,
                1 => P2_9R::USB_CONNECT,
                2 => P2_9R::RXD2,
                3 => P2_9R::ENET_MDIO,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_9R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `USB_CONNECT`" ]
        # [ inline ( always ) ]
        pub fn is_usb_connect(&self) -> bool {
            *self == P2_9R::USB_CONNECT
        }
        # [ doc = "Checks if the value of the field is `RXD2`" ]
        # [ inline ( always ) ]
        pub fn is_rxd2(&self) -> bool {
            *self == P2_9R::RXD2
        }
        # [ doc = "Checks if the value of the field is `ENET_MDIO`" ]
        # [ inline ( always ) ]
        pub fn is_enet_mdio(&self) -> bool {
            *self == P2_9R::ENET_MDIO
        }
    }
    # [ doc = "Possible values of the field `P2_10`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_10R {
        # [ doc = "GPIO P2.10" ]
        GPIO_P2,
        # [ doc = "EINT0" ]
        EINT0,
        # [ doc = "NMI" ]
        NMI,
    }
    impl P2_10R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_10R::GPIO_P2 => 0,
                P2_10R::EINT0 => 1,
                P2_10R::NMI => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_10R {
            match value {
                0 => P2_10R::GPIO_P2,
                1 => P2_10R::EINT0,
                2 => P2_10R::NMI,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_10R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `EINT0`" ]
        # [ inline ( always ) ]
        pub fn is_eint0(&self) -> bool {
            *self == P2_10R::EINT0
        }
        # [ doc = "Checks if the value of the field is `NMI`" ]
        # [ inline ( always ) ]
        pub fn is_nmi(&self) -> bool {
            *self == P2_10R::NMI
        }
    }
    # [ doc = "Possible values of the field `P2_11`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_11R {
        # [ doc = "GPIO P2.11" ]
        GPIO_P2,
        # [ doc = "EINT1" ]
        EINT1,
        # [ doc = "I2STX_CLK" ]
        I2STX_CLK,
    }
    impl P2_11R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_11R::GPIO_P2 => 0,
                P2_11R::EINT1 => 1,
                P2_11R::I2STX_CLK => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_11R {
            match value {
                0 => P2_11R::GPIO_P2,
                1 => P2_11R::EINT1,
                3 => P2_11R::I2STX_CLK,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_11R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `EINT1`" ]
        # [ inline ( always ) ]
        pub fn is_eint1(&self) -> bool {
            *self == P2_11R::EINT1
        }
        # [ doc = "Checks if the value of the field is `I2STX_CLK`" ]
        # [ inline ( always ) ]
        pub fn is_i2stx_clk(&self) -> bool {
            *self == P2_11R::I2STX_CLK
        }
    }
    # [ doc = "Possible values of the field `P2_12`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_12R {
        # [ doc = "GPIO P2.12" ]
        GPIO_P2,
        # [ doc = "EINT2" ]
        EINT2,
        # [ doc = "I2STX_WS" ]
        I2STX_WS,
    }
    impl P2_12R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_12R::GPIO_P2 => 0,
                P2_12R::EINT2 => 1,
                P2_12R::I2STX_WS => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_12R {
            match value {
                0 => P2_12R::GPIO_P2,
                1 => P2_12R::EINT2,
                3 => P2_12R::I2STX_WS,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_12R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `EINT2`" ]
        # [ inline ( always ) ]
        pub fn is_eint2(&self) -> bool {
            *self == P2_12R::EINT2
        }
        # [ doc = "Checks if the value of the field is `I2STX_WS`" ]
        # [ inline ( always ) ]
        pub fn is_i2stx_ws(&self) -> bool {
            *self == P2_12R::I2STX_WS
        }
    }
    # [ doc = "Possible values of the field `P2_13`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_13R {
        # [ doc = "GPIO P2.13" ]
        GPIO_P2,
        # [ doc = "EINT3" ]
        EINT3,
        # [ doc = "I2STX_SDA" ]
        I2STX_SDA,
    }
    impl P2_13R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_13R::GPIO_P2 => 0,
                P2_13R::EINT3 => 1,
                P2_13R::I2STX_SDA => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_13R {
            match value {
                0 => P2_13R::GPIO_P2,
                1 => P2_13R::EINT3,
                3 => P2_13R::I2STX_SDA,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P2`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p2(&self) -> bool {
            *self == P2_13R::GPIO_P2
        }
        # [ doc = "Checks if the value of the field is `EINT3`" ]
        # [ inline ( always ) ]
        pub fn is_eint3(&self) -> bool {
            *self == P2_13R::EINT3
        }
        # [ doc = "Checks if the value of the field is `I2STX_SDA`" ]
        # [ inline ( always ) ]
        pub fn is_i2stx_sda(&self) -> bool {
            *self == P2_13R::I2STX_SDA
        }
    }
    # [ doc = "Values that can be written to the field `P2_0`" ]
    pub enum P2_0W {
        # [ doc = "GPIO P2.0" ]
        GPIO_P2,
        # [ doc = "PWM1.1" ]
        PWM1,
        # [ doc = "TXD1" ]
        TXD1,
    }
    impl P2_0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_0W::GPIO_P2 => 0,
                P2_0W::PWM1 => 1,
                P2_0W::TXD1 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_0W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.0" ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_0W::GPIO_P2)
        }
        # [ doc = "PWM1.1" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P2_0W::PWM1)
        }
        # [ doc = "TXD1" ]
        # [ inline ( always ) ]
        pub fn txd1(self) -> &'a mut W {
            self.variant(P2_0W::TXD1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_1`" ]
    pub enum P2_1W {
        # [ doc = "GPIO P2.1" ]
        GPIO_P2,
        # [ doc = "PWM1.2" ]
        PWM1,
        # [ doc = "RXD1" ]
        RXD1,
    }
    impl P2_1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_1W::GPIO_P2 => 0,
                P2_1W::PWM1 => 1,
                P2_1W::RXD1 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_1W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.1" ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_1W::GPIO_P2)
        }
        # [ doc = "PWM1.2" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P2_1W::PWM1)
        }
        # [ doc = "RXD1" ]
        # [ inline ( always ) ]
        pub fn rxd1(self) -> &'a mut W {
            self.variant(P2_1W::RXD1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_2`" ]
    pub enum P2_2W {
        # [ doc = "GPIO P2.2" ]
        GPIO_P2,
        # [ doc = "PWM1.3" ]
        PWM1,
        # [ doc = "CTS1" ]
        CTS1,
    }
    impl P2_2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_2W::GPIO_P2 => 0,
                P2_2W::PWM1 => 1,
                P2_2W::CTS1 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_2W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.2" ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_2W::GPIO_P2)
        }
        # [ doc = "PWM1.3" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P2_2W::PWM1)
        }
        # [ doc = "CTS1" ]
        # [ inline ( always ) ]
        pub fn cts1(self) -> &'a mut W {
            self.variant(P2_2W::CTS1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_3`" ]
    pub enum P2_3W {
        # [ doc = "GPIO P2.3." ]
        GPIO_P2,
        # [ doc = "PWM1.4" ]
        PWM1,
        # [ doc = "DCD1" ]
        DCD1,
    }
    impl P2_3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_3W::GPIO_P2 => 0,
                P2_3W::PWM1 => 1,
                P2_3W::DCD1 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_3W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.3." ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_3W::GPIO_P2)
        }
        # [ doc = "PWM1.4" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P2_3W::PWM1)
        }
        # [ doc = "DCD1" ]
        # [ inline ( always ) ]
        pub fn dcd1(self) -> &'a mut W {
            self.variant(P2_3W::DCD1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_4`" ]
    pub enum P2_4W {
        # [ doc = "GPIO P2.4." ]
        GPIO_P2,
        # [ doc = "PWM1.5" ]
        PWM1,
        # [ doc = "DSR1" ]
        DSR1,
    }
    impl P2_4W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_4W::GPIO_P2 => 0,
                P2_4W::PWM1 => 1,
                P2_4W::DSR1 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_4W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.4." ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_4W::GPIO_P2)
        }
        # [ doc = "PWM1.5" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P2_4W::PWM1)
        }
        # [ doc = "DSR1" ]
        # [ inline ( always ) ]
        pub fn dsr1(self) -> &'a mut W {
            self.variant(P2_4W::DSR1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_5`" ]
    pub enum P2_5W {
        # [ doc = "GPIO P2.5." ]
        GPIO_P2,
        # [ doc = "PWM1.6" ]
        PWM1,
        # [ doc = "DTR1" ]
        DTR1,
    }
    impl P2_5W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_5W::GPIO_P2 => 0,
                P2_5W::PWM1 => 1,
                P2_5W::DTR1 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_5W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.5." ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_5W::GPIO_P2)
        }
        # [ doc = "PWM1.6" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P2_5W::PWM1)
        }
        # [ doc = "DTR1" ]
        # [ inline ( always ) ]
        pub fn dtr1(self) -> &'a mut W {
            self.variant(P2_5W::DTR1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_6`" ]
    pub enum P2_6W {
        # [ doc = "GPIO P2.6." ]
        GPIO_P2,
        # [ doc = "PCAP1.0" ]
        PCAP1,
        # [ doc = "RI1" ]
        RI1,
    }
    impl P2_6W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_6W::GPIO_P2 => 0,
                P2_6W::PCAP1 => 1,
                P2_6W::RI1 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_6W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.6." ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_6W::GPIO_P2)
        }
        # [ doc = "PCAP1.0" ]
        # [ inline ( always ) ]
        pub fn pcap1(self) -> &'a mut W {
            self.variant(P2_6W::PCAP1)
        }
        # [ doc = "RI1" ]
        # [ inline ( always ) ]
        pub fn ri1(self) -> &'a mut W {
            self.variant(P2_6W::RI1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_7`" ]
    pub enum P2_7W {
        # [ doc = "GPIO P2.7." ]
        GPIO_P2,
        # [ doc = "RD2" ]
        RD2,
        # [ doc = "RTS1" ]
        RTS1,
    }
    impl P2_7W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_7W::GPIO_P2 => 0,
                P2_7W::RD2 => 1,
                P2_7W::RTS1 => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_7W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.7." ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_7W::GPIO_P2)
        }
        # [ doc = "RD2" ]
        # [ inline ( always ) ]
        pub fn rd2(self) -> &'a mut W {
            self.variant(P2_7W::RD2)
        }
        # [ doc = "RTS1" ]
        # [ inline ( always ) ]
        pub fn rts1(self) -> &'a mut W {
            self.variant(P2_7W::RTS1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_8`" ]
    pub enum P2_8W {
        # [ doc = "GPIO P2.8." ]
        GPIO_P2,
        # [ doc = "TD2" ]
        TD2,
        # [ doc = "TXD2" ]
        TXD2,
        # [ doc = "ENET_MDC" ]
        ENET_MDC,
    }
    impl P2_8W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_8W::GPIO_P2 => 0,
                P2_8W::TD2 => 1,
                P2_8W::TXD2 => 2,
                P2_8W::ENET_MDC => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_8W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P2.8." ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_8W::GPIO_P2)
        }
        # [ doc = "TD2" ]
        # [ inline ( always ) ]
        pub fn td2(self) -> &'a mut W {
            self.variant(P2_8W::TD2)
        }
        # [ doc = "TXD2" ]
        # [ inline ( always ) ]
        pub fn txd2(self) -> &'a mut W {
            self.variant(P2_8W::TXD2)
        }
        # [ doc = "ENET_MDC" ]
        # [ inline ( always ) ]
        pub fn enet_mdc(self) -> &'a mut W {
            self.variant(P2_8W::ENET_MDC)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_9`" ]
    pub enum P2_9W {
        # [ doc = "GPIO P2.9" ]
        GPIO_P2,
        # [ doc = "USB_CONNECT" ]
        USB_CONNECT,
        # [ doc = "RXD2" ]
        RXD2,
        # [ doc = "ENET_MDIO" ]
        ENET_MDIO,
    }
    impl P2_9W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_9W::GPIO_P2 => 0,
                P2_9W::USB_CONNECT => 1,
                P2_9W::RXD2 => 2,
                P2_9W::ENET_MDIO => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_9W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P2.9" ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_9W::GPIO_P2)
        }
        # [ doc = "USB_CONNECT" ]
        # [ inline ( always ) ]
        pub fn usb_connect(self) -> &'a mut W {
            self.variant(P2_9W::USB_CONNECT)
        }
        # [ doc = "RXD2" ]
        # [ inline ( always ) ]
        pub fn rxd2(self) -> &'a mut W {
            self.variant(P2_9W::RXD2)
        }
        # [ doc = "ENET_MDIO" ]
        # [ inline ( always ) ]
        pub fn enet_mdio(self) -> &'a mut W {
            self.variant(P2_9W::ENET_MDIO)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_10`" ]
    pub enum P2_10W {
        # [ doc = "GPIO P2.10" ]
        GPIO_P2,
        # [ doc = "EINT0" ]
        EINT0,
        # [ doc = "NMI" ]
        NMI,
    }
    impl P2_10W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_10W::GPIO_P2 => 0,
                P2_10W::EINT0 => 1,
                P2_10W::NMI => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_10W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.10" ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_10W::GPIO_P2)
        }
        # [ doc = "EINT0" ]
        # [ inline ( always ) ]
        pub fn eint0(self) -> &'a mut W {
            self.variant(P2_10W::EINT0)
        }
        # [ doc = "NMI" ]
        # [ inline ( always ) ]
        pub fn nmi(self) -> &'a mut W {
            self.variant(P2_10W::NMI)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_11`" ]
    pub enum P2_11W {
        # [ doc = "GPIO P2.11" ]
        GPIO_P2,
        # [ doc = "EINT1" ]
        EINT1,
        # [ doc = "I2STX_CLK" ]
        I2STX_CLK,
    }
    impl P2_11W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_11W::GPIO_P2 => 0,
                P2_11W::EINT1 => 1,
                P2_11W::I2STX_CLK => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_11W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.11" ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_11W::GPIO_P2)
        }
        # [ doc = "EINT1" ]
        # [ inline ( always ) ]
        pub fn eint1(self) -> &'a mut W {
            self.variant(P2_11W::EINT1)
        }
        # [ doc = "I2STX_CLK" ]
        # [ inline ( always ) ]
        pub fn i2stx_clk(self) -> &'a mut W {
            self.variant(P2_11W::I2STX_CLK)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_12`" ]
    pub enum P2_12W {
        # [ doc = "GPIO P2.12" ]
        GPIO_P2,
        # [ doc = "EINT2" ]
        EINT2,
        # [ doc = "I2STX_WS" ]
        I2STX_WS,
    }
    impl P2_12W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_12W::GPIO_P2 => 0,
                P2_12W::EINT2 => 1,
                P2_12W::I2STX_WS => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_12W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.12" ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_12W::GPIO_P2)
        }
        # [ doc = "EINT2" ]
        # [ inline ( always ) ]
        pub fn eint2(self) -> &'a mut W {
            self.variant(P2_12W::EINT2)
        }
        # [ doc = "I2STX_WS" ]
        # [ inline ( always ) ]
        pub fn i2stx_ws(self) -> &'a mut W {
            self.variant(P2_12W::I2STX_WS)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_13`" ]
    pub enum P2_13W {
        # [ doc = "GPIO P2.13" ]
        GPIO_P2,
        # [ doc = "EINT3" ]
        EINT3,
        # [ doc = "I2STX_SDA" ]
        I2STX_SDA,
    }
    impl P2_13W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_13W::GPIO_P2 => 0,
                P2_13W::EINT3 => 1,
                P2_13W::I2STX_SDA => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_13W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P2.13" ]
        # [ inline ( always ) ]
        pub fn gpio_p2(self) -> &'a mut W {
            self.variant(P2_13W::GPIO_P2)
        }
        # [ doc = "EINT3" ]
        # [ inline ( always ) ]
        pub fn eint3(self) -> &'a mut W {
            self.variant(P2_13W::EINT3)
        }
        # [ doc = "I2STX_SDA" ]
        # [ inline ( always ) ]
        pub fn i2stx_sda(self) -> &'a mut W {
            self.variant(P2_13W::I2STX_SDA)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Pin function select P2.0." ]
        # [ inline ( always ) ]
        pub fn p2_0(&self) -> P2_0R {
            P2_0R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 0;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 2:3 - Pin function select P2.1." ]
        # [ inline ( always ) ]
        pub fn p2_1(&self) -> P2_1R {
            P2_1R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 2;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 4:5 - Pin function select P2.2." ]
        # [ inline ( always ) ]
        pub fn p2_2(&self) -> P2_2R {
            P2_2R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 4;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 6:7 - Pin function select P2.3." ]
        # [ inline ( always ) ]
        pub fn p2_3(&self) -> P2_3R {
            P2_3R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 6;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 8:9 - Pin function select P2.4." ]
        # [ inline ( always ) ]
        pub fn p2_4(&self) -> P2_4R {
            P2_4R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 8;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 10:11 - Pin function select P2.5." ]
        # [ inline ( always ) ]
        pub fn p2_5(&self) -> P2_5R {
            P2_5R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 10;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 12:13 - Pin function select P2.6." ]
        # [ inline ( always ) ]
        pub fn p2_6(&self) -> P2_6R {
            P2_6R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 12;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 14:15 - Pin function select P2.7." ]
        # [ inline ( always ) ]
        pub fn p2_7(&self) -> P2_7R {
            P2_7R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 14;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 16:17 - Pin function select P2.8." ]
        # [ inline ( always ) ]
        pub fn p2_8(&self) -> P2_8R {
            P2_8R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 16;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 18:19 - Pin function select P2.9." ]
        # [ inline ( always ) ]
        pub fn p2_9(&self) -> P2_9R {
            P2_9R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 18;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 20:21 - Pin function select P2.10." ]
        # [ inline ( always ) ]
        pub fn p2_10(&self) -> P2_10R {
            P2_10R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 20;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 22:23 - Pin function select P2.11." ]
        # [ inline ( always ) ]
        pub fn p2_11(&self) -> P2_11R {
            P2_11R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 22;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 24:25 - Pin function select P2.12." ]
        # [ inline ( always ) ]
        pub fn p2_12(&self) -> P2_12R {
            P2_12R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 24;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 26:27 - Pin function select P2.13." ]
        # [ inline ( always ) ]
        pub fn p2_13(&self) -> P2_13R {
            P2_13R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 26;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Pin function select P2.0." ]
        # [ inline ( always ) ]
        pub fn p2_0(&mut self) -> _P2_0W {
            _P2_0W { w: self }
        }
        # [ doc = "Bits 2:3 - Pin function select P2.1." ]
        # [ inline ( always ) ]
        pub fn p2_1(&mut self) -> _P2_1W {
            _P2_1W { w: self }
        }
        # [ doc = "Bits 4:5 - Pin function select P2.2." ]
        # [ inline ( always ) ]
        pub fn p2_2(&mut self) -> _P2_2W {
            _P2_2W { w: self }
        }
        # [ doc = "Bits 6:7 - Pin function select P2.3." ]
        # [ inline ( always ) ]
        pub fn p2_3(&mut self) -> _P2_3W {
            _P2_3W { w: self }
        }
        # [ doc = "Bits 8:9 - Pin function select P2.4." ]
        # [ inline ( always ) ]
        pub fn p2_4(&mut self) -> _P2_4W {
            _P2_4W { w: self }
        }
        # [ doc = "Bits 10:11 - Pin function select P2.5." ]
        # [ inline ( always ) ]
        pub fn p2_5(&mut self) -> _P2_5W {
            _P2_5W { w: self }
        }
        # [ doc = "Bits 12:13 - Pin function select P2.6." ]
        # [ inline ( always ) ]
        pub fn p2_6(&mut self) -> _P2_6W {
            _P2_6W { w: self }
        }
        # [ doc = "Bits 14:15 - Pin function select P2.7." ]
        # [ inline ( always ) ]
        pub fn p2_7(&mut self) -> _P2_7W {
            _P2_7W { w: self }
        }
        # [ doc = "Bits 16:17 - Pin function select P2.8." ]
        # [ inline ( always ) ]
        pub fn p2_8(&mut self) -> _P2_8W {
            _P2_8W { w: self }
        }
        # [ doc = "Bits 18:19 - Pin function select P2.9." ]
        # [ inline ( always ) ]
        pub fn p2_9(&mut self) -> _P2_9W {
            _P2_9W { w: self }
        }
        # [ doc = "Bits 20:21 - Pin function select P2.10." ]
        # [ inline ( always ) ]
        pub fn p2_10(&mut self) -> _P2_10W {
            _P2_10W { w: self }
        }
        # [ doc = "Bits 22:23 - Pin function select P2.11." ]
        # [ inline ( always ) ]
        pub fn p2_11(&mut self) -> _P2_11W {
            _P2_11W { w: self }
        }
        # [ doc = "Bits 24:25 - Pin function select P2.12." ]
        # [ inline ( always ) ]
        pub fn p2_12(&mut self) -> _P2_12W {
            _P2_12W { w: self }
        }
        # [ doc = "Bits 26:27 - Pin function select P2.13." ]
        # [ inline ( always ) ]
        pub fn p2_13(&mut self) -> _P2_13W {
            _P2_13W { w: self }
        }
    }
}
# [ doc = "Pin function select register 7" ]
pub struct PINSEL7 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin function select register 7" ]
pub mod pinsel7 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINSEL7 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P3_25`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P3_25R {
        # [ doc = "GPIO P3.25" ]
        GPIO_P3,
        # [ doc = "MAT0.0" ]
        MAT0,
        # [ doc = "PWM1.2" ]
        PWM1,
    }
    impl P3_25R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P3_25R::GPIO_P3 => 0,
                P3_25R::MAT0 => 2,
                P3_25R::PWM1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P3_25R {
            match value {
                0 => P3_25R::GPIO_P3,
                2 => P3_25R::MAT0,
                3 => P3_25R::PWM1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P3`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p3(&self) -> bool {
            *self == P3_25R::GPIO_P3
        }
        # [ doc = "Checks if the value of the field is `MAT0`" ]
        # [ inline ( always ) ]
        pub fn is_mat0(&self) -> bool {
            *self == P3_25R::MAT0
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P3_25R::PWM1
        }
    }
    # [ doc = "Possible values of the field `P3_26`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P3_26R {
        # [ doc = "GPIO P3.26" ]
        GPIO_P3,
        # [ doc = "STCLK" ]
        STCLK,
        # [ doc = "MAT0.1" ]
        MAT0,
        # [ doc = "PWM1.3" ]
        PWM1,
    }
    impl P3_26R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P3_26R::GPIO_P3 => 0,
                P3_26R::STCLK => 1,
                P3_26R::MAT0 => 2,
                P3_26R::PWM1 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P3_26R {
            match value {
                0 => P3_26R::GPIO_P3,
                1 => P3_26R::STCLK,
                2 => P3_26R::MAT0,
                3 => P3_26R::PWM1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P3`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p3(&self) -> bool {
            *self == P3_26R::GPIO_P3
        }
        # [ doc = "Checks if the value of the field is `STCLK`" ]
        # [ inline ( always ) ]
        pub fn is_stclk(&self) -> bool {
            *self == P3_26R::STCLK
        }
        # [ doc = "Checks if the value of the field is `MAT0`" ]
        # [ inline ( always ) ]
        pub fn is_mat0(&self) -> bool {
            *self == P3_26R::MAT0
        }
        # [ doc = "Checks if the value of the field is `PWM1`" ]
        # [ inline ( always ) ]
        pub fn is_pwm1(&self) -> bool {
            *self == P3_26R::PWM1
        }
    }
    # [ doc = "Values that can be written to the field `P3_25`" ]
    pub enum P3_25W {
        # [ doc = "GPIO P3.25" ]
        GPIO_P3,
        # [ doc = "MAT0.0" ]
        MAT0,
        # [ doc = "PWM1.2" ]
        PWM1,
    }
    impl P3_25W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P3_25W::GPIO_P3 => 0,
                P3_25W::MAT0 => 2,
                P3_25W::PWM1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P3_25W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P3_25W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P3_25W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "GPIO P3.25" ]
        # [ inline ( always ) ]
        pub fn gpio_p3(self) -> &'a mut W {
            self.variant(P3_25W::GPIO_P3)
        }
        # [ doc = "MAT0.0" ]
        # [ inline ( always ) ]
        pub fn mat0(self) -> &'a mut W {
            self.variant(P3_25W::MAT0)
        }
        # [ doc = "PWM1.2" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P3_25W::PWM1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P3_26`" ]
    pub enum P3_26W {
        # [ doc = "GPIO P3.26" ]
        GPIO_P3,
        # [ doc = "STCLK" ]
        STCLK,
        # [ doc = "MAT0.1" ]
        MAT0,
        # [ doc = "PWM1.3" ]
        PWM1,
    }
    impl P3_26W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P3_26W::GPIO_P3 => 0,
                P3_26W::STCLK => 1,
                P3_26W::MAT0 => 2,
                P3_26W::PWM1 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P3_26W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P3_26W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P3_26W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P3.26" ]
        # [ inline ( always ) ]
        pub fn gpio_p3(self) -> &'a mut W {
            self.variant(P3_26W::GPIO_P3)
        }
        # [ doc = "STCLK" ]
        # [ inline ( always ) ]
        pub fn stclk(self) -> &'a mut W {
            self.variant(P3_26W::STCLK)
        }
        # [ doc = "MAT0.1" ]
        # [ inline ( always ) ]
        pub fn mat0(self) -> &'a mut W {
            self.variant(P3_26W::MAT0)
        }
        # [ doc = "PWM1.3" ]
        # [ inline ( always ) ]
        pub fn pwm1(self) -> &'a mut W {
            self.variant(P3_26W::PWM1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 18:19 - Pin function select P3.25." ]
        # [ inline ( always ) ]
        pub fn p3_25(&self) -> P3_25R {
            P3_25R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 18;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 20:21 - Pin function select P3.26." ]
        # [ inline ( always ) ]
        pub fn p3_26(&self) -> P3_26R {
            P3_26R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 20;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 18:19 - Pin function select P3.25." ]
        # [ inline ( always ) ]
        pub fn p3_25(&mut self) -> _P3_25W {
            _P3_25W { w: self }
        }
        # [ doc = "Bits 20:21 - Pin function select P3.26." ]
        # [ inline ( always ) ]
        pub fn p3_26(&mut self) -> _P3_26W {
            _P3_26W { w: self }
        }
    }
}
# [ doc = "Pin function select register 9" ]
pub struct PINSEL9 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin function select register 9" ]
pub mod pinsel9 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINSEL9 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P4_28`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P4_28R {
        # [ doc = "GPIO P4.28" ]
        GPIO_P4,
        # [ doc = "RX_MCLK" ]
        RX_MCLK,
        # [ doc = "MAT2.0" ]
        MAT2,
        # [ doc = "TXD3" ]
        TXD3,
    }
    impl P4_28R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P4_28R::GPIO_P4 => 0,
                P4_28R::RX_MCLK => 1,
                P4_28R::MAT2 => 2,
                P4_28R::TXD3 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P4_28R {
            match value {
                0 => P4_28R::GPIO_P4,
                1 => P4_28R::RX_MCLK,
                2 => P4_28R::MAT2,
                3 => P4_28R::TXD3,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P4`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p4(&self) -> bool {
            *self == P4_28R::GPIO_P4
        }
        # [ doc = "Checks if the value of the field is `RX_MCLK`" ]
        # [ inline ( always ) ]
        pub fn is_rx_mclk(&self) -> bool {
            *self == P4_28R::RX_MCLK
        }
        # [ doc = "Checks if the value of the field is `MAT2`" ]
        # [ inline ( always ) ]
        pub fn is_mat2(&self) -> bool {
            *self == P4_28R::MAT2
        }
        # [ doc = "Checks if the value of the field is `TXD3`" ]
        # [ inline ( always ) ]
        pub fn is_txd3(&self) -> bool {
            *self == P4_28R::TXD3
        }
    }
    # [ doc = "Possible values of the field `P4_29`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P4_29R {
        # [ doc = "GPIO P4.29" ]
        GPIO_P4,
        # [ doc = "TX_MCLK" ]
        TX_MCLK,
        # [ doc = "MAT2.1" ]
        MAT2,
        # [ doc = "RXD3" ]
        RXD3,
    }
    impl P4_29R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P4_29R::GPIO_P4 => 0,
                P4_29R::TX_MCLK => 1,
                P4_29R::MAT2 => 2,
                P4_29R::RXD3 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P4_29R {
            match value {
                0 => P4_29R::GPIO_P4,
                1 => P4_29R::TX_MCLK,
                2 => P4_29R::MAT2,
                3 => P4_29R::RXD3,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `GPIO_P4`" ]
        # [ inline ( always ) ]
        pub fn is_gpio_p4(&self) -> bool {
            *self == P4_29R::GPIO_P4
        }
        # [ doc = "Checks if the value of the field is `TX_MCLK`" ]
        # [ inline ( always ) ]
        pub fn is_tx_mclk(&self) -> bool {
            *self == P4_29R::TX_MCLK
        }
        # [ doc = "Checks if the value of the field is `MAT2`" ]
        # [ inline ( always ) ]
        pub fn is_mat2(&self) -> bool {
            *self == P4_29R::MAT2
        }
        # [ doc = "Checks if the value of the field is `RXD3`" ]
        # [ inline ( always ) ]
        pub fn is_rxd3(&self) -> bool {
            *self == P4_29R::RXD3
        }
    }
    # [ doc = "Values that can be written to the field `P4_28`" ]
    pub enum P4_28W {
        # [ doc = "GPIO P4.28" ]
        GPIO_P4,
        # [ doc = "RX_MCLK" ]
        RX_MCLK,
        # [ doc = "MAT2.0" ]
        MAT2,
        # [ doc = "TXD3" ]
        TXD3,
    }
    impl P4_28W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P4_28W::GPIO_P4 => 0,
                P4_28W::RX_MCLK => 1,
                P4_28W::MAT2 => 2,
                P4_28W::TXD3 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P4_28W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P4_28W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P4_28W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P4.28" ]
        # [ inline ( always ) ]
        pub fn gpio_p4(self) -> &'a mut W {
            self.variant(P4_28W::GPIO_P4)
        }
        # [ doc = "RX_MCLK" ]
        # [ inline ( always ) ]
        pub fn rx_mclk(self) -> &'a mut W {
            self.variant(P4_28W::RX_MCLK)
        }
        # [ doc = "MAT2.0" ]
        # [ inline ( always ) ]
        pub fn mat2(self) -> &'a mut W {
            self.variant(P4_28W::MAT2)
        }
        # [ doc = "TXD3" ]
        # [ inline ( always ) ]
        pub fn txd3(self) -> &'a mut W {
            self.variant(P4_28W::TXD3)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P4_29`" ]
    pub enum P4_29W {
        # [ doc = "GPIO P4.29" ]
        GPIO_P4,
        # [ doc = "TX_MCLK" ]
        TX_MCLK,
        # [ doc = "MAT2.1" ]
        MAT2,
        # [ doc = "RXD3" ]
        RXD3,
    }
    impl P4_29W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P4_29W::GPIO_P4 => 0,
                P4_29W::TX_MCLK => 1,
                P4_29W::MAT2 => 2,
                P4_29W::RXD3 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P4_29W<'a> {
        w: &'a mut W,
    }
    impl<'a> _P4_29W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P4_29W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "GPIO P4.29" ]
        # [ inline ( always ) ]
        pub fn gpio_p4(self) -> &'a mut W {
            self.variant(P4_29W::GPIO_P4)
        }
        # [ doc = "TX_MCLK" ]
        # [ inline ( always ) ]
        pub fn tx_mclk(self) -> &'a mut W {
            self.variant(P4_29W::TX_MCLK)
        }
        # [ doc = "MAT2.1" ]
        # [ inline ( always ) ]
        pub fn mat2(self) -> &'a mut W {
            self.variant(P4_29W::MAT2)
        }
        # [ doc = "RXD3" ]
        # [ inline ( always ) ]
        pub fn rxd3(self) -> &'a mut W {
            self.variant(P4_29W::RXD3)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 24:25 - Pin function select P4.28." ]
        # [ inline ( always ) ]
        pub fn p4_28(&self) -> P4_28R {
            P4_28R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 24;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 26:27 - Pin function select P4.29." ]
        # [ inline ( always ) ]
        pub fn p4_29(&self) -> P4_29R {
            P4_29R::_from({
                              const MASK: u8 = 3;
                              const OFFSET: u8 = 26;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 24:25 - Pin function select P4.28." ]
        # [ inline ( always ) ]
        pub fn p4_28(&mut self) -> _P4_28W {
            _P4_28W { w: self }
        }
        # [ doc = "Bits 26:27 - Pin function select P4.29." ]
        # [ inline ( always ) ]
        pub fn p4_29(&mut self) -> _P4_29W {
            _P4_29W { w: self }
        }
    }
}
# [ doc = "Pin function select register 10" ]
pub struct PINSEL10 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin function select register 10" ]
pub mod pinsel10 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINSEL10 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `TPIUCTRL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TPIUCTRLR {
        # [ doc = "Disabled. TPIU interface is disabled." ]
        DISABLED,
        # [ doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content." ]
        ENABLED,
    }
    impl TPIUCTRLR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                TPIUCTRLR::DISABLED => false,
                TPIUCTRLR::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TPIUCTRLR {
            match value {
                false => TPIUCTRLR::DISABLED,
                true => TPIUCTRLR::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == TPIUCTRLR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == TPIUCTRLR::ENABLED
        }
    }
    # [ doc = "Values that can be written to the field `TPIUCTRL`" ]
    pub enum TPIUCTRLW {
        # [ doc = "Disabled. TPIU interface is disabled." ]
        DISABLED,
        # [ doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content." ]
        ENABLED,
    }
    impl TPIUCTRLW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                TPIUCTRLW::DISABLED => false,
                TPIUCTRLW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TPIUCTRLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TPIUCTRLW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TPIUCTRLW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled. TPIU interface is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(TPIUCTRLW::DISABLED)
        }
        # [ doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(TPIUCTRLW::ENABLED)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 3 - TPIU interface pins control." ]
        # [ inline ( always ) ]
        pub fn tpiuctrl(&self) -> TPIUCTRLR {
            TPIUCTRLR::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 3;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 3 - TPIU interface pins control." ]
        # [ inline ( always ) ]
        pub fn tpiuctrl(&mut self) -> _TPIUCTRLW {
            _TPIUCTRLW { w: self }
        }
    }
}
# [ doc = "Pin mode select register 0" ]
pub struct PINMODE0 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin mode select register 0" ]
pub mod pinmode0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE0 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P0_00MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_00MODER {
        # [ doc = "Pull-up. P0.0 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.0 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.0 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.0 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_00MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_00MODER::PULL_UP => 0,
                P0_00MODER::REPEATER => 1,
                P0_00MODER::DISABLED => 2,
                P0_00MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_00MODER {
            match value {
                0 => P0_00MODER::PULL_UP,
                1 => P0_00MODER::REPEATER,
                2 => P0_00MODER::DISABLED,
                3 => P0_00MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_00MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_00MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_00MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_00MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_01MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_01MODER {
        # [ doc = "Pull-up. P0.1 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.1 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.1 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.1 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_01MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_01MODER::PULL_UP => 0,
                P0_01MODER::REPEATER => 1,
                P0_01MODER::DISABLED => 2,
                P0_01MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_01MODER {
            match value {
                0 => P0_01MODER::PULL_UP,
                1 => P0_01MODER::REPEATER,
                2 => P0_01MODER::DISABLED,
                3 => P0_01MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_01MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_01MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_01MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_01MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_02MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_02MODER {
        # [ doc = "Pull-up. P0.2 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.2 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.2 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.2 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_02MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_02MODER::PULL_UP => 0,
                P0_02MODER::REPEATER => 1,
                P0_02MODER::DISABLED => 2,
                P0_02MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_02MODER {
            match value {
                0 => P0_02MODER::PULL_UP,
                1 => P0_02MODER::REPEATER,
                2 => P0_02MODER::DISABLED,
                3 => P0_02MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_02MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_02MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_02MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_02MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_03MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_03MODER {
        # [ doc = "Pull-up. P0.3 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.3 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.3 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.3 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_03MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_03MODER::PULL_UP => 0,
                P0_03MODER::REPEATER => 1,
                P0_03MODER::DISABLED => 2,
                P0_03MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_03MODER {
            match value {
                0 => P0_03MODER::PULL_UP,
                1 => P0_03MODER::REPEATER,
                2 => P0_03MODER::DISABLED,
                3 => P0_03MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_03MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_03MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_03MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_03MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_04MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_04MODER {
        # [ doc = "Pull-up. P0.4 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.4 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.4 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.4 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_04MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_04MODER::PULL_UP => 0,
                P0_04MODER::REPEATER => 1,
                P0_04MODER::DISABLED => 2,
                P0_04MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_04MODER {
            match value {
                0 => P0_04MODER::PULL_UP,
                1 => P0_04MODER::REPEATER,
                2 => P0_04MODER::DISABLED,
                3 => P0_04MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_04MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_04MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_04MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_04MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_05MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_05MODER {
        # [ doc = "Pull-up. P0.5 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.5 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.5 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.5 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_05MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_05MODER::PULL_UP => 0,
                P0_05MODER::REPEATER => 1,
                P0_05MODER::DISABLED => 2,
                P0_05MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_05MODER {
            match value {
                0 => P0_05MODER::PULL_UP,
                1 => P0_05MODER::REPEATER,
                2 => P0_05MODER::DISABLED,
                3 => P0_05MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_05MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_05MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_05MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_05MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_06MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_06MODER {
        # [ doc = "Pull-up. P0.6 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.6 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.6 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_06MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_06MODER::PULL_UP => 0,
                P0_06MODER::REPEATER => 1,
                P0_06MODER::DISABLED => 2,
                P0_06MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_06MODER {
            match value {
                0 => P0_06MODER::PULL_UP,
                1 => P0_06MODER::REPEATER,
                2 => P0_06MODER::DISABLED,
                3 => P0_06MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_06MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_06MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_06MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_06MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_07MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_07MODER {
        # [ doc = "Pull-up. P0.7 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.7 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.7 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.7 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_07MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_07MODER::PULL_UP => 0,
                P0_07MODER::REPEATER => 1,
                P0_07MODER::DISABLED => 2,
                P0_07MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_07MODER {
            match value {
                0 => P0_07MODER::PULL_UP,
                1 => P0_07MODER::REPEATER,
                2 => P0_07MODER::DISABLED,
                3 => P0_07MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_07MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_07MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_07MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_07MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_08MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_08MODER {
        # [ doc = "Pull-up. P0.8 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.8 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.8 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.8 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_08MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_08MODER::PULL_UP => 0,
                P0_08MODER::REPEATER => 1,
                P0_08MODER::DISABLED => 2,
                P0_08MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_08MODER {
            match value {
                0 => P0_08MODER::PULL_UP,
                1 => P0_08MODER::REPEATER,
                2 => P0_08MODER::DISABLED,
                3 => P0_08MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_08MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_08MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_08MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_08MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_09MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_09MODER {
        # [ doc = "Pull-up. P0.9 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.9 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.9 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.9 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_09MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_09MODER::PULL_UP => 0,
                P0_09MODER::REPEATER => 1,
                P0_09MODER::DISABLED => 2,
                P0_09MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_09MODER {
            match value {
                0 => P0_09MODER::PULL_UP,
                1 => P0_09MODER::REPEATER,
                2 => P0_09MODER::DISABLED,
                3 => P0_09MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_09MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_09MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_09MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_09MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_10MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_10MODER {
        # [ doc = "Pull-up. P0.10 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.10 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.10 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.10 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_10MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_10MODER::PULL_UP => 0,
                P0_10MODER::REPEATER => 1,
                P0_10MODER::DISABLED => 2,
                P0_10MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_10MODER {
            match value {
                0 => P0_10MODER::PULL_UP,
                1 => P0_10MODER::REPEATER,
                2 => P0_10MODER::DISABLED,
                3 => P0_10MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_10MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_10MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_10MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_10MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_11MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_11MODER {
        # [ doc = "Pull-up. P0.11 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.11 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.11 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.11 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_11MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_11MODER::PULL_UP => 0,
                P0_11MODER::REPEATER => 1,
                P0_11MODER::DISABLED => 2,
                P0_11MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_11MODER {
            match value {
                0 => P0_11MODER::PULL_UP,
                1 => P0_11MODER::REPEATER,
                2 => P0_11MODER::DISABLED,
                3 => P0_11MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_11MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_11MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_11MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_11MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_15MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_15MODER {
        # [ doc = "Pull-up. P0.15 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.15 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.15 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.15 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_15MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_15MODER::PULL_UP => 0,
                P0_15MODER::REPEATER => 1,
                P0_15MODER::DISABLED => 2,
                P0_15MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_15MODER {
            match value {
                0 => P0_15MODER::PULL_UP,
                1 => P0_15MODER::REPEATER,
                2 => P0_15MODER::DISABLED,
                3 => P0_15MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_15MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_15MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_15MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_15MODER::PULL_DOWN
        }
    }
    # [ doc = "Values that can be written to the field `P0_00MODE`" ]
    pub enum P0_00MODEW {
        # [ doc = "Pull-up. P0.0 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.0 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.0 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.0 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_00MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_00MODEW::PULL_UP => 0,
                P0_00MODEW::REPEATER => 1,
                P0_00MODEW::DISABLED => 2,
                P0_00MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_00MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_00MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_00MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.0 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_00MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.0 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_00MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.0 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_00MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.0 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_00MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_01MODE`" ]
    pub enum P0_01MODEW {
        # [ doc = "Pull-up. P0.1 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.1 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.1 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.1 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_01MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_01MODEW::PULL_UP => 0,
                P0_01MODEW::REPEATER => 1,
                P0_01MODEW::DISABLED => 2,
                P0_01MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_01MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_01MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_01MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.1 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_01MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.1 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_01MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.1 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_01MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.1 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_01MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_02MODE`" ]
    pub enum P0_02MODEW {
        # [ doc = "Pull-up. P0.2 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.2 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.2 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.2 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_02MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_02MODEW::PULL_UP => 0,
                P0_02MODEW::REPEATER => 1,
                P0_02MODEW::DISABLED => 2,
                P0_02MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_02MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_02MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_02MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.2 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_02MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.2 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_02MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.2 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_02MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.2 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_02MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_03MODE`" ]
    pub enum P0_03MODEW {
        # [ doc = "Pull-up. P0.3 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.3 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.3 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.3 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_03MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_03MODEW::PULL_UP => 0,
                P0_03MODEW::REPEATER => 1,
                P0_03MODEW::DISABLED => 2,
                P0_03MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_03MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_03MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_03MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.3 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_03MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.3 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_03MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.3 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_03MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.3 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_03MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_04MODE`" ]
    pub enum P0_04MODEW {
        # [ doc = "Pull-up. P0.4 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.4 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.4 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.4 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_04MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_04MODEW::PULL_UP => 0,
                P0_04MODEW::REPEATER => 1,
                P0_04MODEW::DISABLED => 2,
                P0_04MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_04MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_04MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_04MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.4 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_04MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.4 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_04MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.4 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_04MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.4 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_04MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_05MODE`" ]
    pub enum P0_05MODEW {
        # [ doc = "Pull-up. P0.5 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.5 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.5 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.5 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_05MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_05MODEW::PULL_UP => 0,
                P0_05MODEW::REPEATER => 1,
                P0_05MODEW::DISABLED => 2,
                P0_05MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_05MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_05MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_05MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.5 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_05MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.5 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_05MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.5 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_05MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.5 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_05MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_06MODE`" ]
    pub enum P0_06MODEW {
        # [ doc = "Pull-up. P0.6 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.6 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.6 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_06MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_06MODEW::PULL_UP => 0,
                P0_06MODEW::REPEATER => 1,
                P0_06MODEW::DISABLED => 2,
                P0_06MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_06MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_06MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_06MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.6 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_06MODEW::PULL_UP)
        }
        # [ doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_06MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.6 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_06MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.6 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_06MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_07MODE`" ]
    pub enum P0_07MODEW {
        # [ doc = "Pull-up. P0.7 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.7 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.7 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.7 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_07MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_07MODEW::PULL_UP => 0,
                P0_07MODEW::REPEATER => 1,
                P0_07MODEW::DISABLED => 2,
                P0_07MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_07MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_07MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_07MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.7 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_07MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.7 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_07MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.7 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_07MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.7 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_07MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_08MODE`" ]
    pub enum P0_08MODEW {
        # [ doc = "Pull-up. P0.8 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.8 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.8 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.8 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_08MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_08MODEW::PULL_UP => 0,
                P0_08MODEW::REPEATER => 1,
                P0_08MODEW::DISABLED => 2,
                P0_08MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_08MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_08MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_08MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.8 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_08MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.8 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_08MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.8 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_08MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.8 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_08MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_09MODE`" ]
    pub enum P0_09MODEW {
        # [ doc = "Pull-up. P0.9 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.9 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.9 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.9 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_09MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_09MODEW::PULL_UP => 0,
                P0_09MODEW::REPEATER => 1,
                P0_09MODEW::DISABLED => 2,
                P0_09MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_09MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_09MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_09MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.9 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_09MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.9 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_09MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.9 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_09MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.9 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_09MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_10MODE`" ]
    pub enum P0_10MODEW {
        # [ doc = "Pull-up. P0.10 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.10 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.10 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.10 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_10MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_10MODEW::PULL_UP => 0,
                P0_10MODEW::REPEATER => 1,
                P0_10MODEW::DISABLED => 2,
                P0_10MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_10MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_10MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_10MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.10 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_10MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.10 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_10MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.10 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_10MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.10 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_10MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_11MODE`" ]
    pub enum P0_11MODEW {
        # [ doc = "Pull-up. P0.11 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.11 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.11 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.11 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_11MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_11MODEW::PULL_UP => 0,
                P0_11MODEW::REPEATER => 1,
                P0_11MODEW::DISABLED => 2,
                P0_11MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_11MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_11MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_11MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.11 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_11MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.11 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_11MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.11 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_11MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.11 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_11MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_15MODE`" ]
    pub enum P0_15MODEW {
        # [ doc = "Pull-up. P0.15 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.15 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.15 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.15 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_15MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_15MODEW::PULL_UP => 0,
                P0_15MODEW::REPEATER => 1,
                P0_15MODEW::DISABLED => 2,
                P0_15MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_15MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_15MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_15MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.15 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_15MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.15 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_15MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.15 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_15MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.15 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_15MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control." ]
        # [ inline ( always ) ]
        pub fn p0_00mode(&self) -> P0_00MODER {
            P0_00MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 2:3 - Port 0 pin 1 control." ]
        # [ inline ( always ) ]
        pub fn p0_01mode(&self) -> P0_01MODER {
            P0_01MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 2;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 4:5 - Port 0 pin 2 control." ]
        # [ inline ( always ) ]
        pub fn p0_02mode(&self) -> P0_02MODER {
            P0_02MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 4;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 6:7 - Port 0 pin 3 control." ]
        # [ inline ( always ) ]
        pub fn p0_03mode(&self) -> P0_03MODER {
            P0_03MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 6;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 8:9 - Port 0 pin 4 control." ]
        # [ inline ( always ) ]
        pub fn p0_04mode(&self) -> P0_04MODER {
            P0_04MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 8;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 10:11 - Port 0 pin 5 control." ]
        # [ inline ( always ) ]
        pub fn p0_05mode(&self) -> P0_05MODER {
            P0_05MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 10;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 12:13 - Port 0 pin 6 control." ]
        # [ inline ( always ) ]
        pub fn p0_06mode(&self) -> P0_06MODER {
            P0_06MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 12;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 14:15 - Port 0 pin 7 control." ]
        # [ inline ( always ) ]
        pub fn p0_07mode(&self) -> P0_07MODER {
            P0_07MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 14;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 16:17 - Port 0 pin 8 control." ]
        # [ inline ( always ) ]
        pub fn p0_08mode(&self) -> P0_08MODER {
            P0_08MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 16;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 18:19 - Port 0 pin 9 control." ]
        # [ inline ( always ) ]
        pub fn p0_09mode(&self) -> P0_09MODER {
            P0_09MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 18;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 20:21 - Port 0 pin 10 control." ]
        # [ inline ( always ) ]
        pub fn p0_10mode(&self) -> P0_10MODER {
            P0_10MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 20;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 22:23 - Port 0 pin 11 control." ]
        # [ inline ( always ) ]
        pub fn p0_11mode(&self) -> P0_11MODER {
            P0_11MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 22;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 30:31 - Port 0 pin 15 control." ]
        # [ inline ( always ) ]
        pub fn p0_15mode(&self) -> P0_15MODER {
            P0_15MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 30;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control." ]
        # [ inline ( always ) ]
        pub fn p0_00mode(&mut self) -> _P0_00MODEW {
            _P0_00MODEW { w: self }
        }
        # [ doc = "Bits 2:3 - Port 0 pin 1 control." ]
        # [ inline ( always ) ]
        pub fn p0_01mode(&mut self) -> _P0_01MODEW {
            _P0_01MODEW { w: self }
        }
        # [ doc = "Bits 4:5 - Port 0 pin 2 control." ]
        # [ inline ( always ) ]
        pub fn p0_02mode(&mut self) -> _P0_02MODEW {
            _P0_02MODEW { w: self }
        }
        # [ doc = "Bits 6:7 - Port 0 pin 3 control." ]
        # [ inline ( always ) ]
        pub fn p0_03mode(&mut self) -> _P0_03MODEW {
            _P0_03MODEW { w: self }
        }
        # [ doc = "Bits 8:9 - Port 0 pin 4 control." ]
        # [ inline ( always ) ]
        pub fn p0_04mode(&mut self) -> _P0_04MODEW {
            _P0_04MODEW { w: self }
        }
        # [ doc = "Bits 10:11 - Port 0 pin 5 control." ]
        # [ inline ( always ) ]
        pub fn p0_05mode(&mut self) -> _P0_05MODEW {
            _P0_05MODEW { w: self }
        }
        # [ doc = "Bits 12:13 - Port 0 pin 6 control." ]
        # [ inline ( always ) ]
        pub fn p0_06mode(&mut self) -> _P0_06MODEW {
            _P0_06MODEW { w: self }
        }
        # [ doc = "Bits 14:15 - Port 0 pin 7 control." ]
        # [ inline ( always ) ]
        pub fn p0_07mode(&mut self) -> _P0_07MODEW {
            _P0_07MODEW { w: self }
        }
        # [ doc = "Bits 16:17 - Port 0 pin 8 control." ]
        # [ inline ( always ) ]
        pub fn p0_08mode(&mut self) -> _P0_08MODEW {
            _P0_08MODEW { w: self }
        }
        # [ doc = "Bits 18:19 - Port 0 pin 9 control." ]
        # [ inline ( always ) ]
        pub fn p0_09mode(&mut self) -> _P0_09MODEW {
            _P0_09MODEW { w: self }
        }
        # [ doc = "Bits 20:21 - Port 0 pin 10 control." ]
        # [ inline ( always ) ]
        pub fn p0_10mode(&mut self) -> _P0_10MODEW {
            _P0_10MODEW { w: self }
        }
        # [ doc = "Bits 22:23 - Port 0 pin 11 control." ]
        # [ inline ( always ) ]
        pub fn p0_11mode(&mut self) -> _P0_11MODEW {
            _P0_11MODEW { w: self }
        }
        # [ doc = "Bits 30:31 - Port 0 pin 15 control." ]
        # [ inline ( always ) ]
        pub fn p0_15mode(&mut self) -> _P0_15MODEW {
            _P0_15MODEW { w: self }
        }
    }
}
# [ doc = "Pin mode select register 1" ]
pub struct PINMODE1 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin mode select register 1" ]
pub mod pinmode1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE1 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P0_16MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_16MODER {
        # [ doc = "Pull-up. P0.16 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.16 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.16 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.16 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_16MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_16MODER::PULL_UP => 0,
                P0_16MODER::REPEATER => 1,
                P0_16MODER::DISABLED => 2,
                P0_16MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_16MODER {
            match value {
                0 => P0_16MODER::PULL_UP,
                1 => P0_16MODER::REPEATER,
                2 => P0_16MODER::DISABLED,
                3 => P0_16MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_16MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_16MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_16MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_16MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_17MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_17MODER {
        # [ doc = "Pull-up. P0.17 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.17 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.17 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.17 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_17MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_17MODER::PULL_UP => 0,
                P0_17MODER::REPEATER => 1,
                P0_17MODER::DISABLED => 2,
                P0_17MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_17MODER {
            match value {
                0 => P0_17MODER::PULL_UP,
                1 => P0_17MODER::REPEATER,
                2 => P0_17MODER::DISABLED,
                3 => P0_17MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_17MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_17MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_17MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_17MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_18MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_18MODER {
        # [ doc = "Pull-up. P0.18 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.18 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.18 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.18 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_18MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_18MODER::PULL_UP => 0,
                P0_18MODER::REPEATER => 1,
                P0_18MODER::DISABLED => 2,
                P0_18MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_18MODER {
            match value {
                0 => P0_18MODER::PULL_UP,
                1 => P0_18MODER::REPEATER,
                2 => P0_18MODER::DISABLED,
                3 => P0_18MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_18MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_18MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_18MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_18MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_19MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_19MODER {
        # [ doc = "Pull-up. P0.19 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.19 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.19 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.19 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_19MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_19MODER::PULL_UP => 0,
                P0_19MODER::REPEATER => 1,
                P0_19MODER::DISABLED => 2,
                P0_19MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_19MODER {
            match value {
                0 => P0_19MODER::PULL_UP,
                1 => P0_19MODER::REPEATER,
                2 => P0_19MODER::DISABLED,
                3 => P0_19MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_19MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_19MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_19MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_19MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_20MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_20MODER {
        # [ doc = "Pull-up. P0.20 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.20 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.20 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.20 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_20MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_20MODER::PULL_UP => 0,
                P0_20MODER::REPEATER => 1,
                P0_20MODER::DISABLED => 2,
                P0_20MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_20MODER {
            match value {
                0 => P0_20MODER::PULL_UP,
                1 => P0_20MODER::REPEATER,
                2 => P0_20MODER::DISABLED,
                3 => P0_20MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_20MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_20MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_20MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_20MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_21MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_21MODER {
        # [ doc = "Pull-up. P0.21 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.21 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.21 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.21 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_21MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_21MODER::PULL_UP => 0,
                P0_21MODER::REPEATER => 1,
                P0_21MODER::DISABLED => 2,
                P0_21MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_21MODER {
            match value {
                0 => P0_21MODER::PULL_UP,
                1 => P0_21MODER::REPEATER,
                2 => P0_21MODER::DISABLED,
                3 => P0_21MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_21MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_21MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_21MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_21MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_22MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_22MODER {
        # [ doc = "Pull-up. P0.22 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.22 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.22 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.22 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_22MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_22MODER::PULL_UP => 0,
                P0_22MODER::REPEATER => 1,
                P0_22MODER::DISABLED => 2,
                P0_22MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_22MODER {
            match value {
                0 => P0_22MODER::PULL_UP,
                1 => P0_22MODER::REPEATER,
                2 => P0_22MODER::DISABLED,
                3 => P0_22MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_22MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_22MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_22MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_22MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_23MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_23MODER {
        # [ doc = "Pull-up. P0.23 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.23 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.23 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.23 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_23MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_23MODER::PULL_UP => 0,
                P0_23MODER::REPEATER => 1,
                P0_23MODER::DISABLED => 2,
                P0_23MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_23MODER {
            match value {
                0 => P0_23MODER::PULL_UP,
                1 => P0_23MODER::REPEATER,
                2 => P0_23MODER::DISABLED,
                3 => P0_23MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_23MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_23MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_23MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_23MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_24MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_24MODER {
        # [ doc = "Pull-up. P0.24 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.24 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.24 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.24 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_24MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_24MODER::PULL_UP => 0,
                P0_24MODER::REPEATER => 1,
                P0_24MODER::DISABLED => 2,
                P0_24MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_24MODER {
            match value {
                0 => P0_24MODER::PULL_UP,
                1 => P0_24MODER::REPEATER,
                2 => P0_24MODER::DISABLED,
                3 => P0_24MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_24MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_24MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_24MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_24MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_25MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_25MODER {
        # [ doc = "Pull-up. P0.25 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.25 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.25 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.25 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_25MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_25MODER::PULL_UP => 0,
                P0_25MODER::REPEATER => 1,
                P0_25MODER::DISABLED => 2,
                P0_25MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_25MODER {
            match value {
                0 => P0_25MODER::PULL_UP,
                1 => P0_25MODER::REPEATER,
                2 => P0_25MODER::DISABLED,
                3 => P0_25MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_25MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_25MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_25MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_25MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P0_26MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_26MODER {
        # [ doc = "Pull-up. P0.26 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.26 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.26 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.26 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_26MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P0_26MODER::PULL_UP => 0,
                P0_26MODER::REPEATER => 1,
                P0_26MODER::DISABLED => 2,
                P0_26MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P0_26MODER {
            match value {
                0 => P0_26MODER::PULL_UP,
                1 => P0_26MODER::REPEATER,
                2 => P0_26MODER::DISABLED,
                3 => P0_26MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P0_26MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P0_26MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P0_26MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P0_26MODER::PULL_DOWN
        }
    }
    # [ doc = "Values that can be written to the field `P0_16MODE`" ]
    pub enum P0_16MODEW {
        # [ doc = "Pull-up. P0.16 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.16 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.16 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.16 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_16MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_16MODEW::PULL_UP => 0,
                P0_16MODEW::REPEATER => 1,
                P0_16MODEW::DISABLED => 2,
                P0_16MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_16MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_16MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_16MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.16 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_16MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.16 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_16MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.16 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_16MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.16 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_16MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_17MODE`" ]
    pub enum P0_17MODEW {
        # [ doc = "Pull-up. P0.17 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.17 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.17 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.17 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_17MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_17MODEW::PULL_UP => 0,
                P0_17MODEW::REPEATER => 1,
                P0_17MODEW::DISABLED => 2,
                P0_17MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_17MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_17MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_17MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.17 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_17MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.17 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_17MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.17 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_17MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.17 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_17MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_18MODE`" ]
    pub enum P0_18MODEW {
        # [ doc = "Pull-up. P0.18 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.18 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.18 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.18 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_18MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_18MODEW::PULL_UP => 0,
                P0_18MODEW::REPEATER => 1,
                P0_18MODEW::DISABLED => 2,
                P0_18MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_18MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_18MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_18MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.18 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_18MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.18 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_18MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.18 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_18MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.18 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_18MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_19MODE`" ]
    pub enum P0_19MODEW {
        # [ doc = "Pull-up. P0.19 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.19 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.19 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.19 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_19MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_19MODEW::PULL_UP => 0,
                P0_19MODEW::REPEATER => 1,
                P0_19MODEW::DISABLED => 2,
                P0_19MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_19MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_19MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_19MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.19 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_19MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.19 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_19MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.19 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_19MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.19 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_19MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_20MODE`" ]
    pub enum P0_20MODEW {
        # [ doc = "Pull-up. P0.20 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.20 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.20 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.20 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_20MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_20MODEW::PULL_UP => 0,
                P0_20MODEW::REPEATER => 1,
                P0_20MODEW::DISABLED => 2,
                P0_20MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_20MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_20MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_20MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.20 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_20MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.20 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_20MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.20 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_20MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.20 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_20MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_21MODE`" ]
    pub enum P0_21MODEW {
        # [ doc = "Pull-up. P0.21 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.21 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.21 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.21 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_21MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_21MODEW::PULL_UP => 0,
                P0_21MODEW::REPEATER => 1,
                P0_21MODEW::DISABLED => 2,
                P0_21MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_21MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_21MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_21MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.21 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_21MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.21 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_21MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.21 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_21MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.21 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_21MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_22MODE`" ]
    pub enum P0_22MODEW {
        # [ doc = "Pull-up. P0.22 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.22 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.22 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.22 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_22MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_22MODEW::PULL_UP => 0,
                P0_22MODEW::REPEATER => 1,
                P0_22MODEW::DISABLED => 2,
                P0_22MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_22MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_22MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_22MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.22 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_22MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.22 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_22MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.22 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_22MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.22 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_22MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_23MODE`" ]
    pub enum P0_23MODEW {
        # [ doc = "Pull-up. P0.23 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.23 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.23 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.23 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_23MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_23MODEW::PULL_UP => 0,
                P0_23MODEW::REPEATER => 1,
                P0_23MODEW::DISABLED => 2,
                P0_23MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_23MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_23MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_23MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.23 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_23MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.23 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_23MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.23 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_23MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.23 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_23MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_24MODE`" ]
    pub enum P0_24MODEW {
        # [ doc = "Pull-up. P0.24 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.24 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.24 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.24 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_24MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_24MODEW::PULL_UP => 0,
                P0_24MODEW::REPEATER => 1,
                P0_24MODEW::DISABLED => 2,
                P0_24MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_24MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_24MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_24MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.24 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_24MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.24 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_24MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.24 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_24MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.24 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_24MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_25MODE`" ]
    pub enum P0_25MODEW {
        # [ doc = "Pull-up. P0.25 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.25 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.25 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.25 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_25MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_25MODEW::PULL_UP => 0,
                P0_25MODEW::REPEATER => 1,
                P0_25MODEW::DISABLED => 2,
                P0_25MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_25MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_25MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_25MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.25 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_25MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.25 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_25MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.25 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_25MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.25 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_25MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_26MODE`" ]
    pub enum P0_26MODEW {
        # [ doc = "Pull-up. P0.26 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P0.26 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P0.26 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P0.26 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P0_26MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P0_26MODEW::PULL_UP => 0,
                P0_26MODEW::REPEATER => 1,
                P0_26MODEW::DISABLED => 2,
                P0_26MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_26MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_26MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_26MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P0.26 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P0_26MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P0.26 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P0_26MODEW::REPEATER)
        }
        # [ doc = "Disabled. P0.26 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P0_26MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P0.26 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P0_26MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Port 1 pin 16 control." ]
        # [ inline ( always ) ]
        pub fn p0_16mode(&self) -> P0_16MODER {
            P0_16MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 2:3 - Port 1 pin 17 control." ]
        # [ inline ( always ) ]
        pub fn p0_17mode(&self) -> P0_17MODER {
            P0_17MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 2;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 4:5 - Port 1 pin 18 control." ]
        # [ inline ( always ) ]
        pub fn p0_18mode(&self) -> P0_18MODER {
            P0_18MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 4;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 6:7 - Port 1 pin 19 control." ]
        # [ inline ( always ) ]
        pub fn p0_19mode(&self) -> P0_19MODER {
            P0_19MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 6;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 8:9 - Port 1 pin 20 control." ]
        # [ inline ( always ) ]
        pub fn p0_20mode(&self) -> P0_20MODER {
            P0_20MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 8;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 10:11 - Port 1 pin 21 control." ]
        # [ inline ( always ) ]
        pub fn p0_21mode(&self) -> P0_21MODER {
            P0_21MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 10;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 12:13 - Port 1 pin 22 control." ]
        # [ inline ( always ) ]
        pub fn p0_22mode(&self) -> P0_22MODER {
            P0_22MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 12;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 14:15 - Port 1 pin 23 control." ]
        # [ inline ( always ) ]
        pub fn p0_23mode(&self) -> P0_23MODER {
            P0_23MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 14;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 16:17 - Port 1 pin 24 control." ]
        # [ inline ( always ) ]
        pub fn p0_24mode(&self) -> P0_24MODER {
            P0_24MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 16;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 18:19 - Port 1 pin 25 control." ]
        # [ inline ( always ) ]
        pub fn p0_25mode(&self) -> P0_25MODER {
            P0_25MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 18;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 20:21 - Port 1 pin 26 control." ]
        # [ inline ( always ) ]
        pub fn p0_26mode(&self) -> P0_26MODER {
            P0_26MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 20;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Port 1 pin 16 control." ]
        # [ inline ( always ) ]
        pub fn p0_16mode(&mut self) -> _P0_16MODEW {
            _P0_16MODEW { w: self }
        }
        # [ doc = "Bits 2:3 - Port 1 pin 17 control." ]
        # [ inline ( always ) ]
        pub fn p0_17mode(&mut self) -> _P0_17MODEW {
            _P0_17MODEW { w: self }
        }
        # [ doc = "Bits 4:5 - Port 1 pin 18 control." ]
        # [ inline ( always ) ]
        pub fn p0_18mode(&mut self) -> _P0_18MODEW {
            _P0_18MODEW { w: self }
        }
        # [ doc = "Bits 6:7 - Port 1 pin 19 control." ]
        # [ inline ( always ) ]
        pub fn p0_19mode(&mut self) -> _P0_19MODEW {
            _P0_19MODEW { w: self }
        }
        # [ doc = "Bits 8:9 - Port 1 pin 20 control." ]
        # [ inline ( always ) ]
        pub fn p0_20mode(&mut self) -> _P0_20MODEW {
            _P0_20MODEW { w: self }
        }
        # [ doc = "Bits 10:11 - Port 1 pin 21 control." ]
        # [ inline ( always ) ]
        pub fn p0_21mode(&mut self) -> _P0_21MODEW {
            _P0_21MODEW { w: self }
        }
        # [ doc = "Bits 12:13 - Port 1 pin 22 control." ]
        # [ inline ( always ) ]
        pub fn p0_22mode(&mut self) -> _P0_22MODEW {
            _P0_22MODEW { w: self }
        }
        # [ doc = "Bits 14:15 - Port 1 pin 23 control." ]
        # [ inline ( always ) ]
        pub fn p0_23mode(&mut self) -> _P0_23MODEW {
            _P0_23MODEW { w: self }
        }
        # [ doc = "Bits 16:17 - Port 1 pin 24 control." ]
        # [ inline ( always ) ]
        pub fn p0_24mode(&mut self) -> _P0_24MODEW {
            _P0_24MODEW { w: self }
        }
        # [ doc = "Bits 18:19 - Port 1 pin 25 control." ]
        # [ inline ( always ) ]
        pub fn p0_25mode(&mut self) -> _P0_25MODEW {
            _P0_25MODEW { w: self }
        }
        # [ doc = "Bits 20:21 - Port 1 pin 26 control." ]
        # [ inline ( always ) ]
        pub fn p0_26mode(&mut self) -> _P0_26MODEW {
            _P0_26MODEW { w: self }
        }
    }
}
# [ doc = "Pin mode select register 2" ]
pub struct PINMODE2 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin mode select register 2" ]
pub mod pinmode2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P1_00MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_00MODER {
        # [ doc = "Pull-up. P1.0 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.0 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.0 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.0 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_00MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_00MODER::PULL_UP => 0,
                P1_00MODER::REPEATER => 1,
                P1_00MODER::DISABLED => 2,
                P1_00MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_00MODER {
            match value {
                0 => P1_00MODER::PULL_UP,
                1 => P1_00MODER::REPEATER,
                2 => P1_00MODER::DISABLED,
                3 => P1_00MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_00MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_00MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_00MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_00MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_01MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_01MODER {
        # [ doc = "Pull-up. P1.1 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.1 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.1 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.1 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_01MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_01MODER::PULL_UP => 0,
                P1_01MODER::REPEATER => 1,
                P1_01MODER::DISABLED => 2,
                P1_01MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_01MODER {
            match value {
                0 => P1_01MODER::PULL_UP,
                1 => P1_01MODER::REPEATER,
                2 => P1_01MODER::DISABLED,
                3 => P1_01MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_01MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_01MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_01MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_01MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_04MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_04MODER {
        # [ doc = "Pull-up. P1.4 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.4 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.4 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.4 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_04MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_04MODER::PULL_UP => 0,
                P1_04MODER::REPEATER => 1,
                P1_04MODER::DISABLED => 2,
                P1_04MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_04MODER {
            match value {
                0 => P1_04MODER::PULL_UP,
                1 => P1_04MODER::REPEATER,
                2 => P1_04MODER::DISABLED,
                3 => P1_04MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_04MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_04MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_04MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_04MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_08MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_08MODER {
        # [ doc = "Pull-up. P1.8 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.8 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.8 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.8 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_08MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_08MODER::PULL_UP => 0,
                P1_08MODER::REPEATER => 1,
                P1_08MODER::DISABLED => 2,
                P1_08MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_08MODER {
            match value {
                0 => P1_08MODER::PULL_UP,
                1 => P1_08MODER::REPEATER,
                2 => P1_08MODER::DISABLED,
                3 => P1_08MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_08MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_08MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_08MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_08MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_09MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_09MODER {
        # [ doc = "Pull-up. P1.9 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.9 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.9 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.9 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_09MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_09MODER::PULL_UP => 0,
                P1_09MODER::REPEATER => 1,
                P1_09MODER::DISABLED => 2,
                P1_09MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_09MODER {
            match value {
                0 => P1_09MODER::PULL_UP,
                1 => P1_09MODER::REPEATER,
                2 => P1_09MODER::DISABLED,
                3 => P1_09MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_09MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_09MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_09MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_09MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_10MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_10MODER {
        # [ doc = "Pull-up. P1.10 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.10 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.10 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.10 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_10MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_10MODER::PULL_UP => 0,
                P1_10MODER::REPEATER => 1,
                P1_10MODER::DISABLED => 2,
                P1_10MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_10MODER {
            match value {
                0 => P1_10MODER::PULL_UP,
                1 => P1_10MODER::REPEATER,
                2 => P1_10MODER::DISABLED,
                3 => P1_10MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_10MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_10MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_10MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_10MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_14MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_14MODER {
        # [ doc = "Pull-up. P1.14 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.14 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.14 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.14 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_14MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_14MODER::PULL_UP => 0,
                P1_14MODER::REPEATER => 1,
                P1_14MODER::DISABLED => 2,
                P1_14MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_14MODER {
            match value {
                0 => P1_14MODER::PULL_UP,
                1 => P1_14MODER::REPEATER,
                2 => P1_14MODER::DISABLED,
                3 => P1_14MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_14MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_14MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_14MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_14MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_15MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_15MODER {
        # [ doc = "Pull-up. P1.15 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.15 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.15 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.15 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_15MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_15MODER::PULL_UP => 0,
                P1_15MODER::REPEATER => 1,
                P1_15MODER::DISABLED => 2,
                P1_15MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_15MODER {
            match value {
                0 => P1_15MODER::PULL_UP,
                1 => P1_15MODER::REPEATER,
                2 => P1_15MODER::DISABLED,
                3 => P1_15MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_15MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_15MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_15MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_15MODER::PULL_DOWN
        }
    }
    # [ doc = "Values that can be written to the field `P1_00MODE`" ]
    pub enum P1_00MODEW {
        # [ doc = "Pull-up. P1.0 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.0 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.0 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.0 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_00MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_00MODEW::PULL_UP => 0,
                P1_00MODEW::REPEATER => 1,
                P1_00MODEW::DISABLED => 2,
                P1_00MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_00MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_00MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_00MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.0 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_00MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.0 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_00MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.0 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_00MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.0 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_00MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_01MODE`" ]
    pub enum P1_01MODEW {
        # [ doc = "Pull-up. P1.1 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.1 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.1 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.1 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_01MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_01MODEW::PULL_UP => 0,
                P1_01MODEW::REPEATER => 1,
                P1_01MODEW::DISABLED => 2,
                P1_01MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_01MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_01MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_01MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.1 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_01MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.1 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_01MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.1 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_01MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.1 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_01MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_04MODE`" ]
    pub enum P1_04MODEW {
        # [ doc = "Pull-up. P1.4 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.4 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.4 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.4 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_04MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_04MODEW::PULL_UP => 0,
                P1_04MODEW::REPEATER => 1,
                P1_04MODEW::DISABLED => 2,
                P1_04MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_04MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_04MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_04MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.4 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_04MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.4 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_04MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.4 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_04MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.4 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_04MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_08MODE`" ]
    pub enum P1_08MODEW {
        # [ doc = "Pull-up. P1.8 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.8 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.8 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.8 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_08MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_08MODEW::PULL_UP => 0,
                P1_08MODEW::REPEATER => 1,
                P1_08MODEW::DISABLED => 2,
                P1_08MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_08MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_08MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_08MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.8 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_08MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.8 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_08MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.8 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_08MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.8 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_08MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_09MODE`" ]
    pub enum P1_09MODEW {
        # [ doc = "Pull-up. P1.9 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.9 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.9 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.9 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_09MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_09MODEW::PULL_UP => 0,
                P1_09MODEW::REPEATER => 1,
                P1_09MODEW::DISABLED => 2,
                P1_09MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_09MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_09MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_09MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.9 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_09MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.9 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_09MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.9 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_09MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.9 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_09MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_10MODE`" ]
    pub enum P1_10MODEW {
        # [ doc = "Pull-up. P1.10 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.10 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.10 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.10 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_10MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_10MODEW::PULL_UP => 0,
                P1_10MODEW::REPEATER => 1,
                P1_10MODEW::DISABLED => 2,
                P1_10MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_10MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_10MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_10MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.10 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_10MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.10 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_10MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.10 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_10MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.10 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_10MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_14MODE`" ]
    pub enum P1_14MODEW {
        # [ doc = "Pull-up. P1.14 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.14 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.14 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.14 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_14MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_14MODEW::PULL_UP => 0,
                P1_14MODEW::REPEATER => 1,
                P1_14MODEW::DISABLED => 2,
                P1_14MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_14MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_14MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_14MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.14 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_14MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.14 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_14MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.14 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_14MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.14 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_14MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_15MODE`" ]
    pub enum P1_15MODEW {
        # [ doc = "Pull-up. P1.15 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.15 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.15 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.15 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_15MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_15MODEW::PULL_UP => 0,
                P1_15MODEW::REPEATER => 1,
                P1_15MODEW::DISABLED => 2,
                P1_15MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_15MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_15MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_15MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.15 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_15MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.15 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_15MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.15 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_15MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.15 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_15MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Port 1 pin 0 control." ]
        # [ inline ( always ) ]
        pub fn p1_00mode(&self) -> P1_00MODER {
            P1_00MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 2:3 - Port 1 pin 1 control." ]
        # [ inline ( always ) ]
        pub fn p1_01mode(&self) -> P1_01MODER {
            P1_01MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 2;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 8:9 - Port 1 pin 4 control." ]
        # [ inline ( always ) ]
        pub fn p1_04mode(&self) -> P1_04MODER {
            P1_04MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 8;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 16:17 - Port 1 pin 8 control." ]
        # [ inline ( always ) ]
        pub fn p1_08mode(&self) -> P1_08MODER {
            P1_08MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 16;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 18:19 - Port 1 pin 9 control." ]
        # [ inline ( always ) ]
        pub fn p1_09mode(&self) -> P1_09MODER {
            P1_09MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 18;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 20:21 - Port 1 pin 10 control." ]
        # [ inline ( always ) ]
        pub fn p1_10mode(&self) -> P1_10MODER {
            P1_10MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 20;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 28:29 - Port 1 pin 14 control." ]
        # [ inline ( always ) ]
        pub fn p1_14mode(&self) -> P1_14MODER {
            P1_14MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 28;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 30:31 - Port 1 pin 15 control." ]
        # [ inline ( always ) ]
        pub fn p1_15mode(&self) -> P1_15MODER {
            P1_15MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 30;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Port 1 pin 0 control." ]
        # [ inline ( always ) ]
        pub fn p1_00mode(&mut self) -> _P1_00MODEW {
            _P1_00MODEW { w: self }
        }
        # [ doc = "Bits 2:3 - Port 1 pin 1 control." ]
        # [ inline ( always ) ]
        pub fn p1_01mode(&mut self) -> _P1_01MODEW {
            _P1_01MODEW { w: self }
        }
        # [ doc = "Bits 8:9 - Port 1 pin 4 control." ]
        # [ inline ( always ) ]
        pub fn p1_04mode(&mut self) -> _P1_04MODEW {
            _P1_04MODEW { w: self }
        }
        # [ doc = "Bits 16:17 - Port 1 pin 8 control." ]
        # [ inline ( always ) ]
        pub fn p1_08mode(&mut self) -> _P1_08MODEW {
            _P1_08MODEW { w: self }
        }
        # [ doc = "Bits 18:19 - Port 1 pin 9 control." ]
        # [ inline ( always ) ]
        pub fn p1_09mode(&mut self) -> _P1_09MODEW {
            _P1_09MODEW { w: self }
        }
        # [ doc = "Bits 20:21 - Port 1 pin 10 control." ]
        # [ inline ( always ) ]
        pub fn p1_10mode(&mut self) -> _P1_10MODEW {
            _P1_10MODEW { w: self }
        }
        # [ doc = "Bits 28:29 - Port 1 pin 14 control." ]
        # [ inline ( always ) ]
        pub fn p1_14mode(&mut self) -> _P1_14MODEW {
            _P1_14MODEW { w: self }
        }
        # [ doc = "Bits 30:31 - Port 1 pin 15 control." ]
        # [ inline ( always ) ]
        pub fn p1_15mode(&mut self) -> _P1_15MODEW {
            _P1_15MODEW { w: self }
        }
    }
}
# [ doc = "Pin mode select register 3." ]
pub struct PINMODE3 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin mode select register 3." ]
pub mod pinmode3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P1_16MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_16MODER {
        # [ doc = "Pull-up. P1.16 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.16 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.16 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.16 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_16MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_16MODER::PULL_UP => 0,
                P1_16MODER::REPEATER => 1,
                P1_16MODER::DISABLED => 2,
                P1_16MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_16MODER {
            match value {
                0 => P1_16MODER::PULL_UP,
                1 => P1_16MODER::REPEATER,
                2 => P1_16MODER::DISABLED,
                3 => P1_16MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_16MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_16MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_16MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_16MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_17MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_17MODER {
        # [ doc = "Pull-up. P1.17 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.17 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.17 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.17 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_17MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_17MODER::PULL_UP => 0,
                P1_17MODER::REPEATER => 1,
                P1_17MODER::DISABLED => 2,
                P1_17MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_17MODER {
            match value {
                0 => P1_17MODER::PULL_UP,
                1 => P1_17MODER::REPEATER,
                2 => P1_17MODER::DISABLED,
                3 => P1_17MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_17MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_17MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_17MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_17MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_18MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_18MODER {
        # [ doc = "Pull-up. P1.18 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.18 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.18 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.18 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_18MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_18MODER::PULL_UP => 0,
                P1_18MODER::REPEATER => 1,
                P1_18MODER::DISABLED => 2,
                P1_18MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_18MODER {
            match value {
                0 => P1_18MODER::PULL_UP,
                1 => P1_18MODER::REPEATER,
                2 => P1_18MODER::DISABLED,
                3 => P1_18MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_18MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_18MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_18MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_18MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_19MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_19MODER {
        # [ doc = "Pull-up. P1.19 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.19 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.19 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.19 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_19MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_19MODER::PULL_UP => 0,
                P1_19MODER::REPEATER => 1,
                P1_19MODER::DISABLED => 2,
                P1_19MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_19MODER {
            match value {
                0 => P1_19MODER::PULL_UP,
                1 => P1_19MODER::REPEATER,
                2 => P1_19MODER::DISABLED,
                3 => P1_19MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_19MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_19MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_19MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_19MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_20MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_20MODER {
        # [ doc = "Pull-up. P1.20 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.20 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.20 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.20 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_20MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_20MODER::PULL_UP => 0,
                P1_20MODER::REPEATER => 1,
                P1_20MODER::DISABLED => 2,
                P1_20MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_20MODER {
            match value {
                0 => P1_20MODER::PULL_UP,
                1 => P1_20MODER::REPEATER,
                2 => P1_20MODER::DISABLED,
                3 => P1_20MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_20MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_20MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_20MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_20MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_21MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_21MODER {
        # [ doc = "Pull-up. P1.21 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.21 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.21 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.21 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_21MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_21MODER::PULL_UP => 0,
                P1_21MODER::REPEATER => 1,
                P1_21MODER::DISABLED => 2,
                P1_21MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_21MODER {
            match value {
                0 => P1_21MODER::PULL_UP,
                1 => P1_21MODER::REPEATER,
                2 => P1_21MODER::DISABLED,
                3 => P1_21MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_21MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_21MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_21MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_21MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_22MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_22MODER {
        # [ doc = "Pull-up. P1.22 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.22 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.22 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.22 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_22MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_22MODER::PULL_UP => 0,
                P1_22MODER::REPEATER => 1,
                P1_22MODER::DISABLED => 2,
                P1_22MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_22MODER {
            match value {
                0 => P1_22MODER::PULL_UP,
                1 => P1_22MODER::REPEATER,
                2 => P1_22MODER::DISABLED,
                3 => P1_22MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_22MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_22MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_22MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_22MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_23MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_23MODER {
        # [ doc = "Pull-up. P1.23 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.23 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.23 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.23 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_23MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_23MODER::PULL_UP => 0,
                P1_23MODER::REPEATER => 1,
                P1_23MODER::DISABLED => 2,
                P1_23MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_23MODER {
            match value {
                0 => P1_23MODER::PULL_UP,
                1 => P1_23MODER::REPEATER,
                2 => P1_23MODER::DISABLED,
                3 => P1_23MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_23MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_23MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_23MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_23MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_24MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_24MODER {
        # [ doc = "Pull-up. P1.24 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.24 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.24 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.24 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_24MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_24MODER::PULL_UP => 0,
                P1_24MODER::REPEATER => 1,
                P1_24MODER::DISABLED => 2,
                P1_24MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_24MODER {
            match value {
                0 => P1_24MODER::PULL_UP,
                1 => P1_24MODER::REPEATER,
                2 => P1_24MODER::DISABLED,
                3 => P1_24MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_24MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_24MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_24MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_24MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_25MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_25MODER {
        # [ doc = "Pull-up. P1.25 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.25 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.25 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.25 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_25MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_25MODER::PULL_UP => 0,
                P1_25MODER::REPEATER => 1,
                P1_25MODER::DISABLED => 2,
                P1_25MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_25MODER {
            match value {
                0 => P1_25MODER::PULL_UP,
                1 => P1_25MODER::REPEATER,
                2 => P1_25MODER::DISABLED,
                3 => P1_25MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_25MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_25MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_25MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_25MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_26MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_26MODER {
        # [ doc = "Pull-up. P1.26 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.26 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.26 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.26 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_26MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_26MODER::PULL_UP => 0,
                P1_26MODER::REPEATER => 1,
                P1_26MODER::DISABLED => 2,
                P1_26MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_26MODER {
            match value {
                0 => P1_26MODER::PULL_UP,
                1 => P1_26MODER::REPEATER,
                2 => P1_26MODER::DISABLED,
                3 => P1_26MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_26MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_26MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_26MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_26MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_27MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_27MODER {
        # [ doc = "Pull-up. P1.27 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.27 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.27 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.27 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_27MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_27MODER::PULL_UP => 0,
                P1_27MODER::REPEATER => 1,
                P1_27MODER::DISABLED => 2,
                P1_27MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_27MODER {
            match value {
                0 => P1_27MODER::PULL_UP,
                1 => P1_27MODER::REPEATER,
                2 => P1_27MODER::DISABLED,
                3 => P1_27MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_27MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_27MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_27MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_27MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_28MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_28MODER {
        # [ doc = "Pull-up. P1.28 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.28 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.28 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.28 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_28MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_28MODER::PULL_UP => 0,
                P1_28MODER::REPEATER => 1,
                P1_28MODER::DISABLED => 2,
                P1_28MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_28MODER {
            match value {
                0 => P1_28MODER::PULL_UP,
                1 => P1_28MODER::REPEATER,
                2 => P1_28MODER::DISABLED,
                3 => P1_28MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_28MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_28MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_28MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_28MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_29MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_29MODER {
        # [ doc = "Pull-up. P1.29 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.29 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.29 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.29 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_29MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_29MODER::PULL_UP => 0,
                P1_29MODER::REPEATER => 1,
                P1_29MODER::DISABLED => 2,
                P1_29MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_29MODER {
            match value {
                0 => P1_29MODER::PULL_UP,
                1 => P1_29MODER::REPEATER,
                2 => P1_29MODER::DISABLED,
                3 => P1_29MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_29MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_29MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_29MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_29MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_30MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_30MODER {
        # [ doc = "Pull-up. P1.30 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.30 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.30 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.30 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_30MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_30MODER::PULL_UP => 0,
                P1_30MODER::REPEATER => 1,
                P1_30MODER::DISABLED => 2,
                P1_30MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_30MODER {
            match value {
                0 => P1_30MODER::PULL_UP,
                1 => P1_30MODER::REPEATER,
                2 => P1_30MODER::DISABLED,
                3 => P1_30MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_30MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_30MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_30MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_30MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P1_31MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_31MODER {
        # [ doc = "Pull-up. P1.31 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.31 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.31 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.31 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_31MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P1_31MODER::PULL_UP => 0,
                P1_31MODER::REPEATER => 1,
                P1_31MODER::DISABLED => 2,
                P1_31MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P1_31MODER {
            match value {
                0 => P1_31MODER::PULL_UP,
                1 => P1_31MODER::REPEATER,
                2 => P1_31MODER::DISABLED,
                3 => P1_31MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P1_31MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P1_31MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P1_31MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P1_31MODER::PULL_DOWN
        }
    }
    # [ doc = "Values that can be written to the field `P1_16MODE`" ]
    pub enum P1_16MODEW {
        # [ doc = "Pull-up. P1.16 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.16 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.16 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.16 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_16MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_16MODEW::PULL_UP => 0,
                P1_16MODEW::REPEATER => 1,
                P1_16MODEW::DISABLED => 2,
                P1_16MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_16MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_16MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_16MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.16 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_16MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.16 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_16MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.16 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_16MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.16 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_16MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_17MODE`" ]
    pub enum P1_17MODEW {
        # [ doc = "Pull-up. P1.17 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.17 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.17 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.17 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_17MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_17MODEW::PULL_UP => 0,
                P1_17MODEW::REPEATER => 1,
                P1_17MODEW::DISABLED => 2,
                P1_17MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_17MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_17MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_17MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.17 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_17MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.17 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_17MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.17 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_17MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.17 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_17MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_18MODE`" ]
    pub enum P1_18MODEW {
        # [ doc = "Pull-up. P1.18 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.18 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.18 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.18 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_18MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_18MODEW::PULL_UP => 0,
                P1_18MODEW::REPEATER => 1,
                P1_18MODEW::DISABLED => 2,
                P1_18MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_18MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_18MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_18MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.18 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_18MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.18 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_18MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.18 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_18MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.18 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_18MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_19MODE`" ]
    pub enum P1_19MODEW {
        # [ doc = "Pull-up. P1.19 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.19 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.19 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.19 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_19MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_19MODEW::PULL_UP => 0,
                P1_19MODEW::REPEATER => 1,
                P1_19MODEW::DISABLED => 2,
                P1_19MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_19MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_19MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_19MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.19 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_19MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.19 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_19MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.19 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_19MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.19 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_19MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_20MODE`" ]
    pub enum P1_20MODEW {
        # [ doc = "Pull-up. P1.20 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.20 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.20 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.20 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_20MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_20MODEW::PULL_UP => 0,
                P1_20MODEW::REPEATER => 1,
                P1_20MODEW::DISABLED => 2,
                P1_20MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_20MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_20MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_20MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.20 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_20MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.20 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_20MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.20 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_20MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.20 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_20MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_21MODE`" ]
    pub enum P1_21MODEW {
        # [ doc = "Pull-up. P1.21 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.21 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.21 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.21 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_21MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_21MODEW::PULL_UP => 0,
                P1_21MODEW::REPEATER => 1,
                P1_21MODEW::DISABLED => 2,
                P1_21MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_21MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_21MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_21MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.21 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_21MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.21 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_21MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.21 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_21MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.21 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_21MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_22MODE`" ]
    pub enum P1_22MODEW {
        # [ doc = "Pull-up. P1.22 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.22 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.22 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.22 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_22MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_22MODEW::PULL_UP => 0,
                P1_22MODEW::REPEATER => 1,
                P1_22MODEW::DISABLED => 2,
                P1_22MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_22MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_22MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_22MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.22 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_22MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.22 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_22MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.22 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_22MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.22 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_22MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_23MODE`" ]
    pub enum P1_23MODEW {
        # [ doc = "Pull-up. P1.23 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.23 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.23 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.23 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_23MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_23MODEW::PULL_UP => 0,
                P1_23MODEW::REPEATER => 1,
                P1_23MODEW::DISABLED => 2,
                P1_23MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_23MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_23MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_23MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.23 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_23MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.23 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_23MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.23 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_23MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.23 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_23MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_24MODE`" ]
    pub enum P1_24MODEW {
        # [ doc = "Pull-up. P1.24 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.24 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.24 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.24 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_24MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_24MODEW::PULL_UP => 0,
                P1_24MODEW::REPEATER => 1,
                P1_24MODEW::DISABLED => 2,
                P1_24MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_24MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_24MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_24MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.24 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_24MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.24 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_24MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.24 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_24MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.24 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_24MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_25MODE`" ]
    pub enum P1_25MODEW {
        # [ doc = "Pull-up. P1.25 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.25 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.25 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.25 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_25MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_25MODEW::PULL_UP => 0,
                P1_25MODEW::REPEATER => 1,
                P1_25MODEW::DISABLED => 2,
                P1_25MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_25MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_25MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_25MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.25 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_25MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.25 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_25MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.25 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_25MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.25 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_25MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_26MODE`" ]
    pub enum P1_26MODEW {
        # [ doc = "Pull-up. P1.26 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.26 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.26 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.26 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_26MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_26MODEW::PULL_UP => 0,
                P1_26MODEW::REPEATER => 1,
                P1_26MODEW::DISABLED => 2,
                P1_26MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_26MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_26MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_26MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.26 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_26MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.26 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_26MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.26 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_26MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.26 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_26MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_27MODE`" ]
    pub enum P1_27MODEW {
        # [ doc = "Pull-up. P1.27 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.27 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.27 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.27 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_27MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_27MODEW::PULL_UP => 0,
                P1_27MODEW::REPEATER => 1,
                P1_27MODEW::DISABLED => 2,
                P1_27MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_27MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_27MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_27MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.27 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_27MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.27 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_27MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.27 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_27MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.27 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_27MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_28MODE`" ]
    pub enum P1_28MODEW {
        # [ doc = "Pull-up. P1.28 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.28 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.28 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.28 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_28MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_28MODEW::PULL_UP => 0,
                P1_28MODEW::REPEATER => 1,
                P1_28MODEW::DISABLED => 2,
                P1_28MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_28MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_28MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_28MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.28 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_28MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.28 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_28MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.28 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_28MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.28 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_28MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_29MODE`" ]
    pub enum P1_29MODEW {
        # [ doc = "Pull-up. P1.29 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.29 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.29 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.29 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_29MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_29MODEW::PULL_UP => 0,
                P1_29MODEW::REPEATER => 1,
                P1_29MODEW::DISABLED => 2,
                P1_29MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_29MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_29MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_29MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.29 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_29MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.29 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_29MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.29 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_29MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.29 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_29MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_30MODE`" ]
    pub enum P1_30MODEW {
        # [ doc = "Pull-up. P1.30 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.30 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.30 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.30 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_30MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_30MODEW::PULL_UP => 0,
                P1_30MODEW::REPEATER => 1,
                P1_30MODEW::DISABLED => 2,
                P1_30MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_30MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_30MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_30MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.30 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_30MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.30 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_30MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.30 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_30MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.30 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_30MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_31MODE`" ]
    pub enum P1_31MODEW {
        # [ doc = "Pull-up. P1.31 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P1.31 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P1.31 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P1.31 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P1_31MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P1_31MODEW::PULL_UP => 0,
                P1_31MODEW::REPEATER => 1,
                P1_31MODEW::DISABLED => 2,
                P1_31MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_31MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_31MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_31MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P1.31 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P1_31MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P1.31 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P1_31MODEW::REPEATER)
        }
        # [ doc = "Disabled. P1.31 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P1_31MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P1.31 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P1_31MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Port 1 pin 16 control." ]
        # [ inline ( always ) ]
        pub fn p1_16mode(&self) -> P1_16MODER {
            P1_16MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 2:3 - Port 1 pin 17 control." ]
        # [ inline ( always ) ]
        pub fn p1_17mode(&self) -> P1_17MODER {
            P1_17MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 2;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 4:5 - Port 1 pin 18 control." ]
        # [ inline ( always ) ]
        pub fn p1_18mode(&self) -> P1_18MODER {
            P1_18MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 4;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 6:7 - Port 1 pin 19 control." ]
        # [ inline ( always ) ]
        pub fn p1_19mode(&self) -> P1_19MODER {
            P1_19MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 6;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 8:9 - Port 1 pin 20 control." ]
        # [ inline ( always ) ]
        pub fn p1_20mode(&self) -> P1_20MODER {
            P1_20MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 8;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 10:11 - Port 1 pin 21 control." ]
        # [ inline ( always ) ]
        pub fn p1_21mode(&self) -> P1_21MODER {
            P1_21MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 10;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 12:13 - Port 1 pin 22 control." ]
        # [ inline ( always ) ]
        pub fn p1_22mode(&self) -> P1_22MODER {
            P1_22MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 12;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 14:15 - Port 1 pin 23 control." ]
        # [ inline ( always ) ]
        pub fn p1_23mode(&self) -> P1_23MODER {
            P1_23MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 14;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 16:17 - Port 1 pin 24 control." ]
        # [ inline ( always ) ]
        pub fn p1_24mode(&self) -> P1_24MODER {
            P1_24MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 16;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 18:19 - Port 1 pin 25 control." ]
        # [ inline ( always ) ]
        pub fn p1_25mode(&self) -> P1_25MODER {
            P1_25MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 18;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 20:21 - Port 1 pin 26 control." ]
        # [ inline ( always ) ]
        pub fn p1_26mode(&self) -> P1_26MODER {
            P1_26MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 20;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 22:23 - Port 1 pin 27 control." ]
        # [ inline ( always ) ]
        pub fn p1_27mode(&self) -> P1_27MODER {
            P1_27MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 22;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 24:25 - Port 1 pin 28 control." ]
        # [ inline ( always ) ]
        pub fn p1_28mode(&self) -> P1_28MODER {
            P1_28MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 24;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 26:27 - Port 1 pin 29 control." ]
        # [ inline ( always ) ]
        pub fn p1_29mode(&self) -> P1_29MODER {
            P1_29MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 26;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 28:29 - Port 1 pin 30 control." ]
        # [ inline ( always ) ]
        pub fn p1_30mode(&self) -> P1_30MODER {
            P1_30MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 28;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 30:31 - Port 1 pin 31 control." ]
        # [ inline ( always ) ]
        pub fn p1_31mode(&self) -> P1_31MODER {
            P1_31MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 30;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Port 1 pin 16 control." ]
        # [ inline ( always ) ]
        pub fn p1_16mode(&mut self) -> _P1_16MODEW {
            _P1_16MODEW { w: self }
        }
        # [ doc = "Bits 2:3 - Port 1 pin 17 control." ]
        # [ inline ( always ) ]
        pub fn p1_17mode(&mut self) -> _P1_17MODEW {
            _P1_17MODEW { w: self }
        }
        # [ doc = "Bits 4:5 - Port 1 pin 18 control." ]
        # [ inline ( always ) ]
        pub fn p1_18mode(&mut self) -> _P1_18MODEW {
            _P1_18MODEW { w: self }
        }
        # [ doc = "Bits 6:7 - Port 1 pin 19 control." ]
        # [ inline ( always ) ]
        pub fn p1_19mode(&mut self) -> _P1_19MODEW {
            _P1_19MODEW { w: self }
        }
        # [ doc = "Bits 8:9 - Port 1 pin 20 control." ]
        # [ inline ( always ) ]
        pub fn p1_20mode(&mut self) -> _P1_20MODEW {
            _P1_20MODEW { w: self }
        }
        # [ doc = "Bits 10:11 - Port 1 pin 21 control." ]
        # [ inline ( always ) ]
        pub fn p1_21mode(&mut self) -> _P1_21MODEW {
            _P1_21MODEW { w: self }
        }
        # [ doc = "Bits 12:13 - Port 1 pin 22 control." ]
        # [ inline ( always ) ]
        pub fn p1_22mode(&mut self) -> _P1_22MODEW {
            _P1_22MODEW { w: self }
        }
        # [ doc = "Bits 14:15 - Port 1 pin 23 control." ]
        # [ inline ( always ) ]
        pub fn p1_23mode(&mut self) -> _P1_23MODEW {
            _P1_23MODEW { w: self }
        }
        # [ doc = "Bits 16:17 - Port 1 pin 24 control." ]
        # [ inline ( always ) ]
        pub fn p1_24mode(&mut self) -> _P1_24MODEW {
            _P1_24MODEW { w: self }
        }
        # [ doc = "Bits 18:19 - Port 1 pin 25 control." ]
        # [ inline ( always ) ]
        pub fn p1_25mode(&mut self) -> _P1_25MODEW {
            _P1_25MODEW { w: self }
        }
        # [ doc = "Bits 20:21 - Port 1 pin 26 control." ]
        # [ inline ( always ) ]
        pub fn p1_26mode(&mut self) -> _P1_26MODEW {
            _P1_26MODEW { w: self }
        }
        # [ doc = "Bits 22:23 - Port 1 pin 27 control." ]
        # [ inline ( always ) ]
        pub fn p1_27mode(&mut self) -> _P1_27MODEW {
            _P1_27MODEW { w: self }
        }
        # [ doc = "Bits 24:25 - Port 1 pin 28 control." ]
        # [ inline ( always ) ]
        pub fn p1_28mode(&mut self) -> _P1_28MODEW {
            _P1_28MODEW { w: self }
        }
        # [ doc = "Bits 26:27 - Port 1 pin 29 control." ]
        # [ inline ( always ) ]
        pub fn p1_29mode(&mut self) -> _P1_29MODEW {
            _P1_29MODEW { w: self }
        }
        # [ doc = "Bits 28:29 - Port 1 pin 30 control." ]
        # [ inline ( always ) ]
        pub fn p1_30mode(&mut self) -> _P1_30MODEW {
            _P1_30MODEW { w: self }
        }
        # [ doc = "Bits 30:31 - Port 1 pin 31 control." ]
        # [ inline ( always ) ]
        pub fn p1_31mode(&mut self) -> _P1_31MODEW {
            _P1_31MODEW { w: self }
        }
    }
}
# [ doc = "Pin mode select register 4" ]
pub struct PINMODE4 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin mode select register 4" ]
pub mod pinmode4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P2_00MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_00MODER {
        # [ doc = "Pull-up. P2.0 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.0 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.0 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.0 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_00MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_00MODER::PULL_UP => 0,
                P2_00MODER::REPEATER => 1,
                P2_00MODER::DISABLED => 2,
                P2_00MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_00MODER {
            match value {
                0 => P2_00MODER::PULL_UP,
                1 => P2_00MODER::REPEATER,
                2 => P2_00MODER::DISABLED,
                3 => P2_00MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_00MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_00MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_00MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_00MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_01MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_01MODER {
        # [ doc = "Pull-up. P2.1 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.1 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.1 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.1 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_01MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_01MODER::PULL_UP => 0,
                P2_01MODER::REPEATER => 1,
                P2_01MODER::DISABLED => 2,
                P2_01MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_01MODER {
            match value {
                0 => P2_01MODER::PULL_UP,
                1 => P2_01MODER::REPEATER,
                2 => P2_01MODER::DISABLED,
                3 => P2_01MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_01MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_01MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_01MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_01MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_02MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_02MODER {
        # [ doc = "Pull-up. P2.2 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.2 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.2 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.2 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_02MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_02MODER::PULL_UP => 0,
                P2_02MODER::REPEATER => 1,
                P2_02MODER::DISABLED => 2,
                P2_02MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_02MODER {
            match value {
                0 => P2_02MODER::PULL_UP,
                1 => P2_02MODER::REPEATER,
                2 => P2_02MODER::DISABLED,
                3 => P2_02MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_02MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_02MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_02MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_02MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_03MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_03MODER {
        # [ doc = "Pull-up. P2.3 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.3 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.3 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.3 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_03MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_03MODER::PULL_UP => 0,
                P2_03MODER::REPEATER => 1,
                P2_03MODER::DISABLED => 2,
                P2_03MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_03MODER {
            match value {
                0 => P2_03MODER::PULL_UP,
                1 => P2_03MODER::REPEATER,
                2 => P2_03MODER::DISABLED,
                3 => P2_03MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_03MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_03MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_03MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_03MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_04MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_04MODER {
        # [ doc = "Pull-up. P2.4 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.4 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.4 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.4 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_04MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_04MODER::PULL_UP => 0,
                P2_04MODER::REPEATER => 1,
                P2_04MODER::DISABLED => 2,
                P2_04MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_04MODER {
            match value {
                0 => P2_04MODER::PULL_UP,
                1 => P2_04MODER::REPEATER,
                2 => P2_04MODER::DISABLED,
                3 => P2_04MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_04MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_04MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_04MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_04MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_05MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_05MODER {
        # [ doc = "Pull-up. P2.5 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.5 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.5 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.5 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_05MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_05MODER::PULL_UP => 0,
                P2_05MODER::REPEATER => 1,
                P2_05MODER::DISABLED => 2,
                P2_05MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_05MODER {
            match value {
                0 => P2_05MODER::PULL_UP,
                1 => P2_05MODER::REPEATER,
                2 => P2_05MODER::DISABLED,
                3 => P2_05MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_05MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_05MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_05MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_05MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_06MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_06MODER {
        # [ doc = "Pull-up. P2.6 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.6 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.6 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.6 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_06MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_06MODER::PULL_UP => 0,
                P2_06MODER::REPEATER => 1,
                P2_06MODER::DISABLED => 2,
                P2_06MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_06MODER {
            match value {
                0 => P2_06MODER::PULL_UP,
                1 => P2_06MODER::REPEATER,
                2 => P2_06MODER::DISABLED,
                3 => P2_06MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_06MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_06MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_06MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_06MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_07MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_07MODER {
        # [ doc = "Pull-up. P2.7 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.7 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.7 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.7 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_07MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_07MODER::PULL_UP => 0,
                P2_07MODER::REPEATER => 1,
                P2_07MODER::DISABLED => 2,
                P2_07MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_07MODER {
            match value {
                0 => P2_07MODER::PULL_UP,
                1 => P2_07MODER::REPEATER,
                2 => P2_07MODER::DISABLED,
                3 => P2_07MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_07MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_07MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_07MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_07MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_08MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_08MODER {
        # [ doc = "Pull-up. P2.8 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.8 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.8 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.8 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_08MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_08MODER::PULL_UP => 0,
                P2_08MODER::REPEATER => 1,
                P2_08MODER::DISABLED => 2,
                P2_08MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_08MODER {
            match value {
                0 => P2_08MODER::PULL_UP,
                1 => P2_08MODER::REPEATER,
                2 => P2_08MODER::DISABLED,
                3 => P2_08MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_08MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_08MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_08MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_08MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_09MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_09MODER {
        # [ doc = "Pull-up. P2.9 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.9 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.9 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.9 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_09MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_09MODER::PULL_UP => 0,
                P2_09MODER::REPEATER => 1,
                P2_09MODER::DISABLED => 2,
                P2_09MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_09MODER {
            match value {
                0 => P2_09MODER::PULL_UP,
                1 => P2_09MODER::REPEATER,
                2 => P2_09MODER::DISABLED,
                3 => P2_09MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_09MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_09MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_09MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_09MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_10MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_10MODER {
        # [ doc = "Pull-up. P2.10 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.10 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.10 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.10 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_10MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_10MODER::PULL_UP => 0,
                P2_10MODER::REPEATER => 1,
                P2_10MODER::DISABLED => 2,
                P2_10MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_10MODER {
            match value {
                0 => P2_10MODER::PULL_UP,
                1 => P2_10MODER::REPEATER,
                2 => P2_10MODER::DISABLED,
                3 => P2_10MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_10MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_10MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_10MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_10MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_11MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_11MODER {
        # [ doc = "Pull-up. P2.11 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.11 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.11 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.11 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_11MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_11MODER::PULL_UP => 0,
                P2_11MODER::REPEATER => 1,
                P2_11MODER::DISABLED => 2,
                P2_11MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_11MODER {
            match value {
                0 => P2_11MODER::PULL_UP,
                1 => P2_11MODER::REPEATER,
                2 => P2_11MODER::DISABLED,
                3 => P2_11MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_11MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_11MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_11MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_11MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_12MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_12MODER {
        # [ doc = "Pull-up. P2.12 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.12 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.12 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.12 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_12MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_12MODER::PULL_UP => 0,
                P2_12MODER::REPEATER => 1,
                P2_12MODER::DISABLED => 2,
                P2_12MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_12MODER {
            match value {
                0 => P2_12MODER::PULL_UP,
                1 => P2_12MODER::REPEATER,
                2 => P2_12MODER::DISABLED,
                3 => P2_12MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_12MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_12MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_12MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_12MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P2_13MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_13MODER {
        # [ doc = "Pull-up. P2.13 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.13 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.13 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.13 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_13MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P2_13MODER::PULL_UP => 0,
                P2_13MODER::REPEATER => 1,
                P2_13MODER::DISABLED => 2,
                P2_13MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P2_13MODER {
            match value {
                0 => P2_13MODER::PULL_UP,
                1 => P2_13MODER::REPEATER,
                2 => P2_13MODER::DISABLED,
                3 => P2_13MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P2_13MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P2_13MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P2_13MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P2_13MODER::PULL_DOWN
        }
    }
    # [ doc = "Values that can be written to the field `P2_00MODE`" ]
    pub enum P2_00MODEW {
        # [ doc = "Pull-up. P2.0 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.0 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.0 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.0 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_00MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_00MODEW::PULL_UP => 0,
                P2_00MODEW::REPEATER => 1,
                P2_00MODEW::DISABLED => 2,
                P2_00MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_00MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_00MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_00MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.0 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_00MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.0 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_00MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.0 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_00MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.0 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_00MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_01MODE`" ]
    pub enum P2_01MODEW {
        # [ doc = "Pull-up. P2.1 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.1 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.1 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.1 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_01MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_01MODEW::PULL_UP => 0,
                P2_01MODEW::REPEATER => 1,
                P2_01MODEW::DISABLED => 2,
                P2_01MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_01MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_01MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_01MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.1 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_01MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.1 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_01MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.1 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_01MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.1 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_01MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_02MODE`" ]
    pub enum P2_02MODEW {
        # [ doc = "Pull-up. P2.2 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.2 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.2 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.2 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_02MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_02MODEW::PULL_UP => 0,
                P2_02MODEW::REPEATER => 1,
                P2_02MODEW::DISABLED => 2,
                P2_02MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_02MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_02MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_02MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.2 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_02MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.2 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_02MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.2 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_02MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.2 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_02MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_03MODE`" ]
    pub enum P2_03MODEW {
        # [ doc = "Pull-up. P2.3 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.3 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.3 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.3 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_03MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_03MODEW::PULL_UP => 0,
                P2_03MODEW::REPEATER => 1,
                P2_03MODEW::DISABLED => 2,
                P2_03MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_03MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_03MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_03MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.3 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_03MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.3 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_03MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.3 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_03MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.3 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_03MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_04MODE`" ]
    pub enum P2_04MODEW {
        # [ doc = "Pull-up. P2.4 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.4 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.4 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.4 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_04MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_04MODEW::PULL_UP => 0,
                P2_04MODEW::REPEATER => 1,
                P2_04MODEW::DISABLED => 2,
                P2_04MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_04MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_04MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_04MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.4 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_04MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.4 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_04MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.4 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_04MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.4 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_04MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_05MODE`" ]
    pub enum P2_05MODEW {
        # [ doc = "Pull-up. P2.5 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.5 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.5 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.5 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_05MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_05MODEW::PULL_UP => 0,
                P2_05MODEW::REPEATER => 1,
                P2_05MODEW::DISABLED => 2,
                P2_05MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_05MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_05MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_05MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.5 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_05MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.5 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_05MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.5 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_05MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.5 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_05MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_06MODE`" ]
    pub enum P2_06MODEW {
        # [ doc = "Pull-up. P2.6 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.6 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.6 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.6 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_06MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_06MODEW::PULL_UP => 0,
                P2_06MODEW::REPEATER => 1,
                P2_06MODEW::DISABLED => 2,
                P2_06MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_06MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_06MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_06MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.6 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_06MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.6 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_06MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.6 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_06MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.6 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_06MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_07MODE`" ]
    pub enum P2_07MODEW {
        # [ doc = "Pull-up. P2.7 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.7 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.7 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.7 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_07MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_07MODEW::PULL_UP => 0,
                P2_07MODEW::REPEATER => 1,
                P2_07MODEW::DISABLED => 2,
                P2_07MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_07MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_07MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_07MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.7 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_07MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.7 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_07MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.7 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_07MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.7 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_07MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_08MODE`" ]
    pub enum P2_08MODEW {
        # [ doc = "Pull-up. P2.8 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.8 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.8 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.8 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_08MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_08MODEW::PULL_UP => 0,
                P2_08MODEW::REPEATER => 1,
                P2_08MODEW::DISABLED => 2,
                P2_08MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_08MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_08MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_08MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.8 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_08MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.8 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_08MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.8 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_08MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.8 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_08MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_09MODE`" ]
    pub enum P2_09MODEW {
        # [ doc = "Pull-up. P2.9 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.9 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.9 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.9 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_09MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_09MODEW::PULL_UP => 0,
                P2_09MODEW::REPEATER => 1,
                P2_09MODEW::DISABLED => 2,
                P2_09MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_09MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_09MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_09MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.9 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_09MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.9 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_09MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.9 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_09MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.9 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_09MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_10MODE`" ]
    pub enum P2_10MODEW {
        # [ doc = "Pull-up. P2.10 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.10 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.10 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.10 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_10MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_10MODEW::PULL_UP => 0,
                P2_10MODEW::REPEATER => 1,
                P2_10MODEW::DISABLED => 2,
                P2_10MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_10MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_10MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_10MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.10 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_10MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.10 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_10MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.10 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_10MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.10 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_10MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_11MODE`" ]
    pub enum P2_11MODEW {
        # [ doc = "Pull-up. P2.11 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.11 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.11 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.11 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_11MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_11MODEW::PULL_UP => 0,
                P2_11MODEW::REPEATER => 1,
                P2_11MODEW::DISABLED => 2,
                P2_11MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_11MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_11MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_11MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.11 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_11MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.11 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_11MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.11 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_11MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.11 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_11MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_12MODE`" ]
    pub enum P2_12MODEW {
        # [ doc = "Pull-up. P2.12 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.12 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.12 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.12 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_12MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_12MODEW::PULL_UP => 0,
                P2_12MODEW::REPEATER => 1,
                P2_12MODEW::DISABLED => 2,
                P2_12MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_12MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_12MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_12MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.12 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_12MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.12 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_12MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.12 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_12MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.12 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_12MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_13MODE`" ]
    pub enum P2_13MODEW {
        # [ doc = "Pull-up. P2.13 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P2.13 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P2.13 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P2.13 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P2_13MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P2_13MODEW::PULL_UP => 0,
                P2_13MODEW::REPEATER => 1,
                P2_13MODEW::DISABLED => 2,
                P2_13MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_13MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_13MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_13MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P2.13 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P2_13MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P2.13 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P2_13MODEW::REPEATER)
        }
        # [ doc = "Disabled. P2.13 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P2_13MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P2.13 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P2_13MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Port 2 pin 0 control." ]
        # [ inline ( always ) ]
        pub fn p2_00mode(&self) -> P2_00MODER {
            P2_00MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 2:3 - Port 2 pin 1 control." ]
        # [ inline ( always ) ]
        pub fn p2_01mode(&self) -> P2_01MODER {
            P2_01MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 2;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 4:5 - Port 2 pin 2 control." ]
        # [ inline ( always ) ]
        pub fn p2_02mode(&self) -> P2_02MODER {
            P2_02MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 4;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 6:7 - Port 2 pin 3 control." ]
        # [ inline ( always ) ]
        pub fn p2_03mode(&self) -> P2_03MODER {
            P2_03MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 6;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 8:9 - Port 2 pin 4 control." ]
        # [ inline ( always ) ]
        pub fn p2_04mode(&self) -> P2_04MODER {
            P2_04MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 8;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 10:11 - Port 2 pin 5 control." ]
        # [ inline ( always ) ]
        pub fn p2_05mode(&self) -> P2_05MODER {
            P2_05MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 10;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 12:13 - Port 2 pin 6 control." ]
        # [ inline ( always ) ]
        pub fn p2_06mode(&self) -> P2_06MODER {
            P2_06MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 12;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 14:15 - Port 2 pin 7 control." ]
        # [ inline ( always ) ]
        pub fn p2_07mode(&self) -> P2_07MODER {
            P2_07MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 14;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 16:17 - Port 2 pin 8 control." ]
        # [ inline ( always ) ]
        pub fn p2_08mode(&self) -> P2_08MODER {
            P2_08MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 16;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 18:19 - Port 2 pin 9 control." ]
        # [ inline ( always ) ]
        pub fn p2_09mode(&self) -> P2_09MODER {
            P2_09MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 18;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 20:21 - Port 2 pin 10 control." ]
        # [ inline ( always ) ]
        pub fn p2_10mode(&self) -> P2_10MODER {
            P2_10MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 20;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 22:23 - Port 2 pin 11 control." ]
        # [ inline ( always ) ]
        pub fn p2_11mode(&self) -> P2_11MODER {
            P2_11MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 22;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 24:25 - Port 2 pin 12 control." ]
        # [ inline ( always ) ]
        pub fn p2_12mode(&self) -> P2_12MODER {
            P2_12MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 24;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 26:27 - Port 2 pin 13 control." ]
        # [ inline ( always ) ]
        pub fn p2_13mode(&self) -> P2_13MODER {
            P2_13MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 26;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Port 2 pin 0 control." ]
        # [ inline ( always ) ]
        pub fn p2_00mode(&mut self) -> _P2_00MODEW {
            _P2_00MODEW { w: self }
        }
        # [ doc = "Bits 2:3 - Port 2 pin 1 control." ]
        # [ inline ( always ) ]
        pub fn p2_01mode(&mut self) -> _P2_01MODEW {
            _P2_01MODEW { w: self }
        }
        # [ doc = "Bits 4:5 - Port 2 pin 2 control." ]
        # [ inline ( always ) ]
        pub fn p2_02mode(&mut self) -> _P2_02MODEW {
            _P2_02MODEW { w: self }
        }
        # [ doc = "Bits 6:7 - Port 2 pin 3 control." ]
        # [ inline ( always ) ]
        pub fn p2_03mode(&mut self) -> _P2_03MODEW {
            _P2_03MODEW { w: self }
        }
        # [ doc = "Bits 8:9 - Port 2 pin 4 control." ]
        # [ inline ( always ) ]
        pub fn p2_04mode(&mut self) -> _P2_04MODEW {
            _P2_04MODEW { w: self }
        }
        # [ doc = "Bits 10:11 - Port 2 pin 5 control." ]
        # [ inline ( always ) ]
        pub fn p2_05mode(&mut self) -> _P2_05MODEW {
            _P2_05MODEW { w: self }
        }
        # [ doc = "Bits 12:13 - Port 2 pin 6 control." ]
        # [ inline ( always ) ]
        pub fn p2_06mode(&mut self) -> _P2_06MODEW {
            _P2_06MODEW { w: self }
        }
        # [ doc = "Bits 14:15 - Port 2 pin 7 control." ]
        # [ inline ( always ) ]
        pub fn p2_07mode(&mut self) -> _P2_07MODEW {
            _P2_07MODEW { w: self }
        }
        # [ doc = "Bits 16:17 - Port 2 pin 8 control." ]
        # [ inline ( always ) ]
        pub fn p2_08mode(&mut self) -> _P2_08MODEW {
            _P2_08MODEW { w: self }
        }
        # [ doc = "Bits 18:19 - Port 2 pin 9 control." ]
        # [ inline ( always ) ]
        pub fn p2_09mode(&mut self) -> _P2_09MODEW {
            _P2_09MODEW { w: self }
        }
        # [ doc = "Bits 20:21 - Port 2 pin 10 control." ]
        # [ inline ( always ) ]
        pub fn p2_10mode(&mut self) -> _P2_10MODEW {
            _P2_10MODEW { w: self }
        }
        # [ doc = "Bits 22:23 - Port 2 pin 11 control." ]
        # [ inline ( always ) ]
        pub fn p2_11mode(&mut self) -> _P2_11MODEW {
            _P2_11MODEW { w: self }
        }
        # [ doc = "Bits 24:25 - Port 2 pin 12 control." ]
        # [ inline ( always ) ]
        pub fn p2_12mode(&mut self) -> _P2_12MODEW {
            _P2_12MODEW { w: self }
        }
        # [ doc = "Bits 26:27 - Port 2 pin 13 control." ]
        # [ inline ( always ) ]
        pub fn p2_13mode(&mut self) -> _P2_13MODEW {
            _P2_13MODEW { w: self }
        }
    }
}
# [ doc = "Pin mode select register 7" ]
pub struct PINMODE7 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin mode select register 7" ]
pub mod pinmode7 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE7 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P3_25MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P3_25MODER {
        # [ doc = "Pull-up. P3.25 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P3.25 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P3.25 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P3.25 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P3_25MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P3_25MODER::PULL_UP => 0,
                P3_25MODER::REPEATER => 1,
                P3_25MODER::DISABLED => 2,
                P3_25MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P3_25MODER {
            match value {
                0 => P3_25MODER::PULL_UP,
                1 => P3_25MODER::REPEATER,
                2 => P3_25MODER::DISABLED,
                3 => P3_25MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P3_25MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P3_25MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P3_25MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P3_25MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P3_26MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P3_26MODER {
        # [ doc = "Pull-up. P3.26 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P3.26 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P3.26 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P3.26 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P3_26MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P3_26MODER::PULL_UP => 0,
                P3_26MODER::REPEATER => 1,
                P3_26MODER::DISABLED => 2,
                P3_26MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P3_26MODER {
            match value {
                0 => P3_26MODER::PULL_UP,
                1 => P3_26MODER::REPEATER,
                2 => P3_26MODER::DISABLED,
                3 => P3_26MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P3_26MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P3_26MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P3_26MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P3_26MODER::PULL_DOWN
        }
    }
    # [ doc = "Values that can be written to the field `P3_25MODE`" ]
    pub enum P3_25MODEW {
        # [ doc = "Pull-up. P3.25 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P3.25 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P3.25 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P3.25 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P3_25MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P3_25MODEW::PULL_UP => 0,
                P3_25MODEW::REPEATER => 1,
                P3_25MODEW::DISABLED => 2,
                P3_25MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P3_25MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P3_25MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P3_25MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P3.25 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P3_25MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P3.25 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P3_25MODEW::REPEATER)
        }
        # [ doc = "Disabled. P3.25 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P3_25MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P3.25 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P3_25MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P3_26MODE`" ]
    pub enum P3_26MODEW {
        # [ doc = "Pull-up. P3.26 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P3.26 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P3.26 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P3.26 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P3_26MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P3_26MODEW::PULL_UP => 0,
                P3_26MODEW::REPEATER => 1,
                P3_26MODEW::DISABLED => 2,
                P3_26MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P3_26MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P3_26MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P3_26MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P3.26 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P3_26MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P3.26 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P3_26MODEW::REPEATER)
        }
        # [ doc = "Disabled. P3.26 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P3_26MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P3.26 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P3_26MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 18:19 - Port 3 pin 25 control." ]
        # [ inline ( always ) ]
        pub fn p3_25mode(&self) -> P3_25MODER {
            P3_25MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 18;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 20:21 - Port 3 pin 26 control." ]
        # [ inline ( always ) ]
        pub fn p3_26mode(&self) -> P3_26MODER {
            P3_26MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 20;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 18:19 - Port 3 pin 25 control." ]
        # [ inline ( always ) ]
        pub fn p3_25mode(&mut self) -> _P3_25MODEW {
            _P3_25MODEW { w: self }
        }
        # [ doc = "Bits 20:21 - Port 3 pin 26 control." ]
        # [ inline ( always ) ]
        pub fn p3_26mode(&mut self) -> _P3_26MODEW {
            _P3_26MODEW { w: self }
        }
    }
}
# [ doc = "Pin mode select register 9" ]
pub struct PINMODE9 {
    register: VolatileCell<u32>,
}
# [ doc = "Pin mode select register 9" ]
pub mod pinmode9 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE9 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P4_28MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P4_28MODER {
        # [ doc = "Pull-up. P4.28 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P4.28 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P4.28 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P4.28 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P4_28MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P4_28MODER::PULL_UP => 0,
                P4_28MODER::REPEATER => 1,
                P4_28MODER::DISABLED => 2,
                P4_28MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P4_28MODER {
            match value {
                0 => P4_28MODER::PULL_UP,
                1 => P4_28MODER::REPEATER,
                2 => P4_28MODER::DISABLED,
                3 => P4_28MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P4_28MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P4_28MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P4_28MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P4_28MODER::PULL_DOWN
        }
    }
    # [ doc = "Possible values of the field `P4_29MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P4_29MODER {
        # [ doc = "Pull-up. P4.29 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P4.29 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P4.29 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P4.29 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P4_29MODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                P4_29MODER::PULL_UP => 0,
                P4_29MODER::REPEATER => 1,
                P4_29MODER::DISABLED => 2,
                P4_29MODER::PULL_DOWN => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> P4_29MODER {
            match value {
                0 => P4_29MODER::PULL_UP,
                1 => P4_29MODER::REPEATER,
                2 => P4_29MODER::DISABLED,
                3 => P4_29MODER::PULL_DOWN,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `PULL_UP`" ]
        # [ inline ( always ) ]
        pub fn is_pull_up(&self) -> bool {
            *self == P4_29MODER::PULL_UP
        }
        # [ doc = "Checks if the value of the field is `REPEATER`" ]
        # [ inline ( always ) ]
        pub fn is_repeater(&self) -> bool {
            *self == P4_29MODER::REPEATER
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == P4_29MODER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `PULL_DOWN`" ]
        # [ inline ( always ) ]
        pub fn is_pull_down(&self) -> bool {
            *self == P4_29MODER::PULL_DOWN
        }
    }
    # [ doc = "Values that can be written to the field `P4_28MODE`" ]
    pub enum P4_28MODEW {
        # [ doc = "Pull-up. P4.28 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P4.28 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P4.28 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P4.28 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P4_28MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P4_28MODEW::PULL_UP => 0,
                P4_28MODEW::REPEATER => 1,
                P4_28MODEW::DISABLED => 2,
                P4_28MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P4_28MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P4_28MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P4_28MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P4.28 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P4_28MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P4.28 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P4_28MODEW::REPEATER)
        }
        # [ doc = "Disabled. P4.28 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P4_28MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P4.28 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P4_28MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P4_29MODE`" ]
    pub enum P4_29MODEW {
        # [ doc = "Pull-up. P4.29 pin has a pull-up resistor enabled." ]
        PULL_UP,
        # [ doc = "Repeater. P4.29 pin has repeater mode enabled." ]
        REPEATER,
        # [ doc = "Disabled. P4.29 pin has neither pull-up nor pull-down." ]
        DISABLED,
        # [ doc = "Pull-down. P4.29 has a pull-down resistor enabled." ]
        PULL_DOWN,
    }
    impl P4_29MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                P4_29MODEW::PULL_UP => 0,
                P4_29MODEW::REPEATER => 1,
                P4_29MODEW::DISABLED => 2,
                P4_29MODEW::PULL_DOWN => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P4_29MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P4_29MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P4_29MODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Pull-up. P4.29 pin has a pull-up resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(P4_29MODEW::PULL_UP)
        }
        # [ doc = "Repeater. P4.29 pin has repeater mode enabled." ]
        # [ inline ( always ) ]
        pub fn repeater(self) -> &'a mut W {
            self.variant(P4_29MODEW::REPEATER)
        }
        # [ doc = "Disabled. P4.29 pin has neither pull-up nor pull-down." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(P4_29MODEW::DISABLED)
        }
        # [ doc = "Pull-down. P4.29 has a pull-down resistor enabled." ]
        # [ inline ( always ) ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(P4_29MODEW::PULL_DOWN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 24:25 - Port 4 pin 28 control." ]
        # [ inline ( always ) ]
        pub fn p4_28mode(&self) -> P4_28MODER {
            P4_28MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 24;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 26:27 - Port 4 pin 29 control." ]
        # [ inline ( always ) ]
        pub fn p4_29mode(&self) -> P4_29MODER {
            P4_29MODER::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 26;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 24:25 - Port 4 pin 28 control." ]
        # [ inline ( always ) ]
        pub fn p4_28mode(&mut self) -> _P4_28MODEW {
            _P4_28MODEW { w: self }
        }
        # [ doc = "Bits 26:27 - Port 4 pin 29 control." ]
        # [ inline ( always ) ]
        pub fn p4_29mode(&mut self) -> _P4_29MODEW {
            _P4_29MODEW { w: self }
        }
    }
}
# [ doc = "Open drain mode control register 0" ]
pub struct PINMODE_OD0 {
    register: VolatileCell<u32>,
}
# [ doc = "Open drain mode control register 0" ]
pub mod pinmode_od0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE_OD0 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P0_00OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_00ODR {
        # [ doc = "Normal. P0.0 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.0 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_00ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_00ODR::NORMAL => false,
                P0_00ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_00ODR {
            match value {
                false => P0_00ODR::NORMAL,
                true => P0_00ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_00ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_00ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_01OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_01ODR {
        # [ doc = "Normal. P0.1 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.1 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_01ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_01ODR::NORMAL => false,
                P0_01ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_01ODR {
            match value {
                false => P0_01ODR::NORMAL,
                true => P0_01ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_01ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_01ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_02OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_02ODR {
        # [ doc = "Normal. P0.2 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.2 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_02ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_02ODR::NORMAL => false,
                P0_02ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_02ODR {
            match value {
                false => P0_02ODR::NORMAL,
                true => P0_02ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_02ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_02ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_03OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_03ODR {
        # [ doc = "Normal. P0.3 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.3 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_03ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_03ODR::NORMAL => false,
                P0_03ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_03ODR {
            match value {
                false => P0_03ODR::NORMAL,
                true => P0_03ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_03ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_03ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_04OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_04ODR {
        # [ doc = "Normal. P0.4 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.4 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_04ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_04ODR::NORMAL => false,
                P0_04ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_04ODR {
            match value {
                false => P0_04ODR::NORMAL,
                true => P0_04ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_04ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_04ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_05OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_05ODR {
        # [ doc = "Normal. P0.5 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.5 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_05ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_05ODR::NORMAL => false,
                P0_05ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_05ODR {
            match value {
                false => P0_05ODR::NORMAL,
                true => P0_05ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_05ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_05ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_06OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_06ODR {
        # [ doc = "Normal. P0.6 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.6 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_06ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_06ODR::NORMAL => false,
                P0_06ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_06ODR {
            match value {
                false => P0_06ODR::NORMAL,
                true => P0_06ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_06ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_06ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_07OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_07ODR {
        # [ doc = "Normal. P0.7 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.7 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_07ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_07ODR::NORMAL => false,
                P0_07ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_07ODR {
            match value {
                false => P0_07ODR::NORMAL,
                true => P0_07ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_07ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_07ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_08OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_08ODR {
        # [ doc = "Normal. P0.8 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.8 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_08ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_08ODR::NORMAL => false,
                P0_08ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_08ODR {
            match value {
                false => P0_08ODR::NORMAL,
                true => P0_08ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_08ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_08ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_09OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_09ODR {
        # [ doc = "Normal. P0.9 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.9 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_09ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_09ODR::NORMAL => false,
                P0_09ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_09ODR {
            match value {
                false => P0_09ODR::NORMAL,
                true => P0_09ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_09ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_09ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_10OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_10ODR {
        # [ doc = "Normal. P0.10 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.10 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_10ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_10ODR::NORMAL => false,
                P0_10ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_10ODR {
            match value {
                false => P0_10ODR::NORMAL,
                true => P0_10ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_10ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_10ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_11OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_11ODR {
        # [ doc = "Normal. P0.11 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.11 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_11ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_11ODR::NORMAL => false,
                P0_11ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_11ODR {
            match value {
                false => P0_11ODR::NORMAL,
                true => P0_11ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_11ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_11ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_15OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_15ODR {
        # [ doc = "Normal. P0.15 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.15 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_15ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_15ODR::NORMAL => false,
                P0_15ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_15ODR {
            match value {
                false => P0_15ODR::NORMAL,
                true => P0_15ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_15ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_15ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_16OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_16ODR {
        # [ doc = "Normal. P0.16 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.16 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_16ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_16ODR::NORMAL => false,
                P0_16ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_16ODR {
            match value {
                false => P0_16ODR::NORMAL,
                true => P0_16ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_16ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_16ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_17OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_17ODR {
        # [ doc = "Normal. P0.17 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.17 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_17ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_17ODR::NORMAL => false,
                P0_17ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_17ODR {
            match value {
                false => P0_17ODR::NORMAL,
                true => P0_17ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_17ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_17ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_18OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_18ODR {
        # [ doc = "Normal. P0.18 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.18 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_18ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_18ODR::NORMAL => false,
                P0_18ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_18ODR {
            match value {
                false => P0_18ODR::NORMAL,
                true => P0_18ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_18ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_18ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_19OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_19ODR {
        # [ doc = "Normal. P0.19 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.19 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_19ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_19ODR::NORMAL => false,
                P0_19ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_19ODR {
            match value {
                false => P0_19ODR::NORMAL,
                true => P0_19ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_19ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_19ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_20OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_20ODR {
        # [ doc = "Normal. P0.20 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.20 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_20ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_20ODR::NORMAL => false,
                P0_20ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_20ODR {
            match value {
                false => P0_20ODR::NORMAL,
                true => P0_20ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_20ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_20ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_21OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_21ODR {
        # [ doc = "Normal. P0.21 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.21 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_21ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_21ODR::NORMAL => false,
                P0_21ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_21ODR {
            match value {
                false => P0_21ODR::NORMAL,
                true => P0_21ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_21ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_21ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_22OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_22ODR {
        # [ doc = "Normal. P0.22 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.22 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_22ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_22ODR::NORMAL => false,
                P0_22ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_22ODR {
            match value {
                false => P0_22ODR::NORMAL,
                true => P0_22ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_22ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_22ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_23OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_23ODR {
        # [ doc = "Normal. P0.23 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.23 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_23ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_23ODR::NORMAL => false,
                P0_23ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_23ODR {
            match value {
                false => P0_23ODR::NORMAL,
                true => P0_23ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_23ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_23ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_24OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_24ODR {
        # [ doc = "Normal. P0.23 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.23 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_24ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_24ODR::NORMAL => false,
                P0_24ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_24ODR {
            match value {
                false => P0_24ODR::NORMAL,
                true => P0_24ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_24ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_24ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_25OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_25ODR {
        # [ doc = "Normal. P0.25 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.25 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_25ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_25ODR::NORMAL => false,
                P0_25ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_25ODR {
            match value {
                false => P0_25ODR::NORMAL,
                true => P0_25ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_25ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_25ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_26OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_26ODR {
        # [ doc = "Normal. P0.26 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.26 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_26ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_26ODR::NORMAL => false,
                P0_26ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_26ODR {
            match value {
                false => P0_26ODR::NORMAL,
                true => P0_26ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_26ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_26ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_29OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_29ODR {
        # [ doc = "Normal. P0.29 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.29 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_29ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_29ODR::NORMAL => false,
                P0_29ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_29ODR {
            match value {
                false => P0_29ODR::NORMAL,
                true => P0_29ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_29ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_29ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P0_30OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0_30ODR {
        # [ doc = "Normal. P0.30 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.30 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_30ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P0_30ODR::NORMAL => false,
                P0_30ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0_30ODR {
            match value {
                false => P0_30ODR::NORMAL,
                true => P0_30ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P0_30ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P0_30ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Values that can be written to the field `P0_00OD`" ]
    pub enum P0_00ODW {
        # [ doc = "Normal. P0.0 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.0 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_00ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_00ODW::NORMAL => false,
                P0_00ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_00ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_00ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_00ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.0 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_00ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.0 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_00ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_01OD`" ]
    pub enum P0_01ODW {
        # [ doc = "Normal. P0.1 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.1 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_01ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_01ODW::NORMAL => false,
                P0_01ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_01ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_01ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_01ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.1 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_01ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.1 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_01ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_02OD`" ]
    pub enum P0_02ODW {
        # [ doc = "Normal. P0.2 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.2 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_02ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_02ODW::NORMAL => false,
                P0_02ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_02ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_02ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_02ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.2 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_02ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.2 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_02ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_03OD`" ]
    pub enum P0_03ODW {
        # [ doc = "Normal. P0.3 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.3 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_03ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_03ODW::NORMAL => false,
                P0_03ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_03ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_03ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_03ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.3 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_03ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.3 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_03ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_04OD`" ]
    pub enum P0_04ODW {
        # [ doc = "Normal. P0.4 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.4 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_04ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_04ODW::NORMAL => false,
                P0_04ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_04ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_04ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_04ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.4 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_04ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.4 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_04ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_05OD`" ]
    pub enum P0_05ODW {
        # [ doc = "Normal. P0.5 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.5 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_05ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_05ODW::NORMAL => false,
                P0_05ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_05ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_05ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_05ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.5 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_05ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.5 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_05ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_06OD`" ]
    pub enum P0_06ODW {
        # [ doc = "Normal. P0.6 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.6 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_06ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_06ODW::NORMAL => false,
                P0_06ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_06ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_06ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_06ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.6 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_06ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.6 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_06ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_07OD`" ]
    pub enum P0_07ODW {
        # [ doc = "Normal. P0.7 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.7 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_07ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_07ODW::NORMAL => false,
                P0_07ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_07ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_07ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_07ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.7 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_07ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.7 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_07ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_08OD`" ]
    pub enum P0_08ODW {
        # [ doc = "Normal. P0.8 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.8 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_08ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_08ODW::NORMAL => false,
                P0_08ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_08ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_08ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_08ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.8 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_08ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.8 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_08ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_09OD`" ]
    pub enum P0_09ODW {
        # [ doc = "Normal. P0.9 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.9 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_09ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_09ODW::NORMAL => false,
                P0_09ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_09ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_09ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_09ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.9 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_09ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.9 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_09ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_10OD`" ]
    pub enum P0_10ODW {
        # [ doc = "Normal. P0.10 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.10 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_10ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_10ODW::NORMAL => false,
                P0_10ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_10ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_10ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_10ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.10 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_10ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.10 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_10ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_11OD`" ]
    pub enum P0_11ODW {
        # [ doc = "Normal. P0.11 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.11 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_11ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_11ODW::NORMAL => false,
                P0_11ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_11ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_11ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_11ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.11 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_11ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.11 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_11ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_15OD`" ]
    pub enum P0_15ODW {
        # [ doc = "Normal. P0.15 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.15 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_15ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_15ODW::NORMAL => false,
                P0_15ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_15ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_15ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_15ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.15 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_15ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.15 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_15ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_16OD`" ]
    pub enum P0_16ODW {
        # [ doc = "Normal. P0.16 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.16 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_16ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_16ODW::NORMAL => false,
                P0_16ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_16ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_16ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_16ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.16 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_16ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.16 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_16ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_17OD`" ]
    pub enum P0_17ODW {
        # [ doc = "Normal. P0.17 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.17 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_17ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_17ODW::NORMAL => false,
                P0_17ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_17ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_17ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_17ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.17 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_17ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.17 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_17ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_18OD`" ]
    pub enum P0_18ODW {
        # [ doc = "Normal. P0.18 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.18 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_18ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_18ODW::NORMAL => false,
                P0_18ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_18ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_18ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_18ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.18 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_18ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.18 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_18ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_19OD`" ]
    pub enum P0_19ODW {
        # [ doc = "Normal. P0.19 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.19 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_19ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_19ODW::NORMAL => false,
                P0_19ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_19ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_19ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_19ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.19 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_19ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.19 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_19ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_20OD`" ]
    pub enum P0_20ODW {
        # [ doc = "Normal. P0.20 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.20 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_20ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_20ODW::NORMAL => false,
                P0_20ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_20ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_20ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_20ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.20 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_20ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.20 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_20ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_21OD`" ]
    pub enum P0_21ODW {
        # [ doc = "Normal. P0.21 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.21 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_21ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_21ODW::NORMAL => false,
                P0_21ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_21ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_21ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_21ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.21 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_21ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.21 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_21ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_22OD`" ]
    pub enum P0_22ODW {
        # [ doc = "Normal. P0.22 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.22 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_22ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_22ODW::NORMAL => false,
                P0_22ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_22ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_22ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_22ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.22 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_22ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.22 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_22ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_23OD`" ]
    pub enum P0_23ODW {
        # [ doc = "Normal. P0.23 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.23 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_23ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_23ODW::NORMAL => false,
                P0_23ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_23ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_23ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_23ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.23 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_23ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.23 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_23ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_24OD`" ]
    pub enum P0_24ODW {
        # [ doc = "Normal. P0.23 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.23 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_24ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_24ODW::NORMAL => false,
                P0_24ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_24ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_24ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_24ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.23 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_24ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.23 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_24ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_25OD`" ]
    pub enum P0_25ODW {
        # [ doc = "Normal. P0.25 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.25 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_25ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_25ODW::NORMAL => false,
                P0_25ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_25ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_25ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_25ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.25 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_25ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.25 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_25ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_26OD`" ]
    pub enum P0_26ODW {
        # [ doc = "Normal. P0.26 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.26 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_26ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_26ODW::NORMAL => false,
                P0_26ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_26ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_26ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_26ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.26 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_26ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.26 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_26ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_29OD`" ]
    pub enum P0_29ODW {
        # [ doc = "Normal. P0.29 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.29 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_29ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_29ODW::NORMAL => false,
                P0_29ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_29ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_29ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_29ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.29 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_29ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.29 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_29ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P0_30OD`" ]
    pub enum P0_30ODW {
        # [ doc = "Normal. P0.30 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P0.30 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P0_30ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P0_30ODW::NORMAL => false,
                P0_30ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P0_30ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_30ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P0_30ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P0.30 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P0_30ODW::NORMAL)
        }
        # [ doc = "Open-drain. P0.30 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P0_30ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_00od(&self) -> P0_00ODR {
            P0_00ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 0;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_01od(&self) -> P0_01ODR {
            P0_01ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 1;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 2 - Port 0 pin 2 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_02od(&self) -> P0_02ODR {
            P0_02ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 2;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 3 - Port 0 pin 3 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_03od(&self) -> P0_03ODR {
            P0_03ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 3;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 4 - Port 0 pin 4 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_04od(&self) -> P0_04ODR {
            P0_04ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 4;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 5 - Port 0 pin 5 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_05od(&self) -> P0_05ODR {
            P0_05ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 5;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 6 - Port 0 pin 6 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_06od(&self) -> P0_06ODR {
            P0_06ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 6;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 7 - Port 0 pin 7 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_07od(&self) -> P0_07ODR {
            P0_07ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 7;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 8 - Port 0 pin 8 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_08od(&self) -> P0_08ODR {
            P0_08ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 8;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 9 - Port 0 pin 9 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_09od(&self) -> P0_09ODR {
            P0_09ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 9;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_10od(&self) -> P0_10ODR {
            P0_10ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 10;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_11od(&self) -> P0_11ODR {
            P0_11ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 11;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 15 - Port 0 pin 15 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_15od(&self) -> P0_15ODR {
            P0_15ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 15;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 16 - Port 0 pin 16 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_16od(&self) -> P0_16ODR {
            P0_16ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 16;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 17 - Port 0 pin 17 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_17od(&self) -> P0_17ODR {
            P0_17ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 17;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 18 - Port 0 pin 18 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_18od(&self) -> P0_18ODR {
            P0_18ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 18;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_19od(&self) -> P0_19ODR {
            P0_19ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 19;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_20od(&self) -> P0_20ODR {
            P0_20ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 20;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 21 - Port 0 pin 21 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_21od(&self) -> P0_21ODR {
            P0_21ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 21;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 22 - Port 0 pin 22 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_22od(&self) -> P0_22ODR {
            P0_22ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 22;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 23 - Port 0 pin 23 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_23od(&self) -> P0_23ODR {
            P0_23ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 23;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 24 - Port 0 pin 24open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_24od(&self) -> P0_24ODR {
            P0_24ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 24;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 25 - Port 0 pin 25 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_25od(&self) -> P0_25ODR {
            P0_25ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 25;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 26 - Port 0 pin 26 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_26od(&self) -> P0_26ODR {
            P0_26ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 26;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 29 - Port 0 pin 29 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_29od(&self) -> P0_29ODR {
            P0_29ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 29;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 30 - Port 0 pin 30 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_30od(&self) -> P0_30ODR {
            P0_30ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 30;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_00od(&mut self) -> _P0_00ODW {
            _P0_00ODW { w: self }
        }
        # [ doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_01od(&mut self) -> _P0_01ODW {
            _P0_01ODW { w: self }
        }
        # [ doc = "Bit 2 - Port 0 pin 2 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_02od(&mut self) -> _P0_02ODW {
            _P0_02ODW { w: self }
        }
        # [ doc = "Bit 3 - Port 0 pin 3 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_03od(&mut self) -> _P0_03ODW {
            _P0_03ODW { w: self }
        }
        # [ doc = "Bit 4 - Port 0 pin 4 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_04od(&mut self) -> _P0_04ODW {
            _P0_04ODW { w: self }
        }
        # [ doc = "Bit 5 - Port 0 pin 5 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_05od(&mut self) -> _P0_05ODW {
            _P0_05ODW { w: self }
        }
        # [ doc = "Bit 6 - Port 0 pin 6 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_06od(&mut self) -> _P0_06ODW {
            _P0_06ODW { w: self }
        }
        # [ doc = "Bit 7 - Port 0 pin 7 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_07od(&mut self) -> _P0_07ODW {
            _P0_07ODW { w: self }
        }
        # [ doc = "Bit 8 - Port 0 pin 8 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_08od(&mut self) -> _P0_08ODW {
            _P0_08ODW { w: self }
        }
        # [ doc = "Bit 9 - Port 0 pin 9 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_09od(&mut self) -> _P0_09ODW {
            _P0_09ODW { w: self }
        }
        # [ doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_10od(&mut self) -> _P0_10ODW {
            _P0_10ODW { w: self }
        }
        # [ doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_11od(&mut self) -> _P0_11ODW {
            _P0_11ODW { w: self }
        }
        # [ doc = "Bit 15 - Port 0 pin 15 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_15od(&mut self) -> _P0_15ODW {
            _P0_15ODW { w: self }
        }
        # [ doc = "Bit 16 - Port 0 pin 16 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_16od(&mut self) -> _P0_16ODW {
            _P0_16ODW { w: self }
        }
        # [ doc = "Bit 17 - Port 0 pin 17 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_17od(&mut self) -> _P0_17ODW {
            _P0_17ODW { w: self }
        }
        # [ doc = "Bit 18 - Port 0 pin 18 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_18od(&mut self) -> _P0_18ODW {
            _P0_18ODW { w: self }
        }
        # [ doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_19od(&mut self) -> _P0_19ODW {
            _P0_19ODW { w: self }
        }
        # [ doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0." ]
        # [ inline ( always ) ]
        pub fn p0_20od(&mut self) -> _P0_20ODW {
            _P0_20ODW { w: self }
        }
        # [ doc = "Bit 21 - Port 0 pin 21 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_21od(&mut self) -> _P0_21ODW {
            _P0_21ODW { w: self }
        }
        # [ doc = "Bit 22 - Port 0 pin 22 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_22od(&mut self) -> _P0_22ODW {
            _P0_22ODW { w: self }
        }
        # [ doc = "Bit 23 - Port 0 pin 23 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_23od(&mut self) -> _P0_23ODW {
            _P0_23ODW { w: self }
        }
        # [ doc = "Bit 24 - Port 0 pin 24open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_24od(&mut self) -> _P0_24ODW {
            _P0_24ODW { w: self }
        }
        # [ doc = "Bit 25 - Port 0 pin 25 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_25od(&mut self) -> _P0_25ODW {
            _P0_25ODW { w: self }
        }
        # [ doc = "Bit 26 - Port 0 pin 26 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_26od(&mut self) -> _P0_26ODW {
            _P0_26ODW { w: self }
        }
        # [ doc = "Bit 29 - Port 0 pin 29 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_29od(&mut self) -> _P0_29ODW {
            _P0_29ODW { w: self }
        }
        # [ doc = "Bit 30 - Port 0 pin 30 open drain mode control" ]
        # [ inline ( always ) ]
        pub fn p0_30od(&mut self) -> _P0_30ODW {
            _P0_30ODW { w: self }
        }
    }
}
# [ doc = "Open drain mode control register 1" ]
pub struct PINMODE_OD1 {
    register: VolatileCell<u32>,
}
# [ doc = "Open drain mode control register 1" ]
pub mod pinmode_od1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE_OD1 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P1_00OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_00ODR {
        # [ doc = "Normal. P1.0 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.0 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_00ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_00ODR::NORMAL => false,
                P1_00ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_00ODR {
            match value {
                false => P1_00ODR::NORMAL,
                true => P1_00ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_00ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_00ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_01OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_01ODR {
        # [ doc = "Normal. P1.1 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.1 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_01ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_01ODR::NORMAL => false,
                P1_01ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_01ODR {
            match value {
                false => P1_01ODR::NORMAL,
                true => P1_01ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_01ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_01ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_04OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_04ODR {
        # [ doc = "Normal. P1.4 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.4 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_04ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_04ODR::NORMAL => false,
                P1_04ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_04ODR {
            match value {
                false => P1_04ODR::NORMAL,
                true => P1_04ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_04ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_04ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_08OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_08ODR {
        # [ doc = "Normal. P1.8 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.8 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_08ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_08ODR::NORMAL => false,
                P1_08ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_08ODR {
            match value {
                false => P1_08ODR::NORMAL,
                true => P1_08ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_08ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_08ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_09OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_09ODR {
        # [ doc = "Normal. P1.9 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.9 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_09ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_09ODR::NORMAL => false,
                P1_09ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_09ODR {
            match value {
                false => P1_09ODR::NORMAL,
                true => P1_09ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_09ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_09ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_10OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_10ODR {
        # [ doc = "Normal. P1.10 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.10 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_10ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_10ODR::NORMAL => false,
                P1_10ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_10ODR {
            match value {
                false => P1_10ODR::NORMAL,
                true => P1_10ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_10ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_10ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_14OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_14ODR {
        # [ doc = "Normal. P1.14 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.14 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_14ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_14ODR::NORMAL => false,
                P1_14ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_14ODR {
            match value {
                false => P1_14ODR::NORMAL,
                true => P1_14ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_14ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_14ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_15OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_15ODR {
        # [ doc = "Normal. P1.15 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.15 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_15ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_15ODR::NORMAL => false,
                P1_15ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_15ODR {
            match value {
                false => P1_15ODR::NORMAL,
                true => P1_15ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_15ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_15ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_16OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_16ODR {
        # [ doc = "Normal. P1.16 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.16 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_16ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_16ODR::NORMAL => false,
                P1_16ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_16ODR {
            match value {
                false => P1_16ODR::NORMAL,
                true => P1_16ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_16ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_16ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_17OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_17ODR {
        # [ doc = "Normal. P1.17 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.17 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_17ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_17ODR::NORMAL => false,
                P1_17ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_17ODR {
            match value {
                false => P1_17ODR::NORMAL,
                true => P1_17ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_17ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_17ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_18OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_18ODR {
        # [ doc = "Normal. P1.18 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.18 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_18ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_18ODR::NORMAL => false,
                P1_18ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_18ODR {
            match value {
                false => P1_18ODR::NORMAL,
                true => P1_18ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_18ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_18ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_19OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_19ODR {
        # [ doc = "Normal. P1.19 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.19 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_19ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_19ODR::NORMAL => false,
                P1_19ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_19ODR {
            match value {
                false => P1_19ODR::NORMAL,
                true => P1_19ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_19ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_19ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_20OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_20ODR {
        # [ doc = "Normal. P1.20 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.20 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_20ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_20ODR::NORMAL => false,
                P1_20ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_20ODR {
            match value {
                false => P1_20ODR::NORMAL,
                true => P1_20ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_20ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_20ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_21OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_21ODR {
        # [ doc = "Normal. P1.21 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.21 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_21ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_21ODR::NORMAL => false,
                P1_21ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_21ODR {
            match value {
                false => P1_21ODR::NORMAL,
                true => P1_21ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_21ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_21ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_22OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_22ODR {
        # [ doc = "Normal. P1.22 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.22 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_22ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_22ODR::NORMAL => false,
                P1_22ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_22ODR {
            match value {
                false => P1_22ODR::NORMAL,
                true => P1_22ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_22ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_22ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_23OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_23ODR {
        # [ doc = "Normal. P1.23 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.23 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_23ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_23ODR::NORMAL => false,
                P1_23ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_23ODR {
            match value {
                false => P1_23ODR::NORMAL,
                true => P1_23ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_23ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_23ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_24OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_24ODR {
        # [ doc = "Normal. P1.24 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.24 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_24ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_24ODR::NORMAL => false,
                P1_24ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_24ODR {
            match value {
                false => P1_24ODR::NORMAL,
                true => P1_24ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_24ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_24ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_25OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_25ODR {
        # [ doc = "Normal. P1.25 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.25 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_25ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_25ODR::NORMAL => false,
                P1_25ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_25ODR {
            match value {
                false => P1_25ODR::NORMAL,
                true => P1_25ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_25ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_25ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_26OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_26ODR {
        # [ doc = "Normal. P1.26 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.26 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_26ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_26ODR::NORMAL => false,
                P1_26ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_26ODR {
            match value {
                false => P1_26ODR::NORMAL,
                true => P1_26ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_26ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_26ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_27OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_27ODR {
        # [ doc = "Normal. P1.27 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.27 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_27ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_27ODR::NORMAL => false,
                P1_27ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_27ODR {
            match value {
                false => P1_27ODR::NORMAL,
                true => P1_27ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_27ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_27ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_28OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_28ODR {
        # [ doc = "Normal. P1.28 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.28 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_28ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_28ODR::NORMAL => false,
                P1_28ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_28ODR {
            match value {
                false => P1_28ODR::NORMAL,
                true => P1_28ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_28ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_28ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_29OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_29ODR {
        # [ doc = "Normal. P1.29 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.29 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_29ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_29ODR::NORMAL => false,
                P1_29ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_29ODR {
            match value {
                false => P1_29ODR::NORMAL,
                true => P1_29ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_29ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_29ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_30OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_30ODR {
        # [ doc = "Normal. P1.30 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.30 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_30ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_30ODR::NORMAL => false,
                P1_30ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_30ODR {
            match value {
                false => P1_30ODR::NORMAL,
                true => P1_30ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_30ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_30ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P1_31OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P1_31ODR {
        # [ doc = "Normal. P1.31 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.31 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_31ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P1_31ODR::NORMAL => false,
                P1_31ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P1_31ODR {
            match value {
                false => P1_31ODR::NORMAL,
                true => P1_31ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P1_31ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P1_31ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Values that can be written to the field `P1_00OD`" ]
    pub enum P1_00ODW {
        # [ doc = "Normal. P1.0 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.0 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_00ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_00ODW::NORMAL => false,
                P1_00ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_00ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_00ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_00ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.0 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_00ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.0 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_00ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_01OD`" ]
    pub enum P1_01ODW {
        # [ doc = "Normal. P1.1 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.1 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_01ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_01ODW::NORMAL => false,
                P1_01ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_01ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_01ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_01ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.1 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_01ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.1 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_01ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_04OD`" ]
    pub enum P1_04ODW {
        # [ doc = "Normal. P1.4 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.4 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_04ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_04ODW::NORMAL => false,
                P1_04ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_04ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_04ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_04ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.4 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_04ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.4 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_04ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_08OD`" ]
    pub enum P1_08ODW {
        # [ doc = "Normal. P1.8 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.8 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_08ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_08ODW::NORMAL => false,
                P1_08ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_08ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_08ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_08ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.8 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_08ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.8 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_08ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_09OD`" ]
    pub enum P1_09ODW {
        # [ doc = "Normal. P1.9 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.9 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_09ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_09ODW::NORMAL => false,
                P1_09ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_09ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_09ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_09ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.9 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_09ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.9 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_09ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_10OD`" ]
    pub enum P1_10ODW {
        # [ doc = "Normal. P1.10 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.10 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_10ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_10ODW::NORMAL => false,
                P1_10ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_10ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_10ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_10ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.10 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_10ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.10 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_10ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_14OD`" ]
    pub enum P1_14ODW {
        # [ doc = "Normal. P1.14 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.14 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_14ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_14ODW::NORMAL => false,
                P1_14ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_14ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_14ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_14ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.14 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_14ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.14 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_14ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_15OD`" ]
    pub enum P1_15ODW {
        # [ doc = "Normal. P1.15 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.15 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_15ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_15ODW::NORMAL => false,
                P1_15ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_15ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_15ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_15ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.15 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_15ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.15 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_15ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_16OD`" ]
    pub enum P1_16ODW {
        # [ doc = "Normal. P1.16 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.16 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_16ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_16ODW::NORMAL => false,
                P1_16ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_16ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_16ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_16ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.16 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_16ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.16 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_16ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_17OD`" ]
    pub enum P1_17ODW {
        # [ doc = "Normal. P1.17 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.17 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_17ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_17ODW::NORMAL => false,
                P1_17ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_17ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_17ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_17ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.17 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_17ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.17 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_17ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_18OD`" ]
    pub enum P1_18ODW {
        # [ doc = "Normal. P1.18 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.18 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_18ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_18ODW::NORMAL => false,
                P1_18ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_18ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_18ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_18ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.18 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_18ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.18 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_18ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_19OD`" ]
    pub enum P1_19ODW {
        # [ doc = "Normal. P1.19 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.19 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_19ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_19ODW::NORMAL => false,
                P1_19ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_19ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_19ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_19ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.19 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_19ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.19 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_19ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_20OD`" ]
    pub enum P1_20ODW {
        # [ doc = "Normal. P1.20 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.20 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_20ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_20ODW::NORMAL => false,
                P1_20ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_20ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_20ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_20ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.20 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_20ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.20 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_20ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_21OD`" ]
    pub enum P1_21ODW {
        # [ doc = "Normal. P1.21 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.21 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_21ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_21ODW::NORMAL => false,
                P1_21ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_21ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_21ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_21ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.21 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_21ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.21 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_21ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_22OD`" ]
    pub enum P1_22ODW {
        # [ doc = "Normal. P1.22 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.22 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_22ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_22ODW::NORMAL => false,
                P1_22ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_22ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_22ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_22ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.22 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_22ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.22 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_22ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_23OD`" ]
    pub enum P1_23ODW {
        # [ doc = "Normal. P1.23 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.23 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_23ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_23ODW::NORMAL => false,
                P1_23ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_23ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_23ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_23ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.23 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_23ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.23 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_23ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_24OD`" ]
    pub enum P1_24ODW {
        # [ doc = "Normal. P1.24 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.24 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_24ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_24ODW::NORMAL => false,
                P1_24ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_24ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_24ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_24ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.24 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_24ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.24 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_24ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_25OD`" ]
    pub enum P1_25ODW {
        # [ doc = "Normal. P1.25 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.25 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_25ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_25ODW::NORMAL => false,
                P1_25ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_25ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_25ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_25ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.25 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_25ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.25 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_25ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_26OD`" ]
    pub enum P1_26ODW {
        # [ doc = "Normal. P1.26 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.26 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_26ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_26ODW::NORMAL => false,
                P1_26ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_26ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_26ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_26ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.26 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_26ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.26 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_26ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_27OD`" ]
    pub enum P1_27ODW {
        # [ doc = "Normal. P1.27 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.27 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_27ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_27ODW::NORMAL => false,
                P1_27ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_27ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_27ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_27ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.27 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_27ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.27 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_27ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_28OD`" ]
    pub enum P1_28ODW {
        # [ doc = "Normal. P1.28 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.28 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_28ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_28ODW::NORMAL => false,
                P1_28ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_28ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_28ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_28ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.28 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_28ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.28 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_28ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_29OD`" ]
    pub enum P1_29ODW {
        # [ doc = "Normal. P1.29 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.29 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_29ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_29ODW::NORMAL => false,
                P1_29ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_29ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_29ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_29ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.29 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_29ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.29 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_29ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_30OD`" ]
    pub enum P1_30ODW {
        # [ doc = "Normal. P1.30 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.30 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_30ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_30ODW::NORMAL => false,
                P1_30ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_30ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_30ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_30ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.30 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_30ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.30 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_30ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P1_31OD`" ]
    pub enum P1_31ODW {
        # [ doc = "Normal. P1.31 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P1.31 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P1_31ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P1_31ODW::NORMAL => false,
                P1_31ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P1_31ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P1_31ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P1_31ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P1.31 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P1_31ODW::NORMAL)
        }
        # [ doc = "Open-drain. P1.31 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P1_31ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Port 1 pin 0 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p1_00od(&self) -> P1_00ODR {
            P1_00ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 0;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_01od(&self) -> P1_01ODR {
            P1_01ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 1;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_04od(&self) -> P1_04ODR {
            P1_04ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 4;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_08od(&self) -> P1_08ODR {
            P1_08ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 8;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_09od(&self) -> P1_09ODR {
            P1_09ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 9;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_10od(&self) -> P1_10ODR {
            P1_10ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 10;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_14od(&self) -> P1_14ODR {
            P1_14ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 14;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_15od(&self) -> P1_15ODR {
            P1_15ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 15;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_16od(&self) -> P1_16ODR {
            P1_16ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 16;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_17od(&self) -> P1_17ODR {
            P1_17ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 17;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_18od(&self) -> P1_18ODR {
            P1_18ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 18;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_19od(&self) -> P1_19ODR {
            P1_19ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 19;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_20od(&self) -> P1_20ODR {
            P1_20ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 20;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_21od(&self) -> P1_21ODR {
            P1_21ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 21;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_22od(&self) -> P1_22ODR {
            P1_22ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 22;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_23od(&self) -> P1_23ODR {
            P1_23ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 23;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_24od(&self) -> P1_24ODR {
            P1_24ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 24;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_25od(&self) -> P1_25ODR {
            P1_25ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 25;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_26od(&self) -> P1_26ODR {
            P1_26ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 26;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_27od(&self) -> P1_27ODR {
            P1_27ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 27;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_28od(&self) -> P1_28ODR {
            P1_28ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 28;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_29od(&self) -> P1_29ODR {
            P1_29ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 29;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_30od(&self) -> P1_30ODR {
            P1_30ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 30;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 31 - Port 1 pin 31 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p1_31od(&self) -> P1_31ODR {
            P1_31ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 31;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Port 1 pin 0 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p1_00od(&mut self) -> _P1_00ODW {
            _P1_00ODW { w: self }
        }
        # [ doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_01od(&mut self) -> _P1_01ODW {
            _P1_01ODW { w: self }
        }
        # [ doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_04od(&mut self) -> _P1_04ODW {
            _P1_04ODW { w: self }
        }
        # [ doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_08od(&mut self) -> _P1_08ODW {
            _P1_08ODW { w: self }
        }
        # [ doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_09od(&mut self) -> _P1_09ODW {
            _P1_09ODW { w: self }
        }
        # [ doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_10od(&mut self) -> _P1_10ODW {
            _P1_10ODW { w: self }
        }
        # [ doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_14od(&mut self) -> _P1_14ODW {
            _P1_14ODW { w: self }
        }
        # [ doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_15od(&mut self) -> _P1_15ODW {
            _P1_15ODW { w: self }
        }
        # [ doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_16od(&mut self) -> _P1_16ODW {
            _P1_16ODW { w: self }
        }
        # [ doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_17od(&mut self) -> _P1_17ODW {
            _P1_17ODW { w: self }
        }
        # [ doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_18od(&mut self) -> _P1_18ODW {
            _P1_18ODW { w: self }
        }
        # [ doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_19od(&mut self) -> _P1_19ODW {
            _P1_19ODW { w: self }
        }
        # [ doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_20od(&mut self) -> _P1_20ODW {
            _P1_20ODW { w: self }
        }
        # [ doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_21od(&mut self) -> _P1_21ODW {
            _P1_21ODW { w: self }
        }
        # [ doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_22od(&mut self) -> _P1_22ODW {
            _P1_22ODW { w: self }
        }
        # [ doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_23od(&mut self) -> _P1_23ODW {
            _P1_23ODW { w: self }
        }
        # [ doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_24od(&mut self) -> _P1_24ODW {
            _P1_24ODW { w: self }
        }
        # [ doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_25od(&mut self) -> _P1_25ODW {
            _P1_25ODW { w: self }
        }
        # [ doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_26od(&mut self) -> _P1_26ODW {
            _P1_26ODW { w: self }
        }
        # [ doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_27od(&mut self) -> _P1_27ODW {
            _P1_27ODW { w: self }
        }
        # [ doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_28od(&mut self) -> _P1_28ODW {
            _P1_28ODW { w: self }
        }
        # [ doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_29od(&mut self) -> _P1_29ODW {
            _P1_29ODW { w: self }
        }
        # [ doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD" ]
        # [ inline ( always ) ]
        pub fn p1_30od(&mut self) -> _P1_30ODW {
            _P1_30ODW { w: self }
        }
        # [ doc = "Bit 31 - Port 1 pin 31 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p1_31od(&mut self) -> _P1_31ODW {
            _P1_31ODW { w: self }
        }
    }
}
# [ doc = "Open drain mode control register 2" ]
pub struct PINMODE_OD2 {
    register: VolatileCell<u32>,
}
# [ doc = "Open drain mode control register 2" ]
pub mod pinmode_od2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE_OD2 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P2_00OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_00ODR {
        # [ doc = "Normal. P2.0 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.0 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_00ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_00ODR::NORMAL => false,
                P2_00ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_00ODR {
            match value {
                false => P2_00ODR::NORMAL,
                true => P2_00ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_00ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_00ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_01OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_01ODR {
        # [ doc = "Normal. P2.1 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.1p in is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_01ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_01ODR::NORMAL => false,
                P2_01ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_01ODR {
            match value {
                false => P2_01ODR::NORMAL,
                true => P2_01ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_01ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_01ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_02OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_02ODR {
        # [ doc = "Normal. P2.2 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.2 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_02ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_02ODR::NORMAL => false,
                P2_02ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_02ODR {
            match value {
                false => P2_02ODR::NORMAL,
                true => P2_02ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_02ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_02ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_03OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_03ODR {
        # [ doc = "Normal. P2.3 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.3 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_03ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_03ODR::NORMAL => false,
                P2_03ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_03ODR {
            match value {
                false => P2_03ODR::NORMAL,
                true => P2_03ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_03ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_03ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_04OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_04ODR {
        # [ doc = "Normal. P2.4 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.4 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_04ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_04ODR::NORMAL => false,
                P2_04ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_04ODR {
            match value {
                false => P2_04ODR::NORMAL,
                true => P2_04ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_04ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_04ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_05OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_05ODR {
        # [ doc = "Normal. P2.5 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.5 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_05ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_05ODR::NORMAL => false,
                P2_05ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_05ODR {
            match value {
                false => P2_05ODR::NORMAL,
                true => P2_05ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_05ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_05ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_06OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_06ODR {
        # [ doc = "Normal. P2.6 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.6 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_06ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_06ODR::NORMAL => false,
                P2_06ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_06ODR {
            match value {
                false => P2_06ODR::NORMAL,
                true => P2_06ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_06ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_06ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_07OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_07ODR {
        # [ doc = "Normal. P2.7 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.7 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_07ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_07ODR::NORMAL => false,
                P2_07ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_07ODR {
            match value {
                false => P2_07ODR::NORMAL,
                true => P2_07ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_07ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_07ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_08OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_08ODR {
        # [ doc = "Normal. P2.8 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.8 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_08ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_08ODR::NORMAL => false,
                P2_08ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_08ODR {
            match value {
                false => P2_08ODR::NORMAL,
                true => P2_08ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_08ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_08ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_09OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_09ODR {
        # [ doc = "Normal. P2.9 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.9 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_09ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_09ODR::NORMAL => false,
                P2_09ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_09ODR {
            match value {
                false => P2_09ODR::NORMAL,
                true => P2_09ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_09ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_09ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_10OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_10ODR {
        # [ doc = "Normal. P2.10 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.10 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_10ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_10ODR::NORMAL => false,
                P2_10ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_10ODR {
            match value {
                false => P2_10ODR::NORMAL,
                true => P2_10ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_10ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_10ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_11OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_11ODR {
        # [ doc = "Normal. P2.11 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.11 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_11ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_11ODR::NORMAL => false,
                P2_11ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_11ODR {
            match value {
                false => P2_11ODR::NORMAL,
                true => P2_11ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_11ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_11ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_12OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_12ODR {
        # [ doc = "Normal. P2.12 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.12 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_12ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_12ODR::NORMAL => false,
                P2_12ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_12ODR {
            match value {
                false => P2_12ODR::NORMAL,
                true => P2_12ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_12ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_12ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Possible values of the field `P2_13OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2_13ODR {
        # [ doc = "Normal. P2.13 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.13 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_13ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P2_13ODR::NORMAL => false,
                P2_13ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2_13ODR {
            match value {
                false => P2_13ODR::NORMAL,
                true => P2_13ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P2_13ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P2_13ODR::OPEN_DRAIN
        }
    }
    # [ doc = "Values that can be written to the field `P2_00OD`" ]
    pub enum P2_00ODW {
        # [ doc = "Normal. P2.0 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.0 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_00ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_00ODW::NORMAL => false,
                P2_00ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_00ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_00ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_00ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.0 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_00ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.0 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_00ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_01OD`" ]
    pub enum P2_01ODW {
        # [ doc = "Normal. P2.1 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.1p in is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_01ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_01ODW::NORMAL => false,
                P2_01ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_01ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_01ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_01ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.1 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_01ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.1p in is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_01ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_02OD`" ]
    pub enum P2_02ODW {
        # [ doc = "Normal. P2.2 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.2 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_02ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_02ODW::NORMAL => false,
                P2_02ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_02ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_02ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_02ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.2 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_02ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.2 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_02ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_03OD`" ]
    pub enum P2_03ODW {
        # [ doc = "Normal. P2.3 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.3 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_03ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_03ODW::NORMAL => false,
                P2_03ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_03ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_03ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_03ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.3 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_03ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.3 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_03ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_04OD`" ]
    pub enum P2_04ODW {
        # [ doc = "Normal. P2.4 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.4 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_04ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_04ODW::NORMAL => false,
                P2_04ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_04ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_04ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_04ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.4 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_04ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.4 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_04ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_05OD`" ]
    pub enum P2_05ODW {
        # [ doc = "Normal. P2.5 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.5 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_05ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_05ODW::NORMAL => false,
                P2_05ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_05ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_05ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_05ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.5 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_05ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.5 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_05ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_06OD`" ]
    pub enum P2_06ODW {
        # [ doc = "Normal. P2.6 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.6 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_06ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_06ODW::NORMAL => false,
                P2_06ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_06ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_06ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_06ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.6 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_06ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.6 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_06ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_07OD`" ]
    pub enum P2_07ODW {
        # [ doc = "Normal. P2.7 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.7 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_07ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_07ODW::NORMAL => false,
                P2_07ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_07ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_07ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_07ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.7 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_07ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.7 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_07ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_08OD`" ]
    pub enum P2_08ODW {
        # [ doc = "Normal. P2.8 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.8 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_08ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_08ODW::NORMAL => false,
                P2_08ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_08ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_08ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_08ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.8 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_08ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.8 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_08ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_09OD`" ]
    pub enum P2_09ODW {
        # [ doc = "Normal. P2.9 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.9 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_09ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_09ODW::NORMAL => false,
                P2_09ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_09ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_09ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_09ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.9 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_09ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.9 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_09ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_10OD`" ]
    pub enum P2_10ODW {
        # [ doc = "Normal. P2.10 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.10 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_10ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_10ODW::NORMAL => false,
                P2_10ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_10ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_10ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_10ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.10 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_10ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.10 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_10ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_11OD`" ]
    pub enum P2_11ODW {
        # [ doc = "Normal. P2.11 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.11 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_11ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_11ODW::NORMAL => false,
                P2_11ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_11ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_11ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_11ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.11 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_11ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.11 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_11ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_12OD`" ]
    pub enum P2_12ODW {
        # [ doc = "Normal. P2.12 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.12 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_12ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_12ODW::NORMAL => false,
                P2_12ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_12ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_12ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_12ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.12 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_12ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.12 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_12ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `P2_13OD`" ]
    pub enum P2_13ODW {
        # [ doc = "Normal. P2.13 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P2.13 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P2_13ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P2_13ODW::NORMAL => false,
                P2_13ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P2_13ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_13ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P2_13ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P2.13 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P2_13ODW::NORMAL)
        }
        # [ doc = "Open-drain. P2.13 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P2_13ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Port 2 pin 0 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p2_00od(&self) -> P2_00ODR {
            P2_00ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 0;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_01od(&self) -> P2_01ODR {
            P2_01ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 1;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_02od(&self) -> P2_02ODR {
            P2_02ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 2;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_03od(&self) -> P2_03ODR {
            P2_03ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 3;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_04od(&self) -> P2_04ODR {
            P2_04ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 4;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_05od(&self) -> P2_05ODR {
            P2_05ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 5;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_06od(&self) -> P2_06ODR {
            P2_06ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 6;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_07od(&self) -> P2_07ODR {
            P2_07ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 7;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_08od(&self) -> P2_08ODR {
            P2_08ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 8;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_09od(&self) -> P2_09ODR {
            P2_09ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 9;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_10od(&self) -> P2_10ODR {
            P2_10ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 10;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_11od(&self) -> P2_11ODR {
            P2_11ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 11;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_12od(&self) -> P2_12ODR {
            P2_12ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 12;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_13od(&self) -> P2_13ODR {
            P2_13ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 13;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Port 2 pin 0 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p2_00od(&mut self) -> _P2_00ODW {
            _P2_00ODW { w: self }
        }
        # [ doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_01od(&mut self) -> _P2_01ODW {
            _P2_01ODW { w: self }
        }
        # [ doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_02od(&mut self) -> _P2_02ODW {
            _P2_02ODW { w: self }
        }
        # [ doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_03od(&mut self) -> _P2_03ODW {
            _P2_03ODW { w: self }
        }
        # [ doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_04od(&mut self) -> _P2_04ODW {
            _P2_04ODW { w: self }
        }
        # [ doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_05od(&mut self) -> _P2_05ODW {
            _P2_05ODW { w: self }
        }
        # [ doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_06od(&mut self) -> _P2_06ODW {
            _P2_06ODW { w: self }
        }
        # [ doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_07od(&mut self) -> _P2_07ODW {
            _P2_07ODW { w: self }
        }
        # [ doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_08od(&mut self) -> _P2_08ODW {
            _P2_08ODW { w: self }
        }
        # [ doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_09od(&mut self) -> _P2_09ODW {
            _P2_09ODW { w: self }
        }
        # [ doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_10od(&mut self) -> _P2_10ODW {
            _P2_10ODW { w: self }
        }
        # [ doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_11od(&mut self) -> _P2_11ODW {
            _P2_11ODW { w: self }
        }
        # [ doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_12od(&mut self) -> _P2_12ODW {
            _P2_12ODW { w: self }
        }
        # [ doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD" ]
        # [ inline ( always ) ]
        pub fn p2_13od(&mut self) -> _P2_13ODW {
            _P2_13ODW { w: self }
        }
    }
}
# [ doc = "Open drain mode control register 3" ]
pub struct PINMODE_OD3 {
    register: VolatileCell<u32>,
}
# [ doc = "Open drain mode control register 3" ]
pub mod pinmode_od3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE_OD3 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P3_25OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P3_25ODR {
        # [ doc = "Normal. P3.25 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P3.25 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P3_25ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P3_25ODR::NORMAL => false,
                P3_25ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P3_25ODR {
            match value {
                false => P3_25ODR::NORMAL,
                true => P3_25ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P3_25ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P3_25ODR::OPEN_DRAIN
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct P3_26ODR {
        bits: bool,
    }
    impl P3_26ODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Values that can be written to the field `P3_25OD`" ]
    pub enum P3_25ODW {
        # [ doc = "Normal. P3.25 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P3.25 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P3_25ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P3_25ODW::NORMAL => false,
                P3_25ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P3_25ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P3_25ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P3_25ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P3.25 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P3_25ODW::NORMAL)
        }
        # [ doc = "Open-drain. P3.25 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P3_25ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P3_26ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P3_26ODW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 25 - Port 3 pin 25 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p3_25od(&self) -> P3_25ODR {
            P3_25ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 25;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD" ]
        # [ inline ( always ) ]
        pub fn p3_26od(&self) -> P3_26ODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P3_26ODR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 25 - Port 3 pin 25 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p3_25od(&mut self) -> _P3_25ODW {
            _P3_25ODW { w: self }
        }
        # [ doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD" ]
        # [ inline ( always ) ]
        pub fn p3_26od(&mut self) -> _P3_26ODW {
            _P3_26ODW { w: self }
        }
    }
}
# [ doc = "Open drain mode control register 4" ]
pub struct PINMODE_OD4 {
    register: VolatileCell<u32>,
}
# [ doc = "Open drain mode control register 4" ]
pub mod pinmode_od4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PINMODE_OD4 {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `P4_28OD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P4_28ODR {
        # [ doc = "Normal. P4.28 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P4.28 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P4_28ODR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                P4_28ODR::NORMAL => false,
                P4_28ODR::OPEN_DRAIN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P4_28ODR {
            match value {
                false => P4_28ODR::NORMAL,
                true => P4_28ODR::OPEN_DRAIN,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == P4_28ODR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OPEN_DRAIN`" ]
        # [ inline ( always ) ]
        pub fn is_open_drain(&self) -> bool {
            *self == P4_28ODR::OPEN_DRAIN
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct P4_29ODR {
        bits: bool,
    }
    impl P4_29ODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Values that can be written to the field `P4_28OD`" ]
    pub enum P4_28ODW {
        # [ doc = "Normal. P4.28 pin is in the normal (not open drain) mode." ]
        NORMAL,
        # [ doc = "Open-drain. P4.28 pin is in the open drain mode." ]
        OPEN_DRAIN,
    }
    impl P4_28ODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                P4_28ODW::NORMAL => false,
                P4_28ODW::OPEN_DRAIN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P4_28ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P4_28ODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: P4_28ODW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. P4.28 pin is in the normal (not open drain) mode." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(P4_28ODW::NORMAL)
        }
        # [ doc = "Open-drain. P4.28 pin is in the open drain mode." ]
        # [ inline ( always ) ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(P4_28ODW::OPEN_DRAIN)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _P4_29ODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P4_29ODW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        # [ doc = r" Clears the field bit" ]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 28 - Port 4 pin 28 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p4_28od(&self) -> P4_28ODR {
            P4_28ODR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 28;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD" ]
        # [ inline ( always ) ]
        pub fn p4_29od(&self) -> P4_29ODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P4_29ODR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 28 - Port 4 pin 28 open drain mode control." ]
        # [ inline ( always ) ]
        pub fn p4_28od(&mut self) -> _P4_28ODW {
            _P4_28ODW { w: self }
        }
        # [ doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD" ]
        # [ inline ( always ) ]
        pub fn p4_29od(&mut self) -> _P4_29ODW {
            _P4_29ODW { w: self }
        }
    }
}
# [ doc = "I2C Pin Configuration register" ]
pub struct I2CPADCFG {
    register: VolatileCell<u32>,
}
# [ doc = "I2C Pin Configuration register" ]
pub mod i2cpadcfg {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::I2CPADCFG {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = "Possible values of the field `SDADRV0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SDADRV0R {
        # [ doc = "Standard. The SDA0 pin is in the standard drive mode." ]
        STANDARD,
        # [ doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode." ]
        FAST_MODE_PLUS,
    }
    impl SDADRV0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                SDADRV0R::STANDARD => false,
                SDADRV0R::FAST_MODE_PLUS => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SDADRV0R {
            match value {
                false => SDADRV0R::STANDARD,
                true => SDADRV0R::FAST_MODE_PLUS,
            }
        }
        # [ doc = "Checks if the value of the field is `STANDARD`" ]
        # [ inline ( always ) ]
        pub fn is_standard(&self) -> bool {
            *self == SDADRV0R::STANDARD
        }
        # [ doc = "Checks if the value of the field is `FAST_MODE_PLUS`" ]
        # [ inline ( always ) ]
        pub fn is_fast_mode_plus(&self) -> bool {
            *self == SDADRV0R::FAST_MODE_PLUS
        }
    }
    # [ doc = "Possible values of the field `SDAI2C0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SDAI2C0R {
        # [ doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled." ]
        ENABLED,
        # [ doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled." ]
        DISABLED,
    }
    impl SDAI2C0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                SDAI2C0R::ENABLED => false,
                SDAI2C0R::DISABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SDAI2C0R {
            match value {
                false => SDAI2C0R::ENABLED,
                true => SDAI2C0R::DISABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == SDAI2C0R::ENABLED
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == SDAI2C0R::DISABLED
        }
    }
    # [ doc = "Possible values of the field `SCLDRV0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SCLDRV0R {
        # [ doc = "Standard. The SCL0 pin is in the standard drive mode." ]
        STANDARD,
        # [ doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode." ]
        FAST_MODE_PLUS,
    }
    impl SCLDRV0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                SCLDRV0R::STANDARD => false,
                SCLDRV0R::FAST_MODE_PLUS => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SCLDRV0R {
            match value {
                false => SCLDRV0R::STANDARD,
                true => SCLDRV0R::FAST_MODE_PLUS,
            }
        }
        # [ doc = "Checks if the value of the field is `STANDARD`" ]
        # [ inline ( always ) ]
        pub fn is_standard(&self) -> bool {
            *self == SCLDRV0R::STANDARD
        }
        # [ doc = "Checks if the value of the field is `FAST_MODE_PLUS`" ]
        # [ inline ( always ) ]
        pub fn is_fast_mode_plus(&self) -> bool {
            *self == SCLDRV0R::FAST_MODE_PLUS
        }
    }
    # [ doc = "Possible values of the field `SCLI2C0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SCLI2C0R {
        # [ doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled." ]
        ENABLED,
        # [ doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled." ]
        DISABLED,
    }
    impl SCLI2C0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            match *self {
                SCLI2C0R::ENABLED => false,
                SCLI2C0R::DISABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SCLI2C0R {
            match value {
                false => SCLI2C0R::ENABLED,
                true => SCLI2C0R::DISABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == SCLI2C0R::ENABLED
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == SCLI2C0R::DISABLED
        }
    }
    # [ doc = "Values that can be written to the field `SDADRV0`" ]
    pub enum SDADRV0W {
        # [ doc = "Standard. The SDA0 pin is in the standard drive mode." ]
        STANDARD,
        # [ doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode." ]
        FAST_MODE_PLUS,
    }
    impl SDADRV0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SDADRV0W::STANDARD => false,
                SDADRV0W::FAST_MODE_PLUS => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SDADRV0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SDADRV0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SDADRV0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Standard. The SDA0 pin is in the standard drive mode." ]
        # [ inline ( always ) ]
        pub fn standard(self) -> &'a mut W {
            self.variant(SDADRV0W::STANDARD)
        }
        # [ doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode." ]
        # [ inline ( always ) ]
        pub fn fast_mode_plus(self) -> &'a mut W {
            self.variant(SDADRV0W::FAST_MODE_PLUS)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `SDAI2C0`" ]
    pub enum SDAI2C0W {
        # [ doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled." ]
        ENABLED,
        # [ doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled." ]
        DISABLED,
    }
    impl SDAI2C0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SDAI2C0W::ENABLED => false,
                SDAI2C0W::DISABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SDAI2C0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SDAI2C0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SDAI2C0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(SDAI2C0W::ENABLED)
        }
        # [ doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(SDAI2C0W::DISABLED)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `SCLDRV0`" ]
    pub enum SCLDRV0W {
        # [ doc = "Standard. The SCL0 pin is in the standard drive mode." ]
        STANDARD,
        # [ doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode." ]
        FAST_MODE_PLUS,
    }
    impl SCLDRV0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SCLDRV0W::STANDARD => false,
                SCLDRV0W::FAST_MODE_PLUS => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SCLDRV0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SCLDRV0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SCLDRV0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Standard. The SCL0 pin is in the standard drive mode." ]
        # [ inline ( always ) ]
        pub fn standard(self) -> &'a mut W {
            self.variant(SCLDRV0W::STANDARD)
        }
        # [ doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode." ]
        # [ inline ( always ) ]
        pub fn fast_mode_plus(self) -> &'a mut W {
            self.variant(SCLDRV0W::FAST_MODE_PLUS)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `SCLI2C0`" ]
    pub enum SCLI2C0W {
        # [ doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled." ]
        ENABLED,
        # [ doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled." ]
        DISABLED,
    }
    impl SCLI2C0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SCLI2C0W::ENABLED => false,
                SCLI2C0W::DISABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SCLI2C0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SCLI2C0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SCLI2C0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(SCLI2C0W::ENABLED)
        }
        # [ doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(SCLI2C0W::DISABLED)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27." ]
        # [ inline ( always ) ]
        pub fn sdadrv0(&self) -> SDADRV0R {
            SDADRV0R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 0;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27." ]
        # [ inline ( always ) ]
        pub fn sdai2c0(&self) -> SDAI2C0R {
            SDAI2C0R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 1;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28." ]
        # [ inline ( always ) ]
        pub fn scldrv0(&self) -> SCLDRV0R {
            SCLDRV0R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 2;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28." ]
        # [ inline ( always ) ]
        pub fn scli2c0(&self) -> SCLI2C0R {
            SCLI2C0R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 3;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27." ]
        # [ inline ( always ) ]
        pub fn sdadrv0(&mut self) -> _SDADRV0W {
            _SDADRV0W { w: self }
        }
        # [ doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27." ]
        # [ inline ( always ) ]
        pub fn sdai2c0(&mut self) -> _SDAI2C0W {
            _SDAI2C0W { w: self }
        }
        # [ doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28." ]
        # [ inline ( always ) ]
        pub fn scldrv0(&mut self) -> _SCLDRV0W {
            _SCLDRV0W { w: self }
        }
        # [ doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28." ]
        # [ inline ( always ) ]
        pub fn scli2c0(&mut self) -> _SCLI2C0W {
            _SCLI2C0W { w: self }
        }
    }
}
# [ doc = "Pin connect block" ]
pub struct PINCONNECT {
    register_block: RegisterBlock,
}
impl Deref for PINCONNECT {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
