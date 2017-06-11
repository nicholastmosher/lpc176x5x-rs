# ! [ doc = "Pulse Width Modulators (PWM1)" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "Pulse Width Modulators (PWM1)" ]
pub const PWM1: Peripheral<PWM1> = unsafe { Peripheral::new(1073840128) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending." ]
    pub ir: IR,
    # [ doc = "0x04 - Timer Control Register. The TCR is used to control the Timer Counter functions." ]
    pub tcr: TCR,
    # [ doc = "0x08 - Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR." ]
    pub tc: TC,
    # [ doc = "0x0c - Prescale Register. Determines how often the PWM counter is incremented." ]
    pub pr: PR,
    # [ doc = "0x10 - Prescale Counter. Prescaler for the main PWM counter." ]
    pub pc: PC,
    # [ doc = "0x14 - Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs." ]
    pub mcr: MCR,
    # [ doc = "0x18 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
    pub mr0: MR0,
    # [ doc = "0x1c - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
    pub mr1: MR1,
    # [ doc = "0x20 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
    pub mr2: MR2,
    # [ doc = "0x24 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
    pub mr3: MR3,
    # [ doc = "0x28 - Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event." ]
    pub ccr: CCR,
    # [ doc = "0x2c - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs." ]
    pub cr0: CR,
    # [ doc = "0x30 - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs." ]
    pub cr1: CR,
    _reserved0: [u8; 12usize],
    # [ doc = "0x40 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
    pub mr4: MR,
    # [ doc = "0x44 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
    pub mr5: MR,
    # [ doc = "0x48 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
    pub mr6: MR,
    # [ doc = "0x4c - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs." ]
    pub pcr: PCR,
    # [ doc = "0x50 - Load Enable Register. Enables use of updated PWM match values." ]
    pub ler: LER,
    _reserved1: [u8; 28usize],
    # [ doc = "0x70 - Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting." ]
    pub ctcr: CTCR,
}
# [ doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending." ]
pub struct IR {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending." ]
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
    pub struct PWMMR0INTR {
        bits: bool,
    }
    impl PWMMR0INTR {
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
    pub struct PWMMR1INTR {
        bits: bool,
    }
    impl PWMMR1INTR {
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
    pub struct PWMMR2INTR {
        bits: bool,
    }
    impl PWMMR2INTR {
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
    pub struct PWMMR3INTR {
        bits: bool,
    }
    impl PWMMR3INTR {
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
    pub struct PWMCAP0INTR {
        bits: bool,
    }
    impl PWMCAP0INTR {
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
    pub struct PWMCAP1INTR {
        bits: bool,
    }
    impl PWMCAP1INTR {
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
    pub struct PWMMR4INTR {
        bits: bool,
    }
    impl PWMMR4INTR {
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
    pub struct PWMMR5INTR {
        bits: bool,
    }
    impl PWMMR5INTR {
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
    pub struct PWMMR6INTR {
        bits: bool,
    }
    impl PWMMR6INTR {
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
    pub struct _PWMMR0INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR0INTW<'a> {
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
    pub struct _PWMMR1INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR1INTW<'a> {
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
    pub struct _PWMMR2INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR2INTW<'a> {
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
    pub struct _PWMMR3INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR3INTW<'a> {
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
    pub struct _PWMCAP0INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMCAP0INTW<'a> {
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
    pub struct _PWMCAP1INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMCAP1INTW<'a> {
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
    pub struct _PWMMR4INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR4INTW<'a> {
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
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR5INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR5INTW<'a> {
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
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR6INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR6INTW<'a> {
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
        # [ doc = "Bit 0 - Interrupt flag for PWM match channel 0." ]
        # [ inline ( always ) ]
        pub fn pwmmr0int(&self) -> PWMMR0INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWMMR0INTR { bits }
        }
        # [ doc = "Bit 1 - Interrupt flag for PWM match channel 1." ]
        # [ inline ( always ) ]
        pub fn pwmmr1int(&self) -> PWMMR1INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWMMR1INTR { bits }
        }
        # [ doc = "Bit 2 - Interrupt flag for PWM match channel 2." ]
        # [ inline ( always ) ]
        pub fn pwmmr2int(&self) -> PWMMR2INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWMMR2INTR { bits }
        }
        # [ doc = "Bit 3 - Interrupt flag for PWM match channel 3." ]
        # [ inline ( always ) ]
        pub fn pwmmr3int(&self) -> PWMMR3INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWMMR3INTR { bits }
        }
        # [ doc = "Bit 4 - Interrupt flag for capture input 0" ]
        # [ inline ( always ) ]
        pub fn pwmcap0int(&self) -> PWMCAP0INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWMCAP0INTR { bits }
        }
        # [ doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)." ]
        # [ inline ( always ) ]
        pub fn pwmcap1int(&self) -> PWMCAP1INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWMCAP1INTR { bits }
        }
        # [ doc = "Bit 8 - Interrupt flag for PWM match channel 4." ]
        # [ inline ( always ) ]
        pub fn pwmmr4int(&self) -> PWMMR4INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWMMR4INTR { bits }
        }
        # [ doc = "Bit 9 - Interrupt flag for PWM match channel 5." ]
        # [ inline ( always ) ]
        pub fn pwmmr5int(&self) -> PWMMR5INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWMMR5INTR { bits }
        }
        # [ doc = "Bit 10 - Interrupt flag for PWM match channel 6." ]
        # [ inline ( always ) ]
        pub fn pwmmr6int(&self) -> PWMMR6INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PWMMR6INTR { bits }
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
        # [ doc = "Bit 0 - Interrupt flag for PWM match channel 0." ]
        # [ inline ( always ) ]
        pub fn pwmmr0int(&mut self) -> _PWMMR0INTW {
            _PWMMR0INTW { w: self }
        }
        # [ doc = "Bit 1 - Interrupt flag for PWM match channel 1." ]
        # [ inline ( always ) ]
        pub fn pwmmr1int(&mut self) -> _PWMMR1INTW {
            _PWMMR1INTW { w: self }
        }
        # [ doc = "Bit 2 - Interrupt flag for PWM match channel 2." ]
        # [ inline ( always ) ]
        pub fn pwmmr2int(&mut self) -> _PWMMR2INTW {
            _PWMMR2INTW { w: self }
        }
        # [ doc = "Bit 3 - Interrupt flag for PWM match channel 3." ]
        # [ inline ( always ) ]
        pub fn pwmmr3int(&mut self) -> _PWMMR3INTW {
            _PWMMR3INTW { w: self }
        }
        # [ doc = "Bit 4 - Interrupt flag for capture input 0" ]
        # [ inline ( always ) ]
        pub fn pwmcap0int(&mut self) -> _PWMCAP0INTW {
            _PWMCAP0INTW { w: self }
        }
        # [ doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)." ]
        # [ inline ( always ) ]
        pub fn pwmcap1int(&mut self) -> _PWMCAP1INTW {
            _PWMCAP1INTW { w: self }
        }
        # [ doc = "Bit 8 - Interrupt flag for PWM match channel 4." ]
        # [ inline ( always ) ]
        pub fn pwmmr4int(&mut self) -> _PWMMR4INTW {
            _PWMMR4INTW { w: self }
        }
        # [ doc = "Bit 9 - Interrupt flag for PWM match channel 5." ]
        # [ inline ( always ) ]
        pub fn pwmmr5int(&mut self) -> _PWMMR5INTW {
            _PWMMR5INTW { w: self }
        }
        # [ doc = "Bit 10 - Interrupt flag for PWM match channel 6." ]
        # [ inline ( always ) ]
        pub fn pwmmr6int(&mut self) -> _PWMMR6INTW {
            _PWMMR6INTW { w: self }
        }
    }
}
# [ doc = "Timer Control Register. The TCR is used to control the Timer Counter functions." ]
pub struct TCR {
    register: VolatileCell<u32>,
}
# [ doc = "Timer Control Register. The TCR is used to control the Timer Counter functions." ]
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
    # [ doc = "Possible values of the field `CE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CER {
        # [ doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting." ]
        THE_PWM_TIMER_COUNTE,
        # [ doc = "The counters are disabled." ]
        THE_COUNTERS_ARE_DIS,
    }
    impl CER {
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
                CER::THE_PWM_TIMER_COUNTE => true,
                CER::THE_COUNTERS_ARE_DIS => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CER {
            match value {
                true => CER::THE_PWM_TIMER_COUNTE,
                false => CER::THE_COUNTERS_ARE_DIS,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_TIMER_COUNTE`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_timer_counte(&self) -> bool {
            *self == CER::THE_PWM_TIMER_COUNTE
        }
        # [ doc = "Checks if the value of the field is `THE_COUNTERS_ARE_DIS`" ]
        # [ inline ( always ) ]
        pub fn is_the_counters_are_dis(&self) -> bool {
            *self == CER::THE_COUNTERS_ARE_DIS
        }
    }
    # [ doc = "Possible values of the field `CR`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CRR {
        # [ doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero." ]
        THE_PWM_TIMER_COUNTE,
        # [ doc = "Clear reset." ]
        CLEAR_RESET_,
    }
    impl CRR {
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
                CRR::THE_PWM_TIMER_COUNTE => true,
                CRR::CLEAR_RESET_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CRR {
            match value {
                true => CRR::THE_PWM_TIMER_COUNTE,
                false => CRR::CLEAR_RESET_,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_TIMER_COUNTE`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_timer_counte(&self) -> bool {
            *self == CRR::THE_PWM_TIMER_COUNTE
        }
        # [ doc = "Checks if the value of the field is `CLEAR_RESET_`" ]
        # [ inline ( always ) ]
        pub fn is_clear_reset_(&self) -> bool {
            *self == CRR::CLEAR_RESET_
        }
    }
    # [ doc = "Possible values of the field `PWMEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENR {
        # [ doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective." ]
        PWM_MODE_IS_ENABLED_,
        # [ doc = "Timer mode is enabled (counter resets to 0)." ]
        TIMER_MODE_IS_ENABLE,
    }
    impl PWMENR {
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
                PWMENR::PWM_MODE_IS_ENABLED_ => true,
                PWMENR::TIMER_MODE_IS_ENABLE => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENR {
            match value {
                true => PWMENR::PWM_MODE_IS_ENABLED_,
                false => PWMENR::TIMER_MODE_IS_ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_pwm_mode_is_enabled_(&self) -> bool {
            *self == PWMENR::PWM_MODE_IS_ENABLED_
        }
        # [ doc = "Checks if the value of the field is `TIMER_MODE_IS_ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_timer_mode_is_enable(&self) -> bool {
            *self == PWMENR::TIMER_MODE_IS_ENABLE
        }
    }
    # [ doc = "Possible values of the field `MDIS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MDISR {
        # [ doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting." ]
        MASTER_USE_PWM0_IS_,
        # [ doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs." ]
        INDIVIDUAL_USE_THE_,
    }
    impl MDISR {
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
                MDISR::MASTER_USE_PWM0_IS_ => true,
                MDISR::INDIVIDUAL_USE_THE_ => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MDISR {
            match value {
                true => MDISR::MASTER_USE_PWM0_IS_,
                false => MDISR::INDIVIDUAL_USE_THE_,
            }
        }
        # [ doc = "Checks if the value of the field is `MASTER_USE_PWM0_IS_`" ]
        # [ inline ( always ) ]
        pub fn is_master_use_pwm0_is_(&self) -> bool {
            *self == MDISR::MASTER_USE_PWM0_IS_
        }
        # [ doc = "Checks if the value of the field is `INDIVIDUAL_USE_THE_`" ]
        # [ inline ( always ) ]
        pub fn is_individual_use_the_(&self) -> bool {
            *self == MDISR::INDIVIDUAL_USE_THE_
        }
    }
    # [ doc = "Values that can be written to the field `CE`" ]
    pub enum CEW {
        # [ doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting." ]
        THE_PWM_TIMER_COUNTE,
        # [ doc = "The counters are disabled." ]
        THE_COUNTERS_ARE_DIS,
    }
    impl CEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CEW::THE_PWM_TIMER_COUNTE => true,
                CEW::THE_COUNTERS_ARE_DIS => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting." ]
        # [ inline ( always ) ]
        pub fn the_pwm_timer_counte(self) -> &'a mut W {
            self.variant(CEW::THE_PWM_TIMER_COUNTE)
        }
        # [ doc = "The counters are disabled." ]
        # [ inline ( always ) ]
        pub fn the_counters_are_dis(self) -> &'a mut W {
            self.variant(CEW::THE_COUNTERS_ARE_DIS)
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
    # [ doc = "Values that can be written to the field `CR`" ]
    pub enum CRW {
        # [ doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero." ]
        THE_PWM_TIMER_COUNTE,
        # [ doc = "Clear reset." ]
        CLEAR_RESET_,
    }
    impl CRW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CRW::THE_PWM_TIMER_COUNTE => true,
                CRW::CLEAR_RESET_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CRW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero." ]
        # [ inline ( always ) ]
        pub fn the_pwm_timer_counte(self) -> &'a mut W {
            self.variant(CRW::THE_PWM_TIMER_COUNTE)
        }
        # [ doc = "Clear reset." ]
        # [ inline ( always ) ]
        pub fn clear_reset_(self) -> &'a mut W {
            self.variant(CRW::CLEAR_RESET_)
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
    # [ doc = "Values that can be written to the field `PWMEN`" ]
    pub enum PWMENW {
        # [ doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective." ]
        PWM_MODE_IS_ENABLED_,
        # [ doc = "Timer mode is enabled (counter resets to 0)." ]
        TIMER_MODE_IS_ENABLE,
    }
    impl PWMENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENW::PWM_MODE_IS_ENABLED_ => true,
                PWMENW::TIMER_MODE_IS_ENABLE => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective." ]
        # [ inline ( always ) ]
        pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
            self.variant(PWMENW::PWM_MODE_IS_ENABLED_)
        }
        # [ doc = "Timer mode is enabled (counter resets to 0)." ]
        # [ inline ( always ) ]
        pub fn timer_mode_is_enable(self) -> &'a mut W {
            self.variant(PWMENW::TIMER_MODE_IS_ENABLE)
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
    # [ doc = "Values that can be written to the field `MDIS`" ]
    pub enum MDISW {
        # [ doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting." ]
        MASTER_USE_PWM0_IS_,
        # [ doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs." ]
        INDIVIDUAL_USE_THE_,
    }
    impl MDISW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MDISW::MASTER_USE_PWM0_IS_ => true,
                MDISW::INDIVIDUAL_USE_THE_ => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MDISW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MDISW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MDISW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting." ]
        # [ inline ( always ) ]
        pub fn master_use_pwm0_is_(self) -> &'a mut W {
            self.variant(MDISW::MASTER_USE_PWM0_IS_)
        }
        # [ doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs." ]
        # [ inline ( always ) ]
        pub fn individual_use_the_(self) -> &'a mut W {
            self.variant(MDISW::INDIVIDUAL_USE_THE_)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Counter Enable" ]
        # [ inline ( always ) ]
        pub fn ce(&self) -> CER {
            CER::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 0;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 1 - Counter Reset" ]
        # [ inline ( always ) ]
        pub fn cr(&self) -> CRR {
            CRR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 1;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 3 - PWM Enable" ]
        # [ inline ( always ) ]
        pub fn pwmen(&self) -> PWMENR {
            PWMENR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 3;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)." ]
        # [ inline ( always ) ]
        pub fn mdis(&self) -> MDISR {
            MDISR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 4;
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
        # [ doc = "Bit 0 - Counter Enable" ]
        # [ inline ( always ) ]
        pub fn ce(&mut self) -> _CEW {
            _CEW { w: self }
        }
        # [ doc = "Bit 1 - Counter Reset" ]
        # [ inline ( always ) ]
        pub fn cr(&mut self) -> _CRW {
            _CRW { w: self }
        }
        # [ doc = "Bit 3 - PWM Enable" ]
        # [ inline ( always ) ]
        pub fn pwmen(&mut self) -> _PWMENW {
            _PWMENW { w: self }
        }
        # [ doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)." ]
        # [ inline ( always ) ]
        pub fn mdis(&mut self) -> _MDISW {
            _MDISW { w: self }
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
# [ doc = "Prescale Register. Determines how often the PWM counter is incremented." ]
pub struct PR {
    register: VolatileCell<u32>,
}
# [ doc = "Prescale Register. Determines how often the PWM counter is incremented." ]
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
# [ doc = "Prescale Counter. Prescaler for the main PWM counter." ]
pub struct PC {
    register: VolatileCell<u32>,
}
# [ doc = "Prescale Counter. Prescaler for the main PWM counter." ]
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
# [ doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs." ]
pub struct MCR {
    register: VolatileCell<u32>,
}
# [ doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs." ]
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
    # [ doc = "Possible values of the field `PWMMR0I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR0IR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR0,
    }
    impl PWMMR0IR {
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
                PWMMR0IR::DISABLED_ => false,
                PWMMR0IR::INTERRUPT_ON_PWMMR0 => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR0IR {
            match value {
                false => PWMMR0IR::DISABLED_,
                true => PWMMR0IR::INTERRUPT_ON_PWMMR0,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR0IR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR0`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_on_pwmmr0(&self) -> bool {
            *self == PWMMR0IR::INTERRUPT_ON_PWMMR0
        }
    }
    # [ doc = "Possible values of the field `PWMMR0R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR0RR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it." ]
        RESET_ON_PWMMR0_THE,
    }
    impl PWMMR0RR {
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
                PWMMR0RR::DISABLED_ => false,
                PWMMR0RR::RESET_ON_PWMMR0_THE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR0RR {
            match value {
                false => PWMMR0RR::DISABLED_,
                true => PWMMR0RR::RESET_ON_PWMMR0_THE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR0RR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `RESET_ON_PWMMR0_THE`" ]
        # [ inline ( always ) ]
        pub fn is_reset_on_pwmmr0_the(&self) -> bool {
            *self == PWMMR0RR::RESET_ON_PWMMR0_THE
        }
    }
    # [ doc = "Possible values of the field `PWMMR0S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR0SR {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC." ]
        STOP_ON_PWMMR0_THE_,
    }
    impl PWMMR0SR {
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
                PWMMR0SR::DISABLED => false,
                PWMMR0SR::STOP_ON_PWMMR0_THE_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR0SR {
            match value {
                false => PWMMR0SR::DISABLED,
                true => PWMMR0SR::STOP_ON_PWMMR0_THE_,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == PWMMR0SR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `STOP_ON_PWMMR0_THE_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_on_pwmmr0_the_(&self) -> bool {
            *self == PWMMR0SR::STOP_ON_PWMMR0_THE_
        }
    }
    # [ doc = "Possible values of the field `PWMMR1I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR1IR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR1,
    }
    impl PWMMR1IR {
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
                PWMMR1IR::DISABLED_ => false,
                PWMMR1IR::INTERRUPT_ON_PWMMR1 => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR1IR {
            match value {
                false => PWMMR1IR::DISABLED_,
                true => PWMMR1IR::INTERRUPT_ON_PWMMR1,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR1IR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR1`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_on_pwmmr1(&self) -> bool {
            *self == PWMMR1IR::INTERRUPT_ON_PWMMR1
        }
    }
    # [ doc = "Possible values of the field `PWMMR1R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR1RR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it." ]
        RESET_ON_PWMMR1_THE,
    }
    impl PWMMR1RR {
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
                PWMMR1RR::DISABLED_ => false,
                PWMMR1RR::RESET_ON_PWMMR1_THE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR1RR {
            match value {
                false => PWMMR1RR::DISABLED_,
                true => PWMMR1RR::RESET_ON_PWMMR1_THE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR1RR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `RESET_ON_PWMMR1_THE`" ]
        # [ inline ( always ) ]
        pub fn is_reset_on_pwmmr1_the(&self) -> bool {
            *self == PWMMR1RR::RESET_ON_PWMMR1_THE
        }
    }
    # [ doc = "Possible values of the field `PWMMR1S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR1SR {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC." ]
        STOP_ON_PWMMR1_THE_,
    }
    impl PWMMR1SR {
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
                PWMMR1SR::DISABLED => false,
                PWMMR1SR::STOP_ON_PWMMR1_THE_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR1SR {
            match value {
                false => PWMMR1SR::DISABLED,
                true => PWMMR1SR::STOP_ON_PWMMR1_THE_,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == PWMMR1SR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `STOP_ON_PWMMR1_THE_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_on_pwmmr1_the_(&self) -> bool {
            *self == PWMMR1SR::STOP_ON_PWMMR1_THE_
        }
    }
    # [ doc = "Possible values of the field `PWMMR2I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR2IR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR2,
    }
    impl PWMMR2IR {
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
                PWMMR2IR::DISABLED_ => false,
                PWMMR2IR::INTERRUPT_ON_PWMMR2 => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR2IR {
            match value {
                false => PWMMR2IR::DISABLED_,
                true => PWMMR2IR::INTERRUPT_ON_PWMMR2,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR2IR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR2`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_on_pwmmr2(&self) -> bool {
            *self == PWMMR2IR::INTERRUPT_ON_PWMMR2
        }
    }
    # [ doc = "Possible values of the field `PWMMR2R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR2RR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it." ]
        RESET_ON_PWMMR2_THE,
    }
    impl PWMMR2RR {
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
                PWMMR2RR::DISABLED_ => false,
                PWMMR2RR::RESET_ON_PWMMR2_THE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR2RR {
            match value {
                false => PWMMR2RR::DISABLED_,
                true => PWMMR2RR::RESET_ON_PWMMR2_THE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR2RR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `RESET_ON_PWMMR2_THE`" ]
        # [ inline ( always ) ]
        pub fn is_reset_on_pwmmr2_the(&self) -> bool {
            *self == PWMMR2RR::RESET_ON_PWMMR2_THE
        }
    }
    # [ doc = "Possible values of the field `PWMMR2S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR2SR {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC." ]
        STOP_ON_PWMMR2_THE_,
    }
    impl PWMMR2SR {
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
                PWMMR2SR::DISABLED => false,
                PWMMR2SR::STOP_ON_PWMMR2_THE_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR2SR {
            match value {
                false => PWMMR2SR::DISABLED,
                true => PWMMR2SR::STOP_ON_PWMMR2_THE_,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == PWMMR2SR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `STOP_ON_PWMMR2_THE_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_on_pwmmr2_the_(&self) -> bool {
            *self == PWMMR2SR::STOP_ON_PWMMR2_THE_
        }
    }
    # [ doc = "Possible values of the field `PWMMR3I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR3IR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR3,
    }
    impl PWMMR3IR {
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
                PWMMR3IR::DISABLED_ => false,
                PWMMR3IR::INTERRUPT_ON_PWMMR3 => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR3IR {
            match value {
                false => PWMMR3IR::DISABLED_,
                true => PWMMR3IR::INTERRUPT_ON_PWMMR3,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR3IR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR3`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_on_pwmmr3(&self) -> bool {
            *self == PWMMR3IR::INTERRUPT_ON_PWMMR3
        }
    }
    # [ doc = "Possible values of the field `PWMMR3R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR3RR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it." ]
        RESET_ON_PWMMR3_THE,
    }
    impl PWMMR3RR {
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
                PWMMR3RR::DISABLED_ => false,
                PWMMR3RR::RESET_ON_PWMMR3_THE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR3RR {
            match value {
                false => PWMMR3RR::DISABLED_,
                true => PWMMR3RR::RESET_ON_PWMMR3_THE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR3RR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `RESET_ON_PWMMR3_THE`" ]
        # [ inline ( always ) ]
        pub fn is_reset_on_pwmmr3_the(&self) -> bool {
            *self == PWMMR3RR::RESET_ON_PWMMR3_THE
        }
    }
    # [ doc = "Possible values of the field `PWMMR3S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR3SR {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC." ]
        STOP_ON_PWMMR3_THE_,
    }
    impl PWMMR3SR {
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
                PWMMR3SR::DISABLED => false,
                PWMMR3SR::STOP_ON_PWMMR3_THE_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR3SR {
            match value {
                false => PWMMR3SR::DISABLED,
                true => PWMMR3SR::STOP_ON_PWMMR3_THE_,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == PWMMR3SR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `STOP_ON_PWMMR3_THE_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_on_pwmmr3_the_(&self) -> bool {
            *self == PWMMR3SR::STOP_ON_PWMMR3_THE_
        }
    }
    # [ doc = "Possible values of the field `PWMMR4I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR4IR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR4,
    }
    impl PWMMR4IR {
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
                PWMMR4IR::DISABLED_ => false,
                PWMMR4IR::INTERRUPT_ON_PWMMR4 => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR4IR {
            match value {
                false => PWMMR4IR::DISABLED_,
                true => PWMMR4IR::INTERRUPT_ON_PWMMR4,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR4IR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR4`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_on_pwmmr4(&self) -> bool {
            *self == PWMMR4IR::INTERRUPT_ON_PWMMR4
        }
    }
    # [ doc = "Possible values of the field `PWMMR4R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR4RR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it." ]
        RESET_ON_PWMMR4_THE,
    }
    impl PWMMR4RR {
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
                PWMMR4RR::DISABLED_ => false,
                PWMMR4RR::RESET_ON_PWMMR4_THE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR4RR {
            match value {
                false => PWMMR4RR::DISABLED_,
                true => PWMMR4RR::RESET_ON_PWMMR4_THE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR4RR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `RESET_ON_PWMMR4_THE`" ]
        # [ inline ( always ) ]
        pub fn is_reset_on_pwmmr4_the(&self) -> bool {
            *self == PWMMR4RR::RESET_ON_PWMMR4_THE
        }
    }
    # [ doc = "Possible values of the field `PWMMR4S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR4SR {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC." ]
        STOP_ON_PWMMR4_THE_,
    }
    impl PWMMR4SR {
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
                PWMMR4SR::DISABLED => false,
                PWMMR4SR::STOP_ON_PWMMR4_THE_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR4SR {
            match value {
                false => PWMMR4SR::DISABLED,
                true => PWMMR4SR::STOP_ON_PWMMR4_THE_,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == PWMMR4SR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `STOP_ON_PWMMR4_THE_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_on_pwmmr4_the_(&self) -> bool {
            *self == PWMMR4SR::STOP_ON_PWMMR4_THE_
        }
    }
    # [ doc = "Possible values of the field `PWMMR5I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR5IR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR5,
    }
    impl PWMMR5IR {
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
                PWMMR5IR::DISABLED_ => false,
                PWMMR5IR::INTERRUPT_ON_PWMMR5 => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR5IR {
            match value {
                false => PWMMR5IR::DISABLED_,
                true => PWMMR5IR::INTERRUPT_ON_PWMMR5,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR5IR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR5`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_on_pwmmr5(&self) -> bool {
            *self == PWMMR5IR::INTERRUPT_ON_PWMMR5
        }
    }
    # [ doc = "Possible values of the field `PWMMR5R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR5RR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it." ]
        RESET_ON_PWMMR5_THE,
    }
    impl PWMMR5RR {
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
                PWMMR5RR::DISABLED_ => false,
                PWMMR5RR::RESET_ON_PWMMR5_THE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR5RR {
            match value {
                false => PWMMR5RR::DISABLED_,
                true => PWMMR5RR::RESET_ON_PWMMR5_THE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR5RR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `RESET_ON_PWMMR5_THE`" ]
        # [ inline ( always ) ]
        pub fn is_reset_on_pwmmr5_the(&self) -> bool {
            *self == PWMMR5RR::RESET_ON_PWMMR5_THE
        }
    }
    # [ doc = "Possible values of the field `PWMMR5S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR5SR {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC." ]
        STOP_ON_PWMMR5_THE_,
    }
    impl PWMMR5SR {
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
                PWMMR5SR::DISABLED => false,
                PWMMR5SR::STOP_ON_PWMMR5_THE_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR5SR {
            match value {
                false => PWMMR5SR::DISABLED,
                true => PWMMR5SR::STOP_ON_PWMMR5_THE_,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == PWMMR5SR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `STOP_ON_PWMMR5_THE_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_on_pwmmr5_the_(&self) -> bool {
            *self == PWMMR5SR::STOP_ON_PWMMR5_THE_
        }
    }
    # [ doc = "Possible values of the field `PWMMR6I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR6IR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR6,
    }
    impl PWMMR6IR {
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
                PWMMR6IR::DISABLED_ => false,
                PWMMR6IR::INTERRUPT_ON_PWMMR6 => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR6IR {
            match value {
                false => PWMMR6IR::DISABLED_,
                true => PWMMR6IR::INTERRUPT_ON_PWMMR6,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR6IR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR6`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_on_pwmmr6(&self) -> bool {
            *self == PWMMR6IR::INTERRUPT_ON_PWMMR6
        }
    }
    # [ doc = "Possible values of the field `PWMMR6R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR6RR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it." ]
        RESET_ON_PWMMR6_THE,
    }
    impl PWMMR6RR {
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
                PWMMR6RR::DISABLED_ => false,
                PWMMR6RR::RESET_ON_PWMMR6_THE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR6RR {
            match value {
                false => PWMMR6RR::DISABLED_,
                true => PWMMR6RR::RESET_ON_PWMMR6_THE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == PWMMR6RR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `RESET_ON_PWMMR6_THE`" ]
        # [ inline ( always ) ]
        pub fn is_reset_on_pwmmr6_the(&self) -> bool {
            *self == PWMMR6RR::RESET_ON_PWMMR6_THE
        }
    }
    # [ doc = "Possible values of the field `PWMMR6S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMMR6SR {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC." ]
        STOP_ON_PWMMR6_THE_,
    }
    impl PWMMR6SR {
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
                PWMMR6SR::DISABLED => false,
                PWMMR6SR::STOP_ON_PWMMR6_THE_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMMR6SR {
            match value {
                false => PWMMR6SR::DISABLED,
                true => PWMMR6SR::STOP_ON_PWMMR6_THE_,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == PWMMR6SR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `STOP_ON_PWMMR6_THE_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_on_pwmmr6_the_(&self) -> bool {
            *self == PWMMR6SR::STOP_ON_PWMMR6_THE_
        }
    }
    # [ doc = "Values that can be written to the field `PWMMR0I`" ]
    pub enum PWMMR0IW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR0,
    }
    impl PWMMR0IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR0IW::DISABLED_ => false,
                PWMMR0IW::INTERRUPT_ON_PWMMR0 => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR0IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR0IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR0IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR0IW::DISABLED_)
        }
        # [ doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC." ]
        # [ inline ( always ) ]
        pub fn interrupt_on_pwmmr0(self) -> &'a mut W {
            self.variant(PWMMR0IW::INTERRUPT_ON_PWMMR0)
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
    # [ doc = "Values that can be written to the field `PWMMR0R`" ]
    pub enum PWMMR0RW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it." ]
        RESET_ON_PWMMR0_THE,
    }
    impl PWMMR0RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR0RW::DISABLED_ => false,
                PWMMR0RW::RESET_ON_PWMMR0_THE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR0RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR0RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR0RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR0RW::DISABLED_)
        }
        # [ doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it." ]
        # [ inline ( always ) ]
        pub fn reset_on_pwmmr0_the(self) -> &'a mut W {
            self.variant(PWMMR0RW::RESET_ON_PWMMR0_THE)
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
    # [ doc = "Values that can be written to the field `PWMMR0S`" ]
    pub enum PWMMR0SW {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC." ]
        STOP_ON_PWMMR0_THE_,
    }
    impl PWMMR0SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR0SW::DISABLED => false,
                PWMMR0SW::STOP_ON_PWMMR0_THE_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR0SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR0SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR0SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(PWMMR0SW::DISABLED)
        }
        # [ doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC." ]
        # [ inline ( always ) ]
        pub fn stop_on_pwmmr0_the_(self) -> &'a mut W {
            self.variant(PWMMR0SW::STOP_ON_PWMMR0_THE_)
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
    # [ doc = "Values that can be written to the field `PWMMR1I`" ]
    pub enum PWMMR1IW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR1,
    }
    impl PWMMR1IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR1IW::DISABLED_ => false,
                PWMMR1IW::INTERRUPT_ON_PWMMR1 => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR1IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR1IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR1IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR1IW::DISABLED_)
        }
        # [ doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC." ]
        # [ inline ( always ) ]
        pub fn interrupt_on_pwmmr1(self) -> &'a mut W {
            self.variant(PWMMR1IW::INTERRUPT_ON_PWMMR1)
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
    # [ doc = "Values that can be written to the field `PWMMR1R`" ]
    pub enum PWMMR1RW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it." ]
        RESET_ON_PWMMR1_THE,
    }
    impl PWMMR1RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR1RW::DISABLED_ => false,
                PWMMR1RW::RESET_ON_PWMMR1_THE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR1RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR1RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR1RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR1RW::DISABLED_)
        }
        # [ doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it." ]
        # [ inline ( always ) ]
        pub fn reset_on_pwmmr1_the(self) -> &'a mut W {
            self.variant(PWMMR1RW::RESET_ON_PWMMR1_THE)
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
    # [ doc = "Values that can be written to the field `PWMMR1S`" ]
    pub enum PWMMR1SW {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC." ]
        STOP_ON_PWMMR1_THE_,
    }
    impl PWMMR1SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR1SW::DISABLED => false,
                PWMMR1SW::STOP_ON_PWMMR1_THE_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR1SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR1SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR1SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(PWMMR1SW::DISABLED)
        }
        # [ doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC." ]
        # [ inline ( always ) ]
        pub fn stop_on_pwmmr1_the_(self) -> &'a mut W {
            self.variant(PWMMR1SW::STOP_ON_PWMMR1_THE_)
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
    # [ doc = "Values that can be written to the field `PWMMR2I`" ]
    pub enum PWMMR2IW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR2,
    }
    impl PWMMR2IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR2IW::DISABLED_ => false,
                PWMMR2IW::INTERRUPT_ON_PWMMR2 => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR2IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR2IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR2IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR2IW::DISABLED_)
        }
        # [ doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC." ]
        # [ inline ( always ) ]
        pub fn interrupt_on_pwmmr2(self) -> &'a mut W {
            self.variant(PWMMR2IW::INTERRUPT_ON_PWMMR2)
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
    # [ doc = "Values that can be written to the field `PWMMR2R`" ]
    pub enum PWMMR2RW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it." ]
        RESET_ON_PWMMR2_THE,
    }
    impl PWMMR2RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR2RW::DISABLED_ => false,
                PWMMR2RW::RESET_ON_PWMMR2_THE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR2RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR2RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR2RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR2RW::DISABLED_)
        }
        # [ doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it." ]
        # [ inline ( always ) ]
        pub fn reset_on_pwmmr2_the(self) -> &'a mut W {
            self.variant(PWMMR2RW::RESET_ON_PWMMR2_THE)
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
    # [ doc = "Values that can be written to the field `PWMMR2S`" ]
    pub enum PWMMR2SW {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC." ]
        STOP_ON_PWMMR2_THE_,
    }
    impl PWMMR2SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR2SW::DISABLED => false,
                PWMMR2SW::STOP_ON_PWMMR2_THE_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR2SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR2SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR2SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(PWMMR2SW::DISABLED)
        }
        # [ doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC." ]
        # [ inline ( always ) ]
        pub fn stop_on_pwmmr2_the_(self) -> &'a mut W {
            self.variant(PWMMR2SW::STOP_ON_PWMMR2_THE_)
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
    # [ doc = "Values that can be written to the field `PWMMR3I`" ]
    pub enum PWMMR3IW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR3,
    }
    impl PWMMR3IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR3IW::DISABLED_ => false,
                PWMMR3IW::INTERRUPT_ON_PWMMR3 => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR3IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR3IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR3IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR3IW::DISABLED_)
        }
        # [ doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC." ]
        # [ inline ( always ) ]
        pub fn interrupt_on_pwmmr3(self) -> &'a mut W {
            self.variant(PWMMR3IW::INTERRUPT_ON_PWMMR3)
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
    # [ doc = "Values that can be written to the field `PWMMR3R`" ]
    pub enum PWMMR3RW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it." ]
        RESET_ON_PWMMR3_THE,
    }
    impl PWMMR3RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR3RW::DISABLED_ => false,
                PWMMR3RW::RESET_ON_PWMMR3_THE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR3RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR3RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR3RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR3RW::DISABLED_)
        }
        # [ doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it." ]
        # [ inline ( always ) ]
        pub fn reset_on_pwmmr3_the(self) -> &'a mut W {
            self.variant(PWMMR3RW::RESET_ON_PWMMR3_THE)
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
    # [ doc = "Values that can be written to the field `PWMMR3S`" ]
    pub enum PWMMR3SW {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC." ]
        STOP_ON_PWMMR3_THE_,
    }
    impl PWMMR3SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR3SW::DISABLED => false,
                PWMMR3SW::STOP_ON_PWMMR3_THE_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR3SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR3SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR3SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(PWMMR3SW::DISABLED)
        }
        # [ doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC." ]
        # [ inline ( always ) ]
        pub fn stop_on_pwmmr3_the_(self) -> &'a mut W {
            self.variant(PWMMR3SW::STOP_ON_PWMMR3_THE_)
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
    # [ doc = "Values that can be written to the field `PWMMR4I`" ]
    pub enum PWMMR4IW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR4,
    }
    impl PWMMR4IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR4IW::DISABLED_ => false,
                PWMMR4IW::INTERRUPT_ON_PWMMR4 => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR4IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR4IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR4IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR4IW::DISABLED_)
        }
        # [ doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC." ]
        # [ inline ( always ) ]
        pub fn interrupt_on_pwmmr4(self) -> &'a mut W {
            self.variant(PWMMR4IW::INTERRUPT_ON_PWMMR4)
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
    # [ doc = "Values that can be written to the field `PWMMR4R`" ]
    pub enum PWMMR4RW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it." ]
        RESET_ON_PWMMR4_THE,
    }
    impl PWMMR4RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR4RW::DISABLED_ => false,
                PWMMR4RW::RESET_ON_PWMMR4_THE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR4RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR4RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR4RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR4RW::DISABLED_)
        }
        # [ doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it." ]
        # [ inline ( always ) ]
        pub fn reset_on_pwmmr4_the(self) -> &'a mut W {
            self.variant(PWMMR4RW::RESET_ON_PWMMR4_THE)
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
    # [ doc = "Values that can be written to the field `PWMMR4S`" ]
    pub enum PWMMR4SW {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC." ]
        STOP_ON_PWMMR4_THE_,
    }
    impl PWMMR4SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR4SW::DISABLED => false,
                PWMMR4SW::STOP_ON_PWMMR4_THE_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR4SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR4SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR4SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(PWMMR4SW::DISABLED)
        }
        # [ doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC." ]
        # [ inline ( always ) ]
        pub fn stop_on_pwmmr4_the_(self) -> &'a mut W {
            self.variant(PWMMR4SW::STOP_ON_PWMMR4_THE_)
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
    # [ doc = "Values that can be written to the field `PWMMR5I`" ]
    pub enum PWMMR5IW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR5,
    }
    impl PWMMR5IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR5IW::DISABLED_ => false,
                PWMMR5IW::INTERRUPT_ON_PWMMR5 => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR5IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR5IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR5IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR5IW::DISABLED_)
        }
        # [ doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC." ]
        # [ inline ( always ) ]
        pub fn interrupt_on_pwmmr5(self) -> &'a mut W {
            self.variant(PWMMR5IW::INTERRUPT_ON_PWMMR5)
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
    # [ doc = "Values that can be written to the field `PWMMR5R`" ]
    pub enum PWMMR5RW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it." ]
        RESET_ON_PWMMR5_THE,
    }
    impl PWMMR5RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR5RW::DISABLED_ => false,
                PWMMR5RW::RESET_ON_PWMMR5_THE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR5RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR5RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR5RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR5RW::DISABLED_)
        }
        # [ doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it." ]
        # [ inline ( always ) ]
        pub fn reset_on_pwmmr5_the(self) -> &'a mut W {
            self.variant(PWMMR5RW::RESET_ON_PWMMR5_THE)
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
    # [ doc = "Values that can be written to the field `PWMMR5S`" ]
    pub enum PWMMR5SW {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC." ]
        STOP_ON_PWMMR5_THE_,
    }
    impl PWMMR5SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR5SW::DISABLED => false,
                PWMMR5SW::STOP_ON_PWMMR5_THE_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR5SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR5SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR5SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(PWMMR5SW::DISABLED)
        }
        # [ doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC." ]
        # [ inline ( always ) ]
        pub fn stop_on_pwmmr5_the_(self) -> &'a mut W {
            self.variant(PWMMR5SW::STOP_ON_PWMMR5_THE_)
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
    # [ doc = "Values that can be written to the field `PWMMR6I`" ]
    pub enum PWMMR6IW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC." ]
        INTERRUPT_ON_PWMMR6,
    }
    impl PWMMR6IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR6IW::DISABLED_ => false,
                PWMMR6IW::INTERRUPT_ON_PWMMR6 => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR6IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR6IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR6IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR6IW::DISABLED_)
        }
        # [ doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC." ]
        # [ inline ( always ) ]
        pub fn interrupt_on_pwmmr6(self) -> &'a mut W {
            self.variant(PWMMR6IW::INTERRUPT_ON_PWMMR6)
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
    # [ doc = "Values that can be written to the field `PWMMR6R`" ]
    pub enum PWMMR6RW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it." ]
        RESET_ON_PWMMR6_THE,
    }
    impl PWMMR6RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR6RW::DISABLED_ => false,
                PWMMR6RW::RESET_ON_PWMMR6_THE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR6RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR6RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR6RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(PWMMR6RW::DISABLED_)
        }
        # [ doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it." ]
        # [ inline ( always ) ]
        pub fn reset_on_pwmmr6_the(self) -> &'a mut W {
            self.variant(PWMMR6RW::RESET_ON_PWMMR6_THE)
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
    # [ doc = "Values that can be written to the field `PWMMR6S`" ]
    pub enum PWMMR6SW {
        # [ doc = "Disabled" ]
        DISABLED,
        # [ doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC." ]
        STOP_ON_PWMMR6_THE_,
    }
    impl PWMMR6SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMMR6SW::DISABLED => false,
                PWMMR6SW::STOP_ON_PWMMR6_THE_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMMR6SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMMR6SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMMR6SW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled" ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(PWMMR6SW::DISABLED)
        }
        # [ doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC." ]
        # [ inline ( always ) ]
        pub fn stop_on_pwmmr6_the_(self) -> &'a mut W {
            self.variant(PWMMR6SW::STOP_ON_PWMMR6_THE_)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Interrupt PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr0i(&self) -> PWMMR0IR {
            PWMMR0IR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 0;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 1 - Reset PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr0r(&self) -> PWMMR0RR {
            PWMMR0RR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 1;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 2 - Stop PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr0s(&self) -> PWMMR0SR {
            PWMMR0SR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 2;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 3 - Interrupt PWM1" ]
        # [ inline ( always ) ]
        pub fn pwmmr1i(&self) -> PWMMR1IR {
            PWMMR1IR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 3;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 4 - Reset PWM1" ]
        # [ inline ( always ) ]
        pub fn pwmmr1r(&self) -> PWMMR1RR {
            PWMMR1RR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 4;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 5 - Stop PWM1" ]
        # [ inline ( always ) ]
        pub fn pwmmr1s(&self) -> PWMMR1SR {
            PWMMR1SR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 5;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 6 - Interrupt PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr2i(&self) -> PWMMR2IR {
            PWMMR2IR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 6;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 7 - Reset PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr2r(&self) -> PWMMR2RR {
            PWMMR2RR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 7;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 8 - Stop PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr2s(&self) -> PWMMR2SR {
            PWMMR2SR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 8;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 9 - Interrupt PWM3" ]
        # [ inline ( always ) ]
        pub fn pwmmr3i(&self) -> PWMMR3IR {
            PWMMR3IR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 9;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 10 - Reset PWM3" ]
        # [ inline ( always ) ]
        pub fn pwmmr3r(&self) -> PWMMR3RR {
            PWMMR3RR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 10;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 11 - Stop PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr3s(&self) -> PWMMR3SR {
            PWMMR3SR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 11;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 12 - Interrupt PWM4" ]
        # [ inline ( always ) ]
        pub fn pwmmr4i(&self) -> PWMMR4IR {
            PWMMR4IR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 12;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 13 - Reset PWM4" ]
        # [ inline ( always ) ]
        pub fn pwmmr4r(&self) -> PWMMR4RR {
            PWMMR4RR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 13;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 14 - Stop PWM4" ]
        # [ inline ( always ) ]
        pub fn pwmmr4s(&self) -> PWMMR4SR {
            PWMMR4SR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 14;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 15 - Interrupt PWM5" ]
        # [ inline ( always ) ]
        pub fn pwmmr5i(&self) -> PWMMR5IR {
            PWMMR5IR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 15;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 16 - Reset PWM5" ]
        # [ inline ( always ) ]
        pub fn pwmmr5r(&self) -> PWMMR5RR {
            PWMMR5RR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 16;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 17 - Stop PWM5" ]
        # [ inline ( always ) ]
        pub fn pwmmr5s(&self) -> PWMMR5SR {
            PWMMR5SR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 17;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 18 - Interrupt PWM6" ]
        # [ inline ( always ) ]
        pub fn pwmmr6i(&self) -> PWMMR6IR {
            PWMMR6IR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 18;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 19 - Reset PWM6" ]
        # [ inline ( always ) ]
        pub fn pwmmr6r(&self) -> PWMMR6RR {
            PWMMR6RR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 19;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 20 - Stop PWM6" ]
        # [ inline ( always ) ]
        pub fn pwmmr6s(&self) -> PWMMR6SR {
            PWMMR6SR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 20;
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
        # [ doc = "Bit 0 - Interrupt PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr0i(&mut self) -> _PWMMR0IW {
            _PWMMR0IW { w: self }
        }
        # [ doc = "Bit 1 - Reset PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr0r(&mut self) -> _PWMMR0RW {
            _PWMMR0RW { w: self }
        }
        # [ doc = "Bit 2 - Stop PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr0s(&mut self) -> _PWMMR0SW {
            _PWMMR0SW { w: self }
        }
        # [ doc = "Bit 3 - Interrupt PWM1" ]
        # [ inline ( always ) ]
        pub fn pwmmr1i(&mut self) -> _PWMMR1IW {
            _PWMMR1IW { w: self }
        }
        # [ doc = "Bit 4 - Reset PWM1" ]
        # [ inline ( always ) ]
        pub fn pwmmr1r(&mut self) -> _PWMMR1RW {
            _PWMMR1RW { w: self }
        }
        # [ doc = "Bit 5 - Stop PWM1" ]
        # [ inline ( always ) ]
        pub fn pwmmr1s(&mut self) -> _PWMMR1SW {
            _PWMMR1SW { w: self }
        }
        # [ doc = "Bit 6 - Interrupt PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr2i(&mut self) -> _PWMMR2IW {
            _PWMMR2IW { w: self }
        }
        # [ doc = "Bit 7 - Reset PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr2r(&mut self) -> _PWMMR2RW {
            _PWMMR2RW { w: self }
        }
        # [ doc = "Bit 8 - Stop PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr2s(&mut self) -> _PWMMR2SW {
            _PWMMR2SW { w: self }
        }
        # [ doc = "Bit 9 - Interrupt PWM3" ]
        # [ inline ( always ) ]
        pub fn pwmmr3i(&mut self) -> _PWMMR3IW {
            _PWMMR3IW { w: self }
        }
        # [ doc = "Bit 10 - Reset PWM3" ]
        # [ inline ( always ) ]
        pub fn pwmmr3r(&mut self) -> _PWMMR3RW {
            _PWMMR3RW { w: self }
        }
        # [ doc = "Bit 11 - Stop PWM0" ]
        # [ inline ( always ) ]
        pub fn pwmmr3s(&mut self) -> _PWMMR3SW {
            _PWMMR3SW { w: self }
        }
        # [ doc = "Bit 12 - Interrupt PWM4" ]
        # [ inline ( always ) ]
        pub fn pwmmr4i(&mut self) -> _PWMMR4IW {
            _PWMMR4IW { w: self }
        }
        # [ doc = "Bit 13 - Reset PWM4" ]
        # [ inline ( always ) ]
        pub fn pwmmr4r(&mut self) -> _PWMMR4RW {
            _PWMMR4RW { w: self }
        }
        # [ doc = "Bit 14 - Stop PWM4" ]
        # [ inline ( always ) ]
        pub fn pwmmr4s(&mut self) -> _PWMMR4SW {
            _PWMMR4SW { w: self }
        }
        # [ doc = "Bit 15 - Interrupt PWM5" ]
        # [ inline ( always ) ]
        pub fn pwmmr5i(&mut self) -> _PWMMR5IW {
            _PWMMR5IW { w: self }
        }
        # [ doc = "Bit 16 - Reset PWM5" ]
        # [ inline ( always ) ]
        pub fn pwmmr5r(&mut self) -> _PWMMR5RW {
            _PWMMR5RW { w: self }
        }
        # [ doc = "Bit 17 - Stop PWM5" ]
        # [ inline ( always ) ]
        pub fn pwmmr5s(&mut self) -> _PWMMR5SW {
            _PWMMR5SW { w: self }
        }
        # [ doc = "Bit 18 - Interrupt PWM6" ]
        # [ inline ( always ) ]
        pub fn pwmmr6i(&mut self) -> _PWMMR6IW {
            _PWMMR6IW { w: self }
        }
        # [ doc = "Bit 19 - Reset PWM6" ]
        # [ inline ( always ) ]
        pub fn pwmmr6r(&mut self) -> _PWMMR6RW {
            _PWMMR6RW { w: self }
        }
        # [ doc = "Bit 20 - Stop PWM6" ]
        # [ inline ( always ) ]
        pub fn pwmmr6s(&mut self) -> _PWMMR6SW {
            _PWMMR6SW { w: self }
        }
    }
}
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
pub struct MR0 {
    register: VolatileCell<u32>,
}
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
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
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
pub struct MR1 {
    register: VolatileCell<u32>,
}
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
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
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
pub struct MR2 {
    register: VolatileCell<u32>,
}
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
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
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
pub struct MR3 {
    register: VolatileCell<u32>,
}
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
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
# [ doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event." ]
pub struct CCR {
    register: VolatileCell<u32>,
}
# [ doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event." ]
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
    # [ doc = "Possible values of the field `CAP0_R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP0_RR {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC." ]
        RISING_EDGE_A_SYNCH,
    }
    impl CAP0_RR {
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
                CAP0_RR::DISABLED_THIS_FEATU => false,
                CAP0_RR::RISING_EDGE_A_SYNCH => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP0_RR {
            match value {
                false => CAP0_RR::DISABLED_THIS_FEATU,
                true => CAP0_RR::RISING_EDGE_A_SYNCH,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_this_featu(&self) -> bool {
            *self == CAP0_RR::DISABLED_THIS_FEATU
        }
        # [ doc = "Checks if the value of the field is `RISING_EDGE_A_SYNCH`" ]
        # [ inline ( always ) ]
        pub fn is_rising_edge_a_synch(&self) -> bool {
            *self == CAP0_RR::RISING_EDGE_A_SYNCH
        }
    }
    # [ doc = "Possible values of the field `CAP0_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP0_FR {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC." ]
        FALLING_EDGE_A_SYNC,
    }
    impl CAP0_FR {
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
                CAP0_FR::DISABLED_THIS_FEATU => false,
                CAP0_FR::FALLING_EDGE_A_SYNC => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP0_FR {
            match value {
                false => CAP0_FR::DISABLED_THIS_FEATU,
                true => CAP0_FR::FALLING_EDGE_A_SYNC,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_this_featu(&self) -> bool {
            *self == CAP0_FR::DISABLED_THIS_FEATU
        }
        # [ doc = "Checks if the value of the field is `FALLING_EDGE_A_SYNC`" ]
        # [ inline ( always ) ]
        pub fn is_falling_edge_a_sync(&self) -> bool {
            *self == CAP0_FR::FALLING_EDGE_A_SYNC
        }
    }
    # [ doc = "Possible values of the field `CAP0_I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP0_IR {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt." ]
        INTERRUPT_A_CR0_LOA,
    }
    impl CAP0_IR {
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
                CAP0_IR::DISABLED_THIS_FEATU => false,
                CAP0_IR::INTERRUPT_A_CR0_LOA => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP0_IR {
            match value {
                false => CAP0_IR::DISABLED_THIS_FEATU,
                true => CAP0_IR::INTERRUPT_A_CR0_LOA,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_this_featu(&self) -> bool {
            *self == CAP0_IR::DISABLED_THIS_FEATU
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_A_CR0_LOA`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_a_cr0_loa(&self) -> bool {
            *self == CAP0_IR::INTERRUPT_A_CR0_LOA
        }
    }
    # [ doc = "Possible values of the field `CAP1_R`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP1_RR {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC." ]
        RISING_EDGE_A_SYNCH,
    }
    impl CAP1_RR {
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
                CAP1_RR::DISABLED_THIS_FEATU => false,
                CAP1_RR::RISING_EDGE_A_SYNCH => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP1_RR {
            match value {
                false => CAP1_RR::DISABLED_THIS_FEATU,
                true => CAP1_RR::RISING_EDGE_A_SYNCH,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_this_featu(&self) -> bool {
            *self == CAP1_RR::DISABLED_THIS_FEATU
        }
        # [ doc = "Checks if the value of the field is `RISING_EDGE_A_SYNCH`" ]
        # [ inline ( always ) ]
        pub fn is_rising_edge_a_synch(&self) -> bool {
            *self == CAP1_RR::RISING_EDGE_A_SYNCH
        }
    }
    # [ doc = "Possible values of the field `CAP1_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP1_FR {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC." ]
        FALLING_EDGE_A_SYNC,
    }
    impl CAP1_FR {
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
                CAP1_FR::DISABLED_THIS_FEATU => false,
                CAP1_FR::FALLING_EDGE_A_SYNC => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP1_FR {
            match value {
                false => CAP1_FR::DISABLED_THIS_FEATU,
                true => CAP1_FR::FALLING_EDGE_A_SYNC,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_this_featu(&self) -> bool {
            *self == CAP1_FR::DISABLED_THIS_FEATU
        }
        # [ doc = "Checks if the value of the field is `FALLING_EDGE_A_SYNC`" ]
        # [ inline ( always ) ]
        pub fn is_falling_edge_a_sync(&self) -> bool {
            *self == CAP1_FR::FALLING_EDGE_A_SYNC
        }
    }
    # [ doc = "Possible values of the field `CAP1_I`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CAP1_IR {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt." ]
        INTERRUPT_A_CR1_LOA,
    }
    impl CAP1_IR {
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
                CAP1_IR::DISABLED_THIS_FEATU => false,
                CAP1_IR::INTERRUPT_A_CR1_LOA => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CAP1_IR {
            match value {
                false => CAP1_IR::DISABLED_THIS_FEATU,
                true => CAP1_IR::INTERRUPT_A_CR1_LOA,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_this_featu(&self) -> bool {
            *self == CAP1_IR::DISABLED_THIS_FEATU
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_A_CR1_LOA`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_a_cr1_loa(&self) -> bool {
            *self == CAP1_IR::INTERRUPT_A_CR1_LOA
        }
    }
    # [ doc = "Values that can be written to the field `CAP0_R`" ]
    pub enum CAP0_RW {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC." ]
        RISING_EDGE_A_SYNCH,
    }
    impl CAP0_RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP0_RW::DISABLED_THIS_FEATU => false,
                CAP0_RW::RISING_EDGE_A_SYNCH => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP0_RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0_RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP0_RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled. This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_this_featu(self) -> &'a mut W {
            self.variant(CAP0_RW::DISABLED_THIS_FEATU)
        }
        # [ doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC." ]
        # [ inline ( always ) ]
        pub fn rising_edge_a_synch(self) -> &'a mut W {
            self.variant(CAP0_RW::RISING_EDGE_A_SYNCH)
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
    # [ doc = "Values that can be written to the field `CAP0_F`" ]
    pub enum CAP0_FW {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC." ]
        FALLING_EDGE_A_SYNC,
    }
    impl CAP0_FW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP0_FW::DISABLED_THIS_FEATU => false,
                CAP0_FW::FALLING_EDGE_A_SYNC => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP0_FW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0_FW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP0_FW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled. This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_this_featu(self) -> &'a mut W {
            self.variant(CAP0_FW::DISABLED_THIS_FEATU)
        }
        # [ doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC." ]
        # [ inline ( always ) ]
        pub fn falling_edge_a_sync(self) -> &'a mut W {
            self.variant(CAP0_FW::FALLING_EDGE_A_SYNC)
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
    # [ doc = "Values that can be written to the field `CAP0_I`" ]
    pub enum CAP0_IW {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt." ]
        INTERRUPT_A_CR0_LOA,
    }
    impl CAP0_IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP0_IW::DISABLED_THIS_FEATU => false,
                CAP0_IW::INTERRUPT_A_CR0_LOA => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP0_IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0_IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP0_IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled. This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_this_featu(self) -> &'a mut W {
            self.variant(CAP0_IW::DISABLED_THIS_FEATU)
        }
        # [ doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn interrupt_a_cr0_loa(self) -> &'a mut W {
            self.variant(CAP0_IW::INTERRUPT_A_CR0_LOA)
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
    # [ doc = "Values that can be written to the field `CAP1_R`" ]
    pub enum CAP1_RW {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC." ]
        RISING_EDGE_A_SYNCH,
    }
    impl CAP1_RW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP1_RW::DISABLED_THIS_FEATU => false,
                CAP1_RW::RISING_EDGE_A_SYNCH => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP1_RW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1_RW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP1_RW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled. This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_this_featu(self) -> &'a mut W {
            self.variant(CAP1_RW::DISABLED_THIS_FEATU)
        }
        # [ doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC." ]
        # [ inline ( always ) ]
        pub fn rising_edge_a_synch(self) -> &'a mut W {
            self.variant(CAP1_RW::RISING_EDGE_A_SYNCH)
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
    # [ doc = "Values that can be written to the field `CAP1_F`" ]
    pub enum CAP1_FW {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC." ]
        FALLING_EDGE_A_SYNC,
    }
    impl CAP1_FW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP1_FW::DISABLED_THIS_FEATU => false,
                CAP1_FW::FALLING_EDGE_A_SYNC => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP1_FW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1_FW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP1_FW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled. This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_this_featu(self) -> &'a mut W {
            self.variant(CAP1_FW::DISABLED_THIS_FEATU)
        }
        # [ doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC." ]
        # [ inline ( always ) ]
        pub fn falling_edge_a_sync(self) -> &'a mut W {
            self.variant(CAP1_FW::FALLING_EDGE_A_SYNC)
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
    # [ doc = "Values that can be written to the field `CAP1_I`" ]
    pub enum CAP1_IW {
        # [ doc = "Disabled. This feature is disabled." ]
        DISABLED_THIS_FEATU,
        # [ doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt." ]
        INTERRUPT_A_CR1_LOA,
    }
    impl CAP1_IW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CAP1_IW::DISABLED_THIS_FEATU => false,
                CAP1_IW::INTERRUPT_A_CR1_LOA => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CAP1_IW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1_IW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CAP1_IW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled. This feature is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_this_featu(self) -> &'a mut W {
            self.variant(CAP1_IW::DISABLED_THIS_FEATU)
        }
        # [ doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn interrupt_a_cr1_loa(self) -> &'a mut W {
            self.variant(CAP1_IW::INTERRUPT_A_CR1_LOA)
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
        # [ doc = "Bit 0 - Capture on PWMn_CAP0 rising edge" ]
        # [ inline ( always ) ]
        pub fn cap0_r(&self) -> CAP0_RR {
            CAP0_RR::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 0;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 1 - Capture on PWMn_CAP0 falling edge" ]
        # [ inline ( always ) ]
        pub fn cap0_f(&self) -> CAP0_FR {
            CAP0_FR::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 1;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 2 - Interrupt on PWMn_CAP0 event" ]
        # [ inline ( always ) ]
        pub fn cap0_i(&self) -> CAP0_IR {
            CAP0_IR::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 2;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0." ]
        # [ inline ( always ) ]
        pub fn cap1_r(&self) -> CAP1_RR {
            CAP1_RR::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 3;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0." ]
        # [ inline ( always ) ]
        pub fn cap1_f(&self) -> CAP1_FR {
            CAP1_FR::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 4;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0." ]
        # [ inline ( always ) ]
        pub fn cap1_i(&self) -> CAP1_IR {
            CAP1_IR::_from({
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
        # [ doc = "Bit 0 - Capture on PWMn_CAP0 rising edge" ]
        # [ inline ( always ) ]
        pub fn cap0_r(&mut self) -> _CAP0_RW {
            _CAP0_RW { w: self }
        }
        # [ doc = "Bit 1 - Capture on PWMn_CAP0 falling edge" ]
        # [ inline ( always ) ]
        pub fn cap0_f(&mut self) -> _CAP0_FW {
            _CAP0_FW { w: self }
        }
        # [ doc = "Bit 2 - Interrupt on PWMn_CAP0 event" ]
        # [ inline ( always ) ]
        pub fn cap0_i(&mut self) -> _CAP0_IW {
            _CAP0_IW { w: self }
        }
        # [ doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0." ]
        # [ inline ( always ) ]
        pub fn cap1_r(&mut self) -> _CAP1_RW {
            _CAP1_RW { w: self }
        }
        # [ doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0." ]
        # [ inline ( always ) ]
        pub fn cap1_f(&mut self) -> _CAP1_FW {
            _CAP1_FW { w: self }
        }
        # [ doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0." ]
        # [ inline ( always ) ]
        pub fn cap1_i(&mut self) -> _CAP1_IW {
            _CAP1_IW { w: self }
        }
    }
}
# [ doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs." ]
pub struct CR {
    register: VolatileCell<u32>,
}
# [ doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs." ]
pub mod cr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CR {
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
    # [ doc = "Possible values of the field `PWMSEL2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL2R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL2R {
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
                PWMSEL2R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL2R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL2R {
            match value {
                false => PWMSEL2R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL2R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL2R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL2R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMSEL3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL3R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL3R {
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
                PWMSEL3R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL3R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL3R {
            match value {
                false => PWMSEL3R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL3R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL3R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL3R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMSEL4`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL4R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL4R {
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
                PWMSEL4R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL4R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL4R {
            match value {
                false => PWMSEL4R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL4R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL4R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL4R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMSEL5`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL5R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL5R {
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
                PWMSEL5R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL5R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL5R {
            match value {
                false => PWMSEL5R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL5R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL5R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL5R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMSEL6`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL6R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL6R {
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
                PWMSEL6R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL6R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL6R {
            match value {
                false => PWMSEL6R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL6R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL6R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL6R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMENA1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA1R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA1R {
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
                PWMENA1R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA1R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA1R {
            match value {
                false => PWMENA1R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA1R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA1R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA1R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA2R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA2R {
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
                PWMENA2R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA2R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA2R {
            match value {
                false => PWMENA2R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA2R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA2R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA2R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA3R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA3R {
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
                PWMENA3R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA3R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA3R {
            match value {
                false => PWMENA3R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA3R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA3R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA3R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA4`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA4R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA4R {
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
                PWMENA4R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA4R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA4R {
            match value {
                false => PWMENA4R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA4R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA4R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA4R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA5`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA5R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA5R {
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
                PWMENA5R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA5R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA5R {
            match value {
                false => PWMENA5R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA5R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA5R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA5R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA6`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA6R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA6R {
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
                PWMENA6R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA6R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA6R {
            match value {
                false => PWMENA6R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA6R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA6R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA6R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Values that can be written to the field `PWMSEL2`" ]
    pub enum PWMSEL2W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL2W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL2W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL2W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL2W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMSEL3`" ]
    pub enum PWMSEL3W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL3W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL3W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL3W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL3W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMSEL4`" ]
    pub enum PWMSEL4W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL4W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL4W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL4W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL4W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL4W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL4W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMSEL5`" ]
    pub enum PWMSEL5W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL5W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL5W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL5W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL5W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL5W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL5W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMSEL6`" ]
    pub enum PWMSEL6W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL6W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL6W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL6W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL6W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL6W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL6W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMENA1`" ]
    pub enum PWMENA1W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA1W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA1W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA1W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA1W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA2`" ]
    pub enum PWMENA2W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA2W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA2W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA2W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA2W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA3`" ]
    pub enum PWMENA3W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA3W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA3W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA3W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA3W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA4`" ]
    pub enum PWMENA4W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA4W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA4W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA4W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA4W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA4W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA4W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA5`" ]
    pub enum PWMENA5W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA5W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA5W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA5W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA5W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA5W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA5W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA6`" ]
    pub enum PWMENA6W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA6W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA6W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA6W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA6W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA6W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA6W::THE_PWM_OUTPUT_IS_EN)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 2 - PWM[2] output single/double edge mode control." ]
        # [ inline ( always ) ]
        pub fn pwmsel2(&self) -> PWMSEL2R {
            PWMSEL2R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 2;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 3 - PWM[3] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel3(&self) -> PWMSEL3R {
            PWMSEL3R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 3;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 4 - PWM[4] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel4(&self) -> PWMSEL4R {
            PWMSEL4R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 4;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 5 - PWM[5] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel5(&self) -> PWMSEL5R {
            PWMSEL5R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 5;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 6 - PWM[6] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel6(&self) -> PWMSEL6R {
            PWMSEL6R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 6;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 9 - PWM[1] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena1(&self) -> PWMENA1R {
            PWMENA1R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 9;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 10 - PWM[2] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena2(&self) -> PWMENA2R {
            PWMENA2R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 10;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 11 - PWM[3] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena3(&self) -> PWMENA3R {
            PWMENA3R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 11;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 12 - PWM[4] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena4(&self) -> PWMENA4R {
            PWMENA4R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 12;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 13 - PWM[5] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena5(&self) -> PWMENA5R {
            PWMENA5R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 13;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 14 - PWM[6] output enable control. See PWMENA1 for details." ]
        # [ inline ( always ) ]
        pub fn pwmena6(&self) -> PWMENA6R {
            PWMENA6R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 14;
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
        # [ doc = "Bit 2 - PWM[2] output single/double edge mode control." ]
        # [ inline ( always ) ]
        pub fn pwmsel2(&mut self) -> _PWMSEL2W {
            _PWMSEL2W { w: self }
        }
        # [ doc = "Bit 3 - PWM[3] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel3(&mut self) -> _PWMSEL3W {
            _PWMSEL3W { w: self }
        }
        # [ doc = "Bit 4 - PWM[4] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel4(&mut self) -> _PWMSEL4W {
            _PWMSEL4W { w: self }
        }
        # [ doc = "Bit 5 - PWM[5] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel5(&mut self) -> _PWMSEL5W {
            _PWMSEL5W { w: self }
        }
        # [ doc = "Bit 6 - PWM[6] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel6(&mut self) -> _PWMSEL6W {
            _PWMSEL6W { w: self }
        }
        # [ doc = "Bit 9 - PWM[1] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena1(&mut self) -> _PWMENA1W {
            _PWMENA1W { w: self }
        }
        # [ doc = "Bit 10 - PWM[2] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena2(&mut self) -> _PWMENA2W {
            _PWMENA2W { w: self }
        }
        # [ doc = "Bit 11 - PWM[3] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena3(&mut self) -> _PWMENA3W {
            _PWMENA3W { w: self }
        }
        # [ doc = "Bit 12 - PWM[4] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena4(&mut self) -> _PWMENA4W {
            _PWMENA4W { w: self }
        }
        # [ doc = "Bit 13 - PWM[5] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena5(&mut self) -> _PWMENA5W {
            _PWMENA5W { w: self }
        }
        # [ doc = "Bit 14 - PWM[6] output enable control. See PWMENA1 for details." ]
        # [ inline ( always ) ]
        pub fn pwmena6(&mut self) -> _PWMENA6W {
            _PWMENA6W { w: self }
        }
    }
}
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
pub struct MR {
    register: VolatileCell<u32>,
}
# [ doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges." ]
pub mod mr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MR {
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
# [ doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs." ]
pub struct PCR {
    register: VolatileCell<u32>,
}
# [ doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs." ]
pub mod pcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PCR {
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
    # [ doc = "Possible values of the field `PWMSEL2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL2R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL2R {
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
                PWMSEL2R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL2R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL2R {
            match value {
                false => PWMSEL2R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL2R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL2R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL2R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMSEL3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL3R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL3R {
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
                PWMSEL3R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL3R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL3R {
            match value {
                false => PWMSEL3R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL3R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL3R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL3R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMSEL4`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL4R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL4R {
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
                PWMSEL4R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL4R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL4R {
            match value {
                false => PWMSEL4R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL4R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL4R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL4R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMSEL5`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL5R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL5R {
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
                PWMSEL5R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL5R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL5R {
            match value {
                false => PWMSEL5R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL5R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL5R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL5R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMSEL6`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMSEL6R {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL6R {
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
                PWMSEL6R::SINGLE_EDGE_CONTROLL => false,
                PWMSEL6R::DOUBLE_EDGE_CONTROLL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMSEL6R {
            match value {
                false => PWMSEL6R::SINGLE_EDGE_CONTROLL,
                true => PWMSEL6R::DOUBLE_EDGE_CONTROLL,
            }
        }
        # [ doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_single_edge_controll(&self) -> bool {
            *self == PWMSEL6R::SINGLE_EDGE_CONTROLL
        }
        # [ doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`" ]
        # [ inline ( always ) ]
        pub fn is_double_edge_controll(&self) -> bool {
            *self == PWMSEL6R::DOUBLE_EDGE_CONTROLL
        }
    }
    # [ doc = "Possible values of the field `PWMENA1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA1R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA1R {
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
                PWMENA1R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA1R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA1R {
            match value {
                false => PWMENA1R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA1R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA1R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA1R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA2R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA2R {
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
                PWMENA2R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA2R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA2R {
            match value {
                false => PWMENA2R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA2R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA2R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA2R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA3R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA3R {
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
                PWMENA3R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA3R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA3R {
            match value {
                false => PWMENA3R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA3R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA3R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA3R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA4`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA4R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA4R {
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
                PWMENA4R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA4R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA4R {
            match value {
                false => PWMENA4R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA4R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA4R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA4R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA5`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA5R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA5R {
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
                PWMENA5R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA5R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA5R {
            match value {
                false => PWMENA5R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA5R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA5R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA5R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Possible values of the field `PWMENA6`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PWMENA6R {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA6R {
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
                PWMENA6R::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA6R::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PWMENA6R {
            match value {
                false => PWMENA6R::THE_PWM_OUTPUT_IS_DI,
                true => PWMENA6R::THE_PWM_OUTPUT_IS_EN,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_di(&self) -> bool {
            *self == PWMENA6R::THE_PWM_OUTPUT_IS_DI
        }
        # [ doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`" ]
        # [ inline ( always ) ]
        pub fn is_the_pwm_output_is_en(&self) -> bool {
            *self == PWMENA6R::THE_PWM_OUTPUT_IS_EN
        }
    }
    # [ doc = "Values that can be written to the field `PWMSEL2`" ]
    pub enum PWMSEL2W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL2W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL2W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL2W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL2W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMSEL3`" ]
    pub enum PWMSEL3W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL3W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL3W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL3W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL3W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMSEL4`" ]
    pub enum PWMSEL4W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL4W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL4W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL4W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL4W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL4W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL4W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMSEL5`" ]
    pub enum PWMSEL5W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL5W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL5W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL5W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL5W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL5W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL5W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMSEL6`" ]
    pub enum PWMSEL6W {
        # [ doc = "Single edge controlled mode is selected." ]
        SINGLE_EDGE_CONTROLL,
        # [ doc = "Double edge controlled mode is selected." ]
        DOUBLE_EDGE_CONTROLL,
    }
    impl PWMSEL6W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMSEL6W::SINGLE_EDGE_CONTROLL => false,
                PWMSEL6W::DOUBLE_EDGE_CONTROLL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMSEL6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMSEL6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMSEL6W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Single edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn single_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL6W::SINGLE_EDGE_CONTROLL)
        }
        # [ doc = "Double edge controlled mode is selected." ]
        # [ inline ( always ) ]
        pub fn double_edge_controll(self) -> &'a mut W {
            self.variant(PWMSEL6W::DOUBLE_EDGE_CONTROLL)
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
    # [ doc = "Values that can be written to the field `PWMENA1`" ]
    pub enum PWMENA1W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA1W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA1W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA1W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA1W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA2`" ]
    pub enum PWMENA2W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA2W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA2W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA2W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA2W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA3`" ]
    pub enum PWMENA3W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA3W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA3W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA3W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA3W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA4`" ]
    pub enum PWMENA4W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA4W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA4W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA4W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA4W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA4W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA4W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA5`" ]
    pub enum PWMENA5W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA5W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA5W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA5W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA5W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA5W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA5W::THE_PWM_OUTPUT_IS_EN)
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
    # [ doc = "Values that can be written to the field `PWMENA6`" ]
    pub enum PWMENA6W {
        # [ doc = "The PWM output is disabled." ]
        THE_PWM_OUTPUT_IS_DI,
        # [ doc = "The PWM output is enabled." ]
        THE_PWM_OUTPUT_IS_EN,
    }
    impl PWMENA6W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PWMENA6W::THE_PWM_OUTPUT_IS_DI => false,
                PWMENA6W::THE_PWM_OUTPUT_IS_EN => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PWMENA6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PWMENA6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PWMENA6W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The PWM output is disabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_di(self) -> &'a mut W {
            self.variant(PWMENA6W::THE_PWM_OUTPUT_IS_DI)
        }
        # [ doc = "The PWM output is enabled." ]
        # [ inline ( always ) ]
        pub fn the_pwm_output_is_en(self) -> &'a mut W {
            self.variant(PWMENA6W::THE_PWM_OUTPUT_IS_EN)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 2 - PWM[2] output single/double edge mode control." ]
        # [ inline ( always ) ]
        pub fn pwmsel2(&self) -> PWMSEL2R {
            PWMSEL2R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 2;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 3 - PWM[3] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel3(&self) -> PWMSEL3R {
            PWMSEL3R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 3;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 4 - PWM[4] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel4(&self) -> PWMSEL4R {
            PWMSEL4R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 4;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 5 - PWM[5] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel5(&self) -> PWMSEL5R {
            PWMSEL5R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 5;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 6 - PWM[6] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel6(&self) -> PWMSEL6R {
            PWMSEL6R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 6;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 9 - PWM[1] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena1(&self) -> PWMENA1R {
            PWMENA1R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 9;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 10 - PWM[2] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena2(&self) -> PWMENA2R {
            PWMENA2R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 10;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 11 - PWM[3] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena3(&self) -> PWMENA3R {
            PWMENA3R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 11;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 12 - PWM[4] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena4(&self) -> PWMENA4R {
            PWMENA4R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 12;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 13 - PWM[5] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena5(&self) -> PWMENA5R {
            PWMENA5R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 13;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 14 - PWM[6] output enable control. See PWMENA1 for details." ]
        # [ inline ( always ) ]
        pub fn pwmena6(&self) -> PWMENA6R {
            PWMENA6R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 14;
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
        # [ doc = "Bit 2 - PWM[2] output single/double edge mode control." ]
        # [ inline ( always ) ]
        pub fn pwmsel2(&mut self) -> _PWMSEL2W {
            _PWMSEL2W { w: self }
        }
        # [ doc = "Bit 3 - PWM[3] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel3(&mut self) -> _PWMSEL3W {
            _PWMSEL3W { w: self }
        }
        # [ doc = "Bit 4 - PWM[4] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel4(&mut self) -> _PWMSEL4W {
            _PWMSEL4W { w: self }
        }
        # [ doc = "Bit 5 - PWM[5] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel5(&mut self) -> _PWMSEL5W {
            _PWMSEL5W { w: self }
        }
        # [ doc = "Bit 6 - PWM[6] output edge control." ]
        # [ inline ( always ) ]
        pub fn pwmsel6(&mut self) -> _PWMSEL6W {
            _PWMSEL6W { w: self }
        }
        # [ doc = "Bit 9 - PWM[1] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena1(&mut self) -> _PWMENA1W {
            _PWMENA1W { w: self }
        }
        # [ doc = "Bit 10 - PWM[2] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena2(&mut self) -> _PWMENA2W {
            _PWMENA2W { w: self }
        }
        # [ doc = "Bit 11 - PWM[3] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena3(&mut self) -> _PWMENA3W {
            _PWMENA3W { w: self }
        }
        # [ doc = "Bit 12 - PWM[4] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena4(&mut self) -> _PWMENA4W {
            _PWMENA4W { w: self }
        }
        # [ doc = "Bit 13 - PWM[5] output enable control." ]
        # [ inline ( always ) ]
        pub fn pwmena5(&mut self) -> _PWMENA5W {
            _PWMENA5W { w: self }
        }
        # [ doc = "Bit 14 - PWM[6] output enable control. See PWMENA1 for details." ]
        # [ inline ( always ) ]
        pub fn pwmena6(&mut self) -> _PWMENA6W {
            _PWMENA6W { w: self }
        }
    }
}
# [ doc = "Load Enable Register. Enables use of updated PWM match values." ]
pub struct LER {
    register: VolatileCell<u32>,
}
# [ doc = "Load Enable Register. Enables use of updated PWM match values." ]
pub mod ler {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::LER {
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
    pub struct MAT0LATCHENR {
        bits: bool,
    }
    impl MAT0LATCHENR {
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
    pub struct MAT1LATCHENR {
        bits: bool,
    }
    impl MAT1LATCHENR {
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
    pub struct MAT2LATCHENR {
        bits: bool,
    }
    impl MAT2LATCHENR {
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
    pub struct MAT3LATCHENR {
        bits: bool,
    }
    impl MAT3LATCHENR {
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
    pub struct MAT4LATCHENR {
        bits: bool,
    }
    impl MAT4LATCHENR {
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
    pub struct MAT5LATCHENR {
        bits: bool,
    }
    impl MAT5LATCHENR {
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
    pub struct MAT6LATCHENR {
        bits: bool,
    }
    impl MAT6LATCHENR {
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
    pub struct _MAT0LATCHENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MAT0LATCHENW<'a> {
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
    pub struct _MAT1LATCHENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MAT1LATCHENW<'a> {
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
    pub struct _MAT2LATCHENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MAT2LATCHENW<'a> {
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
    pub struct _MAT3LATCHENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MAT3LATCHENW<'a> {
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
    pub struct _MAT4LATCHENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MAT4LATCHENW<'a> {
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
    pub struct _MAT5LATCHENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MAT5LATCHENW<'a> {
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
    pub struct _MAT6LATCHENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MAT6LATCHENW<'a> {
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
        # [ doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7." ]
        # [ inline ( always ) ]
        pub fn mat0latchen(&self) -> MAT0LATCHENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MAT0LATCHENR { bits }
        }
        # [ doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat1latchen(&self) -> MAT1LATCHENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MAT1LATCHENR { bits }
        }
        # [ doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat2latchen(&self) -> MAT2LATCHENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MAT2LATCHENR { bits }
        }
        # [ doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat3latchen(&self) -> MAT3LATCHENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MAT3LATCHENR { bits }
        }
        # [ doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat4latchen(&self) -> MAT4LATCHENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MAT4LATCHENR { bits }
        }
        # [ doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat5latchen(&self) -> MAT5LATCHENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MAT5LATCHENR { bits }
        }
        # [ doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat6latchen(&self) -> MAT6LATCHENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MAT6LATCHENR { bits }
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
        # [ doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7." ]
        # [ inline ( always ) ]
        pub fn mat0latchen(&mut self) -> _MAT0LATCHENW {
            _MAT0LATCHENW { w: self }
        }
        # [ doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat1latchen(&mut self) -> _MAT1LATCHENW {
            _MAT1LATCHENW { w: self }
        }
        # [ doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat2latchen(&mut self) -> _MAT2LATCHENW {
            _MAT2LATCHENW { w: self }
        }
        # [ doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat3latchen(&mut self) -> _MAT3LATCHENW {
            _MAT3LATCHENW { w: self }
        }
        # [ doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat4latchen(&mut self) -> _MAT4LATCHENW {
            _MAT4LATCHENW { w: self }
        }
        # [ doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat5latchen(&mut self) -> _MAT5LATCHENW {
            _MAT5LATCHENW { w: self }
        }
        # [ doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details." ]
        # [ inline ( always ) ]
        pub fn mat6latchen(&mut self) -> _MAT6LATCHENW {
            _MAT6LATCHENW { w: self }
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
    # [ doc = "Possible values of the field `MOD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MODR {
        # [ doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register." ]
        TIMER_MODE_THE_TC_I,
        # [ doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2." ]
        RISING_EDGE_COUNTER_,
        # [ doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2." ]
        FALLING_EDGE_COUNTER,
        # [ doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2." ]
        DUAL_EDGE_COUNTER_MO,
    }
    impl MODR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                MODR::TIMER_MODE_THE_TC_I => 0,
                MODR::RISING_EDGE_COUNTER_ => 1,
                MODR::FALLING_EDGE_COUNTER => 2,
                MODR::DUAL_EDGE_COUNTER_MO => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> MODR {
            match value {
                0 => MODR::TIMER_MODE_THE_TC_I,
                1 => MODR::RISING_EDGE_COUNTER_,
                2 => MODR::FALLING_EDGE_COUNTER,
                3 => MODR::DUAL_EDGE_COUNTER_MO,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `TIMER_MODE_THE_TC_I`" ]
        # [ inline ( always ) ]
        pub fn is_timer_mode_the_tc_i(&self) -> bool {
            *self == MODR::TIMER_MODE_THE_TC_I
        }
        # [ doc = "Checks if the value of the field is `RISING_EDGE_COUNTER_`" ]
        # [ inline ( always ) ]
        pub fn is_rising_edge_counter_(&self) -> bool {
            *self == MODR::RISING_EDGE_COUNTER_
        }
        # [ doc = "Checks if the value of the field is `FALLING_EDGE_COUNTER`" ]
        # [ inline ( always ) ]
        pub fn is_falling_edge_counter(&self) -> bool {
            *self == MODR::FALLING_EDGE_COUNTER
        }
        # [ doc = "Checks if the value of the field is `DUAL_EDGE_COUNTER_MO`" ]
        # [ inline ( always ) ]
        pub fn is_dual_edge_counter_mo(&self) -> bool {
            *self == MODR::DUAL_EDGE_COUNTER_MO
        }
    }
    # [ doc = "Possible values of the field `CIS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CISR {
        # [ doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)" ]
        FOR_PWM0_00_EQ_PWM0_,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl CISR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                CISR::FOR_PWM0_00_EQ_PWM0_ => 0,
                CISR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> CISR {
            match value {
                0 => CISR::FOR_PWM0_00_EQ_PWM0_,
                i => CISR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `FOR_PWM0_00_EQ_PWM0_`" ]
        # [ inline ( always ) ]
        pub fn is_for_pwm0_00_eq_pwm0_(&self) -> bool {
            *self == CISR::FOR_PWM0_00_EQ_PWM0_
        }
    }
    # [ doc = "Values that can be written to the field `MOD`" ]
    pub enum MODW {
        # [ doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register." ]
        TIMER_MODE_THE_TC_I,
        # [ doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2." ]
        RISING_EDGE_COUNTER_,
        # [ doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2." ]
        FALLING_EDGE_COUNTER,
        # [ doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2." ]
        DUAL_EDGE_COUNTER_MO,
    }
    impl MODW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                MODW::TIMER_MODE_THE_TC_I => 0,
                MODW::RISING_EDGE_COUNTER_ => 1,
                MODW::FALLING_EDGE_COUNTER => 2,
                MODW::DUAL_EDGE_COUNTER_MO => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MODW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register." ]
        # [ inline ( always ) ]
        pub fn timer_mode_the_tc_i(self) -> &'a mut W {
            self.variant(MODW::TIMER_MODE_THE_TC_I)
        }
        # [ doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2." ]
        # [ inline ( always ) ]
        pub fn rising_edge_counter_(self) -> &'a mut W {
            self.variant(MODW::RISING_EDGE_COUNTER_)
        }
        # [ doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2." ]
        # [ inline ( always ) ]
        pub fn falling_edge_counter(self) -> &'a mut W {
            self.variant(MODW::FALLING_EDGE_COUNTER)
        }
        # [ doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2." ]
        # [ inline ( always ) ]
        pub fn dual_edge_counter_mo(self) -> &'a mut W {
            self.variant(MODW::DUAL_EDGE_COUNTER_MO)
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
    # [ doc = "Values that can be written to the field `CIS`" ]
    pub enum CISW {
        # [ doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)" ]
        FOR_PWM0_00_EQ_PWM0_,
    }
    impl CISW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                CISW::FOR_PWM0_00_EQ_PWM0_ => 0,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CISW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CISW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CISW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)" ]
        # [ inline ( always ) ]
        pub fn for_pwm0_00_eq_pwm0_(self) -> &'a mut W {
            self.variant(CISW::FOR_PWM0_00_EQ_PWM0_)
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
        # [ doc = "Bits 0:1 - Counter/ Timer Mode" ]
        # [ inline ( always ) ]
        pub fn mod_(&self) -> MODR {
            MODR::_from({
                            const MASK: u8 = 3;
                            const OFFSET: u8 = 0;
                            ((self.bits >> OFFSET) & MASK as u32) as u8
                        })
        }
        # [ doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved." ]
        # [ inline ( always ) ]
        pub fn cis(&self) -> CISR {
            CISR::_from({
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
        # [ doc = "Bits 0:1 - Counter/ Timer Mode" ]
        # [ inline ( always ) ]
        pub fn mod_(&mut self) -> _MODW {
            _MODW { w: self }
        }
        # [ doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved." ]
        # [ inline ( always ) ]
        pub fn cis(&mut self) -> _CISW {
            _CISW { w: self }
        }
    }
}
# [ doc = "Pulse Width Modulators (PWM1)" ]
pub struct PWM1 {
    register_block: RegisterBlock,
}
impl Deref for PWM1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
