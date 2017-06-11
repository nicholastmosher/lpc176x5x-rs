# ! [ doc = "System and clock control" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "System and clock control" ]
pub const SYSCON: Peripheral<SYSCON> = unsafe { Peripheral::new(1074774016) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Flash Accelerator Configuration Register. Controls flash access timing." ]
    pub flashcfg: FLASHCFG,
    _reserved0: [u8; 124usize],
    # [ doc = "0x80 - PLL0 Control Register" ]
    pub pll0con: PLL0CON,
    # [ doc = "0x84 - PLL0 Configuration Register" ]
    pub pll0cfg: PLL0CFG,
    # [ doc = "0x88 - PLL0 Status Register" ]
    pub pll0stat: PLL0STAT,
    # [ doc = "0x8c - PLL0 Feed Register" ]
    pub pll0feed: PLL0FEED,
    _reserved1: [u8; 16usize],
    # [ doc = "0xa0 - PLL1 Control Register" ]
    pub pll1con: PLL1CON,
    # [ doc = "0xa4 - PLL1 Configuration Register" ]
    pub pll1cfg: PLL1CFG,
    # [ doc = "0xa8 - PLL1 Status Register" ]
    pub pll1stat: PLL1STAT,
    # [ doc = "0xac - PLL1 Feed Register" ]
    pub pll1feed: PLL1FEED,
    _reserved2: [u8; 16usize],
    # [ doc = "0xc0 - Power Control Register" ]
    pub pcon: PCON,
    # [ doc = "0xc4 - Power Control for Peripherals Register" ]
    pub pconp: PCONP,
    _reserved3: [u8; 60usize],
    # [ doc = "0x104 - CPU Clock Configuration Register" ]
    pub cclkcfg: CCLKCFG,
    # [ doc = "0x108 - USB Clock Configuration Register" ]
    pub usbclkcfg: USBCLKCFG,
    # [ doc = "0x10c - Clock Source Select Register" ]
    pub clksrcsel: CLKSRCSEL,
    # [ doc = "0x110 - Allows clearing the current CAN channel sleep state as well as reading that state." ]
    pub cansleepclr: CANSLEEPCLR,
    # [ doc = "0x114 - Allows reading the wake-up state of the CAN channels." ]
    pub canwakeflags: CANWAKEFLAGS,
    _reserved4: [u8; 40usize],
    # [ doc = "0x140 - External Interrupt Flag Register" ]
    pub extint: EXTINT,
    _reserved5: [u8; 4usize],
    # [ doc = "0x148 - External Interrupt Mode register" ]
    pub extmode: EXTMODE,
    # [ doc = "0x14c - External Interrupt Polarity Register" ]
    pub extpolar: EXTPOLAR,
    _reserved6: [u8; 48usize],
    # [ doc = "0x180 - Reset Source Identification Register" ]
    pub rsid: RSID,
    _reserved7: [u8; 28usize],
    # [ doc = "0x1a0 - System control and status" ]
    pub scs: SCS,
    _reserved8: [u8; 4usize],
    # [ doc = "0x1a8 - Peripheral Clock Selection register 0." ]
    pub pclksel0: PCLKSEL0,
    # [ doc = "0x1ac - Peripheral Clock Selection register 1." ]
    pub pclksel1: PCLKSEL1,
    _reserved9: [u8; 16usize],
    # [ doc = "0x1c0 - USB Interrupt Status" ]
    pub usbintst: USBINTST,
    # [ doc = "0x1c4 - Selects between alternative requests on DMA channels 0 through 7 and 10 through 15" ]
    pub dmacreqsel: DMACREQSEL,
    # [ doc = "0x1c8 - Clock Output Configuration Register" ]
    pub clkoutcfg: CLKOUTCFG,
}
# [ doc = "Flash Accelerator Configuration Register. Controls flash access timing." ]
pub struct FLASHCFG {
    register: VolatileCell<u32>,
}
# [ doc = "Flash Accelerator Configuration Register. Controls flash access timing." ]
pub mod flashcfg {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::FLASHCFG {
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
    # [ doc = "Possible values of the field `FLASHTIM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum FLASHTIMR {
        # [ doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock." ]
        _1CLK,
        # [ doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock." ]
        _2CLK,
        # [ doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock." ]
        _3CLK,
        # [ doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock." ]
        _4CLK,
        # [ doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only." ]
        _5CLK,
        # [ doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions." ]
        _6CLK,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl FLASHTIMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                FLASHTIMR::_1CLK => 0,
                FLASHTIMR::_2CLK => 1,
                FLASHTIMR::_3CLK => 2,
                FLASHTIMR::_4CLK => 3,
                FLASHTIMR::_5CLK => 4,
                FLASHTIMR::_6CLK => 5,
                FLASHTIMR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> FLASHTIMR {
            match value {
                0 => FLASHTIMR::_1CLK,
                1 => FLASHTIMR::_2CLK,
                2 => FLASHTIMR::_3CLK,
                3 => FLASHTIMR::_4CLK,
                4 => FLASHTIMR::_5CLK,
                5 => FLASHTIMR::_6CLK,
                i => FLASHTIMR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `_1CLK`" ]
        # [ inline ( always ) ]
        pub fn is_1clk(&self) -> bool {
            *self == FLASHTIMR::_1CLK
        }
        # [ doc = "Checks if the value of the field is `_2CLK`" ]
        # [ inline ( always ) ]
        pub fn is_2clk(&self) -> bool {
            *self == FLASHTIMR::_2CLK
        }
        # [ doc = "Checks if the value of the field is `_3CLK`" ]
        # [ inline ( always ) ]
        pub fn is_3clk(&self) -> bool {
            *self == FLASHTIMR::_3CLK
        }
        # [ doc = "Checks if the value of the field is `_4CLK`" ]
        # [ inline ( always ) ]
        pub fn is_4clk(&self) -> bool {
            *self == FLASHTIMR::_4CLK
        }
        # [ doc = "Checks if the value of the field is `_5CLK`" ]
        # [ inline ( always ) ]
        pub fn is_5clk(&self) -> bool {
            *self == FLASHTIMR::_5CLK
        }
        # [ doc = "Checks if the value of the field is `_6CLK`" ]
        # [ inline ( always ) ]
        pub fn is_6clk(&self) -> bool {
            *self == FLASHTIMR::_6CLK
        }
    }
    # [ doc = "Values that can be written to the field `FLASHTIM`" ]
    pub enum FLASHTIMW {
        # [ doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock." ]
        _1CLK,
        # [ doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock." ]
        _2CLK,
        # [ doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock." ]
        _3CLK,
        # [ doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock." ]
        _4CLK,
        # [ doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only." ]
        _5CLK,
        # [ doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions." ]
        _6CLK,
    }
    impl FLASHTIMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                FLASHTIMW::_1CLK => 0,
                FLASHTIMW::_2CLK => 1,
                FLASHTIMW::_3CLK => 2,
                FLASHTIMW::_4CLK => 3,
                FLASHTIMW::_5CLK => 4,
                FLASHTIMW::_6CLK => 5,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FLASHTIMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FLASHTIMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: FLASHTIMW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock." ]
        # [ inline ( always ) ]
        pub fn _1clk(self) -> &'a mut W {
            self.variant(FLASHTIMW::_1CLK)
        }
        # [ doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock." ]
        # [ inline ( always ) ]
        pub fn _2clk(self) -> &'a mut W {
            self.variant(FLASHTIMW::_2CLK)
        }
        # [ doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock." ]
        # [ inline ( always ) ]
        pub fn _3clk(self) -> &'a mut W {
            self.variant(FLASHTIMW::_3CLK)
        }
        # [ doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock." ]
        # [ inline ( always ) ]
        pub fn _4clk(self) -> &'a mut W {
            self.variant(FLASHTIMW::_4CLK)
        }
        # [ doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only." ]
        # [ inline ( always ) ]
        pub fn _5clk(self) -> &'a mut W {
            self.variant(FLASHTIMW::_5CLK)
        }
        # [ doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions." ]
        # [ inline ( always ) ]
        pub fn _6clk(self) -> &'a mut W {
            self.variant(FLASHTIMW::_6CLK)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
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
        # [ doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved." ]
        # [ inline ( always ) ]
        pub fn flashtim(&self) -> FLASHTIMR {
            FLASHTIMR::_from({
                                 const MASK: u8 = 15;
                                 const OFFSET: u8 = 12;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 12346 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved." ]
        # [ inline ( always ) ]
        pub fn flashtim(&mut self) -> _FLASHTIMW {
            _FLASHTIMW { w: self }
        }
    }
}
# [ doc = "PLL0 Control Register" ]
pub struct PLL0CON {
    register: VolatileCell<u32>,
}
# [ doc = "PLL0 Control Register" ]
pub mod pll0con {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PLL0CON {
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
    pub struct PLLE0R {
        bits: bool,
    }
    impl PLLE0R {
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
    pub struct PLLC0R {
        bits: bool,
    }
    impl PLLC0R {
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
    pub struct _PLLE0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLE0W<'a> {
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
    pub struct _PLLC0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLC0W<'a> {
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
        # [ doc = "Bit 0 - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register." ]
        # [ inline ( always ) ]
        pub fn plle0(&self) -> PLLE0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLE0R { bits }
        }
        # [ doc = "Bit 1 - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register." ]
        # [ inline ( always ) ]
        pub fn pllc0(&self) -> PLLC0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLC0R { bits }
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
        # [ doc = "Bit 0 - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register." ]
        # [ inline ( always ) ]
        pub fn plle0(&mut self) -> _PLLE0W {
            _PLLE0W { w: self }
        }
        # [ doc = "Bit 1 - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register." ]
        # [ inline ( always ) ]
        pub fn pllc0(&mut self) -> _PLLC0W {
            _PLLC0W { w: self }
        }
    }
}
# [ doc = "PLL0 Configuration Register" ]
pub struct PLL0CFG {
    register: VolatileCell<u32>,
}
# [ doc = "PLL0 Configuration Register" ]
pub mod pll0cfg {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PLL0CFG {
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
    pub struct MSEL0R {
        bits: u16,
    }
    impl MSEL0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct NSEL0R {
        bits: u8,
    }
    impl NSEL0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MSEL0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MSEL0W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _NSEL0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NSEL0W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
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
        # [ doc = "Bits 0:14 - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware." ]
        # [ inline ( always ) ]
        pub fn msel0(&self) -> MSEL0R {
            let bits = {
                const MASK: u16 = 32767;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            MSEL0R { bits }
        }
        # [ doc = "Bits 16:23 - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32." ]
        # [ inline ( always ) ]
        pub fn nsel0(&self) -> NSEL0R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            NSEL0R { bits }
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
        # [ doc = "Bits 0:14 - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware." ]
        # [ inline ( always ) ]
        pub fn msel0(&mut self) -> _MSEL0W {
            _MSEL0W { w: self }
        }
        # [ doc = "Bits 16:23 - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32." ]
        # [ inline ( always ) ]
        pub fn nsel0(&mut self) -> _NSEL0W {
            _NSEL0W { w: self }
        }
    }
}
# [ doc = "PLL0 Status Register" ]
pub struct PLL0STAT {
    register: VolatileCell<u32>,
}
# [ doc = "PLL0 Status Register" ]
pub mod pll0stat {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::PLL0STAT {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MSEL0R {
        bits: u16,
    }
    impl MSEL0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct NSEL0R {
        bits: u8,
    }
    impl NSEL0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PLLE0_STATR {
        bits: bool,
    }
    impl PLLE0_STATR {
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
    pub struct PLLC0_STATR {
        bits: bool,
    }
    impl PLLC0_STATR {
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
    pub struct PLOCK0R {
        bits: bool,
    }
    impl PLOCK0R {
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
        # [ doc = "Bits 0:14 - Read-back for the PLL0 Multiplier value. This is the value currently used by PLL0, and is one less than the actual multiplier." ]
        # [ inline ( always ) ]
        pub fn msel0(&self) -> MSEL0R {
            let bits = {
                const MASK: u16 = 32767;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            MSEL0R { bits }
        }
        # [ doc = "Bits 16:23 - Read-back for the PLL0 Pre-Divider value. This is the value currently used by PLL0, and is one less than the actual divider." ]
        # [ inline ( always ) ]
        pub fn nsel0(&self) -> NSEL0R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            NSEL0R { bits }
        }
        # [ doc = "Bit 24 - Read-back for the PLL0 Enable bit. This bit reflects the state of the PLEC0 bit in PLL0CON after a valid PLL0 feed. When one, PLL0 is currently enabled. When zero, PLL0 is turned off. This bit is automatically cleared when Power-down mode is entered." ]
        # [ inline ( always ) ]
        pub fn plle0_stat(&self) -> PLLE0_STATR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLE0_STATR { bits }
        }
        # [ doc = "Bit 25 - Read-back for the PLL0 Connect bit. This bit reflects the state of the PLLC0 bit in PLL0CON after a valid PLL0 feed. When PLLC0 and PLLE0 are both one, PLL0 is connected as the clock source for the CPU. When either PLLC0 or PLLE0 is zero, PLL0 is bypassed. This bit is automatically cleared when Power-down mode is entered." ]
        # [ inline ( always ) ]
        pub fn pllc0_stat(&self) -> PLLC0_STATR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLC0_STATR { bits }
        }
        # [ doc = "Bit 26 - Reflects the PLL0 Lock status. When zero, PLL0 is not locked. When one, PLL0 is locked onto the requested frequency. See text for details." ]
        # [ inline ( always ) ]
        pub fn plock0(&self) -> PLOCK0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLOCK0R { bits }
        }
    }
}
# [ doc = "PLL0 Feed Register" ]
pub struct PLL0FEED {
    register: VolatileCell<u32>,
}
# [ doc = "PLL0 Feed Register" ]
pub mod pll0feed {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PLL0FEED {
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
    pub struct _PLL0FEEDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLL0FEEDW<'a> {
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
        # [ doc = "Bits 0:7 - The PLL0 feed sequence must be written to this register in order for PLL0 configuration and control register changes to take effect." ]
        # [ inline ( always ) ]
        pub fn pll0feed(&mut self) -> _PLL0FEEDW {
            _PLL0FEEDW { w: self }
        }
    }
}
# [ doc = "PLL1 Control Register" ]
pub struct PLL1CON {
    register: VolatileCell<u32>,
}
# [ doc = "PLL1 Control Register" ]
pub mod pll1con {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PLL1CON {
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
    pub struct PLLE1R {
        bits: bool,
    }
    impl PLLE1R {
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
    pub struct PLLC1R {
        bits: bool,
    }
    impl PLLC1R {
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
    pub struct _PLLE1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLE1W<'a> {
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
    pub struct _PLLC1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLLC1W<'a> {
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
        # [ doc = "Bit 0 - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency." ]
        # [ inline ( always ) ]
        pub fn plle1(&self) -> PLLE1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLE1R { bits }
        }
        # [ doc = "Bit 1 - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register." ]
        # [ inline ( always ) ]
        pub fn pllc1(&self) -> PLLC1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLC1R { bits }
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
        # [ doc = "Bit 0 - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency." ]
        # [ inline ( always ) ]
        pub fn plle1(&mut self) -> _PLLE1W {
            _PLLE1W { w: self }
        }
        # [ doc = "Bit 1 - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register." ]
        # [ inline ( always ) ]
        pub fn pllc1(&mut self) -> _PLLC1W {
            _PLLC1W { w: self }
        }
    }
}
# [ doc = "PLL1 Configuration Register" ]
pub struct PLL1CFG {
    register: VolatileCell<u32>,
}
# [ doc = "PLL1 Configuration Register" ]
pub mod pll1cfg {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PLL1CFG {
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
    pub struct MSEL1R {
        bits: u8,
    }
    impl MSEL1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PSEL1R {
        bits: u8,
    }
    impl PSEL1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MSEL1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MSEL1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PSEL1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PSEL1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        # [ doc = "Bits 0:4 - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations." ]
        # [ inline ( always ) ]
        pub fn msel1(&self) -> MSEL1R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MSEL1R { bits }
        }
        # [ doc = "Bits 5:6 - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations." ]
        # [ inline ( always ) ]
        pub fn psel1(&self) -> PSEL1R {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PSEL1R { bits }
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
        # [ doc = "Bits 0:4 - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations." ]
        # [ inline ( always ) ]
        pub fn msel1(&mut self) -> _MSEL1W {
            _MSEL1W { w: self }
        }
        # [ doc = "Bits 5:6 - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations." ]
        # [ inline ( always ) ]
        pub fn psel1(&mut self) -> _PSEL1W {
            _PSEL1W { w: self }
        }
    }
}
# [ doc = "PLL1 Status Register" ]
pub struct PLL1STAT {
    register: VolatileCell<u32>,
}
# [ doc = "PLL1 Status Register" ]
pub mod pll1stat {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::PLL1STAT {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MSEL1R {
        bits: u8,
    }
    impl MSEL1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PSEL1R {
        bits: u8,
    }
    impl PSEL1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct PLLE1_STATR {
        bits: bool,
    }
    impl PLLE1_STATR {
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
    pub struct PLLC1_STATR {
        bits: bool,
    }
    impl PLLC1_STATR {
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
    pub struct PLOCK1R {
        bits: bool,
    }
    impl PLOCK1R {
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
        # [ doc = "Bits 0:4 - Read-back for the PLL1 Multiplier value. This is the value currently used by PLL1." ]
        # [ inline ( always ) ]
        pub fn msel1(&self) -> MSEL1R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MSEL1R { bits }
        }
        # [ doc = "Bits 5:6 - Read-back for the PLL1 Divider value. This is the value currently used by PLL1." ]
        # [ inline ( always ) ]
        pub fn psel1(&self) -> PSEL1R {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PSEL1R { bits }
        }
        # [ doc = "Bit 8 - Read-back for the PLL1 Enable bit. When one, PLL1 is currently activated. When zero, PLL1 is turned off. This bit is automatically cleared when Power-down mode is activated." ]
        # [ inline ( always ) ]
        pub fn plle1_stat(&self) -> PLLE1_STATR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLE1_STATR { bits }
        }
        # [ doc = "Bit 9 - Read-back for the PLL1 Connect bit. When PLLC and PLLE are both one, PLL1 is connected as the clock source for the microcontroller. When either PLLC or PLLE is zero, PLL1 is bypassed and the oscillator clock is used directly by the microcontroller. This bit is automatically cleared when Power-down mode is activated." ]
        # [ inline ( always ) ]
        pub fn pllc1_stat(&self) -> PLLC1_STATR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLLC1_STATR { bits }
        }
        # [ doc = "Bit 10 - Reflects the PLL1 Lock status. When zero, PLL1 is not locked. When one, PLL1 is locked onto the requested frequency." ]
        # [ inline ( always ) ]
        pub fn plock1(&self) -> PLOCK1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PLOCK1R { bits }
        }
    }
}
# [ doc = "PLL1 Feed Register" ]
pub struct PLL1FEED {
    register: VolatileCell<u32>,
}
# [ doc = "PLL1 Feed Register" ]
pub mod pll1feed {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PLL1FEED {
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
    pub struct _PLL1FEEDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PLL1FEEDW<'a> {
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
        # [ doc = "Bits 0:7 - The PLL1 feed sequence must be written to this register in order for PLL1 configuration and control register changes to take effect." ]
        # [ inline ( always ) ]
        pub fn pll1feed(&mut self) -> _PLL1FEEDW {
            _PLL1FEEDW { w: self }
        }
    }
}
# [ doc = "Power Control Register" ]
pub struct PCON {
    register: VolatileCell<u32>,
}
# [ doc = "Power Control Register" ]
pub mod pcon {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PCON {
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
    pub struct PM0R {
        bits: bool,
    }
    impl PM0R {
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
    pub struct PM1R {
        bits: bool,
    }
    impl PM1R {
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
    pub struct BODRPMR {
        bits: bool,
    }
    impl BODRPMR {
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
    pub struct BOGDR {
        bits: bool,
    }
    impl BOGDR {
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
    pub struct BORDR {
        bits: bool,
    }
    impl BORDR {
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
    pub struct SMFLAGR {
        bits: bool,
    }
    impl SMFLAGR {
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
    pub struct DSFLAGR {
        bits: bool,
    }
    impl DSFLAGR {
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
    pub struct PDFLAGR {
        bits: bool,
    }
    impl PDFLAGR {
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
    pub struct DPDFLAGR {
        bits: bool,
    }
    impl DPDFLAGR {
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
    pub struct _PM0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PM0W<'a> {
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
    pub struct _PM1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PM1W<'a> {
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
    pub struct _BODRPMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BODRPMW<'a> {
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
    pub struct _BOGDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BOGDW<'a> {
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
    pub struct _BORDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BORDW<'a> {
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
    pub struct _SMFLAGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMFLAGW<'a> {
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
    pub struct _DSFLAGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DSFLAGW<'a> {
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
    pub struct _PDFLAGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PDFLAGW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DPDFLAGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPDFLAGW<'a> {
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
        # [ doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode." ]
        # [ inline ( always ) ]
        pub fn pm0(&self) -> PM0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PM0R { bits }
        }
        # [ doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode." ]
        # [ inline ( always ) ]
        pub fn pm1(&self) -> PM1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PM1R { bits }
        }
        # [ doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection." ]
        # [ inline ( always ) ]
        pub fn bodrpm(&self) -> BODRPMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BODRPMR { bits }
        }
        # [ doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit." ]
        # [ inline ( always ) ]
        pub fn bogd(&self) -> BOGDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOGDR { bits }
        }
        # [ doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled." ]
        # [ inline ( always ) ]
        pub fn bord(&self) -> BORDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BORDR { bits }
        }
        # [ doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit." ]
        # [ inline ( always ) ]
        pub fn smflag(&self) -> SMFLAGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SMFLAGR { bits }
        }
        # [ doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit." ]
        # [ inline ( always ) ]
        pub fn dsflag(&self) -> DSFLAGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DSFLAGR { bits }
        }
        # [ doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit." ]
        # [ inline ( always ) ]
        pub fn pdflag(&self) -> PDFLAGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PDFLAGR { bits }
        }
        # [ doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit." ]
        # [ inline ( always ) ]
        pub fn dpdflag(&self) -> DPDFLAGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPDFLAGR { bits }
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
        # [ doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode." ]
        # [ inline ( always ) ]
        pub fn pm0(&mut self) -> _PM0W {
            _PM0W { w: self }
        }
        # [ doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode." ]
        # [ inline ( always ) ]
        pub fn pm1(&mut self) -> _PM1W {
            _PM1W { w: self }
        }
        # [ doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection." ]
        # [ inline ( always ) ]
        pub fn bodrpm(&mut self) -> _BODRPMW {
            _BODRPMW { w: self }
        }
        # [ doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit." ]
        # [ inline ( always ) ]
        pub fn bogd(&mut self) -> _BOGDW {
            _BOGDW { w: self }
        }
        # [ doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled." ]
        # [ inline ( always ) ]
        pub fn bord(&mut self) -> _BORDW {
            _BORDW { w: self }
        }
        # [ doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit." ]
        # [ inline ( always ) ]
        pub fn smflag(&mut self) -> _SMFLAGW {
            _SMFLAGW { w: self }
        }
        # [ doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit." ]
        # [ inline ( always ) ]
        pub fn dsflag(&mut self) -> _DSFLAGW {
            _DSFLAGW { w: self }
        }
        # [ doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit." ]
        # [ inline ( always ) ]
        pub fn pdflag(&mut self) -> _PDFLAGW {
            _PDFLAGW { w: self }
        }
        # [ doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit." ]
        # [ inline ( always ) ]
        pub fn dpdflag(&mut self) -> _DPDFLAGW {
            _DPDFLAGW { w: self }
        }
    }
}
# [ doc = "Power Control for Peripherals Register" ]
pub struct PCONP {
    register: VolatileCell<u32>,
}
# [ doc = "Power Control for Peripherals Register" ]
pub mod pconp {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PCONP {
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
    pub struct PCTIM0R {
        bits: bool,
    }
    impl PCTIM0R {
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
    pub struct PCTIM1R {
        bits: bool,
    }
    impl PCTIM1R {
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
    pub struct PCUART0R {
        bits: bool,
    }
    impl PCUART0R {
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
    pub struct PCUART1R {
        bits: bool,
    }
    impl PCUART1R {
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
    pub struct PCPWM1R {
        bits: bool,
    }
    impl PCPWM1R {
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
    pub struct PCI2C0R {
        bits: bool,
    }
    impl PCI2C0R {
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
    pub struct PCSPIR {
        bits: bool,
    }
    impl PCSPIR {
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
    pub struct PCRTCR {
        bits: bool,
    }
    impl PCRTCR {
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
    pub struct PCSSP1R {
        bits: bool,
    }
    impl PCSSP1R {
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
    pub struct PCADCR {
        bits: bool,
    }
    impl PCADCR {
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
    pub struct PCCAN1R {
        bits: bool,
    }
    impl PCCAN1R {
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
    pub struct PCCAN2R {
        bits: bool,
    }
    impl PCCAN2R {
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
    pub struct PCGPIOR {
        bits: bool,
    }
    impl PCGPIOR {
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
    pub struct PCRITR {
        bits: bool,
    }
    impl PCRITR {
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
    pub struct PCMCPWMR {
        bits: bool,
    }
    impl PCMCPWMR {
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
    pub struct PCQEIR {
        bits: bool,
    }
    impl PCQEIR {
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
    pub struct PCI2C1R {
        bits: bool,
    }
    impl PCI2C1R {
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
    pub struct PCSSP0R {
        bits: bool,
    }
    impl PCSSP0R {
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
    pub struct PCTIM2R {
        bits: bool,
    }
    impl PCTIM2R {
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
    pub struct PCTIM3R {
        bits: bool,
    }
    impl PCTIM3R {
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
    pub struct PCUART2R {
        bits: bool,
    }
    impl PCUART2R {
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
    pub struct PCUART3R {
        bits: bool,
    }
    impl PCUART3R {
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
    pub struct PCI2C2R {
        bits: bool,
    }
    impl PCI2C2R {
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
    pub struct PCI2SR {
        bits: bool,
    }
    impl PCI2SR {
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
    pub struct PCGPDMAR {
        bits: bool,
    }
    impl PCGPDMAR {
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
    pub struct PCENETR {
        bits: bool,
    }
    impl PCENETR {
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
    pub struct PCUSBR {
        bits: bool,
    }
    impl PCUSBR {
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
    pub struct _PCTIM0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCTIM0W<'a> {
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
    pub struct _PCTIM1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCTIM1W<'a> {
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
    pub struct _PCUART0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCUART0W<'a> {
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
    pub struct _PCUART1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCUART1W<'a> {
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
    pub struct _PCPWM1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCPWM1W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PCI2C0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCI2C0W<'a> {
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
            const OFFSET: u8 = 7;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCSPIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCSPIW<'a> {
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
    pub struct _PCRTCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCRTCW<'a> {
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
    pub struct _PCSSP1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCSSP1W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PCADCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCADCW<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCCAN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCCAN1W<'a> {
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
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCCAN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCCAN2W<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCGPIOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCGPIOW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PCRITW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCRITW<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCMCPWMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCMCPWMW<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCQEIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCQEIW<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCI2C1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCI2C1W<'a> {
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
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCSSP0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCSSP0W<'a> {
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
            const OFFSET: u8 = 21;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCTIM2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCTIM2W<'a> {
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
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCTIM3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCTIM3W<'a> {
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
            const OFFSET: u8 = 23;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCUART2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCUART2W<'a> {
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
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCUART3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCUART3W<'a> {
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
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCI2C2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCI2C2W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PCI2SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCI2SW<'a> {
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
            const OFFSET: u8 = 27;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCGPDMAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCGPDMAW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PCENETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCENETW<'a> {
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
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCUSBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCUSBW<'a> {
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
        # [ doc = "Bit 1 - Timer/Counter 0 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pctim0(&self) -> PCTIM0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCTIM0R { bits }
        }
        # [ doc = "Bit 2 - Timer/Counter 1 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pctim1(&self) -> PCTIM1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCTIM1R { bits }
        }
        # [ doc = "Bit 3 - UART0 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcuart0(&self) -> PCUART0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCUART0R { bits }
        }
        # [ doc = "Bit 4 - UART1 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcuart1(&self) -> PCUART1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCUART1R { bits }
        }
        # [ doc = "Bit 6 - PWM1 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcpwm1(&self) -> PCPWM1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCPWM1R { bits }
        }
        # [ doc = "Bit 7 - The I2C0 interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pci2c0(&self) -> PCI2C0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCI2C0R { bits }
        }
        # [ doc = "Bit 8 - The SPI interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcspi(&self) -> PCSPIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCSPIR { bits }
        }
        # [ doc = "Bit 9 - The RTC power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcrtc(&self) -> PCRTCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCRTCR { bits }
        }
        # [ doc = "Bit 10 - The SSP 1 interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcssp1(&self) -> PCSSP1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCSSP1R { bits }
        }
        # [ doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN." ]
        # [ inline ( always ) ]
        pub fn pcadc(&self) -> PCADCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCADCR { bits }
        }
        # [ doc = "Bit 13 - CAN Controller 1 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pccan1(&self) -> PCCAN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCCAN1R { bits }
        }
        # [ doc = "Bit 14 - CAN Controller 2 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pccan2(&self) -> PCCAN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCCAN2R { bits }
        }
        # [ doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts." ]
        # [ inline ( always ) ]
        pub fn pcgpio(&self) -> PCGPIOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCGPIOR { bits }
        }
        # [ doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcrit(&self) -> PCRITR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCRITR { bits }
        }
        # [ doc = "Bit 17 - Motor Control PWM" ]
        # [ inline ( always ) ]
        pub fn pcmcpwm(&self) -> PCMCPWMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCMCPWMR { bits }
        }
        # [ doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcqei(&self) -> PCQEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCQEIR { bits }
        }
        # [ doc = "Bit 19 - The I2C1 interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pci2c1(&self) -> PCI2C1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCI2C1R { bits }
        }
        # [ doc = "Bit 21 - The SSP0 interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcssp0(&self) -> PCSSP0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCSSP0R { bits }
        }
        # [ doc = "Bit 22 - Timer 2 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pctim2(&self) -> PCTIM2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCTIM2R { bits }
        }
        # [ doc = "Bit 23 - Timer 3 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pctim3(&self) -> PCTIM3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCTIM3R { bits }
        }
        # [ doc = "Bit 24 - UART 2 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcuart2(&self) -> PCUART2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCUART2R { bits }
        }
        # [ doc = "Bit 25 - UART 3 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcuart3(&self) -> PCUART3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCUART3R { bits }
        }
        # [ doc = "Bit 26 - I2C interface 2 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pci2c2(&self) -> PCI2C2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCI2C2R { bits }
        }
        # [ doc = "Bit 27 - I2S interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pci2s(&self) -> PCI2SR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCI2SR { bits }
        }
        # [ doc = "Bit 29 - GPDMA function power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcgpdma(&self) -> PCGPDMAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCGPDMAR { bits }
        }
        # [ doc = "Bit 30 - Ethernet block power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcenet(&self) -> PCENETR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCENETR { bits }
        }
        # [ doc = "Bit 31 - USB interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcusb(&self) -> PCUSBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PCUSBR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 958 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 1 - Timer/Counter 0 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pctim0(&mut self) -> _PCTIM0W {
            _PCTIM0W { w: self }
        }
        # [ doc = "Bit 2 - Timer/Counter 1 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pctim1(&mut self) -> _PCTIM1W {
            _PCTIM1W { w: self }
        }
        # [ doc = "Bit 3 - UART0 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcuart0(&mut self) -> _PCUART0W {
            _PCUART0W { w: self }
        }
        # [ doc = "Bit 4 - UART1 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcuart1(&mut self) -> _PCUART1W {
            _PCUART1W { w: self }
        }
        # [ doc = "Bit 6 - PWM1 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcpwm1(&mut self) -> _PCPWM1W {
            _PCPWM1W { w: self }
        }
        # [ doc = "Bit 7 - The I2C0 interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pci2c0(&mut self) -> _PCI2C0W {
            _PCI2C0W { w: self }
        }
        # [ doc = "Bit 8 - The SPI interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcspi(&mut self) -> _PCSPIW {
            _PCSPIW { w: self }
        }
        # [ doc = "Bit 9 - The RTC power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcrtc(&mut self) -> _PCRTCW {
            _PCRTCW { w: self }
        }
        # [ doc = "Bit 10 - The SSP 1 interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcssp1(&mut self) -> _PCSSP1W {
            _PCSSP1W { w: self }
        }
        # [ doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN." ]
        # [ inline ( always ) ]
        pub fn pcadc(&mut self) -> _PCADCW {
            _PCADCW { w: self }
        }
        # [ doc = "Bit 13 - CAN Controller 1 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pccan1(&mut self) -> _PCCAN1W {
            _PCCAN1W { w: self }
        }
        # [ doc = "Bit 14 - CAN Controller 2 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pccan2(&mut self) -> _PCCAN2W {
            _PCCAN2W { w: self }
        }
        # [ doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts." ]
        # [ inline ( always ) ]
        pub fn pcgpio(&mut self) -> _PCGPIOW {
            _PCGPIOW { w: self }
        }
        # [ doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcrit(&mut self) -> _PCRITW {
            _PCRITW { w: self }
        }
        # [ doc = "Bit 17 - Motor Control PWM" ]
        # [ inline ( always ) ]
        pub fn pcmcpwm(&mut self) -> _PCMCPWMW {
            _PCMCPWMW { w: self }
        }
        # [ doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcqei(&mut self) -> _PCQEIW {
            _PCQEIW { w: self }
        }
        # [ doc = "Bit 19 - The I2C1 interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pci2c1(&mut self) -> _PCI2C1W {
            _PCI2C1W { w: self }
        }
        # [ doc = "Bit 21 - The SSP0 interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcssp0(&mut self) -> _PCSSP0W {
            _PCSSP0W { w: self }
        }
        # [ doc = "Bit 22 - Timer 2 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pctim2(&mut self) -> _PCTIM2W {
            _PCTIM2W { w: self }
        }
        # [ doc = "Bit 23 - Timer 3 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pctim3(&mut self) -> _PCTIM3W {
            _PCTIM3W { w: self }
        }
        # [ doc = "Bit 24 - UART 2 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcuart2(&mut self) -> _PCUART2W {
            _PCUART2W { w: self }
        }
        # [ doc = "Bit 25 - UART 3 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcuart3(&mut self) -> _PCUART3W {
            _PCUART3W { w: self }
        }
        # [ doc = "Bit 26 - I2C interface 2 power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pci2c2(&mut self) -> _PCI2C2W {
            _PCI2C2W { w: self }
        }
        # [ doc = "Bit 27 - I2S interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pci2s(&mut self) -> _PCI2SW {
            _PCI2SW { w: self }
        }
        # [ doc = "Bit 29 - GPDMA function power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcgpdma(&mut self) -> _PCGPDMAW {
            _PCGPDMAW { w: self }
        }
        # [ doc = "Bit 30 - Ethernet block power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcenet(&mut self) -> _PCENETW {
            _PCENETW { w: self }
        }
        # [ doc = "Bit 31 - USB interface power/clock control bit." ]
        # [ inline ( always ) ]
        pub fn pcusb(&mut self) -> _PCUSBW {
            _PCUSBW { w: self }
        }
    }
}
# [ doc = "CPU Clock Configuration Register" ]
pub struct CCLKCFG {
    register: VolatileCell<u32>,
}
# [ doc = "CPU Clock Configuration Register" ]
pub mod cclkcfg {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CCLKCFG {
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
    pub struct CCLKSELR {
        bits: u8,
    }
    impl CCLKSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCLKSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCLKSELW<'a> {
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
        # [ doc = "Bits 0:7 - Selects the divide value for creating the CPU clock (CCLK) from the PLL0 output. 0 = pllclk is divided by 1 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 1 = pllclk is divided by 2 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 2 = pllclk is divided by 3 to produce the CPU clock. 3 = pllclk is divided by 4 to produce the CPU clock. ... 255 = pllclk is divided by 256 to produce the CPU clock." ]
        # [ inline ( always ) ]
        pub fn cclksel(&self) -> CCLKSELR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CCLKSELR { bits }
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
        # [ doc = "Bits 0:7 - Selects the divide value for creating the CPU clock (CCLK) from the PLL0 output. 0 = pllclk is divided by 1 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 1 = pllclk is divided by 2 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 2 = pllclk is divided by 3 to produce the CPU clock. 3 = pllclk is divided by 4 to produce the CPU clock. ... 255 = pllclk is divided by 256 to produce the CPU clock." ]
        # [ inline ( always ) ]
        pub fn cclksel(&mut self) -> _CCLKSELW {
            _CCLKSELW { w: self }
        }
    }
}
# [ doc = "USB Clock Configuration Register" ]
pub struct USBCLKCFG {
    register: VolatileCell<u32>,
}
# [ doc = "USB Clock Configuration Register" ]
pub mod usbclkcfg {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::USBCLKCFG {
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
    pub struct USBSELR {
        bits: u8,
    }
    impl USBSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _USBSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USBSELW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz." ]
        # [ inline ( always ) ]
        pub fn usbsel(&self) -> USBSELR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            USBSELR { bits }
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
        # [ doc = "Bits 0:3 - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz." ]
        # [ inline ( always ) ]
        pub fn usbsel(&mut self) -> _USBSELW {
            _USBSELW { w: self }
        }
    }
}
# [ doc = "Clock Source Select Register" ]
pub struct CLKSRCSEL {
    register: VolatileCell<u32>,
}
# [ doc = "Clock Source Select Register" ]
pub mod clksrcsel {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CLKSRCSEL {
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
    # [ doc = "Possible values of the field `CLKSRC`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CLKSRCR {
        # [ doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)." ]
        SELECTS_THE_INTERNAL,
        # [ doc = "Selects the main oscillator as the PLL0 clock source.  Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s." ]
        SELECTS_THE_MAIN_OSC,
        # [ doc = "Selects the RTC oscillator as the PLL0 clock source." ]
        SELECTS_THE_RTC_OSCI,
    }
    impl CLKSRCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                CLKSRCR::SELECTS_THE_INTERNAL => 0,
                CLKSRCR::SELECTS_THE_MAIN_OSC => 1,
                CLKSRCR::SELECTS_THE_RTC_OSCI => 2,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> CLKSRCR {
            match value {
                0 => CLKSRCR::SELECTS_THE_INTERNAL,
                1 => CLKSRCR::SELECTS_THE_MAIN_OSC,
                2 => CLKSRCR::SELECTS_THE_RTC_OSCI,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`" ]
        # [ inline ( always ) ]
        pub fn is_selects_the_internal(&self) -> bool {
            *self == CLKSRCR::SELECTS_THE_INTERNAL
        }
        # [ doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`" ]
        # [ inline ( always ) ]
        pub fn is_selects_the_main_osc(&self) -> bool {
            *self == CLKSRCR::SELECTS_THE_MAIN_OSC
        }
        # [ doc = "Checks if the value of the field is `SELECTS_THE_RTC_OSCI`" ]
        # [ inline ( always ) ]
        pub fn is_selects_the_rtc_osci(&self) -> bool {
            *self == CLKSRCR::SELECTS_THE_RTC_OSCI
        }
    }
    # [ doc = "Values that can be written to the field `CLKSRC`" ]
    pub enum CLKSRCW {
        # [ doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)." ]
        SELECTS_THE_INTERNAL,
        # [ doc = "Selects the main oscillator as the PLL0 clock source.  Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s." ]
        SELECTS_THE_MAIN_OSC,
        # [ doc = "Selects the RTC oscillator as the PLL0 clock source." ]
        SELECTS_THE_RTC_OSCI,
    }
    impl CLKSRCW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                CLKSRCW::SELECTS_THE_INTERNAL => 0,
                CLKSRCW::SELECTS_THE_MAIN_OSC => 1,
                CLKSRCW::SELECTS_THE_RTC_OSCI => 2,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKSRCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKSRCW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CLKSRCW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)." ]
        # [ inline ( always ) ]
        pub fn selects_the_internal(self) -> &'a mut W {
            self.variant(CLKSRCW::SELECTS_THE_INTERNAL)
        }
        # [ doc = "Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s." ]
        # [ inline ( always ) ]
        pub fn selects_the_main_osc(self) -> &'a mut W {
            self.variant(CLKSRCW::SELECTS_THE_MAIN_OSC)
        }
        # [ doc = "Selects the RTC oscillator as the PLL0 clock source." ]
        # [ inline ( always ) ]
        pub fn selects_the_rtc_osci(self) -> &'a mut W {
            self.variant(CLKSRCW::SELECTS_THE_RTC_OSCI)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device." ]
        # [ inline ( always ) ]
        pub fn clksrc(&self) -> CLKSRCR {
            CLKSRCR::_from({
                               const MASK: u8 = 3;
                               const OFFSET: u8 = 0;
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
        # [ doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device." ]
        # [ inline ( always ) ]
        pub fn clksrc(&mut self) -> _CLKSRCW {
            _CLKSRCW { w: self }
        }
    }
}
# [ doc = "Allows clearing the current CAN channel sleep state as well as reading that state." ]
pub struct CANSLEEPCLR {
    register: VolatileCell<u32>,
}
# [ doc = "Allows clearing the current CAN channel sleep state as well as reading that state." ]
pub mod cansleepclr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CANSLEEPCLR {
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
    pub struct CAN1SLEEPR {
        bits: bool,
    }
    impl CAN1SLEEPR {
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
    pub struct CAN2SLEEPR {
        bits: bool,
    }
    impl CAN2SLEEPR {
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
    pub struct _CAN1SLEEPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAN1SLEEPW<'a> {
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
    pub struct _CAN2SLEEPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAN2SLEEPW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1." ]
        # [ inline ( always ) ]
        pub fn can1sleep(&self) -> CAN1SLEEPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAN1SLEEPR { bits }
        }
        # [ doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2." ]
        # [ inline ( always ) ]
        pub fn can2sleep(&self) -> CAN2SLEEPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAN2SLEEPR { bits }
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
        # [ doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1." ]
        # [ inline ( always ) ]
        pub fn can1sleep(&mut self) -> _CAN1SLEEPW {
            _CAN1SLEEPW { w: self }
        }
        # [ doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2." ]
        # [ inline ( always ) ]
        pub fn can2sleep(&mut self) -> _CAN2SLEEPW {
            _CAN2SLEEPW { w: self }
        }
    }
}
# [ doc = "Allows reading the wake-up state of the CAN channels." ]
pub struct CANWAKEFLAGS {
    register: VolatileCell<u32>,
}
# [ doc = "Allows reading the wake-up state of the CAN channels." ]
pub mod canwakeflags {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CANWAKEFLAGS {
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
    pub struct CAN1WAKER {
        bits: bool,
    }
    impl CAN1WAKER {
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
    pub struct CAN2WAKER {
        bits: bool,
    }
    impl CAN2WAKER {
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
    pub struct _CAN1WAKEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAN1WAKEW<'a> {
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
    pub struct _CAN2WAKEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAN2WAKEW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit." ]
        # [ inline ( always ) ]
        pub fn can1wake(&self) -> CAN1WAKER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAN1WAKER { bits }
        }
        # [ doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit." ]
        # [ inline ( always ) ]
        pub fn can2wake(&self) -> CAN2WAKER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAN2WAKER { bits }
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
        # [ doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit." ]
        # [ inline ( always ) ]
        pub fn can1wake(&mut self) -> _CAN1WAKEW {
            _CAN1WAKEW { w: self }
        }
        # [ doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit." ]
        # [ inline ( always ) ]
        pub fn can2wake(&mut self) -> _CAN2WAKEW {
            _CAN2WAKEW { w: self }
        }
    }
}
# [ doc = "External Interrupt Flag Register" ]
pub struct EXTINT {
    register: VolatileCell<u32>,
}
# [ doc = "External Interrupt Flag Register" ]
pub mod extint {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EXTINT {
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
    pub struct EINT0R {
        bits: bool,
    }
    impl EINT0R {
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
    pub struct EINT1R {
        bits: bool,
    }
    impl EINT1R {
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
    pub struct EINT2R {
        bits: bool,
    }
    impl EINT2R {
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
    pub struct EINT3R {
        bits: bool,
    }
    impl EINT3R {
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
    pub struct _EINT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EINT0W<'a> {
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
    pub struct _EINT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EINT1W<'a> {
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
    pub struct _EINT2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EINT2W<'a> {
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
    pub struct _EINT3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EINT3W<'a> {
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
        # [ doc = "Bit 0 - In level-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state." ]
        # [ inline ( always ) ]
        pub fn eint0(&self) -> EINT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EINT0R { bits }
        }
        # [ doc = "Bit 1 - In level-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state." ]
        # [ inline ( always ) ]
        pub fn eint1(&self) -> EINT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EINT1R { bits }
        }
        # [ doc = "Bit 2 - In level-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state." ]
        # [ inline ( always ) ]
        pub fn eint2(&self) -> EINT2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EINT2R { bits }
        }
        # [ doc = "Bit 3 - In level-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state." ]
        # [ inline ( always ) ]
        pub fn eint3(&self) -> EINT3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EINT3R { bits }
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
        # [ doc = "Bit 0 - In level-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state." ]
        # [ inline ( always ) ]
        pub fn eint0(&mut self) -> _EINT0W {
            _EINT0W { w: self }
        }
        # [ doc = "Bit 1 - In level-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state." ]
        # [ inline ( always ) ]
        pub fn eint1(&mut self) -> _EINT1W {
            _EINT1W { w: self }
        }
        # [ doc = "Bit 2 - In level-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state." ]
        # [ inline ( always ) ]
        pub fn eint2(&mut self) -> _EINT2W {
            _EINT2W { w: self }
        }
        # [ doc = "Bit 3 - In level-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state." ]
        # [ inline ( always ) ]
        pub fn eint3(&mut self) -> _EINT3W {
            _EINT3W { w: self }
        }
    }
}
# [ doc = "External Interrupt Mode register" ]
pub struct EXTMODE {
    register: VolatileCell<u32>,
}
# [ doc = "External Interrupt Mode register" ]
pub mod extmode {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EXTMODE {
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
    # [ doc = "Possible values of the field `EXTMODE0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EXTMODE0R {
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT0." ]
        LEVEL_SENSITIVE,
        # [ doc = "Edge-sensitive. EINT0 is edge sensitive." ]
        EDGE_SENSITIVE,
    }
    impl EXTMODE0R {
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
                EXTMODE0R::LEVEL_SENSITIVE => false,
                EXTMODE0R::EDGE_SENSITIVE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> EXTMODE0R {
            match value {
                false => EXTMODE0R::LEVEL_SENSITIVE,
                true => EXTMODE0R::EDGE_SENSITIVE,
            }
        }
        # [ doc = "Checks if the value of the field is `LEVEL_SENSITIVE`" ]
        # [ inline ( always ) ]
        pub fn is_level_sensitive(&self) -> bool {
            *self == EXTMODE0R::LEVEL_SENSITIVE
        }
        # [ doc = "Checks if the value of the field is `EDGE_SENSITIVE`" ]
        # [ inline ( always ) ]
        pub fn is_edge_sensitive(&self) -> bool {
            *self == EXTMODE0R::EDGE_SENSITIVE
        }
    }
    # [ doc = "Possible values of the field `EXTMODE1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EXTMODE1R {
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT1." ]
        LEVEL_SENSITIVE,
        # [ doc = "Edge-sensitive. EINT1 is edge sensitive." ]
        EDGE_SENSITIVE,
    }
    impl EXTMODE1R {
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
                EXTMODE1R::LEVEL_SENSITIVE => false,
                EXTMODE1R::EDGE_SENSITIVE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> EXTMODE1R {
            match value {
                false => EXTMODE1R::LEVEL_SENSITIVE,
                true => EXTMODE1R::EDGE_SENSITIVE,
            }
        }
        # [ doc = "Checks if the value of the field is `LEVEL_SENSITIVE`" ]
        # [ inline ( always ) ]
        pub fn is_level_sensitive(&self) -> bool {
            *self == EXTMODE1R::LEVEL_SENSITIVE
        }
        # [ doc = "Checks if the value of the field is `EDGE_SENSITIVE`" ]
        # [ inline ( always ) ]
        pub fn is_edge_sensitive(&self) -> bool {
            *self == EXTMODE1R::EDGE_SENSITIVE
        }
    }
    # [ doc = "Possible values of the field `EXTMODE2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EXTMODE2R {
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT2." ]
        LEVEL_SENSITIVE,
        # [ doc = "Edge-sensitive. EINT2 is edge sensitive." ]
        EDGE_SENSITIVE,
    }
    impl EXTMODE2R {
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
                EXTMODE2R::LEVEL_SENSITIVE => false,
                EXTMODE2R::EDGE_SENSITIVE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> EXTMODE2R {
            match value {
                false => EXTMODE2R::LEVEL_SENSITIVE,
                true => EXTMODE2R::EDGE_SENSITIVE,
            }
        }
        # [ doc = "Checks if the value of the field is `LEVEL_SENSITIVE`" ]
        # [ inline ( always ) ]
        pub fn is_level_sensitive(&self) -> bool {
            *self == EXTMODE2R::LEVEL_SENSITIVE
        }
        # [ doc = "Checks if the value of the field is `EDGE_SENSITIVE`" ]
        # [ inline ( always ) ]
        pub fn is_edge_sensitive(&self) -> bool {
            *self == EXTMODE2R::EDGE_SENSITIVE
        }
    }
    # [ doc = "Possible values of the field `EXTMODE3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EXTMODE3R {
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT3." ]
        LEVEL_SENSITIVE,
        # [ doc = "Edge-sensitive. EINT3 is edge sensitive." ]
        EDGE_SENSITIVE,
    }
    impl EXTMODE3R {
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
                EXTMODE3R::LEVEL_SENSITIVE => false,
                EXTMODE3R::EDGE_SENSITIVE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> EXTMODE3R {
            match value {
                false => EXTMODE3R::LEVEL_SENSITIVE,
                true => EXTMODE3R::EDGE_SENSITIVE,
            }
        }
        # [ doc = "Checks if the value of the field is `LEVEL_SENSITIVE`" ]
        # [ inline ( always ) ]
        pub fn is_level_sensitive(&self) -> bool {
            *self == EXTMODE3R::LEVEL_SENSITIVE
        }
        # [ doc = "Checks if the value of the field is `EDGE_SENSITIVE`" ]
        # [ inline ( always ) ]
        pub fn is_edge_sensitive(&self) -> bool {
            *self == EXTMODE3R::EDGE_SENSITIVE
        }
    }
    # [ doc = "Values that can be written to the field `EXTMODE0`" ]
    pub enum EXTMODE0W {
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT0." ]
        LEVEL_SENSITIVE,
        # [ doc = "Edge-sensitive. EINT0 is edge sensitive." ]
        EDGE_SENSITIVE,
    }
    impl EXTMODE0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                EXTMODE0W::LEVEL_SENSITIVE => false,
                EXTMODE0W::EDGE_SENSITIVE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTMODE0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTMODE0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EXTMODE0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT0." ]
        # [ inline ( always ) ]
        pub fn level_sensitive(self) -> &'a mut W {
            self.variant(EXTMODE0W::LEVEL_SENSITIVE)
        }
        # [ doc = "Edge-sensitive. EINT0 is edge sensitive." ]
        # [ inline ( always ) ]
        pub fn edge_sensitive(self) -> &'a mut W {
            self.variant(EXTMODE0W::EDGE_SENSITIVE)
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
    # [ doc = "Values that can be written to the field `EXTMODE1`" ]
    pub enum EXTMODE1W {
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT1." ]
        LEVEL_SENSITIVE,
        # [ doc = "Edge-sensitive. EINT1 is edge sensitive." ]
        EDGE_SENSITIVE,
    }
    impl EXTMODE1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                EXTMODE1W::LEVEL_SENSITIVE => false,
                EXTMODE1W::EDGE_SENSITIVE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTMODE1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTMODE1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EXTMODE1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT1." ]
        # [ inline ( always ) ]
        pub fn level_sensitive(self) -> &'a mut W {
            self.variant(EXTMODE1W::LEVEL_SENSITIVE)
        }
        # [ doc = "Edge-sensitive. EINT1 is edge sensitive." ]
        # [ inline ( always ) ]
        pub fn edge_sensitive(self) -> &'a mut W {
            self.variant(EXTMODE1W::EDGE_SENSITIVE)
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
    # [ doc = "Values that can be written to the field `EXTMODE2`" ]
    pub enum EXTMODE2W {
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT2." ]
        LEVEL_SENSITIVE,
        # [ doc = "Edge-sensitive. EINT2 is edge sensitive." ]
        EDGE_SENSITIVE,
    }
    impl EXTMODE2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                EXTMODE2W::LEVEL_SENSITIVE => false,
                EXTMODE2W::EDGE_SENSITIVE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTMODE2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTMODE2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EXTMODE2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT2." ]
        # [ inline ( always ) ]
        pub fn level_sensitive(self) -> &'a mut W {
            self.variant(EXTMODE2W::LEVEL_SENSITIVE)
        }
        # [ doc = "Edge-sensitive. EINT2 is edge sensitive." ]
        # [ inline ( always ) ]
        pub fn edge_sensitive(self) -> &'a mut W {
            self.variant(EXTMODE2W::EDGE_SENSITIVE)
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
    # [ doc = "Values that can be written to the field `EXTMODE3`" ]
    pub enum EXTMODE3W {
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT3." ]
        LEVEL_SENSITIVE,
        # [ doc = "Edge-sensitive. EINT3 is edge sensitive." ]
        EDGE_SENSITIVE,
    }
    impl EXTMODE3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                EXTMODE3W::LEVEL_SENSITIVE => false,
                EXTMODE3W::EDGE_SENSITIVE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTMODE3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTMODE3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EXTMODE3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Level-sensitive. Level-sensitivity is selected for EINT3." ]
        # [ inline ( always ) ]
        pub fn level_sensitive(self) -> &'a mut W {
            self.variant(EXTMODE3W::LEVEL_SENSITIVE)
        }
        # [ doc = "Edge-sensitive. EINT3 is edge sensitive." ]
        # [ inline ( always ) ]
        pub fn edge_sensitive(self) -> &'a mut W {
            self.variant(EXTMODE3W::EDGE_SENSITIVE)
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
        # [ doc = "Bit 0 - External interrupt 0 EINT0 mode." ]
        # [ inline ( always ) ]
        pub fn extmode0(&self) -> EXTMODE0R {
            EXTMODE0R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 0;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 1 - External interrupt 1 EINT1 mode." ]
        # [ inline ( always ) ]
        pub fn extmode1(&self) -> EXTMODE1R {
            EXTMODE1R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 1;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 2 - External interrupt 2 EINT2 mode." ]
        # [ inline ( always ) ]
        pub fn extmode2(&self) -> EXTMODE2R {
            EXTMODE2R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 2;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 3 - External interrupt 3 EINT3 mode." ]
        # [ inline ( always ) ]
        pub fn extmode3(&self) -> EXTMODE3R {
            EXTMODE3R::_from({
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
        # [ doc = "Bit 0 - External interrupt 0 EINT0 mode." ]
        # [ inline ( always ) ]
        pub fn extmode0(&mut self) -> _EXTMODE0W {
            _EXTMODE0W { w: self }
        }
        # [ doc = "Bit 1 - External interrupt 1 EINT1 mode." ]
        # [ inline ( always ) ]
        pub fn extmode1(&mut self) -> _EXTMODE1W {
            _EXTMODE1W { w: self }
        }
        # [ doc = "Bit 2 - External interrupt 2 EINT2 mode." ]
        # [ inline ( always ) ]
        pub fn extmode2(&mut self) -> _EXTMODE2W {
            _EXTMODE2W { w: self }
        }
        # [ doc = "Bit 3 - External interrupt 3 EINT3 mode." ]
        # [ inline ( always ) ]
        pub fn extmode3(&mut self) -> _EXTMODE3W {
            _EXTMODE3W { w: self }
        }
    }
}
# [ doc = "External Interrupt Polarity Register" ]
pub struct EXTPOLAR {
    register: VolatileCell<u32>,
}
# [ doc = "External Interrupt Polarity Register" ]
pub mod extpolar {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EXTPOLAR {
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
    # [ doc = "Possible values of the field `EXTPOLAR0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EXTPOLAR0R {
        # [ doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)." ]
        FALLING_EDGE,
        # [ doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)." ]
        RISING_EDGE,
    }
    impl EXTPOLAR0R {
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
                EXTPOLAR0R::FALLING_EDGE => false,
                EXTPOLAR0R::RISING_EDGE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> EXTPOLAR0R {
            match value {
                false => EXTPOLAR0R::FALLING_EDGE,
                true => EXTPOLAR0R::RISING_EDGE,
            }
        }
        # [ doc = "Checks if the value of the field is `FALLING_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_falling_edge(&self) -> bool {
            *self == EXTPOLAR0R::FALLING_EDGE
        }
        # [ doc = "Checks if the value of the field is `RISING_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_rising_edge(&self) -> bool {
            *self == EXTPOLAR0R::RISING_EDGE
        }
    }
    # [ doc = "Possible values of the field `EXTPOLAR1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EXTPOLAR1R {
        # [ doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)." ]
        FALLING_EDGE,
        # [ doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)." ]
        RISING_EDGE,
    }
    impl EXTPOLAR1R {
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
                EXTPOLAR1R::FALLING_EDGE => false,
                EXTPOLAR1R::RISING_EDGE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> EXTPOLAR1R {
            match value {
                false => EXTPOLAR1R::FALLING_EDGE,
                true => EXTPOLAR1R::RISING_EDGE,
            }
        }
        # [ doc = "Checks if the value of the field is `FALLING_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_falling_edge(&self) -> bool {
            *self == EXTPOLAR1R::FALLING_EDGE
        }
        # [ doc = "Checks if the value of the field is `RISING_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_rising_edge(&self) -> bool {
            *self == EXTPOLAR1R::RISING_EDGE
        }
    }
    # [ doc = "Possible values of the field `EXTPOLAR2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EXTPOLAR2R {
        # [ doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)." ]
        FALLING_EDGE,
        # [ doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)." ]
        RISING_EDGE,
    }
    impl EXTPOLAR2R {
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
                EXTPOLAR2R::FALLING_EDGE => false,
                EXTPOLAR2R::RISING_EDGE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> EXTPOLAR2R {
            match value {
                false => EXTPOLAR2R::FALLING_EDGE,
                true => EXTPOLAR2R::RISING_EDGE,
            }
        }
        # [ doc = "Checks if the value of the field is `FALLING_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_falling_edge(&self) -> bool {
            *self == EXTPOLAR2R::FALLING_EDGE
        }
        # [ doc = "Checks if the value of the field is `RISING_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_rising_edge(&self) -> bool {
            *self == EXTPOLAR2R::RISING_EDGE
        }
    }
    # [ doc = "Possible values of the field `EXTPOLAR3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EXTPOLAR3R {
        # [ doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)." ]
        FALLING_EDGE,
        # [ doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)." ]
        RISING_EDGE,
    }
    impl EXTPOLAR3R {
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
                EXTPOLAR3R::FALLING_EDGE => false,
                EXTPOLAR3R::RISING_EDGE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> EXTPOLAR3R {
            match value {
                false => EXTPOLAR3R::FALLING_EDGE,
                true => EXTPOLAR3R::RISING_EDGE,
            }
        }
        # [ doc = "Checks if the value of the field is `FALLING_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_falling_edge(&self) -> bool {
            *self == EXTPOLAR3R::FALLING_EDGE
        }
        # [ doc = "Checks if the value of the field is `RISING_EDGE`" ]
        # [ inline ( always ) ]
        pub fn is_rising_edge(&self) -> bool {
            *self == EXTPOLAR3R::RISING_EDGE
        }
    }
    # [ doc = "Values that can be written to the field `EXTPOLAR0`" ]
    pub enum EXTPOLAR0W {
        # [ doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)." ]
        FALLING_EDGE,
        # [ doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)." ]
        RISING_EDGE,
    }
    impl EXTPOLAR0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                EXTPOLAR0W::FALLING_EDGE => false,
                EXTPOLAR0W::RISING_EDGE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTPOLAR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTPOLAR0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EXTPOLAR0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)." ]
        # [ inline ( always ) ]
        pub fn falling_edge(self) -> &'a mut W {
            self.variant(EXTPOLAR0W::FALLING_EDGE)
        }
        # [ doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)." ]
        # [ inline ( always ) ]
        pub fn rising_edge(self) -> &'a mut W {
            self.variant(EXTPOLAR0W::RISING_EDGE)
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
    # [ doc = "Values that can be written to the field `EXTPOLAR1`" ]
    pub enum EXTPOLAR1W {
        # [ doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)." ]
        FALLING_EDGE,
        # [ doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)." ]
        RISING_EDGE,
    }
    impl EXTPOLAR1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                EXTPOLAR1W::FALLING_EDGE => false,
                EXTPOLAR1W::RISING_EDGE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTPOLAR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTPOLAR1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EXTPOLAR1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)." ]
        # [ inline ( always ) ]
        pub fn falling_edge(self) -> &'a mut W {
            self.variant(EXTPOLAR1W::FALLING_EDGE)
        }
        # [ doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)." ]
        # [ inline ( always ) ]
        pub fn rising_edge(self) -> &'a mut W {
            self.variant(EXTPOLAR1W::RISING_EDGE)
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
    # [ doc = "Values that can be written to the field `EXTPOLAR2`" ]
    pub enum EXTPOLAR2W {
        # [ doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)." ]
        FALLING_EDGE,
        # [ doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)." ]
        RISING_EDGE,
    }
    impl EXTPOLAR2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                EXTPOLAR2W::FALLING_EDGE => false,
                EXTPOLAR2W::RISING_EDGE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTPOLAR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTPOLAR2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EXTPOLAR2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)." ]
        # [ inline ( always ) ]
        pub fn falling_edge(self) -> &'a mut W {
            self.variant(EXTPOLAR2W::FALLING_EDGE)
        }
        # [ doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)." ]
        # [ inline ( always ) ]
        pub fn rising_edge(self) -> &'a mut W {
            self.variant(EXTPOLAR2W::RISING_EDGE)
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
    # [ doc = "Values that can be written to the field `EXTPOLAR3`" ]
    pub enum EXTPOLAR3W {
        # [ doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)." ]
        FALLING_EDGE,
        # [ doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)." ]
        RISING_EDGE,
    }
    impl EXTPOLAR3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                EXTPOLAR3W::FALLING_EDGE => false,
                EXTPOLAR3W::RISING_EDGE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EXTPOLAR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTPOLAR3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EXTPOLAR3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)." ]
        # [ inline ( always ) ]
        pub fn falling_edge(self) -> &'a mut W {
            self.variant(EXTPOLAR3W::FALLING_EDGE)
        }
        # [ doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)." ]
        # [ inline ( always ) ]
        pub fn rising_edge(self) -> &'a mut W {
            self.variant(EXTPOLAR3W::RISING_EDGE)
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
        # [ doc = "Bit 0 - External interrupt 0 EINT0 polarity." ]
        # [ inline ( always ) ]
        pub fn extpolar0(&self) -> EXTPOLAR0R {
            EXTPOLAR0R::_from({
                                  const MASK: bool = true;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) != 0
                              })
        }
        # [ doc = "Bit 1 - External interrupt 1 EINT1 polarity." ]
        # [ inline ( always ) ]
        pub fn extpolar1(&self) -> EXTPOLAR1R {
            EXTPOLAR1R::_from({
                                  const MASK: bool = true;
                                  const OFFSET: u8 = 1;
                                  ((self.bits >> OFFSET) & MASK as u32) != 0
                              })
        }
        # [ doc = "Bit 2 - External interrupt 2 EINT2 polarity." ]
        # [ inline ( always ) ]
        pub fn extpolar2(&self) -> EXTPOLAR2R {
            EXTPOLAR2R::_from({
                                  const MASK: bool = true;
                                  const OFFSET: u8 = 2;
                                  ((self.bits >> OFFSET) & MASK as u32) != 0
                              })
        }
        # [ doc = "Bit 3 - External interrupt 3 EINT3 polarity." ]
        # [ inline ( always ) ]
        pub fn extpolar3(&self) -> EXTPOLAR3R {
            EXTPOLAR3R::_from({
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
        # [ doc = "Bit 0 - External interrupt 0 EINT0 polarity." ]
        # [ inline ( always ) ]
        pub fn extpolar0(&mut self) -> _EXTPOLAR0W {
            _EXTPOLAR0W { w: self }
        }
        # [ doc = "Bit 1 - External interrupt 1 EINT1 polarity." ]
        # [ inline ( always ) ]
        pub fn extpolar1(&mut self) -> _EXTPOLAR1W {
            _EXTPOLAR1W { w: self }
        }
        # [ doc = "Bit 2 - External interrupt 2 EINT2 polarity." ]
        # [ inline ( always ) ]
        pub fn extpolar2(&mut self) -> _EXTPOLAR2W {
            _EXTPOLAR2W { w: self }
        }
        # [ doc = "Bit 3 - External interrupt 3 EINT3 polarity." ]
        # [ inline ( always ) ]
        pub fn extpolar3(&mut self) -> _EXTPOLAR3W {
            _EXTPOLAR3W { w: self }
        }
    }
}
# [ doc = "Reset Source Identification Register" ]
pub struct RSID {
    register: VolatileCell<u32>,
}
# [ doc = "Reset Source Identification Register" ]
pub mod rsid {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RSID {
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
    pub struct PORR {
        bits: bool,
    }
    impl PORR {
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
    pub struct EXTRR {
        bits: bool,
    }
    impl EXTRR {
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
    pub struct WDTRR {
        bits: bool,
    }
    impl WDTRR {
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
    pub struct BODRR {
        bits: bool,
    }
    impl BODRR {
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
    pub struct _PORW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PORW<'a> {
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
    pub struct _EXTRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTRW<'a> {
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
    pub struct _WDTRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WDTRW<'a> {
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
    pub struct _BODRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BODRW<'a> {
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
        # [ doc = "Bit 0 - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset." ]
        # [ inline ( always ) ]
        pub fn por(&self) -> PORR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PORR { bits }
        }
        # [ doc = "Bit 1 - Assertion of the RESET signal sets this bit. This bit is cleared only by software or POR." ]
        # [ inline ( always ) ]
        pub fn extr(&self) -> EXTRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTRR { bits }
        }
        # [ doc = "Bit 2 - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR." ]
        # [ inline ( always ) ]
        pub fn wdtr(&self) -> WDTRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WDTRR { bits }
        }
        # [ doc = "Bit 3 - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not." ]
        # [ inline ( always ) ]
        pub fn bodr(&self) -> BODRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BODRR { bits }
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
        # [ doc = "Bit 0 - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset." ]
        # [ inline ( always ) ]
        pub fn por(&mut self) -> _PORW {
            _PORW { w: self }
        }
        # [ doc = "Bit 1 - Assertion of the RESET signal sets this bit. This bit is cleared only by software or POR." ]
        # [ inline ( always ) ]
        pub fn extr(&mut self) -> _EXTRW {
            _EXTRW { w: self }
        }
        # [ doc = "Bit 2 - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR." ]
        # [ inline ( always ) ]
        pub fn wdtr(&mut self) -> _WDTRW {
            _WDTRW { w: self }
        }
        # [ doc = "Bit 3 - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not." ]
        # [ inline ( always ) ]
        pub fn bodr(&mut self) -> _BODRW {
            _BODRW { w: self }
        }
    }
}
# [ doc = "System control and status" ]
pub struct SCS {
    register: VolatileCell<u32>,
}
# [ doc = "System control and status" ]
pub mod scs {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SCS {
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
    # [ doc = "Possible values of the field `OSCRANGE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OSCRANGER {
        # [ doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz." ]
        LOW,
        # [ doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz." ]
        HIGH,
    }
    impl OSCRANGER {
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
                OSCRANGER::LOW => false,
                OSCRANGER::HIGH => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> OSCRANGER {
            match value {
                false => OSCRANGER::LOW,
                true => OSCRANGER::HIGH,
            }
        }
        # [ doc = "Checks if the value of the field is `LOW`" ]
        # [ inline ( always ) ]
        pub fn is_low(&self) -> bool {
            *self == OSCRANGER::LOW
        }
        # [ doc = "Checks if the value of the field is `HIGH`" ]
        # [ inline ( always ) ]
        pub fn is_high(&self) -> bool {
            *self == OSCRANGER::HIGH
        }
    }
    # [ doc = "Possible values of the field `OSCEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OSCENR {
        # [ doc = "Disabled. The main oscillator is disabled." ]
        DISABLED,
        # [ doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins." ]
        ENABLED,
    }
    impl OSCENR {
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
                OSCENR::DISABLED => false,
                OSCENR::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> OSCENR {
            match value {
                false => OSCENR::DISABLED,
                true => OSCENR::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == OSCENR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == OSCENR::ENABLED
        }
    }
    # [ doc = "Possible values of the field `OSCSTAT`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OSCSTATR {
        # [ doc = "Not ready. The main oscillator is not ready to be used as a clock source." ]
        NOT_READY,
        # [ doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit." ]
        READY,
    }
    impl OSCSTATR {
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
                OSCSTATR::NOT_READY => false,
                OSCSTATR::READY => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> OSCSTATR {
            match value {
                false => OSCSTATR::NOT_READY,
                true => OSCSTATR::READY,
            }
        }
        # [ doc = "Checks if the value of the field is `NOT_READY`" ]
        # [ inline ( always ) ]
        pub fn is_not_ready(&self) -> bool {
            *self == OSCSTATR::NOT_READY
        }
        # [ doc = "Checks if the value of the field is `READY`" ]
        # [ inline ( always ) ]
        pub fn is_ready(&self) -> bool {
            *self == OSCSTATR::READY
        }
    }
    # [ doc = "Values that can be written to the field `OSCRANGE`" ]
    pub enum OSCRANGEW {
        # [ doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz." ]
        LOW,
        # [ doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz." ]
        HIGH,
    }
    impl OSCRANGEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                OSCRANGEW::LOW => false,
                OSCRANGEW::HIGH => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OSCRANGEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSCRANGEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: OSCRANGEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz." ]
        # [ inline ( always ) ]
        pub fn low(self) -> &'a mut W {
            self.variant(OSCRANGEW::LOW)
        }
        # [ doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz." ]
        # [ inline ( always ) ]
        pub fn high(self) -> &'a mut W {
            self.variant(OSCRANGEW::HIGH)
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
    # [ doc = "Values that can be written to the field `OSCEN`" ]
    pub enum OSCENW {
        # [ doc = "Disabled. The main oscillator is disabled." ]
        DISABLED,
        # [ doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins." ]
        ENABLED,
    }
    impl OSCENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                OSCENW::DISABLED => false,
                OSCENW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OSCENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSCENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: OSCENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled. The main oscillator is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(OSCENW::DISABLED)
        }
        # [ doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(OSCENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `OSCSTAT`" ]
    pub enum OSCSTATW {
        # [ doc = "Not ready. The main oscillator is not ready to be used as a clock source." ]
        NOT_READY,
        # [ doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit." ]
        READY,
    }
    impl OSCSTATW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                OSCSTATW::NOT_READY => false,
                OSCSTATW::READY => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OSCSTATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSCSTATW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: OSCSTATW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Not ready. The main oscillator is not ready to be used as a clock source." ]
        # [ inline ( always ) ]
        pub fn not_ready(self) -> &'a mut W {
            self.variant(OSCSTATW::NOT_READY)
        }
        # [ doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit." ]
        # [ inline ( always ) ]
        pub fn ready(self) -> &'a mut W {
            self.variant(OSCSTATW::READY)
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
        # [ doc = "Bit 4 - Main oscillator range select." ]
        # [ inline ( always ) ]
        pub fn oscrange(&self) -> OSCRANGER {
            OSCRANGER::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 4;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 5 - Main oscillator enable." ]
        # [ inline ( always ) ]
        pub fn oscen(&self) -> OSCENR {
            OSCENR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 5;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 6 - Main oscillator status." ]
        # [ inline ( always ) ]
        pub fn oscstat(&self) -> OSCSTATR {
            OSCSTATR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 6;
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
        # [ doc = "Bit 4 - Main oscillator range select." ]
        # [ inline ( always ) ]
        pub fn oscrange(&mut self) -> _OSCRANGEW {
            _OSCRANGEW { w: self }
        }
        # [ doc = "Bit 5 - Main oscillator enable." ]
        # [ inline ( always ) ]
        pub fn oscen(&mut self) -> _OSCENW {
            _OSCENW { w: self }
        }
        # [ doc = "Bit 6 - Main oscillator status." ]
        # [ inline ( always ) ]
        pub fn oscstat(&mut self) -> _OSCSTATW {
            _OSCSTATW { w: self }
        }
    }
}
# [ doc = "Peripheral Clock Selection register 0." ]
pub struct PCLKSEL0 {
    register: VolatileCell<u32>,
}
# [ doc = "Peripheral Clock Selection register 0." ]
pub mod pclksel0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PCLKSEL0 {
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
    # [ doc = "Possible values of the field `PCLK_WDT`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_WDTR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_WDTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_WDTR::CCLK_DIV_4 => 0,
                PCLK_WDTR::CCLK => 1,
                PCLK_WDTR::CCLK_DIV_2 => 2,
                PCLK_WDTR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_WDTR {
            match value {
                0 => PCLK_WDTR::CCLK_DIV_4,
                1 => PCLK_WDTR::CCLK,
                2 => PCLK_WDTR::CCLK_DIV_2,
                3 => PCLK_WDTR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_WDTR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_WDTR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_WDTR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_WDTR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_TIMER0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_TIMER0R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_TIMER0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_TIMER0R::CCLK_DIV_4 => 0,
                PCLK_TIMER0R::CCLK => 1,
                PCLK_TIMER0R::CCLK_DIV_2 => 2,
                PCLK_TIMER0R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_TIMER0R {
            match value {
                0 => PCLK_TIMER0R::CCLK_DIV_4,
                1 => PCLK_TIMER0R::CCLK,
                2 => PCLK_TIMER0R::CCLK_DIV_2,
                3 => PCLK_TIMER0R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_TIMER0R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_TIMER0R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_TIMER0R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_TIMER0R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_TIMER1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_TIMER1R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_TIMER1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_TIMER1R::CCLK_DIV_4 => 0,
                PCLK_TIMER1R::CCLK => 1,
                PCLK_TIMER1R::CCLK_DIV_2 => 2,
                PCLK_TIMER1R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_TIMER1R {
            match value {
                0 => PCLK_TIMER1R::CCLK_DIV_4,
                1 => PCLK_TIMER1R::CCLK,
                2 => PCLK_TIMER1R::CCLK_DIV_2,
                3 => PCLK_TIMER1R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_TIMER1R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_TIMER1R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_TIMER1R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_TIMER1R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_UART0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_UART0R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_UART0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_UART0R::CCLK_DIV_4 => 0,
                PCLK_UART0R::CCLK => 1,
                PCLK_UART0R::CCLK_DIV_2 => 2,
                PCLK_UART0R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_UART0R {
            match value {
                0 => PCLK_UART0R::CCLK_DIV_4,
                1 => PCLK_UART0R::CCLK,
                2 => PCLK_UART0R::CCLK_DIV_2,
                3 => PCLK_UART0R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_UART0R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_UART0R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_UART0R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_UART0R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_UART1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_UART1R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_UART1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_UART1R::CCLK_DIV_4 => 0,
                PCLK_UART1R::CCLK => 1,
                PCLK_UART1R::CCLK_DIV_2 => 2,
                PCLK_UART1R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_UART1R {
            match value {
                0 => PCLK_UART1R::CCLK_DIV_4,
                1 => PCLK_UART1R::CCLK,
                2 => PCLK_UART1R::CCLK_DIV_2,
                3 => PCLK_UART1R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_UART1R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_UART1R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_UART1R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_UART1R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_PWM1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_PWM1R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_PWM1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_PWM1R::CCLK_DIV_4 => 0,
                PCLK_PWM1R::CCLK => 1,
                PCLK_PWM1R::CCLK_DIV_2 => 2,
                PCLK_PWM1R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_PWM1R {
            match value {
                0 => PCLK_PWM1R::CCLK_DIV_4,
                1 => PCLK_PWM1R::CCLK,
                2 => PCLK_PWM1R::CCLK_DIV_2,
                3 => PCLK_PWM1R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_PWM1R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_PWM1R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_PWM1R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_PWM1R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_I2C0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_I2C0R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_I2C0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_I2C0R::CCLK_DIV_4 => 0,
                PCLK_I2C0R::CCLK => 1,
                PCLK_I2C0R::CCLK_DIV_2 => 2,
                PCLK_I2C0R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_I2C0R {
            match value {
                0 => PCLK_I2C0R::CCLK_DIV_4,
                1 => PCLK_I2C0R::CCLK,
                2 => PCLK_I2C0R::CCLK_DIV_2,
                3 => PCLK_I2C0R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_I2C0R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_I2C0R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_I2C0R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_I2C0R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_SPI`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_SPIR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_SPIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_SPIR::CCLK_DIV_4 => 0,
                PCLK_SPIR::CCLK => 1,
                PCLK_SPIR::CCLK_DIV_2 => 2,
                PCLK_SPIR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_SPIR {
            match value {
                0 => PCLK_SPIR::CCLK_DIV_4,
                1 => PCLK_SPIR::CCLK,
                2 => PCLK_SPIR::CCLK_DIV_2,
                3 => PCLK_SPIR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_SPIR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_SPIR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_SPIR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_SPIR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_SSP1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_SSP1R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_SSP1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_SSP1R::CCLK_DIV_4 => 0,
                PCLK_SSP1R::CCLK => 1,
                PCLK_SSP1R::CCLK_DIV_2 => 2,
                PCLK_SSP1R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_SSP1R {
            match value {
                0 => PCLK_SSP1R::CCLK_DIV_4,
                1 => PCLK_SSP1R::CCLK,
                2 => PCLK_SSP1R::CCLK_DIV_2,
                3 => PCLK_SSP1R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_SSP1R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_SSP1R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_SSP1R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_SSP1R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_DAC`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_DACR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_DACR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_DACR::CCLK_DIV_4 => 0,
                PCLK_DACR::CCLK => 1,
                PCLK_DACR::CCLK_DIV_2 => 2,
                PCLK_DACR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_DACR {
            match value {
                0 => PCLK_DACR::CCLK_DIV_4,
                1 => PCLK_DACR::CCLK,
                2 => PCLK_DACR::CCLK_DIV_2,
                3 => PCLK_DACR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_DACR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_DACR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_DACR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_DACR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_ADC`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_ADCR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_ADCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_ADCR::CCLK_DIV_4 => 0,
                PCLK_ADCR::CCLK => 1,
                PCLK_ADCR::CCLK_DIV_2 => 2,
                PCLK_ADCR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_ADCR {
            match value {
                0 => PCLK_ADCR::CCLK_DIV_4,
                1 => PCLK_ADCR::CCLK,
                2 => PCLK_ADCR::CCLK_DIV_2,
                3 => PCLK_ADCR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_ADCR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_ADCR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_ADCR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_ADCR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_CAN1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_CAN1R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 6. PCLK_peripheral = CCLK/6." ]
        CCLK_DIV_6,
    }
    impl PCLK_CAN1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_CAN1R::CCLK_DIV_4 => 0,
                PCLK_CAN1R::CCLK => 1,
                PCLK_CAN1R::CCLK_DIV_2 => 2,
                PCLK_CAN1R::CCLK_DIV_6 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_CAN1R {
            match value {
                0 => PCLK_CAN1R::CCLK_DIV_4,
                1 => PCLK_CAN1R::CCLK,
                2 => PCLK_CAN1R::CCLK_DIV_2,
                3 => PCLK_CAN1R::CCLK_DIV_6,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_CAN1R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_CAN1R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_CAN1R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_6`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_6(&self) -> bool {
            *self == PCLK_CAN1R::CCLK_DIV_6
        }
    }
    # [ doc = "Possible values of the field `PCLK_CAN2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_CAN2R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 6. PCLK_peripheral = CCLK/6," ]
        CCLK_DIV_6,
    }
    impl PCLK_CAN2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_CAN2R::CCLK_DIV_4 => 0,
                PCLK_CAN2R::CCLK => 1,
                PCLK_CAN2R::CCLK_DIV_2 => 2,
                PCLK_CAN2R::CCLK_DIV_6 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_CAN2R {
            match value {
                0 => PCLK_CAN2R::CCLK_DIV_4,
                1 => PCLK_CAN2R::CCLK,
                2 => PCLK_CAN2R::CCLK_DIV_2,
                3 => PCLK_CAN2R::CCLK_DIV_6,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_CAN2R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_CAN2R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_CAN2R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_6`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_6(&self) -> bool {
            *self == PCLK_CAN2R::CCLK_DIV_6
        }
    }
    # [ doc = "Possible values of the field `PCLK_ACF`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_ACFR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 6. PCLK_peripheral = CCLK/6" ]
        CCLK_DIV_6,
    }
    impl PCLK_ACFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_ACFR::CCLK_DIV_4 => 0,
                PCLK_ACFR::CCLK => 1,
                PCLK_ACFR::CCLK_DIV_2 => 2,
                PCLK_ACFR::CCLK_DIV_6 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_ACFR {
            match value {
                0 => PCLK_ACFR::CCLK_DIV_4,
                1 => PCLK_ACFR::CCLK,
                2 => PCLK_ACFR::CCLK_DIV_2,
                3 => PCLK_ACFR::CCLK_DIV_6,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_ACFR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_ACFR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_ACFR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_6`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_6(&self) -> bool {
            *self == PCLK_ACFR::CCLK_DIV_6
        }
    }
    # [ doc = "Values that can be written to the field `PCLK_WDT`" ]
    pub enum PCLK_WDTW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_WDTW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_WDTW::CCLK_DIV_4 => 0,
                PCLK_WDTW::CCLK => 1,
                PCLK_WDTW::CCLK_DIV_2 => 2,
                PCLK_WDTW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_WDTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_WDTW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_WDTW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_WDTW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_WDTW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_WDTW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_WDTW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_TIMER0`" ]
    pub enum PCLK_TIMER0W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_TIMER0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_TIMER0W::CCLK_DIV_4 => 0,
                PCLK_TIMER0W::CCLK => 1,
                PCLK_TIMER0W::CCLK_DIV_2 => 2,
                PCLK_TIMER0W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_TIMER0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_TIMER0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_TIMER0W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_TIMER0W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_TIMER0W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_TIMER0W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_TIMER0W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_TIMER1`" ]
    pub enum PCLK_TIMER1W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_TIMER1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_TIMER1W::CCLK_DIV_4 => 0,
                PCLK_TIMER1W::CCLK => 1,
                PCLK_TIMER1W::CCLK_DIV_2 => 2,
                PCLK_TIMER1W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_TIMER1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_TIMER1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_TIMER1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_TIMER1W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_TIMER1W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_TIMER1W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_TIMER1W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_UART0`" ]
    pub enum PCLK_UART0W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_UART0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_UART0W::CCLK_DIV_4 => 0,
                PCLK_UART0W::CCLK => 1,
                PCLK_UART0W::CCLK_DIV_2 => 2,
                PCLK_UART0W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_UART0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_UART0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_UART0W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_UART0W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_UART0W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_UART0W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_UART0W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_UART1`" ]
    pub enum PCLK_UART1W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_UART1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_UART1W::CCLK_DIV_4 => 0,
                PCLK_UART1W::CCLK => 1,
                PCLK_UART1W::CCLK_DIV_2 => 2,
                PCLK_UART1W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_UART1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_UART1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_UART1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_UART1W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_UART1W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_UART1W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_UART1W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_PWM1`" ]
    pub enum PCLK_PWM1W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_PWM1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_PWM1W::CCLK_DIV_4 => 0,
                PCLK_PWM1W::CCLK => 1,
                PCLK_PWM1W::CCLK_DIV_2 => 2,
                PCLK_PWM1W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_PWM1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_PWM1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_PWM1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_PWM1W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_PWM1W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_PWM1W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_PWM1W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_I2C0`" ]
    pub enum PCLK_I2C0W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_I2C0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_I2C0W::CCLK_DIV_4 => 0,
                PCLK_I2C0W::CCLK => 1,
                PCLK_I2C0W::CCLK_DIV_2 => 2,
                PCLK_I2C0W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_I2C0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_I2C0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_I2C0W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_I2C0W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_I2C0W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_I2C0W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_I2C0W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_SPI`" ]
    pub enum PCLK_SPIW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_SPIW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_SPIW::CCLK_DIV_4 => 0,
                PCLK_SPIW::CCLK => 1,
                PCLK_SPIW::CCLK_DIV_2 => 2,
                PCLK_SPIW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_SPIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_SPIW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_SPIW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_SPIW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_SPIW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_SPIW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_SPIW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_SSP1`" ]
    pub enum PCLK_SSP1W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_SSP1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_SSP1W::CCLK_DIV_4 => 0,
                PCLK_SSP1W::CCLK => 1,
                PCLK_SSP1W::CCLK_DIV_2 => 2,
                PCLK_SSP1W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_SSP1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_SSP1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_SSP1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_SSP1W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_SSP1W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_SSP1W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_SSP1W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_DAC`" ]
    pub enum PCLK_DACW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_DACW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_DACW::CCLK_DIV_4 => 0,
                PCLK_DACW::CCLK => 1,
                PCLK_DACW::CCLK_DIV_2 => 2,
                PCLK_DACW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_DACW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_DACW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_DACW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_DACW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_DACW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_DACW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_DACW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_ADC`" ]
    pub enum PCLK_ADCW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_ADCW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_ADCW::CCLK_DIV_4 => 0,
                PCLK_ADCW::CCLK => 1,
                PCLK_ADCW::CCLK_DIV_2 => 2,
                PCLK_ADCW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_ADCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_ADCW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_ADCW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_ADCW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_ADCW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_ADCW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_ADCW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_CAN1`" ]
    pub enum PCLK_CAN1W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 6. PCLK_peripheral = CCLK/6." ]
        CCLK_DIV_6,
    }
    impl PCLK_CAN1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_CAN1W::CCLK_DIV_4 => 0,
                PCLK_CAN1W::CCLK => 1,
                PCLK_CAN1W::CCLK_DIV_2 => 2,
                PCLK_CAN1W::CCLK_DIV_6 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_CAN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_CAN1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_CAN1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_CAN1W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_CAN1W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_CAN1W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 6. PCLK_peripheral = CCLK/6." ]
        # [ inline ( always ) ]
        pub fn cclk_div_6(self) -> &'a mut W {
            self.variant(PCLK_CAN1W::CCLK_DIV_6)
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
    # [ doc = "Values that can be written to the field `PCLK_CAN2`" ]
    pub enum PCLK_CAN2W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 6. PCLK_peripheral = CCLK/6," ]
        CCLK_DIV_6,
    }
    impl PCLK_CAN2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_CAN2W::CCLK_DIV_4 => 0,
                PCLK_CAN2W::CCLK => 1,
                PCLK_CAN2W::CCLK_DIV_2 => 2,
                PCLK_CAN2W::CCLK_DIV_6 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_CAN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_CAN2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_CAN2W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_CAN2W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_CAN2W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_CAN2W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 6. PCLK_peripheral = CCLK/6," ]
        # [ inline ( always ) ]
        pub fn cclk_div_6(self) -> &'a mut W {
            self.variant(PCLK_CAN2W::CCLK_DIV_6)
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
    # [ doc = "Values that can be written to the field `PCLK_ACF`" ]
    pub enum PCLK_ACFW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 6. PCLK_peripheral = CCLK/6" ]
        CCLK_DIV_6,
    }
    impl PCLK_ACFW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_ACFW::CCLK_DIV_4 => 0,
                PCLK_ACFW::CCLK => 1,
                PCLK_ACFW::CCLK_DIV_2 => 2,
                PCLK_ACFW::CCLK_DIV_6 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_ACFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_ACFW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_ACFW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_ACFW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_ACFW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_ACFW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 6. PCLK_peripheral = CCLK/6" ]
        # [ inline ( always ) ]
        pub fn cclk_div_6(self) -> &'a mut W {
            self.variant(PCLK_ACFW::CCLK_DIV_6)
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
        # [ doc = "Bits 0:1 - Peripheral clock selection for WDT." ]
        # [ inline ( always ) ]
        pub fn pclk_wdt(&self) -> PCLK_WDTR {
            PCLK_WDTR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 0;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bits 2:3 - Peripheral clock selection for TIMER0." ]
        # [ inline ( always ) ]
        pub fn pclk_timer0(&self) -> PCLK_TIMER0R {
            PCLK_TIMER0R::_from({
                                    const MASK: u8 = 3;
                                    const OFFSET: u8 = 2;
                                    ((self.bits >> OFFSET) & MASK as u32) as u8
                                })
        }
        # [ doc = "Bits 4:5 - Peripheral clock selection for TIMER1." ]
        # [ inline ( always ) ]
        pub fn pclk_timer1(&self) -> PCLK_TIMER1R {
            PCLK_TIMER1R::_from({
                                    const MASK: u8 = 3;
                                    const OFFSET: u8 = 4;
                                    ((self.bits >> OFFSET) & MASK as u32) as u8
                                })
        }
        # [ doc = "Bits 6:7 - Peripheral clock selection for UART0." ]
        # [ inline ( always ) ]
        pub fn pclk_uart0(&self) -> PCLK_UART0R {
            PCLK_UART0R::_from({
                                   const MASK: u8 = 3;
                                   const OFFSET: u8 = 6;
                                   ((self.bits >> OFFSET) & MASK as u32) as u8
                               })
        }
        # [ doc = "Bits 8:9 - Peripheral clock selection for UART1." ]
        # [ inline ( always ) ]
        pub fn pclk_uart1(&self) -> PCLK_UART1R {
            PCLK_UART1R::_from({
                                   const MASK: u8 = 3;
                                   const OFFSET: u8 = 8;
                                   ((self.bits >> OFFSET) & MASK as u32) as u8
                               })
        }
        # [ doc = "Bits 12:13 - Peripheral clock selection for PWM1." ]
        # [ inline ( always ) ]
        pub fn pclk_pwm1(&self) -> PCLK_PWM1R {
            PCLK_PWM1R::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 12;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 14:15 - Peripheral clock selection for I2C0." ]
        # [ inline ( always ) ]
        pub fn pclk_i2c0(&self) -> PCLK_I2C0R {
            PCLK_I2C0R::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 14;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 16:17 - Peripheral clock selection for SPI." ]
        # [ inline ( always ) ]
        pub fn pclk_spi(&self) -> PCLK_SPIR {
            PCLK_SPIR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 16;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bits 20:21 - Peripheral clock selection for SSP1." ]
        # [ inline ( always ) ]
        pub fn pclk_ssp1(&self) -> PCLK_SSP1R {
            PCLK_SSP1R::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 20;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 22:23 - Peripheral clock selection for DAC." ]
        # [ inline ( always ) ]
        pub fn pclk_dac(&self) -> PCLK_DACR {
            PCLK_DACR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 22;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bits 24:25 - Peripheral clock selection for ADC." ]
        # [ inline ( always ) ]
        pub fn pclk_adc(&self) -> PCLK_ADCR {
            PCLK_ADCR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 24;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used." ]
        # [ inline ( always ) ]
        pub fn pclk_can1(&self) -> PCLK_CAN1R {
            PCLK_CAN1R::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 26;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used." ]
        # [ inline ( always ) ]
        pub fn pclk_can2(&self) -> PCLK_CAN2R {
            PCLK_CAN2R::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 28;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used." ]
        # [ inline ( always ) ]
        pub fn pclk_acf(&self) -> PCLK_ACFR {
            PCLK_ACFR::_from({
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
        # [ doc = "Bits 0:1 - Peripheral clock selection for WDT." ]
        # [ inline ( always ) ]
        pub fn pclk_wdt(&mut self) -> _PCLK_WDTW {
            _PCLK_WDTW { w: self }
        }
        # [ doc = "Bits 2:3 - Peripheral clock selection for TIMER0." ]
        # [ inline ( always ) ]
        pub fn pclk_timer0(&mut self) -> _PCLK_TIMER0W {
            _PCLK_TIMER0W { w: self }
        }
        # [ doc = "Bits 4:5 - Peripheral clock selection for TIMER1." ]
        # [ inline ( always ) ]
        pub fn pclk_timer1(&mut self) -> _PCLK_TIMER1W {
            _PCLK_TIMER1W { w: self }
        }
        # [ doc = "Bits 6:7 - Peripheral clock selection for UART0." ]
        # [ inline ( always ) ]
        pub fn pclk_uart0(&mut self) -> _PCLK_UART0W {
            _PCLK_UART0W { w: self }
        }
        # [ doc = "Bits 8:9 - Peripheral clock selection for UART1." ]
        # [ inline ( always ) ]
        pub fn pclk_uart1(&mut self) -> _PCLK_UART1W {
            _PCLK_UART1W { w: self }
        }
        # [ doc = "Bits 12:13 - Peripheral clock selection for PWM1." ]
        # [ inline ( always ) ]
        pub fn pclk_pwm1(&mut self) -> _PCLK_PWM1W {
            _PCLK_PWM1W { w: self }
        }
        # [ doc = "Bits 14:15 - Peripheral clock selection for I2C0." ]
        # [ inline ( always ) ]
        pub fn pclk_i2c0(&mut self) -> _PCLK_I2C0W {
            _PCLK_I2C0W { w: self }
        }
        # [ doc = "Bits 16:17 - Peripheral clock selection for SPI." ]
        # [ inline ( always ) ]
        pub fn pclk_spi(&mut self) -> _PCLK_SPIW {
            _PCLK_SPIW { w: self }
        }
        # [ doc = "Bits 20:21 - Peripheral clock selection for SSP1." ]
        # [ inline ( always ) ]
        pub fn pclk_ssp1(&mut self) -> _PCLK_SSP1W {
            _PCLK_SSP1W { w: self }
        }
        # [ doc = "Bits 22:23 - Peripheral clock selection for DAC." ]
        # [ inline ( always ) ]
        pub fn pclk_dac(&mut self) -> _PCLK_DACW {
            _PCLK_DACW { w: self }
        }
        # [ doc = "Bits 24:25 - Peripheral clock selection for ADC." ]
        # [ inline ( always ) ]
        pub fn pclk_adc(&mut self) -> _PCLK_ADCW {
            _PCLK_ADCW { w: self }
        }
        # [ doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used." ]
        # [ inline ( always ) ]
        pub fn pclk_can1(&mut self) -> _PCLK_CAN1W {
            _PCLK_CAN1W { w: self }
        }
        # [ doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used." ]
        # [ inline ( always ) ]
        pub fn pclk_can2(&mut self) -> _PCLK_CAN2W {
            _PCLK_CAN2W { w: self }
        }
        # [ doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used." ]
        # [ inline ( always ) ]
        pub fn pclk_acf(&mut self) -> _PCLK_ACFW {
            _PCLK_ACFW { w: self }
        }
    }
}
# [ doc = "Peripheral Clock Selection register 1." ]
pub struct PCLKSEL1 {
    register: VolatileCell<u32>,
}
# [ doc = "Peripheral Clock Selection register 1." ]
pub mod pclksel1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PCLKSEL1 {
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
    # [ doc = "Possible values of the field `PCLK_QEI`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_QEIR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_QEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_QEIR::CCLK_DIV_4 => 0,
                PCLK_QEIR::CCLK => 1,
                PCLK_QEIR::CCLK_DIV_2 => 2,
                PCLK_QEIR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_QEIR {
            match value {
                0 => PCLK_QEIR::CCLK_DIV_4,
                1 => PCLK_QEIR::CCLK,
                2 => PCLK_QEIR::CCLK_DIV_2,
                3 => PCLK_QEIR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_QEIR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_QEIR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_QEIR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_QEIR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_GPIOINT`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_GPIOINTR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_GPIOINTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_GPIOINTR::CCLK_DIV_4 => 0,
                PCLK_GPIOINTR::CCLK => 1,
                PCLK_GPIOINTR::CCLK_DIV_2 => 2,
                PCLK_GPIOINTR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_GPIOINTR {
            match value {
                0 => PCLK_GPIOINTR::CCLK_DIV_4,
                1 => PCLK_GPIOINTR::CCLK,
                2 => PCLK_GPIOINTR::CCLK_DIV_2,
                3 => PCLK_GPIOINTR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_GPIOINTR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_GPIOINTR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_GPIOINTR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_GPIOINTR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_PCB`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_PCBR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_PCBR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_PCBR::CCLK_DIV_4 => 0,
                PCLK_PCBR::CCLK => 1,
                PCLK_PCBR::CCLK_DIV_2 => 2,
                PCLK_PCBR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_PCBR {
            match value {
                0 => PCLK_PCBR::CCLK_DIV_4,
                1 => PCLK_PCBR::CCLK,
                2 => PCLK_PCBR::CCLK_DIV_2,
                3 => PCLK_PCBR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_PCBR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_PCBR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_PCBR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_PCBR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_I2C1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_I2C1R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_I2C1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_I2C1R::CCLK_DIV_4 => 0,
                PCLK_I2C1R::CCLK => 1,
                PCLK_I2C1R::CCLK_DIV_2 => 2,
                PCLK_I2C1R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_I2C1R {
            match value {
                0 => PCLK_I2C1R::CCLK_DIV_4,
                1 => PCLK_I2C1R::CCLK,
                2 => PCLK_I2C1R::CCLK_DIV_2,
                3 => PCLK_I2C1R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_I2C1R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_I2C1R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_I2C1R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_I2C1R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_SSP0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_SSP0R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_SSP0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_SSP0R::CCLK_DIV_4 => 0,
                PCLK_SSP0R::CCLK => 1,
                PCLK_SSP0R::CCLK_DIV_2 => 2,
                PCLK_SSP0R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_SSP0R {
            match value {
                0 => PCLK_SSP0R::CCLK_DIV_4,
                1 => PCLK_SSP0R::CCLK,
                2 => PCLK_SSP0R::CCLK_DIV_2,
                3 => PCLK_SSP0R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_SSP0R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_SSP0R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_SSP0R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_SSP0R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_TIMER2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_TIMER2R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_TIMER2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_TIMER2R::CCLK_DIV_4 => 0,
                PCLK_TIMER2R::CCLK => 1,
                PCLK_TIMER2R::CCLK_DIV_2 => 2,
                PCLK_TIMER2R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_TIMER2R {
            match value {
                0 => PCLK_TIMER2R::CCLK_DIV_4,
                1 => PCLK_TIMER2R::CCLK,
                2 => PCLK_TIMER2R::CCLK_DIV_2,
                3 => PCLK_TIMER2R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_TIMER2R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_TIMER2R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_TIMER2R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_TIMER2R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_TIMER3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_TIMER3R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_TIMER3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_TIMER3R::CCLK_DIV_4 => 0,
                PCLK_TIMER3R::CCLK => 1,
                PCLK_TIMER3R::CCLK_DIV_2 => 2,
                PCLK_TIMER3R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_TIMER3R {
            match value {
                0 => PCLK_TIMER3R::CCLK_DIV_4,
                1 => PCLK_TIMER3R::CCLK,
                2 => PCLK_TIMER3R::CCLK_DIV_2,
                3 => PCLK_TIMER3R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_TIMER3R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_TIMER3R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_TIMER3R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_TIMER3R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_UART2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_UART2R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_UART2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_UART2R::CCLK_DIV_4 => 0,
                PCLK_UART2R::CCLK => 1,
                PCLK_UART2R::CCLK_DIV_2 => 2,
                PCLK_UART2R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_UART2R {
            match value {
                0 => PCLK_UART2R::CCLK_DIV_4,
                1 => PCLK_UART2R::CCLK,
                2 => PCLK_UART2R::CCLK_DIV_2,
                3 => PCLK_UART2R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_UART2R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_UART2R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_UART2R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_UART2R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_UART3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_UART3R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_UART3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_UART3R::CCLK_DIV_4 => 0,
                PCLK_UART3R::CCLK => 1,
                PCLK_UART3R::CCLK_DIV_2 => 2,
                PCLK_UART3R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_UART3R {
            match value {
                0 => PCLK_UART3R::CCLK_DIV_4,
                1 => PCLK_UART3R::CCLK,
                2 => PCLK_UART3R::CCLK_DIV_2,
                3 => PCLK_UART3R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_UART3R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_UART3R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_UART3R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_UART3R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_I2C2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_I2C2R {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_I2C2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_I2C2R::CCLK_DIV_4 => 0,
                PCLK_I2C2R::CCLK => 1,
                PCLK_I2C2R::CCLK_DIV_2 => 2,
                PCLK_I2C2R::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_I2C2R {
            match value {
                0 => PCLK_I2C2R::CCLK_DIV_4,
                1 => PCLK_I2C2R::CCLK,
                2 => PCLK_I2C2R::CCLK_DIV_2,
                3 => PCLK_I2C2R::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_I2C2R::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_I2C2R::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_I2C2R::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_I2C2R::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_I2S`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_I2SR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_I2SR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_I2SR::CCLK_DIV_4 => 0,
                PCLK_I2SR::CCLK => 1,
                PCLK_I2SR::CCLK_DIV_2 => 2,
                PCLK_I2SR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_I2SR {
            match value {
                0 => PCLK_I2SR::CCLK_DIV_4,
                1 => PCLK_I2SR::CCLK,
                2 => PCLK_I2SR::CCLK_DIV_2,
                3 => PCLK_I2SR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_I2SR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_I2SR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_I2SR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_I2SR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_RIT`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_RITR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_RITR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_RITR::CCLK_DIV_4 => 0,
                PCLK_RITR::CCLK => 1,
                PCLK_RITR::CCLK_DIV_2 => 2,
                PCLK_RITR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_RITR {
            match value {
                0 => PCLK_RITR::CCLK_DIV_4,
                1 => PCLK_RITR::CCLK,
                2 => PCLK_RITR::CCLK_DIV_2,
                3 => PCLK_RITR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_RITR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_RITR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_RITR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_RITR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_SYSCON`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_SYSCONR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_SYSCONR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_SYSCONR::CCLK_DIV_4 => 0,
                PCLK_SYSCONR::CCLK => 1,
                PCLK_SYSCONR::CCLK_DIV_2 => 2,
                PCLK_SYSCONR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_SYSCONR {
            match value {
                0 => PCLK_SYSCONR::CCLK_DIV_4,
                1 => PCLK_SYSCONR::CCLK,
                2 => PCLK_SYSCONR::CCLK_DIV_2,
                3 => PCLK_SYSCONR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_SYSCONR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_SYSCONR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_SYSCONR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_SYSCONR::CCLK_DIV_8
        }
    }
    # [ doc = "Possible values of the field `PCLK_MC`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PCLK_MCR {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_MCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PCLK_MCR::CCLK_DIV_4 => 0,
                PCLK_MCR::CCLK => 1,
                PCLK_MCR::CCLK_DIV_2 => 2,
                PCLK_MCR::CCLK_DIV_8 => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PCLK_MCR {
            match value {
                0 => PCLK_MCR::CCLK_DIV_4,
                1 => PCLK_MCR::CCLK,
                2 => PCLK_MCR::CCLK_DIV_2,
                3 => PCLK_MCR::CCLK_DIV_8,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_4`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_4(&self) -> bool {
            *self == PCLK_MCR::CCLK_DIV_4
        }
        # [ doc = "Checks if the value of the field is `CCLK`" ]
        # [ inline ( always ) ]
        pub fn is_cclk(&self) -> bool {
            *self == PCLK_MCR::CCLK
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_2`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_2(&self) -> bool {
            *self == PCLK_MCR::CCLK_DIV_2
        }
        # [ doc = "Checks if the value of the field is `CCLK_DIV_8`" ]
        # [ inline ( always ) ]
        pub fn is_cclk_div_8(&self) -> bool {
            *self == PCLK_MCR::CCLK_DIV_8
        }
    }
    # [ doc = "Values that can be written to the field `PCLK_QEI`" ]
    pub enum PCLK_QEIW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_QEIW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_QEIW::CCLK_DIV_4 => 0,
                PCLK_QEIW::CCLK => 1,
                PCLK_QEIW::CCLK_DIV_2 => 2,
                PCLK_QEIW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_QEIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_QEIW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_QEIW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_QEIW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_QEIW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_QEIW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_QEIW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_GPIOINT`" ]
    pub enum PCLK_GPIOINTW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_GPIOINTW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_GPIOINTW::CCLK_DIV_4 => 0,
                PCLK_GPIOINTW::CCLK => 1,
                PCLK_GPIOINTW::CCLK_DIV_2 => 2,
                PCLK_GPIOINTW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_GPIOINTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_GPIOINTW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_GPIOINTW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_GPIOINTW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_GPIOINTW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_GPIOINTW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_GPIOINTW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_PCB`" ]
    pub enum PCLK_PCBW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_PCBW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_PCBW::CCLK_DIV_4 => 0,
                PCLK_PCBW::CCLK => 1,
                PCLK_PCBW::CCLK_DIV_2 => 2,
                PCLK_PCBW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_PCBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_PCBW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_PCBW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_PCBW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_PCBW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_PCBW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_PCBW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_I2C1`" ]
    pub enum PCLK_I2C1W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_I2C1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_I2C1W::CCLK_DIV_4 => 0,
                PCLK_I2C1W::CCLK => 1,
                PCLK_I2C1W::CCLK_DIV_2 => 2,
                PCLK_I2C1W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_I2C1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_I2C1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_I2C1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_I2C1W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_I2C1W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_I2C1W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_I2C1W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_SSP0`" ]
    pub enum PCLK_SSP0W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_SSP0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_SSP0W::CCLK_DIV_4 => 0,
                PCLK_SSP0W::CCLK => 1,
                PCLK_SSP0W::CCLK_DIV_2 => 2,
                PCLK_SSP0W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_SSP0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_SSP0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_SSP0W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_SSP0W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_SSP0W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_SSP0W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_SSP0W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_TIMER2`" ]
    pub enum PCLK_TIMER2W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_TIMER2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_TIMER2W::CCLK_DIV_4 => 0,
                PCLK_TIMER2W::CCLK => 1,
                PCLK_TIMER2W::CCLK_DIV_2 => 2,
                PCLK_TIMER2W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_TIMER2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_TIMER2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_TIMER2W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_TIMER2W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_TIMER2W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_TIMER2W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_TIMER2W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_TIMER3`" ]
    pub enum PCLK_TIMER3W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_TIMER3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_TIMER3W::CCLK_DIV_4 => 0,
                PCLK_TIMER3W::CCLK => 1,
                PCLK_TIMER3W::CCLK_DIV_2 => 2,
                PCLK_TIMER3W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_TIMER3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_TIMER3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_TIMER3W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_TIMER3W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_TIMER3W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_TIMER3W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_TIMER3W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_UART2`" ]
    pub enum PCLK_UART2W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_UART2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_UART2W::CCLK_DIV_4 => 0,
                PCLK_UART2W::CCLK => 1,
                PCLK_UART2W::CCLK_DIV_2 => 2,
                PCLK_UART2W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_UART2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_UART2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_UART2W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_UART2W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_UART2W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_UART2W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_UART2W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_UART3`" ]
    pub enum PCLK_UART3W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_UART3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_UART3W::CCLK_DIV_4 => 0,
                PCLK_UART3W::CCLK => 1,
                PCLK_UART3W::CCLK_DIV_2 => 2,
                PCLK_UART3W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_UART3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_UART3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_UART3W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_UART3W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_UART3W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_UART3W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_UART3W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_I2C2`" ]
    pub enum PCLK_I2C2W {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_I2C2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_I2C2W::CCLK_DIV_4 => 0,
                PCLK_I2C2W::CCLK => 1,
                PCLK_I2C2W::CCLK_DIV_2 => 2,
                PCLK_I2C2W::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_I2C2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_I2C2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_I2C2W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_I2C2W::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_I2C2W::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_I2C2W::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_I2C2W::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_I2S`" ]
    pub enum PCLK_I2SW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_I2SW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_I2SW::CCLK_DIV_4 => 0,
                PCLK_I2SW::CCLK => 1,
                PCLK_I2SW::CCLK_DIV_2 => 2,
                PCLK_I2SW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_I2SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_I2SW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_I2SW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_I2SW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_I2SW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_I2SW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_I2SW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_RIT`" ]
    pub enum PCLK_RITW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_RITW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_RITW::CCLK_DIV_4 => 0,
                PCLK_RITW::CCLK => 1,
                PCLK_RITW::CCLK_DIV_2 => 2,
                PCLK_RITW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_RITW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_RITW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_RITW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_RITW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_RITW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_RITW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_RITW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_SYSCON`" ]
    pub enum PCLK_SYSCONW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_SYSCONW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_SYSCONW::CCLK_DIV_4 => 0,
                PCLK_SYSCONW::CCLK => 1,
                PCLK_SYSCONW::CCLK_DIV_2 => 2,
                PCLK_SYSCONW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_SYSCONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_SYSCONW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_SYSCONW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_SYSCONW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_SYSCONW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_SYSCONW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_SYSCONW::CCLK_DIV_8)
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
    # [ doc = "Values that can be written to the field `PCLK_MC`" ]
    pub enum PCLK_MCW {
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        CCLK_DIV_4,
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        CCLK,
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        CCLK_DIV_2,
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        CCLK_DIV_8,
    }
    impl PCLK_MCW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PCLK_MCW::CCLK_DIV_4 => 0,
                PCLK_MCW::CCLK => 1,
                PCLK_MCW::CCLK_DIV_2 => 2,
                PCLK_MCW::CCLK_DIV_8 => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PCLK_MCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PCLK_MCW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PCLK_MCW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "CCLK div 4. PCLK_peripheral = CCLK/4" ]
        # [ inline ( always ) ]
        pub fn cclk_div_4(self) -> &'a mut W {
            self.variant(PCLK_MCW::CCLK_DIV_4)
        }
        # [ doc = "CCLK. PCLK_peripheral = CCLK" ]
        # [ inline ( always ) ]
        pub fn cclk(self) -> &'a mut W {
            self.variant(PCLK_MCW::CCLK)
        }
        # [ doc = "CCLK div 2. PCLK_peripheral = CCLK/2" ]
        # [ inline ( always ) ]
        pub fn cclk_div_2(self) -> &'a mut W {
            self.variant(PCLK_MCW::CCLK_DIV_2)
        }
        # [ doc = "CCLK div 8. PCLK_peripheral = CCLK/8" ]
        # [ inline ( always ) ]
        pub fn cclk_div_8(self) -> &'a mut W {
            self.variant(PCLK_MCW::CCLK_DIV_8)
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
        # [ doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface." ]
        # [ inline ( always ) ]
        pub fn pclk_qei(&self) -> PCLK_QEIR {
            PCLK_QEIR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 0;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts." ]
        # [ inline ( always ) ]
        pub fn pclk_gpioint(&self) -> PCLK_GPIOINTR {
            PCLK_GPIOINTR::_from({
                                     const MASK: u8 = 3;
                                     const OFFSET: u8 = 2;
                                     ((self.bits >> OFFSET) & MASK as u32) as u8
                                 })
        }
        # [ doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block." ]
        # [ inline ( always ) ]
        pub fn pclk_pcb(&self) -> PCLK_PCBR {
            PCLK_PCBR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 4;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bits 6:7 - Peripheral clock selection for I2C1." ]
        # [ inline ( always ) ]
        pub fn pclk_i2c1(&self) -> PCLK_I2C1R {
            PCLK_I2C1R::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 6;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 10:11 - Peripheral clock selection for SSP0." ]
        # [ inline ( always ) ]
        pub fn pclk_ssp0(&self) -> PCLK_SSP0R {
            PCLK_SSP0R::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 10;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 12:13 - Peripheral clock selection for TIMER2." ]
        # [ inline ( always ) ]
        pub fn pclk_timer2(&self) -> PCLK_TIMER2R {
            PCLK_TIMER2R::_from({
                                    const MASK: u8 = 3;
                                    const OFFSET: u8 = 12;
                                    ((self.bits >> OFFSET) & MASK as u32) as u8
                                })
        }
        # [ doc = "Bits 14:15 - Peripheral clock selection for TIMER3." ]
        # [ inline ( always ) ]
        pub fn pclk_timer3(&self) -> PCLK_TIMER3R {
            PCLK_TIMER3R::_from({
                                    const MASK: u8 = 3;
                                    const OFFSET: u8 = 14;
                                    ((self.bits >> OFFSET) & MASK as u32) as u8
                                })
        }
        # [ doc = "Bits 16:17 - Peripheral clock selection for UART2." ]
        # [ inline ( always ) ]
        pub fn pclk_uart2(&self) -> PCLK_UART2R {
            PCLK_UART2R::_from({
                                   const MASK: u8 = 3;
                                   const OFFSET: u8 = 16;
                                   ((self.bits >> OFFSET) & MASK as u32) as u8
                               })
        }
        # [ doc = "Bits 18:19 - Peripheral clock selection for UART3." ]
        # [ inline ( always ) ]
        pub fn pclk_uart3(&self) -> PCLK_UART3R {
            PCLK_UART3R::_from({
                                   const MASK: u8 = 3;
                                   const OFFSET: u8 = 18;
                                   ((self.bits >> OFFSET) & MASK as u32) as u8
                               })
        }
        # [ doc = "Bits 20:21 - Peripheral clock selection for I2C2." ]
        # [ inline ( always ) ]
        pub fn pclk_i2c2(&self) -> PCLK_I2C2R {
            PCLK_I2C2R::_from({
                                  const MASK: u8 = 3;
                                  const OFFSET: u8 = 20;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 22:23 - Peripheral clock selection for I2S." ]
        # [ inline ( always ) ]
        pub fn pclk_i2s(&self) -> PCLK_I2SR {
            PCLK_I2SR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 22;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer." ]
        # [ inline ( always ) ]
        pub fn pclk_rit(&self) -> PCLK_RITR {
            PCLK_RITR::_from({
                                 const MASK: u8 = 3;
                                 const OFFSET: u8 = 26;
                                 ((self.bits >> OFFSET) & MASK as u32) as u8
                             })
        }
        # [ doc = "Bits 28:29 - Peripheral clock selection for the System Control block." ]
        # [ inline ( always ) ]
        pub fn pclk_syscon(&self) -> PCLK_SYSCONR {
            PCLK_SYSCONR::_from({
                                    const MASK: u8 = 3;
                                    const OFFSET: u8 = 28;
                                    ((self.bits >> OFFSET) & MASK as u32) as u8
                                })
        }
        # [ doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM." ]
        # [ inline ( always ) ]
        pub fn pclk_mc(&self) -> PCLK_MCR {
            PCLK_MCR::_from({
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
        # [ doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface." ]
        # [ inline ( always ) ]
        pub fn pclk_qei(&mut self) -> _PCLK_QEIW {
            _PCLK_QEIW { w: self }
        }
        # [ doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts." ]
        # [ inline ( always ) ]
        pub fn pclk_gpioint(&mut self) -> _PCLK_GPIOINTW {
            _PCLK_GPIOINTW { w: self }
        }
        # [ doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block." ]
        # [ inline ( always ) ]
        pub fn pclk_pcb(&mut self) -> _PCLK_PCBW {
            _PCLK_PCBW { w: self }
        }
        # [ doc = "Bits 6:7 - Peripheral clock selection for I2C1." ]
        # [ inline ( always ) ]
        pub fn pclk_i2c1(&mut self) -> _PCLK_I2C1W {
            _PCLK_I2C1W { w: self }
        }
        # [ doc = "Bits 10:11 - Peripheral clock selection for SSP0." ]
        # [ inline ( always ) ]
        pub fn pclk_ssp0(&mut self) -> _PCLK_SSP0W {
            _PCLK_SSP0W { w: self }
        }
        # [ doc = "Bits 12:13 - Peripheral clock selection for TIMER2." ]
        # [ inline ( always ) ]
        pub fn pclk_timer2(&mut self) -> _PCLK_TIMER2W {
            _PCLK_TIMER2W { w: self }
        }
        # [ doc = "Bits 14:15 - Peripheral clock selection for TIMER3." ]
        # [ inline ( always ) ]
        pub fn pclk_timer3(&mut self) -> _PCLK_TIMER3W {
            _PCLK_TIMER3W { w: self }
        }
        # [ doc = "Bits 16:17 - Peripheral clock selection for UART2." ]
        # [ inline ( always ) ]
        pub fn pclk_uart2(&mut self) -> _PCLK_UART2W {
            _PCLK_UART2W { w: self }
        }
        # [ doc = "Bits 18:19 - Peripheral clock selection for UART3." ]
        # [ inline ( always ) ]
        pub fn pclk_uart3(&mut self) -> _PCLK_UART3W {
            _PCLK_UART3W { w: self }
        }
        # [ doc = "Bits 20:21 - Peripheral clock selection for I2C2." ]
        # [ inline ( always ) ]
        pub fn pclk_i2c2(&mut self) -> _PCLK_I2C2W {
            _PCLK_I2C2W { w: self }
        }
        # [ doc = "Bits 22:23 - Peripheral clock selection for I2S." ]
        # [ inline ( always ) ]
        pub fn pclk_i2s(&mut self) -> _PCLK_I2SW {
            _PCLK_I2SW { w: self }
        }
        # [ doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer." ]
        # [ inline ( always ) ]
        pub fn pclk_rit(&mut self) -> _PCLK_RITW {
            _PCLK_RITW { w: self }
        }
        # [ doc = "Bits 28:29 - Peripheral clock selection for the System Control block." ]
        # [ inline ( always ) ]
        pub fn pclk_syscon(&mut self) -> _PCLK_SYSCONW {
            _PCLK_SYSCONW { w: self }
        }
        # [ doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM." ]
        # [ inline ( always ) ]
        pub fn pclk_mc(&mut self) -> _PCLK_MCW {
            _PCLK_MCW { w: self }
        }
    }
}
# [ doc = "USB Interrupt Status" ]
pub struct USBINTST {
    register: VolatileCell<u32>,
}
# [ doc = "USB Interrupt Status" ]
pub mod usbintst {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::USBINTST {
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
    pub struct USB_INT_REQ_LPR {
        bits: bool,
    }
    impl USB_INT_REQ_LPR {
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
    pub struct USB_INT_REQ_HPR {
        bits: bool,
    }
    impl USB_INT_REQ_HPR {
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
    pub struct USB_INT_REQ_DMAR {
        bits: bool,
    }
    impl USB_INT_REQ_DMAR {
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
    pub struct USB_HOST_INTR {
        bits: bool,
    }
    impl USB_HOST_INTR {
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
    pub struct USB_ATX_INTR {
        bits: bool,
    }
    impl USB_ATX_INTR {
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
    pub struct USB_OTG_INTR {
        bits: bool,
    }
    impl USB_OTG_INTR {
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
    pub struct USB_I2C_INTR {
        bits: bool,
    }
    impl USB_I2C_INTR {
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
    pub struct USB_NEED_CLKR {
        bits: bool,
    }
    impl USB_NEED_CLKR {
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
    pub struct EN_USB_INTSR {
        bits: bool,
    }
    impl EN_USB_INTSR {
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
    pub struct _USB_INT_REQ_LPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USB_INT_REQ_LPW<'a> {
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
    pub struct _USB_INT_REQ_HPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USB_INT_REQ_HPW<'a> {
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
    pub struct _USB_INT_REQ_DMAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USB_INT_REQ_DMAW<'a> {
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
    pub struct _USB_HOST_INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USB_HOST_INTW<'a> {
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
    pub struct _USB_ATX_INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USB_ATX_INTW<'a> {
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
    pub struct _USB_OTG_INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USB_OTG_INTW<'a> {
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
    pub struct _USB_I2C_INTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USB_I2C_INTW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _USB_NEED_CLKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USB_NEED_CLKW<'a> {
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
    pub struct _EN_USB_INTSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EN_USB_INTSW<'a> {
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
        # [ doc = "Bit 0 - Low priority interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_int_req_lp(&self) -> USB_INT_REQ_LPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USB_INT_REQ_LPR { bits }
        }
        # [ doc = "Bit 1 - High priority interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_int_req_hp(&self) -> USB_INT_REQ_HPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USB_INT_REQ_HPR { bits }
        }
        # [ doc = "Bit 2 - DMA interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_int_req_dma(&self) -> USB_INT_REQ_DMAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USB_INT_REQ_DMAR { bits }
        }
        # [ doc = "Bit 3 - USB host interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_host_int(&self) -> USB_HOST_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USB_HOST_INTR { bits }
        }
        # [ doc = "Bit 4 - External ATX interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_atx_int(&self) -> USB_ATX_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USB_ATX_INTR { bits }
        }
        # [ doc = "Bit 5 - OTG interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_otg_int(&self) -> USB_OTG_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USB_OTG_INTR { bits }
        }
        # [ doc = "Bit 6 - I2C module interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_i2c_int(&self) -> USB_I2C_INTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USB_I2C_INTR { bits }
        }
        # [ doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_need_clk(&self) -> USB_NEED_CLKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USB_NEED_CLKR { bits }
        }
        # [ doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines." ]
        # [ inline ( always ) ]
        pub fn en_usb_ints(&self) -> EN_USB_INTSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EN_USB_INTSR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 2147483648 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Low priority interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_int_req_lp(&mut self) -> _USB_INT_REQ_LPW {
            _USB_INT_REQ_LPW { w: self }
        }
        # [ doc = "Bit 1 - High priority interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_int_req_hp(&mut self) -> _USB_INT_REQ_HPW {
            _USB_INT_REQ_HPW { w: self }
        }
        # [ doc = "Bit 2 - DMA interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_int_req_dma(&mut self) -> _USB_INT_REQ_DMAW {
            _USB_INT_REQ_DMAW { w: self }
        }
        # [ doc = "Bit 3 - USB host interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_host_int(&mut self) -> _USB_HOST_INTW {
            _USB_HOST_INTW { w: self }
        }
        # [ doc = "Bit 4 - External ATX interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_atx_int(&mut self) -> _USB_ATX_INTW {
            _USB_ATX_INTW { w: self }
        }
        # [ doc = "Bit 5 - OTG interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_otg_int(&mut self) -> _USB_OTG_INTW {
            _USB_OTG_INTW { w: self }
        }
        # [ doc = "Bit 6 - I2C module interrupt line status. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_i2c_int(&mut self) -> _USB_I2C_INTW {
            _USB_I2C_INTW { w: self }
        }
        # [ doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only." ]
        # [ inline ( always ) ]
        pub fn usb_need_clk(&mut self) -> _USB_NEED_CLKW {
            _USB_NEED_CLKW { w: self }
        }
        # [ doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines." ]
        # [ inline ( always ) ]
        pub fn en_usb_ints(&mut self) -> _EN_USB_INTSW {
            _EN_USB_INTSW { w: self }
        }
    }
}
# [ doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15" ]
pub struct DMACREQSEL {
    register: VolatileCell<u32>,
}
# [ doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15" ]
pub mod dmacreqsel {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DMACREQSEL {
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
    pub struct DMASEL08R {
        bits: bool,
    }
    impl DMASEL08R {
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
    pub struct DMASEL09R {
        bits: bool,
    }
    impl DMASEL09R {
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
    pub struct DMASEL10R {
        bits: bool,
    }
    impl DMASEL10R {
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
    pub struct DMASEL11R {
        bits: bool,
    }
    impl DMASEL11R {
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
    pub struct DMASEL12R {
        bits: bool,
    }
    impl DMASEL12R {
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
    pub struct DMASEL13R {
        bits: bool,
    }
    impl DMASEL13R {
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
    pub struct DMASEL14R {
        bits: bool,
    }
    impl DMASEL14R {
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
    pub struct DMASEL15R {
        bits: bool,
    }
    impl DMASEL15R {
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
    pub struct _DMASEL08W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMASEL08W<'a> {
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
    pub struct _DMASEL09W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMASEL09W<'a> {
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
    pub struct _DMASEL10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMASEL10W<'a> {
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
    pub struct _DMASEL11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMASEL11W<'a> {
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
    pub struct _DMASEL12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMASEL12W<'a> {
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
    pub struct _DMASEL13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMASEL13W<'a> {
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
    pub struct _DMASEL14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMASEL14W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DMASEL15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMASEL15W<'a> {
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
            const OFFSET: u8 = 7;
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
        # [ doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel08(&self) -> DMASEL08R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMASEL08R { bits }
        }
        # [ doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel09(&self) -> DMASEL09R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMASEL09R { bits }
        }
        # [ doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel10(&self) -> DMASEL10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMASEL10R { bits }
        }
        # [ doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel11(&self) -> DMASEL11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMASEL11R { bits }
        }
        # [ doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel12(&self) -> DMASEL12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMASEL12R { bits }
        }
        # [ doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel13(&self) -> DMASEL13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMASEL13R { bits }
        }
        # [ doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel14(&self) -> DMASEL14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMASEL14R { bits }
        }
        # [ doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel15(&self) -> DMASEL15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMASEL15R { bits }
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
        # [ doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel08(&mut self) -> _DMASEL08W {
            _DMASEL08W { w: self }
        }
        # [ doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel09(&mut self) -> _DMASEL09W {
            _DMASEL09W { w: self }
        }
        # [ doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel10(&mut self) -> _DMASEL10W {
            _DMASEL10W { w: self }
        }
        # [ doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel11(&mut self) -> _DMASEL11W {
            _DMASEL11W { w: self }
        }
        # [ doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel12(&mut self) -> _DMASEL12W {
            _DMASEL12W { w: self }
        }
        # [ doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel13(&mut self) -> _DMASEL13W {
            _DMASEL13W { w: self }
        }
        # [ doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel14(&mut self) -> _DMASEL14W {
            _DMASEL14W { w: self }
        }
        # [ doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected." ]
        # [ inline ( always ) ]
        pub fn dmasel15(&mut self) -> _DMASEL15W {
            _DMASEL15W { w: self }
        }
    }
}
# [ doc = "Clock Output Configuration Register" ]
pub struct CLKOUTCFG {
    register: VolatileCell<u32>,
}
# [ doc = "Clock Output Configuration Register" ]
pub mod clkoutcfg {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CLKOUTCFG {
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
    # [ doc = "Possible values of the field `CLKOUTSEL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CLKOUTSELR {
        # [ doc = "Selects the CPU clock as the CLKOUT source." ]
        SELECTS_THE_CPU_CLOC,
        # [ doc = "Selects the main oscillator as the CLKOUT source." ]
        SELECTS_THE_MAIN_OSC,
        # [ doc = "Selects the Internal RC oscillator as the CLKOUT source." ]
        SELECTS_THE_INTERNAL,
        # [ doc = "Selects the USB clock as the CLKOUT source." ]
        SELECTS_THE_USB_CLOC,
        # [ doc = "Selects the RTC oscillator as the CLKOUT source." ]
        SELECTS_THE_RTC_OSCI,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl CLKOUTSELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                CLKOUTSELR::SELECTS_THE_CPU_CLOC => 0,
                CLKOUTSELR::SELECTS_THE_MAIN_OSC => 1,
                CLKOUTSELR::SELECTS_THE_INTERNAL => 2,
                CLKOUTSELR::SELECTS_THE_USB_CLOC => 3,
                CLKOUTSELR::SELECTS_THE_RTC_OSCI => 4,
                CLKOUTSELR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> CLKOUTSELR {
            match value {
                0 => CLKOUTSELR::SELECTS_THE_CPU_CLOC,
                1 => CLKOUTSELR::SELECTS_THE_MAIN_OSC,
                2 => CLKOUTSELR::SELECTS_THE_INTERNAL,
                3 => CLKOUTSELR::SELECTS_THE_USB_CLOC,
                4 => CLKOUTSELR::SELECTS_THE_RTC_OSCI,
                i => CLKOUTSELR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `SELECTS_THE_CPU_CLOC`" ]
        # [ inline ( always ) ]
        pub fn is_selects_the_cpu_cloc(&self) -> bool {
            *self == CLKOUTSELR::SELECTS_THE_CPU_CLOC
        }
        # [ doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`" ]
        # [ inline ( always ) ]
        pub fn is_selects_the_main_osc(&self) -> bool {
            *self == CLKOUTSELR::SELECTS_THE_MAIN_OSC
        }
        # [ doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`" ]
        # [ inline ( always ) ]
        pub fn is_selects_the_internal(&self) -> bool {
            *self == CLKOUTSELR::SELECTS_THE_INTERNAL
        }
        # [ doc = "Checks if the value of the field is `SELECTS_THE_USB_CLOC`" ]
        # [ inline ( always ) ]
        pub fn is_selects_the_usb_cloc(&self) -> bool {
            *self == CLKOUTSELR::SELECTS_THE_USB_CLOC
        }
        # [ doc = "Checks if the value of the field is `SELECTS_THE_RTC_OSCI`" ]
        # [ inline ( always ) ]
        pub fn is_selects_the_rtc_osci(&self) -> bool {
            *self == CLKOUTSELR::SELECTS_THE_RTC_OSCI
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKOUTDIVR {
        bits: u8,
    }
    impl CLKOUTDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKOUT_ENR {
        bits: bool,
    }
    impl CLKOUT_ENR {
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
    pub struct CLKOUT_ACTR {
        bits: bool,
    }
    impl CLKOUT_ACTR {
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
    # [ doc = "Values that can be written to the field `CLKOUTSEL`" ]
    pub enum CLKOUTSELW {
        # [ doc = "Selects the CPU clock as the CLKOUT source." ]
        SELECTS_THE_CPU_CLOC,
        # [ doc = "Selects the main oscillator as the CLKOUT source." ]
        SELECTS_THE_MAIN_OSC,
        # [ doc = "Selects the Internal RC oscillator as the CLKOUT source." ]
        SELECTS_THE_INTERNAL,
        # [ doc = "Selects the USB clock as the CLKOUT source." ]
        SELECTS_THE_USB_CLOC,
        # [ doc = "Selects the RTC oscillator as the CLKOUT source." ]
        SELECTS_THE_RTC_OSCI,
    }
    impl CLKOUTSELW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                CLKOUTSELW::SELECTS_THE_CPU_CLOC => 0,
                CLKOUTSELW::SELECTS_THE_MAIN_OSC => 1,
                CLKOUTSELW::SELECTS_THE_INTERNAL => 2,
                CLKOUTSELW::SELECTS_THE_USB_CLOC => 3,
                CLKOUTSELW::SELECTS_THE_RTC_OSCI => 4,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKOUTSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKOUTSELW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CLKOUTSELW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        # [ doc = "Selects the CPU clock as the CLKOUT source." ]
        # [ inline ( always ) ]
        pub fn selects_the_cpu_cloc(self) -> &'a mut W {
            self.variant(CLKOUTSELW::SELECTS_THE_CPU_CLOC)
        }
        # [ doc = "Selects the main oscillator as the CLKOUT source." ]
        # [ inline ( always ) ]
        pub fn selects_the_main_osc(self) -> &'a mut W {
            self.variant(CLKOUTSELW::SELECTS_THE_MAIN_OSC)
        }
        # [ doc = "Selects the Internal RC oscillator as the CLKOUT source." ]
        # [ inline ( always ) ]
        pub fn selects_the_internal(self) -> &'a mut W {
            self.variant(CLKOUTSELW::SELECTS_THE_INTERNAL)
        }
        # [ doc = "Selects the USB clock as the CLKOUT source." ]
        # [ inline ( always ) ]
        pub fn selects_the_usb_cloc(self) -> &'a mut W {
            self.variant(CLKOUTSELW::SELECTS_THE_USB_CLOC)
        }
        # [ doc = "Selects the RTC oscillator as the CLKOUT source." ]
        # [ inline ( always ) ]
        pub fn selects_the_rtc_osci(self) -> &'a mut W {
            self.variant(CLKOUTSELW::SELECTS_THE_RTC_OSCI)
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
    # [ doc = r" Proxy" ]
    pub struct _CLKOUTDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKOUTDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CLKOUT_ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKOUT_ENW<'a> {
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
    pub struct _CLKOUT_ACTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKOUT_ACTW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use." ]
        # [ inline ( always ) ]
        pub fn clkoutsel(&self) -> CLKOUTSELR {
            CLKOUTSELR::_from({
                                  const MASK: u8 = 15;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) as u8
                              })
        }
        # [ doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16." ]
        # [ inline ( always ) ]
        pub fn clkoutdiv(&self) -> CLKOUTDIVR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKOUTDIVR { bits }
        }
        # [ doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT." ]
        # [ inline ( always ) ]
        pub fn clkout_en(&self) -> CLKOUT_ENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CLKOUT_ENR { bits }
        }
        # [ doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped." ]
        # [ inline ( always ) ]
        pub fn clkout_act(&self) -> CLKOUT_ACTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CLKOUT_ACTR { bits }
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
        # [ doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use." ]
        # [ inline ( always ) ]
        pub fn clkoutsel(&mut self) -> _CLKOUTSELW {
            _CLKOUTSELW { w: self }
        }
        # [ doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16." ]
        # [ inline ( always ) ]
        pub fn clkoutdiv(&mut self) -> _CLKOUTDIVW {
            _CLKOUTDIVW { w: self }
        }
        # [ doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT." ]
        # [ inline ( always ) ]
        pub fn clkout_en(&mut self) -> _CLKOUT_ENW {
            _CLKOUT_ENW { w: self }
        }
        # [ doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped." ]
        # [ inline ( always ) ]
        pub fn clkout_act(&mut self) -> _CLKOUT_ACTW {
            _CLKOUT_ACTW { w: self }
        }
    }
}
# [ doc = "System and clock control" ]
pub struct SYSCON {
    register_block: RegisterBlock,
}
impl Deref for SYSCON {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
