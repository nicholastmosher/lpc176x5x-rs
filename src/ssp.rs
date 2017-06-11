# ! [ doc = "SSP1 controller" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "SSP1 controller" ]
pub const SSP1: Peripheral<SSP1> = unsafe { Peripheral::new(1073938432) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Control Register 0. Selects the serial clock rate, bus type, and data size." ]
    pub cr0: CR0,
    # [ doc = "0x04 - Control Register 1. Selects master/slave and other modes." ]
    pub cr1: CR1,
    # [ doc = "0x08 - Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO." ]
    pub dr: DR,
    # [ doc = "0x0c - Status Register" ]
    pub sr: SR,
    # [ doc = "0x10 - Clock Prescale Register" ]
    pub cpsr: CPSR,
    # [ doc = "0x14 - Interrupt Mask Set and Clear Register" ]
    pub imsc: IMSC,
    # [ doc = "0x18 - Raw Interrupt Status Register" ]
    pub ris: RIS,
    # [ doc = "0x1c - Masked Interrupt Status Register" ]
    pub mis: MIS,
    # [ doc = "0x20 - SSPICR Interrupt Clear Register" ]
    pub icr: ICR,
    # [ doc = "0x24 - SSP0 DMA control register" ]
    pub dmacr: DMACR,
}
# [ doc = "Control Register 0. Selects the serial clock rate, bus type, and data size." ]
pub struct CR0 {
    register: VolatileCell<u32>,
}
# [ doc = "Control Register 0. Selects the serial clock rate, bus type, and data size." ]
pub mod cr0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CR0 {
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
    # [ doc = "Possible values of the field `DSS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DSSR {
        # [ doc = "4-bit transfer" ]
        _4_BIT_TRANSFER,
        # [ doc = "5-bit transfer" ]
        _5_BIT_TRANSFER,
        # [ doc = "6-bit transfer" ]
        _6_BIT_TRANSFER,
        # [ doc = "7-bit transfer" ]
        _7_BIT_TRANSFER,
        # [ doc = "8-bit transfer" ]
        _8_BIT_TRANSFER,
        # [ doc = "9-bit transfer" ]
        _9_BIT_TRANSFER,
        # [ doc = "10-bit transfer" ]
        _10_BIT_TRANSFER,
        # [ doc = "11-bit transfer" ]
        _11_BIT_TRANSFER,
        # [ doc = "12-bit transfer" ]
        _12_BIT_TRANSFER,
        # [ doc = "13-bit transfer" ]
        _13_BIT_TRANSFER,
        # [ doc = "14-bit transfer" ]
        _14_BIT_TRANSFER,
        # [ doc = "15-bit transfer" ]
        _15_BIT_TRANSFER,
        # [ doc = "16-bit transfer" ]
        _16_BIT_TRANSFER,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl DSSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                DSSR::_4_BIT_TRANSFER => 3,
                DSSR::_5_BIT_TRANSFER => 4,
                DSSR::_6_BIT_TRANSFER => 5,
                DSSR::_7_BIT_TRANSFER => 6,
                DSSR::_8_BIT_TRANSFER => 7,
                DSSR::_9_BIT_TRANSFER => 8,
                DSSR::_10_BIT_TRANSFER => 9,
                DSSR::_11_BIT_TRANSFER => 10,
                DSSR::_12_BIT_TRANSFER => 11,
                DSSR::_13_BIT_TRANSFER => 12,
                DSSR::_14_BIT_TRANSFER => 13,
                DSSR::_15_BIT_TRANSFER => 14,
                DSSR::_16_BIT_TRANSFER => 15,
                DSSR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> DSSR {
            match value {
                3 => DSSR::_4_BIT_TRANSFER,
                4 => DSSR::_5_BIT_TRANSFER,
                5 => DSSR::_6_BIT_TRANSFER,
                6 => DSSR::_7_BIT_TRANSFER,
                7 => DSSR::_8_BIT_TRANSFER,
                8 => DSSR::_9_BIT_TRANSFER,
                9 => DSSR::_10_BIT_TRANSFER,
                10 => DSSR::_11_BIT_TRANSFER,
                11 => DSSR::_12_BIT_TRANSFER,
                12 => DSSR::_13_BIT_TRANSFER,
                13 => DSSR::_14_BIT_TRANSFER,
                14 => DSSR::_15_BIT_TRANSFER,
                15 => DSSR::_16_BIT_TRANSFER,
                i => DSSR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `_4_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_4_bit_transfer(&self) -> bool {
            *self == DSSR::_4_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_5_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_5_bit_transfer(&self) -> bool {
            *self == DSSR::_5_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_6_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_6_bit_transfer(&self) -> bool {
            *self == DSSR::_6_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_7_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_7_bit_transfer(&self) -> bool {
            *self == DSSR::_7_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_8_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_8_bit_transfer(&self) -> bool {
            *self == DSSR::_8_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_9_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_9_bit_transfer(&self) -> bool {
            *self == DSSR::_9_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_10_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_10_bit_transfer(&self) -> bool {
            *self == DSSR::_10_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_11_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_11_bit_transfer(&self) -> bool {
            *self == DSSR::_11_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_12_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_12_bit_transfer(&self) -> bool {
            *self == DSSR::_12_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_13_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_13_bit_transfer(&self) -> bool {
            *self == DSSR::_13_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_14_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_14_bit_transfer(&self) -> bool {
            *self == DSSR::_14_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_15_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_15_bit_transfer(&self) -> bool {
            *self == DSSR::_15_BIT_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_16_BIT_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_16_bit_transfer(&self) -> bool {
            *self == DSSR::_16_BIT_TRANSFER
        }
    }
    # [ doc = "Possible values of the field `FRF`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum FRFR {
        # [ doc = "SPI" ]
        SPI,
        # [ doc = "TI" ]
        TI,
        # [ doc = "Microwire" ]
        MICROWIRE,
        # [ doc = "This combination is not supported and should not be used." ]
        THIS_COMBINATION_IS_,
    }
    impl FRFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                FRFR::SPI => 0,
                FRFR::TI => 1,
                FRFR::MICROWIRE => 2,
                FRFR::THIS_COMBINATION_IS_ => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> FRFR {
            match value {
                0 => FRFR::SPI,
                1 => FRFR::TI,
                2 => FRFR::MICROWIRE,
                3 => FRFR::THIS_COMBINATION_IS_,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `SPI`" ]
        # [ inline ( always ) ]
        pub fn is_spi(&self) -> bool {
            *self == FRFR::SPI
        }
        # [ doc = "Checks if the value of the field is `TI`" ]
        # [ inline ( always ) ]
        pub fn is_ti(&self) -> bool {
            *self == FRFR::TI
        }
        # [ doc = "Checks if the value of the field is `MICROWIRE`" ]
        # [ inline ( always ) ]
        pub fn is_microwire(&self) -> bool {
            *self == FRFR::MICROWIRE
        }
        # [ doc = "Checks if the value of the field is `THIS_COMBINATION_IS_`" ]
        # [ inline ( always ) ]
        pub fn is_this_combination_is_(&self) -> bool {
            *self == FRFR::THIS_COMBINATION_IS_
        }
    }
    # [ doc = "Possible values of the field `CPOL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CPOLR {
        # [ doc = "SSP controller maintains the bus clock low between frames." ]
        BUS_LOW,
        # [ doc = "SSP controller maintains the bus clock high between frames." ]
        BUS_HIGH,
    }
    impl CPOLR {
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
                CPOLR::BUS_LOW => false,
                CPOLR::BUS_HIGH => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CPOLR {
            match value {
                false => CPOLR::BUS_LOW,
                true => CPOLR::BUS_HIGH,
            }
        }
        # [ doc = "Checks if the value of the field is `BUS_LOW`" ]
        # [ inline ( always ) ]
        pub fn is_bus_low(&self) -> bool {
            *self == CPOLR::BUS_LOW
        }
        # [ doc = "Checks if the value of the field is `BUS_HIGH`" ]
        # [ inline ( always ) ]
        pub fn is_bus_high(&self) -> bool {
            *self == CPOLR::BUS_HIGH
        }
    }
    # [ doc = "Possible values of the field `CPHA`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CPHAR {
        # [ doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line." ]
        FIRST_CLOCK,
        # [ doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line." ]
        SECOND_CLOCK,
    }
    impl CPHAR {
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
                CPHAR::FIRST_CLOCK => false,
                CPHAR::SECOND_CLOCK => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CPHAR {
            match value {
                false => CPHAR::FIRST_CLOCK,
                true => CPHAR::SECOND_CLOCK,
            }
        }
        # [ doc = "Checks if the value of the field is `FIRST_CLOCK`" ]
        # [ inline ( always ) ]
        pub fn is_first_clock(&self) -> bool {
            *self == CPHAR::FIRST_CLOCK
        }
        # [ doc = "Checks if the value of the field is `SECOND_CLOCK`" ]
        # [ inline ( always ) ]
        pub fn is_second_clock(&self) -> bool {
            *self == CPHAR::SECOND_CLOCK
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SCRR {
        bits: u8,
    }
    impl SCRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Values that can be written to the field `DSS`" ]
    pub enum DSSW {
        # [ doc = "4-bit transfer" ]
        _4_BIT_TRANSFER,
        # [ doc = "5-bit transfer" ]
        _5_BIT_TRANSFER,
        # [ doc = "6-bit transfer" ]
        _6_BIT_TRANSFER,
        # [ doc = "7-bit transfer" ]
        _7_BIT_TRANSFER,
        # [ doc = "8-bit transfer" ]
        _8_BIT_TRANSFER,
        # [ doc = "9-bit transfer" ]
        _9_BIT_TRANSFER,
        # [ doc = "10-bit transfer" ]
        _10_BIT_TRANSFER,
        # [ doc = "11-bit transfer" ]
        _11_BIT_TRANSFER,
        # [ doc = "12-bit transfer" ]
        _12_BIT_TRANSFER,
        # [ doc = "13-bit transfer" ]
        _13_BIT_TRANSFER,
        # [ doc = "14-bit transfer" ]
        _14_BIT_TRANSFER,
        # [ doc = "15-bit transfer" ]
        _15_BIT_TRANSFER,
        # [ doc = "16-bit transfer" ]
        _16_BIT_TRANSFER,
    }
    impl DSSW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                DSSW::_4_BIT_TRANSFER => 3,
                DSSW::_5_BIT_TRANSFER => 4,
                DSSW::_6_BIT_TRANSFER => 5,
                DSSW::_7_BIT_TRANSFER => 6,
                DSSW::_8_BIT_TRANSFER => 7,
                DSSW::_9_BIT_TRANSFER => 8,
                DSSW::_10_BIT_TRANSFER => 9,
                DSSW::_11_BIT_TRANSFER => 10,
                DSSW::_12_BIT_TRANSFER => 11,
                DSSW::_13_BIT_TRANSFER => 12,
                DSSW::_14_BIT_TRANSFER => 13,
                DSSW::_15_BIT_TRANSFER => 14,
                DSSW::_16_BIT_TRANSFER => 15,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DSSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DSSW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: DSSW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "4-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _4_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_4_BIT_TRANSFER)
        }
        # [ doc = "5-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _5_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_5_BIT_TRANSFER)
        }
        # [ doc = "6-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _6_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_6_BIT_TRANSFER)
        }
        # [ doc = "7-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _7_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_7_BIT_TRANSFER)
        }
        # [ doc = "8-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _8_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_8_BIT_TRANSFER)
        }
        # [ doc = "9-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _9_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_9_BIT_TRANSFER)
        }
        # [ doc = "10-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _10_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_10_BIT_TRANSFER)
        }
        # [ doc = "11-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _11_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_11_BIT_TRANSFER)
        }
        # [ doc = "12-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _12_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_12_BIT_TRANSFER)
        }
        # [ doc = "13-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _13_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_13_BIT_TRANSFER)
        }
        # [ doc = "14-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _14_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_14_BIT_TRANSFER)
        }
        # [ doc = "15-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _15_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_15_BIT_TRANSFER)
        }
        # [ doc = "16-bit transfer" ]
        # [ inline ( always ) ]
        pub fn _16_bit_transfer(self) -> &'a mut W {
            self.variant(DSSW::_16_BIT_TRANSFER)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `FRF`" ]
    pub enum FRFW {
        # [ doc = "SPI" ]
        SPI,
        # [ doc = "TI" ]
        TI,
        # [ doc = "Microwire" ]
        MICROWIRE,
        # [ doc = "This combination is not supported and should not be used." ]
        THIS_COMBINATION_IS_,
    }
    impl FRFW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                FRFW::SPI => 0,
                FRFW::TI => 1,
                FRFW::MICROWIRE => 2,
                FRFW::THIS_COMBINATION_IS_ => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FRFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FRFW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: FRFW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "SPI" ]
        # [ inline ( always ) ]
        pub fn spi(self) -> &'a mut W {
            self.variant(FRFW::SPI)
        }
        # [ doc = "TI" ]
        # [ inline ( always ) ]
        pub fn ti(self) -> &'a mut W {
            self.variant(FRFW::TI)
        }
        # [ doc = "Microwire" ]
        # [ inline ( always ) ]
        pub fn microwire(self) -> &'a mut W {
            self.variant(FRFW::MICROWIRE)
        }
        # [ doc = "This combination is not supported and should not be used." ]
        # [ inline ( always ) ]
        pub fn this_combination_is_(self) -> &'a mut W {
            self.variant(FRFW::THIS_COMBINATION_IS_)
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
    # [ doc = "Values that can be written to the field `CPOL`" ]
    pub enum CPOLW {
        # [ doc = "SSP controller maintains the bus clock low between frames." ]
        BUS_LOW,
        # [ doc = "SSP controller maintains the bus clock high between frames." ]
        BUS_HIGH,
    }
    impl CPOLW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CPOLW::BUS_LOW => false,
                CPOLW::BUS_HIGH => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CPOLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CPOLW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CPOLW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "SSP controller maintains the bus clock low between frames." ]
        # [ inline ( always ) ]
        pub fn bus_low(self) -> &'a mut W {
            self.variant(CPOLW::BUS_LOW)
        }
        # [ doc = "SSP controller maintains the bus clock high between frames." ]
        # [ inline ( always ) ]
        pub fn bus_high(self) -> &'a mut W {
            self.variant(CPOLW::BUS_HIGH)
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
    # [ doc = "Values that can be written to the field `CPHA`" ]
    pub enum CPHAW {
        # [ doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line." ]
        FIRST_CLOCK,
        # [ doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line." ]
        SECOND_CLOCK,
    }
    impl CPHAW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CPHAW::FIRST_CLOCK => false,
                CPHAW::SECOND_CLOCK => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CPHAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CPHAW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CPHAW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line." ]
        # [ inline ( always ) ]
        pub fn first_clock(self) -> &'a mut W {
            self.variant(CPHAW::FIRST_CLOCK)
        }
        # [ doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line." ]
        # [ inline ( always ) ]
        pub fn second_clock(self) -> &'a mut W {
            self.variant(CPHAW::SECOND_CLOCK)
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
    # [ doc = r" Proxy" ]
    pub struct _SCRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SCRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
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
        # [ doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used." ]
        # [ inline ( always ) ]
        pub fn dss(&self) -> DSSR {
            DSSR::_from({
                            const MASK: u8 = 15;
                            const OFFSET: u8 = 0;
                            ((self.bits >> OFFSET) & MASK as u32) as u8
                        })
        }
        # [ doc = "Bits 4:5 - Frame Format." ]
        # [ inline ( always ) ]
        pub fn frf(&self) -> FRFR {
            FRFR::_from({
                            const MASK: u8 = 3;
                            const OFFSET: u8 = 4;
                            ((self.bits >> OFFSET) & MASK as u32) as u8
                        })
        }
        # [ doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode." ]
        # [ inline ( always ) ]
        pub fn cpol(&self) -> CPOLR {
            CPOLR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 6;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode." ]
        # [ inline ( always ) ]
        pub fn cpha(&self) -> CPHAR {
            CPHAR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 7;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X [SCR+1])." ]
        # [ inline ( always ) ]
        pub fn scr(&self) -> SCRR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SCRR { bits }
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
        # [ doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used." ]
        # [ inline ( always ) ]
        pub fn dss(&mut self) -> _DSSW {
            _DSSW { w: self }
        }
        # [ doc = "Bits 4:5 - Frame Format." ]
        # [ inline ( always ) ]
        pub fn frf(&mut self) -> _FRFW {
            _FRFW { w: self }
        }
        # [ doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode." ]
        # [ inline ( always ) ]
        pub fn cpol(&mut self) -> _CPOLW {
            _CPOLW { w: self }
        }
        # [ doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode." ]
        # [ inline ( always ) ]
        pub fn cpha(&mut self) -> _CPHAW {
            _CPHAW { w: self }
        }
        # [ doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X [SCR+1])." ]
        # [ inline ( always ) ]
        pub fn scr(&mut self) -> _SCRW {
            _SCRW { w: self }
        }
    }
}
# [ doc = "Control Register 1. Selects master/slave and other modes." ]
pub struct CR1 {
    register: VolatileCell<u32>,
}
# [ doc = "Control Register 1. Selects master/slave and other modes." ]
pub mod cr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CR1 {
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
    # [ doc = "Possible values of the field `LBM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum LBMR {
        # [ doc = "During normal operation." ]
        NORMAL,
        # [ doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)." ]
        OUPTU,
    }
    impl LBMR {
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
                LBMR::NORMAL => false,
                LBMR::OUPTU => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> LBMR {
            match value {
                false => LBMR::NORMAL,
                true => LBMR::OUPTU,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL`" ]
        # [ inline ( always ) ]
        pub fn is_normal(&self) -> bool {
            *self == LBMR::NORMAL
        }
        # [ doc = "Checks if the value of the field is `OUPTU`" ]
        # [ inline ( always ) ]
        pub fn is_ouptu(&self) -> bool {
            *self == LBMR::OUPTU
        }
    }
    # [ doc = "Possible values of the field `SSE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SSER {
        # [ doc = "The SSP controller is disabled." ]
        DISABLED,
        # [ doc = "The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit." ]
        ENABLED,
    }
    impl SSER {
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
                SSER::DISABLED => false,
                SSER::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SSER {
            match value {
                false => SSER::DISABLED,
                true => SSER::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == SSER::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == SSER::ENABLED
        }
    }
    # [ doc = "Possible values of the field `MS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MSR {
        # [ doc = "The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line." ]
        MASTER,
        # [ doc = "The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines." ]
        SLAVE,
    }
    impl MSR {
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
                MSR::MASTER => false,
                MSR::SLAVE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MSR {
            match value {
                false => MSR::MASTER,
                true => MSR::SLAVE,
            }
        }
        # [ doc = "Checks if the value of the field is `MASTER`" ]
        # [ inline ( always ) ]
        pub fn is_master(&self) -> bool {
            *self == MSR::MASTER
        }
        # [ doc = "Checks if the value of the field is `SLAVE`" ]
        # [ inline ( always ) ]
        pub fn is_slave(&self) -> bool {
            *self == MSR::SLAVE
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SODR {
        bits: bool,
    }
    impl SODR {
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
    # [ doc = "Values that can be written to the field `LBM`" ]
    pub enum LBMW {
        # [ doc = "During normal operation." ]
        NORMAL,
        # [ doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)." ]
        OUPTU,
    }
    impl LBMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                LBMW::NORMAL => false,
                LBMW::OUPTU => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LBMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LBMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: LBMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "During normal operation." ]
        # [ inline ( always ) ]
        pub fn normal(self) -> &'a mut W {
            self.variant(LBMW::NORMAL)
        }
        # [ doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)." ]
        # [ inline ( always ) ]
        pub fn ouptu(self) -> &'a mut W {
            self.variant(LBMW::OUPTU)
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
    # [ doc = "Values that can be written to the field `SSE`" ]
    pub enum SSEW {
        # [ doc = "The SSP controller is disabled." ]
        DISABLED,
        # [ doc = "The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit." ]
        ENABLED,
    }
    impl SSEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SSEW::DISABLED => false,
                SSEW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SSEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SSEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SSEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The SSP controller is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(SSEW::DISABLED)
        }
        # [ doc = "The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(SSEW::ENABLED)
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
    # [ doc = "Values that can be written to the field `MS`" ]
    pub enum MSW {
        # [ doc = "The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line." ]
        MASTER,
        # [ doc = "The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines." ]
        SLAVE,
    }
    impl MSW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MSW::MASTER => false,
                MSW::SLAVE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MSW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MSW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line." ]
        # [ inline ( always ) ]
        pub fn master(self) -> &'a mut W {
            self.variant(MSW::MASTER)
        }
        # [ doc = "The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines." ]
        # [ inline ( always ) ]
        pub fn slave(self) -> &'a mut W {
            self.variant(MSW::SLAVE)
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
    # [ doc = r" Proxy" ]
    pub struct _SODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SODW<'a> {
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
        # [ doc = "Bit 0 - Loop Back Mode." ]
        # [ inline ( always ) ]
        pub fn lbm(&self) -> LBMR {
            LBMR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 0;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 1 - SSP Enable." ]
        # [ inline ( always ) ]
        pub fn sse(&self) -> SSER {
            SSER::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 1;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0." ]
        # [ inline ( always ) ]
        pub fn ms(&self) -> MSR {
            MSR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 2;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)." ]
        # [ inline ( always ) ]
        pub fn sod(&self) -> SODR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SODR { bits }
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
        # [ doc = "Bit 0 - Loop Back Mode." ]
        # [ inline ( always ) ]
        pub fn lbm(&mut self) -> _LBMW {
            _LBMW { w: self }
        }
        # [ doc = "Bit 1 - SSP Enable." ]
        # [ inline ( always ) ]
        pub fn sse(&mut self) -> _SSEW {
            _SSEW { w: self }
        }
        # [ doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0." ]
        # [ inline ( always ) ]
        pub fn ms(&mut self) -> _MSW {
            _MSW { w: self }
        }
        # [ doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)." ]
        # [ inline ( always ) ]
        pub fn sod(&mut self) -> _SODW {
            _SODW { w: self }
        }
    }
}
# [ doc = "Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO." ]
pub struct DR {
    register: VolatileCell<u32>,
}
# [ doc = "Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO." ]
pub mod dr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DR {
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
    # [ doc = r" Value of the field" ]
    pub struct DATAR {
        bits: u16,
    }
    impl DATAR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATAW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
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
        # [ doc = "Bits 0:15 - Write: software can write data to be sent in a future frame to this register whenever the TNF bit in the Status register is 1, indicating that the Tx FIFO is not full. If the Tx FIFO was previously empty and the SSP controller is not busy on the bus, transmission of the data will begin immediately. Otherwise the data written to this register will be sent as soon as all previous data has been sent (and received). If the data length is less than 16 bits, software must right-justify the data written to this register. Read: software can read data from this register whenever the RNE bit in the Status register is 1, indicating that the Rx FIFO is not empty. When software reads this register, the SSP controller returns data from the least recent frame in the Rx FIFO. If the data length is less than 16 bits, the data is right-justified in this field with higher order bits filled with 0s." ]
        # [ inline ( always ) ]
        pub fn data(&self) -> DATAR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DATAR { bits }
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
        # [ doc = "Bits 0:15 - Write: software can write data to be sent in a future frame to this register whenever the TNF bit in the Status register is 1, indicating that the Tx FIFO is not full. If the Tx FIFO was previously empty and the SSP controller is not busy on the bus, transmission of the data will begin immediately. Otherwise the data written to this register will be sent as soon as all previous data has been sent (and received). If the data length is less than 16 bits, software must right-justify the data written to this register. Read: software can read data from this register whenever the RNE bit in the Status register is 1, indicating that the Rx FIFO is not empty. When software reads this register, the SSP controller returns data from the least recent frame in the Rx FIFO. If the data length is less than 16 bits, the data is right-justified in this field with higher order bits filled with 0s." ]
        # [ inline ( always ) ]
        pub fn data(&mut self) -> _DATAW {
            _DATAW { w: self }
        }
    }
}
# [ doc = "Status Register" ]
pub struct SR {
    register: VolatileCell<u32>,
}
# [ doc = "Status Register" ]
pub mod sr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::SR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TFER {
        bits: bool,
    }
    impl TFER {
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
    # [ doc = r" Value of the field" ]
    pub struct TNFR {
        bits: bool,
    }
    impl TNFR {
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
    # [ doc = r" Value of the field" ]
    pub struct RNER {
        bits: bool,
    }
    impl RNER {
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
    # [ doc = r" Value of the field" ]
    pub struct RFFR {
        bits: bool,
    }
    impl RFFR {
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
    # [ doc = r" Value of the field" ]
    pub struct BSYR {
        bits: bool,
    }
    impl BSYR {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not." ]
        # [ inline ( always ) ]
        pub fn tfe(&self) -> TFER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TFER { bits }
        }
        # [ doc = "Bit 1 - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not." ]
        # [ inline ( always ) ]
        pub fn tnf(&self) -> TNFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TNFR { bits }
        }
        # [ doc = "Bit 2 - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not." ]
        # [ inline ( always ) ]
        pub fn rne(&self) -> RNER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RNER { bits }
        }
        # [ doc = "Bit 3 - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not." ]
        # [ inline ( always ) ]
        pub fn rff(&self) -> RFFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RFFR { bits }
        }
        # [ doc = "Bit 4 - Busy. This bit is 0 if the SSPn controller is idle, or 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty." ]
        # [ inline ( always ) ]
        pub fn bsy(&self) -> BSYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BSYR { bits }
        }
    }
}
# [ doc = "Clock Prescale Register" ]
pub struct CPSR {
    register: VolatileCell<u32>,
}
# [ doc = "Clock Prescale Register" ]
pub mod cpsr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CPSR {
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
    # [ doc = r" Value of the field" ]
    pub struct CPSDVSRR {
        bits: u8,
    }
    impl CPSDVSRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CPSDVSRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CPSDVSRW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
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
        # [ doc = "Bits 0:7 - This even value between 2 and 254, by which PCLK is divided to yield the prescaler output clock. Bit 0 always reads as 0." ]
        # [ inline ( always ) ]
        pub fn cpsdvsr(&self) -> CPSDVSRR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CPSDVSRR { bits }
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
        # [ doc = "Bits 0:7 - This even value between 2 and 254, by which PCLK is divided to yield the prescaler output clock. Bit 0 always reads as 0." ]
        # [ inline ( always ) ]
        pub fn cpsdvsr(&mut self) -> _CPSDVSRW {
            _CPSDVSRW { w: self }
        }
    }
}
# [ doc = "Interrupt Mask Set and Clear Register" ]
pub struct IMSC {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt Mask Set and Clear Register" ]
pub mod imsc {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::IMSC {
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
    # [ doc = r" Value of the field" ]
    pub struct RORIMR {
        bits: bool,
    }
    impl RORIMR {
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
    # [ doc = r" Value of the field" ]
    pub struct RTIMR {
        bits: bool,
    }
    impl RTIMR {
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
    # [ doc = r" Value of the field" ]
    pub struct RXIMR {
        bits: bool,
    }
    impl RXIMR {
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
    # [ doc = r" Value of the field" ]
    pub struct TXIMR {
        bits: bool,
    }
    impl TXIMR {
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
    # [ doc = r" Proxy" ]
    pub struct _RORIMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RORIMW<'a> {
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
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RTIMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RTIMW<'a> {
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
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RXIMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXIMW<'a> {
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
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TXIMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXIMW<'a> {
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
        # [ doc = "Bit 0 - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs." ]
        # [ inline ( always ) ]
        pub fn rorim(&self) -> RORIMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RORIMR { bits }
        }
        # [ doc = "Bit 1 - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
        # [ inline ( always ) ]
        pub fn rtim(&self) -> RTIMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RTIMR { bits }
        }
        # [ doc = "Bit 2 - Software should set this bit to enable interrupt when the Rx FIFO is at least half full." ]
        # [ inline ( always ) ]
        pub fn rxim(&self) -> RXIMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RXIMR { bits }
        }
        # [ doc = "Bit 3 - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty." ]
        # [ inline ( always ) ]
        pub fn txim(&self) -> TXIMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TXIMR { bits }
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
        # [ doc = "Bit 0 - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs." ]
        # [ inline ( always ) ]
        pub fn rorim(&mut self) -> _RORIMW {
            _RORIMW { w: self }
        }
        # [ doc = "Bit 1 - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
        # [ inline ( always ) ]
        pub fn rtim(&mut self) -> _RTIMW {
            _RTIMW { w: self }
        }
        # [ doc = "Bit 2 - Software should set this bit to enable interrupt when the Rx FIFO is at least half full." ]
        # [ inline ( always ) ]
        pub fn rxim(&mut self) -> _RXIMW {
            _RXIMW { w: self }
        }
        # [ doc = "Bit 3 - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty." ]
        # [ inline ( always ) ]
        pub fn txim(&mut self) -> _TXIMW {
            _TXIMW { w: self }
        }
    }
}
# [ doc = "Raw Interrupt Status Register" ]
pub struct RIS {
    register: VolatileCell<u32>,
}
# [ doc = "Raw Interrupt Status Register" ]
pub mod ris {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::RIS {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RORRISR {
        bits: bool,
    }
    impl RORRISR {
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
    # [ doc = r" Value of the field" ]
    pub struct RTRISR {
        bits: bool,
    }
    impl RTRISR {
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
    # [ doc = r" Value of the field" ]
    pub struct RXRISR {
        bits: bool,
    }
    impl RXRISR {
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
    # [ doc = r" Value of the field" ]
    pub struct TXRISR {
        bits: bool,
    }
    impl TXRISR {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs." ]
        # [ inline ( always ) ]
        pub fn rorris(&self) -> RORRISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RORRISR { bits }
        }
        # [ doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, and has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
        # [ inline ( always ) ]
        pub fn rtris(&self) -> RTRISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RTRISR { bits }
        }
        # [ doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full." ]
        # [ inline ( always ) ]
        pub fn rxris(&self) -> RXRISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RXRISR { bits }
        }
        # [ doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty." ]
        # [ inline ( always ) ]
        pub fn txris(&self) -> TXRISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TXRISR { bits }
        }
    }
}
# [ doc = "Masked Interrupt Status Register" ]
pub struct MIS {
    register: VolatileCell<u32>,
}
# [ doc = "Masked Interrupt Status Register" ]
pub mod mis {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::MIS {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RORMISR {
        bits: bool,
    }
    impl RORMISR {
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
    # [ doc = r" Value of the field" ]
    pub struct RTMISR {
        bits: bool,
    }
    impl RTMISR {
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
    # [ doc = r" Value of the field" ]
    pub struct RXMISR {
        bits: bool,
    }
    impl RXMISR {
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
    # [ doc = r" Value of the field" ]
    pub struct TXMISR {
        bits: bool,
    }
    impl TXMISR {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full, and this interrupt is enabled." ]
        # [ inline ( always ) ]
        pub fn rormis(&self) -> RORMISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RORMISR { bits }
        }
        # [ doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, has not been read for a time-out period, and this interrupt is enabled. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
        # [ inline ( always ) ]
        pub fn rtmis(&self) -> RTMISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RTMISR { bits }
        }
        # [ doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full, and this interrupt is enabled." ]
        # [ inline ( always ) ]
        pub fn rxmis(&self) -> RXMISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RXMISR { bits }
        }
        # [ doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty, and this interrupt is enabled." ]
        # [ inline ( always ) ]
        pub fn txmis(&self) -> TXMISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TXMISR { bits }
        }
    }
}
# [ doc = "SSPICR Interrupt Clear Register" ]
pub struct ICR {
    register: VolatileCell<u32>,
}
# [ doc = "SSPICR Interrupt Clear Register" ]
pub mod icr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ICR {
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RORICW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RORICW<'a> {
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
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RTICW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RTICW<'a> {
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
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
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
        # [ doc = "Bit 0 - Writing a 1 to this bit clears the frame was received when RxFIFO was full interrupt." ]
        # [ inline ( always ) ]
        pub fn roric(&mut self) -> _RORICW {
            _RORICW { w: self }
        }
        # [ doc = "Bit 1 - Writing a 1 to this bit clears the Rx FIFO was not empty and has not been read for a time-out period interrupt. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR / [SCR+1])." ]
        # [ inline ( always ) ]
        pub fn rtic(&mut self) -> _RTICW {
            _RTICW { w: self }
        }
    }
}
# [ doc = "SSP0 DMA control register" ]
pub struct DMACR {
    register: VolatileCell<u32>,
}
# [ doc = "SSP0 DMA control register" ]
pub mod dmacr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DMACR {
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
    # [ doc = r" Value of the field" ]
    pub struct RXDMAER {
        bits: bool,
    }
    impl RXDMAER {
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
    # [ doc = r" Value of the field" ]
    pub struct TXDMAER {
        bits: bool,
    }
    impl TXDMAER {
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
    # [ doc = r" Proxy" ]
    pub struct _RXDMAEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXDMAEW<'a> {
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
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TXDMAEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXDMAEW<'a> {
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
            const OFFSET: u8 = 1;
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
        # [ doc = "Bit 0 - Receive DMA Enable. When this bit is set to one 1, DMA for the receive FIFO is enabled, otherwise receive DMA is disabled." ]
        # [ inline ( always ) ]
        pub fn rxdmae(&self) -> RXDMAER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RXDMAER { bits }
        }
        # [ doc = "Bit 1 - Transmit DMA Enable. When this bit is set to one 1, DMA for the transmit FIFO is enabled, otherwise transmit DMA is disabled" ]
        # [ inline ( always ) ]
        pub fn txdmae(&self) -> TXDMAER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TXDMAER { bits }
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
        # [ doc = "Bit 0 - Receive DMA Enable. When this bit is set to one 1, DMA for the receive FIFO is enabled, otherwise receive DMA is disabled." ]
        # [ inline ( always ) ]
        pub fn rxdmae(&mut self) -> _RXDMAEW {
            _RXDMAEW { w: self }
        }
        # [ doc = "Bit 1 - Transmit DMA Enable. When this bit is set to one 1, DMA for the transmit FIFO is enabled, otherwise transmit DMA is disabled" ]
        # [ inline ( always ) ]
        pub fn txdmae(&mut self) -> _TXDMAEW {
            _TXDMAEW { w: self }
        }
    }
}
# [ doc = "SSP controller" ]
pub const SSP0: Peripheral<SSP0> = unsafe { Peripheral::new(1074298880) };
# [ doc = r" Register block" ]
pub struct SSP0 {
    register_block: RegisterBlock,
}
impl Deref for SSP0 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "SSP1 controller" ]
pub struct SSP1 {
    register_block: RegisterBlock,
}
impl Deref for SSP1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
