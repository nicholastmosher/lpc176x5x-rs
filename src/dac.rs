# ! [ doc = "Digital-to-Analog Converter (DAC)" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "Digital-to-Analog Converter (DAC)" ]
pub const DAC: Peripheral<DAC> = unsafe { Peripheral::new(1074315264) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit." ]
    pub dacr: DACR,
    # [ doc = "0x04 - DAC Control register. This register controls DMA and timer operation." ]
    pub ctrl: CTRL,
    # [ doc = "0x08 - DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer." ]
    pub cntval: CNTVAL,
}
# [ doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit." ]
pub struct DACR {
    register: VolatileCell<u32>,
}
# [ doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit." ]
pub mod dacr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DACR {
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
    pub struct VALUER {
        bits: u16,
    }
    impl VALUER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Possible values of the field `BIAS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum BIASR {
        # [ doc = "The settling time of the DAC is 1 us max, and the maximum current is 700 uA. This allows a maximum update rate of 1 MHz." ]
        FAST,
        # [ doc = "The settling time of the DAC is 2.5 us and the maximum current is 350 uA. This allows a maximum update rate of 400 kHz." ]
        SLOW,
    }
    impl BIASR {
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
                BIASR::FAST => false,
                BIASR::SLOW => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> BIASR {
            match value {
                false => BIASR::FAST,
                true => BIASR::SLOW,
            }
        }
        # [ doc = "Checks if the value of the field is `FAST`" ]
        # [ inline ( always ) ]
        pub fn is_fast(&self) -> bool {
            *self == BIASR::FAST
        }
        # [ doc = "Checks if the value of the field is `SLOW`" ]
        # [ inline ( always ) ]
        pub fn is_slow(&self) -> bool {
            *self == BIASR::SLOW
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _VALUEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _VALUEW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `BIAS`" ]
    pub enum BIASW {
        # [ doc = "The settling time of the DAC is 1 us max, and the maximum current is 700 uA. This allows a maximum update rate of 1 MHz." ]
        FAST,
        # [ doc = "The settling time of the DAC is 2.5 us and the maximum current is 350 uA. This allows a maximum update rate of 400 kHz." ]
        SLOW,
    }
    impl BIASW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                BIASW::FAST => false,
                BIASW::SLOW => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BIASW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BIASW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BIASW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The settling time of the DAC is 1 us max, and the maximum current is 700 uA. This allows a maximum update rate of 1 MHz." ]
        # [ inline ( always ) ]
        pub fn fast(self) -> &'a mut W {
            self.variant(BIASW::FAST)
        }
        # [ doc = "The settling time of the DAC is 2.5 us and the maximum current is 350 uA. This allows a maximum update rate of 400 kHz." ]
        # [ inline ( always ) ]
        pub fn slow(self) -> &'a mut W {
            self.variant(BIASW::SLOW)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 6:15 - After the selected settling time after this field is written with a new VALUE, the voltage on the DAC_OUT pin (with respect to VSSA) is VALUE x ((VREFP - V REFN)/1024) + VREFN." ]
        # [ inline ( always ) ]
        pub fn value(&self) -> VALUER {
            let bits = {
                const MASK: u16 = 1023;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            VALUER { bits }
        }
        # [ doc = "Bit 16 - Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet." ]
        # [ inline ( always ) ]
        pub fn bias(&self) -> BIASR {
            BIASR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 16;
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
        # [ doc = "Bits 6:15 - After the selected settling time after this field is written with a new VALUE, the voltage on the DAC_OUT pin (with respect to VSSA) is VALUE x ((VREFP - V REFN)/1024) + VREFN." ]
        # [ inline ( always ) ]
        pub fn value(&mut self) -> _VALUEW {
            _VALUEW { w: self }
        }
        # [ doc = "Bit 16 - Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet." ]
        # [ inline ( always ) ]
        pub fn bias(&mut self) -> _BIASW {
            _BIASW { w: self }
        }
    }
}
# [ doc = "DAC Control register. This register controls DMA and timer operation." ]
pub struct CTRL {
    register: VolatileCell<u32>,
}
# [ doc = "DAC Control register. This register controls DMA and timer operation." ]
pub mod ctrl {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CTRL {
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
    # [ doc = "Possible values of the field `INT_DMA_REQ`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum INT_DMA_REQR {
        # [ doc = "Clear on any write to the DACR register." ]
        CLEAR_ON_ANY_WRITE_T,
        # [ doc = "Set by hardware when the timer times out." ]
        SET_BY_HARDWARE_WHEN,
    }
    impl INT_DMA_REQR {
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
                INT_DMA_REQR::CLEAR_ON_ANY_WRITE_T => false,
                INT_DMA_REQR::SET_BY_HARDWARE_WHEN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> INT_DMA_REQR {
            match value {
                false => INT_DMA_REQR::CLEAR_ON_ANY_WRITE_T,
                true => INT_DMA_REQR::SET_BY_HARDWARE_WHEN,
            }
        }
        # [ doc = "Checks if the value of the field is `CLEAR_ON_ANY_WRITE_T`" ]
        # [ inline ( always ) ]
        pub fn is_clear_on_any_write_t(&self) -> bool {
            *self == INT_DMA_REQR::CLEAR_ON_ANY_WRITE_T
        }
        # [ doc = "Checks if the value of the field is `SET_BY_HARDWARE_WHEN`" ]
        # [ inline ( always ) ]
        pub fn is_set_by_hardware_when(&self) -> bool {
            *self == INT_DMA_REQR::SET_BY_HARDWARE_WHEN
        }
    }
    # [ doc = "Possible values of the field `DBLBUF_ENA`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DBLBUF_ENAR {
        # [ doc = "Disable" ]
        DISABLE,
        # [ doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter." ]
        ENABLE_WHEN_THIS_BI,
    }
    impl DBLBUF_ENAR {
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
                DBLBUF_ENAR::DISABLE => false,
                DBLBUF_ENAR::ENABLE_WHEN_THIS_BI => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DBLBUF_ENAR {
            match value {
                false => DBLBUF_ENAR::DISABLE,
                true => DBLBUF_ENAR::ENABLE_WHEN_THIS_BI,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == DBLBUF_ENAR::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE_WHEN_THIS_BI`" ]
        # [ inline ( always ) ]
        pub fn is_enable_when_this_bi(&self) -> bool {
            *self == DBLBUF_ENAR::ENABLE_WHEN_THIS_BI
        }
    }
    # [ doc = "Possible values of the field `CNT_ENA`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CNT_ENAR {
        # [ doc = "Disable" ]
        DISABLE,
        # [ doc = "Enable" ]
        ENABLE,
    }
    impl CNT_ENAR {
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
                CNT_ENAR::DISABLE => false,
                CNT_ENAR::ENABLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CNT_ENAR {
            match value {
                false => CNT_ENAR::DISABLE,
                true => CNT_ENAR::ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == CNT_ENAR::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == CNT_ENAR::ENABLE
        }
    }
    # [ doc = "Possible values of the field `DMA_ENA`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DMA_ENAR {
        # [ doc = "Disable" ]
        DISABLE,
        # [ doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)." ]
        ENABLE_DMA_BURST_RE,
    }
    impl DMA_ENAR {
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
                DMA_ENAR::DISABLE => false,
                DMA_ENAR::ENABLE_DMA_BURST_RE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DMA_ENAR {
            match value {
                false => DMA_ENAR::DISABLE,
                true => DMA_ENAR::ENABLE_DMA_BURST_RE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == DMA_ENAR::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE_DMA_BURST_RE`" ]
        # [ inline ( always ) ]
        pub fn is_enable_dma_burst_re(&self) -> bool {
            *self == DMA_ENAR::ENABLE_DMA_BURST_RE
        }
    }
    # [ doc = "Values that can be written to the field `INT_DMA_REQ`" ]
    pub enum INT_DMA_REQW {
        # [ doc = "Clear on any write to the DACR register." ]
        CLEAR_ON_ANY_WRITE_T,
        # [ doc = "Set by hardware when the timer times out." ]
        SET_BY_HARDWARE_WHEN,
    }
    impl INT_DMA_REQW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                INT_DMA_REQW::CLEAR_ON_ANY_WRITE_T => false,
                INT_DMA_REQW::SET_BY_HARDWARE_WHEN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _INT_DMA_REQW<'a> {
        w: &'a mut W,
    }
    impl<'a> _INT_DMA_REQW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: INT_DMA_REQW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Clear on any write to the DACR register." ]
        # [ inline ( always ) ]
        pub fn clear_on_any_write_t(self) -> &'a mut W {
            self.variant(INT_DMA_REQW::CLEAR_ON_ANY_WRITE_T)
        }
        # [ doc = "Set by hardware when the timer times out." ]
        # [ inline ( always ) ]
        pub fn set_by_hardware_when(self) -> &'a mut W {
            self.variant(INT_DMA_REQW::SET_BY_HARDWARE_WHEN)
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
    # [ doc = "Values that can be written to the field `DBLBUF_ENA`" ]
    pub enum DBLBUF_ENAW {
        # [ doc = "Disable" ]
        DISABLE,
        # [ doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter." ]
        ENABLE_WHEN_THIS_BI,
    }
    impl DBLBUF_ENAW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                DBLBUF_ENAW::DISABLE => false,
                DBLBUF_ENAW::ENABLE_WHEN_THIS_BI => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DBLBUF_ENAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DBLBUF_ENAW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: DBLBUF_ENAW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable" ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(DBLBUF_ENAW::DISABLE)
        }
        # [ doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter." ]
        # [ inline ( always ) ]
        pub fn enable_when_this_bi(self) -> &'a mut W {
            self.variant(DBLBUF_ENAW::ENABLE_WHEN_THIS_BI)
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
    # [ doc = "Values that can be written to the field `CNT_ENA`" ]
    pub enum CNT_ENAW {
        # [ doc = "Disable" ]
        DISABLE,
        # [ doc = "Enable" ]
        ENABLE,
    }
    impl CNT_ENAW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CNT_ENAW::DISABLE => false,
                CNT_ENAW::ENABLE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CNT_ENAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNT_ENAW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CNT_ENAW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable" ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(CNT_ENAW::DISABLE)
        }
        # [ doc = "Enable" ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(CNT_ENAW::ENABLE)
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
    # [ doc = "Values that can be written to the field `DMA_ENA`" ]
    pub enum DMA_ENAW {
        # [ doc = "Disable" ]
        DISABLE,
        # [ doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)." ]
        ENABLE_DMA_BURST_RE,
    }
    impl DMA_ENAW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                DMA_ENAW::DISABLE => false,
                DMA_ENAW::ENABLE_DMA_BURST_RE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DMA_ENAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMA_ENAW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: DMA_ENAW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable" ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(DMA_ENAW::DISABLE)
        }
        # [ doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)." ]
        # [ inline ( always ) ]
        pub fn enable_dma_burst_re(self) -> &'a mut W {
            self.variant(DMA_ENAW::ENABLE_DMA_BURST_RE)
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
        # [ doc = "Bit 0 - DMA interrupt request" ]
        # [ inline ( always ) ]
        pub fn int_dma_req(&self) -> INT_DMA_REQR {
            INT_DMA_REQR::_from({
                                    const MASK: bool = true;
                                    const OFFSET: u8 = 0;
                                    ((self.bits >> OFFSET) & MASK as u32) != 0
                                })
        }
        # [ doc = "Bit 1 - Double buffering" ]
        # [ inline ( always ) ]
        pub fn dblbuf_ena(&self) -> DBLBUF_ENAR {
            DBLBUF_ENAR::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 1;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 2 - Time-out counter operation" ]
        # [ inline ( always ) ]
        pub fn cnt_ena(&self) -> CNT_ENAR {
            CNT_ENAR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 2;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 3 - DMA access" ]
        # [ inline ( always ) ]
        pub fn dma_ena(&self) -> DMA_ENAR {
            DMA_ENAR::_from({
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
        # [ doc = "Bit 0 - DMA interrupt request" ]
        # [ inline ( always ) ]
        pub fn int_dma_req(&mut self) -> _INT_DMA_REQW {
            _INT_DMA_REQW { w: self }
        }
        # [ doc = "Bit 1 - Double buffering" ]
        # [ inline ( always ) ]
        pub fn dblbuf_ena(&mut self) -> _DBLBUF_ENAW {
            _DBLBUF_ENAW { w: self }
        }
        # [ doc = "Bit 2 - Time-out counter operation" ]
        # [ inline ( always ) ]
        pub fn cnt_ena(&mut self) -> _CNT_ENAW {
            _CNT_ENAW { w: self }
        }
        # [ doc = "Bit 3 - DMA access" ]
        # [ inline ( always ) ]
        pub fn dma_ena(&mut self) -> _DMA_ENAW {
            _DMA_ENAW { w: self }
        }
    }
}
# [ doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer." ]
pub struct CNTVAL {
    register: VolatileCell<u32>,
}
# [ doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer." ]
pub mod cntval {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CNTVAL {
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
    pub struct VALUER {
        bits: u16,
    }
    impl VALUER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _VALUEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _VALUEW<'a> {
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
        # [ doc = "Bits 0:15 - 16-bit reload value for the DAC interrupt/DMA timer." ]
        # [ inline ( always ) ]
        pub fn value(&self) -> VALUER {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            VALUER { bits }
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
        # [ doc = "Bits 0:15 - 16-bit reload value for the DAC interrupt/DMA timer." ]
        # [ inline ( always ) ]
        pub fn value(&mut self) -> _VALUEW {
            _VALUEW { w: self }
        }
    }
}
# [ doc = "Digital-to-Analog Converter (DAC)" ]
pub struct DAC {
    register_block: RegisterBlock,
}
impl Deref for DAC {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
