# ! [ doc = "Timer0/1/2/3" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "Timer0/1/2/3" ]
pub const TIMER0: Peripheral<TIMER0> = unsafe { Peripheral::new(1073758208) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending." ]
    pub ir: IR,
    # [ doc = "0x04 - Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR." ]
    pub tcr: TCR,
    # [ doc = "0x08 - Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR." ]
    pub tc: TC,
    # [ doc = "0x0c - Prescale Register. When the Prescale Counter (PC) is equal to this value, the next clock increments the TC and clears the PC." ]
    pub pr: PR,
    # [ doc = "0x10 - Prescale Counter. The 32 bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface." ]
    pub pc: PC,
    # [ doc = "0x14 - Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs." ]
    pub mcr: MCR,
    # [ doc = "0x18 - Match Register 0. MR0 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR0 matches the TC." ]
    pub mr0: MR0,
    # [ doc = "0x1c - Match Register 1. MR1 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR1 matches the TC." ]
    pub mr1: MR1,
    # [ doc = "0x20 - Match Register 2. MR2 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR2 matches the TC." ]
    pub mr2: MR2,
    # [ doc = "0x24 - Match Register 3. MR3 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR3 matches the TC." ]
    pub mr3: MR3,
    # [ doc = "0x28 - Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place." ]
    pub ccr: CCR,
    # [ doc = "0x2c - Capture Register 0. CR0 is loaded with the value of TC when there is an event on the CAPn.0 input." ]
    pub cr0: CR,
    # [ doc = "0x30 - Capture Register 0. CR0 is loaded with the value of TC when there is an event on the CAPn.0 input." ]
    pub cr1: CR,
    _reserved0: [u8; 8usize],
    # [ doc = "0x3c - External Match Register. The EMR controls the external match pins." ]
    pub emr: EMR,
    _reserved1: [u8; 48usize],
    # [ doc = "0x70 - Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting." ]
    pub ctcr: CTCR,
}
# [ doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending." ]
pub struct IR {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending." ]
pub mod ir {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::IR {
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
    pub struct MR0INTR {
        bits: bool,
    }
    impl MR0INTR {
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
    pub struct MR1INTR {
        bits: bool,
    }
    impl MR1INTR {
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
    pub struct MR2INTR {
        bits: bool,
    }
    impl MR2INTR {
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
    pub struct MR3INTR {
        bits: bool,
    }
    impl MR3INTR {
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
    pub struct CR0INTR {
        bits: bool,
    }
    impl CR0INTR {
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
    pub struct CR1INTR {
        bits: bool,
    }
    impl CR1INTR {
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
    pub struct _MR0INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR0INTW<'a> {
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
    pub struct _MR1INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR1INTW<'a> {
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
    pub struct _MR2INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR2INTW<'a> {
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
    pub struct _MR3INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR3INTW<'a> {
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
    pub struct _CR0INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CR0INTW<'a> {
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
    pub struct _CR1INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CR1INTW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Interrupt flag for match channel 0." ]
        # [ inline ( always ) ]
        pub fn mr0int(&self) -> MR0INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MR0INTR { bits }
        }
        # [ doc = "Bit 1 - Interrupt flag for match channel 1." ]
        # [ inline ( always ) ]
        pub fn mr1int(&self) -> MR1INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MR1INTR { bits }
        }
        # [ doc = "Bit 2 - Interrupt flag for match channel 2." ]
        # [ inline ( always ) ]
        pub fn mr2int(&self) -> MR2INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MR2INTR { bits }
        }
        # [ doc = "Bit 3 - Interrupt flag for match channel 3." ]
        # [ inline ( always ) ]
        pub fn mr3int(&self) -> MR3INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MR3INTR { bits }
        }
        # [ doc = "Bit 4 - Interrupt flag for capture channel 0 event." ]
        # [ inline ( always ) ]
        pub fn cr0int(&self) -> CR0INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CR0INTR { bits }
        }
        # [ doc = "Bit 5 - Interrupt flag for capture channel 1 event." ]
        # [ inline ( always ) ]
        pub fn cr1int(&self) -> CR1INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CR1INTR { bits }
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
        # [ doc = "Bit 0 - Interrupt flag for match channel 0." ]
        # [ inline ( always ) ]
        pub fn mr0int(&mut self) -> _MR0INTW {
            _MR0INTW { w: self }
        }
        # [ doc = "Bit 1 - Interrupt flag for match channel 1." ]
        # [ inline ( always ) ]
        pub fn mr1int(&mut self) -> _MR1INTW {
            _MR1INTW { w: self }
        }
        # [ doc = "Bit 2 - Interrupt flag for match channel 2." ]
        # [ inline ( always ) ]
        pub fn mr2int(&mut self) -> _MR2INTW {
            _MR2INTW { w: self }
        }
        # [ doc = "Bit 3 - Interrupt flag for match channel 3." ]
        # [ inline ( always ) ]
        pub fn mr3int(&mut self) -> _MR3INTW {
            _MR3INTW { w: self }
        }
        # [ doc = "Bit 4 - Interrupt flag for capture channel 0 event." ]
        # [ inline ( always ) ]
        pub fn cr0int(&mut self) -> _CR0INTW {
            _CR0INTW { w: self }
        }
        # [ doc = "Bit 5 - Interrupt flag for capture channel 1 event." ]
        # [ inline ( always ) ]
        pub fn cr1int(&mut self) -> _CR1INTW {
            _CR1INTW { w: self }
        }
    }
}
# [ doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR." ]
pub struct TCR {
    register: VolatileCell<u32>,
}
# [ doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR." ]
pub mod tcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TCR {
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
    pub struct CENR {
        bits: bool,
    }
    impl CENR {
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
    pub struct CRSTR {
        bits: bool,
    }
    impl CRSTR {
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
    pub struct _CENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CENW<'a> {
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
    pub struct _CRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRSTW<'a> {
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
        # [ doc = "Bit 0 - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled." ]
        # [ inline ( always ) ]
        pub fn cen(&self) -> CENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CENR { bits }
        }
        # [ doc = "Bit 1 - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR[1] is returned to zero." ]
        # [ inline ( always ) ]
        pub fn crst(&self) -> CRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CRSTR { bits }
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
        # [ doc = "Bit 0 - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled." ]
        # [ inline ( always ) ]
        pub fn cen(&mut self) -> _CENW {
            _CENW { w: self }
        }
        # [ doc = "Bit 1 - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR[1] is returned to zero." ]
        # [ inline ( always ) ]
        pub fn crst(&mut self) -> _CRSTW {
            _CRSTW { w: self }
        }
    }
}
# [ doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR." ]
pub struct TC {
    register: VolatileCell<u32>,
}
# [ doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR." ]
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
    pub struct TCR {
        bits: u32,
    }
    impl TCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCW<'a> {
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
        # [ doc = "Bits 0:31 - Timer counter value." ]
        # [ inline ( always ) ]
        pub fn tc(&self) -> TCR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            TCR { bits }
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
        # [ doc = "Bits 0:31 - Timer counter value." ]
        # [ inline ( always ) ]
        pub fn tc(&mut self) -> _TCW {
            _TCW { w: self }
        }
    }
}
# [ doc = "Prescale Register. When the Prescale Counter (PC) is equal to this value, the next clock increments the TC and clears the PC." ]
pub struct PR {
    register: VolatileCell<u32>,
}
# [ doc = "Prescale Register. When the Prescale Counter (PC) is equal to this value, the next clock increments the TC and clears the PC." ]
pub mod pr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PR {
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
    pub struct PMR {
        bits: u32,
    }
    impl PMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PMW<'a> {
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
        # [ doc = "Bits 0:31 - Prescale counter maximum value." ]
        # [ inline ( always ) ]
        pub fn pm(&self) -> PMR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            PMR { bits }
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
        # [ doc = "Bits 0:31 - Prescale counter maximum value." ]
        # [ inline ( always ) ]
        pub fn pm(&mut self) -> _PMW {
            _PMW { w: self }
        }
    }
}
# [ doc = "Prescale Counter. The 32 bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface." ]
pub struct PC {
    register: VolatileCell<u32>,
}
# [ doc = "Prescale Counter. The 32 bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface." ]
pub mod pc {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PC {
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
    pub struct PCR {
        bits: u32,
    }
    impl PCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCW<'a> {
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
        # [ doc = "Bits 0:31 - Prescale counter value." ]
        # [ inline ( always ) ]
        pub fn pc(&self) -> PCR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            PCR { bits }
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
        # [ doc = "Bits 0:31 - Prescale counter value." ]
        # [ inline ( always ) ]
        pub fn pc(&mut self) -> _PCW {
            _PCW { w: self }
        }
    }
}
# [ doc = "Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs." ]
pub struct MCR {
    register: VolatileCell<u32>,
}
# [ doc = "Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs." ]
pub mod mcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MCR {
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
    # [ doc = "Possible values of the field `MR0I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR0IR {
        # [ doc = "Interrupt is generated when MR0 matches the value in the TC." ]
        INTERRUPT_IS_GENERAT,
        # [ doc = "Interrupt is disabled" ]
        INTERRUPT_IS_DISABLE,
    }
    impl MR0IR {
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
                MR0IR::INTERRUPT_IS_GENERAT => true,
                MR0IR::INTERRUPT_IS_DISABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR0IR {
            match value {
                true => MR0IR::INTERRUPT_IS_GENERAT,
                false => MR0IR::INTERRUPT_IS_DISABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_IS_GENERAT`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_is_generat(&self) -> bool {
            *self == MR0IR::INTERRUPT_IS_GENERAT
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_IS_DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_is_disable(&self) -> bool {
            *self == MR0IR::INTERRUPT_IS_DISABLE
        }
    }
    # [ doc = "Possible values of the field `MR0R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR0RR {
        # [ doc = "TC will be reset if MR0 matches it." ]
        TC_WILL_BE_RESET_IF_,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR0RR {
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
                MR0RR::TC_WILL_BE_RESET_IF_ => true,
                MR0RR::FEATURE_DISABLED_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR0RR {
            match value {
                true => MR0RR::TC_WILL_BE_RESET_IF_,
                false => MR0RR::FEATURE_DISABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `TC_WILL_BE_RESET_IF_`" ]
        # [ inline ( always ) ]
        pub fn is_tc_will_be_reset_if_(&self) -> bool {
            *self == MR0RR::TC_WILL_BE_RESET_IF_
        }
        # [ doc = "Checks if the value of the field is `FEATURE_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_feature_disabled_(&self) -> bool {
            *self == MR0RR::FEATURE_DISABLED_
        }
    }
    # [ doc = "Possible values of the field `MR0S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR0SR {
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR0 matches the TC." ]
        TC_AND_PC_WILL_BE_ST,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR0SR {
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
                MR0SR::TC_AND_PC_WILL_BE_ST => true,
                MR0SR::FEATURE_DISABLED_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR0SR {
            match value {
                true => MR0SR::TC_AND_PC_WILL_BE_ST,
                false => MR0SR::FEATURE_DISABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `TC_AND_PC_WILL_BE_ST`" ]
        # [ inline ( always ) ]
        pub fn is_tc_and_pc_will_be_st(&self) -> bool {
            *self == MR0SR::TC_AND_PC_WILL_BE_ST
        }
        # [ doc = "Checks if the value of the field is `FEATURE_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_feature_disabled_(&self) -> bool {
            *self == MR0SR::FEATURE_DISABLED_
        }
    }
    # [ doc = "Possible values of the field `MR1I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR1IR {
        # [ doc = "Interrupt is generated when MR1 matches the value in the TC." ]
        INTERRUPT_IS_GENERAT,
        # [ doc = "Interrupt is disabled." ]
        INTERRUPT_IS_DISABLE,
    }
    impl MR1IR {
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
                MR1IR::INTERRUPT_IS_GENERAT => true,
                MR1IR::INTERRUPT_IS_DISABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR1IR {
            match value {
                true => MR1IR::INTERRUPT_IS_GENERAT,
                false => MR1IR::INTERRUPT_IS_DISABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_IS_GENERAT`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_is_generat(&self) -> bool {
            *self == MR1IR::INTERRUPT_IS_GENERAT
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_IS_DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_is_disable(&self) -> bool {
            *self == MR1IR::INTERRUPT_IS_DISABLE
        }
    }
    # [ doc = "Possible values of the field `MR1R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR1RR {
        # [ doc = "TC will be reset if MR1 matches it." ]
        TC_WILL_BE_RESET_IF_,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR1RR {
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
                MR1RR::TC_WILL_BE_RESET_IF_ => true,
                MR1RR::FEATURE_DISABLED_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR1RR {
            match value {
                true => MR1RR::TC_WILL_BE_RESET_IF_,
                false => MR1RR::FEATURE_DISABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `TC_WILL_BE_RESET_IF_`" ]
        # [ inline ( always ) ]
        pub fn is_tc_will_be_reset_if_(&self) -> bool {
            *self == MR1RR::TC_WILL_BE_RESET_IF_
        }
        # [ doc = "Checks if the value of the field is `FEATURE_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_feature_disabled_(&self) -> bool {
            *self == MR1RR::FEATURE_DISABLED_
        }
    }
    # [ doc = "Possible values of the field `MR1S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR1SR {
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR1 matches the TC." ]
        TC_AND_PC_WILL_BE_ST,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR1SR {
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
                MR1SR::TC_AND_PC_WILL_BE_ST => true,
                MR1SR::FEATURE_DISABLED_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR1SR {
            match value {
                true => MR1SR::TC_AND_PC_WILL_BE_ST,
                false => MR1SR::FEATURE_DISABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `TC_AND_PC_WILL_BE_ST`" ]
        # [ inline ( always ) ]
        pub fn is_tc_and_pc_will_be_st(&self) -> bool {
            *self == MR1SR::TC_AND_PC_WILL_BE_ST
        }
        # [ doc = "Checks if the value of the field is `FEATURE_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_feature_disabled_(&self) -> bool {
            *self == MR1SR::FEATURE_DISABLED_
        }
    }
    # [ doc = "Possible values of the field `MR2I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR2IR {
        # [ doc = "Interrupt is generated when MR2 matches the value in the TC." ]
        INTERRUPT_IS_GENERAT,
        # [ doc = "Interrupt is disabled" ]
        INTERRUPT_IS_DISABLE,
    }
    impl MR2IR {
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
                MR2IR::INTERRUPT_IS_GENERAT => true,
                MR2IR::INTERRUPT_IS_DISABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR2IR {
            match value {
                true => MR2IR::INTERRUPT_IS_GENERAT,
                false => MR2IR::INTERRUPT_IS_DISABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_IS_GENERAT`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_is_generat(&self) -> bool {
            *self == MR2IR::INTERRUPT_IS_GENERAT
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_IS_DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_is_disable(&self) -> bool {
            *self == MR2IR::INTERRUPT_IS_DISABLE
        }
    }
    # [ doc = "Possible values of the field `MR2R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR2RR {
        # [ doc = "TC will be reset if MR2 matches it." ]
        TC_WILL_BE_RESET_IF_,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR2RR {
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
                MR2RR::TC_WILL_BE_RESET_IF_ => true,
                MR2RR::FEATURE_DISABLED_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR2RR {
            match value {
                true => MR2RR::TC_WILL_BE_RESET_IF_,
                false => MR2RR::FEATURE_DISABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `TC_WILL_BE_RESET_IF_`" ]
        # [ inline ( always ) ]
        pub fn is_tc_will_be_reset_if_(&self) -> bool {
            *self == MR2RR::TC_WILL_BE_RESET_IF_
        }
        # [ doc = "Checks if the value of the field is `FEATURE_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_feature_disabled_(&self) -> bool {
            *self == MR2RR::FEATURE_DISABLED_
        }
    }
    # [ doc = "Possible values of the field `MR2S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR2SR {
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR2 matches the TC" ]
        TC_AND_PC_WILL_BE_ST,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR2SR {
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
                MR2SR::TC_AND_PC_WILL_BE_ST => true,
                MR2SR::FEATURE_DISABLED_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR2SR {
            match value {
                true => MR2SR::TC_AND_PC_WILL_BE_ST,
                false => MR2SR::FEATURE_DISABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `TC_AND_PC_WILL_BE_ST`" ]
        # [ inline ( always ) ]
        pub fn is_tc_and_pc_will_be_st(&self) -> bool {
            *self == MR2SR::TC_AND_PC_WILL_BE_ST
        }
        # [ doc = "Checks if the value of the field is `FEATURE_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_feature_disabled_(&self) -> bool {
            *self == MR2SR::FEATURE_DISABLED_
        }
    }
    # [ doc = "Possible values of the field `MR3I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR3IR {
        # [ doc = "Interrupt is generated when MR3 matches the value in the TC." ]
        INTERRUPT_IS_GENERAT,
        # [ doc = "This interrupt is disabled" ]
        THIS_INTERRUPT_IS_DI,
    }
    impl MR3IR {
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
                MR3IR::INTERRUPT_IS_GENERAT => true,
                MR3IR::THIS_INTERRUPT_IS_DI => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR3IR {
            match value {
                true => MR3IR::INTERRUPT_IS_GENERAT,
                false => MR3IR::THIS_INTERRUPT_IS_DI,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_IS_GENERAT`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_is_generat(&self) -> bool {
            *self == MR3IR::INTERRUPT_IS_GENERAT
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_is_di(&self) -> bool {
            *self == MR3IR::THIS_INTERRUPT_IS_DI
        }
    }
    # [ doc = "Possible values of the field `MR3R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR3RR {
        # [ doc = "TC will be reset if MR3 matches it." ]
        TC_WILL_BE_RESET_IF_,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR3RR {
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
                MR3RR::TC_WILL_BE_RESET_IF_ => true,
                MR3RR::FEATURE_DISABLED_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR3RR {
            match value {
                true => MR3RR::TC_WILL_BE_RESET_IF_,
                false => MR3RR::FEATURE_DISABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `TC_WILL_BE_RESET_IF_`" ]
        # [ inline ( always ) ]
        pub fn is_tc_will_be_reset_if_(&self) -> bool {
            *self == MR3RR::TC_WILL_BE_RESET_IF_
        }
        # [ doc = "Checks if the value of the field is `FEATURE_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_feature_disabled_(&self) -> bool {
            *self == MR3RR::FEATURE_DISABLED_
        }
    }
    # [ doc = "Possible values of the field `MR3S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MR3SR {
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR3 matches the TC." ]
        TC_AND_PC_WILL_BE_ST,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR3SR {
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
                MR3SR::TC_AND_PC_WILL_BE_ST => true,
                MR3SR::FEATURE_DISABLED_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MR3SR {
            match value {
                true => MR3SR::TC_AND_PC_WILL_BE_ST,
                false => MR3SR::FEATURE_DISABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `TC_AND_PC_WILL_BE_ST`" ]
        # [ inline ( always ) ]
        pub fn is_tc_and_pc_will_be_st(&self) -> bool {
            *self == MR3SR::TC_AND_PC_WILL_BE_ST
        }
        # [ doc = "Checks if the value of the field is `FEATURE_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_feature_disabled_(&self) -> bool {
            *self == MR3SR::FEATURE_DISABLED_
        }
    }
    # [ doc = "Values that can be written to the field `MR0I`" ]
    pub enum MR0IW {
        # [ doc = "Interrupt is generated when MR0 matches the value in the TC." ]
        INTERRUPT_IS_GENERAT,
        # [ doc = "Interrupt is disabled" ]
        INTERRUPT_IS_DISABLE,
    }
    impl MR0IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR0IW::INTERRUPT_IS_GENERAT => true,
                MR0IW::INTERRUPT_IS_DISABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR0IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR0IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR0IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Interrupt is generated when MR0 matches the value in the TC." ]
        # [ inline ( always ) ]
        pub fn interrupt_is_generat(self) -> &'a mut W {
            self.variant(MR0IW::INTERRUPT_IS_GENERAT)
        }
        # [ doc = "Interrupt is disabled" ]
        # [ inline ( always ) ]
        pub fn interrupt_is_disable(self) -> &'a mut W {
            self.variant(MR0IW::INTERRUPT_IS_DISABLE)
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
    # [ doc = "Values that can be written to the field `MR0R`" ]
    pub enum MR0RW {
        # [ doc = "TC will be reset if MR0 matches it." ]
        TC_WILL_BE_RESET_IF_,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR0RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR0RW::TC_WILL_BE_RESET_IF_ => true,
                MR0RW::FEATURE_DISABLED_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR0RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR0RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR0RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "TC will be reset if MR0 matches it." ]
        # [ inline ( always ) ]
        pub fn tc_will_be_reset_if_(self) -> &'a mut W {
            self.variant(MR0RW::TC_WILL_BE_RESET_IF_)
        }
        # [ doc = "Feature disabled." ]
        # [ inline ( always ) ]
        pub fn feature_disabled_(self) -> &'a mut W {
            self.variant(MR0RW::FEATURE_DISABLED_)
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
    # [ doc = "Values that can be written to the field `MR0S`" ]
    pub enum MR0SW {
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR0 matches the TC." ]
        TC_AND_PC_WILL_BE_ST,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR0SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR0SW::TC_AND_PC_WILL_BE_ST => true,
                MR0SW::FEATURE_DISABLED_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR0SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR0SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR0SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR0 matches the TC." ]
        # [ inline ( always ) ]
        pub fn tc_and_pc_will_be_st(self) -> &'a mut W {
            self.variant(MR0SW::TC_AND_PC_WILL_BE_ST)
        }
        # [ doc = "Feature disabled." ]
        # [ inline ( always ) ]
        pub fn feature_disabled_(self) -> &'a mut W {
            self.variant(MR0SW::FEATURE_DISABLED_)
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
    # [ doc = "Values that can be written to the field `MR1I`" ]
    pub enum MR1IW {
        # [ doc = "Interrupt is generated when MR1 matches the value in the TC." ]
        INTERRUPT_IS_GENERAT,
        # [ doc = "Interrupt is disabled." ]
        INTERRUPT_IS_DISABLE,
    }
    impl MR1IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR1IW::INTERRUPT_IS_GENERAT => true,
                MR1IW::INTERRUPT_IS_DISABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR1IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR1IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR1IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Interrupt is generated when MR1 matches the value in the TC." ]
        # [ inline ( always ) ]
        pub fn interrupt_is_generat(self) -> &'a mut W {
            self.variant(MR1IW::INTERRUPT_IS_GENERAT)
        }
        # [ doc = "Interrupt is disabled." ]
        # [ inline ( always ) ]
        pub fn interrupt_is_disable(self) -> &'a mut W {
            self.variant(MR1IW::INTERRUPT_IS_DISABLE)
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
    # [ doc = "Values that can be written to the field `MR1R`" ]
    pub enum MR1RW {
        # [ doc = "TC will be reset if MR1 matches it." ]
        TC_WILL_BE_RESET_IF_,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR1RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR1RW::TC_WILL_BE_RESET_IF_ => true,
                MR1RW::FEATURE_DISABLED_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR1RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR1RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR1RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "TC will be reset if MR1 matches it." ]
        # [ inline ( always ) ]
        pub fn tc_will_be_reset_if_(self) -> &'a mut W {
            self.variant(MR1RW::TC_WILL_BE_RESET_IF_)
        }
        # [ doc = "Feature disabled." ]
        # [ inline ( always ) ]
        pub fn feature_disabled_(self) -> &'a mut W {
            self.variant(MR1RW::FEATURE_DISABLED_)
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
    # [ doc = "Values that can be written to the field `MR1S`" ]
    pub enum MR1SW {
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR1 matches the TC." ]
        TC_AND_PC_WILL_BE_ST,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR1SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR1SW::TC_AND_PC_WILL_BE_ST => true,
                MR1SW::FEATURE_DISABLED_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR1SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR1SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR1SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR1 matches the TC." ]
        # [ inline ( always ) ]
        pub fn tc_and_pc_will_be_st(self) -> &'a mut W {
            self.variant(MR1SW::TC_AND_PC_WILL_BE_ST)
        }
        # [ doc = "Feature disabled." ]
        # [ inline ( always ) ]
        pub fn feature_disabled_(self) -> &'a mut W {
            self.variant(MR1SW::FEATURE_DISABLED_)
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
    # [ doc = "Values that can be written to the field `MR2I`" ]
    pub enum MR2IW {
        # [ doc = "Interrupt is generated when MR2 matches the value in the TC." ]
        INTERRUPT_IS_GENERAT,
        # [ doc = "Interrupt is disabled" ]
        INTERRUPT_IS_DISABLE,
    }
    impl MR2IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR2IW::INTERRUPT_IS_GENERAT => true,
                MR2IW::INTERRUPT_IS_DISABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR2IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR2IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR2IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Interrupt is generated when MR2 matches the value in the TC." ]
        # [ inline ( always ) ]
        pub fn interrupt_is_generat(self) -> &'a mut W {
            self.variant(MR2IW::INTERRUPT_IS_GENERAT)
        }
        # [ doc = "Interrupt is disabled" ]
        # [ inline ( always ) ]
        pub fn interrupt_is_disable(self) -> &'a mut W {
            self.variant(MR2IW::INTERRUPT_IS_DISABLE)
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
    # [ doc = "Values that can be written to the field `MR2R`" ]
    pub enum MR2RW {
        # [ doc = "TC will be reset if MR2 matches it." ]
        TC_WILL_BE_RESET_IF_,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR2RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR2RW::TC_WILL_BE_RESET_IF_ => true,
                MR2RW::FEATURE_DISABLED_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR2RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR2RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR2RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "TC will be reset if MR2 matches it." ]
        # [ inline ( always ) ]
        pub fn tc_will_be_reset_if_(self) -> &'a mut W {
            self.variant(MR2RW::TC_WILL_BE_RESET_IF_)
        }
        # [ doc = "Feature disabled." ]
        # [ inline ( always ) ]
        pub fn feature_disabled_(self) -> &'a mut W {
            self.variant(MR2RW::FEATURE_DISABLED_)
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
    # [ doc = "Values that can be written to the field `MR2S`" ]
    pub enum MR2SW {
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR2 matches the TC" ]
        TC_AND_PC_WILL_BE_ST,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR2SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR2SW::TC_AND_PC_WILL_BE_ST => true,
                MR2SW::FEATURE_DISABLED_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR2SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR2SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR2SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR2 matches the TC" ]
        # [ inline ( always ) ]
        pub fn tc_and_pc_will_be_st(self) -> &'a mut W {
            self.variant(MR2SW::TC_AND_PC_WILL_BE_ST)
        }
        # [ doc = "Feature disabled." ]
        # [ inline ( always ) ]
        pub fn feature_disabled_(self) -> &'a mut W {
            self.variant(MR2SW::FEATURE_DISABLED_)
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
    # [ doc = "Values that can be written to the field `MR3I`" ]
    pub enum MR3IW {
        # [ doc = "Interrupt is generated when MR3 matches the value in the TC." ]
        INTERRUPT_IS_GENERAT,
        # [ doc = "This interrupt is disabled" ]
        THIS_INTERRUPT_IS_DI,
    }
    impl MR3IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR3IW::INTERRUPT_IS_GENERAT => true,
                MR3IW::THIS_INTERRUPT_IS_DI => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR3IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR3IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR3IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Interrupt is generated when MR3 matches the value in the TC." ]
        # [ inline ( always ) ]
        pub fn interrupt_is_generat(self) -> &'a mut W {
            self.variant(MR3IW::INTERRUPT_IS_GENERAT)
        }
        # [ doc = "This interrupt is disabled" ]
        # [ inline ( always ) ]
        pub fn this_interrupt_is_di(self) -> &'a mut W {
            self.variant(MR3IW::THIS_INTERRUPT_IS_DI)
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
    # [ doc = "Values that can be written to the field `MR3R`" ]
    pub enum MR3RW {
        # [ doc = "TC will be reset if MR3 matches it." ]
        TC_WILL_BE_RESET_IF_,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR3RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR3RW::TC_WILL_BE_RESET_IF_ => true,
                MR3RW::FEATURE_DISABLED_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR3RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR3RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR3RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "TC will be reset if MR3 matches it." ]
        # [ inline ( always ) ]
        pub fn tc_will_be_reset_if_(self) -> &'a mut W {
            self.variant(MR3RW::TC_WILL_BE_RESET_IF_)
        }
        # [ doc = "Feature disabled." ]
        # [ inline ( always ) ]
        pub fn feature_disabled_(self) -> &'a mut W {
            self.variant(MR3RW::FEATURE_DISABLED_)
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
    # [ doc = "Values that can be written to the field `MR3S`" ]
    pub enum MR3SW {
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR3 matches the TC." ]
        TC_AND_PC_WILL_BE_ST,
        # [ doc = "Feature disabled." ]
        FEATURE_DISABLED_,
    }
    impl MR3SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MR3SW::TC_AND_PC_WILL_BE_ST => true,
                MR3SW::FEATURE_DISABLED_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MR3SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MR3SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MR3SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "TC and PC will be stopped and TCR[0] will be set to 0 if MR3 matches the TC." ]
        # [ inline ( always ) ]
        pub fn tc_and_pc_will_be_st(self) -> &'a mut W {
            self.variant(MR3SW::TC_AND_PC_WILL_BE_ST)
        }
        # [ doc = "Feature disabled." ]
        # [ inline ( always ) ]
        pub fn feature_disabled_(self) -> &'a mut W {
            self.variant(MR3SW::FEATURE_DISABLED_)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Interrupt on MR0" ]
        # [ inline ( always ) ]
        pub fn mr0i(&self) -> MR0IR {
            MR0IR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 0;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 1 - Reset on MR0" ]
        # [ inline ( always ) ]
        pub fn mr0r(&self) -> MR0RR {
            MR0RR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 1;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 2 - Stop on MR0" ]
        # [ inline ( always ) ]
        pub fn mr0s(&self) -> MR0SR {
            MR0SR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 2;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 3 - Interrupt on MR1" ]
        # [ inline ( always ) ]
        pub fn mr1i(&self) -> MR1IR {
            MR1IR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 3;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 4 - Reset on MR1" ]
        # [ inline ( always ) ]
        pub fn mr1r(&self) -> MR1RR {
            MR1RR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 4;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 5 - Stop on MR1" ]
        # [ inline ( always ) ]
        pub fn mr1s(&self) -> MR1SR {
            MR1SR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 5;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 6 - Interrupt on MR2" ]
        # [ inline ( always ) ]
        pub fn mr2i(&self) -> MR2IR {
            MR2IR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 6;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 7 - Reset on MR2" ]
        # [ inline ( always ) ]
        pub fn mr2r(&self) -> MR2RR {
            MR2RR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 7;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 8 - Stop on MR2." ]
        # [ inline ( always ) ]
        pub fn mr2s(&self) -> MR2SR {
            MR2SR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 8;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 9 - Interrupt on MR3" ]
        # [ inline ( always ) ]
        pub fn mr3i(&self) -> MR3IR {
            MR3IR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 9;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 10 - Reset on MR3" ]
        # [ inline ( always ) ]
        pub fn mr3r(&self) -> MR3RR {
            MR3RR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 10;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 11 - Stop on MR3" ]
        # [ inline ( always ) ]
        pub fn mr3s(&self) -> MR3SR {
            MR3SR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 11;
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
        # [ doc = "Bit 0 - Interrupt on MR0" ]
        # [ inline ( always ) ]
        pub fn mr0i(&mut self) -> _MR0IW {
            _MR0IW { w: self }
        }
        # [ doc = "Bit 1 - Reset on MR0" ]
        # [ inline ( always ) ]
        pub fn mr0r(&mut self) -> _MR0RW {
            _MR0RW { w: self }
        }
        # [ doc = "Bit 2 - Stop on MR0" ]
        # [ inline ( always ) ]
        pub fn mr0s(&mut self) -> _MR0SW {
            _MR0SW { w: self }
        }
        # [ doc = "Bit 3 - Interrupt on MR1" ]
        # [ inline ( always ) ]
        pub fn mr1i(&mut self) -> _MR1IW {
            _MR1IW { w: self }
        }
        # [ doc = "Bit 4 - Reset on MR1" ]
        # [ inline ( always ) ]
        pub fn mr1r(&mut self) -> _MR1RW {
            _MR1RW { w: self }
        }
        # [ doc = "Bit 5 - Stop on MR1" ]
        # [ inline ( always ) ]
        pub fn mr1s(&mut self) -> _MR1SW {
            _MR1SW { w: self }
        }
        # [ doc = "Bit 6 - Interrupt on MR2" ]
        # [ inline ( always ) ]
        pub fn mr2i(&mut self) -> _MR2IW {
            _MR2IW { w: self }
        }
        # [ doc = "Bit 7 - Reset on MR2" ]
        # [ inline ( always ) ]
        pub fn mr2r(&mut self) -> _MR2RW {
            _MR2RW { w: self }
        }
        # [ doc = "Bit 8 - Stop on MR2." ]
        # [ inline ( always ) ]
        pub fn mr2s(&mut self) -> _MR2SW {
            _MR2SW { w: self }
        }
        # [ doc = "Bit 9 - Interrupt on MR3" ]
        # [ inline ( always ) ]
        pub fn mr3i(&mut self) -> _MR3IW {
            _MR3IW { w: self }
        }
        # [ doc = "Bit 10 - Reset on MR3" ]
        # [ inline ( always ) ]
        pub fn mr3r(&mut self) -> _MR3RW {
            _MR3RW { w: self }
        }
        # [ doc = "Bit 11 - Stop on MR3" ]
        # [ inline ( always ) ]
        pub fn mr3s(&mut self) -> _MR3SW {
            _MR3SW { w: self }
        }
    }
}
# [ doc = "Match Register 0. MR0 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR0 matches the TC." ]
pub struct MR0 {
    register: VolatileCell<u32>,
}
# [ doc = "Match Register 0. MR0 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR0 matches the TC." ]
pub mod mr0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MR0 {
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
    pub struct MATCHR {
        bits: u32,
    }
    impl MATCHR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MATCHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MATCHW<'a> {
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
        # [ doc = "Bits 0:31 - Timer counter match value." ]
        # [ inline ( always ) ]
        pub fn match_(&self) -> MATCHR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            MATCHR { bits }
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
        # [ doc = "Bits 0:31 - Timer counter match value." ]
        # [ inline ( always ) ]
        pub fn match_(&mut self) -> _MATCHW {
            _MATCHW { w: self }
        }
    }
}
# [ doc = "Match Register 1. MR1 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR1 matches the TC." ]
pub struct MR1 {
    register: VolatileCell<u32>,
}
# [ doc = "Match Register 1. MR1 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR1 matches the TC." ]
pub mod mr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MR1 {
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
    pub struct MATCHR {
        bits: u32,
    }
    impl MATCHR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MATCHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MATCHW<'a> {
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
        # [ doc = "Bits 0:31 - Timer counter match value." ]
        # [ inline ( always ) ]
        pub fn match_(&self) -> MATCHR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            MATCHR { bits }
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
        # [ doc = "Bits 0:31 - Timer counter match value." ]
        # [ inline ( always ) ]
        pub fn match_(&mut self) -> _MATCHW {
            _MATCHW { w: self }
        }
    }
}
# [ doc = "Match Register 2. MR2 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR2 matches the TC." ]
pub struct MR2 {
    register: VolatileCell<u32>,
}
# [ doc = "Match Register 2. MR2 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR2 matches the TC." ]
pub mod mr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MR2 {
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
    pub struct MATCHR {
        bits: u32,
    }
    impl MATCHR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MATCHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MATCHW<'a> {
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
        # [ doc = "Bits 0:31 - Timer counter match value." ]
        # [ inline ( always ) ]
        pub fn match_(&self) -> MATCHR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            MATCHR { bits }
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
        # [ doc = "Bits 0:31 - Timer counter match value." ]
        # [ inline ( always ) ]
        pub fn match_(&mut self) -> _MATCHW {
            _MATCHW { w: self }
        }
    }
}
# [ doc = "Match Register 3. MR3 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR3 matches the TC." ]
pub struct MR3 {
    register: VolatileCell<u32>,
}
# [ doc = "Match Register 3. MR3 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR3 matches the TC." ]
pub mod mr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MR3 {
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
    pub struct MATCHR {
        bits: u32,
    }
    impl MATCHR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MATCHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MATCHW<'a> {
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
        # [ doc = "Bits 0:31 - Timer counter match value." ]
        # [ inline ( always ) ]
        pub fn match_(&self) -> MATCHR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            MATCHR { bits }
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
        # [ doc = "Bits 0:31 - Timer counter match value." ]
        # [ inline ( always ) ]
        pub fn match_(&mut self) -> _MATCHW {
            _MATCHW { w: self }
        }
    }
}
# [ doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place." ]
pub struct CCR {
    register: VolatileCell<u32>,
}
# [ doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place." ]
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
    # [ doc = "Possible values of the field `CAP0RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP0RER {
        # [ doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP0RER {
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
                CAP0RER::ENABLE => true,
                CAP0RER::DISABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP0RER {
            match value {
                true => CAP0RER::ENABLE,
                false => CAP0RER::DISABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == CAP0RER::ENABLE
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == CAP0RER::DISABLE
        }
    }
    # [ doc = "Possible values of the field `CAP0FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP0FER {
        # [ doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP0FER {
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
                CAP0FER::ENABLE => true,
                CAP0FER::DISABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP0FER {
            match value {
                true => CAP0FER::ENABLE,
                false => CAP0FER::DISABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == CAP0FER::ENABLE
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == CAP0FER::DISABLE
        }
    }
    # [ doc = "Possible values of the field `CAP0I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP0IR {
        # [ doc = "A CR0 load due to a CAPn.0 event will generate an interrupt." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP0IR {
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
                CAP0IR::ENABLE => true,
                CAP0IR::DISABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP0IR {
            match value {
                true => CAP0IR::ENABLE,
                false => CAP0IR::DISABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == CAP0IR::ENABLE
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == CAP0IR::DISABLE
        }
    }
    # [ doc = "Possible values of the field `CAP1RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP1RER {
        # [ doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP1RER {
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
                CAP1RER::ENABLE => true,
                CAP1RER::DISABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP1RER {
            match value {
                true => CAP1RER::ENABLE,
                false => CAP1RER::DISABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == CAP1RER::ENABLE
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == CAP1RER::DISABLE
        }
    }
    # [ doc = "Possible values of the field `CAP1FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP1FER {
        # [ doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP1FER {
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
                CAP1FER::ENABLE => true,
                CAP1FER::DISABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP1FER {
            match value {
                true => CAP1FER::ENABLE,
                false => CAP1FER::DISABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == CAP1FER::ENABLE
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == CAP1FER::DISABLE
        }
    }
    # [ doc = "Possible values of the field `CAP1I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP1IR {
        # [ doc = "A CR1 load due to a CAPn.1 event will generate an interrupt." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP1IR {
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
                CAP1IR::ENABLE => true,
                CAP1IR::DISABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP1IR {
            match value {
                true => CAP1IR::ENABLE,
                false => CAP1IR::DISABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == CAP1IR::ENABLE
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == CAP1IR::DISABLE
        }
    }
    # [ doc = "Values that can be written to the field `CAP0RE`" ]
    pub enum CAP0REW {
        # [ doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP0REW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP0REW::ENABLE => true,
                CAP0REW::DISABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP0REW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0REW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP0REW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(CAP0REW::ENABLE)
        }
        # [ doc = "This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(CAP0REW::DISABLE)
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
    # [ doc = "Values that can be written to the field `CAP0FE`" ]
    pub enum CAP0FEW {
        # [ doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP0FEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP0FEW::ENABLE => true,
                CAP0FEW::DISABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP0FEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0FEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP0FEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(CAP0FEW::ENABLE)
        }
        # [ doc = "This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(CAP0FEW::DISABLE)
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
    # [ doc = "Values that can be written to the field `CAP0I`" ]
    pub enum CAP0IW {
        # [ doc = "A CR0 load due to a CAPn.0 event will generate an interrupt." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP0IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP0IW::ENABLE => true,
                CAP0IW::DISABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP0IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP0IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "A CR0 load due to a CAPn.0 event will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(CAP0IW::ENABLE)
        }
        # [ doc = "This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(CAP0IW::DISABLE)
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
    # [ doc = "Values that can be written to the field `CAP1RE`" ]
    pub enum CAP1REW {
        # [ doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP1REW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP1REW::ENABLE => true,
                CAP1REW::DISABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP1REW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1REW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP1REW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(CAP1REW::ENABLE)
        }
        # [ doc = "This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(CAP1REW::DISABLE)
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
    # [ doc = "Values that can be written to the field `CAP1FE`" ]
    pub enum CAP1FEW {
        # [ doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP1FEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP1FEW::ENABLE => true,
                CAP1FEW::DISABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP1FEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1FEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP1FEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(CAP1FEW::ENABLE)
        }
        # [ doc = "This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(CAP1FEW::DISABLE)
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
    # [ doc = "Values that can be written to the field `CAP1I`" ]
    pub enum CAP1IW {
        # [ doc = "A CR1 load due to a CAPn.1 event will generate an interrupt." ]
        ENABLE,
        # [ doc = "This feature is disabled." ]
        DISABLE,
    }
    impl CAP1IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP1IW::ENABLE => true,
                CAP1IW::DISABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP1IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP1IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "A CR1 load due to a CAPn.1 event will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(CAP1IW::ENABLE)
        }
        # [ doc = "This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(CAP1IW::DISABLE)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Capture on CAPn.0 rising edge" ]
        # [ inline ( always ) ]
        pub fn cap0re(&self) -> CAP0RER {
            CAP0RER::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 0;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 1 - Capture on CAPn.0 falling edge" ]
        # [ inline ( always ) ]
        pub fn cap0fe(&self) -> CAP0FER {
            CAP0FER::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 1;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 2 - Interrupt on CAPn.0 event" ]
        # [ inline ( always ) ]
        pub fn cap0i(&self) -> CAP0IR {
            CAP0IR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 2;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 3 - Capture on CAPn.1 rising edge" ]
        # [ inline ( always ) ]
        pub fn cap1re(&self) -> CAP1RER {
            CAP1RER::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 3;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 4 - Capture on CAPn.1 falling edge" ]
        # [ inline ( always ) ]
        pub fn cap1fe(&self) -> CAP1FER {
            CAP1FER::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 4;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 5 - Interrupt on CAPn.1 event" ]
        # [ inline ( always ) ]
        pub fn cap1i(&self) -> CAP1IR {
            CAP1IR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 5;
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
        # [ doc = "Bit 0 - Capture on CAPn.0 rising edge" ]
        # [ inline ( always ) ]
        pub fn cap0re(&mut self) -> _CAP0REW {
            _CAP0REW { w: self }
        }
        # [ doc = "Bit 1 - Capture on CAPn.0 falling edge" ]
        # [ inline ( always ) ]
        pub fn cap0fe(&mut self) -> _CAP0FEW {
            _CAP0FEW { w: self }
        }
        # [ doc = "Bit 2 - Interrupt on CAPn.0 event" ]
        # [ inline ( always ) ]
        pub fn cap0i(&mut self) -> _CAP0IW {
            _CAP0IW { w: self }
        }
        # [ doc = "Bit 3 - Capture on CAPn.1 rising edge" ]
        # [ inline ( always ) ]
        pub fn cap1re(&mut self) -> _CAP1REW {
            _CAP1REW { w: self }
        }
        # [ doc = "Bit 4 - Capture on CAPn.1 falling edge" ]
        # [ inline ( always ) ]
        pub fn cap1fe(&mut self) -> _CAP1FEW {
            _CAP1FEW { w: self }
        }
        # [ doc = "Bit 5 - Interrupt on CAPn.1 event" ]
        # [ inline ( always ) ]
        pub fn cap1i(&mut self) -> _CAP1IW {
            _CAP1IW { w: self }
        }
    }
}
# [ doc = "Capture Register 0. CR0 is loaded with the value of TC when there is an event on the CAPn.0 input." ]
pub struct CR {
    register: VolatileCell<u32>,
}
# [ doc = "Capture Register 0. CR0 is loaded with the value of TC when there is an event on the CAPn.0 input." ]
pub mod cr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::CR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAPR {
        bits: u32,
    }
    impl CAPR {
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
        # [ doc = "Bits 0:31 - Timer counter capture value." ]
        # [ inline ( always ) ]
        pub fn cap(&self) -> CAPR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            CAPR { bits }
        }
    }
}
# [ doc = "External Match Register. The EMR controls the external match pins." ]
pub struct EMR {
    register: VolatileCell<u32>,
}
# [ doc = "External Match Register. The EMR controls the external match pins." ]
pub mod emr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EMR {
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
    pub struct EM0R {
        bits: bool,
    }
    impl EM0R {
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
    pub struct EM1R {
        bits: bool,
    }
    impl EM1R {
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
    pub struct EM2R {
        bits: bool,
    }
    impl EM2R {
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
    pub struct EM3R {
        bits: bool,
    }
    impl EM3R {
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
    # [ doc = "Possible values of the field `EMC0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EMC0R {
        # [ doc = "Do Nothing." ]
        DO_NOTHING_,
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        CLEAR_THE_CORRESPOND,
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        SET_THE_CORRESPONDIN,
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        TOGGLE_THE_CORRESPON,
    }
    impl EMC0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                EMC0R::DO_NOTHING_ => 0,
                EMC0R::CLEAR_THE_CORRESPOND => 1,
                EMC0R::SET_THE_CORRESPONDIN => 2,
                EMC0R::TOGGLE_THE_CORRESPON => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> EMC0R {
            match value {
                0 => EMC0R::DO_NOTHING_,
                1 => EMC0R::CLEAR_THE_CORRESPOND,
                2 => EMC0R::SET_THE_CORRESPONDIN,
                3 => EMC0R::TOGGLE_THE_CORRESPON,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `DO_NOTHING_`" ]
        # [ inline ( always ) ]
        pub fn is_do_nothing_(&self) -> bool {
            *self == EMC0R::DO_NOTHING_
        }
        # [ doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`" ]
        # [ inline ( always ) ]
        pub fn is_clear_the_correspond(&self) -> bool {
            *self == EMC0R::CLEAR_THE_CORRESPOND
        }
        # [ doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`" ]
        # [ inline ( always ) ]
        pub fn is_set_the_correspondin(&self) -> bool {
            *self == EMC0R::SET_THE_CORRESPONDIN
        }
        # [ doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`" ]
        # [ inline ( always ) ]
        pub fn is_toggle_the_correspon(&self) -> bool {
            *self == EMC0R::TOGGLE_THE_CORRESPON
        }
    }
    # [ doc = "Possible values of the field `EMC1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EMC1R {
        # [ doc = "Do Nothing." ]
        DO_NOTHING_,
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        CLEAR_THE_CORRESPOND,
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        SET_THE_CORRESPONDIN,
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        TOGGLE_THE_CORRESPON,
    }
    impl EMC1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                EMC1R::DO_NOTHING_ => 0,
                EMC1R::CLEAR_THE_CORRESPOND => 1,
                EMC1R::SET_THE_CORRESPONDIN => 2,
                EMC1R::TOGGLE_THE_CORRESPON => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> EMC1R {
            match value {
                0 => EMC1R::DO_NOTHING_,
                1 => EMC1R::CLEAR_THE_CORRESPOND,
                2 => EMC1R::SET_THE_CORRESPONDIN,
                3 => EMC1R::TOGGLE_THE_CORRESPON,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `DO_NOTHING_`" ]
        # [ inline ( always ) ]
        pub fn is_do_nothing_(&self) -> bool {
            *self == EMC1R::DO_NOTHING_
        }
        # [ doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`" ]
        # [ inline ( always ) ]
        pub fn is_clear_the_correspond(&self) -> bool {
            *self == EMC1R::CLEAR_THE_CORRESPOND
        }
        # [ doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`" ]
        # [ inline ( always ) ]
        pub fn is_set_the_correspondin(&self) -> bool {
            *self == EMC1R::SET_THE_CORRESPONDIN
        }
        # [ doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`" ]
        # [ inline ( always ) ]
        pub fn is_toggle_the_correspon(&self) -> bool {
            *self == EMC1R::TOGGLE_THE_CORRESPON
        }
    }
    # [ doc = "Possible values of the field `EMC2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EMC2R {
        # [ doc = "Do Nothing." ]
        DO_NOTHING_,
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        CLEAR_THE_CORRESPOND,
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        SET_THE_CORRESPONDIN,
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        TOGGLE_THE_CORRESPON,
    }
    impl EMC2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                EMC2R::DO_NOTHING_ => 0,
                EMC2R::CLEAR_THE_CORRESPOND => 1,
                EMC2R::SET_THE_CORRESPONDIN => 2,
                EMC2R::TOGGLE_THE_CORRESPON => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> EMC2R {
            match value {
                0 => EMC2R::DO_NOTHING_,
                1 => EMC2R::CLEAR_THE_CORRESPOND,
                2 => EMC2R::SET_THE_CORRESPONDIN,
                3 => EMC2R::TOGGLE_THE_CORRESPON,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `DO_NOTHING_`" ]
        # [ inline ( always ) ]
        pub fn is_do_nothing_(&self) -> bool {
            *self == EMC2R::DO_NOTHING_
        }
        # [ doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`" ]
        # [ inline ( always ) ]
        pub fn is_clear_the_correspond(&self) -> bool {
            *self == EMC2R::CLEAR_THE_CORRESPOND
        }
        # [ doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`" ]
        # [ inline ( always ) ]
        pub fn is_set_the_correspondin(&self) -> bool {
            *self == EMC2R::SET_THE_CORRESPONDIN
        }
        # [ doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`" ]
        # [ inline ( always ) ]
        pub fn is_toggle_the_correspon(&self) -> bool {
            *self == EMC2R::TOGGLE_THE_CORRESPON
        }
    }
    # [ doc = "Possible values of the field `EMC3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EMC3R {
        # [ doc = "Do Nothing." ]
        DO_NOTHING_,
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        CLEAR_THE_CORRESPOND,
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        SET_THE_CORRESPONDIN,
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        TOGGLE_THE_CORRESPON,
    }
    impl EMC3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                EMC3R::DO_NOTHING_ => 0,
                EMC3R::CLEAR_THE_CORRESPOND => 1,
                EMC3R::SET_THE_CORRESPONDIN => 2,
                EMC3R::TOGGLE_THE_CORRESPON => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> EMC3R {
            match value {
                0 => EMC3R::DO_NOTHING_,
                1 => EMC3R::CLEAR_THE_CORRESPOND,
                2 => EMC3R::SET_THE_CORRESPONDIN,
                3 => EMC3R::TOGGLE_THE_CORRESPON,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `DO_NOTHING_`" ]
        # [ inline ( always ) ]
        pub fn is_do_nothing_(&self) -> bool {
            *self == EMC3R::DO_NOTHING_
        }
        # [ doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`" ]
        # [ inline ( always ) ]
        pub fn is_clear_the_correspond(&self) -> bool {
            *self == EMC3R::CLEAR_THE_CORRESPOND
        }
        # [ doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`" ]
        # [ inline ( always ) ]
        pub fn is_set_the_correspondin(&self) -> bool {
            *self == EMC3R::SET_THE_CORRESPONDIN
        }
        # [ doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`" ]
        # [ inline ( always ) ]
        pub fn is_toggle_the_correspon(&self) -> bool {
            *self == EMC3R::TOGGLE_THE_CORRESPON
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EM0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EM0W<'a> {
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
    pub struct _EM1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EM1W<'a> {
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
    pub struct _EM2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EM2W<'a> {
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
    pub struct _EM3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EM3W<'a> {
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
    # [ doc = "Values that can be written to the field `EMC0`" ]
    pub enum EMC0W {
        # [ doc = "Do Nothing." ]
        DO_NOTHING_,
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        CLEAR_THE_CORRESPOND,
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        SET_THE_CORRESPONDIN,
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        TOGGLE_THE_CORRESPON,
    }
    impl EMC0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                EMC0W::DO_NOTHING_ => 0,
                EMC0W::CLEAR_THE_CORRESPOND => 1,
                EMC0W::SET_THE_CORRESPONDIN => 2,
                EMC0W::TOGGLE_THE_CORRESPON => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EMC0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EMC0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EMC0W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Do Nothing." ]
        # [ inline ( always ) ]
        pub fn do_nothing_(self) -> &'a mut W {
            self.variant(EMC0W::DO_NOTHING_)
        }
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        # [ inline ( always ) ]
        pub fn clear_the_correspond(self) -> &'a mut W {
            self.variant(EMC0W::CLEAR_THE_CORRESPOND)
        }
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        # [ inline ( always ) ]
        pub fn set_the_correspondin(self) -> &'a mut W {
            self.variant(EMC0W::SET_THE_CORRESPONDIN)
        }
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        # [ inline ( always ) ]
        pub fn toggle_the_correspon(self) -> &'a mut W {
            self.variant(EMC0W::TOGGLE_THE_CORRESPON)
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
    # [ doc = "Values that can be written to the field `EMC1`" ]
    pub enum EMC1W {
        # [ doc = "Do Nothing." ]
        DO_NOTHING_,
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        CLEAR_THE_CORRESPOND,
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        SET_THE_CORRESPONDIN,
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        TOGGLE_THE_CORRESPON,
    }
    impl EMC1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                EMC1W::DO_NOTHING_ => 0,
                EMC1W::CLEAR_THE_CORRESPOND => 1,
                EMC1W::SET_THE_CORRESPONDIN => 2,
                EMC1W::TOGGLE_THE_CORRESPON => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EMC1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EMC1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EMC1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Do Nothing." ]
        # [ inline ( always ) ]
        pub fn do_nothing_(self) -> &'a mut W {
            self.variant(EMC1W::DO_NOTHING_)
        }
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        # [ inline ( always ) ]
        pub fn clear_the_correspond(self) -> &'a mut W {
            self.variant(EMC1W::CLEAR_THE_CORRESPOND)
        }
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        # [ inline ( always ) ]
        pub fn set_the_correspondin(self) -> &'a mut W {
            self.variant(EMC1W::SET_THE_CORRESPONDIN)
        }
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        # [ inline ( always ) ]
        pub fn toggle_the_correspon(self) -> &'a mut W {
            self.variant(EMC1W::TOGGLE_THE_CORRESPON)
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
    # [ doc = "Values that can be written to the field `EMC2`" ]
    pub enum EMC2W {
        # [ doc = "Do Nothing." ]
        DO_NOTHING_,
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        CLEAR_THE_CORRESPOND,
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        SET_THE_CORRESPONDIN,
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        TOGGLE_THE_CORRESPON,
    }
    impl EMC2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                EMC2W::DO_NOTHING_ => 0,
                EMC2W::CLEAR_THE_CORRESPOND => 1,
                EMC2W::SET_THE_CORRESPONDIN => 2,
                EMC2W::TOGGLE_THE_CORRESPON => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EMC2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EMC2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EMC2W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Do Nothing." ]
        # [ inline ( always ) ]
        pub fn do_nothing_(self) -> &'a mut W {
            self.variant(EMC2W::DO_NOTHING_)
        }
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        # [ inline ( always ) ]
        pub fn clear_the_correspond(self) -> &'a mut W {
            self.variant(EMC2W::CLEAR_THE_CORRESPOND)
        }
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        # [ inline ( always ) ]
        pub fn set_the_correspondin(self) -> &'a mut W {
            self.variant(EMC2W::SET_THE_CORRESPONDIN)
        }
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        # [ inline ( always ) ]
        pub fn toggle_the_correspon(self) -> &'a mut W {
            self.variant(EMC2W::TOGGLE_THE_CORRESPON)
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
    # [ doc = "Values that can be written to the field `EMC3`" ]
    pub enum EMC3W {
        # [ doc = "Do Nothing." ]
        DO_NOTHING_,
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        CLEAR_THE_CORRESPOND,
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        SET_THE_CORRESPONDIN,
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        TOGGLE_THE_CORRESPON,
    }
    impl EMC3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                EMC3W::DO_NOTHING_ => 0,
                EMC3W::CLEAR_THE_CORRESPOND => 1,
                EMC3W::SET_THE_CORRESPONDIN => 2,
                EMC3W::TOGGLE_THE_CORRESPON => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EMC3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EMC3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EMC3W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Do Nothing." ]
        # [ inline ( always ) ]
        pub fn do_nothing_(self) -> &'a mut W {
            self.variant(EMC3W::DO_NOTHING_)
        }
        # [ doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)." ]
        # [ inline ( always ) ]
        pub fn clear_the_correspond(self) -> &'a mut W {
            self.variant(EMC3W::CLEAR_THE_CORRESPOND)
        }
        # [ doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)." ]
        # [ inline ( always ) ]
        pub fn set_the_correspondin(self) -> &'a mut W {
            self.variant(EMC3W::SET_THE_CORRESPONDIN)
        }
        # [ doc = "Toggle the corresponding External Match bit/output." ]
        # [ inline ( always ) ]
        pub fn toggle_the_correspon(self) -> &'a mut W {
            self.variant(EMC3W::TOGGLE_THE_CORRESPON)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)." ]
        # [ inline ( always ) ]
        pub fn em0(&self) -> EM0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EM0R { bits }
        }
        # [ doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)." ]
        # [ inline ( always ) ]
        pub fn em1(&self) -> EM1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EM1R { bits }
        }
        # [ doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)." ]
        # [ inline ( always ) ]
        pub fn em2(&self) -> EM2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EM2R { bits }
        }
        # [ doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)." ]
        # [ inline ( always ) ]
        pub fn em3(&self) -> EM3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EM3R { bits }
        }
        # [ doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0." ]
        # [ inline ( always ) ]
        pub fn emc0(&self) -> EMC0R {
            EMC0R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 4;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1." ]
        # [ inline ( always ) ]
        pub fn emc1(&self) -> EMC1R {
            EMC1R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 6;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2." ]
        # [ inline ( always ) ]
        pub fn emc2(&self) -> EMC2R {
            EMC2R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 8;
                             ((self.bits >> OFFSET) & MASK as u32) as u8
                         })
        }
        # [ doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3." ]
        # [ inline ( always ) ]
        pub fn emc3(&self) -> EMC3R {
            EMC3R::_from({
                             const MASK: u8 = 3;
                             const OFFSET: u8 = 10;
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
        # [ doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)." ]
        # [ inline ( always ) ]
        pub fn em0(&mut self) -> _EM0W {
            _EM0W { w: self }
        }
        # [ doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)." ]
        # [ inline ( always ) ]
        pub fn em1(&mut self) -> _EM1W {
            _EM1W { w: self }
        }
        # [ doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)." ]
        # [ inline ( always ) ]
        pub fn em2(&mut self) -> _EM2W {
            _EM2W { w: self }
        }
        # [ doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)." ]
        # [ inline ( always ) ]
        pub fn em3(&mut self) -> _EM3W {
            _EM3W { w: self }
        }
        # [ doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0." ]
        # [ inline ( always ) ]
        pub fn emc0(&mut self) -> _EMC0W {
            _EMC0W { w: self }
        }
        # [ doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1." ]
        # [ inline ( always ) ]
        pub fn emc1(&mut self) -> _EMC1W {
            _EMC1W { w: self }
        }
        # [ doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2." ]
        # [ inline ( always ) ]
        pub fn emc2(&mut self) -> _EMC2W {
            _EMC2W { w: self }
        }
        # [ doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3." ]
        # [ inline ( always ) ]
        pub fn emc3(&mut self) -> _EMC3W {
            _EMC3W { w: self }
        }
    }
}
# [ doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting." ]
pub struct CTCR {
    register: VolatileCell<u32>,
}
# [ doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting." ]
pub mod ctcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CTCR {
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
    # [ doc = "Possible values of the field `CTMODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CTMODER {
        # [ doc = "Timer Mode: every rising PCLK edge" ]
        TIMER_MODE_EVERY_RI,
        # [ doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2." ]
        RISING,
        # [ doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2." ]
        FALLING,
        # [ doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2." ]
        DUALEDGE,
    }
    impl CTMODER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                CTMODER::TIMER_MODE_EVERY_RI => 0,
                CTMODER::RISING => 1,
                CTMODER::FALLING => 2,
                CTMODER::DUALEDGE => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> CTMODER {
            match value {
                0 => CTMODER::TIMER_MODE_EVERY_RI,
                1 => CTMODER::RISING,
                2 => CTMODER::FALLING,
                3 => CTMODER::DUALEDGE,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `TIMER_MODE_EVERY_RI`" ]
        # [ inline ( always ) ]
        pub fn is_timer_mode_every_ri(&self) -> bool {
            *self == CTMODER::TIMER_MODE_EVERY_RI
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == CTMODER::RISING
        }
        # [ doc = "Checks if the value of the field is `FALLING`" ]
        # [ inline ( always ) ]
        pub fn is_falling(&self) -> bool {
            *self == CTMODER::FALLING
        }
        # [ doc = "Checks if the value of the field is `DUALEDGE`" ]
        # [ inline ( always ) ]
        pub fn is_dualedge(&self) -> bool {
            *self == CTMODER::DUALEDGE
        }
    }
    # [ doc = "Possible values of the field `CINSEL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CINSELR {
        # [ doc = "CAPn.0 for TIMERn" ]
        CAPN_0_FOR_TIMERN,
        # [ doc = "CAPn.1 for TIMERn" ]
        CAPN_1_FOR_TIMERN,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl CINSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                CINSELR::CAPN_0_FOR_TIMERN => 0,
                CINSELR::CAPN_1_FOR_TIMERN => 1,
                CINSELR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> CINSELR {
            match value {
                0 => CINSELR::CAPN_0_FOR_TIMERN,
                1 => CINSELR::CAPN_1_FOR_TIMERN,
                i => CINSELR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `CAPN_0_FOR_TIMERN`" ]
        # [ inline ( always ) ]
        pub fn is_capn_0_for_timern(&self) -> bool {
            *self == CINSELR::CAPN_0_FOR_TIMERN
        }
        # [ doc = "Checks if the value of the field is `CAPN_1_FOR_TIMERN`" ]
        # [ inline ( always ) ]
        pub fn is_capn_1_for_timern(&self) -> bool {
            *self == CINSELR::CAPN_1_FOR_TIMERN
        }
    }
    # [ doc = "Values that can be written to the field `CTMODE`" ]
    pub enum CTMODEW {
        # [ doc = "Timer Mode: every rising PCLK edge" ]
        TIMER_MODE_EVERY_RI,
        # [ doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2." ]
        RISING,
        # [ doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2." ]
        FALLING,
        # [ doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2." ]
        DUALEDGE,
    }
    impl CTMODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                CTMODEW::TIMER_MODE_EVERY_RI => 0,
                CTMODEW::RISING => 1,
                CTMODEW::FALLING => 2,
                CTMODEW::DUALEDGE => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CTMODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTMODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CTMODEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Timer Mode: every rising PCLK edge" ]
        # [ inline ( always ) ]
        pub fn timer_mode_every_ri(self) -> &'a mut W {
            self.variant(CTMODEW::TIMER_MODE_EVERY_RI)
        }
        # [ doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2." ]
        # [ inline ( always ) ]
        pub fn rising(self) -> &'a mut W {
            self.variant(CTMODEW::RISING)
        }
        # [ doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2." ]
        # [ inline ( always ) ]
        pub fn falling(self) -> &'a mut W {
            self.variant(CTMODEW::FALLING)
        }
        # [ doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2." ]
        # [ inline ( always ) ]
        pub fn dualedge(self) -> &'a mut W {
            self.variant(CTMODEW::DUALEDGE)
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
    # [ doc = "Values that can be written to the field `CINSEL`" ]
    pub enum CINSELW {
        # [ doc = "CAPn.0 for TIMERn" ]
        CAPN_0_FOR_TIMERN,
        # [ doc = "CAPn.1 for TIMERn" ]
        CAPN_1_FOR_TIMERN,
    }
    impl CINSELW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                CINSELW::CAPN_0_FOR_TIMERN => 0,
                CINSELW::CAPN_1_FOR_TIMERN => 1,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CINSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CINSELW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CINSELW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "CAPn.0 for TIMERn" ]
        # [ inline ( always ) ]
        pub fn capn_0_for_timern(self) -> &'a mut W {
            self.variant(CINSELW::CAPN_0_FOR_TIMERN)
        }
        # [ doc = "CAPn.1 for TIMERn" ]
        # [ inline ( always ) ]
        pub fn capn_1_for_timern(self) -> &'a mut W {
            self.variant(CINSELW::CAPN_1_FOR_TIMERN)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register." ]
        # [ inline ( always ) ]
        pub fn ctmode(&self) -> CTMODER {
            CTMODER::_from({
                               const MASK: u8 = 3;
                               const OFFSET: u8 = 0;
                               ((self.bits >> OFFSET) & MASK as u32) as u8
                           })
        }
        # [ doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer." ]
        # [ inline ( always ) ]
        pub fn cinsel(&self) -> CINSELR {
            CINSELR::_from({
                               const MASK: u8 = 3;
                               const OFFSET: u8 = 2;
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
        # [ doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register." ]
        # [ inline ( always ) ]
        pub fn ctmode(&mut self) -> _CTMODEW {
            _CTMODEW { w: self }
        }
        # [ doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer." ]
        # [ inline ( always ) ]
        pub fn cinsel(&mut self) -> _CINSELW {
            _CINSELW { w: self }
        }
    }
}
# [ doc = "Timer0/1/2/3" ]
pub struct TIMER0 {
    register_block: RegisterBlock,
}
impl Deref for TIMER0 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "TIMER1" ]
pub const TIMER1: Peripheral<TIMER1> = unsafe { Peripheral::new(1073774592) };
# [ doc = r" Register block" ]
pub struct TIMER1 {
    register_block: RegisterBlock,
}
impl Deref for TIMER1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "TIMER2" ]
pub const TIMER2: Peripheral<TIMER2> = unsafe { Peripheral::new(1074331648) };
# [ doc = r" Register block" ]
pub struct TIMER2 {
    register_block: RegisterBlock,
}
impl Deref for TIMER2 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "TIMER3" ]
pub const TIMER3: Peripheral<TIMER3> = unsafe { Peripheral::new(1074348032) };
# [ doc = r" Register block" ]
pub struct TIMER3 {
    register_block: RegisterBlock,
}
impl Deref for TIMER3 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
