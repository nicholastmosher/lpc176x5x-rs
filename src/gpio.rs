# ! [ doc = "General Purpose I/O" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "General Purpose I/O" ]
pub const GPIO: Peripheral<GPIO> = unsafe { Peripheral::new(537509888) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - GPIO Port Direction control register." ]
    pub dir0: DIR,
    _reserved0: [u8; 12usize],
    # [ doc = "0x10 - Mask register for Port." ]
    pub mask0: MASK,
    # [ doc = "0x14 - Port Pin value register using FIOMASK." ]
    pub pin0: PIN,
    # [ doc = "0x18 - Port Output Set register using FIOMASK." ]
    pub set0: SET,
    # [ doc = "0x1c - Port Output Clear register using FIOMASK." ]
    pub clr0: CLR,
    # [ doc = "0x20 - GPIO Port Direction control register." ]
    pub dir1: DIR,
    _reserved1: [u8; 12usize],
    # [ doc = "0x30 - Mask register for Port." ]
    pub mask1: MASK,
    # [ doc = "0x34 - Port Pin value register using FIOMASK." ]
    pub pin1: PIN,
    # [ doc = "0x38 - Port Output Set register using FIOMASK." ]
    pub set1: SET,
    # [ doc = "0x3c - Port Output Clear register using FIOMASK." ]
    pub clr1: CLR,
    # [ doc = "0x40 - GPIO Port Direction control register." ]
    pub dir2: DIR,
    _reserved2: [u8; 12usize],
    # [ doc = "0x50 - Mask register for Port." ]
    pub mask2: MASK,
    # [ doc = "0x54 - Port Pin value register using FIOMASK." ]
    pub pin2: PIN,
    # [ doc = "0x58 - Port Output Set register using FIOMASK." ]
    pub set2: SET,
    # [ doc = "0x5c - Port Output Clear register using FIOMASK." ]
    pub clr2: CLR,
    # [ doc = "0x60 - GPIO Port Direction control register." ]
    pub dir3: DIR,
    _reserved3: [u8; 12usize],
    # [ doc = "0x70 - Mask register for Port." ]
    pub mask3: MASK,
    # [ doc = "0x74 - Port Pin value register using FIOMASK." ]
    pub pin3: PIN,
    # [ doc = "0x78 - Port Output Set register using FIOMASK." ]
    pub set3: SET,
    # [ doc = "0x7c - Port Output Clear register using FIOMASK." ]
    pub clr3: CLR,
    # [ doc = "0x80 - GPIO Port Direction control register." ]
    pub dir4: DIR,
    _reserved4: [u8; 12usize],
    # [ doc = "0x90 - Mask register for Port." ]
    pub mask4: MASK,
    # [ doc = "0x94 - Port Pin value register using FIOMASK." ]
    pub pin4: PIN,
    # [ doc = "0x98 - Port Output Set register using FIOMASK." ]
    pub set4: SET,
    # [ doc = "0x9c - Port Output Clear register using FIOMASK." ]
    pub clr4: CLR,
}
# [ doc = "GPIO Port Direction control register." ]
pub struct DIR {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Port Direction control register." ]
pub mod dir {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DIR {
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
    pub struct PINDIR0R {
        bits: bool,
    }
    impl PINDIR0R {
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
    pub struct PINDIR1R {
        bits: bool,
    }
    impl PINDIR1R {
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
    pub struct PINDIR2R {
        bits: bool,
    }
    impl PINDIR2R {
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
    pub struct PINDIR3R {
        bits: bool,
    }
    impl PINDIR3R {
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
    pub struct PINDIR4R {
        bits: bool,
    }
    impl PINDIR4R {
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
    pub struct PINDIR5R {
        bits: bool,
    }
    impl PINDIR5R {
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
    pub struct PINDIR6R {
        bits: bool,
    }
    impl PINDIR6R {
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
    pub struct PINDIR7R {
        bits: bool,
    }
    impl PINDIR7R {
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
    pub struct PINDIR8R {
        bits: bool,
    }
    impl PINDIR8R {
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
    pub struct PINDIR9R {
        bits: bool,
    }
    impl PINDIR9R {
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
    pub struct PINDIR10R {
        bits: bool,
    }
    impl PINDIR10R {
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
    pub struct PINDIR11R {
        bits: bool,
    }
    impl PINDIR11R {
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
    pub struct PINDIR12R {
        bits: bool,
    }
    impl PINDIR12R {
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
    pub struct PINDIR13R {
        bits: bool,
    }
    impl PINDIR13R {
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
    pub struct PINDIR14R {
        bits: bool,
    }
    impl PINDIR14R {
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
    pub struct PINDIR15R {
        bits: bool,
    }
    impl PINDIR15R {
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
    pub struct PINDIR16R {
        bits: bool,
    }
    impl PINDIR16R {
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
    pub struct PINDIR17R {
        bits: bool,
    }
    impl PINDIR17R {
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
    pub struct PINDIR18R {
        bits: bool,
    }
    impl PINDIR18R {
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
    pub struct PINDIR19R {
        bits: bool,
    }
    impl PINDIR19R {
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
    pub struct PINDIR20R {
        bits: bool,
    }
    impl PINDIR20R {
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
    pub struct PINDIR21R {
        bits: bool,
    }
    impl PINDIR21R {
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
    pub struct PINDIR22R {
        bits: bool,
    }
    impl PINDIR22R {
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
    pub struct PINDIR23R {
        bits: bool,
    }
    impl PINDIR23R {
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
    pub struct PINDIR24R {
        bits: bool,
    }
    impl PINDIR24R {
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
    pub struct PINDIR25R {
        bits: bool,
    }
    impl PINDIR25R {
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
    pub struct PINDIR26R {
        bits: bool,
    }
    impl PINDIR26R {
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
    pub struct PINDIR27R {
        bits: bool,
    }
    impl PINDIR27R {
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
    pub struct PINDIR28R {
        bits: bool,
    }
    impl PINDIR28R {
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
    pub struct PINDIR29R {
        bits: bool,
    }
    impl PINDIR29R {
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
    pub struct PINDIR30R {
        bits: bool,
    }
    impl PINDIR30R {
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
    pub struct PINDIR31R {
        bits: bool,
    }
    impl PINDIR31R {
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
    pub struct _PINDIR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR0W<'a> {
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
    pub struct _PINDIR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR1W<'a> {
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
    pub struct _PINDIR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR2W<'a> {
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
    pub struct _PINDIR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR3W<'a> {
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
    pub struct _PINDIR4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR4W<'a> {
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
    pub struct _PINDIR5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR5W<'a> {
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
    pub struct _PINDIR6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR6W<'a> {
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
    pub struct _PINDIR7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR7W<'a> {
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
    pub struct _PINDIR8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR8W<'a> {
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
    pub struct _PINDIR9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR9W<'a> {
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
    pub struct _PINDIR10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR10W<'a> {
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
    pub struct _PINDIR11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR11W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PINDIR12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR12W<'a> {
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
    pub struct _PINDIR13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR13W<'a> {
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
    pub struct _PINDIR14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR14W<'a> {
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
    pub struct _PINDIR15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR15W<'a> {
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
    pub struct _PINDIR16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR16W<'a> {
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
    pub struct _PINDIR17W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR17W<'a> {
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
    pub struct _PINDIR18W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR18W<'a> {
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
    pub struct _PINDIR19W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR19W<'a> {
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
    pub struct _PINDIR20W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR20W<'a> {
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
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINDIR21W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR21W<'a> {
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
    pub struct _PINDIR22W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR22W<'a> {
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
    pub struct _PINDIR23W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR23W<'a> {
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
    pub struct _PINDIR24W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR24W<'a> {
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
    pub struct _PINDIR25W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR25W<'a> {
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
    pub struct _PINDIR26W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR26W<'a> {
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
    pub struct _PINDIR27W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR27W<'a> {
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
    pub struct _PINDIR28W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR28W<'a> {
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
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINDIR29W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR29W<'a> {
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
    pub struct _PINDIR30W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR30W<'a> {
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
    pub struct _PINDIR31W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINDIR31W<'a> {
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
        # [ doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir0(&self) -> PINDIR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR0R { bits }
        }
        # [ doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir1(&self) -> PINDIR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR1R { bits }
        }
        # [ doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir2(&self) -> PINDIR2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR2R { bits }
        }
        # [ doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir3(&self) -> PINDIR3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR3R { bits }
        }
        # [ doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir4(&self) -> PINDIR4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR4R { bits }
        }
        # [ doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir5(&self) -> PINDIR5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR5R { bits }
        }
        # [ doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir6(&self) -> PINDIR6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR6R { bits }
        }
        # [ doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir7(&self) -> PINDIR7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR7R { bits }
        }
        # [ doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir8(&self) -> PINDIR8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR8R { bits }
        }
        # [ doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir9(&self) -> PINDIR9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR9R { bits }
        }
        # [ doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir10(&self) -> PINDIR10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR10R { bits }
        }
        # [ doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir11(&self) -> PINDIR11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR11R { bits }
        }
        # [ doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir12(&self) -> PINDIR12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR12R { bits }
        }
        # [ doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir13(&self) -> PINDIR13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR13R { bits }
        }
        # [ doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir14(&self) -> PINDIR14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR14R { bits }
        }
        # [ doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir15(&self) -> PINDIR15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR15R { bits }
        }
        # [ doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir16(&self) -> PINDIR16R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR16R { bits }
        }
        # [ doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir17(&self) -> PINDIR17R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR17R { bits }
        }
        # [ doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir18(&self) -> PINDIR18R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR18R { bits }
        }
        # [ doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir19(&self) -> PINDIR19R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR19R { bits }
        }
        # [ doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir20(&self) -> PINDIR20R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR20R { bits }
        }
        # [ doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir21(&self) -> PINDIR21R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR21R { bits }
        }
        # [ doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir22(&self) -> PINDIR22R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR22R { bits }
        }
        # [ doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir23(&self) -> PINDIR23R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR23R { bits }
        }
        # [ doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir24(&self) -> PINDIR24R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR24R { bits }
        }
        # [ doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir25(&self) -> PINDIR25R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR25R { bits }
        }
        # [ doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir26(&self) -> PINDIR26R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR26R { bits }
        }
        # [ doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir27(&self) -> PINDIR27R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR27R { bits }
        }
        # [ doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir28(&self) -> PINDIR28R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR28R { bits }
        }
        # [ doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir29(&self) -> PINDIR29R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR29R { bits }
        }
        # [ doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir30(&self) -> PINDIR30R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR30R { bits }
        }
        # [ doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir31(&self) -> PINDIR31R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINDIR31R { bits }
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
        # [ doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir0(&mut self) -> _PINDIR0W {
            _PINDIR0W { w: self }
        }
        # [ doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir1(&mut self) -> _PINDIR1W {
            _PINDIR1W { w: self }
        }
        # [ doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir2(&mut self) -> _PINDIR2W {
            _PINDIR2W { w: self }
        }
        # [ doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir3(&mut self) -> _PINDIR3W {
            _PINDIR3W { w: self }
        }
        # [ doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir4(&mut self) -> _PINDIR4W {
            _PINDIR4W { w: self }
        }
        # [ doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir5(&mut self) -> _PINDIR5W {
            _PINDIR5W { w: self }
        }
        # [ doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir6(&mut self) -> _PINDIR6W {
            _PINDIR6W { w: self }
        }
        # [ doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir7(&mut self) -> _PINDIR7W {
            _PINDIR7W { w: self }
        }
        # [ doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir8(&mut self) -> _PINDIR8W {
            _PINDIR8W { w: self }
        }
        # [ doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir9(&mut self) -> _PINDIR9W {
            _PINDIR9W { w: self }
        }
        # [ doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir10(&mut self) -> _PINDIR10W {
            _PINDIR10W { w: self }
        }
        # [ doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir11(&mut self) -> _PINDIR11W {
            _PINDIR11W { w: self }
        }
        # [ doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir12(&mut self) -> _PINDIR12W {
            _PINDIR12W { w: self }
        }
        # [ doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir13(&mut self) -> _PINDIR13W {
            _PINDIR13W { w: self }
        }
        # [ doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir14(&mut self) -> _PINDIR14W {
            _PINDIR14W { w: self }
        }
        # [ doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir15(&mut self) -> _PINDIR15W {
            _PINDIR15W { w: self }
        }
        # [ doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir16(&mut self) -> _PINDIR16W {
            _PINDIR16W { w: self }
        }
        # [ doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir17(&mut self) -> _PINDIR17W {
            _PINDIR17W { w: self }
        }
        # [ doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir18(&mut self) -> _PINDIR18W {
            _PINDIR18W { w: self }
        }
        # [ doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir19(&mut self) -> _PINDIR19W {
            _PINDIR19W { w: self }
        }
        # [ doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir20(&mut self) -> _PINDIR20W {
            _PINDIR20W { w: self }
        }
        # [ doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir21(&mut self) -> _PINDIR21W {
            _PINDIR21W { w: self }
        }
        # [ doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir22(&mut self) -> _PINDIR22W {
            _PINDIR22W { w: self }
        }
        # [ doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir23(&mut self) -> _PINDIR23W {
            _PINDIR23W { w: self }
        }
        # [ doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir24(&mut self) -> _PINDIR24W {
            _PINDIR24W { w: self }
        }
        # [ doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir25(&mut self) -> _PINDIR25W {
            _PINDIR25W { w: self }
        }
        # [ doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir26(&mut self) -> _PINDIR26W {
            _PINDIR26W { w: self }
        }
        # [ doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir27(&mut self) -> _PINDIR27W {
            _PINDIR27W { w: self }
        }
        # [ doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir28(&mut self) -> _PINDIR28W {
            _PINDIR28W { w: self }
        }
        # [ doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir29(&mut self) -> _PINDIR29W {
            _PINDIR29W { w: self }
        }
        # [ doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir30(&mut self) -> _PINDIR30W {
            _PINDIR30W { w: self }
        }
        # [ doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px[0], bit 31 in DIRx controls pin Px[31]. 0 = Controlled pin is input. 1 = Controlled pin is output." ]
        # [ inline ( always ) ]
        pub fn pindir31(&mut self) -> _PINDIR31W {
            _PINDIR31W { w: self }
        }
    }
}
# [ doc = "Mask register for Port." ]
pub struct MASK {
    register: VolatileCell<u32>,
}
# [ doc = "Mask register for Port." ]
pub mod mask {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MASK {
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
    pub struct PINMASK0R {
        bits: bool,
    }
    impl PINMASK0R {
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
    pub struct PINMASK1R {
        bits: bool,
    }
    impl PINMASK1R {
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
    pub struct PINMASK2R {
        bits: bool,
    }
    impl PINMASK2R {
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
    pub struct PINMASK3R {
        bits: bool,
    }
    impl PINMASK3R {
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
    pub struct PINMASK4R {
        bits: bool,
    }
    impl PINMASK4R {
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
    pub struct PINMASK5R {
        bits: bool,
    }
    impl PINMASK5R {
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
    pub struct PINMASK6R {
        bits: bool,
    }
    impl PINMASK6R {
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
    pub struct PINMASK7R {
        bits: bool,
    }
    impl PINMASK7R {
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
    pub struct PINMASK8R {
        bits: bool,
    }
    impl PINMASK8R {
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
    pub struct PINMASK9R {
        bits: bool,
    }
    impl PINMASK9R {
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
    pub struct PINMASK10R {
        bits: bool,
    }
    impl PINMASK10R {
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
    pub struct PINMASK11R {
        bits: bool,
    }
    impl PINMASK11R {
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
    pub struct PINMASK12R {
        bits: bool,
    }
    impl PINMASK12R {
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
    pub struct PINMASK13R {
        bits: bool,
    }
    impl PINMASK13R {
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
    pub struct PINMASK14R {
        bits: bool,
    }
    impl PINMASK14R {
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
    pub struct PINMASK15R {
        bits: bool,
    }
    impl PINMASK15R {
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
    pub struct PINMASK16R {
        bits: bool,
    }
    impl PINMASK16R {
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
    pub struct PINMASK17R {
        bits: bool,
    }
    impl PINMASK17R {
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
    pub struct PINMASK18R {
        bits: bool,
    }
    impl PINMASK18R {
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
    pub struct PINMASK19R {
        bits: bool,
    }
    impl PINMASK19R {
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
    pub struct PINMASK20R {
        bits: bool,
    }
    impl PINMASK20R {
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
    pub struct PINMASK21R {
        bits: bool,
    }
    impl PINMASK21R {
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
    pub struct PINMASK22R {
        bits: bool,
    }
    impl PINMASK22R {
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
    pub struct PINMASK23R {
        bits: bool,
    }
    impl PINMASK23R {
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
    pub struct PINMASK24R {
        bits: bool,
    }
    impl PINMASK24R {
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
    pub struct PINMASK25R {
        bits: bool,
    }
    impl PINMASK25R {
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
    pub struct PINMASK26R {
        bits: bool,
    }
    impl PINMASK26R {
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
    pub struct PINMASK27R {
        bits: bool,
    }
    impl PINMASK27R {
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
    pub struct PINMASK28R {
        bits: bool,
    }
    impl PINMASK28R {
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
    pub struct PINMASK29R {
        bits: bool,
    }
    impl PINMASK29R {
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
    pub struct PINMASK30R {
        bits: bool,
    }
    impl PINMASK30R {
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
    pub struct PINMASK31R {
        bits: bool,
    }
    impl PINMASK31R {
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
    pub struct _PINMASK0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK0W<'a> {
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
    pub struct _PINMASK1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK1W<'a> {
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
    pub struct _PINMASK2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK2W<'a> {
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
    pub struct _PINMASK3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK3W<'a> {
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
    pub struct _PINMASK4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK4W<'a> {
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
    pub struct _PINMASK5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK5W<'a> {
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
    pub struct _PINMASK6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK6W<'a> {
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
    pub struct _PINMASK7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK7W<'a> {
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
    pub struct _PINMASK8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK8W<'a> {
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
    pub struct _PINMASK9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK9W<'a> {
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
    pub struct _PINMASK10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK10W<'a> {
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
    pub struct _PINMASK11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK11W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PINMASK12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK12W<'a> {
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
    pub struct _PINMASK13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK13W<'a> {
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
    pub struct _PINMASK14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK14W<'a> {
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
    pub struct _PINMASK15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK15W<'a> {
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
    pub struct _PINMASK16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK16W<'a> {
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
    pub struct _PINMASK17W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK17W<'a> {
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
    pub struct _PINMASK18W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK18W<'a> {
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
    pub struct _PINMASK19W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK19W<'a> {
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
    pub struct _PINMASK20W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK20W<'a> {
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
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINMASK21W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK21W<'a> {
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
    pub struct _PINMASK22W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK22W<'a> {
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
    pub struct _PINMASK23W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK23W<'a> {
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
    pub struct _PINMASK24W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK24W<'a> {
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
    pub struct _PINMASK25W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK25W<'a> {
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
    pub struct _PINMASK26W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK26W<'a> {
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
    pub struct _PINMASK27W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK27W<'a> {
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
    pub struct _PINMASK28W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK28W<'a> {
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
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINMASK29W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK29W<'a> {
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
    pub struct _PINMASK30W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK30W<'a> {
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
    pub struct _PINMASK31W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASK31W<'a> {
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
        # [ doc = "Bit 0 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask0(&self) -> PINMASK0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK0R { bits }
        }
        # [ doc = "Bit 1 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask1(&self) -> PINMASK1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK1R { bits }
        }
        # [ doc = "Bit 2 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask2(&self) -> PINMASK2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK2R { bits }
        }
        # [ doc = "Bit 3 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask3(&self) -> PINMASK3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK3R { bits }
        }
        # [ doc = "Bit 4 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask4(&self) -> PINMASK4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK4R { bits }
        }
        # [ doc = "Bit 5 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask5(&self) -> PINMASK5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK5R { bits }
        }
        # [ doc = "Bit 6 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask6(&self) -> PINMASK6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK6R { bits }
        }
        # [ doc = "Bit 7 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask7(&self) -> PINMASK7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK7R { bits }
        }
        # [ doc = "Bit 8 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask8(&self) -> PINMASK8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK8R { bits }
        }
        # [ doc = "Bit 9 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask9(&self) -> PINMASK9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK9R { bits }
        }
        # [ doc = "Bit 10 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask10(&self) -> PINMASK10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK10R { bits }
        }
        # [ doc = "Bit 11 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask11(&self) -> PINMASK11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK11R { bits }
        }
        # [ doc = "Bit 12 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask12(&self) -> PINMASK12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK12R { bits }
        }
        # [ doc = "Bit 13 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask13(&self) -> PINMASK13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK13R { bits }
        }
        # [ doc = "Bit 14 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask14(&self) -> PINMASK14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK14R { bits }
        }
        # [ doc = "Bit 15 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask15(&self) -> PINMASK15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK15R { bits }
        }
        # [ doc = "Bit 16 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask16(&self) -> PINMASK16R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK16R { bits }
        }
        # [ doc = "Bit 17 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask17(&self) -> PINMASK17R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK17R { bits }
        }
        # [ doc = "Bit 18 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask18(&self) -> PINMASK18R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK18R { bits }
        }
        # [ doc = "Bit 19 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask19(&self) -> PINMASK19R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK19R { bits }
        }
        # [ doc = "Bit 20 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask20(&self) -> PINMASK20R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK20R { bits }
        }
        # [ doc = "Bit 21 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask21(&self) -> PINMASK21R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK21R { bits }
        }
        # [ doc = "Bit 22 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask22(&self) -> PINMASK22R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK22R { bits }
        }
        # [ doc = "Bit 23 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask23(&self) -> PINMASK23R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK23R { bits }
        }
        # [ doc = "Bit 24 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask24(&self) -> PINMASK24R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK24R { bits }
        }
        # [ doc = "Bit 25 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask25(&self) -> PINMASK25R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK25R { bits }
        }
        # [ doc = "Bit 26 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask26(&self) -> PINMASK26R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK26R { bits }
        }
        # [ doc = "Bit 27 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask27(&self) -> PINMASK27R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK27R { bits }
        }
        # [ doc = "Bit 28 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask28(&self) -> PINMASK28R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK28R { bits }
        }
        # [ doc = "Bit 29 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask29(&self) -> PINMASK29R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK29R { bits }
        }
        # [ doc = "Bit 30 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask30(&self) -> PINMASK30R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK30R { bits }
        }
        # [ doc = "Bit 31 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask31(&self) -> PINMASK31R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINMASK31R { bits }
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
        # [ doc = "Bit 0 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask0(&mut self) -> _PINMASK0W {
            _PINMASK0W { w: self }
        }
        # [ doc = "Bit 1 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask1(&mut self) -> _PINMASK1W {
            _PINMASK1W { w: self }
        }
        # [ doc = "Bit 2 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask2(&mut self) -> _PINMASK2W {
            _PINMASK2W { w: self }
        }
        # [ doc = "Bit 3 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask3(&mut self) -> _PINMASK3W {
            _PINMASK3W { w: self }
        }
        # [ doc = "Bit 4 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask4(&mut self) -> _PINMASK4W {
            _PINMASK4W { w: self }
        }
        # [ doc = "Bit 5 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask5(&mut self) -> _PINMASK5W {
            _PINMASK5W { w: self }
        }
        # [ doc = "Bit 6 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask6(&mut self) -> _PINMASK6W {
            _PINMASK6W { w: self }
        }
        # [ doc = "Bit 7 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask7(&mut self) -> _PINMASK7W {
            _PINMASK7W { w: self }
        }
        # [ doc = "Bit 8 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask8(&mut self) -> _PINMASK8W {
            _PINMASK8W { w: self }
        }
        # [ doc = "Bit 9 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask9(&mut self) -> _PINMASK9W {
            _PINMASK9W { w: self }
        }
        # [ doc = "Bit 10 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask10(&mut self) -> _PINMASK10W {
            _PINMASK10W { w: self }
        }
        # [ doc = "Bit 11 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask11(&mut self) -> _PINMASK11W {
            _PINMASK11W { w: self }
        }
        # [ doc = "Bit 12 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask12(&mut self) -> _PINMASK12W {
            _PINMASK12W { w: self }
        }
        # [ doc = "Bit 13 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask13(&mut self) -> _PINMASK13W {
            _PINMASK13W { w: self }
        }
        # [ doc = "Bit 14 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask14(&mut self) -> _PINMASK14W {
            _PINMASK14W { w: self }
        }
        # [ doc = "Bit 15 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask15(&mut self) -> _PINMASK15W {
            _PINMASK15W { w: self }
        }
        # [ doc = "Bit 16 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask16(&mut self) -> _PINMASK16W {
            _PINMASK16W { w: self }
        }
        # [ doc = "Bit 17 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask17(&mut self) -> _PINMASK17W {
            _PINMASK17W { w: self }
        }
        # [ doc = "Bit 18 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask18(&mut self) -> _PINMASK18W {
            _PINMASK18W { w: self }
        }
        # [ doc = "Bit 19 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask19(&mut self) -> _PINMASK19W {
            _PINMASK19W { w: self }
        }
        # [ doc = "Bit 20 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask20(&mut self) -> _PINMASK20W {
            _PINMASK20W { w: self }
        }
        # [ doc = "Bit 21 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask21(&mut self) -> _PINMASK21W {
            _PINMASK21W { w: self }
        }
        # [ doc = "Bit 22 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask22(&mut self) -> _PINMASK22W {
            _PINMASK22W { w: self }
        }
        # [ doc = "Bit 23 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask23(&mut self) -> _PINMASK23W {
            _PINMASK23W { w: self }
        }
        # [ doc = "Bit 24 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask24(&mut self) -> _PINMASK24W {
            _PINMASK24W { w: self }
        }
        # [ doc = "Bit 25 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask25(&mut self) -> _PINMASK25W {
            _PINMASK25W { w: self }
        }
        # [ doc = "Bit 26 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask26(&mut self) -> _PINMASK26W {
            _PINMASK26W { w: self }
        }
        # [ doc = "Bit 27 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask27(&mut self) -> _PINMASK27W {
            _PINMASK27W { w: self }
        }
        # [ doc = "Bit 28 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask28(&mut self) -> _PINMASK28W {
            _PINMASK28W { w: self }
        }
        # [ doc = "Bit 29 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask29(&mut self) -> _PINMASK29W {
            _PINMASK29W { w: self }
        }
        # [ doc = "Bit 30 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask30(&mut self) -> _PINMASK30W {
            _PINMASK30W { w: self }
        }
        # [ doc = "Bit 31 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin." ]
        # [ inline ( always ) ]
        pub fn pinmask31(&mut self) -> _PINMASK31W {
            _PINMASK31W { w: self }
        }
    }
}
# [ doc = "Port Pin value register using FIOMASK." ]
pub struct PIN {
    register: VolatileCell<u32>,
}
# [ doc = "Port Pin value register using FIOMASK." ]
pub mod pin {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::PIN {
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
    pub struct PINVAL0R {
        bits: bool,
    }
    impl PINVAL0R {
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
    pub struct PINVAL1R {
        bits: bool,
    }
    impl PINVAL1R {
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
    pub struct PINVAL2R {
        bits: bool,
    }
    impl PINVAL2R {
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
    pub struct PINVAL3R {
        bits: bool,
    }
    impl PINVAL3R {
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
    pub struct PINVAL4R {
        bits: bool,
    }
    impl PINVAL4R {
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
    pub struct PINVAL5R {
        bits: bool,
    }
    impl PINVAL5R {
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
    pub struct PINVAL6R {
        bits: bool,
    }
    impl PINVAL6R {
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
    pub struct PINVAL7R {
        bits: bool,
    }
    impl PINVAL7R {
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
    pub struct PINVAL8R {
        bits: bool,
    }
    impl PINVAL8R {
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
    pub struct PINVAL9R {
        bits: bool,
    }
    impl PINVAL9R {
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
    pub struct PINVAL10R {
        bits: bool,
    }
    impl PINVAL10R {
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
    pub struct PINVAL11R {
        bits: bool,
    }
    impl PINVAL11R {
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
    pub struct PINVAL12R {
        bits: bool,
    }
    impl PINVAL12R {
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
    pub struct PINVAL13R {
        bits: bool,
    }
    impl PINVAL13R {
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
    pub struct PINVAL14R {
        bits: bool,
    }
    impl PINVAL14R {
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
    pub struct PINVAL15R {
        bits: bool,
    }
    impl PINVAL15R {
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
    pub struct PINVAL16R {
        bits: bool,
    }
    impl PINVAL16R {
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
    pub struct PINVAL17R {
        bits: bool,
    }
    impl PINVAL17R {
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
    pub struct PINVAL18R {
        bits: bool,
    }
    impl PINVAL18R {
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
    pub struct PINVAL19R {
        bits: bool,
    }
    impl PINVAL19R {
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
    pub struct PINVAL20R {
        bits: bool,
    }
    impl PINVAL20R {
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
    pub struct PINVAL21R {
        bits: bool,
    }
    impl PINVAL21R {
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
    pub struct PINVAL22R {
        bits: bool,
    }
    impl PINVAL22R {
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
    pub struct PINVAL23R {
        bits: bool,
    }
    impl PINVAL23R {
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
    pub struct PINVAL24R {
        bits: bool,
    }
    impl PINVAL24R {
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
    pub struct PINVAL25R {
        bits: bool,
    }
    impl PINVAL25R {
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
    pub struct PINVAL26R {
        bits: bool,
    }
    impl PINVAL26R {
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
    pub struct PINVAL27R {
        bits: bool,
    }
    impl PINVAL27R {
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
    pub struct PINVAL28R {
        bits: bool,
    }
    impl PINVAL28R {
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
    pub struct PINVAL29R {
        bits: bool,
    }
    impl PINVAL29R {
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
    pub struct PINVAL30R {
        bits: bool,
    }
    impl PINVAL30R {
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
    pub struct PINVAL31R {
        bits: bool,
    }
    impl PINVAL31R {
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
    pub struct _PINVAL0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL0W<'a> {
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
    pub struct _PINVAL1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL1W<'a> {
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
    pub struct _PINVAL2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL2W<'a> {
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
    pub struct _PINVAL3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL3W<'a> {
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
    pub struct _PINVAL4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL4W<'a> {
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
    pub struct _PINVAL5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL5W<'a> {
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
    pub struct _PINVAL6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL6W<'a> {
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
    pub struct _PINVAL7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL7W<'a> {
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
    pub struct _PINVAL8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL8W<'a> {
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
    pub struct _PINVAL9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL9W<'a> {
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
    pub struct _PINVAL10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL10W<'a> {
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
    pub struct _PINVAL11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL11W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PINVAL12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL12W<'a> {
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
    pub struct _PINVAL13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL13W<'a> {
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
    pub struct _PINVAL14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL14W<'a> {
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
    pub struct _PINVAL15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL15W<'a> {
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
    pub struct _PINVAL16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL16W<'a> {
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
    pub struct _PINVAL17W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL17W<'a> {
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
    pub struct _PINVAL18W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL18W<'a> {
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
    pub struct _PINVAL19W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL19W<'a> {
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
    pub struct _PINVAL20W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL20W<'a> {
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
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINVAL21W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL21W<'a> {
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
    pub struct _PINVAL22W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL22W<'a> {
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
    pub struct _PINVAL23W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL23W<'a> {
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
    pub struct _PINVAL24W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL24W<'a> {
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
    pub struct _PINVAL25W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL25W<'a> {
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
    pub struct _PINVAL26W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL26W<'a> {
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
    pub struct _PINVAL27W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL27W<'a> {
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
    pub struct _PINVAL28W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL28W<'a> {
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
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINVAL29W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL29W<'a> {
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
    pub struct _PINVAL30W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL30W<'a> {
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
    pub struct _PINVAL31W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINVAL31W<'a> {
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
        # [ doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval0(&self) -> PINVAL0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL0R { bits }
        }
        # [ doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval1(&self) -> PINVAL1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL1R { bits }
        }
        # [ doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval2(&self) -> PINVAL2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL2R { bits }
        }
        # [ doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval3(&self) -> PINVAL3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL3R { bits }
        }
        # [ doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval4(&self) -> PINVAL4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL4R { bits }
        }
        # [ doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval5(&self) -> PINVAL5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL5R { bits }
        }
        # [ doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval6(&self) -> PINVAL6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL6R { bits }
        }
        # [ doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval7(&self) -> PINVAL7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL7R { bits }
        }
        # [ doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval8(&self) -> PINVAL8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL8R { bits }
        }
        # [ doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval9(&self) -> PINVAL9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL9R { bits }
        }
        # [ doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval10(&self) -> PINVAL10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL10R { bits }
        }
        # [ doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval11(&self) -> PINVAL11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL11R { bits }
        }
        # [ doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval12(&self) -> PINVAL12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL12R { bits }
        }
        # [ doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval13(&self) -> PINVAL13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL13R { bits }
        }
        # [ doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval14(&self) -> PINVAL14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL14R { bits }
        }
        # [ doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval15(&self) -> PINVAL15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL15R { bits }
        }
        # [ doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval16(&self) -> PINVAL16R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL16R { bits }
        }
        # [ doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval17(&self) -> PINVAL17R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL17R { bits }
        }
        # [ doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval18(&self) -> PINVAL18R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL18R { bits }
        }
        # [ doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval19(&self) -> PINVAL19R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL19R { bits }
        }
        # [ doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval20(&self) -> PINVAL20R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL20R { bits }
        }
        # [ doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval21(&self) -> PINVAL21R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL21R { bits }
        }
        # [ doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval22(&self) -> PINVAL22R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL22R { bits }
        }
        # [ doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval23(&self) -> PINVAL23R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL23R { bits }
        }
        # [ doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval24(&self) -> PINVAL24R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL24R { bits }
        }
        # [ doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval25(&self) -> PINVAL25R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL25R { bits }
        }
        # [ doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval26(&self) -> PINVAL26R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL26R { bits }
        }
        # [ doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval27(&self) -> PINVAL27R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL27R { bits }
        }
        # [ doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval28(&self) -> PINVAL28R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL28R { bits }
        }
        # [ doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval29(&self) -> PINVAL29R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL29R { bits }
        }
        # [ doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval30(&self) -> PINVAL30R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL30R { bits }
        }
        # [ doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval31(&self) -> PINVAL31R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINVAL31R { bits }
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
        # [ doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval0(&mut self) -> _PINVAL0W {
            _PINVAL0W { w: self }
        }
        # [ doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval1(&mut self) -> _PINVAL1W {
            _PINVAL1W { w: self }
        }
        # [ doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval2(&mut self) -> _PINVAL2W {
            _PINVAL2W { w: self }
        }
        # [ doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval3(&mut self) -> _PINVAL3W {
            _PINVAL3W { w: self }
        }
        # [ doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval4(&mut self) -> _PINVAL4W {
            _PINVAL4W { w: self }
        }
        # [ doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval5(&mut self) -> _PINVAL5W {
            _PINVAL5W { w: self }
        }
        # [ doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval6(&mut self) -> _PINVAL6W {
            _PINVAL6W { w: self }
        }
        # [ doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval7(&mut self) -> _PINVAL7W {
            _PINVAL7W { w: self }
        }
        # [ doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval8(&mut self) -> _PINVAL8W {
            _PINVAL8W { w: self }
        }
        # [ doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval9(&mut self) -> _PINVAL9W {
            _PINVAL9W { w: self }
        }
        # [ doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval10(&mut self) -> _PINVAL10W {
            _PINVAL10W { w: self }
        }
        # [ doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval11(&mut self) -> _PINVAL11W {
            _PINVAL11W { w: self }
        }
        # [ doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval12(&mut self) -> _PINVAL12W {
            _PINVAL12W { w: self }
        }
        # [ doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval13(&mut self) -> _PINVAL13W {
            _PINVAL13W { w: self }
        }
        # [ doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval14(&mut self) -> _PINVAL14W {
            _PINVAL14W { w: self }
        }
        # [ doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval15(&mut self) -> _PINVAL15W {
            _PINVAL15W { w: self }
        }
        # [ doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval16(&mut self) -> _PINVAL16W {
            _PINVAL16W { w: self }
        }
        # [ doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval17(&mut self) -> _PINVAL17W {
            _PINVAL17W { w: self }
        }
        # [ doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval18(&mut self) -> _PINVAL18W {
            _PINVAL18W { w: self }
        }
        # [ doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval19(&mut self) -> _PINVAL19W {
            _PINVAL19W { w: self }
        }
        # [ doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval20(&mut self) -> _PINVAL20W {
            _PINVAL20W { w: self }
        }
        # [ doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval21(&mut self) -> _PINVAL21W {
            _PINVAL21W { w: self }
        }
        # [ doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval22(&mut self) -> _PINVAL22W {
            _PINVAL22W { w: self }
        }
        # [ doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval23(&mut self) -> _PINVAL23W {
            _PINVAL23W { w: self }
        }
        # [ doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval24(&mut self) -> _PINVAL24W {
            _PINVAL24W { w: self }
        }
        # [ doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval25(&mut self) -> _PINVAL25W {
            _PINVAL25W { w: self }
        }
        # [ doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval26(&mut self) -> _PINVAL26W {
            _PINVAL26W { w: self }
        }
        # [ doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval27(&mut self) -> _PINVAL27W {
            _PINVAL27W { w: self }
        }
        # [ doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval28(&mut self) -> _PINVAL28W {
            _PINVAL28W { w: self }
        }
        # [ doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval29(&mut self) -> _PINVAL29W {
            _PINVAL29W { w: self }
        }
        # [ doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval30(&mut self) -> _PINVAL30W {
            _PINVAL30W { w: self }
        }
        # [ doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px[0], bit 31 in PINx corresponds to pin Px[31]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinval31(&mut self) -> _PINVAL31W {
            _PINVAL31W { w: self }
        }
    }
}
# [ doc = "Port Output Set register using FIOMASK." ]
pub struct SET {
    register: VolatileCell<u32>,
}
# [ doc = "Port Output Set register using FIOMASK." ]
pub mod set {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SET {
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
    pub struct PINSET0R {
        bits: bool,
    }
    impl PINSET0R {
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
    pub struct PINSET1R {
        bits: bool,
    }
    impl PINSET1R {
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
    pub struct PINSET2R {
        bits: bool,
    }
    impl PINSET2R {
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
    pub struct PINSET3R {
        bits: bool,
    }
    impl PINSET3R {
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
    pub struct PINSET4R {
        bits: bool,
    }
    impl PINSET4R {
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
    pub struct PINSET5R {
        bits: bool,
    }
    impl PINSET5R {
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
    pub struct PINSET6R {
        bits: bool,
    }
    impl PINSET6R {
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
    pub struct PINSET7R {
        bits: bool,
    }
    impl PINSET7R {
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
    pub struct PINSET8R {
        bits: bool,
    }
    impl PINSET8R {
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
    pub struct PINSET9R {
        bits: bool,
    }
    impl PINSET9R {
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
    pub struct PINSET10R {
        bits: bool,
    }
    impl PINSET10R {
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
    pub struct PINSET11R {
        bits: bool,
    }
    impl PINSET11R {
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
    pub struct PINSET12R {
        bits: bool,
    }
    impl PINSET12R {
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
    pub struct PINSET13R {
        bits: bool,
    }
    impl PINSET13R {
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
    pub struct PINSET14R {
        bits: bool,
    }
    impl PINSET14R {
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
    pub struct PINSET15R {
        bits: bool,
    }
    impl PINSET15R {
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
    pub struct PINSET16R {
        bits: bool,
    }
    impl PINSET16R {
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
    pub struct PINSET17R {
        bits: bool,
    }
    impl PINSET17R {
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
    pub struct PINSET18R {
        bits: bool,
    }
    impl PINSET18R {
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
    pub struct PINSET19R {
        bits: bool,
    }
    impl PINSET19R {
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
    pub struct PINSET20R {
        bits: bool,
    }
    impl PINSET20R {
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
    pub struct PINSET21R {
        bits: bool,
    }
    impl PINSET21R {
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
    pub struct PINSET22R {
        bits: bool,
    }
    impl PINSET22R {
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
    pub struct PINSET23R {
        bits: bool,
    }
    impl PINSET23R {
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
    pub struct PINSET24R {
        bits: bool,
    }
    impl PINSET24R {
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
    pub struct PINSET25R {
        bits: bool,
    }
    impl PINSET25R {
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
    pub struct PINSET26R {
        bits: bool,
    }
    impl PINSET26R {
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
    pub struct PINSET27R {
        bits: bool,
    }
    impl PINSET27R {
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
    pub struct PINSET28R {
        bits: bool,
    }
    impl PINSET28R {
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
    pub struct PINSET29R {
        bits: bool,
    }
    impl PINSET29R {
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
    pub struct PINSET30R {
        bits: bool,
    }
    impl PINSET30R {
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
    pub struct PINSET31R {
        bits: bool,
    }
    impl PINSET31R {
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
    pub struct _PINSET0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET0W<'a> {
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
    pub struct _PINSET1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET1W<'a> {
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
    pub struct _PINSET2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET2W<'a> {
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
    pub struct _PINSET3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET3W<'a> {
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
    pub struct _PINSET4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET4W<'a> {
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
    pub struct _PINSET5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET5W<'a> {
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
    pub struct _PINSET6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET6W<'a> {
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
    pub struct _PINSET7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET7W<'a> {
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
    pub struct _PINSET8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET8W<'a> {
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
    pub struct _PINSET9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET9W<'a> {
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
    pub struct _PINSET10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET10W<'a> {
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
    pub struct _PINSET11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET11W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PINSET12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET12W<'a> {
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
    pub struct _PINSET13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET13W<'a> {
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
    pub struct _PINSET14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET14W<'a> {
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
    pub struct _PINSET15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET15W<'a> {
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
    pub struct _PINSET16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET16W<'a> {
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
    pub struct _PINSET17W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET17W<'a> {
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
    pub struct _PINSET18W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET18W<'a> {
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
    pub struct _PINSET19W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET19W<'a> {
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
    pub struct _PINSET20W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET20W<'a> {
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
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINSET21W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET21W<'a> {
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
    pub struct _PINSET22W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET22W<'a> {
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
    pub struct _PINSET23W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET23W<'a> {
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
    pub struct _PINSET24W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET24W<'a> {
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
    pub struct _PINSET25W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET25W<'a> {
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
    pub struct _PINSET26W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET26W<'a> {
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
    pub struct _PINSET27W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET27W<'a> {
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
    pub struct _PINSET28W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET28W<'a> {
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
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINSET29W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET29W<'a> {
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
    pub struct _PINSET30W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET30W<'a> {
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
    pub struct _PINSET31W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINSET31W<'a> {
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
        # [ doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset0(&self) -> PINSET0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET0R { bits }
        }
        # [ doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset1(&self) -> PINSET1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET1R { bits }
        }
        # [ doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset2(&self) -> PINSET2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET2R { bits }
        }
        # [ doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset3(&self) -> PINSET3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET3R { bits }
        }
        # [ doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset4(&self) -> PINSET4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET4R { bits }
        }
        # [ doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset5(&self) -> PINSET5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET5R { bits }
        }
        # [ doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset6(&self) -> PINSET6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET6R { bits }
        }
        # [ doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset7(&self) -> PINSET7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET7R { bits }
        }
        # [ doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset8(&self) -> PINSET8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET8R { bits }
        }
        # [ doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset9(&self) -> PINSET9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET9R { bits }
        }
        # [ doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset10(&self) -> PINSET10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET10R { bits }
        }
        # [ doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset11(&self) -> PINSET11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET11R { bits }
        }
        # [ doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset12(&self) -> PINSET12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET12R { bits }
        }
        # [ doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset13(&self) -> PINSET13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET13R { bits }
        }
        # [ doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset14(&self) -> PINSET14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET14R { bits }
        }
        # [ doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset15(&self) -> PINSET15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET15R { bits }
        }
        # [ doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset16(&self) -> PINSET16R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET16R { bits }
        }
        # [ doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset17(&self) -> PINSET17R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET17R { bits }
        }
        # [ doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset18(&self) -> PINSET18R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET18R { bits }
        }
        # [ doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset19(&self) -> PINSET19R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET19R { bits }
        }
        # [ doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset20(&self) -> PINSET20R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET20R { bits }
        }
        # [ doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset21(&self) -> PINSET21R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET21R { bits }
        }
        # [ doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset22(&self) -> PINSET22R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET22R { bits }
        }
        # [ doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset23(&self) -> PINSET23R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET23R { bits }
        }
        # [ doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset24(&self) -> PINSET24R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET24R { bits }
        }
        # [ doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset25(&self) -> PINSET25R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET25R { bits }
        }
        # [ doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset26(&self) -> PINSET26R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET26R { bits }
        }
        # [ doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset27(&self) -> PINSET27R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET27R { bits }
        }
        # [ doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset28(&self) -> PINSET28R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET28R { bits }
        }
        # [ doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset29(&self) -> PINSET29R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET29R { bits }
        }
        # [ doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset30(&self) -> PINSET30R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET30R { bits }
        }
        # [ doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset31(&self) -> PINSET31R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PINSET31R { bits }
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
        # [ doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset0(&mut self) -> _PINSET0W {
            _PINSET0W { w: self }
        }
        # [ doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset1(&mut self) -> _PINSET1W {
            _PINSET1W { w: self }
        }
        # [ doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset2(&mut self) -> _PINSET2W {
            _PINSET2W { w: self }
        }
        # [ doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset3(&mut self) -> _PINSET3W {
            _PINSET3W { w: self }
        }
        # [ doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset4(&mut self) -> _PINSET4W {
            _PINSET4W { w: self }
        }
        # [ doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset5(&mut self) -> _PINSET5W {
            _PINSET5W { w: self }
        }
        # [ doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset6(&mut self) -> _PINSET6W {
            _PINSET6W { w: self }
        }
        # [ doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset7(&mut self) -> _PINSET7W {
            _PINSET7W { w: self }
        }
        # [ doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset8(&mut self) -> _PINSET8W {
            _PINSET8W { w: self }
        }
        # [ doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset9(&mut self) -> _PINSET9W {
            _PINSET9W { w: self }
        }
        # [ doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset10(&mut self) -> _PINSET10W {
            _PINSET10W { w: self }
        }
        # [ doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset11(&mut self) -> _PINSET11W {
            _PINSET11W { w: self }
        }
        # [ doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset12(&mut self) -> _PINSET12W {
            _PINSET12W { w: self }
        }
        # [ doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset13(&mut self) -> _PINSET13W {
            _PINSET13W { w: self }
        }
        # [ doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset14(&mut self) -> _PINSET14W {
            _PINSET14W { w: self }
        }
        # [ doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset15(&mut self) -> _PINSET15W {
            _PINSET15W { w: self }
        }
        # [ doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset16(&mut self) -> _PINSET16W {
            _PINSET16W { w: self }
        }
        # [ doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset17(&mut self) -> _PINSET17W {
            _PINSET17W { w: self }
        }
        # [ doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset18(&mut self) -> _PINSET18W {
            _PINSET18W { w: self }
        }
        # [ doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset19(&mut self) -> _PINSET19W {
            _PINSET19W { w: self }
        }
        # [ doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset20(&mut self) -> _PINSET20W {
            _PINSET20W { w: self }
        }
        # [ doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset21(&mut self) -> _PINSET21W {
            _PINSET21W { w: self }
        }
        # [ doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset22(&mut self) -> _PINSET22W {
            _PINSET22W { w: self }
        }
        # [ doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset23(&mut self) -> _PINSET23W {
            _PINSET23W { w: self }
        }
        # [ doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset24(&mut self) -> _PINSET24W {
            _PINSET24W { w: self }
        }
        # [ doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset25(&mut self) -> _PINSET25W {
            _PINSET25W { w: self }
        }
        # [ doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset26(&mut self) -> _PINSET26W {
            _PINSET26W { w: self }
        }
        # [ doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset27(&mut self) -> _PINSET27W {
            _PINSET27W { w: self }
        }
        # [ doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset28(&mut self) -> _PINSET28W {
            _PINSET28W { w: self }
        }
        # [ doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset29(&mut self) -> _PINSET29W {
            _PINSET29W { w: self }
        }
        # [ doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset30(&mut self) -> _PINSET30W {
            _PINSET30W { w: self }
        }
        # [ doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px[0], bit 31 in SETx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH." ]
        # [ inline ( always ) ]
        pub fn pinset31(&mut self) -> _PINSET31W {
            _PINSET31W { w: self }
        }
    }
}
# [ doc = "Port Output Clear register using FIOMASK." ]
pub struct CLR {
    register: VolatileCell<u32>,
}
# [ doc = "Port Output Clear register using FIOMASK." ]
pub mod clr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CLR {
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
    pub struct _PINCLR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR0W<'a> {
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
    pub struct _PINCLR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR1W<'a> {
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
    pub struct _PINCLR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR2W<'a> {
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
    pub struct _PINCLR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR3W<'a> {
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
    pub struct _PINCLR4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR4W<'a> {
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
    pub struct _PINCLR5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR5W<'a> {
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
    pub struct _PINCLR6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR6W<'a> {
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
    pub struct _PINCLR7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR7W<'a> {
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
    pub struct _PINCLR8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR8W<'a> {
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
    pub struct _PINCLR9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR9W<'a> {
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
    pub struct _PINCLR10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR10W<'a> {
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
    pub struct _PINCLR11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR11W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _PINCLR12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR12W<'a> {
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
    pub struct _PINCLR13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR13W<'a> {
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
    pub struct _PINCLR14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR14W<'a> {
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
    pub struct _PINCLR15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR15W<'a> {
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
    pub struct _PINCLR16W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR16W<'a> {
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
    pub struct _PINCLR17W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR17W<'a> {
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
    pub struct _PINCLR18W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR18W<'a> {
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
    pub struct _PINCLR19W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR19W<'a> {
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
    pub struct _PINCLR20W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR20W<'a> {
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
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINCLR21W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR21W<'a> {
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
    pub struct _PINCLR22W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR22W<'a> {
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
    pub struct _PINCLR23W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR23W<'a> {
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
    pub struct _PINCLR24W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR24W<'a> {
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
    pub struct _PINCLR25W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR25W<'a> {
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
    pub struct _PINCLR26W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR26W<'a> {
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
    pub struct _PINCLR27W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR27W<'a> {
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
    pub struct _PINCLR28W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR28W<'a> {
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
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PINCLR29W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR29W<'a> {
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
    pub struct _PINCLR30W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR30W<'a> {
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
    pub struct _PINCLR31W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINCLR31W<'a> {
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
        # [ doc = "Bit 0 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr0(&mut self) -> _PINCLR0W {
            _PINCLR0W { w: self }
        }
        # [ doc = "Bit 1 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr1(&mut self) -> _PINCLR1W {
            _PINCLR1W { w: self }
        }
        # [ doc = "Bit 2 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr2(&mut self) -> _PINCLR2W {
            _PINCLR2W { w: self }
        }
        # [ doc = "Bit 3 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr3(&mut self) -> _PINCLR3W {
            _PINCLR3W { w: self }
        }
        # [ doc = "Bit 4 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr4(&mut self) -> _PINCLR4W {
            _PINCLR4W { w: self }
        }
        # [ doc = "Bit 5 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr5(&mut self) -> _PINCLR5W {
            _PINCLR5W { w: self }
        }
        # [ doc = "Bit 6 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr6(&mut self) -> _PINCLR6W {
            _PINCLR6W { w: self }
        }
        # [ doc = "Bit 7 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr7(&mut self) -> _PINCLR7W {
            _PINCLR7W { w: self }
        }
        # [ doc = "Bit 8 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr8(&mut self) -> _PINCLR8W {
            _PINCLR8W { w: self }
        }
        # [ doc = "Bit 9 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr9(&mut self) -> _PINCLR9W {
            _PINCLR9W { w: self }
        }
        # [ doc = "Bit 10 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr10(&mut self) -> _PINCLR10W {
            _PINCLR10W { w: self }
        }
        # [ doc = "Bit 11 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr11(&mut self) -> _PINCLR11W {
            _PINCLR11W { w: self }
        }
        # [ doc = "Bit 12 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr12(&mut self) -> _PINCLR12W {
            _PINCLR12W { w: self }
        }
        # [ doc = "Bit 13 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr13(&mut self) -> _PINCLR13W {
            _PINCLR13W { w: self }
        }
        # [ doc = "Bit 14 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr14(&mut self) -> _PINCLR14W {
            _PINCLR14W { w: self }
        }
        # [ doc = "Bit 15 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr15(&mut self) -> _PINCLR15W {
            _PINCLR15W { w: self }
        }
        # [ doc = "Bit 16 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr16(&mut self) -> _PINCLR16W {
            _PINCLR16W { w: self }
        }
        # [ doc = "Bit 17 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr17(&mut self) -> _PINCLR17W {
            _PINCLR17W { w: self }
        }
        # [ doc = "Bit 18 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr18(&mut self) -> _PINCLR18W {
            _PINCLR18W { w: self }
        }
        # [ doc = "Bit 19 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr19(&mut self) -> _PINCLR19W {
            _PINCLR19W { w: self }
        }
        # [ doc = "Bit 20 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr20(&mut self) -> _PINCLR20W {
            _PINCLR20W { w: self }
        }
        # [ doc = "Bit 21 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr21(&mut self) -> _PINCLR21W {
            _PINCLR21W { w: self }
        }
        # [ doc = "Bit 22 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr22(&mut self) -> _PINCLR22W {
            _PINCLR22W { w: self }
        }
        # [ doc = "Bit 23 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr23(&mut self) -> _PINCLR23W {
            _PINCLR23W { w: self }
        }
        # [ doc = "Bit 24 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr24(&mut self) -> _PINCLR24W {
            _PINCLR24W { w: self }
        }
        # [ doc = "Bit 25 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr25(&mut self) -> _PINCLR25W {
            _PINCLR25W { w: self }
        }
        # [ doc = "Bit 26 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr26(&mut self) -> _PINCLR26W {
            _PINCLR26W { w: self }
        }
        # [ doc = "Bit 27 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr27(&mut self) -> _PINCLR27W {
            _PINCLR27W { w: self }
        }
        # [ doc = "Bit 28 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr28(&mut self) -> _PINCLR28W {
            _PINCLR28W { w: self }
        }
        # [ doc = "Bit 29 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr29(&mut self) -> _PINCLR29W {
            _PINCLR29W { w: self }
        }
        # [ doc = "Bit 30 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr30(&mut self) -> _PINCLR30W {
            _PINCLR30W { w: self }
        }
        # [ doc = "Bit 31 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px[0], bit 31 in CLRx controls pin Px[31]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW." ]
        # [ inline ( always ) ]
        pub fn pinclr31(&mut self) -> _PINCLR31W {
            _PINCLR31W { w: self }
        }
    }
}
# [ doc = "General Purpose I/O" ]
pub struct GPIO {
    register_block: RegisterBlock,
}
impl Deref for GPIO {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
