# ! [ doc = "Watchdog Timer (WDT)" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "Watchdog Timer (WDT)" ]
pub const WDT: Peripheral<WDT> = unsafe { Peripheral::new(1073741824) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer." ]
    pub mod_: MOD,
    # [ doc = "0x04 - Watchdog timer constant register. The value in this register determines the time-out value." ]
    pub tc: TC,
    # [ doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC." ]
    pub feed: FEED,
    # [ doc = "0x0c - Watchdog timer value register. This register reads out the current value of the Watchdog timer." ]
    pub tv: TV,
    # [ doc = "0x10 - Watchdog clock select register." ]
    pub clksel: CLKSEL,
}
# [ doc = "Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer." ]
pub struct MOD {
    register: VolatileCell<u32>,
}
# [ doc = "Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer." ]
pub mod mod_ {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MOD {
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
    # [ doc = "Possible values of the field `WDEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum WDENR {
        # [ doc = "The watchdog timer is stopped." ]
        STOP,
        # [ doc = "The watchdog timer is running." ]
        RUN,
    }
    impl WDENR {
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
                WDENR::STOP => false,
                WDENR::RUN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> WDENR {
            match value {
                false => WDENR::STOP,
                true => WDENR::RUN,
            }
        }
        # [ doc = "Checks if the value of the field is `STOP`" ]
        # [ inline ( always ) ]
        pub fn is_stop(&self) -> bool {
            *self == WDENR::STOP
        }
        # [ doc = "Checks if the value of the field is `RUN`" ]
        # [ inline ( always ) ]
        pub fn is_run(&self) -> bool {
            *self == WDENR::RUN
        }
    }
    # [ doc = "Possible values of the field `WDRESET`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum WDRESETR {
        # [ doc = "A watchdog timeout will not cause a chip reset." ]
        NORESET,
        # [ doc = "A watchdog timeout will cause a chip reset." ]
        RESET,
    }
    impl WDRESETR {
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
                WDRESETR::NORESET => false,
                WDRESETR::RESET => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> WDRESETR {
            match value {
                false => WDRESETR::NORESET,
                true => WDRESETR::RESET,
            }
        }
        # [ doc = "Checks if the value of the field is `NORESET`" ]
        # [ inline ( always ) ]
        pub fn is_noreset(&self) -> bool {
            *self == WDRESETR::NORESET
        }
        # [ doc = "Checks if the value of the field is `RESET`" ]
        # [ inline ( always ) ]
        pub fn is_reset(&self) -> bool {
            *self == WDRESETR::RESET
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WDTOFR {
        bits: bool,
    }
    impl WDTOFR {
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
    pub struct WDINTR {
        bits: bool,
    }
    impl WDINTR {
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
    # [ doc = "Values that can be written to the field `WDEN`" ]
    pub enum WDENW {
        # [ doc = "The watchdog timer is stopped." ]
        STOP,
        # [ doc = "The watchdog timer is running." ]
        RUN,
    }
    impl WDENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                WDENW::STOP => false,
                WDENW::RUN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WDENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WDENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: WDENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The watchdog timer is stopped." ]
        # [ inline ( always ) ]
        pub fn stop(self) -> &'a mut W {
            self.variant(WDENW::STOP)
        }
        # [ doc = "The watchdog timer is running." ]
        # [ inline ( always ) ]
        pub fn run(self) -> &'a mut W {
            self.variant(WDENW::RUN)
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
    # [ doc = "Values that can be written to the field `WDRESET`" ]
    pub enum WDRESETW {
        # [ doc = "A watchdog timeout will not cause a chip reset." ]
        NORESET,
        # [ doc = "A watchdog timeout will cause a chip reset." ]
        RESET,
    }
    impl WDRESETW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                WDRESETW::NORESET => false,
                WDRESETW::RESET => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WDRESETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WDRESETW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: WDRESETW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "A watchdog timeout will not cause a chip reset." ]
        # [ inline ( always ) ]
        pub fn noreset(self) -> &'a mut W {
            self.variant(WDRESETW::NORESET)
        }
        # [ doc = "A watchdog timeout will cause a chip reset." ]
        # [ inline ( always ) ]
        pub fn reset(self) -> &'a mut W {
            self.variant(WDRESETW::RESET)
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
    pub struct _WDTOFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WDTOFW<'a> {
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
    pub struct _WDINTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WDINTW<'a> {
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
        # [ doc = "Bit 0 - Watchdog enable bit. This bit is Set Only." ]
        # [ inline ( always ) ]
        pub fn wden(&self) -> WDENR {
            WDENR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 0;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only. See Table 652." ]
        # [ inline ( always ) ]
        pub fn wdreset(&self) -> WDRESETR {
            WDRESETR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 1;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software." ]
        # [ inline ( always ) ]
        pub fn wdtof(&self) -> WDTOFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WDTOFR { bits }
        }
        # [ doc = "Bit 3 - Watchdog interrupt flag. Cleared by software." ]
        # [ inline ( always ) ]
        pub fn wdint(&self) -> WDINTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WDINTR { bits }
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
        # [ doc = "Bit 0 - Watchdog enable bit. This bit is Set Only." ]
        # [ inline ( always ) ]
        pub fn wden(&mut self) -> _WDENW {
            _WDENW { w: self }
        }
        # [ doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only. See Table 652." ]
        # [ inline ( always ) ]
        pub fn wdreset(&mut self) -> _WDRESETW {
            _WDRESETW { w: self }
        }
        # [ doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software." ]
        # [ inline ( always ) ]
        pub fn wdtof(&mut self) -> _WDTOFW {
            _WDTOFW { w: self }
        }
        # [ doc = "Bit 3 - Watchdog interrupt flag. Cleared by software." ]
        # [ inline ( always ) ]
        pub fn wdint(&mut self) -> _WDINTW {
            _WDINTW { w: self }
        }
    }
}
# [ doc = "Watchdog timer constant register. The value in this register determines the time-out value." ]
pub struct TC {
    register: VolatileCell<u32>,
}
# [ doc = "Watchdog timer constant register. The value in this register determines the time-out value." ]
pub mod tc {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TC {
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
    pub struct COUNTR {
        bits: u32,
    }
    impl COUNTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _COUNTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _COUNTW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:31 - Watchdog time-out interval." ]
        # [ inline ( always ) ]
        pub fn count(&self) -> COUNTR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            COUNTR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 255 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:31 - Watchdog time-out interval." ]
        # [ inline ( always ) ]
        pub fn count(&mut self) -> _COUNTW {
            _COUNTW { w: self }
        }
    }
}
# [ doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC." ]
pub struct FEED {
    register: VolatileCell<u32>,
}
# [ doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC." ]
pub mod feed {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::FEED {
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
    pub struct _FEEDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FEEDW<'a> {
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
        # [ doc = "Bits 0:7 - Feed value should be 0xAA followed by 0x55." ]
        # [ inline ( always ) ]
        pub fn feed(&mut self) -> _FEEDW {
            _FEEDW { w: self }
        }
    }
}
# [ doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer." ]
pub struct TV {
    register: VolatileCell<u32>,
}
# [ doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer." ]
pub mod tv {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::TV {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct COUNTR {
        bits: u32,
    }
    impl COUNTR {
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
        # [ doc = "Bits 0:31 - Counter timer value." ]
        # [ inline ( always ) ]
        pub fn count(&self) -> COUNTR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            COUNTR { bits }
        }
    }
}
# [ doc = "Watchdog clock select register." ]
pub struct CLKSEL {
    register: VolatileCell<u32>,
}
# [ doc = "Watchdog clock select register." ]
pub mod clksel {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CLKSEL {
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
    # [ doc = "Possible values of the field `CLKSEL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CLKSELR {
        # [ doc = "IRC" ]
        IRC,
        # [ doc = "Peripheral clock" ]
        PCLK,
        # [ doc = "RTC oscillator" ]
        RTCOSC,
    }
    impl CLKSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                CLKSELR::IRC => 0,
                CLKSELR::PCLK => 1,
                CLKSELR::RTCOSC => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> CLKSELR {
            match value {
                0 => CLKSELR::IRC,
                1 => CLKSELR::PCLK,
                2 => CLKSELR::RTCOSC,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `IRC`" ]
        # [ inline ( always ) ]
        pub fn is_irc(&self) -> bool {
            *self == CLKSELR::IRC
        }
        # [ doc = "Checks if the value of the field is `PCLK`" ]
        # [ inline ( always ) ]
        pub fn is_pclk(&self) -> bool {
            *self == CLKSELR::PCLK
        }
        # [ doc = "Checks if the value of the field is `RTCOSC`" ]
        # [ inline ( always ) ]
        pub fn is_rtcosc(&self) -> bool {
            *self == CLKSELR::RTCOSC
        }
    }
    # [ doc = "Possible values of the field `LOCK`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum LOCKR {
        # [ doc = "This bit is set to 0 on any reset. It cannot be cleared by software." ]
        UNLOCKED,
        # [ doc = "Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register\n\t\t\t\t\t\t\t\t\t\tcannot be modified." ]
        LOCKED,
    }
    impl LOCKR {
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
                LOCKR::UNLOCKED => false,
                LOCKR::LOCKED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> LOCKR {
            match value {
                false => LOCKR::UNLOCKED,
                true => LOCKR::LOCKED,
            }
        }
        # [ doc = "Checks if the value of the field is `UNLOCKED`" ]
        # [ inline ( always ) ]
        pub fn is_unlocked(&self) -> bool {
            *self == LOCKR::UNLOCKED
        }
        # [ doc = "Checks if the value of the field is `LOCKED`" ]
        # [ inline ( always ) ]
        pub fn is_locked(&self) -> bool {
            *self == LOCKR::LOCKED
        }
    }
    # [ doc = "Values that can be written to the field `CLKSEL`" ]
    pub enum CLKSELW {
        # [ doc = "IRC" ]
        IRC,
        # [ doc = "Peripheral clock" ]
        PCLK,
        # [ doc = "RTC oscillator" ]
        RTCOSC,
    }
    impl CLKSELW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                CLKSELW::IRC => 0,
                CLKSELW::PCLK => 1,
                CLKSELW::RTCOSC => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKSELW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CLKSELW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "IRC" ]
        # [ inline ( always ) ]
        pub fn irc(self) -> &'a mut W {
            self.variant(CLKSELW::IRC)
        }
        # [ doc = "Peripheral clock" ]
        # [ inline ( always ) ]
        pub fn pclk(self) -> &'a mut W {
            self.variant(CLKSELW::PCLK)
        }
        # [ doc = "RTC oscillator" ]
        # [ inline ( always ) ]
        pub fn rtcosc(self) -> &'a mut W {
            self.variant(CLKSELW::RTCOSC)
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
    # [ doc = "Values that can be written to the field `LOCK`" ]
    pub enum LOCKW {
        # [ doc = "This bit is set to 0 on any reset. It cannot be cleared by software." ]
        UNLOCKED,
        # [ doc = "Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register\n\t\t\t\t\t\t\t\t\t\tcannot be modified." ]
        LOCKED,
    }
    impl LOCKW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                LOCKW::UNLOCKED => false,
                LOCKW::LOCKED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LOCKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LOCKW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: LOCKW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "This bit is set to 0 on any reset. It cannot be cleared by software." ]
        # [ inline ( always ) ]
        pub fn unlocked(self) -> &'a mut W {
            self.variant(LOCKW::UNLOCKED)
        }
        # [ doc = "Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register cannot be modified." ]
        # [ inline ( always ) ]
        pub fn locked(self) -> &'a mut W {
            self.variant(LOCKW::LOCKED)
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
        # [ doc = "Bits 0:1 - Selects source of WDT clock" ]
        # [ inline ( always ) ]
        pub fn clksel(&self) -> CLKSELR {
            CLKSELR::_from({
                               const MASK: u8 = 3;
                               const OFFSET: u8 = 0;
                               ((self.bits >> OFFSET) & MASK as u32) as u8
                           })
        }
        # [ doc = "Bit 31 - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0." ]
        # [ inline ( always ) ]
        pub fn lock(&self) -> LOCKR {
            LOCKR::_from({
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
        # [ doc = "Bits 0:1 - Selects source of WDT clock" ]
        # [ inline ( always ) ]
        pub fn clksel(&mut self) -> _CLKSELW {
            _CLKSELW { w: self }
        }
        # [ doc = "Bit 31 - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0." ]
        # [ inline ( always ) ]
        pub fn lock(&mut self) -> _LOCKW {
            _LOCKW { w: self }
        }
    }
}
# [ doc = "Watchdog Timer (WDT)" ]
pub struct WDT {
    register_block: RegisterBlock,
}
impl Deref for WDT {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
