# ! [ doc = "I2S interface" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "I2S interface" ]
pub const I2S: Peripheral<I2S> = unsafe { Peripheral::new(1074429952) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel." ]
    pub dao: DAO,
    # [ doc = "0x04 - I2S Digital Audio Input Register. Contains control bits for the I2S receive channel." ]
    pub dai: DAI,
    # [ doc = "0x08 - I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO." ]
    pub txfifo: TXFIFO,
    # [ doc = "0x0c - I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO." ]
    pub rxfifo: RXFIFO,
    # [ doc = "0x10 - I2S Status Feedback Register. Contains status information about the I2S interface." ]
    pub state: STATE,
    # [ doc = "0x14 - I2S DMA Configuration Register 1. Contains control information for DMA request 1." ]
    pub dma1: DMA1,
    # [ doc = "0x18 - I2S DMA Configuration Register 2. Contains control information for DMA request 2." ]
    pub dma2: DMA2,
    # [ doc = "0x1c - I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated." ]
    pub irq: IRQ,
    # [ doc = "0x20 - I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK." ]
    pub txrate: TXRATE,
    # [ doc = "0x24 - I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK." ]
    pub rxrate: RXRATE,
    # [ doc = "0x28 - I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock." ]
    pub txbitrate: TXBITRATE,
    # [ doc = "0x2c - I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock." ]
    pub rxbitrate: RXBITRATE,
    # [ doc = "0x30 - I2S Transmit mode control." ]
    pub txmode: TXMODE,
    # [ doc = "0x34 - I2S Receive mode control." ]
    pub rxmode: RXMODE,
}
# [ doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel." ]
pub struct DAO {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel." ]
pub mod dao {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DAO {
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
    # [ doc = "Possible values of the field `WORDWIDTH`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum WORDWIDTHR {
        # [ doc = "8-bit data" ]
        _8_BIT_DATA,
        # [ doc = "16-bit data" ]
        _16_BIT_DATA,
        # [ doc = "32-bit data" ]
        _32_BIT_DATA,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl WORDWIDTHR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                WORDWIDTHR::_8_BIT_DATA => 0,
                WORDWIDTHR::_16_BIT_DATA => 1,
                WORDWIDTHR::_32_BIT_DATA => 3,
                WORDWIDTHR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> WORDWIDTHR {
            match value {
                0 => WORDWIDTHR::_8_BIT_DATA,
                1 => WORDWIDTHR::_16_BIT_DATA,
                3 => WORDWIDTHR::_32_BIT_DATA,
                i => WORDWIDTHR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `_8_BIT_DATA`" ]
        # [ inline ( always ) ]
        pub fn is_8_bit_data(&self) -> bool {
            *self == WORDWIDTHR::_8_BIT_DATA
        }
        # [ doc = "Checks if the value of the field is `_16_BIT_DATA`" ]
        # [ inline ( always ) ]
        pub fn is_16_bit_data(&self) -> bool {
            *self == WORDWIDTHR::_16_BIT_DATA
        }
        # [ doc = "Checks if the value of the field is `_32_BIT_DATA`" ]
        # [ inline ( always ) ]
        pub fn is_32_bit_data(&self) -> bool {
            *self == WORDWIDTHR::_32_BIT_DATA
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MONOR {
        bits: bool,
    }
    impl MONOR {
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
    pub struct STOPR {
        bits: bool,
    }
    impl STOPR {
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
    pub struct RESETR {
        bits: bool,
    }
    impl RESETR {
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
    pub struct WS_SELR {
        bits: bool,
    }
    impl WS_SELR {
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
    pub struct WS_HALFPERIODR {
        bits: u16,
    }
    impl WS_HALFPERIODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MUTER {
        bits: bool,
    }
    impl MUTER {
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
    # [ doc = "Values that can be written to the field `WORDWIDTH`" ]
    pub enum WORDWIDTHW {
        # [ doc = "8-bit data" ]
        _8_BIT_DATA,
        # [ doc = "16-bit data" ]
        _16_BIT_DATA,
        # [ doc = "32-bit data" ]
        _32_BIT_DATA,
    }
    impl WORDWIDTHW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                WORDWIDTHW::_8_BIT_DATA => 0,
                WORDWIDTHW::_16_BIT_DATA => 1,
                WORDWIDTHW::_32_BIT_DATA => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WORDWIDTHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WORDWIDTHW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: WORDWIDTHW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "8-bit data" ]
        # [ inline ( always ) ]
        pub fn _8_bit_data(self) -> &'a mut W {
            self.variant(WORDWIDTHW::_8_BIT_DATA)
        }
        # [ doc = "16-bit data" ]
        # [ inline ( always ) ]
        pub fn _16_bit_data(self) -> &'a mut W {
            self.variant(WORDWIDTHW::_16_BIT_DATA)
        }
        # [ doc = "32-bit data" ]
        # [ inline ( always ) ]
        pub fn _32_bit_data(self) -> &'a mut W {
            self.variant(WORDWIDTHW::_32_BIT_DATA)
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
    # [ doc = r" Proxy" ]
    pub struct _MONOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MONOW<'a> {
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
    pub struct _STOPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STOPW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _RESETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESETW<'a> {
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
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WS_SELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WS_SELW<'a> {
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
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WS_HALFPERIODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WS_HALFPERIODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 511;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MUTEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MUTEW<'a> {
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
            const OFFSET: u8 = 15;
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
        # [ doc = "Bits 0:1 - Selects the number of bytes in data as follows:" ]
        # [ inline ( always ) ]
        pub fn wordwidth(&self) -> WORDWIDTHR {
            WORDWIDTHR::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format." ]
        # [ inline ( always ) ]
        pub fn mono(&self) -> MONOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MONOR { bits }
        }
        # [ doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode." ]
        # [ inline ( always ) ]
        pub fn stop(&self) -> STOPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            STOPR { bits }
        }
        # [ doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO." ]
        # [ inline ( always ) ]
        pub fn reset(&self) -> RESETR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RESETR { bits }
        }
        # [ doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE." ]
        # [ inline ( always ) ]
        pub fn ws_sel(&self) -> WS_SELR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WS_SELR { bits }
        }
        # [ doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31." ]
        # [ inline ( always ) ]
        pub fn ws_halfperiod(&self) -> WS_HALFPERIODR {
            let bits = {
                const MASK: u16 = 511;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            WS_HALFPERIODR { bits }
        }
        # [ doc = "Bit 15 - When 1, the transmit channel sends only zeroes." ]
        # [ inline ( always ) ]
        pub fn mute(&self) -> MUTER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MUTER { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 34785 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Selects the number of bytes in data as follows:" ]
        # [ inline ( always ) ]
        pub fn wordwidth(&mut self) -> _WORDWIDTHW {
            _WORDWIDTHW { w: self }
        }
        # [ doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format." ]
        # [ inline ( always ) ]
        pub fn mono(&mut self) -> _MONOW {
            _MONOW { w: self }
        }
        # [ doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode." ]
        # [ inline ( always ) ]
        pub fn stop(&mut self) -> _STOPW {
            _STOPW { w: self }
        }
        # [ doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO." ]
        # [ inline ( always ) ]
        pub fn reset(&mut self) -> _RESETW {
            _RESETW { w: self }
        }
        # [ doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE." ]
        # [ inline ( always ) ]
        pub fn ws_sel(&mut self) -> _WS_SELW {
            _WS_SELW { w: self }
        }
        # [ doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31." ]
        # [ inline ( always ) ]
        pub fn ws_halfperiod(&mut self) -> _WS_HALFPERIODW {
            _WS_HALFPERIODW { w: self }
        }
        # [ doc = "Bit 15 - When 1, the transmit channel sends only zeroes." ]
        # [ inline ( always ) ]
        pub fn mute(&mut self) -> _MUTEW {
            _MUTEW { w: self }
        }
    }
}
# [ doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel." ]
pub struct DAI {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel." ]
pub mod dai {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DAI {
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
    # [ doc = "Possible values of the field `WORDWIDTH`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum WORDWIDTHR {
        # [ doc = "8-bit data" ]
        _8_BIT_DATA,
        # [ doc = "16-bit data" ]
        _16_BIT_DATA,
        # [ doc = "32-bit data" ]
        _32_BIT_DATA,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl WORDWIDTHR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                WORDWIDTHR::_8_BIT_DATA => 0,
                WORDWIDTHR::_16_BIT_DATA => 1,
                WORDWIDTHR::_32_BIT_DATA => 3,
                WORDWIDTHR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> WORDWIDTHR {
            match value {
                0 => WORDWIDTHR::_8_BIT_DATA,
                1 => WORDWIDTHR::_16_BIT_DATA,
                3 => WORDWIDTHR::_32_BIT_DATA,
                i => WORDWIDTHR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `_8_BIT_DATA`" ]
        # [ inline ( always ) ]
        pub fn is_8_bit_data(&self) -> bool {
            *self == WORDWIDTHR::_8_BIT_DATA
        }
        # [ doc = "Checks if the value of the field is `_16_BIT_DATA`" ]
        # [ inline ( always ) ]
        pub fn is_16_bit_data(&self) -> bool {
            *self == WORDWIDTHR::_16_BIT_DATA
        }
        # [ doc = "Checks if the value of the field is `_32_BIT_DATA`" ]
        # [ inline ( always ) ]
        pub fn is_32_bit_data(&self) -> bool {
            *self == WORDWIDTHR::_32_BIT_DATA
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MONOR {
        bits: bool,
    }
    impl MONOR {
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
    pub struct STOPR {
        bits: bool,
    }
    impl STOPR {
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
    pub struct RESETR {
        bits: bool,
    }
    impl RESETR {
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
    pub struct WS_SELR {
        bits: bool,
    }
    impl WS_SELR {
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
    pub struct WS_HALFPERIODR {
        bits: u16,
    }
    impl WS_HALFPERIODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Values that can be written to the field `WORDWIDTH`" ]
    pub enum WORDWIDTHW {
        # [ doc = "8-bit data" ]
        _8_BIT_DATA,
        # [ doc = "16-bit data" ]
        _16_BIT_DATA,
        # [ doc = "32-bit data" ]
        _32_BIT_DATA,
    }
    impl WORDWIDTHW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                WORDWIDTHW::_8_BIT_DATA => 0,
                WORDWIDTHW::_16_BIT_DATA => 1,
                WORDWIDTHW::_32_BIT_DATA => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WORDWIDTHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WORDWIDTHW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: WORDWIDTHW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "8-bit data" ]
        # [ inline ( always ) ]
        pub fn _8_bit_data(self) -> &'a mut W {
            self.variant(WORDWIDTHW::_8_BIT_DATA)
        }
        # [ doc = "16-bit data" ]
        # [ inline ( always ) ]
        pub fn _16_bit_data(self) -> &'a mut W {
            self.variant(WORDWIDTHW::_16_BIT_DATA)
        }
        # [ doc = "32-bit data" ]
        # [ inline ( always ) ]
        pub fn _32_bit_data(self) -> &'a mut W {
            self.variant(WORDWIDTHW::_32_BIT_DATA)
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
    # [ doc = r" Proxy" ]
    pub struct _MONOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MONOW<'a> {
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
    pub struct _STOPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STOPW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _RESETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESETW<'a> {
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
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WS_SELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WS_SELW<'a> {
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
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WS_HALFPERIODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WS_HALFPERIODW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 511;
            const OFFSET: u8 = 6;
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
        # [ doc = "Bits 0:1 - Selects the number of bytes in data as follows:" ]
        # [ inline ( always ) ]
        pub fn wordwidth(&self) -> WORDWIDTHR {
            WORDWIDTHR::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format." ]
        # [ inline ( always ) ]
        pub fn mono(&self) -> MONOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MONOR { bits }
        }
        # [ doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode." ]
        # [ inline ( always ) ]
        pub fn stop(&self) -> STOPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            STOPR { bits }
        }
        # [ doc = "Bit 4 - When 1, asynchronously reset the transmit channel and FIFO." ]
        # [ inline ( always ) ]
        pub fn reset(&self) -> RESETR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RESETR { bits }
        }
        # [ doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE." ]
        # [ inline ( always ) ]
        pub fn ws_sel(&self) -> WS_SELR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WS_SELR { bits }
        }
        # [ doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31." ]
        # [ inline ( always ) ]
        pub fn ws_halfperiod(&self) -> WS_HALFPERIODR {
            let bits = {
                const MASK: u16 = 511;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            WS_HALFPERIODR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 2017 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:1 - Selects the number of bytes in data as follows:" ]
        # [ inline ( always ) ]
        pub fn wordwidth(&mut self) -> _WORDWIDTHW {
            _WORDWIDTHW { w: self }
        }
        # [ doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format." ]
        # [ inline ( always ) ]
        pub fn mono(&mut self) -> _MONOW {
            _MONOW { w: self }
        }
        # [ doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode." ]
        # [ inline ( always ) ]
        pub fn stop(&mut self) -> _STOPW {
            _STOPW { w: self }
        }
        # [ doc = "Bit 4 - When 1, asynchronously reset the transmit channel and FIFO." ]
        # [ inline ( always ) ]
        pub fn reset(&mut self) -> _RESETW {
            _RESETW { w: self }
        }
        # [ doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE." ]
        # [ inline ( always ) ]
        pub fn ws_sel(&mut self) -> _WS_SELW {
            _WS_SELW { w: self }
        }
        # [ doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31." ]
        # [ inline ( always ) ]
        pub fn ws_halfperiod(&mut self) -> _WS_HALFPERIODW {
            _WS_HALFPERIODW { w: self }
        }
    }
}
# [ doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO." ]
pub struct TXFIFO {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO." ]
pub mod txfifo {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TXFIFO {
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
    pub struct _I2STXFIFOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _I2STXFIFOW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
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
        # [ doc = "Bits 0:31 - 8 x 32-bit transmit FIFO." ]
        # [ inline ( always ) ]
        pub fn i2stxfifo(&mut self) -> _I2STXFIFOW {
            _I2STXFIFOW { w: self }
        }
    }
}
# [ doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO." ]
pub struct RXFIFO {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO." ]
pub mod rxfifo {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::RXFIFO {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct I2SRXFIFOR {
        bits: u32,
    }
    impl I2SRXFIFOR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:31 - 8 x 32-bit transmit FIFO." ]
        # [ inline ( always ) ]
        pub fn i2srxfifo(&self) -> I2SRXFIFOR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            I2SRXFIFOR { bits }
        }
    }
}
# [ doc = "I2S Status Feedback Register. Contains status information about the I2S interface." ]
pub struct STATE {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Status Feedback Register. Contains status information about the I2S interface." ]
pub mod state {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::STATE {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IRQR {
        bits: bool,
    }
    impl IRQR {
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
    pub struct DMAREQ1R {
        bits: bool,
    }
    impl DMAREQ1R {
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
    pub struct DMAREQ2R {
        bits: bool,
    }
    impl DMAREQ2R {
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
    pub struct RX_LEVELR {
        bits: u8,
    }
    impl RX_LEVELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TX_LEVELR {
        bits: u8,
    }
    impl TX_LEVELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This is determined by comparing the current FIFO levels to the rx_depth_irq and tx_depth_irq fields in the IRQ register." ]
        # [ inline ( always ) ]
        pub fn irq(&self) -> IRQR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IRQR { bits }
        }
        # [ doc = "Bit 1 - This bit reflects the presence of Receive or Transmit DMA Request 1. This is determined by comparing the current FIFO levels to the rx_depth_dma1 and tx_depth_dma1 fields in the DMA1 register." ]
        # [ inline ( always ) ]
        pub fn dmareq1(&self) -> DMAREQ1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMAREQ1R { bits }
        }
        # [ doc = "Bit 2 - This bit reflects the presence of Receive or Transmit DMA Request 2. This is determined by comparing the current FIFO levels to the rx_depth_dma2 and tx_depth_dma2 fields in the DMA2 register." ]
        # [ inline ( always ) ]
        pub fn dmareq2(&self) -> DMAREQ2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMAREQ2R { bits }
        }
        # [ doc = "Bits 8:11 - Reflects the current level of the Receive FIFO." ]
        # [ inline ( always ) ]
        pub fn rx_level(&self) -> RX_LEVELR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            RX_LEVELR { bits }
        }
        # [ doc = "Bits 16:19 - Reflects the current level of the Transmit FIFO." ]
        # [ inline ( always ) ]
        pub fn tx_level(&self) -> TX_LEVELR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TX_LEVELR { bits }
        }
    }
}
# [ doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1." ]
pub struct DMA1 {
    register: VolatileCell<u32>,
}
# [ doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1." ]
pub mod dma1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DMA1 {
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
    pub struct RX_DMA1_ENABLER {
        bits: bool,
    }
    impl RX_DMA1_ENABLER {
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
    pub struct TX_DMA1_ENABLER {
        bits: bool,
    }
    impl TX_DMA1_ENABLER {
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
    pub struct RX_DEPTH_DMA1R {
        bits: u8,
    }
    impl RX_DEPTH_DMA1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TX_DEPTH_DMA1R {
        bits: u8,
    }
    impl TX_DEPTH_DMA1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RX_DMA1_ENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RX_DMA1_ENABLEW<'a> {
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
    pub struct _TX_DMA1_ENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TX_DMA1_ENABLEW<'a> {
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
    pub struct _RX_DEPTH_DMA1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RX_DEPTH_DMA1W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _TX_DEPTH_DMA1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TX_DEPTH_DMA1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
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
        # [ doc = "Bit 0 - When 1, enables DMA1 for I2S receive." ]
        # [ inline ( always ) ]
        pub fn rx_dma1_enable(&self) -> RX_DMA1_ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RX_DMA1_ENABLER { bits }
        }
        # [ doc = "Bit 1 - When 1, enables DMA1 for I2S transmit." ]
        # [ inline ( always ) ]
        pub fn tx_dma1_enable(&self) -> TX_DMA1_ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TX_DMA1_ENABLER { bits }
        }
        # [ doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1." ]
        # [ inline ( always ) ]
        pub fn rx_depth_dma1(&self) -> RX_DEPTH_DMA1R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            RX_DEPTH_DMA1R { bits }
        }
        # [ doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1." ]
        # [ inline ( always ) ]
        pub fn tx_depth_dma1(&self) -> TX_DEPTH_DMA1R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TX_DEPTH_DMA1R { bits }
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
        # [ doc = "Bit 0 - When 1, enables DMA1 for I2S receive." ]
        # [ inline ( always ) ]
        pub fn rx_dma1_enable(&mut self) -> _RX_DMA1_ENABLEW {
            _RX_DMA1_ENABLEW { w: self }
        }
        # [ doc = "Bit 1 - When 1, enables DMA1 for I2S transmit." ]
        # [ inline ( always ) ]
        pub fn tx_dma1_enable(&mut self) -> _TX_DMA1_ENABLEW {
            _TX_DMA1_ENABLEW { w: self }
        }
        # [ doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1." ]
        # [ inline ( always ) ]
        pub fn rx_depth_dma1(&mut self) -> _RX_DEPTH_DMA1W {
            _RX_DEPTH_DMA1W { w: self }
        }
        # [ doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1." ]
        # [ inline ( always ) ]
        pub fn tx_depth_dma1(&mut self) -> _TX_DEPTH_DMA1W {
            _TX_DEPTH_DMA1W { w: self }
        }
    }
}
# [ doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2." ]
pub struct DMA2 {
    register: VolatileCell<u32>,
}
# [ doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2." ]
pub mod dma2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DMA2 {
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
    pub struct RX_DMA2_ENABLER {
        bits: bool,
    }
    impl RX_DMA2_ENABLER {
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
    pub struct TX_DMA2_ENABLER {
        bits: bool,
    }
    impl TX_DMA2_ENABLER {
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
    pub struct RX_DEPTH_DMA2R {
        bits: u8,
    }
    impl RX_DEPTH_DMA2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TX_DEPTH_DMA2R {
        bits: u8,
    }
    impl TX_DEPTH_DMA2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RX_DMA2_ENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RX_DMA2_ENABLEW<'a> {
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
    pub struct _TX_DMA2_ENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TX_DMA2_ENABLEW<'a> {
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
    pub struct _RX_DEPTH_DMA2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RX_DEPTH_DMA2W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _TX_DEPTH_DMA2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TX_DEPTH_DMA2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
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
        # [ doc = "Bit 0 - When 1, enables DMA1 for I2S receive." ]
        # [ inline ( always ) ]
        pub fn rx_dma2_enable(&self) -> RX_DMA2_ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RX_DMA2_ENABLER { bits }
        }
        # [ doc = "Bit 1 - When 1, enables DMA1 for I2S transmit." ]
        # [ inline ( always ) ]
        pub fn tx_dma2_enable(&self) -> TX_DMA2_ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TX_DMA2_ENABLER { bits }
        }
        # [ doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA2." ]
        # [ inline ( always ) ]
        pub fn rx_depth_dma2(&self) -> RX_DEPTH_DMA2R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            RX_DEPTH_DMA2R { bits }
        }
        # [ doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA2." ]
        # [ inline ( always ) ]
        pub fn tx_depth_dma2(&self) -> TX_DEPTH_DMA2R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TX_DEPTH_DMA2R { bits }
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
        # [ doc = "Bit 0 - When 1, enables DMA1 for I2S receive." ]
        # [ inline ( always ) ]
        pub fn rx_dma2_enable(&mut self) -> _RX_DMA2_ENABLEW {
            _RX_DMA2_ENABLEW { w: self }
        }
        # [ doc = "Bit 1 - When 1, enables DMA1 for I2S transmit." ]
        # [ inline ( always ) ]
        pub fn tx_dma2_enable(&mut self) -> _TX_DMA2_ENABLEW {
            _TX_DMA2_ENABLEW { w: self }
        }
        # [ doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA2." ]
        # [ inline ( always ) ]
        pub fn rx_depth_dma2(&mut self) -> _RX_DEPTH_DMA2W {
            _RX_DEPTH_DMA2W { w: self }
        }
        # [ doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA2." ]
        # [ inline ( always ) ]
        pub fn tx_depth_dma2(&mut self) -> _TX_DEPTH_DMA2W {
            _TX_DEPTH_DMA2W { w: self }
        }
    }
}
# [ doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated." ]
pub struct IRQ {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated." ]
pub mod irq {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::IRQ {
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
    pub struct RX_IRQ_ENABLER {
        bits: bool,
    }
    impl RX_IRQ_ENABLER {
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
    pub struct TX_IRQ_ENABLER {
        bits: bool,
    }
    impl TX_IRQ_ENABLER {
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
    pub struct RX_DEPTH_IRQR {
        bits: u8,
    }
    impl RX_DEPTH_IRQR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TX_DEPTH_IRQR {
        bits: u8,
    }
    impl TX_DEPTH_IRQR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RX_IRQ_ENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RX_IRQ_ENABLEW<'a> {
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
    pub struct _TX_IRQ_ENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TX_IRQ_ENABLEW<'a> {
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
    pub struct _RX_DEPTH_IRQW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RX_DEPTH_IRQW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _TX_DEPTH_IRQW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TX_DEPTH_IRQW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
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
        # [ doc = "Bit 0 - When 1, enables I2S receive interrupt." ]
        # [ inline ( always ) ]
        pub fn rx_irq_enable(&self) -> RX_IRQ_ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RX_IRQ_ENABLER { bits }
        }
        # [ doc = "Bit 1 - When 1, enables I2S transmit interrupt." ]
        # [ inline ( always ) ]
        pub fn tx_irq_enable(&self) -> TX_IRQ_ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TX_IRQ_ENABLER { bits }
        }
        # [ doc = "Bits 8:11 - Set the FIFO level on which to create an irq request." ]
        # [ inline ( always ) ]
        pub fn rx_depth_irq(&self) -> RX_DEPTH_IRQR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            RX_DEPTH_IRQR { bits }
        }
        # [ doc = "Bits 16:19 - Set the FIFO level on which to create an irq request." ]
        # [ inline ( always ) ]
        pub fn tx_depth_irq(&self) -> TX_DEPTH_IRQR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TX_DEPTH_IRQR { bits }
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
        # [ doc = "Bit 0 - When 1, enables I2S receive interrupt." ]
        # [ inline ( always ) ]
        pub fn rx_irq_enable(&mut self) -> _RX_IRQ_ENABLEW {
            _RX_IRQ_ENABLEW { w: self }
        }
        # [ doc = "Bit 1 - When 1, enables I2S transmit interrupt." ]
        # [ inline ( always ) ]
        pub fn tx_irq_enable(&mut self) -> _TX_IRQ_ENABLEW {
            _TX_IRQ_ENABLEW { w: self }
        }
        # [ doc = "Bits 8:11 - Set the FIFO level on which to create an irq request." ]
        # [ inline ( always ) ]
        pub fn rx_depth_irq(&mut self) -> _RX_DEPTH_IRQW {
            _RX_DEPTH_IRQW { w: self }
        }
        # [ doc = "Bits 16:19 - Set the FIFO level on which to create an irq request." ]
        # [ inline ( always ) ]
        pub fn tx_depth_irq(&mut self) -> _TX_DEPTH_IRQW {
            _TX_DEPTH_IRQW { w: self }
        }
    }
}
# [ doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK." ]
pub struct TXRATE {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK." ]
pub mod txrate {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TXRATE {
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
    pub struct Y_DIVIDERR {
        bits: u8,
    }
    impl Y_DIVIDERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct X_DIVIDERR {
        bits: u8,
    }
    impl X_DIVIDERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Y_DIVIDERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _Y_DIVIDERW<'a> {
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
    pub struct _X_DIVIDERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _X_DIVIDERW<'a> {
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
        # [ doc = "Bits 0:7 - I2S transmit MCLK rate denominator. This value is used to divide PCLK to produce the transmit MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock." ]
        # [ inline ( always ) ]
        pub fn y_divider(&self) -> Y_DIVIDERR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            Y_DIVIDERR { bits }
        }
        # [ doc = "Bits 8:15 - I2S transmit MCLK rate numerator. This value is used to multiply PCLK by to produce the transmit MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2." ]
        # [ inline ( always ) ]
        pub fn x_divider(&self) -> X_DIVIDERR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            X_DIVIDERR { bits }
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
        # [ doc = "Bits 0:7 - I2S transmit MCLK rate denominator. This value is used to divide PCLK to produce the transmit MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock." ]
        # [ inline ( always ) ]
        pub fn y_divider(&mut self) -> _Y_DIVIDERW {
            _Y_DIVIDERW { w: self }
        }
        # [ doc = "Bits 8:15 - I2S transmit MCLK rate numerator. This value is used to multiply PCLK by to produce the transmit MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2." ]
        # [ inline ( always ) ]
        pub fn x_divider(&mut self) -> _X_DIVIDERW {
            _X_DIVIDERW { w: self }
        }
    }
}
# [ doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK." ]
pub struct RXRATE {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK." ]
pub mod rxrate {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RXRATE {
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
    pub struct Y_DIVIDERR {
        bits: u8,
    }
    impl Y_DIVIDERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct X_DIVIDERR {
        bits: u8,
    }
    impl X_DIVIDERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Y_DIVIDERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _Y_DIVIDERW<'a> {
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
    pub struct _X_DIVIDERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _X_DIVIDERW<'a> {
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
        # [ doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock." ]
        # [ inline ( always ) ]
        pub fn y_divider(&self) -> Y_DIVIDERR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            Y_DIVIDERR { bits }
        }
        # [ doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2." ]
        # [ inline ( always ) ]
        pub fn x_divider(&self) -> X_DIVIDERR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            X_DIVIDERR { bits }
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
        # [ doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock." ]
        # [ inline ( always ) ]
        pub fn y_divider(&mut self) -> _Y_DIVIDERW {
            _Y_DIVIDERW { w: self }
        }
        # [ doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2." ]
        # [ inline ( always ) ]
        pub fn x_divider(&mut self) -> _X_DIVIDERW {
            _X_DIVIDERW { w: self }
        }
    }
}
# [ doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock." ]
pub struct TXBITRATE {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock." ]
pub mod txbitrate {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TXBITRATE {
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
    pub struct TX_BITRATER {
        bits: u8,
    }
    impl TX_BITRATER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TX_BITRATEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TX_BITRATEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 63;
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
        # [ doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock." ]
        # [ inline ( always ) ]
        pub fn tx_bitrate(&self) -> TX_BITRATER {
            let bits = {
                const MASK: u8 = 63;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TX_BITRATER { bits }
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
        # [ doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock." ]
        # [ inline ( always ) ]
        pub fn tx_bitrate(&mut self) -> _TX_BITRATEW {
            _TX_BITRATEW { w: self }
        }
    }
}
# [ doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock." ]
pub struct RXBITRATE {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock." ]
pub mod rxbitrate {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RXBITRATE {
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
    pub struct RX_BITRATER {
        bits: u8,
    }
    impl RX_BITRATER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RX_BITRATEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RX_BITRATEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 63;
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
        # [ doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock." ]
        # [ inline ( always ) ]
        pub fn rx_bitrate(&self) -> RX_BITRATER {
            let bits = {
                const MASK: u8 = 63;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            RX_BITRATER { bits }
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
        # [ doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock." ]
        # [ inline ( always ) ]
        pub fn rx_bitrate(&mut self) -> _RX_BITRATEW {
            _RX_BITRATEW { w: self }
        }
    }
}
# [ doc = "I2S Transmit mode control." ]
pub struct TXMODE {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Transmit mode control." ]
pub mod txmode {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TXMODE {
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
    # [ doc = "Possible values of the field `TXCLKSEL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TXCLKSELR {
        # [ doc = "Select the TX fractional rate divider clock output as the source" ]
        SELECT_THE_TX_FRACTI,
        # [ doc = "Select the RX_MCLK signal as the TX_MCLK clock source" ]
        SELECT_THE_RX_MCLK_S,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl TXCLKSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                TXCLKSELR::SELECT_THE_TX_FRACTI => 0,
                TXCLKSELR::SELECT_THE_RX_MCLK_S => 2,
                TXCLKSELR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> TXCLKSELR {
            match value {
                0 => TXCLKSELR::SELECT_THE_TX_FRACTI,
                2 => TXCLKSELR::SELECT_THE_RX_MCLK_S,
                i => TXCLKSELR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `SELECT_THE_TX_FRACTI`" ]
        # [ inline ( always ) ]
        pub fn is_select_the_tx_fracti(&self) -> bool {
            *self == TXCLKSELR::SELECT_THE_TX_FRACTI
        }
        # [ doc = "Checks if the value of the field is `SELECT_THE_RX_MCLK_S`" ]
        # [ inline ( always ) ]
        pub fn is_select_the_rx_mclk_s(&self) -> bool {
            *self == TXCLKSELR::SELECT_THE_RX_MCLK_S
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TX4PINR {
        bits: bool,
    }
    impl TX4PINR {
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
    pub struct TXMCENAR {
        bits: bool,
    }
    impl TXMCENAR {
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
    # [ doc = "Values that can be written to the field `TXCLKSEL`" ]
    pub enum TXCLKSELW {
        # [ doc = "Select the TX fractional rate divider clock output as the source" ]
        SELECT_THE_TX_FRACTI,
        # [ doc = "Select the RX_MCLK signal as the TX_MCLK clock source" ]
        SELECT_THE_RX_MCLK_S,
    }
    impl TXCLKSELW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                TXCLKSELW::SELECT_THE_TX_FRACTI => 0,
                TXCLKSELW::SELECT_THE_RX_MCLK_S => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TXCLKSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXCLKSELW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TXCLKSELW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "Select the TX fractional rate divider clock output as the source" ]
        # [ inline ( always ) ]
        pub fn select_the_tx_fracti(self) -> &'a mut W {
            self.variant(TXCLKSELW::SELECT_THE_TX_FRACTI)
        }
        # [ doc = "Select the RX_MCLK signal as the TX_MCLK clock source" ]
        # [ inline ( always ) ]
        pub fn select_the_rx_mclk_s(self) -> &'a mut W {
            self.variant(TXCLKSELW::SELECT_THE_RX_MCLK_S)
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
    # [ doc = r" Proxy" ]
    pub struct _TX4PINW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TX4PINW<'a> {
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
    pub struct _TXMCENAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXMCENAW<'a> {
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
        # [ doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider." ]
        # [ inline ( always ) ]
        pub fn txclksel(&self) -> TXCLKSELR {
            TXCLKSELR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 0;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode." ]
        # [ inline ( always ) ]
        pub fn tx4pin(&self) -> TX4PINR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TX4PINR { bits }
        }
        # [ doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled." ]
        # [ inline ( always ) ]
        pub fn txmcena(&self) -> TXMCENAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TXMCENAR { bits }
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
        # [ doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider." ]
        # [ inline ( always ) ]
        pub fn txclksel(&mut self) -> _TXCLKSELW {
            _TXCLKSELW { w: self }
        }
        # [ doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode." ]
        # [ inline ( always ) ]
        pub fn tx4pin(&mut self) -> _TX4PINW {
            _TX4PINW { w: self }
        }
        # [ doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled." ]
        # [ inline ( always ) ]
        pub fn txmcena(&mut self) -> _TXMCENAW {
            _TXMCENAW { w: self }
        }
    }
}
# [ doc = "I2S Receive mode control." ]
pub struct RXMODE {
    register: VolatileCell<u32>,
}
# [ doc = "I2S Receive mode control." ]
pub mod rxmode {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RXMODE {
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
    # [ doc = "Possible values of the field `RXCLKSEL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RXCLKSELR {
        # [ doc = "Select the RX fractional rate divider clock output as the source" ]
        SELECT_THE_RX_FRACTI,
        # [ doc = "Select the TX_MCLK signal as the RX_MCLK clock source" ]
        SELECT_THE_TX_MCLK_S,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl RXCLKSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                RXCLKSELR::SELECT_THE_RX_FRACTI => 0,
                RXCLKSELR::SELECT_THE_TX_MCLK_S => 2,
                RXCLKSELR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> RXCLKSELR {
            match value {
                0 => RXCLKSELR::SELECT_THE_RX_FRACTI,
                2 => RXCLKSELR::SELECT_THE_TX_MCLK_S,
                i => RXCLKSELR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `SELECT_THE_RX_FRACTI`" ]
        # [ inline ( always ) ]
        pub fn is_select_the_rx_fracti(&self) -> bool {
            *self == RXCLKSELR::SELECT_THE_RX_FRACTI
        }
        # [ doc = "Checks if the value of the field is `SELECT_THE_TX_MCLK_S`" ]
        # [ inline ( always ) ]
        pub fn is_select_the_tx_mclk_s(&self) -> bool {
            *self == RXCLKSELR::SELECT_THE_TX_MCLK_S
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RX4PINR {
        bits: bool,
    }
    impl RX4PINR {
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
    pub struct RXMCENAR {
        bits: bool,
    }
    impl RXMCENAR {
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
    # [ doc = "Values that can be written to the field `RXCLKSEL`" ]
    pub enum RXCLKSELW {
        # [ doc = "Select the RX fractional rate divider clock output as the source" ]
        SELECT_THE_RX_FRACTI,
        # [ doc = "Select the TX_MCLK signal as the RX_MCLK clock source" ]
        SELECT_THE_TX_MCLK_S,
    }
    impl RXCLKSELW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                RXCLKSELW::SELECT_THE_RX_FRACTI => 0,
                RXCLKSELW::SELECT_THE_TX_MCLK_S => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RXCLKSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXCLKSELW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RXCLKSELW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "Select the RX fractional rate divider clock output as the source" ]
        # [ inline ( always ) ]
        pub fn select_the_rx_fracti(self) -> &'a mut W {
            self.variant(RXCLKSELW::SELECT_THE_RX_FRACTI)
        }
        # [ doc = "Select the TX_MCLK signal as the RX_MCLK clock source" ]
        # [ inline ( always ) ]
        pub fn select_the_tx_mclk_s(self) -> &'a mut W {
            self.variant(RXCLKSELW::SELECT_THE_TX_MCLK_S)
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
    # [ doc = r" Proxy" ]
    pub struct _RX4PINW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RX4PINW<'a> {
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
    pub struct _RXMCENAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXMCENAW<'a> {
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
        # [ doc = "Bits 0:1 - Clock source selection for the receive bit clock divider." ]
        # [ inline ( always ) ]
        pub fn rxclksel(&self) -> RXCLKSELR {
            RXCLKSELR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 0;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode." ]
        # [ inline ( always ) ]
        pub fn rx4pin(&self) -> RX4PINR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RX4PINR { bits }
        }
        # [ doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled." ]
        # [ inline ( always ) ]
        pub fn rxmcena(&self) -> RXMCENAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RXMCENAR { bits }
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
        # [ doc = "Bits 0:1 - Clock source selection for the receive bit clock divider." ]
        # [ inline ( always ) ]
        pub fn rxclksel(&mut self) -> _RXCLKSELW {
            _RXCLKSELW { w: self }
        }
        # [ doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode." ]
        # [ inline ( always ) ]
        pub fn rx4pin(&mut self) -> _RX4PINW {
            _RX4PINW { w: self }
        }
        # [ doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled." ]
        # [ inline ( always ) ]
        pub fn rxmcena(&mut self) -> _RXMCENAW {
            _RXMCENAW { w: self }
        }
    }
}
# [ doc = "I2S interface" ]
pub struct I2S {
    register_block: RegisterBlock,
}
impl Deref for I2S {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
