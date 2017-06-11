#![doc = "Quadrature Encoder Interface (QEI)"]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

#[doc = "Quadrature Encoder Interface (QEI)"]
pub const QEI: Peripheral<QEI> = unsafe { Peripheral::new(1074511872) };

use vcell::VolatileCell;

#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub con: CON,
    #[doc = "0x04 - Status register"]
    pub stat: STAT,
    #[doc = "0x08 - Configuration register"]
    pub conf: CONF,
    #[doc = "0x0c - Position register"]
    pub pos: POS,
    #[doc = "0x10 - Maximum position register"]
    pub maxpos: MAXPOS,
    #[doc = "0x14 - Position compare register 0"]
    pub cmpos0: CMPOS0,
    #[doc = "0x18 - Position compare register 1"]
    pub cmpos1: CMPOS1,
    #[doc = "0x1c - Position compare register 2"]
    pub cmpos2: CMPOS2,
    #[doc = "0x20 - Index count register 0"]
    pub inxcnt: INXCNT,
    #[doc = "0x24 - Index compare register 0"]
    pub inxcmp0: INXCMP0,
    #[doc = "0x28 - Velocity timer reload register"]
    pub load: LOAD,
    #[doc = "0x2c - Velocity timer register"]
    pub time: TIME,
    #[doc = "0x30 - Velocity counter register"]
    pub vel: VEL,
    #[doc = "0x34 - Velocity capture register"]
    pub cap: CAP,
    #[doc = "0x38 - Velocity compare register"]
    pub velcomp: VELCOMP,
    #[doc = "0x3c - Digital filter register"]
    pub filter: FILTER,
    _reserved0: [u8; 3992usize],
    #[doc = "0xfd8 - Interrupt enable clear register"]
    pub iec: ies::IEC,
    #[doc = "0xfdc - Interrupt enable set register"]
    pub ies: IES,
    #[doc = "0xfe0 - Interrupt status register"]
    pub intstat: INTSTAT,
    #[doc = "0xfe4 - Interrupt enable register"]
    pub ie: IE,
    #[doc = "0xfe8 - Interrupt status clear register"]
    pub clr: CLR,
    #[doc = "0xfec - Interrupt status set register"]
    pub set: SET,
}

#[doc = "Control register"]
pub struct CON {
    register: VolatileCell<u32>,
}

#[doc = "Control register"]
pub mod con {
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::CON {
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
    }

    #[doc = r" Proxy"]
    pub struct _RESPW<'a> {
        w: &'a mut W,
    }

    impl<'a> _RESPW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _RESPIW<'a> {
        w: &'a mut W,
    }

    impl<'a> _RESPIW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _RESVW<'a> {
        w: &'a mut W,
    }

    impl<'a> _RESVW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _RESIW<'a> {
        w: &'a mut W,
    }

    impl<'a> _RESIW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Reset position counter. When set = 1, resets the position counter to all zeros. Autoclears when the position counter is cleared."]
        #[inline(always)]
        pub fn resp(&mut self) -> _RESPW {
            _RESPW { w: self }
        }
        #[doc = "Bit 1 - Reset position counter on index. When set = 1, resets the position counter to all zeros once only the first time an index pulse occurs. Autoclears when the position counter is cleared."]
        #[inline(always)]
        pub fn respi(&mut self) -> _RESPIW {
            _RESPIW { w: self }
        }
        #[doc = "Bit 2 - Reset velocity. When set = 1, resets the velocity counter to all zeros, reloads the velocity timer, and presets the velocity compare register. Autoclears when the velocity counter is cleared."]
        #[inline(always)]
        pub fn resv(&mut self) -> _RESVW {
            _RESVW { w: self }
        }
        #[doc = "Bit 3 - Reset index counter. When set = 1, resets the index counter to all zeros. Autoclears when the index counter is cleared."]
        #[inline(always)]
        pub fn resi(&mut self) -> _RESIW {
            _RESIW { w: self }
        }
    }
}

#[doc = "Configuration register"]
pub struct CONF {
    register: VolatileCell<u32>,
}

#[doc = "Configuration register"]
pub mod conf {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::CONF {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }

    #[doc = r" Value of the field"]
    pub struct DIRINVR {
        bits: bool,
    }

    impl DIRINVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct SIGMODER {
        bits: bool,
    }

    impl SIGMODER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct CAPMODER {
        bits: bool,
    }

    impl CAPMODER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct INVINXR {
        bits: bool,
    }

    impl INVINXR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct CRESPIR {
        bits: bool,
    }

    impl CRESPIR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct INXGATER {
        bits: u8,
    }

    impl INXGATER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }

    #[doc = r" Proxy"]
    pub struct _DIRINVW<'a> {
        w: &'a mut W,
    }

    impl<'a> _DIRINVW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _SIGMODEW<'a> {
        w: &'a mut W,
    }

    impl<'a> _SIGMODEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _CAPMODEW<'a> {
        w: &'a mut W,
    }

    impl<'a> _CAPMODEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _INVINXW<'a> {
        w: &'a mut W,
    }

    impl<'a> _INVINXW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _CRESPIW<'a> {
        w: &'a mut W,
    }

    impl<'a> _CRESPIW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _INXGATEW<'a> {
        w: &'a mut W,
    }

    impl<'a> _INXGATEW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
        #[inline(always)]
        pub fn dirinv(&self) -> DIRINVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DIRINVR { bits }
        }
        #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
        #[inline(always)]
        pub fn sigmode(&self) -> SIGMODER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SIGMODER { bits }
        }
        #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
        #[inline(always)]
        pub fn capmode(&self) -> CAPMODER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAPMODER { bits }
        }
        #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
        #[inline(always)]
        pub fn invinx(&self) -> INVINXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INVINXR { bits }
        }
        #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
        #[inline(always)]
        pub fn crespi(&self) -> CRESPIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CRESPIR { bits }
        }
        #[doc = "Bits 16:19 - Index gating configuration: When INXGATE[16] = 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE[17] = 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE[18] = 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE[19] = 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
        #[inline(always)]
        pub fn inxgate(&self) -> INXGATER {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            INXGATER { bits }
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
        #[inline(always)]
        pub fn dirinv(&mut self) -> _DIRINVW {
            _DIRINVW { w: self }
        }
        #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
        #[inline(always)]
        pub fn sigmode(&mut self) -> _SIGMODEW {
            _SIGMODEW { w: self }
        }
        #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
        #[inline(always)]
        pub fn capmode(&mut self) -> _CAPMODEW {
            _CAPMODEW { w: self }
        }
        #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
        #[inline(always)]
        pub fn invinx(&mut self) -> _INVINXW {
            _INVINXW { w: self }
        }
        #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
        #[inline(always)]
        pub fn crespi(&mut self) -> _CRESPIW {
            _CRESPIW { w: self }
        }
        #[doc = "Bits 16:19 - Index gating configuration: When INXGATE[16] = 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE[17] = 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE[18] = 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE[19] = 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
        #[inline(always)]
        pub fn inxgate(&mut self) -> _INXGATEW {
            _INXGATEW { w: self }
        }
    }
}

#[doc = "Status register"]
pub struct STAT {
    register: VolatileCell<u32>,
}

#[doc = "Status register"]
pub mod stat {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    impl super::STAT {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }

    #[doc = r" Value of the field"]
    pub struct DIRR {
        bits: bool,
    }

    impl DIRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - Direction bit. In combination with DIRINV bit indicates forward or reverse direction. See Table 597."]
        #[inline(always)]
        pub fn dir(&self) -> DIRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DIRR { bits }
        }
    }
}

#[doc = "Position register"]
pub struct POS {
    register: VolatileCell<u32>,
}

#[doc = "Position register"]
pub mod pos {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    impl super::POS {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }

    #[doc = r" Value of the field"]
    pub struct POSR {
        bits: u32,
    }

    impl POSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Current position value."]
        #[inline(always)]
        pub fn pos(&self) -> POSR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            POSR { bits }
        }
    }
}

#[doc = "Maximum position register"]
pub struct MAXPOS {
    register: VolatileCell<u32>,
}

#[doc = "Maximum position register"]
pub mod maxpos {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::MAXPOS {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }

    #[doc = r" Value of the field"]
    pub struct MAXPOSR {
        bits: u32,
    }

    impl MAXPOSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    #[doc = r" Proxy"]
    pub struct _MAXPOSW<'a> {
        w: &'a mut W,
    }

    impl<'a> _MAXPOSW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Current maximum position value."]
        #[inline(always)]
        pub fn maxpos(&self) -> MAXPOSR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            MAXPOSR { bits }
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:31 - Current maximum position value."]
        #[inline(always)]
        pub fn maxpos(&mut self) -> _MAXPOSW {
            _MAXPOSW { w: self }
        }
    }
}

#[doc = "Position compare register 0"]
pub struct CMPOS0 {
    register: VolatileCell<u32>,
}

#[doc = "Position compare register 0"]
pub mod cmpos0 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::CMPOS0 {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }

    #[doc = r" Value of the field"]
    pub struct PCMP0R {
        bits: u32,
    }

    impl PCMP0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    #[doc = r" Proxy"]
    pub struct _PCMP0W<'a> {
        w: &'a mut W,
    }

    impl<'a> _PCMP0W<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Position compare value 0."]
        #[inline(always)]
        pub fn pcmp0(&self) -> PCMP0R {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            PCMP0R { bits }
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:31 - Position compare value 0."]
        #[inline(always)]
        pub fn pcmp0(&mut self) -> _PCMP0W {
            _PCMP0W { w: self }
        }
    }
}

#[doc = "Position compare register 1"]
pub struct CMPOS1 {
    register: VolatileCell<u32>,
}

#[doc = "Position compare register 1"]
pub mod cmpos1 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::CMPOS1 {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }

    #[doc = r" Value of the field"]
    pub struct PCMP1R {
        bits: u32,
    }

    impl PCMP1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    #[doc = r" Proxy"]
    pub struct _PCMP1W<'a> {
        w: &'a mut W,
    }

    impl<'a> _PCMP1W<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Position compare value 1."]
        #[inline(always)]
        pub fn pcmp1(&self) -> PCMP1R {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            PCMP1R { bits }
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:31 - Position compare value 1."]
        #[inline(always)]
        pub fn pcmp1(&mut self) -> _PCMP1W {
            _PCMP1W { w: self }
        }
    }
}

#[doc = "Position compare register 2"]
pub struct CMPOS2 {
    register: VolatileCell<u32>,
}

#[doc = "Position compare register 2"]
pub mod cmpos2 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::CMPOS2 {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }

    #[doc = r" Value of the field"]
    pub struct PCMP2R {
        bits: u32,
    }

    impl PCMP2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    #[doc = r" Proxy"]
    pub struct _PCMP2W<'a> {
        w: &'a mut W,
    }

    impl<'a> _PCMP2W<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Position compare value 2."]
        #[inline(always)]
        pub fn pcmp2(&self) -> PCMP2R {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            PCMP2R { bits }
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:31 - Position compare value 2."]
        #[inline(always)]
        pub fn pcmp2(&mut self) -> _PCMP2W {
            _PCMP2W { w: self }
        }
    }
}

#[doc = "Index count register 0"]
pub struct INXCNT {
    register: VolatileCell<u32>,
}

#[doc = "Index count register 0"]
pub mod inxcnt {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    impl super::INXCNT {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }

    #[doc = r" Value of the field"]
    pub struct ENCPOSR {
        bits: u32,
    }

    impl ENCPOSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Current index counter value."]
        #[inline(always)]
        pub fn encpos(&self) -> ENCPOSR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            ENCPOSR { bits }
        }
    }
}

#[doc = "Index compare register 0"]
pub struct INXCMP0 {
    register: VolatileCell<u32>,
}

#[doc = "Index compare register 0"]
pub mod inxcmp0 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::INXCMP0 {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }

    #[doc = r" Value of the field"]
    pub struct ICMP0R {
        bits: u32,
    }

    impl ICMP0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    #[doc = r" Proxy"]
    pub struct _ICMP0W<'a> {
        w: &'a mut W,
    }

    impl<'a> _ICMP0W<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Index compare value 0."]
        #[inline(always)]
        pub fn icmp0(&self) -> ICMP0R {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            ICMP0R { bits }
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:31 - Index compare value 0."]
        #[inline(always)]
        pub fn icmp0(&mut self) -> _ICMP0W {
            _ICMP0W { w: self }
        }
    }
}

#[doc = "Velocity timer reload register"]
pub struct LOAD {
    register: VolatileCell<u32>,
}

#[doc = "Velocity timer reload register"]
pub mod load {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::LOAD {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }

    #[doc = r" Value of the field"]
    pub struct VELLOADR {
        bits: u32,
    }

    impl VELLOADR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    #[doc = r" Proxy"]
    pub struct _VELLOADW<'a> {
        w: &'a mut W,
    }

    impl<'a> _VELLOADW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Current velocity timer load value."]
        #[inline(always)]
        pub fn velload(&self) -> VELLOADR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            VELLOADR { bits }
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:31 - Current velocity timer load value."]
        #[inline(always)]
        pub fn velload(&mut self) -> _VELLOADW {
            _VELLOADW { w: self }
        }
    }
}

#[doc = "Velocity timer register"]
pub struct TIME {
    register: VolatileCell<u32>,
}

#[doc = "Velocity timer register"]
pub mod time {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    impl super::TIME {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }

    #[doc = r" Value of the field"]
    pub struct VELVALR {
        bits: u32,
    }

    impl VELVALR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Current velocity timer value."]
        #[inline(always)]
        pub fn velval(&self) -> VELVALR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            VELVALR { bits }
        }
    }
}

#[doc = "Velocity counter register"]
pub struct VEL {
    register: VolatileCell<u32>,
}

#[doc = "Velocity counter register"]
pub mod vel {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    impl super::VEL {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }

    #[doc = r" Value of the field"]
    pub struct VELPCR {
        bits: u32,
    }

    impl VELPCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Current velocity pulse count."]
        #[inline(always)]
        pub fn velpc(&self) -> VELPCR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            VELPCR { bits }
        }
    }
}

#[doc = "Velocity capture register"]
pub struct CAP {
    register: VolatileCell<u32>,
}

#[doc = "Velocity capture register"]
pub mod cap {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    impl super::CAP {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }

    #[doc = r" Value of the field"]
    pub struct VELCAPR {
        bits: u32,
    }

    impl VELCAPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Last velocity capture."]
        #[inline(always)]
        pub fn velcap(&self) -> VELCAPR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            VELCAPR { bits }
        }
    }
}

#[doc = "Velocity compare register"]
pub struct VELCOMP {
    register: VolatileCell<u32>,
}

#[doc = "Velocity compare register"]
pub mod velcomp {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::VELCOMP {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }

    #[doc = r" Value of the field"]
    pub struct VELPCR {
        bits: u32,
    }

    impl VELPCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    #[doc = r" Proxy"]
    pub struct _VELPCW<'a> {
        w: &'a mut W,
    }

    impl<'a> _VELPCW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Compare velocity pulse count."]
        #[inline(always)]
        pub fn velpc(&self) -> VELPCR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            VELPCR { bits }
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:31 - Compare velocity pulse count."]
        #[inline(always)]
        pub fn velpc(&mut self) -> _VELPCW {
            _VELPCW { w: self }
        }
    }
}

#[doc = "Digital filter register"]
pub struct FILTER {
    register: VolatileCell<u32>,
}

#[doc = "Digital filter register"]
pub mod filter {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::FILTER {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }

    #[doc = r" Value of the field"]
    pub struct FILTAR {
        bits: u32,
    }

    impl FILTAR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }

    #[doc = r" Proxy"]
    pub struct _FILTAW<'a> {
        w: &'a mut W,
    }

    impl<'a> _FILTAW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Digital filter sampling delay."]
        #[inline(always)]
        pub fn filta(&self) -> FILTAR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            FILTAR { bits }
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:31 - Digital filter sampling delay."]
        #[inline(always)]
        pub fn filta(&mut self) -> _FILTAW {
            _FILTAW { w: self }
        }
    }
}

#[doc = "Interrupt status register"]
pub struct INTSTAT {
    register: VolatileCell<u32>,
}

#[doc = "Interrupt status register"]
pub mod intstat {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    impl super::INTSTAT {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }

    #[doc = r" Value of the field"]
    pub struct INX_INTR {
        bits: bool,
    }

    impl INX_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct TIM_INTR {
        bits: bool,
    }

    impl TIM_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct VELC_INTR {
        bits: bool,
    }

    impl VELC_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct DIR_INTR {
        bits: bool,
    }

    impl DIR_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct ERR_INTR {
        bits: bool,
    }

    impl ERR_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct ENCLK_INTR {
        bits: bool,
    }

    impl ENCLK_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS0_INTR {
        bits: bool,
    }

    impl POS0_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS1_INTR {
        bits: bool,
    }

    impl POS1_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS2_INTR {
        bits: bool,
    }

    impl POS2_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct REV0_INTR {
        bits: bool,
    }

    impl REV0_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS0REV_INTR {
        bits: bool,
    }

    impl POS0REV_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS1REV_INTR {
        bits: bool,
    }

    impl POS1REV_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS2REV_INTR {
        bits: bool,
    }

    impl POS2REV_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct REV1_INTR {
        bits: bool,
    }

    impl REV1_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct REV2_INTR {
        bits: bool,
    }

    impl REV2_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct MAXPOS_INTR {
        bits: bool,
    }

    impl MAXPOS_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - Indicates that an index pulse was detected."]
        #[inline(always)]
        pub fn inx_int(&self) -> INX_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INX_INTR { bits }
        }
        #[doc = "Bit 1 - Indicates that a velocity timer overflow occurred"]
        #[inline(always)]
        pub fn tim_int(&self) -> TIM_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM_INTR { bits }
        }
        #[doc = "Bit 2 - Indicates that captured velocity is less than compare velocity."]
        #[inline(always)]
        pub fn velc_int(&self) -> VELC_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            VELC_INTR { bits }
        }
        #[doc = "Bit 3 - Indicates that a change of direction was detected."]
        #[inline(always)]
        pub fn dir_int(&self) -> DIR_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DIR_INTR { bits }
        }
        #[doc = "Bit 4 - Indicates that an encoder phase error was detected."]
        #[inline(always)]
        pub fn err_int(&self) -> ERR_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ERR_INTR { bits }
        }
        #[doc = "Bit 5 - Indicates that and encoder clock pulse was detected."]
        #[inline(always)]
        pub fn enclk_int(&self) -> ENCLK_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ENCLK_INTR { bits }
        }
        #[doc = "Bit 6 - Indicates that the position 0 compare value is equal to the current position."]
        #[inline(always)]
        pub fn pos0_int(&self) -> POS0_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS0_INTR { bits }
        }
        #[doc = "Bit 7 - Indicates that the position 1compare value is equal to the current position."]
        #[inline(always)]
        pub fn pos1_int(&self) -> POS1_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS1_INTR { bits }
        }
        #[doc = "Bit 8 - Indicates that the position 2 compare value is equal to the current position."]
        #[inline(always)]
        pub fn pos2_int(&self) -> POS2_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS2_INTR { bits }
        }
        #[doc = "Bit 9 - Indicates that the index compare 0 value is equal to the current index count."]
        #[inline(always)]
        pub fn rev0_int(&self) -> REV0_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            REV0_INTR { bits }
        }
        #[doc = "Bit 10 - Combined position 0 and revolution count interrupt. Set when both the POS0_Int bit is set and the REV0_Int is set."]
        #[inline(always)]
        pub fn pos0rev_int(&self) -> POS0REV_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS0REV_INTR { bits }
        }
        #[doc = "Bit 11 - Combined position 1 and revolution count interrupt. Set when both the POS1_Int bit is set and the REV1_Int is set."]
        #[inline(always)]
        pub fn pos1rev_int(&self) -> POS1REV_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS1REV_INTR { bits }
        }
        #[doc = "Bit 12 - Combined position 2 and revolution count interrupt. Set when both the POS2_Int bit is set and the REV2_Int is set."]
        #[inline(always)]
        pub fn pos2rev_int(&self) -> POS2REV_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS2REV_INTR { bits }
        }
        #[doc = "Bit 13 - Indicates that the index compare 1value is equal to the current index count."]
        #[inline(always)]
        pub fn rev1_int(&self) -> REV1_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            REV1_INTR { bits }
        }
        #[doc = "Bit 14 - Indicates that the index compare 2 value is equal to the current index count."]
        #[inline(always)]
        pub fn rev2_int(&self) -> REV2_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            REV2_INTR { bits }
        }
        #[doc = "Bit 15 - Indicates that the current position count goes through the MAXPOS value to zero in the forward direction, or through zero to MAXPOS in the reverse direction."]
        #[inline(always)]
        pub fn maxpos_int(&self) -> MAXPOS_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MAXPOS_INTR { bits }
        }
    }
}

#[doc = "Interrupt status set register"]
pub struct SET {
    register: VolatileCell<u32>,
}

#[doc = "Interrupt status set register"]
pub mod set {
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::SET {
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
    }

    #[doc = r" Proxy"]
    pub struct _INX_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _INX_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _TIM_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _TIM_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _VELC_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _VELC_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _DIR_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _DIR_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _ERR_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _ERR_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _ENCLK_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _ENCLK_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS0_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS0_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS1_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS1_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS2_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS2_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _REV0_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _REV0_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS0REV_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS0REV_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS1REV_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS1REV_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS2REV_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS2REV_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _REV1_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _REV1_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _REV2_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _REV2_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _MAXPOS_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _MAXPOS_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Writing a 1 sets the INX_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn inx_int(&mut self) -> _INX_INTW {
            _INX_INTW { w: self }
        }
        #[doc = "Bit 1 - Writing a 1 sets the TIN_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn tim_int(&mut self) -> _TIM_INTW {
            _TIM_INTW { w: self }
        }
        #[doc = "Bit 2 - Writing a 1 sets the VELC_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn velc_int(&mut self) -> _VELC_INTW {
            _VELC_INTW { w: self }
        }
        #[doc = "Bit 3 - Writing a 1 sets the DIR_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn dir_int(&mut self) -> _DIR_INTW {
            _DIR_INTW { w: self }
        }
        #[doc = "Bit 4 - Writing a 1 sets the ERR_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn err_int(&mut self) -> _ERR_INTW {
            _ERR_INTW { w: self }
        }
        #[doc = "Bit 5 - Writing a 1 sets the ENCLK_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn enclk_int(&mut self) -> _ENCLK_INTW {
            _ENCLK_INTW { w: self }
        }
        #[doc = "Bit 6 - Writing a 1 sets the POS0_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos0_int(&mut self) -> _POS0_INTW {
            _POS0_INTW { w: self }
        }
        #[doc = "Bit 7 - Writing a 1 sets the POS1_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos1_int(&mut self) -> _POS1_INTW {
            _POS1_INTW { w: self }
        }
        #[doc = "Bit 8 - Writing a 1 sets the POS2_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos2_int(&mut self) -> _POS2_INTW {
            _POS2_INTW { w: self }
        }
        #[doc = "Bit 9 - Writing a 1 sets the REV0_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn rev0_int(&mut self) -> _REV0_INTW {
            _REV0_INTW { w: self }
        }
        #[doc = "Bit 10 - Writing a 1 sets the POS0REV_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos0rev_int(&mut self) -> _POS0REV_INTW {
            _POS0REV_INTW { w: self }
        }
        #[doc = "Bit 11 - Writing a 1 sets the POS1REV_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos1rev_int(&mut self) -> _POS1REV_INTW {
            _POS1REV_INTW { w: self }
        }
        #[doc = "Bit 12 - Writing a 1 sets the POS2REV_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos2rev_int(&mut self) -> _POS2REV_INTW {
            _POS2REV_INTW { w: self }
        }
        #[doc = "Bit 13 - Writing a 1 sets the REV1_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn rev1_int(&mut self) -> _REV1_INTW {
            _REV1_INTW { w: self }
        }
        #[doc = "Bit 14 - Writing a 1 sets the REV2_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn rev2_int(&mut self) -> _REV2_INTW {
            _REV2_INTW { w: self }
        }
        #[doc = "Bit 15 - Writing a 1 sets the MAXPOS_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn maxpos_int(&mut self) -> _MAXPOS_INTW {
            _MAXPOS_INTW { w: self }
        }
    }
}

#[doc = "Interrupt status clear register"]
pub struct CLR {
    register: VolatileCell<u32>,
}

#[doc = "Interrupt status clear register"]
pub mod clr {
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::CLR {
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
    }

    #[doc = r" Proxy"]
    pub struct _INX_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _INX_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _TIM_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _TIM_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _VELC_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _VELC_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _DIR_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _DIR_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _ERR_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _ERR_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _ENCLK_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _ENCLK_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS0_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS0_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS1_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS1_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS2_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS2_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _REV0_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _REV0_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS0REV_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS0REV_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS1REV_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS1REV_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS2REV_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS2REV_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _REV1_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _REV1_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _REV2_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _REV2_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _MAXPOS_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _MAXPOS_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Writing a 1 clears the INX_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn inx_int(&mut self) -> _INX_INTW {
            _INX_INTW { w: self }
        }
        #[doc = "Bit 1 - Writing a 1 clears the TIN_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn tim_int(&mut self) -> _TIM_INTW {
            _TIM_INTW { w: self }
        }
        #[doc = "Bit 2 - Writing a 1 clears the VELC_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn velc_int(&mut self) -> _VELC_INTW {
            _VELC_INTW { w: self }
        }
        #[doc = "Bit 3 - Writing a 1 clears the DIR_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn dir_int(&mut self) -> _DIR_INTW {
            _DIR_INTW { w: self }
        }
        #[doc = "Bit 4 - Writing a 1 clears the ERR_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn err_int(&mut self) -> _ERR_INTW {
            _ERR_INTW { w: self }
        }
        #[doc = "Bit 5 - Writing a 1 clears the ENCLK_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn enclk_int(&mut self) -> _ENCLK_INTW {
            _ENCLK_INTW { w: self }
        }
        #[doc = "Bit 6 - Writing a 1 clears the POS0_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos0_int(&mut self) -> _POS0_INTW {
            _POS0_INTW { w: self }
        }
        #[doc = "Bit 7 - Writing a 1 clears the POS1_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos1_int(&mut self) -> _POS1_INTW {
            _POS1_INTW { w: self }
        }
        #[doc = "Bit 8 - Writing a 1 clears the POS2_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos2_int(&mut self) -> _POS2_INTW {
            _POS2_INTW { w: self }
        }
        #[doc = "Bit 9 - Writing a 1 clears the REV0_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn rev0_int(&mut self) -> _REV0_INTW {
            _REV0_INTW { w: self }
        }
        #[doc = "Bit 10 - Writing a 1 clears the POS0REV_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos0rev_int(&mut self) -> _POS0REV_INTW {
            _POS0REV_INTW { w: self }
        }
        #[doc = "Bit 11 - Writing a 1 clears the POS1REV_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos1rev_int(&mut self) -> _POS1REV_INTW {
            _POS1REV_INTW { w: self }
        }
        #[doc = "Bit 12 - Writing a 1 clears the POS2REV_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn pos2rev_int(&mut self) -> _POS2REV_INTW {
            _POS2REV_INTW { w: self }
        }
        #[doc = "Bit 13 - Writing a 1 clears the REV1_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn rev1_int(&mut self) -> _REV1_INTW {
            _REV1_INTW { w: self }
        }
        #[doc = "Bit 14 - Writing a 1 clears the REV2_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn rev2_int(&mut self) -> _REV2_INTW {
            _REV2_INTW { w: self }
        }
        #[doc = "Bit 15 - Writing a 1 clears the MAXPOS_Int bit in QEIINTSTAT."]
        #[inline(always)]
        pub fn maxpos_int(&mut self) -> _MAXPOS_INTW {
            _MAXPOS_INTW { w: self }
        }
    }
}

#[doc = "Interrupt enable register"]
pub struct IE {
    register: VolatileCell<u32>,
}

#[doc = "Interrupt enable register"]
pub mod ie {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }

    impl super::IE {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }

    #[doc = r" Value of the field"]
    pub struct INX_INTR {
        bits: bool,
    }

    impl INX_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct TIM_INTR {
        bits: bool,
    }

    impl TIM_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct VELC_INTR {
        bits: bool,
    }

    impl VELC_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct DIR_INTR {
        bits: bool,
    }

    impl DIR_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct ERR_INTR {
        bits: bool,
    }

    impl ERR_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct ENCLK_INTR {
        bits: bool,
    }

    impl ENCLK_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS0_INTR {
        bits: bool,
    }

    impl POS0_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS1_INTR {
        bits: bool,
    }

    impl POS1_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS2_INTR {
        bits: bool,
    }

    impl POS2_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct REV0_INTR {
        bits: bool,
    }

    impl REV0_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS0REV_INTR {
        bits: bool,
    }

    impl POS0REV_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS1REV_INTR {
        bits: bool,
    }

    impl POS1REV_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct POS2REV_INTR {
        bits: bool,
    }

    impl POS2REV_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct REV1_INTR {
        bits: bool,
    }

    impl REV1_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct REV2_INTR {
        bits: bool,
    }

    impl REV2_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    #[doc = r" Value of the field"]
    pub struct MAXPOS_INTR {
        bits: bool,
    }

    impl MAXPOS_INTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }

    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - When 1, the INX_Int interrupt is enabled."]
        #[inline(always)]
        pub fn inx_int(&self) -> INX_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INX_INTR { bits }
        }
        #[doc = "Bit 1 - When 1, the TIN_Int interrupt is enabled."]
        #[inline(always)]
        pub fn tim_int(&self) -> TIM_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIM_INTR { bits }
        }
        #[doc = "Bit 2 - When 1, the VELC_Int interrupt is enabled."]
        #[inline(always)]
        pub fn velc_int(&self) -> VELC_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            VELC_INTR { bits }
        }
        #[doc = "Bit 3 - When 1, the DIR_Int interrupt is enabled."]
        #[inline(always)]
        pub fn dir_int(&self) -> DIR_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DIR_INTR { bits }
        }
        #[doc = "Bit 4 - When 1, the ERR_Int interrupt is enabled."]
        #[inline(always)]
        pub fn err_int(&self) -> ERR_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ERR_INTR { bits }
        }
        #[doc = "Bit 5 - When 1, the ENCLK_Int interrupt is enabled."]
        #[inline(always)]
        pub fn enclk_int(&self) -> ENCLK_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ENCLK_INTR { bits }
        }
        #[doc = "Bit 6 - When 1, the POS0_Int interrupt is enabled."]
        #[inline(always)]
        pub fn pos0_int(&self) -> POS0_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS0_INTR { bits }
        }
        #[doc = "Bit 7 - When 1, the POS1_Int interrupt is enabled."]
        #[inline(always)]
        pub fn pos1_int(&self) -> POS1_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS1_INTR { bits }
        }
        #[doc = "Bit 8 - When 1, the POS2_Int interrupt is enabled."]
        #[inline(always)]
        pub fn pos2_int(&self) -> POS2_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS2_INTR { bits }
        }
        #[doc = "Bit 9 - When 1, the REV0_Int interrupt is enabled."]
        #[inline(always)]
        pub fn rev0_int(&self) -> REV0_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            REV0_INTR { bits }
        }
        #[doc = "Bit 10 - When 1, the POS0REV_Int interrupt is enabled."]
        #[inline(always)]
        pub fn pos0rev_int(&self) -> POS0REV_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS0REV_INTR { bits }
        }
        #[doc = "Bit 11 - When 1, the POS1REV_Int interrupt is enabled."]
        #[inline(always)]
        pub fn pos1rev_int(&self) -> POS1REV_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS1REV_INTR { bits }
        }
        #[doc = "Bit 12 - When 1, the POS2REV_Int interrupt is enabled."]
        #[inline(always)]
        pub fn pos2rev_int(&self) -> POS2REV_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POS2REV_INTR { bits }
        }
        #[doc = "Bit 13 - When 1, the REV1_Int interrupt is enabled."]
        #[inline(always)]
        pub fn rev1_int(&self) -> REV1_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            REV1_INTR { bits }
        }
        #[doc = "Bit 14 - When 1, the REV2_Int interrupt is enabled."]
        #[inline(always)]
        pub fn rev2_int(&self) -> REV2_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            REV2_INTR { bits }
        }
        #[doc = "Bit 15 - When 1, the MAXPOS_Int interrupt is enabled."]
        #[inline(always)]
        pub fn maxpos_int(&self) -> MAXPOS_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MAXPOS_INTR { bits }
        }
    }
}

#[doc = "Interrupt enable set register"]
pub struct IES {
    register: VolatileCell<u32>,
}

#[doc = "Interrupt enable set register"]
pub mod ies {
    use super::VolatileCell;

    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }

    impl super::IES {
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
    }

    #[doc = r" Proxy"]
    pub struct _INX_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _INX_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _TIM_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _TIM_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _VELC_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _VELC_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _DIR_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _DIR_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _ERR_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _ERR_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _ENCLK_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _ENCLK_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS0_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS0_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS1_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS1_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS2_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS2_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _REV0_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _REV0_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS0REV_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS0REV_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS1REV_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS1REV_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _POS2REV_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _POS2REV_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _REV1_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _REV1_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _REV2_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _REV2_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    #[doc = r" Proxy"]
    pub struct _MAXPOS_INTW<'a> {
        w: &'a mut W,
    }

    impl<'a> _MAXPOS_INTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }

    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Writing a 1 enables the INX_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn inx_int(&mut self) -> _INX_INTW {
            _INX_INTW { w: self }
        }
        #[doc = "Bit 1 - Writing a 1 enables the TIN_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn tim_int(&mut self) -> _TIM_INTW {
            _TIM_INTW { w: self }
        }
        #[doc = "Bit 2 - Writing a 1 enables the VELC_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn velc_int(&mut self) -> _VELC_INTW {
            _VELC_INTW { w: self }
        }
        #[doc = "Bit 3 - Writing a 1 enables the DIR_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn dir_int(&mut self) -> _DIR_INTW {
            _DIR_INTW { w: self }
        }
        #[doc = "Bit 4 - Writing a 1 enables the ERR_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn err_int(&mut self) -> _ERR_INTW {
            _ERR_INTW { w: self }
        }
        #[doc = "Bit 5 - Writing a 1 enables the ENCLK_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn enclk_int(&mut self) -> _ENCLK_INTW {
            _ENCLK_INTW { w: self }
        }
        #[doc = "Bit 6 - Writing a 1 enables the POS0_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn pos0_int(&mut self) -> _POS0_INTW {
            _POS0_INTW { w: self }
        }
        #[doc = "Bit 7 - Writing a 1 enables the POS1_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn pos1_int(&mut self) -> _POS1_INTW {
            _POS1_INTW { w: self }
        }
        #[doc = "Bit 8 - Writing a 1 enables the POS2_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn pos2_int(&mut self) -> _POS2_INTW {
            _POS2_INTW { w: self }
        }
        #[doc = "Bit 9 - Writing a 1 enables the REV0_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn rev0_int(&mut self) -> _REV0_INTW {
            _REV0_INTW { w: self }
        }
        #[doc = "Bit 10 - Writing a 1 enables the POS0REV_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn pos0rev_int(&mut self) -> _POS0REV_INTW {
            _POS0REV_INTW { w: self }
        }
        #[doc = "Bit 11 - Writing a 1 enables the POS1REV_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn pos1rev_int(&mut self) -> _POS1REV_INTW {
            _POS1REV_INTW { w: self }
        }
        #[doc = "Bit 12 - Writing a 1 enables the POS2REV_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn pos2rev_int(&mut self) -> _POS2REV_INTW {
            _POS2REV_INTW { w: self }
        }
        #[doc = "Bit 13 - Writing a 1 enables the REV1_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn rev1_int(&mut self) -> _REV1_INTW {
            _REV1_INTW { w: self }
        }
        #[doc = "Bit 14 - Writing a 1 enables the REV2_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn rev2_int(&mut self) -> _REV2_INTW {
            _REV2_INTW { w: self }
        }
        #[doc = "Bit 15 - Writing a 1 enables the MAXPOS_Int interrupt in the QEIIE register."]
        #[inline(always)]
        pub fn maxpos_int(&mut self) -> _MAXPOS_INTW {
            _MAXPOS_INTW { w: self }
        }
    }

    #[doc = "Interrupt enable clear register"]
    pub struct IEC {
        register: VolatileCell<u32>,
    }

    #[doc = "Interrupt enable clear register"]
    pub mod iec {
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }

        impl super::IEC {
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
                where F: FnOnce(&mut W) -> &mut W
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
        }

        #[doc = r" Proxy"]
        pub struct _INX_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _INX_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _TIM_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _TIM_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _VELC_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _VELC_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _DIR_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _DIR_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _ERR_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _ERR_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _ENCLK_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _ENCLK_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _POS0_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _POS0_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _POS1_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _POS1_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _POS2_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _POS2_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _REV0_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _REV0_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _POS0REV_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _POS0REV_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _POS1REV_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _POS1REV_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _POS2REV_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _POS2REV_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _REV1_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _REV1_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _REV2_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _REV2_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        #[doc = r" Proxy"]
        pub struct _MAXPOS_INTW<'a> {
            w: &'a mut W,
        }

        impl<'a> _MAXPOS_INTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }

        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Writing a 1 disables the INX_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn inx_int(&mut self) -> _INX_INTW {
                _INX_INTW { w: self }
            }
            #[doc = "Bit 1 - Writing a 1 disables the TIN_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn tim_int(&mut self) -> _TIM_INTW {
                _TIM_INTW { w: self }
            }
            #[doc = "Bit 2 - Writing a 1 disables the VELC_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn velc_int(&mut self) -> _VELC_INTW {
                _VELC_INTW { w: self }
            }
            #[doc = "Bit 3 - Writing a 1 disables the DIR_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn dir_int(&mut self) -> _DIR_INTW {
                _DIR_INTW { w: self }
            }
            #[doc = "Bit 4 - Writing a 1 disables the ERR_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn err_int(&mut self) -> _ERR_INTW {
                _ERR_INTW { w: self }
            }
            #[doc = "Bit 5 - Writing a 1 disables the ENCLK_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn enclk_int(&mut self) -> _ENCLK_INTW {
                _ENCLK_INTW { w: self }
            }
            #[doc = "Bit 6 - Writing a 1 disables the POS0_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn pos0_int(&mut self) -> _POS0_INTW {
                _POS0_INTW { w: self }
            }
            #[doc = "Bit 7 - Writing a 1 disables the POS1_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn pos1_int(&mut self) -> _POS1_INTW {
                _POS1_INTW { w: self }
            }
            #[doc = "Bit 8 - Writing a 1 disables the POS2_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn pos2_int(&mut self) -> _POS2_INTW {
                _POS2_INTW { w: self }
            }
            #[doc = "Bit 9 - Writing a 1 disables the REV0_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn rev0_int(&mut self) -> _REV0_INTW {
                _REV0_INTW { w: self }
            }
            #[doc = "Bit 10 - Writing a 1 disables the POS0REV_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn pos0rev_int(&mut self) -> _POS0REV_INTW {
                _POS0REV_INTW { w: self }
            }
            #[doc = "Bit 11 - Writing a 1 disables the POS1REV_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn pos1rev_int(&mut self) -> _POS1REV_INTW {
                _POS1REV_INTW { w: self }
            }
            #[doc = "Bit 12 - Writing a 1 disables the POS2REV_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn pos2rev_int(&mut self) -> _POS2REV_INTW {
                _POS2REV_INTW { w: self }
            }
            #[doc = "Bit 13 - Writing a 1 disables the REV1_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn rev1_int(&mut self) -> _REV1_INTW {
                _REV1_INTW { w: self }
            }
            #[doc = "Bit 14 - Writing a 1 disables the REV2_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn rev2_int(&mut self) -> _REV2_INTW {
                _REV2_INTW { w: self }
            }
            #[doc = "Bit 15 - Writing a 1 disables the MAXPOS_Int interrupt in the QEIIE register."]
            #[inline(always)]
            pub fn maxpos_int(&mut self) -> _MAXPOS_INTW {
                _MAXPOS_INTW { w: self }
            }
        }
    }
}

#[doc = "Quadrature Encoder Interface (QEI)"]
pub struct QEI {
    register_block: RegisterBlock,
}

impl Deref for QEI {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
