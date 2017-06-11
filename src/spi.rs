# ! [ doc = "SPI" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "SPI" ]
pub const SPI: Peripheral<SPI> = unsafe { Peripheral::new(1073872896) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - SPI Control Register. This register controls the operation of the SPI." ]
    pub spicr: SPICR,
    # [ doc = "0x04 - SPI Status Register. This register shows the status of the SPI." ]
    pub sr: SR,
    # [ doc = "0x08 - SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register." ]
    pub dr: DR,
    # [ doc = "0x0c - SPI Clock Counter Register. This register controls the frequency of a master's SCK0." ]
    pub ccr: CCR,
    _reserved0: [u8; 12usize],
    # [ doc = "0x1c - SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface." ]
    pub int: INT,
}
# [ doc = "SPI Control Register. This register controls the operation of the SPI." ]
pub struct SPICR {
    register: VolatileCell<u32>,
}
# [ doc = "SPI Control Register. This register controls the operation of the SPI." ]
pub mod spicr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SPICR {
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
    # [ doc = "Possible values of the field `BITENABLE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum BITENABLER {
        # [ doc = "The SPI controller sends and receives the number of bits selected by bits 11:8." ]
        THE_SPI_CONTROLLER_S,
        # [ doc = r" Reserved" ]
        _Reserved(bool),
    }
    impl BITENABLER {
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
                BITENABLER::THE_SPI_CONTROLLER_S => true,
                BITENABLER::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> BITENABLER {
            match value {
                true => BITENABLER::THE_SPI_CONTROLLER_S,
                i => BITENABLER::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `THE_SPI_CONTROLLER_S`" ]
        # [ inline ( always ) ]
        pub fn is_the_spi_controller_s(&self) -> bool {
            *self == BITENABLER::THE_SPI_CONTROLLER_S
        }
    }
    # [ doc = "Possible values of the field `CPHA`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CPHAR {
        # [ doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal." ]
        FIRST_EDGE,
        # [ doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active." ]
        SECOND_EDGE,
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
                CPHAR::FIRST_EDGE => false,
                CPHAR::SECOND_EDGE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CPHAR {
            match value {
                false => CPHAR::FIRST_EDGE,
                true => CPHAR::SECOND_EDGE,
            }
        }
        # [ doc = "Checks if the value of the field is `FIRST_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_first_edge(&self) -> bool {
            *self == CPHAR::FIRST_EDGE
        }
        # [ doc = "Checks if the value of the field is `SECOND_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_second_edge(&self) -> bool {
            *self == CPHAR::SECOND_EDGE
        }
    }
    # [ doc = "Possible values of the field `CPOL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CPOLR {
        # [ doc = "SCK is active high." ]
        SCK_IS_ACTIVE_HIGH_,
        # [ doc = "SCK is active low." ]
        SCK_IS_ACTIVE_LOW_,
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
                CPOLR::SCK_IS_ACTIVE_HIGH_ => false,
                CPOLR::SCK_IS_ACTIVE_LOW_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CPOLR {
            match value {
                false => CPOLR::SCK_IS_ACTIVE_HIGH_,
                true => CPOLR::SCK_IS_ACTIVE_LOW_,
            }
        }
        # [ doc = "Checks if the value of the field is `SCK_IS_ACTIVE_HIGH_`" ]
        # [ inline ( always ) ]
        pub fn is_sck_is_active_high_(&self) -> bool {
            *self == CPOLR::SCK_IS_ACTIVE_HIGH_
        }
        # [ doc = "Checks if the value of the field is `SCK_IS_ACTIVE_LOW_`" ]
        # [ inline ( always ) ]
        pub fn is_sck_is_active_low_(&self) -> bool {
            *self == CPOLR::SCK_IS_ACTIVE_LOW_
        }
    }
    # [ doc = "Possible values of the field `MSTR`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MSTRR {
        # [ doc = "The SPI operates in Slave mode." ]
        SLAVE,
        # [ doc = "The SPI operates in Master mode." ]
        MASTER,
    }
    impl MSTRR {
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
                MSTRR::SLAVE => false,
                MSTRR::MASTER => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MSTRR {
            match value {
                false => MSTRR::SLAVE,
                true => MSTRR::MASTER,
            }
        }
        # [ doc = "Checks if the value of the field is `SLAVE`" ]
        # [ inline ( always ) ]
        pub fn is_slave(&self) -> bool {
            *self == MSTRR::SLAVE
        }
        # [ doc = "Checks if the value of the field is `MASTER`" ]
        # [ inline ( always ) ]
        pub fn is_master(&self) -> bool {
            *self == MSTRR::MASTER
        }
    }
    # [ doc = "Possible values of the field `LSBF`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum LSBFR {
        # [ doc = "SPI data is transferred MSB (bit 7) first." ]
        MSB,
        # [ doc = "SPI data is transferred LSB (bit 0) first." ]
        LSB,
    }
    impl LSBFR {
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
                LSBFR::MSB => false,
                LSBFR::LSB => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> LSBFR {
            match value {
                false => LSBFR::MSB,
                true => LSBFR::LSB,
            }
        }
        # [ doc = "Checks if the value of the field is `MSB`" ]
        # [ inline ( always ) ]
        pub fn is_msb(&self) -> bool {
            *self == LSBFR::MSB
        }
        # [ doc = "Checks if the value of the field is `LSB`" ]
        # [ inline ( always ) ]
        pub fn is_lsb(&self) -> bool {
            *self == LSBFR::LSB
        }
    }
    # [ doc = "Possible values of the field `SPIE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SPIER {
        # [ doc = "SPI interrupts are inhibited." ]
        INTBLOCK,
        # [ doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated." ]
        HWINT,
    }
    impl SPIER {
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
                SPIER::INTBLOCK => false,
                SPIER::HWINT => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SPIER {
            match value {
                false => SPIER::INTBLOCK,
                true => SPIER::HWINT,
            }
        }
        # [ doc = "Checks if the value of the field is `INTBLOCK`" ]
        # [ inline ( always ) ]
        pub fn is_intblock(&self) -> bool {
            *self == SPIER::INTBLOCK
        }
        # [ doc = "Checks if the value of the field is `HWINT`" ]
        # [ inline ( always ) ]
        pub fn is_hwint(&self) -> bool {
            *self == SPIER::HWINT
        }
    }
    # [ doc = "Possible values of the field `BITST`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum BITSTR {
        # [ doc = "8 bits per transfer" ]
        _8_BITS_PER_TRANSFER,
        # [ doc = "9 bits per transfer" ]
        _9_BITS_PER_TRANSFER,
        # [ doc = "10 bits per transfer" ]
        _10_BITS_PER_TRANSFER,
        # [ doc = "11 bits per transfer" ]
        _11_BITS_PER_TRANSFER,
        # [ doc = "12 bits per transfer" ]
        _12_BITS_PER_TRANSFER,
        # [ doc = "13 bits per transfer" ]
        _13_BITS_PER_TRANSFER,
        # [ doc = "14 bits per transfer" ]
        _14_BITS_PER_TRANSFER,
        # [ doc = "15 bits per transfer" ]
        _15_BITS_PER_TRANSFER,
        # [ doc = "16 bits per transfer" ]
        _16_BITS_PER_TRANSFER,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl BITSTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                BITSTR::_8_BITS_PER_TRANSFER => 8,
                BITSTR::_9_BITS_PER_TRANSFER => 9,
                BITSTR::_10_BITS_PER_TRANSFER => 10,
                BITSTR::_11_BITS_PER_TRANSFER => 11,
                BITSTR::_12_BITS_PER_TRANSFER => 12,
                BITSTR::_13_BITS_PER_TRANSFER => 13,
                BITSTR::_14_BITS_PER_TRANSFER => 14,
                BITSTR::_15_BITS_PER_TRANSFER => 15,
                BITSTR::_16_BITS_PER_TRANSFER => 0,
                BITSTR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> BITSTR {
            match value {
                8 => BITSTR::_8_BITS_PER_TRANSFER,
                9 => BITSTR::_9_BITS_PER_TRANSFER,
                10 => BITSTR::_10_BITS_PER_TRANSFER,
                11 => BITSTR::_11_BITS_PER_TRANSFER,
                12 => BITSTR::_12_BITS_PER_TRANSFER,
                13 => BITSTR::_13_BITS_PER_TRANSFER,
                14 => BITSTR::_14_BITS_PER_TRANSFER,
                15 => BITSTR::_15_BITS_PER_TRANSFER,
                0 => BITSTR::_16_BITS_PER_TRANSFER,
                i => BITSTR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `_8_BITS_PER_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_8_bits_per_transfer(&self) -> bool {
            *self == BITSTR::_8_BITS_PER_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_9_BITS_PER_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_9_bits_per_transfer(&self) -> bool {
            *self == BITSTR::_9_BITS_PER_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_10_BITS_PER_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_10_bits_per_transfer(&self) -> bool {
            *self == BITSTR::_10_BITS_PER_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_11_BITS_PER_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_11_bits_per_transfer(&self) -> bool {
            *self == BITSTR::_11_BITS_PER_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_12_BITS_PER_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_12_bits_per_transfer(&self) -> bool {
            *self == BITSTR::_12_BITS_PER_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_13_BITS_PER_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_13_bits_per_transfer(&self) -> bool {
            *self == BITSTR::_13_BITS_PER_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_14_BITS_PER_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_14_bits_per_transfer(&self) -> bool {
            *self == BITSTR::_14_BITS_PER_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_15_BITS_PER_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_15_bits_per_transfer(&self) -> bool {
            *self == BITSTR::_15_BITS_PER_TRANSFER
        }
        # [ doc = "Checks if the value of the field is `_16_BITS_PER_TRANSFER`" ]
        # [ inline ( always ) ]
        pub fn is_16_bits_per_transfer(&self) -> bool {
            *self == BITSTR::_16_BITS_PER_TRANSFER
        }
    }
    # [ doc = "Values that can be written to the field `BITENABLE`" ]
    pub enum BITENABLEW {
        # [ doc = "The SPI controller sends and receives the number of bits selected by bits 11:8." ]
        THE_SPI_CONTROLLER_S,
    }
    impl BITENABLEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                BITENABLEW::THE_SPI_CONTROLLER_S => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BITENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BITENABLEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BITENABLEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The SPI controller sends and receives the number of bits selected by bits 11:8." ]
        # [ inline ( always ) ]
        pub fn the_spi_controller_s(self) -> &'a mut W {
            self.variant(BITENABLEW::THE_SPI_CONTROLLER_S)
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
    # [ doc = "Values that can be written to the field `CPHA`" ]
    pub enum CPHAW {
        # [ doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal." ]
        FIRST_EDGE,
        # [ doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active." ]
        SECOND_EDGE,
    }
    impl CPHAW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CPHAW::FIRST_EDGE => false,
                CPHAW::SECOND_EDGE => true,
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
        # [ doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal." ]
        # [ inline ( always ) ]
        pub fn first_edge(self) -> &'a mut W {
            self.variant(CPHAW::FIRST_EDGE)
        }
        # [ doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active." ]
        # [ inline ( always ) ]
        pub fn second_edge(self) -> &'a mut W {
            self.variant(CPHAW::SECOND_EDGE)
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
    # [ doc = "Values that can be written to the field `CPOL`" ]
    pub enum CPOLW {
        # [ doc = "SCK is active high." ]
        SCK_IS_ACTIVE_HIGH_,
        # [ doc = "SCK is active low." ]
        SCK_IS_ACTIVE_LOW_,
    }
    impl CPOLW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CPOLW::SCK_IS_ACTIVE_HIGH_ => false,
                CPOLW::SCK_IS_ACTIVE_LOW_ => true,
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
        # [ doc = "SCK is active high." ]
        # [ inline ( always ) ]
        pub fn sck_is_active_high_(self) -> &'a mut W {
            self.variant(CPOLW::SCK_IS_ACTIVE_HIGH_)
        }
        # [ doc = "SCK is active low." ]
        # [ inline ( always ) ]
        pub fn sck_is_active_low_(self) -> &'a mut W {
            self.variant(CPOLW::SCK_IS_ACTIVE_LOW_)
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
    # [ doc = "Values that can be written to the field `MSTR`" ]
    pub enum MSTRW {
        # [ doc = "The SPI operates in Slave mode." ]
        SLAVE,
        # [ doc = "The SPI operates in Master mode." ]
        MASTER,
    }
    impl MSTRW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MSTRW::SLAVE => false,
                MSTRW::MASTER => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MSTRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MSTRW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MSTRW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The SPI operates in Slave mode." ]
        # [ inline ( always ) ]
        pub fn slave(self) -> &'a mut W {
            self.variant(MSTRW::SLAVE)
        }
        # [ doc = "The SPI operates in Master mode." ]
        # [ inline ( always ) ]
        pub fn master(self) -> &'a mut W {
            self.variant(MSTRW::MASTER)
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
    # [ doc = "Values that can be written to the field `LSBF`" ]
    pub enum LSBFW {
        # [ doc = "SPI data is transferred MSB (bit 7) first." ]
        MSB,
        # [ doc = "SPI data is transferred LSB (bit 0) first." ]
        LSB,
    }
    impl LSBFW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                LSBFW::MSB => false,
                LSBFW::LSB => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LSBFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LSBFW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: LSBFW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "SPI data is transferred MSB (bit 7) first." ]
        # [ inline ( always ) ]
        pub fn msb(self) -> &'a mut W {
            self.variant(LSBFW::MSB)
        }
        # [ doc = "SPI data is transferred LSB (bit 0) first." ]
        # [ inline ( always ) ]
        pub fn lsb(self) -> &'a mut W {
            self.variant(LSBFW::LSB)
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
    # [ doc = "Values that can be written to the field `SPIE`" ]
    pub enum SPIEW {
        # [ doc = "SPI interrupts are inhibited." ]
        INTBLOCK,
        # [ doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated." ]
        HWINT,
    }
    impl SPIEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SPIEW::INTBLOCK => false,
                SPIEW::HWINT => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SPIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SPIEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SPIEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "SPI interrupts are inhibited." ]
        # [ inline ( always ) ]
        pub fn intblock(self) -> &'a mut W {
            self.variant(SPIEW::INTBLOCK)
        }
        # [ doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated." ]
        # [ inline ( always ) ]
        pub fn hwint(self) -> &'a mut W {
            self.variant(SPIEW::HWINT)
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
    # [ doc = "Values that can be written to the field `BITST`" ]
    pub enum BITSTW {
        # [ doc = "8 bits per transfer" ]
        _8_BITS_PER_TRANSFER,
        # [ doc = "9 bits per transfer" ]
        _9_BITS_PER_TRANSFER,
        # [ doc = "10 bits per transfer" ]
        _10_BITS_PER_TRANSFER,
        # [ doc = "11 bits per transfer" ]
        _11_BITS_PER_TRANSFER,
        # [ doc = "12 bits per transfer" ]
        _12_BITS_PER_TRANSFER,
        # [ doc = "13 bits per transfer" ]
        _13_BITS_PER_TRANSFER,
        # [ doc = "14 bits per transfer" ]
        _14_BITS_PER_TRANSFER,
        # [ doc = "15 bits per transfer" ]
        _15_BITS_PER_TRANSFER,
        # [ doc = "16 bits per transfer" ]
        _16_BITS_PER_TRANSFER,
    }
    impl BITSTW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                BITSTW::_8_BITS_PER_TRANSFER => 8,
                BITSTW::_9_BITS_PER_TRANSFER => 9,
                BITSTW::_10_BITS_PER_TRANSFER => 10,
                BITSTW::_11_BITS_PER_TRANSFER => 11,
                BITSTW::_12_BITS_PER_TRANSFER => 12,
                BITSTW::_13_BITS_PER_TRANSFER => 13,
                BITSTW::_14_BITS_PER_TRANSFER => 14,
                BITSTW::_15_BITS_PER_TRANSFER => 15,
                BITSTW::_16_BITS_PER_TRANSFER => 0,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BITSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BITSTW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BITSTW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "8 bits per transfer" ]
        # [ inline ( always ) ]
        pub fn _8_bits_per_transfer(self) -> &'a mut W {
            self.variant(BITSTW::_8_BITS_PER_TRANSFER)
        }
        # [ doc = "9 bits per transfer" ]
        # [ inline ( always ) ]
        pub fn _9_bits_per_transfer(self) -> &'a mut W {
            self.variant(BITSTW::_9_BITS_PER_TRANSFER)
        }
        # [ doc = "10 bits per transfer" ]
        # [ inline ( always ) ]
        pub fn _10_bits_per_transfer(self) -> &'a mut W {
            self.variant(BITSTW::_10_BITS_PER_TRANSFER)
        }
        # [ doc = "11 bits per transfer" ]
        # [ inline ( always ) ]
        pub fn _11_bits_per_transfer(self) -> &'a mut W {
            self.variant(BITSTW::_11_BITS_PER_TRANSFER)
        }
        # [ doc = "12 bits per transfer" ]
        # [ inline ( always ) ]
        pub fn _12_bits_per_transfer(self) -> &'a mut W {
            self.variant(BITSTW::_12_BITS_PER_TRANSFER)
        }
        # [ doc = "13 bits per transfer" ]
        # [ inline ( always ) ]
        pub fn _13_bits_per_transfer(self) -> &'a mut W {
            self.variant(BITSTW::_13_BITS_PER_TRANSFER)
        }
        # [ doc = "14 bits per transfer" ]
        # [ inline ( always ) ]
        pub fn _14_bits_per_transfer(self) -> &'a mut W {
            self.variant(BITSTW::_14_BITS_PER_TRANSFER)
        }
        # [ doc = "15 bits per transfer" ]
        # [ inline ( always ) ]
        pub fn _15_bits_per_transfer(self) -> &'a mut W {
            self.variant(BITSTW::_15_BITS_PER_TRANSFER)
        }
        # [ doc = "16 bits per transfer" ]
        # [ inline ( always ) ]
        pub fn _16_bits_per_transfer(self) -> &'a mut W {
            self.variant(BITSTW::_16_BITS_PER_TRANSFER)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
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
        # [ doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer." ]
        # [ inline ( always ) ]
        pub fn bitenable(&self) -> BITENABLER {
            BITENABLER::_from({
                                  const MASK: bool = true;
                                  const OFFSET: u8 = 2;
                                  ((self.bits >> OFFSET) & MASK as u32) != 0
                              })
        }
        # [ doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending." ]
        # [ inline ( always ) ]
        pub fn cpha(&self) -> CPHAR {
            CPHAR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 3;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 4 - Clock polarity control." ]
        # [ inline ( always ) ]
        pub fn cpol(&self) -> CPOLR {
            CPOLR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 4;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 5 - Master mode select." ]
        # [ inline ( always ) ]
        pub fn mstr(&self) -> MSTRR {
            MSTRR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 5;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred." ]
        # [ inline ( always ) ]
        pub fn lsbf(&self) -> LSBFR {
            LSBFR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 6;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 7 - Serial peripheral interrupt enable." ]
        # [ inline ( always ) ]
        pub fn spie(&self) -> SPIER {
            SPIER::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 7;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:" ]
        # [ inline ( always ) ]
        pub fn bitst(&self) -> BITSTR {
            BITSTR::_from({
                              const MASK: u8 = 15;
                              const OFFSET: u8 = 8;
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
        # [ doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer." ]
        # [ inline ( always ) ]
        pub fn bitenable(&mut self) -> _BITENABLEW {
            _BITENABLEW { w: self }
        }
        # [ doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending." ]
        # [ inline ( always ) ]
        pub fn cpha(&mut self) -> _CPHAW {
            _CPHAW { w: self }
        }
        # [ doc = "Bit 4 - Clock polarity control." ]
        # [ inline ( always ) ]
        pub fn cpol(&mut self) -> _CPOLW {
            _CPOLW { w: self }
        }
        # [ doc = "Bit 5 - Master mode select." ]
        # [ inline ( always ) ]
        pub fn mstr(&mut self) -> _MSTRW {
            _MSTRW { w: self }
        }
        # [ doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred." ]
        # [ inline ( always ) ]
        pub fn lsbf(&mut self) -> _LSBFW {
            _LSBFW { w: self }
        }
        # [ doc = "Bit 7 - Serial peripheral interrupt enable." ]
        # [ inline ( always ) ]
        pub fn spie(&mut self) -> _SPIEW {
            _SPIEW { w: self }
        }
        # [ doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:" ]
        # [ inline ( always ) ]
        pub fn bitst(&mut self) -> _BITSTW {
            _BITSTW { w: self }
        }
    }
}
# [ doc = "SPI Status Register. This register shows the status of the SPI." ]
pub struct SR {
    register: VolatileCell<u32>,
}
# [ doc = "SPI Status Register. This register shows the status of the SPI." ]
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
    pub struct ABRTR {
        bits: bool,
    }
    impl ABRTR {
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
    pub struct MODFR {
        bits: bool,
    }
    impl MODFR {
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
    pub struct ROVRR {
        bits: bool,
    }
    impl ROVRR {
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
    pub struct WCOLR {
        bits: bool,
    }
    impl WCOLR {
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
    pub struct SPIFR {
        bits: bool,
    }
    impl SPIFR {
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
        # [ doc = "Bit 3 - Slave abort. When 1, this bit indicates that a slave abort has occurred. This bit is cleared by reading this register." ]
        # [ inline ( always ) ]
        pub fn abrt(&self) -> ABRTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ABRTR { bits }
        }
        # [ doc = "Bit 4 - Mode fault. when 1, this bit indicates that a Mode fault error has occurred. This bit is cleared by reading this register, then writing the SPI0 control register." ]
        # [ inline ( always ) ]
        pub fn modf(&self) -> MODFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MODFR { bits }
        }
        # [ doc = "Bit 5 - Read overrun. When 1, this bit indicates that a read overrun has occurred. This bit is cleared by reading this register." ]
        # [ inline ( always ) ]
        pub fn rovr(&self) -> ROVRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ROVRR { bits }
        }
        # [ doc = "Bit 6 - Write collision. When 1, this bit indicates that a write collision has occurred. This bit is cleared by reading this register, then accessing the SPI Data Register." ]
        # [ inline ( always ) ]
        pub fn wcol(&self) -> WCOLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WCOLR { bits }
        }
        # [ doc = "Bit 7 - SPI transfer complete flag. When 1, this bit indicates when a SPI data transfer is complete. When a master, this bit is set at the end of the last cycle of the transfer. When a slave, this bit is set on the last data sampling edge of the SCK. This bit is cleared by first reading this register, then accessing the SPI Data Register. Note: this is not the SPI interrupt flag. This flag is found in the SPINT register." ]
        # [ inline ( always ) ]
        pub fn spif(&self) -> SPIFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SPIFR { bits }
        }
    }
}
# [ doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register." ]
pub struct DR {
    register: VolatileCell<u32>,
}
# [ doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register." ]
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
    pub struct DATALOWR {
        bits: u8,
    }
    impl DATALOWR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATAHIGHR {
        bits: u8,
    }
    impl DATAHIGHR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATALOWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATALOWW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DATAHIGHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATAHIGHW<'a> {
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
        # [ doc = "Bits 0:7 - SPI Bi-directional data port." ]
        # [ inline ( always ) ]
        pub fn datalow(&self) -> DATALOWR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATALOWR { bits }
        }
        # [ doc = "Bits 8:15 - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes." ]
        # [ inline ( always ) ]
        pub fn datahigh(&self) -> DATAHIGHR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATAHIGHR { bits }
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
        # [ doc = "Bits 0:7 - SPI Bi-directional data port." ]
        # [ inline ( always ) ]
        pub fn datalow(&mut self) -> _DATALOWW {
            _DATALOWW { w: self }
        }
        # [ doc = "Bits 8:15 - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes." ]
        # [ inline ( always ) ]
        pub fn datahigh(&mut self) -> _DATAHIGHW {
            _DATAHIGHW { w: self }
        }
    }
}
# [ doc = "SPI Clock Counter Register. This register controls the frequency of a master's SCK0." ]
pub struct CCR {
    register: VolatileCell<u32>,
}
# [ doc = "SPI Clock Counter Register. This register controls the frequency of a master's SCK0." ]
pub mod ccr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCR {
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
    pub struct COUNTERR {
        bits: u8,
    }
    impl COUNTERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _COUNTERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _COUNTERW<'a> {
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
        # [ doc = "Bits 0:7 - SPI0 Clock counter setting." ]
        # [ inline ( always ) ]
        pub fn counter(&self) -> COUNTERR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            COUNTERR { bits }
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
        # [ doc = "Bits 0:7 - SPI0 Clock counter setting." ]
        # [ inline ( always ) ]
        pub fn counter(&mut self) -> _COUNTERW {
            _COUNTERW { w: self }
        }
    }
}
# [ doc = "SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface." ]
pub struct INT {
    register: VolatileCell<u32>,
}
# [ doc = "SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface." ]
pub mod int {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::INT {
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
    pub struct SPIFR {
        bits: bool,
    }
    impl SPIFR {
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
    pub struct _SPIFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SPIFW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - SPI interrupt flag. Set by the SPI interface to generate an interrupt. Cleared by writing a 1 to this bit. Note: this bit will be set once when SPIE = 1 and at least one of SPIF and WCOL bits is 1. However, only when the SPI Interrupt bit is set and SPI0 Interrupt is enabled in the NVIC, SPI based interrupt can be processed by interrupt handling software." ]
        # [ inline ( always ) ]
        pub fn spif(&self) -> SPIFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SPIFR { bits }
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
        # [ doc = "Bit 0 - SPI interrupt flag. Set by the SPI interface to generate an interrupt. Cleared by writing a 1 to this bit. Note: this bit will be set once when SPIE = 1 and at least one of SPIF and WCOL bits is 1. However, only when the SPI Interrupt bit is set and SPI0 Interrupt is enabled in the NVIC, SPI based interrupt can be processed by interrupt handling software." ]
        # [ inline ( always ) ]
        pub fn spif(&mut self) -> _SPIFW {
            _SPIFW { w: self }
        }
    }
}
# [ doc = "SPI" ]
pub struct SPI {
    register_block: RegisterBlock,
}
impl Deref for SPI {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
