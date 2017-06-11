# ! [ doc = "GPIO" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "GPIO" ]
pub const GPIOINT: Peripheral<GPIOINT> = unsafe { Peripheral::new(1073905792) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - GPIO overall Interrupt Status." ]
    pub status: STATUS,
    # [ doc = "0x04 - GPIO Interrupt Status for Rising edge for Port 0." ]
    pub statr0: STATR0,
    # [ doc = "0x08 - GPIO Interrupt Status for Falling edge for Port 0." ]
    pub statf0: STATF0,
    # [ doc = "0x0c - GPIO Interrupt Clear." ]
    pub clr0: CLR0,
    # [ doc = "0x10 - GPIO Interrupt Enable for Rising edge for Port 0." ]
    pub enr0: ENR0,
    # [ doc = "0x14 - GPIO Interrupt Enable for Falling edge for Port 0." ]
    pub enf0: ENF0,
    _reserved0: [u8; 12usize],
    # [ doc = "0x24 - GPIO Interrupt Status for Rising edge for Port 0." ]
    pub statr2: STATR2,
    # [ doc = "0x28 - GPIO Interrupt Status for Falling edge for Port 0." ]
    pub statf2: STATF2,
    # [ doc = "0x2c - GPIO Interrupt Clear." ]
    pub clr2: CLR2,
    # [ doc = "0x30 - GPIO Interrupt Enable for Rising edge for Port 0." ]
    pub enr2: ENR2,
    # [ doc = "0x34 - GPIO Interrupt Enable for Falling edge for Port 0." ]
    pub enf2: ENF2,
}
# [ doc = "GPIO overall Interrupt Status." ]
pub struct STATUS {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO overall Interrupt Status." ]
pub mod status {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::STATUS {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = "Possible values of the field `P0INT`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P0INTR {
        # [ doc = "No pending interrupts on Port 0." ]
        NO_PENDING_INTERRUPT,
        # [ doc = "At least one pending interrupt on Port 0." ]
        AT_LEAST_ONE_PENDING,
    }
    impl P0INTR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                P0INTR::NO_PENDING_INTERRUPT => false,
                P0INTR::AT_LEAST_ONE_PENDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P0INTR {
            match value {
                false => P0INTR::NO_PENDING_INTERRUPT,
                true => P0INTR::AT_LEAST_ONE_PENDING,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`" ]
        # [ inline ( always ) ]
        pub fn is_no_pending_interrupt(&self) -> bool {
            *self == P0INTR::NO_PENDING_INTERRUPT
        }
        # [ doc = "Checks if the value of the field is `AT_LEAST_ONE_PENDING`" ]
        # [ inline ( always ) ]
        pub fn is_at_least_one_pending(&self) -> bool {
            *self == P0INTR::AT_LEAST_ONE_PENDING
        }
    }
    # [ doc = "Possible values of the field `P2INT`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum P2INTR {
        # [ doc = "No pending interrupts on Port 2." ]
        NO_PENDING_INTERRUPT,
        # [ doc = "At least one pending interrupt on Port 2." ]
        AT_LEAST_ONE_PENDING,
    }
    impl P2INTR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                P2INTR::NO_PENDING_INTERRUPT => false,
                P2INTR::AT_LEAST_ONE_PENDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> P2INTR {
            match value {
                false => P2INTR::NO_PENDING_INTERRUPT,
                true => P2INTR::AT_LEAST_ONE_PENDING,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`" ]
        # [ inline ( always ) ]
        pub fn is_no_pending_interrupt(&self) -> bool {
            *self == P2INTR::NO_PENDING_INTERRUPT
        }
        # [ doc = "Checks if the value of the field is `AT_LEAST_ONE_PENDING`" ]
        # [ inline ( always ) ]
        pub fn is_at_least_one_pending(&self) -> bool {
            *self == P2INTR::AT_LEAST_ONE_PENDING
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Port 0 GPIO interrupt pending." ]
        # [ inline ( always ) ]
        pub fn p0int(&self) -> P0INTR {
            P0INTR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 2 - Port 2 GPIO interrupt pending." ]
        # [ inline ( always ) ]
        pub fn p2int(&self) -> P2INTR {
            P2INTR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 2;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
    }
}
# [ doc = "GPIO Interrupt Status for Rising edge for Port 0." ]
pub struct STATR0 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Status for Rising edge for Port 0." ]
pub mod statr0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::STATR0 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct P0_0REIR {
        bits: bool,
    }
    impl P0_0REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_1REIR {
        bits: bool,
    }
    impl P0_1REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_2REIR {
        bits: bool,
    }
    impl P0_2REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_3REIR {
        bits: bool,
    }
    impl P0_3REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_4REIR {
        bits: bool,
    }
    impl P0_4REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_5REIR {
        bits: bool,
    }
    impl P0_5REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_6REIR {
        bits: bool,
    }
    impl P0_6REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_7REIR {
        bits: bool,
    }
    impl P0_7REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_8REIR {
        bits: bool,
    }
    impl P0_8REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_9REIR {
        bits: bool,
    }
    impl P0_9REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_10REIR {
        bits: bool,
    }
    impl P0_10REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_11REIR {
        bits: bool,
    }
    impl P0_11REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_12REIR {
        bits: bool,
    }
    impl P0_12REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_13REIR {
        bits: bool,
    }
    impl P0_13REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_14REIR {
        bits: bool,
    }
    impl P0_14REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_15REIR {
        bits: bool,
    }
    impl P0_15REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_16REIR {
        bits: bool,
    }
    impl P0_16REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_17REIR {
        bits: bool,
    }
    impl P0_17REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_18REIR {
        bits: bool,
    }
    impl P0_18REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_19REIR {
        bits: bool,
    }
    impl P0_19REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_20REIR {
        bits: bool,
    }
    impl P0_20REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_21REIR {
        bits: bool,
    }
    impl P0_21REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_22REIR {
        bits: bool,
    }
    impl P0_22REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_23REIR {
        bits: bool,
    }
    impl P0_23REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_24REIR {
        bits: bool,
    }
    impl P0_24REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_25REIR {
        bits: bool,
    }
    impl P0_25REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_26REIR {
        bits: bool,
    }
    impl P0_26REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_27REIR {
        bits: bool,
    }
    impl P0_27REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_28REIR {
        bits: bool,
    }
    impl P0_28REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_29REIR {
        bits: bool,
    }
    impl P0_29REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_30REIR {
        bits: bool,
    }
    impl P0_30REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - Status of Rising Edge Interrupt for P0[0]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_0rei(&self) -> P0_0REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_0REIR { bits }
        }
        # [ doc = "Bit 1 - Status of Rising Edge Interrupt for P0[1]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_1rei(&self) -> P0_1REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_1REIR { bits }
        }
        # [ doc = "Bit 2 - Status of Rising Edge Interrupt for P0[2]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_2rei(&self) -> P0_2REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_2REIR { bits }
        }
        # [ doc = "Bit 3 - Status of Rising Edge Interrupt for P0[3]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_3rei(&self) -> P0_3REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_3REIR { bits }
        }
        # [ doc = "Bit 4 - Status of Rising Edge Interrupt for P0[4]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_4rei(&self) -> P0_4REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_4REIR { bits }
        }
        # [ doc = "Bit 5 - Status of Rising Edge Interrupt for P0[5]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_5rei(&self) -> P0_5REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_5REIR { bits }
        }
        # [ doc = "Bit 6 - Status of Rising Edge Interrupt for P0[6]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_6rei(&self) -> P0_6REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_6REIR { bits }
        }
        # [ doc = "Bit 7 - Status of Rising Edge Interrupt for P0[7]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_7rei(&self) -> P0_7REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_7REIR { bits }
        }
        # [ doc = "Bit 8 - Status of Rising Edge Interrupt for P0[8]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_8rei(&self) -> P0_8REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_8REIR { bits }
        }
        # [ doc = "Bit 9 - Status of Rising Edge Interrupt for P0[9]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_9rei(&self) -> P0_9REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_9REIR { bits }
        }
        # [ doc = "Bit 10 - Status of Rising Edge Interrupt for P0[10]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_10rei(&self) -> P0_10REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_10REIR { bits }
        }
        # [ doc = "Bit 11 - Status of Rising Edge Interrupt for P0[11]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_11rei(&self) -> P0_11REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_11REIR { bits }
        }
        # [ doc = "Bit 12 - Status of Rising Edge Interrupt for P0[12]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_12rei(&self) -> P0_12REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_12REIR { bits }
        }
        # [ doc = "Bit 13 - Status of Rising Edge Interrupt for P0[13]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_13rei(&self) -> P0_13REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_13REIR { bits }
        }
        # [ doc = "Bit 14 - Status of Rising Edge Interrupt for P0[14]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_14rei(&self) -> P0_14REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_14REIR { bits }
        }
        # [ doc = "Bit 15 - Status of Rising Edge Interrupt for P0[15]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_15rei(&self) -> P0_15REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_15REIR { bits }
        }
        # [ doc = "Bit 16 - Status of Rising Edge Interrupt for P0[16]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_16rei(&self) -> P0_16REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_16REIR { bits }
        }
        # [ doc = "Bit 17 - Status of Rising Edge Interrupt for P0[17]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_17rei(&self) -> P0_17REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_17REIR { bits }
        }
        # [ doc = "Bit 18 - Status of Rising Edge Interrupt for P0[18]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_18rei(&self) -> P0_18REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_18REIR { bits }
        }
        # [ doc = "Bit 19 - Status of Rising Edge Interrupt for P0[19]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_19rei(&self) -> P0_19REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_19REIR { bits }
        }
        # [ doc = "Bit 20 - Status of Rising Edge Interrupt for P0[20]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_20rei(&self) -> P0_20REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_20REIR { bits }
        }
        # [ doc = "Bit 21 - Status of Rising Edge Interrupt for P0[21]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_21rei(&self) -> P0_21REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_21REIR { bits }
        }
        # [ doc = "Bit 22 - Status of Rising Edge Interrupt for P0[22]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_22rei(&self) -> P0_22REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_22REIR { bits }
        }
        # [ doc = "Bit 23 - Status of Rising Edge Interrupt for P0[23]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_23rei(&self) -> P0_23REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_23REIR { bits }
        }
        # [ doc = "Bit 24 - Status of Rising Edge Interrupt for P0[24]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_24rei(&self) -> P0_24REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_24REIR { bits }
        }
        # [ doc = "Bit 25 - Status of Rising Edge Interrupt for P0[25]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_25rei(&self) -> P0_25REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_25REIR { bits }
        }
        # [ doc = "Bit 26 - Status of Rising Edge Interrupt for P0[26]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_26rei(&self) -> P0_26REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_26REIR { bits }
        }
        # [ doc = "Bit 27 - Status of Rising Edge Interrupt for P0[27]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_27rei(&self) -> P0_27REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_27REIR { bits }
        }
        # [ doc = "Bit 28 - Status of Rising Edge Interrupt for P0[28]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_28rei(&self) -> P0_28REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_28REIR { bits }
        }
        # [ doc = "Bit 29 - Status of Rising Edge Interrupt for P0[29]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_29rei(&self) -> P0_29REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_29REIR { bits }
        }
        # [ doc = "Bit 30 - Status of Rising Edge Interrupt for P0[30]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_30rei(&self) -> P0_30REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_30REIR { bits }
        }
    }
}
# [ doc = "GPIO Interrupt Status for Falling edge for Port 0." ]
pub struct STATF0 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Status for Falling edge for Port 0." ]
pub mod statf0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::STATF0 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct P0_0FEIR {
        bits: bool,
    }
    impl P0_0FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_1FEIR {
        bits: bool,
    }
    impl P0_1FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_2FEIR {
        bits: bool,
    }
    impl P0_2FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_3FEIR {
        bits: bool,
    }
    impl P0_3FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_4FEIR {
        bits: bool,
    }
    impl P0_4FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_5FEIR {
        bits: bool,
    }
    impl P0_5FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_6FEIR {
        bits: bool,
    }
    impl P0_6FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_7FEIR {
        bits: bool,
    }
    impl P0_7FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_8FEIR {
        bits: bool,
    }
    impl P0_8FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_9FEIR {
        bits: bool,
    }
    impl P0_9FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_10FEIR {
        bits: bool,
    }
    impl P0_10FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_11FEIR {
        bits: bool,
    }
    impl P0_11FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_12FEIR {
        bits: bool,
    }
    impl P0_12FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_13FEIR {
        bits: bool,
    }
    impl P0_13FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_14FEIR {
        bits: bool,
    }
    impl P0_14FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_15FEIR {
        bits: bool,
    }
    impl P0_15FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_16FEIR {
        bits: bool,
    }
    impl P0_16FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_17FEIR {
        bits: bool,
    }
    impl P0_17FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_18FEIR {
        bits: bool,
    }
    impl P0_18FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_19FEIR {
        bits: bool,
    }
    impl P0_19FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_20FEIR {
        bits: bool,
    }
    impl P0_20FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_21FEIR {
        bits: bool,
    }
    impl P0_21FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_22FEIR {
        bits: bool,
    }
    impl P0_22FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_23FEIR {
        bits: bool,
    }
    impl P0_23FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_24FEIR {
        bits: bool,
    }
    impl P0_24FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_25FEIR {
        bits: bool,
    }
    impl P0_25FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_26FEIR {
        bits: bool,
    }
    impl P0_26FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_27FEIR {
        bits: bool,
    }
    impl P0_27FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_28FEIR {
        bits: bool,
    }
    impl P0_28FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_29FEIR {
        bits: bool,
    }
    impl P0_29FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_30FEIR {
        bits: bool,
    }
    impl P0_30FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - Status of Falling Edge Interrupt for P0[0]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_0fei(&self) -> P0_0FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_0FEIR { bits }
        }
        # [ doc = "Bit 1 - Status of Falling Edge Interrupt for P0[1]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_1fei(&self) -> P0_1FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_1FEIR { bits }
        }
        # [ doc = "Bit 2 - Status of Falling Edge Interrupt for P0[2]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_2fei(&self) -> P0_2FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_2FEIR { bits }
        }
        # [ doc = "Bit 3 - Status of Falling Edge Interrupt for P0[3]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_3fei(&self) -> P0_3FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_3FEIR { bits }
        }
        # [ doc = "Bit 4 - Status of Falling Edge Interrupt for P0[4]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_4fei(&self) -> P0_4FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_4FEIR { bits }
        }
        # [ doc = "Bit 5 - Status of Falling Edge Interrupt for P0[5]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_5fei(&self) -> P0_5FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_5FEIR { bits }
        }
        # [ doc = "Bit 6 - Status of Falling Edge Interrupt for P0[6]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_6fei(&self) -> P0_6FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_6FEIR { bits }
        }
        # [ doc = "Bit 7 - Status of Falling Edge Interrupt for P0[7]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_7fei(&self) -> P0_7FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_7FEIR { bits }
        }
        # [ doc = "Bit 8 - Status of Falling Edge Interrupt for P0[8]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_8fei(&self) -> P0_8FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_8FEIR { bits }
        }
        # [ doc = "Bit 9 - Status of Falling Edge Interrupt for P0[9]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_9fei(&self) -> P0_9FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_9FEIR { bits }
        }
        # [ doc = "Bit 10 - Status of Falling Edge Interrupt for P0[10]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_10fei(&self) -> P0_10FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_10FEIR { bits }
        }
        # [ doc = "Bit 11 - Status of Falling Edge Interrupt for P0[11]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_11fei(&self) -> P0_11FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_11FEIR { bits }
        }
        # [ doc = "Bit 12 - Status of Falling Edge Interrupt for P0[12]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_12fei(&self) -> P0_12FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_12FEIR { bits }
        }
        # [ doc = "Bit 13 - Status of Falling Edge Interrupt for P0[13]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_13fei(&self) -> P0_13FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_13FEIR { bits }
        }
        # [ doc = "Bit 14 - Status of Falling Edge Interrupt for P0[14]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_14fei(&self) -> P0_14FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_14FEIR { bits }
        }
        # [ doc = "Bit 15 - Status of Falling Edge Interrupt for P0[15]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_15fei(&self) -> P0_15FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_15FEIR { bits }
        }
        # [ doc = "Bit 16 - Status of Falling Edge Interrupt for P0[16]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_16fei(&self) -> P0_16FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_16FEIR { bits }
        }
        # [ doc = "Bit 17 - Status of Falling Edge Interrupt for P0[17]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_17fei(&self) -> P0_17FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_17FEIR { bits }
        }
        # [ doc = "Bit 18 - Status of Falling Edge Interrupt for P0[18]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_18fei(&self) -> P0_18FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_18FEIR { bits }
        }
        # [ doc = "Bit 19 - Status of Falling Edge Interrupt for P0[19]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_19fei(&self) -> P0_19FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_19FEIR { bits }
        }
        # [ doc = "Bit 20 - Status of Falling Edge Interrupt for P0[20]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_20fei(&self) -> P0_20FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_20FEIR { bits }
        }
        # [ doc = "Bit 21 - Status of Falling Edge Interrupt for P0[21]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_21fei(&self) -> P0_21FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_21FEIR { bits }
        }
        # [ doc = "Bit 22 - Status of Falling Edge Interrupt for P0[22]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_22fei(&self) -> P0_22FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_22FEIR { bits }
        }
        # [ doc = "Bit 23 - Status of Falling Edge Interrupt for P0[23]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_23fei(&self) -> P0_23FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_23FEIR { bits }
        }
        # [ doc = "Bit 24 - Status of Falling Edge Interrupt for P0[24]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_24fei(&self) -> P0_24FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_24FEIR { bits }
        }
        # [ doc = "Bit 25 - Status of Falling Edge Interrupt for P0[25]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_25fei(&self) -> P0_25FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_25FEIR { bits }
        }
        # [ doc = "Bit 26 - Status of Falling Edge Interrupt for P0[26]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_26fei(&self) -> P0_26FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_26FEIR { bits }
        }
        # [ doc = "Bit 27 - Status of Falling Edge Interrupt for P0[27]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_27fei(&self) -> P0_27FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_27FEIR { bits }
        }
        # [ doc = "Bit 28 - Status of Falling Edge Interrupt for P0[28]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_28fei(&self) -> P0_28FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_28FEIR { bits }
        }
        # [ doc = "Bit 29 - Status of Falling Edge Interrupt for P0[29]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_29fei(&self) -> P0_29FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_29FEIR { bits }
        }
        # [ doc = "Bit 30 - Status of Falling Edge Interrupt for P0[30]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p0_30fei(&self) -> P0_30FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_30FEIR { bits }
        }
    }
}
# [ doc = "GPIO Interrupt Clear." ]
pub struct CLR0 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Clear." ]
pub mod clr0 {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CLR0 {
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
    pub struct _P0_0CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_0CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_1CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_1CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_2CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_2CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_3CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_3CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_4CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_4CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_5CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_5CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_6CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_6CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_7CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_7CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_8CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_8CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_9CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_9CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_10CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_10CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_11CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_11CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_12CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_12CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_13CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_13CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_14CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_14CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_15CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_15CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_16CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_16CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_17CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_17CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_18CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_18CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_19CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_19CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_20CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_20CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_21CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_21CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_22CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_22CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_23CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_23CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_24CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_24CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_25CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_25CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_26CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_26CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_27CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_27CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_28CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_28CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_29CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_29CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_30CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_30CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Clear GPIO port Interrupts for P0[0]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_0ci(&mut self) -> _P0_0CIW {
            _P0_0CIW { w: self }
        }
        # [ doc = "Bit 1 - Clear GPIO port Interrupts for P0[1]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_1ci(&mut self) -> _P0_1CIW {
            _P0_1CIW { w: self }
        }
        # [ doc = "Bit 2 - Clear GPIO port Interrupts for P0[2]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_2ci(&mut self) -> _P0_2CIW {
            _P0_2CIW { w: self }
        }
        # [ doc = "Bit 3 - Clear GPIO port Interrupts for P0[3]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_3ci(&mut self) -> _P0_3CIW {
            _P0_3CIW { w: self }
        }
        # [ doc = "Bit 4 - Clear GPIO port Interrupts for P0[4]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_4ci(&mut self) -> _P0_4CIW {
            _P0_4CIW { w: self }
        }
        # [ doc = "Bit 5 - Clear GPIO port Interrupts for P0[5]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_5ci(&mut self) -> _P0_5CIW {
            _P0_5CIW { w: self }
        }
        # [ doc = "Bit 6 - Clear GPIO port Interrupts for P0[6]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_6ci(&mut self) -> _P0_6CIW {
            _P0_6CIW { w: self }
        }
        # [ doc = "Bit 7 - Clear GPIO port Interrupts for P0[7]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_7ci(&mut self) -> _P0_7CIW {
            _P0_7CIW { w: self }
        }
        # [ doc = "Bit 8 - Clear GPIO port Interrupts for P0[8]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_8ci(&mut self) -> _P0_8CIW {
            _P0_8CIW { w: self }
        }
        # [ doc = "Bit 9 - Clear GPIO port Interrupts for P0[9]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_9ci(&mut self) -> _P0_9CIW {
            _P0_9CIW { w: self }
        }
        # [ doc = "Bit 10 - Clear GPIO port Interrupts for P0[10]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_10ci(&mut self) -> _P0_10CIW {
            _P0_10CIW { w: self }
        }
        # [ doc = "Bit 11 - Clear GPIO port Interrupts for P0[11]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_11ci(&mut self) -> _P0_11CIW {
            _P0_11CIW { w: self }
        }
        # [ doc = "Bit 12 - Clear GPIO port Interrupts for P0[12]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_12ci(&mut self) -> _P0_12CIW {
            _P0_12CIW { w: self }
        }
        # [ doc = "Bit 13 - Clear GPIO port Interrupts for P0[13]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_13ci(&mut self) -> _P0_13CIW {
            _P0_13CIW { w: self }
        }
        # [ doc = "Bit 14 - Clear GPIO port Interrupts for P0[14]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_14ci(&mut self) -> _P0_14CIW {
            _P0_14CIW { w: self }
        }
        # [ doc = "Bit 15 - Clear GPIO port Interrupts for P0[15]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_15ci(&mut self) -> _P0_15CIW {
            _P0_15CIW { w: self }
        }
        # [ doc = "Bit 16 - Clear GPIO port Interrupts for P0[16]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_16ci(&mut self) -> _P0_16CIW {
            _P0_16CIW { w: self }
        }
        # [ doc = "Bit 17 - Clear GPIO port Interrupts for P0[17]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_17ci(&mut self) -> _P0_17CIW {
            _P0_17CIW { w: self }
        }
        # [ doc = "Bit 18 - Clear GPIO port Interrupts for P0[18]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_18ci(&mut self) -> _P0_18CIW {
            _P0_18CIW { w: self }
        }
        # [ doc = "Bit 19 - Clear GPIO port Interrupts for P0[19]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_19ci(&mut self) -> _P0_19CIW {
            _P0_19CIW { w: self }
        }
        # [ doc = "Bit 20 - Clear GPIO port Interrupts for P0[20]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_20ci(&mut self) -> _P0_20CIW {
            _P0_20CIW { w: self }
        }
        # [ doc = "Bit 21 - Clear GPIO port Interrupts for P0[21]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_21ci(&mut self) -> _P0_21CIW {
            _P0_21CIW { w: self }
        }
        # [ doc = "Bit 22 - Clear GPIO port Interrupts for P0[22]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_22ci(&mut self) -> _P0_22CIW {
            _P0_22CIW { w: self }
        }
        # [ doc = "Bit 23 - Clear GPIO port Interrupts for P0[23]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_23ci(&mut self) -> _P0_23CIW {
            _P0_23CIW { w: self }
        }
        # [ doc = "Bit 24 - Clear GPIO port Interrupts for P0[24]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_24ci(&mut self) -> _P0_24CIW {
            _P0_24CIW { w: self }
        }
        # [ doc = "Bit 25 - Clear GPIO port Interrupts for P0[25]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_25ci(&mut self) -> _P0_25CIW {
            _P0_25CIW { w: self }
        }
        # [ doc = "Bit 26 - Clear GPIO port Interrupts for P0[26]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_26ci(&mut self) -> _P0_26CIW {
            _P0_26CIW { w: self }
        }
        # [ doc = "Bit 27 - Clear GPIO port Interrupts for P0[27]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_27ci(&mut self) -> _P0_27CIW {
            _P0_27CIW { w: self }
        }
        # [ doc = "Bit 28 - Clear GPIO port Interrupts for P0[28]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_28ci(&mut self) -> _P0_28CIW {
            _P0_28CIW { w: self }
        }
        # [ doc = "Bit 29 - Clear GPIO port Interrupts for P0[29]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_29ci(&mut self) -> _P0_29CIW {
            _P0_29CIW { w: self }
        }
        # [ doc = "Bit 30 - Clear GPIO port Interrupts for P0[30]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p0_30ci(&mut self) -> _P0_30CIW {
            _P0_30CIW { w: self }
        }
    }
}
# [ doc = "GPIO Interrupt Enable for Rising edge for Port 0." ]
pub struct ENR0 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Enable for Rising edge for Port 0." ]
pub mod enr0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ENR0 {
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
    pub struct P0_0ERR {
        bits: bool,
    }
    impl P0_0ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_1ERR {
        bits: bool,
    }
    impl P0_1ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_2ERR {
        bits: bool,
    }
    impl P0_2ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_3ERR {
        bits: bool,
    }
    impl P0_3ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_4ERR {
        bits: bool,
    }
    impl P0_4ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_5ERR {
        bits: bool,
    }
    impl P0_5ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_6ERR {
        bits: bool,
    }
    impl P0_6ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_7ERR {
        bits: bool,
    }
    impl P0_7ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_8ERR {
        bits: bool,
    }
    impl P0_8ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_9ERR {
        bits: bool,
    }
    impl P0_9ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_10ERR {
        bits: bool,
    }
    impl P0_10ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_11ERR {
        bits: bool,
    }
    impl P0_11ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_12ERR {
        bits: bool,
    }
    impl P0_12ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_13ERR {
        bits: bool,
    }
    impl P0_13ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_14ERR {
        bits: bool,
    }
    impl P0_14ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_15ERR {
        bits: bool,
    }
    impl P0_15ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_16ERR {
        bits: bool,
    }
    impl P0_16ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_17ERR {
        bits: bool,
    }
    impl P0_17ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_18ERR {
        bits: bool,
    }
    impl P0_18ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_19ERR {
        bits: bool,
    }
    impl P0_19ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_20ERR {
        bits: bool,
    }
    impl P0_20ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_21ERR {
        bits: bool,
    }
    impl P0_21ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_22ERR {
        bits: bool,
    }
    impl P0_22ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_23ERR {
        bits: bool,
    }
    impl P0_23ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_24ERR {
        bits: bool,
    }
    impl P0_24ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_25ERR {
        bits: bool,
    }
    impl P0_25ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_26ERR {
        bits: bool,
    }
    impl P0_26ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_27ERR {
        bits: bool,
    }
    impl P0_27ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_28ERR {
        bits: bool,
    }
    impl P0_28ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_29ERR {
        bits: bool,
    }
    impl P0_29ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_30ERR {
        bits: bool,
    }
    impl P0_30ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _P0_0ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_0ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_1ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_1ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_2ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_2ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_3ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_3ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_4ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_4ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_5ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_5ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_6ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_6ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_7ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_7ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_8ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_8ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_9ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_9ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_10ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_10ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_11ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_11ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_12ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_12ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_13ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_13ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_14ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_14ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_15ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_15ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_16ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_16ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_17ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_17ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_18ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_18ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_19ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_19ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_20ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_20ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_21ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_21ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_22ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_22ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_23ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_23ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_24ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_24ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_25ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_25ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_26ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_26ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_27ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_27ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_28ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_28ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_29ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_29ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_30ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_30ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Enable rising edge interrupt for P0[0]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_0er(&self) -> P0_0ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_0ERR { bits }
        }
        # [ doc = "Bit 1 - Enable rising edge interrupt for P0[1]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_1er(&self) -> P0_1ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_1ERR { bits }
        }
        # [ doc = "Bit 2 - Enable rising edge interrupt for P0[2]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_2er(&self) -> P0_2ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_2ERR { bits }
        }
        # [ doc = "Bit 3 - Enable rising edge interrupt for P0[3]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_3er(&self) -> P0_3ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_3ERR { bits }
        }
        # [ doc = "Bit 4 - Enable rising edge interrupt for P0[4]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_4er(&self) -> P0_4ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_4ERR { bits }
        }
        # [ doc = "Bit 5 - Enable rising edge interrupt for P0[5]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_5er(&self) -> P0_5ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_5ERR { bits }
        }
        # [ doc = "Bit 6 - Enable rising edge interrupt for P0[6]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_6er(&self) -> P0_6ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_6ERR { bits }
        }
        # [ doc = "Bit 7 - Enable rising edge interrupt for P0[7]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_7er(&self) -> P0_7ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_7ERR { bits }
        }
        # [ doc = "Bit 8 - Enable rising edge interrupt for P0[8]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_8er(&self) -> P0_8ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_8ERR { bits }
        }
        # [ doc = "Bit 9 - Enable rising edge interrupt for P0[9]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_9er(&self) -> P0_9ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_9ERR { bits }
        }
        # [ doc = "Bit 10 - Enable rising edge interrupt for P0[10]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_10er(&self) -> P0_10ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_10ERR { bits }
        }
        # [ doc = "Bit 11 - Enable rising edge interrupt for P0[11]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_11er(&self) -> P0_11ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_11ERR { bits }
        }
        # [ doc = "Bit 12 - Enable rising edge interrupt for P0[12]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_12er(&self) -> P0_12ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_12ERR { bits }
        }
        # [ doc = "Bit 13 - Enable rising edge interrupt for P0[13]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_13er(&self) -> P0_13ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_13ERR { bits }
        }
        # [ doc = "Bit 14 - Enable rising edge interrupt for P0[14]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_14er(&self) -> P0_14ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_14ERR { bits }
        }
        # [ doc = "Bit 15 - Enable rising edge interrupt for P0[15]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_15er(&self) -> P0_15ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_15ERR { bits }
        }
        # [ doc = "Bit 16 - Enable rising edge interrupt for P0[16]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_16er(&self) -> P0_16ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_16ERR { bits }
        }
        # [ doc = "Bit 17 - Enable rising edge interrupt for P0[17]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_17er(&self) -> P0_17ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_17ERR { bits }
        }
        # [ doc = "Bit 18 - Enable rising edge interrupt for P0[18]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_18er(&self) -> P0_18ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_18ERR { bits }
        }
        # [ doc = "Bit 19 - Enable rising edge interrupt for P0[19]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_19er(&self) -> P0_19ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_19ERR { bits }
        }
        # [ doc = "Bit 20 - Enable rising edge interrupt for P0[20]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_20er(&self) -> P0_20ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_20ERR { bits }
        }
        # [ doc = "Bit 21 - Enable rising edge interrupt for P0[21]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_21er(&self) -> P0_21ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_21ERR { bits }
        }
        # [ doc = "Bit 22 - Enable rising edge interrupt for P0[22]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_22er(&self) -> P0_22ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_22ERR { bits }
        }
        # [ doc = "Bit 23 - Enable rising edge interrupt for P0[23]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_23er(&self) -> P0_23ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_23ERR { bits }
        }
        # [ doc = "Bit 24 - Enable rising edge interrupt for P0[24]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_24er(&self) -> P0_24ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_24ERR { bits }
        }
        # [ doc = "Bit 25 - Enable rising edge interrupt for P0[25]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_25er(&self) -> P0_25ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_25ERR { bits }
        }
        # [ doc = "Bit 26 - Enable rising edge interrupt for P0[26]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_26er(&self) -> P0_26ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_26ERR { bits }
        }
        # [ doc = "Bit 27 - Enable rising edge interrupt for P0[27]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_27er(&self) -> P0_27ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_27ERR { bits }
        }
        # [ doc = "Bit 28 - Enable rising edge interrupt for P0[28]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_28er(&self) -> P0_28ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_28ERR { bits }
        }
        # [ doc = "Bit 29 - Enable rising edge interrupt for P0[29]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_29er(&self) -> P0_29ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_29ERR { bits }
        }
        # [ doc = "Bit 30 - Enable rising edge interrupt for P0[30]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_30er(&self) -> P0_30ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_30ERR { bits }
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
        # [ doc = "Bit 0 - Enable rising edge interrupt for P0[0]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_0er(&mut self) -> _P0_0ERW {
            _P0_0ERW { w: self }
        }
        # [ doc = "Bit 1 - Enable rising edge interrupt for P0[1]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_1er(&mut self) -> _P0_1ERW {
            _P0_1ERW { w: self }
        }
        # [ doc = "Bit 2 - Enable rising edge interrupt for P0[2]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_2er(&mut self) -> _P0_2ERW {
            _P0_2ERW { w: self }
        }
        # [ doc = "Bit 3 - Enable rising edge interrupt for P0[3]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_3er(&mut self) -> _P0_3ERW {
            _P0_3ERW { w: self }
        }
        # [ doc = "Bit 4 - Enable rising edge interrupt for P0[4]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_4er(&mut self) -> _P0_4ERW {
            _P0_4ERW { w: self }
        }
        # [ doc = "Bit 5 - Enable rising edge interrupt for P0[5]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_5er(&mut self) -> _P0_5ERW {
            _P0_5ERW { w: self }
        }
        # [ doc = "Bit 6 - Enable rising edge interrupt for P0[6]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_6er(&mut self) -> _P0_6ERW {
            _P0_6ERW { w: self }
        }
        # [ doc = "Bit 7 - Enable rising edge interrupt for P0[7]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_7er(&mut self) -> _P0_7ERW {
            _P0_7ERW { w: self }
        }
        # [ doc = "Bit 8 - Enable rising edge interrupt for P0[8]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_8er(&mut self) -> _P0_8ERW {
            _P0_8ERW { w: self }
        }
        # [ doc = "Bit 9 - Enable rising edge interrupt for P0[9]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_9er(&mut self) -> _P0_9ERW {
            _P0_9ERW { w: self }
        }
        # [ doc = "Bit 10 - Enable rising edge interrupt for P0[10]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_10er(&mut self) -> _P0_10ERW {
            _P0_10ERW { w: self }
        }
        # [ doc = "Bit 11 - Enable rising edge interrupt for P0[11]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_11er(&mut self) -> _P0_11ERW {
            _P0_11ERW { w: self }
        }
        # [ doc = "Bit 12 - Enable rising edge interrupt for P0[12]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_12er(&mut self) -> _P0_12ERW {
            _P0_12ERW { w: self }
        }
        # [ doc = "Bit 13 - Enable rising edge interrupt for P0[13]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_13er(&mut self) -> _P0_13ERW {
            _P0_13ERW { w: self }
        }
        # [ doc = "Bit 14 - Enable rising edge interrupt for P0[14]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_14er(&mut self) -> _P0_14ERW {
            _P0_14ERW { w: self }
        }
        # [ doc = "Bit 15 - Enable rising edge interrupt for P0[15]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_15er(&mut self) -> _P0_15ERW {
            _P0_15ERW { w: self }
        }
        # [ doc = "Bit 16 - Enable rising edge interrupt for P0[16]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_16er(&mut self) -> _P0_16ERW {
            _P0_16ERW { w: self }
        }
        # [ doc = "Bit 17 - Enable rising edge interrupt for P0[17]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_17er(&mut self) -> _P0_17ERW {
            _P0_17ERW { w: self }
        }
        # [ doc = "Bit 18 - Enable rising edge interrupt for P0[18]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_18er(&mut self) -> _P0_18ERW {
            _P0_18ERW { w: self }
        }
        # [ doc = "Bit 19 - Enable rising edge interrupt for P0[19]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_19er(&mut self) -> _P0_19ERW {
            _P0_19ERW { w: self }
        }
        # [ doc = "Bit 20 - Enable rising edge interrupt for P0[20]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_20er(&mut self) -> _P0_20ERW {
            _P0_20ERW { w: self }
        }
        # [ doc = "Bit 21 - Enable rising edge interrupt for P0[21]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_21er(&mut self) -> _P0_21ERW {
            _P0_21ERW { w: self }
        }
        # [ doc = "Bit 22 - Enable rising edge interrupt for P0[22]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_22er(&mut self) -> _P0_22ERW {
            _P0_22ERW { w: self }
        }
        # [ doc = "Bit 23 - Enable rising edge interrupt for P0[23]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_23er(&mut self) -> _P0_23ERW {
            _P0_23ERW { w: self }
        }
        # [ doc = "Bit 24 - Enable rising edge interrupt for P0[24]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_24er(&mut self) -> _P0_24ERW {
            _P0_24ERW { w: self }
        }
        # [ doc = "Bit 25 - Enable rising edge interrupt for P0[25]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_25er(&mut self) -> _P0_25ERW {
            _P0_25ERW { w: self }
        }
        # [ doc = "Bit 26 - Enable rising edge interrupt for P0[26]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_26er(&mut self) -> _P0_26ERW {
            _P0_26ERW { w: self }
        }
        # [ doc = "Bit 27 - Enable rising edge interrupt for P0[27]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_27er(&mut self) -> _P0_27ERW {
            _P0_27ERW { w: self }
        }
        # [ doc = "Bit 28 - Enable rising edge interrupt for P0[28]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_28er(&mut self) -> _P0_28ERW {
            _P0_28ERW { w: self }
        }
        # [ doc = "Bit 29 - Enable rising edge interrupt for P0[29]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_29er(&mut self) -> _P0_29ERW {
            _P0_29ERW { w: self }
        }
        # [ doc = "Bit 30 - Enable rising edge interrupt for P0[30]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_30er(&mut self) -> _P0_30ERW {
            _P0_30ERW { w: self }
        }
    }
}
# [ doc = "GPIO Interrupt Enable for Falling edge for Port 0." ]
pub struct ENF0 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Enable for Falling edge for Port 0." ]
pub mod enf0 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ENF0 {
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
    pub struct P0_0EFR {
        bits: bool,
    }
    impl P0_0EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_1EFR {
        bits: bool,
    }
    impl P0_1EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_2EFR {
        bits: bool,
    }
    impl P0_2EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_3EFR {
        bits: bool,
    }
    impl P0_3EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_4EFR {
        bits: bool,
    }
    impl P0_4EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_5EFR {
        bits: bool,
    }
    impl P0_5EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_6EFR {
        bits: bool,
    }
    impl P0_6EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_7EFR {
        bits: bool,
    }
    impl P0_7EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_8EFR {
        bits: bool,
    }
    impl P0_8EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_9EFR {
        bits: bool,
    }
    impl P0_9EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_10EFR {
        bits: bool,
    }
    impl P0_10EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_11EFR {
        bits: bool,
    }
    impl P0_11EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_12EFR {
        bits: bool,
    }
    impl P0_12EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_13EFR {
        bits: bool,
    }
    impl P0_13EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_14EFR {
        bits: bool,
    }
    impl P0_14EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_15EFR {
        bits: bool,
    }
    impl P0_15EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_16EFR {
        bits: bool,
    }
    impl P0_16EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_17EFR {
        bits: bool,
    }
    impl P0_17EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_18EFR {
        bits: bool,
    }
    impl P0_18EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_19EFR {
        bits: bool,
    }
    impl P0_19EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_20EFR {
        bits: bool,
    }
    impl P0_20EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_21EFR {
        bits: bool,
    }
    impl P0_21EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_22EFR {
        bits: bool,
    }
    impl P0_22EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_23EFR {
        bits: bool,
    }
    impl P0_23EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_24EFR {
        bits: bool,
    }
    impl P0_24EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_25EFR {
        bits: bool,
    }
    impl P0_25EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_26EFR {
        bits: bool,
    }
    impl P0_26EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_27EFR {
        bits: bool,
    }
    impl P0_27EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_28EFR {
        bits: bool,
    }
    impl P0_28EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_29EFR {
        bits: bool,
    }
    impl P0_29EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P0_30EFR {
        bits: bool,
    }
    impl P0_30EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _P0_0EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_0EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_1EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_1EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_2EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_2EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_3EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_3EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_4EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_4EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_5EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_5EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_6EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_6EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_7EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_7EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_8EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_8EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_9EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_9EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_10EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_10EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_11EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_11EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_12EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_12EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_13EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_13EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_14EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_14EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_15EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_15EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_16EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_16EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_17EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_17EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_18EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_18EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_19EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_19EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_20EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_20EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_21EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_21EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_22EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_22EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_23EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_23EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_24EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_24EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_25EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_25EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_26EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_26EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_27EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_27EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_28EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_28EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_29EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_29EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P0_30EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P0_30EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Enable falling edge interrupt for P0[0]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_0ef(&self) -> P0_0EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_0EFR { bits }
        }
        # [ doc = "Bit 1 - Enable falling edge interrupt for P0[1]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_1ef(&self) -> P0_1EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_1EFR { bits }
        }
        # [ doc = "Bit 2 - Enable falling edge interrupt for P0[2]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_2ef(&self) -> P0_2EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_2EFR { bits }
        }
        # [ doc = "Bit 3 - Enable falling edge interrupt for P0[3]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_3ef(&self) -> P0_3EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_3EFR { bits }
        }
        # [ doc = "Bit 4 - Enable falling edge interrupt for P0[4]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_4ef(&self) -> P0_4EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_4EFR { bits }
        }
        # [ doc = "Bit 5 - Enable falling edge interrupt for P0[5]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_5ef(&self) -> P0_5EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_5EFR { bits }
        }
        # [ doc = "Bit 6 - Enable falling edge interrupt for P0[6]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_6ef(&self) -> P0_6EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_6EFR { bits }
        }
        # [ doc = "Bit 7 - Enable falling edge interrupt for P0[7]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_7ef(&self) -> P0_7EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_7EFR { bits }
        }
        # [ doc = "Bit 8 - Enable falling edge interrupt for P0[8]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_8ef(&self) -> P0_8EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_8EFR { bits }
        }
        # [ doc = "Bit 9 - Enable falling edge interrupt for P0[9]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_9ef(&self) -> P0_9EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_9EFR { bits }
        }
        # [ doc = "Bit 10 - Enable falling edge interrupt for P0[10]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_10ef(&self) -> P0_10EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_10EFR { bits }
        }
        # [ doc = "Bit 11 - Enable falling edge interrupt for P0[11]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_11ef(&self) -> P0_11EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_11EFR { bits }
        }
        # [ doc = "Bit 12 - Enable falling edge interrupt for P0[12]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_12ef(&self) -> P0_12EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_12EFR { bits }
        }
        # [ doc = "Bit 13 - Enable falling edge interrupt for P0[13]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_13ef(&self) -> P0_13EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_13EFR { bits }
        }
        # [ doc = "Bit 14 - Enable falling edge interrupt for P0[14]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_14ef(&self) -> P0_14EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_14EFR { bits }
        }
        # [ doc = "Bit 15 - Enable falling edge interrupt for P0[15]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_15ef(&self) -> P0_15EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_15EFR { bits }
        }
        # [ doc = "Bit 16 - Enable falling edge interrupt for P0[16]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_16ef(&self) -> P0_16EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_16EFR { bits }
        }
        # [ doc = "Bit 17 - Enable falling edge interrupt for P0[17]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_17ef(&self) -> P0_17EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_17EFR { bits }
        }
        # [ doc = "Bit 18 - Enable falling edge interrupt for P0[18]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_18ef(&self) -> P0_18EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_18EFR { bits }
        }
        # [ doc = "Bit 19 - Enable falling edge interrupt for P0[19]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_19ef(&self) -> P0_19EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_19EFR { bits }
        }
        # [ doc = "Bit 20 - Enable falling edge interrupt for P0[20]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_20ef(&self) -> P0_20EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_20EFR { bits }
        }
        # [ doc = "Bit 21 - Enable falling edge interrupt for P0[21]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_21ef(&self) -> P0_21EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_21EFR { bits }
        }
        # [ doc = "Bit 22 - Enable falling edge interrupt for P0[22]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_22ef(&self) -> P0_22EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_22EFR { bits }
        }
        # [ doc = "Bit 23 - Enable falling edge interrupt for P0[23]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_23ef(&self) -> P0_23EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_23EFR { bits }
        }
        # [ doc = "Bit 24 - Enable falling edge interrupt for P0[24]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_24ef(&self) -> P0_24EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_24EFR { bits }
        }
        # [ doc = "Bit 25 - Enable falling edge interrupt for P0[25]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_25ef(&self) -> P0_25EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_25EFR { bits }
        }
        # [ doc = "Bit 26 - Enable falling edge interrupt for P0[26]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_26ef(&self) -> P0_26EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_26EFR { bits }
        }
        # [ doc = "Bit 27 - Enable falling edge interrupt for P0[27]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_27ef(&self) -> P0_27EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_27EFR { bits }
        }
        # [ doc = "Bit 28 - Enable falling edge interrupt for P0[28]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_28ef(&self) -> P0_28EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_28EFR { bits }
        }
        # [ doc = "Bit 29 - Enable falling edge interrupt for P0[29]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_29ef(&self) -> P0_29EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_29EFR { bits }
        }
        # [ doc = "Bit 30 - Enable falling edge interrupt for P0[30]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_30ef(&self) -> P0_30EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P0_30EFR { bits }
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
        # [ doc = "Bit 0 - Enable falling edge interrupt for P0[0]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_0ef(&mut self) -> _P0_0EFW {
            _P0_0EFW { w: self }
        }
        # [ doc = "Bit 1 - Enable falling edge interrupt for P0[1]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_1ef(&mut self) -> _P0_1EFW {
            _P0_1EFW { w: self }
        }
        # [ doc = "Bit 2 - Enable falling edge interrupt for P0[2]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_2ef(&mut self) -> _P0_2EFW {
            _P0_2EFW { w: self }
        }
        # [ doc = "Bit 3 - Enable falling edge interrupt for P0[3]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_3ef(&mut self) -> _P0_3EFW {
            _P0_3EFW { w: self }
        }
        # [ doc = "Bit 4 - Enable falling edge interrupt for P0[4]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_4ef(&mut self) -> _P0_4EFW {
            _P0_4EFW { w: self }
        }
        # [ doc = "Bit 5 - Enable falling edge interrupt for P0[5]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_5ef(&mut self) -> _P0_5EFW {
            _P0_5EFW { w: self }
        }
        # [ doc = "Bit 6 - Enable falling edge interrupt for P0[6]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_6ef(&mut self) -> _P0_6EFW {
            _P0_6EFW { w: self }
        }
        # [ doc = "Bit 7 - Enable falling edge interrupt for P0[7]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_7ef(&mut self) -> _P0_7EFW {
            _P0_7EFW { w: self }
        }
        # [ doc = "Bit 8 - Enable falling edge interrupt for P0[8]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_8ef(&mut self) -> _P0_8EFW {
            _P0_8EFW { w: self }
        }
        # [ doc = "Bit 9 - Enable falling edge interrupt for P0[9]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_9ef(&mut self) -> _P0_9EFW {
            _P0_9EFW { w: self }
        }
        # [ doc = "Bit 10 - Enable falling edge interrupt for P0[10]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_10ef(&mut self) -> _P0_10EFW {
            _P0_10EFW { w: self }
        }
        # [ doc = "Bit 11 - Enable falling edge interrupt for P0[11]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_11ef(&mut self) -> _P0_11EFW {
            _P0_11EFW { w: self }
        }
        # [ doc = "Bit 12 - Enable falling edge interrupt for P0[12]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_12ef(&mut self) -> _P0_12EFW {
            _P0_12EFW { w: self }
        }
        # [ doc = "Bit 13 - Enable falling edge interrupt for P0[13]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_13ef(&mut self) -> _P0_13EFW {
            _P0_13EFW { w: self }
        }
        # [ doc = "Bit 14 - Enable falling edge interrupt for P0[14]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_14ef(&mut self) -> _P0_14EFW {
            _P0_14EFW { w: self }
        }
        # [ doc = "Bit 15 - Enable falling edge interrupt for P0[15]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_15ef(&mut self) -> _P0_15EFW {
            _P0_15EFW { w: self }
        }
        # [ doc = "Bit 16 - Enable falling edge interrupt for P0[16]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_16ef(&mut self) -> _P0_16EFW {
            _P0_16EFW { w: self }
        }
        # [ doc = "Bit 17 - Enable falling edge interrupt for P0[17]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_17ef(&mut self) -> _P0_17EFW {
            _P0_17EFW { w: self }
        }
        # [ doc = "Bit 18 - Enable falling edge interrupt for P0[18]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_18ef(&mut self) -> _P0_18EFW {
            _P0_18EFW { w: self }
        }
        # [ doc = "Bit 19 - Enable falling edge interrupt for P0[19]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_19ef(&mut self) -> _P0_19EFW {
            _P0_19EFW { w: self }
        }
        # [ doc = "Bit 20 - Enable falling edge interrupt for P0[20]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_20ef(&mut self) -> _P0_20EFW {
            _P0_20EFW { w: self }
        }
        # [ doc = "Bit 21 - Enable falling edge interrupt for P0[21]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_21ef(&mut self) -> _P0_21EFW {
            _P0_21EFW { w: self }
        }
        # [ doc = "Bit 22 - Enable falling edge interrupt for P0[22]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_22ef(&mut self) -> _P0_22EFW {
            _P0_22EFW { w: self }
        }
        # [ doc = "Bit 23 - Enable falling edge interrupt for P0[23]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_23ef(&mut self) -> _P0_23EFW {
            _P0_23EFW { w: self }
        }
        # [ doc = "Bit 24 - Enable falling edge interrupt for P0[24]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_24ef(&mut self) -> _P0_24EFW {
            _P0_24EFW { w: self }
        }
        # [ doc = "Bit 25 - Enable falling edge interrupt for P0[25]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_25ef(&mut self) -> _P0_25EFW {
            _P0_25EFW { w: self }
        }
        # [ doc = "Bit 26 - Enable falling edge interrupt for P0[26]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_26ef(&mut self) -> _P0_26EFW {
            _P0_26EFW { w: self }
        }
        # [ doc = "Bit 27 - Enable falling edge interrupt for P0[27]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_27ef(&mut self) -> _P0_27EFW {
            _P0_27EFW { w: self }
        }
        # [ doc = "Bit 28 - Enable falling edge interrupt for P0[28]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_28ef(&mut self) -> _P0_28EFW {
            _P0_28EFW { w: self }
        }
        # [ doc = "Bit 29 - Enable falling edge interrupt for P0[29]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_29ef(&mut self) -> _P0_29EFW {
            _P0_29EFW { w: self }
        }
        # [ doc = "Bit 30 - Enable falling edge interrupt for P0[30]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p0_30ef(&mut self) -> _P0_30EFW {
            _P0_30EFW { w: self }
        }
    }
}
# [ doc = "GPIO Interrupt Status for Rising edge for Port 0." ]
pub struct STATR2 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Status for Rising edge for Port 0." ]
pub mod statr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::STATR2 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct P2_0REIR {
        bits: bool,
    }
    impl P2_0REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_1REIR {
        bits: bool,
    }
    impl P2_1REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_2REIR {
        bits: bool,
    }
    impl P2_2REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_3REIR {
        bits: bool,
    }
    impl P2_3REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_4REIR {
        bits: bool,
    }
    impl P2_4REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_5REIR {
        bits: bool,
    }
    impl P2_5REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_6REIR {
        bits: bool,
    }
    impl P2_6REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_7REIR {
        bits: bool,
    }
    impl P2_7REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_8REIR {
        bits: bool,
    }
    impl P2_8REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_9REIR {
        bits: bool,
    }
    impl P2_9REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_10REIR {
        bits: bool,
    }
    impl P2_10REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_11REIR {
        bits: bool,
    }
    impl P2_11REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_12REIR {
        bits: bool,
    }
    impl P2_12REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_13REIR {
        bits: bool,
    }
    impl P2_13REIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - Status of Rising Edge Interrupt for P2[0]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_0rei(&self) -> P2_0REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_0REIR { bits }
        }
        # [ doc = "Bit 1 - Status of Rising Edge Interrupt for P2[1]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_1rei(&self) -> P2_1REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_1REIR { bits }
        }
        # [ doc = "Bit 2 - Status of Rising Edge Interrupt for P2[2]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_2rei(&self) -> P2_2REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_2REIR { bits }
        }
        # [ doc = "Bit 3 - Status of Rising Edge Interrupt for P2[3]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_3rei(&self) -> P2_3REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_3REIR { bits }
        }
        # [ doc = "Bit 4 - Status of Rising Edge Interrupt for P2[4]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_4rei(&self) -> P2_4REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_4REIR { bits }
        }
        # [ doc = "Bit 5 - Status of Rising Edge Interrupt for P2[5]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_5rei(&self) -> P2_5REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_5REIR { bits }
        }
        # [ doc = "Bit 6 - Status of Rising Edge Interrupt for P2[6]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_6rei(&self) -> P2_6REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_6REIR { bits }
        }
        # [ doc = "Bit 7 - Status of Rising Edge Interrupt for P2[7]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_7rei(&self) -> P2_7REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_7REIR { bits }
        }
        # [ doc = "Bit 8 - Status of Rising Edge Interrupt for P2[8]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_8rei(&self) -> P2_8REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_8REIR { bits }
        }
        # [ doc = "Bit 9 - Status of Rising Edge Interrupt for P2[9]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_9rei(&self) -> P2_9REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_9REIR { bits }
        }
        # [ doc = "Bit 10 - Status of Rising Edge Interrupt for P2[10]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_10rei(&self) -> P2_10REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_10REIR { bits }
        }
        # [ doc = "Bit 11 - Status of Rising Edge Interrupt for P2[11]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_11rei(&self) -> P2_11REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_11REIR { bits }
        }
        # [ doc = "Bit 12 - Status of Rising Edge Interrupt for P2[12]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_12rei(&self) -> P2_12REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_12REIR { bits }
        }
        # [ doc = "Bit 13 - Status of Rising Edge Interrupt for P2[13]. 0 = No rising edge detected. 1 = Rising edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_13rei(&self) -> P2_13REIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_13REIR { bits }
        }
    }
}
# [ doc = "GPIO Interrupt Status for Falling edge for Port 0." ]
pub struct STATF2 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Status for Falling edge for Port 0." ]
pub mod statf2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::STATF2 {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct P2_0FEIR {
        bits: bool,
    }
    impl P2_0FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_1FEIR {
        bits: bool,
    }
    impl P2_1FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_2FEIR {
        bits: bool,
    }
    impl P2_2FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_3FEIR {
        bits: bool,
    }
    impl P2_3FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_4FEIR {
        bits: bool,
    }
    impl P2_4FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_5FEIR {
        bits: bool,
    }
    impl P2_5FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_6FEIR {
        bits: bool,
    }
    impl P2_6FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_7FEIR {
        bits: bool,
    }
    impl P2_7FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_8FEIR {
        bits: bool,
    }
    impl P2_8FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_9FEIR {
        bits: bool,
    }
    impl P2_9FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_10FEIR {
        bits: bool,
    }
    impl P2_10FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_11FEIR {
        bits: bool,
    }
    impl P2_11FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_12FEIR {
        bits: bool,
    }
    impl P2_12FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_13FEIR {
        bits: bool,
    }
    impl P2_13FEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - Status of Falling Edge Interrupt for P2[0]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_0fei(&self) -> P2_0FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_0FEIR { bits }
        }
        # [ doc = "Bit 1 - Status of Falling Edge Interrupt for P2[1]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_1fei(&self) -> P2_1FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_1FEIR { bits }
        }
        # [ doc = "Bit 2 - Status of Falling Edge Interrupt for P2[2]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_2fei(&self) -> P2_2FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_2FEIR { bits }
        }
        # [ doc = "Bit 3 - Status of Falling Edge Interrupt for P2[3]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_3fei(&self) -> P2_3FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_3FEIR { bits }
        }
        # [ doc = "Bit 4 - Status of Falling Edge Interrupt for P2[4]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_4fei(&self) -> P2_4FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_4FEIR { bits }
        }
        # [ doc = "Bit 5 - Status of Falling Edge Interrupt for P2[5]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_5fei(&self) -> P2_5FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_5FEIR { bits }
        }
        # [ doc = "Bit 6 - Status of Falling Edge Interrupt for P2[6]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_6fei(&self) -> P2_6FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_6FEIR { bits }
        }
        # [ doc = "Bit 7 - Status of Falling Edge Interrupt for P2[7]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_7fei(&self) -> P2_7FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_7FEIR { bits }
        }
        # [ doc = "Bit 8 - Status of Falling Edge Interrupt for P2[8]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_8fei(&self) -> P2_8FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_8FEIR { bits }
        }
        # [ doc = "Bit 9 - Status of Falling Edge Interrupt for P2[9]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_9fei(&self) -> P2_9FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_9FEIR { bits }
        }
        # [ doc = "Bit 10 - Status of Falling Edge Interrupt for P2[10]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_10fei(&self) -> P2_10FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_10FEIR { bits }
        }
        # [ doc = "Bit 11 - Status of Falling Edge Interrupt for P2[11]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_11fei(&self) -> P2_11FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_11FEIR { bits }
        }
        # [ doc = "Bit 12 - Status of Falling Edge Interrupt for P2[12]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_12fei(&self) -> P2_12FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_12FEIR { bits }
        }
        # [ doc = "Bit 13 - Status of Falling Edge Interrupt for P2[13]. 0 = No falling edge detected. 1 = Falling edge interrupt generated." ]
        # [ inline ( always ) ]
        pub fn p2_13fei(&self) -> P2_13FEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_13FEIR { bits }
        }
    }
}
# [ doc = "GPIO Interrupt Clear." ]
pub struct CLR2 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Clear." ]
pub mod clr2 {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CLR2 {
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
    pub struct _P2_0CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_0CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_1CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_1CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_2CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_2CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_3CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_3CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_4CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_4CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_5CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_5CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_6CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_6CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_7CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_7CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_8CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_8CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_9CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_9CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_10CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_10CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_11CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_11CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_12CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_12CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_13CIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_13CIW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Clear GPIO port Interrupts for P2[0]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_0ci(&mut self) -> _P2_0CIW {
            _P2_0CIW { w: self }
        }
        # [ doc = "Bit 1 - Clear GPIO port Interrupts for P2[1]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_1ci(&mut self) -> _P2_1CIW {
            _P2_1CIW { w: self }
        }
        # [ doc = "Bit 2 - Clear GPIO port Interrupts for P2[2]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_2ci(&mut self) -> _P2_2CIW {
            _P2_2CIW { w: self }
        }
        # [ doc = "Bit 3 - Clear GPIO port Interrupts for P2[3]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_3ci(&mut self) -> _P2_3CIW {
            _P2_3CIW { w: self }
        }
        # [ doc = "Bit 4 - Clear GPIO port Interrupts for P2[4]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_4ci(&mut self) -> _P2_4CIW {
            _P2_4CIW { w: self }
        }
        # [ doc = "Bit 5 - Clear GPIO port Interrupts for P2[5]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_5ci(&mut self) -> _P2_5CIW {
            _P2_5CIW { w: self }
        }
        # [ doc = "Bit 6 - Clear GPIO port Interrupts for P2[6]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_6ci(&mut self) -> _P2_6CIW {
            _P2_6CIW { w: self }
        }
        # [ doc = "Bit 7 - Clear GPIO port Interrupts for P2[7]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_7ci(&mut self) -> _P2_7CIW {
            _P2_7CIW { w: self }
        }
        # [ doc = "Bit 8 - Clear GPIO port Interrupts for P2[8]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_8ci(&mut self) -> _P2_8CIW {
            _P2_8CIW { w: self }
        }
        # [ doc = "Bit 9 - Clear GPIO port Interrupts for P2[9]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_9ci(&mut self) -> _P2_9CIW {
            _P2_9CIW { w: self }
        }
        # [ doc = "Bit 10 - Clear GPIO port Interrupts for P2[10]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_10ci(&mut self) -> _P2_10CIW {
            _P2_10CIW { w: self }
        }
        # [ doc = "Bit 11 - Clear GPIO port Interrupts for P2[11]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_11ci(&mut self) -> _P2_11CIW {
            _P2_11CIW { w: self }
        }
        # [ doc = "Bit 12 - Clear GPIO port Interrupts for P2[12]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_12ci(&mut self) -> _P2_12CIW {
            _P2_12CIW { w: self }
        }
        # [ doc = "Bit 13 - Clear GPIO port Interrupts for P2[13]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF." ]
        # [ inline ( always ) ]
        pub fn p2_13ci(&mut self) -> _P2_13CIW {
            _P2_13CIW { w: self }
        }
    }
}
# [ doc = "GPIO Interrupt Enable for Rising edge for Port 0." ]
pub struct ENR2 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Enable for Rising edge for Port 0." ]
pub mod enr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ENR2 {
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
    pub struct P2_0ERR {
        bits: bool,
    }
    impl P2_0ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_1ERR {
        bits: bool,
    }
    impl P2_1ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_2ERR {
        bits: bool,
    }
    impl P2_2ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_3ERR {
        bits: bool,
    }
    impl P2_3ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_4ERR {
        bits: bool,
    }
    impl P2_4ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_5ERR {
        bits: bool,
    }
    impl P2_5ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_6ERR {
        bits: bool,
    }
    impl P2_6ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_7ERR {
        bits: bool,
    }
    impl P2_7ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_8ERR {
        bits: bool,
    }
    impl P2_8ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_9ERR {
        bits: bool,
    }
    impl P2_9ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_10ERR {
        bits: bool,
    }
    impl P2_10ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_11ERR {
        bits: bool,
    }
    impl P2_11ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_12ERR {
        bits: bool,
    }
    impl P2_12ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_13ERR {
        bits: bool,
    }
    impl P2_13ERR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _P2_0ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_0ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_1ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_1ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_2ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_2ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_3ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_3ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_4ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_4ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_5ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_5ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_6ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_6ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_7ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_7ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_8ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_8ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_9ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_9ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_10ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_10ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_11ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_11ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_12ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_12ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_13ERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_13ERW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Enable rising edge interrupt for P2[0]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_0er(&self) -> P2_0ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_0ERR { bits }
        }
        # [ doc = "Bit 1 - Enable rising edge interrupt for P2[1]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_1er(&self) -> P2_1ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_1ERR { bits }
        }
        # [ doc = "Bit 2 - Enable rising edge interrupt for P2[2]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_2er(&self) -> P2_2ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_2ERR { bits }
        }
        # [ doc = "Bit 3 - Enable rising edge interrupt for P2[3]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_3er(&self) -> P2_3ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_3ERR { bits }
        }
        # [ doc = "Bit 4 - Enable rising edge interrupt for P2[4]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_4er(&self) -> P2_4ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_4ERR { bits }
        }
        # [ doc = "Bit 5 - Enable rising edge interrupt for P2[5]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_5er(&self) -> P2_5ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_5ERR { bits }
        }
        # [ doc = "Bit 6 - Enable rising edge interrupt for P2[6]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_6er(&self) -> P2_6ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_6ERR { bits }
        }
        # [ doc = "Bit 7 - Enable rising edge interrupt for P2[7]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_7er(&self) -> P2_7ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_7ERR { bits }
        }
        # [ doc = "Bit 8 - Enable rising edge interrupt for P2[8]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_8er(&self) -> P2_8ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_8ERR { bits }
        }
        # [ doc = "Bit 9 - Enable rising edge interrupt for P2[9]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_9er(&self) -> P2_9ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_9ERR { bits }
        }
        # [ doc = "Bit 10 - Enable rising edge interrupt for P2[10]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_10er(&self) -> P2_10ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_10ERR { bits }
        }
        # [ doc = "Bit 11 - Enable rising edge interrupt for P2[11]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_11er(&self) -> P2_11ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_11ERR { bits }
        }
        # [ doc = "Bit 12 - Enable rising edge interrupt for P2[12]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_12er(&self) -> P2_12ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_12ERR { bits }
        }
        # [ doc = "Bit 13 - Enable rising edge interrupt for P2[13]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_13er(&self) -> P2_13ERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_13ERR { bits }
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
        # [ doc = "Bit 0 - Enable rising edge interrupt for P2[0]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_0er(&mut self) -> _P2_0ERW {
            _P2_0ERW { w: self }
        }
        # [ doc = "Bit 1 - Enable rising edge interrupt for P2[1]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_1er(&mut self) -> _P2_1ERW {
            _P2_1ERW { w: self }
        }
        # [ doc = "Bit 2 - Enable rising edge interrupt for P2[2]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_2er(&mut self) -> _P2_2ERW {
            _P2_2ERW { w: self }
        }
        # [ doc = "Bit 3 - Enable rising edge interrupt for P2[3]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_3er(&mut self) -> _P2_3ERW {
            _P2_3ERW { w: self }
        }
        # [ doc = "Bit 4 - Enable rising edge interrupt for P2[4]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_4er(&mut self) -> _P2_4ERW {
            _P2_4ERW { w: self }
        }
        # [ doc = "Bit 5 - Enable rising edge interrupt for P2[5]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_5er(&mut self) -> _P2_5ERW {
            _P2_5ERW { w: self }
        }
        # [ doc = "Bit 6 - Enable rising edge interrupt for P2[6]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_6er(&mut self) -> _P2_6ERW {
            _P2_6ERW { w: self }
        }
        # [ doc = "Bit 7 - Enable rising edge interrupt for P2[7]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_7er(&mut self) -> _P2_7ERW {
            _P2_7ERW { w: self }
        }
        # [ doc = "Bit 8 - Enable rising edge interrupt for P2[8]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_8er(&mut self) -> _P2_8ERW {
            _P2_8ERW { w: self }
        }
        # [ doc = "Bit 9 - Enable rising edge interrupt for P2[9]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_9er(&mut self) -> _P2_9ERW {
            _P2_9ERW { w: self }
        }
        # [ doc = "Bit 10 - Enable rising edge interrupt for P2[10]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_10er(&mut self) -> _P2_10ERW {
            _P2_10ERW { w: self }
        }
        # [ doc = "Bit 11 - Enable rising edge interrupt for P2[11]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_11er(&mut self) -> _P2_11ERW {
            _P2_11ERW { w: self }
        }
        # [ doc = "Bit 12 - Enable rising edge interrupt for P2[12]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_12er(&mut self) -> _P2_12ERW {
            _P2_12ERW { w: self }
        }
        # [ doc = "Bit 13 - Enable rising edge interrupt for P2[13]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_13er(&mut self) -> _P2_13ERW {
            _P2_13ERW { w: self }
        }
    }
}
# [ doc = "GPIO Interrupt Enable for Falling edge for Port 0." ]
pub struct ENF2 {
    register: VolatileCell<u32>,
}
# [ doc = "GPIO Interrupt Enable for Falling edge for Port 0." ]
pub mod enf2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ENF2 {
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
    pub struct P2_0EFR {
        bits: bool,
    }
    impl P2_0EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_1EFR {
        bits: bool,
    }
    impl P2_1EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_2EFR {
        bits: bool,
    }
    impl P2_2EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_3EFR {
        bits: bool,
    }
    impl P2_3EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_4EFR {
        bits: bool,
    }
    impl P2_4EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_5EFR {
        bits: bool,
    }
    impl P2_5EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_6EFR {
        bits: bool,
    }
    impl P2_6EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_7EFR {
        bits: bool,
    }
    impl P2_7EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_8EFR {
        bits: bool,
    }
    impl P2_8EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_9EFR {
        bits: bool,
    }
    impl P2_9EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_10EFR {
        bits: bool,
    }
    impl P2_10EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_11EFR {
        bits: bool,
    }
    impl P2_11EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_12EFR {
        bits: bool,
    }
    impl P2_12EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct P2_13EFR {
        bits: bool,
    }
    impl P2_13EFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _P2_0EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_0EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_1EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_1EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_2EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_2EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_3EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_3EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_4EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_4EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_5EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_5EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_6EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_6EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_7EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_7EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_8EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_8EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_9EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_9EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_10EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_10EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_11EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_11EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_12EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_12EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _P2_13EFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _P2_13EFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Enable falling edge interrupt for P2[0]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_0ef(&self) -> P2_0EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_0EFR { bits }
        }
        # [ doc = "Bit 1 - Enable falling edge interrupt for P2[1]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_1ef(&self) -> P2_1EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_1EFR { bits }
        }
        # [ doc = "Bit 2 - Enable falling edge interrupt for P2[2]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_2ef(&self) -> P2_2EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_2EFR { bits }
        }
        # [ doc = "Bit 3 - Enable falling edge interrupt for P2[3]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_3ef(&self) -> P2_3EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_3EFR { bits }
        }
        # [ doc = "Bit 4 - Enable falling edge interrupt for P2[4]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_4ef(&self) -> P2_4EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_4EFR { bits }
        }
        # [ doc = "Bit 5 - Enable falling edge interrupt for P2[5]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_5ef(&self) -> P2_5EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_5EFR { bits }
        }
        # [ doc = "Bit 6 - Enable falling edge interrupt for P2[6]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_6ef(&self) -> P2_6EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_6EFR { bits }
        }
        # [ doc = "Bit 7 - Enable falling edge interrupt for P2[7]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_7ef(&self) -> P2_7EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_7EFR { bits }
        }
        # [ doc = "Bit 8 - Enable falling edge interrupt for P2[8]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_8ef(&self) -> P2_8EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_8EFR { bits }
        }
        # [ doc = "Bit 9 - Enable falling edge interrupt for P2[9]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_9ef(&self) -> P2_9EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_9EFR { bits }
        }
        # [ doc = "Bit 10 - Enable falling edge interrupt for P2[10]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_10ef(&self) -> P2_10EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_10EFR { bits }
        }
        # [ doc = "Bit 11 - Enable falling edge interrupt for P2[11]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_11ef(&self) -> P2_11EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_11EFR { bits }
        }
        # [ doc = "Bit 12 - Enable falling edge interrupt for P2[12]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_12ef(&self) -> P2_12EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_12EFR { bits }
        }
        # [ doc = "Bit 13 - Enable falling edge interrupt for P2[13]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_13ef(&self) -> P2_13EFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            P2_13EFR { bits }
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
        # [ doc = "Bit 0 - Enable falling edge interrupt for P2[0]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_0ef(&mut self) -> _P2_0EFW {
            _P2_0EFW { w: self }
        }
        # [ doc = "Bit 1 - Enable falling edge interrupt for P2[1]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_1ef(&mut self) -> _P2_1EFW {
            _P2_1EFW { w: self }
        }
        # [ doc = "Bit 2 - Enable falling edge interrupt for P2[2]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_2ef(&mut self) -> _P2_2EFW {
            _P2_2EFW { w: self }
        }
        # [ doc = "Bit 3 - Enable falling edge interrupt for P2[3]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_3ef(&mut self) -> _P2_3EFW {
            _P2_3EFW { w: self }
        }
        # [ doc = "Bit 4 - Enable falling edge interrupt for P2[4]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_4ef(&mut self) -> _P2_4EFW {
            _P2_4EFW { w: self }
        }
        # [ doc = "Bit 5 - Enable falling edge interrupt for P2[5]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_5ef(&mut self) -> _P2_5EFW {
            _P2_5EFW { w: self }
        }
        # [ doc = "Bit 6 - Enable falling edge interrupt for P2[6]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_6ef(&mut self) -> _P2_6EFW {
            _P2_6EFW { w: self }
        }
        # [ doc = "Bit 7 - Enable falling edge interrupt for P2[7]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_7ef(&mut self) -> _P2_7EFW {
            _P2_7EFW { w: self }
        }
        # [ doc = "Bit 8 - Enable falling edge interrupt for P2[8]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_8ef(&mut self) -> _P2_8EFW {
            _P2_8EFW { w: self }
        }
        # [ doc = "Bit 9 - Enable falling edge interrupt for P2[9]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_9ef(&mut self) -> _P2_9EFW {
            _P2_9EFW { w: self }
        }
        # [ doc = "Bit 10 - Enable falling edge interrupt for P2[10]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_10ef(&mut self) -> _P2_10EFW {
            _P2_10EFW { w: self }
        }
        # [ doc = "Bit 11 - Enable falling edge interrupt for P2[11]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_11ef(&mut self) -> _P2_11EFW {
            _P2_11EFW { w: self }
        }
        # [ doc = "Bit 12 - Enable falling edge interrupt for P2[12]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_12ef(&mut self) -> _P2_12EFW {
            _P2_12EFW { w: self }
        }
        # [ doc = "Bit 13 - Enable falling edge interrupt for P2[13]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt." ]
        # [ inline ( always ) ]
        pub fn p2_13ef(&mut self) -> _P2_13EFW {
            _P2_13EFW { w: self }
        }
    }
}
# [ doc = "GPIO" ]
pub struct GPIOINT {
    register_block: RegisterBlock,
}
impl Deref for GPIOINT {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
