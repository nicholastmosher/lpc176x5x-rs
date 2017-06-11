# ! [ doc = "UART0/2/3" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "UART0/2/3" ]
pub const UART0: Peripheral<UART0> = unsafe { Peripheral::new(1073790976) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Receiver Buffer Register. Contains the next received character to be read (DLAB =0)." ]
    pub rbr: RBR,
    # [ doc = "0x04 - Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1)." ]
    pub dlm: DLM,
    # [ doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending." ]
    pub iir: IIR,
    # [ doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation." ]
    pub lcr: LCR,
    _reserved0: [u8; 4usize],
    # [ doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors." ]
    pub lsr: LSR,
    _reserved1: [u8; 4usize],
    # [ doc = "0x1c - Scratch Pad Register. 8-bit temporary storage for software." ]
    pub scr: SCR,
    # [ doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature." ]
    pub acr: ACR,
    _reserved2: [u8; 4usize],
    # [ doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider." ]
    pub fdr: FDR,
    _reserved3: [u8; 4usize],
    # [ doc = "0x30 - Transmit Enable Register. Turns off UART transmitter for use with software flow control." ]
    pub ter: TER,
    _reserved4: [u8; 24usize],
    # [ doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes." ]
    pub rs485ctrl: RS485CTRL,
    # [ doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode." ]
    pub rs485adrmatch: RS485ADRMATCH,
    # [ doc = "0x54 - RS-485/EIA-485 direction control delay." ]
    pub rs485dly: RS485DLY,
}
# [ doc = "Receiver Buffer Register. Contains the next received character to be read (DLAB =0)." ]
pub struct RBR {
    register: VolatileCell<u32>,
}
# [ doc = "Receiver Buffer Register. Contains the next received character to be read (DLAB =0)." ]
pub mod rbr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::RBR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RBRR {
        bits: u8,
    }
    impl RBRR {
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
        # [ doc = "Bits 0:7 - The UARTn Receiver Buffer Register contains the oldest received byte in the UARTn Rx FIFO." ]
        # [ inline ( always ) ]
        pub fn rbr(&self) -> RBRR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            RBRR { bits }
        }
    }
}
# [ doc = "Transmit Holding Regiter. The next character to be transmitted is written here (DLAB =0)." ]
pub struct THR {
    register: VolatileCell<u32>,
}
# [ doc = "Transmit Holding Regiter. The next character to be transmitted is written here (DLAB =0)." ]
pub mod thr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::THR {
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
    pub struct _THRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _THRW<'a> {
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
        # [ doc = "Bits 0:7 - Writing to the UARTn Transmit Holding Register causes the data to be stored in the UARTn transmit FIFO. The byte will be sent when it reaches the bottom of the FIFO and the transmitter is available." ]
        # [ inline ( always ) ]
        pub fn thr(&mut self) -> _THRW {
            _THRW { w: self }
        }
    }
}
# [ doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1)." ]
pub struct DLL {
    register: VolatileCell<u32>,
}
# [ doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1)." ]
pub mod dll {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DLL {
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
    pub struct DLLSBR {
        bits: u8,
    }
    impl DLLSBR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DLLSBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DLLSBW<'a> {
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
        # [ doc = "Bits 0:7 - The UARTn Divisor Latch LSB Register, along with the UnDLM register, determines the baud rate of the UARTn." ]
        # [ inline ( always ) ]
        pub fn dllsb(&self) -> DLLSBR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DLLSBR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 1 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:7 - The UARTn Divisor Latch LSB Register, along with the UnDLM register, determines the baud rate of the UARTn." ]
        # [ inline ( always ) ]
        pub fn dllsb(&mut self) -> _DLLSBW {
            _DLLSBW { w: self }
        }
    }
}
# [ doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1)." ]
pub struct DLM {
    register: VolatileCell<u32>,
}
# [ doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1)." ]
pub mod dlm {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DLM {
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
    pub struct DLMSBR {
        bits: u8,
    }
    impl DLMSBR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DLMSBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DLMSBW<'a> {
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
        # [ doc = "Bits 0:7 - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn." ]
        # [ inline ( always ) ]
        pub fn dlmsb(&self) -> DLMSBR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DLMSBR { bits }
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
        # [ doc = "Bits 0:7 - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn." ]
        # [ inline ( always ) ]
        pub fn dlmsb(&mut self) -> _DLMSBW {
            _DLMSBW { w: self }
        }
    }
}
# [ doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART interrupts (DLAB =0)." ]
pub struct IER {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART interrupts (DLAB =0)." ]
pub mod ier {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::IER {
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
    # [ doc = "Possible values of the field `RBRIE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RBRIER {
        # [ doc = "Disable the RDA interrupts." ]
        DISABLE_THE_RDA_INTE,
        # [ doc = "Enable the RDA interrupts." ]
        ENABLE_THE_RDA_INTER,
    }
    impl RBRIER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RBRIER::DISABLE_THE_RDA_INTE => false,
                RBRIER::ENABLE_THE_RDA_INTER => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RBRIER {
            match value {
                false => RBRIER::DISABLE_THE_RDA_INTE,
                true => RBRIER::ENABLE_THE_RDA_INTER,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_THE_RDA_INTE`" ]
        # [ inline ( always ) ]
        pub fn is_disable_the_rda_inte(&self) -> bool {
            *self == RBRIER::DISABLE_THE_RDA_INTE
        }
        # [ doc = "Checks if the value of the field is `ENABLE_THE_RDA_INTER`" ]
        # [ inline ( always ) ]
        pub fn is_enable_the_rda_inter(&self) -> bool {
            *self == RBRIER::ENABLE_THE_RDA_INTER
        }
    }
    # [ doc = "Possible values of the field `THREIE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum THREIER {
        # [ doc = "Disable the THRE interrupts." ]
        DISABLE_THE_THRE_INT,
        # [ doc = "Enable the THRE interrupts." ]
        ENABLE_THE_THRE_INTE,
    }
    impl THREIER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                THREIER::DISABLE_THE_THRE_INT => false,
                THREIER::ENABLE_THE_THRE_INTE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> THREIER {
            match value {
                false => THREIER::DISABLE_THE_THRE_INT,
                true => THREIER::ENABLE_THE_THRE_INTE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_THE_THRE_INT`" ]
        # [ inline ( always ) ]
        pub fn is_disable_the_thre_int(&self) -> bool {
            *self == THREIER::DISABLE_THE_THRE_INT
        }
        # [ doc = "Checks if the value of the field is `ENABLE_THE_THRE_INTE`" ]
        # [ inline ( always ) ]
        pub fn is_enable_the_thre_inte(&self) -> bool {
            *self == THREIER::ENABLE_THE_THRE_INTE
        }
    }
    # [ doc = "Possible values of the field `RXIE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RXIER {
        # [ doc = "Disable the RX line status interrupts." ]
        DISABLE_THE_RX_LINE_,
        # [ doc = "Enable the RX line status interrupts." ]
        ENABLE_THE_RX_LINE_S,
    }
    impl RXIER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RXIER::DISABLE_THE_RX_LINE_ => false,
                RXIER::ENABLE_THE_RX_LINE_S => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RXIER {
            match value {
                false => RXIER::DISABLE_THE_RX_LINE_,
                true => RXIER::ENABLE_THE_RX_LINE_S,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_THE_RX_LINE_`" ]
        # [ inline ( always ) ]
        pub fn is_disable_the_rx_line_(&self) -> bool {
            *self == RXIER::DISABLE_THE_RX_LINE_
        }
        # [ doc = "Checks if the value of the field is `ENABLE_THE_RX_LINE_S`" ]
        # [ inline ( always ) ]
        pub fn is_enable_the_rx_line_s(&self) -> bool {
            *self == RXIER::ENABLE_THE_RX_LINE_S
        }
    }
    # [ doc = "Possible values of the field `ABEOINTEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ABEOINTENR {
        # [ doc = "Disable end of auto-baud Interrupt." ]
        DISABLE_END_OF_AUTO_,
        # [ doc = "Enable end of auto-baud Interrupt." ]
        ENABLE_END_OF_AUTO_B,
    }
    impl ABEOINTENR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ABEOINTENR::DISABLE_END_OF_AUTO_ => false,
                ABEOINTENR::ENABLE_END_OF_AUTO_B => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABEOINTENR {
            match value {
                false => ABEOINTENR::DISABLE_END_OF_AUTO_,
                true => ABEOINTENR::ENABLE_END_OF_AUTO_B,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_END_OF_AUTO_`" ]
        # [ inline ( always ) ]
        pub fn is_disable_end_of_auto_(&self) -> bool {
            *self == ABEOINTENR::DISABLE_END_OF_AUTO_
        }
        # [ doc = "Checks if the value of the field is `ENABLE_END_OF_AUTO_B`" ]
        # [ inline ( always ) ]
        pub fn is_enable_end_of_auto_b(&self) -> bool {
            *self == ABEOINTENR::ENABLE_END_OF_AUTO_B
        }
    }
    # [ doc = "Possible values of the field `ABTOINTEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ABTOINTENR {
        # [ doc = "Disable auto-baud time-out Interrupt." ]
        DISABLE_AUTO_BAUD_TI,
        # [ doc = "Enable auto-baud time-out Interrupt." ]
        ENABLE_AUTO_BAUD_TIM,
    }
    impl ABTOINTENR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ABTOINTENR::DISABLE_AUTO_BAUD_TI => false,
                ABTOINTENR::ENABLE_AUTO_BAUD_TIM => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABTOINTENR {
            match value {
                false => ABTOINTENR::DISABLE_AUTO_BAUD_TI,
                true => ABTOINTENR::ENABLE_AUTO_BAUD_TIM,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_AUTO_BAUD_TI`" ]
        # [ inline ( always ) ]
        pub fn is_disable_auto_baud_ti(&self) -> bool {
            *self == ABTOINTENR::DISABLE_AUTO_BAUD_TI
        }
        # [ doc = "Checks if the value of the field is `ENABLE_AUTO_BAUD_TIM`" ]
        # [ inline ( always ) ]
        pub fn is_enable_auto_baud_tim(&self) -> bool {
            *self == ABTOINTENR::ENABLE_AUTO_BAUD_TIM
        }
    }
    # [ doc = "Values that can be written to the field `RBRIE`" ]
    pub enum RBRIEW {
        # [ doc = "Disable the RDA interrupts." ]
        DISABLE_THE_RDA_INTE,
        # [ doc = "Enable the RDA interrupts." ]
        ENABLE_THE_RDA_INTER,
    }
    impl RBRIEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                RBRIEW::DISABLE_THE_RDA_INTE => false,
                RBRIEW::ENABLE_THE_RDA_INTER => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RBRIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RBRIEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RBRIEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable the RDA interrupts." ]
        # [ inline ( always ) ]
        pub fn disable_the_rda_inte(self) -> &'a mut W {
            self.variant(RBRIEW::DISABLE_THE_RDA_INTE)
        }
        # [ doc = "Enable the RDA interrupts." ]
        # [ inline ( always ) ]
        pub fn enable_the_rda_inter(self) -> &'a mut W {
            self.variant(RBRIEW::ENABLE_THE_RDA_INTER)
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
    # [ doc = "Values that can be written to the field `THREIE`" ]
    pub enum THREIEW {
        # [ doc = "Disable the THRE interrupts." ]
        DISABLE_THE_THRE_INT,
        # [ doc = "Enable the THRE interrupts." ]
        ENABLE_THE_THRE_INTE,
    }
    impl THREIEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                THREIEW::DISABLE_THE_THRE_INT => false,
                THREIEW::ENABLE_THE_THRE_INTE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _THREIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _THREIEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: THREIEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable the THRE interrupts." ]
        # [ inline ( always ) ]
        pub fn disable_the_thre_int(self) -> &'a mut W {
            self.variant(THREIEW::DISABLE_THE_THRE_INT)
        }
        # [ doc = "Enable the THRE interrupts." ]
        # [ inline ( always ) ]
        pub fn enable_the_thre_inte(self) -> &'a mut W {
            self.variant(THREIEW::ENABLE_THE_THRE_INTE)
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
    # [ doc = "Values that can be written to the field `RXIE`" ]
    pub enum RXIEW {
        # [ doc = "Disable the RX line status interrupts." ]
        DISABLE_THE_RX_LINE_,
        # [ doc = "Enable the RX line status interrupts." ]
        ENABLE_THE_RX_LINE_S,
    }
    impl RXIEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                RXIEW::DISABLE_THE_RX_LINE_ => false,
                RXIEW::ENABLE_THE_RX_LINE_S => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RXIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXIEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RXIEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable the RX line status interrupts." ]
        # [ inline ( always ) ]
        pub fn disable_the_rx_line_(self) -> &'a mut W {
            self.variant(RXIEW::DISABLE_THE_RX_LINE_)
        }
        # [ doc = "Enable the RX line status interrupts." ]
        # [ inline ( always ) ]
        pub fn enable_the_rx_line_s(self) -> &'a mut W {
            self.variant(RXIEW::ENABLE_THE_RX_LINE_S)
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
    # [ doc = "Values that can be written to the field `ABEOINTEN`" ]
    pub enum ABEOINTENW {
        # [ doc = "Disable end of auto-baud Interrupt." ]
        DISABLE_END_OF_AUTO_,
        # [ doc = "Enable end of auto-baud Interrupt." ]
        ENABLE_END_OF_AUTO_B,
    }
    impl ABEOINTENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ABEOINTENW::DISABLE_END_OF_AUTO_ => false,
                ABEOINTENW::ENABLE_END_OF_AUTO_B => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ABEOINTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABEOINTENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ABEOINTENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable end of auto-baud Interrupt." ]
        # [ inline ( always ) ]
        pub fn disable_end_of_auto_(self) -> &'a mut W {
            self.variant(ABEOINTENW::DISABLE_END_OF_AUTO_)
        }
        # [ doc = "Enable end of auto-baud Interrupt." ]
        # [ inline ( always ) ]
        pub fn enable_end_of_auto_b(self) -> &'a mut W {
            self.variant(ABEOINTENW::ENABLE_END_OF_AUTO_B)
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
    # [ doc = "Values that can be written to the field `ABTOINTEN`" ]
    pub enum ABTOINTENW {
        # [ doc = "Disable auto-baud time-out Interrupt." ]
        DISABLE_AUTO_BAUD_TI,
        # [ doc = "Enable auto-baud time-out Interrupt." ]
        ENABLE_AUTO_BAUD_TIM,
    }
    impl ABTOINTENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ABTOINTENW::DISABLE_AUTO_BAUD_TI => false,
                ABTOINTENW::ENABLE_AUTO_BAUD_TIM => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ABTOINTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABTOINTENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ABTOINTENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable auto-baud time-out Interrupt." ]
        # [ inline ( always ) ]
        pub fn disable_auto_baud_ti(self) -> &'a mut W {
            self.variant(ABTOINTENW::DISABLE_AUTO_BAUD_TI)
        }
        # [ doc = "Enable auto-baud time-out Interrupt." ]
        # [ inline ( always ) ]
        pub fn enable_auto_baud_tim(self) -> &'a mut W {
            self.variant(ABTOINTENW::ENABLE_AUTO_BAUD_TIM)
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
        # [ doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UARTn. It also controls the Character Receive Time-out interrupt." ]
        # [ inline ( always ) ]
        pub fn rbrie(&self) -> RBRIER {
            RBRIER::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UARTn. The status of this can be read from UnLSR[5]." ]
        # [ inline ( always ) ]
        pub fn threie(&self) -> THREIER {
            THREIER::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 1;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 2 - RX Line Status Interrupt Enable. Enables the UARTn RX line status interrupts. The status of this interrupt can be read from UnLSR[4:1]." ]
        # [ inline ( always ) ]
        pub fn rxie(&self) -> RXIER {
            RXIER::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 2;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 8 - Enables the end of auto-baud interrupt." ]
        # [ inline ( always ) ]
        pub fn abeointen(&self) -> ABEOINTENR {
            ABEOINTENR::_from({
                                  const MASK: bool = true;
                                  const OFFSET: u8 = 8;
                                  ((self.bits >> OFFSET) & MASK as u32) != 0
                              })
        }
        # [ doc = "Bit 9 - Enables the auto-baud time-out interrupt." ]
        # [ inline ( always ) ]
        pub fn abtointen(&self) -> ABTOINTENR {
            ABTOINTENR::_from({
                                  const MASK: bool = true;
                                  const OFFSET: u8 = 9;
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
        # [ doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UARTn. It also controls the Character Receive Time-out interrupt." ]
        # [ inline ( always ) ]
        pub fn rbrie(&mut self) -> _RBRIEW {
            _RBRIEW { w: self }
        }
        # [ doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UARTn. The status of this can be read from UnLSR[5]." ]
        # [ inline ( always ) ]
        pub fn threie(&mut self) -> _THREIEW {
            _THREIEW { w: self }
        }
        # [ doc = "Bit 2 - RX Line Status Interrupt Enable. Enables the UARTn RX line status interrupts. The status of this interrupt can be read from UnLSR[4:1]." ]
        # [ inline ( always ) ]
        pub fn rxie(&mut self) -> _RXIEW {
            _RXIEW { w: self }
        }
        # [ doc = "Bit 8 - Enables the end of auto-baud interrupt." ]
        # [ inline ( always ) ]
        pub fn abeointen(&mut self) -> _ABEOINTENW {
            _ABEOINTENW { w: self }
        }
        # [ doc = "Bit 9 - Enables the auto-baud time-out interrupt." ]
        # [ inline ( always ) ]
        pub fn abtointen(&mut self) -> _ABTOINTENW {
            _ABTOINTENW { w: self }
        }
    }
}
# [ doc = "Interrupt ID Register. Identifies which interrupt(s) are pending." ]
pub struct IIR {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt ID Register. Identifies which interrupt(s) are pending." ]
pub mod iir {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::IIR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = "Possible values of the field `INTSTATUS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum INTSTATUSR {
        # [ doc = "At least one interrupt is pending." ]
        AT_LEAST_ONE_INTERRU,
        # [ doc = "No interrupt is pending." ]
        NO_INTERRUPT_IS_PEND,
    }
    impl INTSTATUSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                INTSTATUSR::AT_LEAST_ONE_INTERRU => false,
                INTSTATUSR::NO_INTERRUPT_IS_PEND => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> INTSTATUSR {
            match value {
                false => INTSTATUSR::AT_LEAST_ONE_INTERRU,
                true => INTSTATUSR::NO_INTERRUPT_IS_PEND,
            }
        }
        # [ doc = "Checks if the value of the field is `AT_LEAST_ONE_INTERRU`" ]
        # [ inline ( always ) ]
        pub fn is_at_least_one_interru(&self) -> bool {
            *self == INTSTATUSR::AT_LEAST_ONE_INTERRU
        }
        # [ doc = "Checks if the value of the field is `NO_INTERRUPT_IS_PEND`" ]
        # [ inline ( always ) ]
        pub fn is_no_interrupt_is_pend(&self) -> bool {
            *self == INTSTATUSR::NO_INTERRUPT_IS_PEND
        }
    }
    # [ doc = "Possible values of the field `INTID`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum INTIDR {
        # [ doc = "1   - Receive Line Status (RLS)." ]
        _1_RECEIVE_LINE_S,
        # [ doc = "2a - Receive Data Available (RDA)." ]
        _2A_RECEIVE_DATA_AV,
        # [ doc = "2b - Character Time-out Indicator (CTI)." ]
        _2B_CHARACTER_TIME_,
        # [ doc = "3   - THRE Interrupt" ]
        _3_THRE_INTERRUPT,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl INTIDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                INTIDR::_1_RECEIVE_LINE_S => 3,
                INTIDR::_2A_RECEIVE_DATA_AV => 2,
                INTIDR::_2B_CHARACTER_TIME_ => 6,
                INTIDR::_3_THRE_INTERRUPT => 1,
                INTIDR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> INTIDR {
            match value {
                3 => INTIDR::_1_RECEIVE_LINE_S,
                2 => INTIDR::_2A_RECEIVE_DATA_AV,
                6 => INTIDR::_2B_CHARACTER_TIME_,
                1 => INTIDR::_3_THRE_INTERRUPT,
                i => INTIDR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `_1_RECEIVE_LINE_S`" ]
        # [ inline ( always ) ]
        pub fn is_1_receive_line_s(&self) -> bool {
            *self == INTIDR::_1_RECEIVE_LINE_S
        }
        # [ doc = "Checks if the value of the field is `_2A_RECEIVE_DATA_AV`" ]
        # [ inline ( always ) ]
        pub fn is_2a_receive_data_av(&self) -> bool {
            *self == INTIDR::_2A_RECEIVE_DATA_AV
        }
        # [ doc = "Checks if the value of the field is `_2B_CHARACTER_TIME_`" ]
        # [ inline ( always ) ]
        pub fn is_2b_character_time_(&self) -> bool {
            *self == INTIDR::_2B_CHARACTER_TIME_
        }
        # [ doc = "Checks if the value of the field is `_3_THRE_INTERRUPT`" ]
        # [ inline ( always ) ]
        pub fn is_3_thre_interrupt(&self) -> bool {
            *self == INTIDR::_3_THRE_INTERRUPT
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FIFOENABLER {
        bits: u8,
    }
    impl FIFOENABLER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ABEOINTR {
        bits: bool,
    }
    impl ABEOINTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ABTOINTR {
        bits: bool,
    }
    impl ABTOINTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - Interrupt status. Note that UnIIR[0] is active low. The pending interrupt can be determined by evaluating UnIIR[3:1]." ]
        # [ inline ( always ) ]
        pub fn intstatus(&self) -> INTSTATUSR {
            INTSTATUSR::_from({
                                  const MASK: bool = true;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) != 0
                              })
        }
        # [ doc = "Bits 1:3 - Interrupt identification. UnIER[3:1] identifies an interrupt corresponding to the UARTn Rx or TX FIFO. All other combinations of UnIER[3:1] not listed below are reserved (000,100,101,111)." ]
        # [ inline ( always ) ]
        pub fn intid(&self) -> INTIDR {
            INTIDR::_from({
                              const MASK: u8 = 7;
                              const OFFSET: u8 = 1;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 6:7 - Copies of UnFCR[0]." ]
        # [ inline ( always ) ]
        pub fn fifoenable(&self) -> FIFOENABLER {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            FIFOENABLER { bits }
        }
        # [ doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled." ]
        # [ inline ( always ) ]
        pub fn abeoint(&self) -> ABEOINTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ABEOINTR { bits }
        }
        # [ doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled." ]
        # [ inline ( always ) ]
        pub fn abtoint(&self) -> ABTOINTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ABTOINTR { bits }
        }
    }
}
# [ doc = "FIFO Control Register. Controls UART FIFO usage and modes." ]
pub struct FCR {
    register: VolatileCell<u32>,
}
# [ doc = "FIFO Control Register. Controls UART FIFO usage and modes." ]
pub mod fcr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::FCR {
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
    # [ doc = "Values that can be written to the field `FIFOEN`" ]
    pub enum FIFOENW {
        # [ doc = "UARTn FIFOs are disabled. Must not be used in the application." ]
        UARTN_FIFOS_ARE_DISA,
        # [ doc = "Active high enable for both UARTn Rx and TX FIFOs and UnFCR[7:1] access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the related UART FIFOs." ]
        ACTIVE_HIGH_ENABLE_F,
    }
    impl FIFOENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                FIFOENW::UARTN_FIFOS_ARE_DISA => false,
                FIFOENW::ACTIVE_HIGH_ENABLE_F => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FIFOENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FIFOENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: FIFOENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "UARTn FIFOs are disabled. Must not be used in the application." ]
        # [ inline ( always ) ]
        pub fn uartn_fifos_are_disa(self) -> &'a mut W {
            self.variant(FIFOENW::UARTN_FIFOS_ARE_DISA)
        }
        # [ doc = "Active high enable for both UARTn Rx and TX FIFOs and UnFCR[7:1] access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the related UART FIFOs." ]
        # [ inline ( always ) ]
        pub fn active_high_enable_f(self) -> &'a mut W {
            self.variant(FIFOENW::ACTIVE_HIGH_ENABLE_F)
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
    # [ doc = "Values that can be written to the field `RXFIFORES`" ]
    pub enum RXFIFORESW {
        # [ doc = "No impact on either of UARTn FIFOs." ]
        NO_IMPACT_ON_EITHER_,
        # [ doc = "Writing a logic 1 to UnFCR[1] will clear all bytes in UARTn Rx FIFO, reset the pointer logic. This bit is self-clearing." ]
        WRITING_A_LOGIC_1_TO,
    }
    impl RXFIFORESW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                RXFIFORESW::NO_IMPACT_ON_EITHER_ => false,
                RXFIFORESW::WRITING_A_LOGIC_1_TO => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RXFIFORESW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXFIFORESW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RXFIFORESW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "No impact on either of UARTn FIFOs." ]
        # [ inline ( always ) ]
        pub fn no_impact_on_either_(self) -> &'a mut W {
            self.variant(RXFIFORESW::NO_IMPACT_ON_EITHER_)
        }
        # [ doc = "Writing a logic 1 to UnFCR[1] will clear all bytes in UARTn Rx FIFO, reset the pointer logic. This bit is self-clearing." ]
        # [ inline ( always ) ]
        pub fn writing_a_logic_1_to(self) -> &'a mut W {
            self.variant(RXFIFORESW::WRITING_A_LOGIC_1_TO)
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
    # [ doc = "Values that can be written to the field `TXFIFORES`" ]
    pub enum TXFIFORESW {
        # [ doc = "No impact on either of UARTn FIFOs." ]
        NO_IMPACT_ON_EITHER_,
        # [ doc = "Writing a logic 1 to UnFCR[2] will clear all bytes in UARTn TX FIFO, reset the pointer logic. This bit is self-clearing." ]
        WRITING_A_LOGIC_1_TO,
    }
    impl TXFIFORESW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                TXFIFORESW::NO_IMPACT_ON_EITHER_ => false,
                TXFIFORESW::WRITING_A_LOGIC_1_TO => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TXFIFORESW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXFIFORESW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TXFIFORESW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "No impact on either of UARTn FIFOs." ]
        # [ inline ( always ) ]
        pub fn no_impact_on_either_(self) -> &'a mut W {
            self.variant(TXFIFORESW::NO_IMPACT_ON_EITHER_)
        }
        # [ doc = "Writing a logic 1 to UnFCR[2] will clear all bytes in UARTn TX FIFO, reset the pointer logic. This bit is self-clearing." ]
        # [ inline ( always ) ]
        pub fn writing_a_logic_1_to(self) -> &'a mut W {
            self.variant(TXFIFORESW::WRITING_A_LOGIC_1_TO)
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
    pub struct _DMAMODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMAMODEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    # [ doc = "Values that can be written to the field `RXTRIGLVL`" ]
    pub enum RXTRIGLVLW {
        # [ doc = "Trigger level 0 (1 character or 0x01)." ]
        TRIGGER_LEVEL_0_1_C,
        # [ doc = "Trigger level 1 (4 characters or 0x04)." ]
        TRIGGER_LEVEL_1_4_C,
        # [ doc = "Trigger level 2 (8 characters or 0x08)." ]
        TRIGGER_LEVEL_2_8_C,
        # [ doc = "Trigger level 3 (14 characters or 0x0E)." ]
        TRIGGER_LEVEL_3_14_,
    }
    impl RXTRIGLVLW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                RXTRIGLVLW::TRIGGER_LEVEL_0_1_C => 0,
                RXTRIGLVLW::TRIGGER_LEVEL_1_4_C => 1,
                RXTRIGLVLW::TRIGGER_LEVEL_2_8_C => 2,
                RXTRIGLVLW::TRIGGER_LEVEL_3_14_ => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RXTRIGLVLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXTRIGLVLW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RXTRIGLVLW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Trigger level 0 (1 character or 0x01)." ]
        # [ inline ( always ) ]
        pub fn trigger_level_0_1_c(self) -> &'a mut W {
            self.variant(RXTRIGLVLW::TRIGGER_LEVEL_0_1_C)
        }
        # [ doc = "Trigger level 1 (4 characters or 0x04)." ]
        # [ inline ( always ) ]
        pub fn trigger_level_1_4_c(self) -> &'a mut W {
            self.variant(RXTRIGLVLW::TRIGGER_LEVEL_1_4_C)
        }
        # [ doc = "Trigger level 2 (8 characters or 0x08)." ]
        # [ inline ( always ) ]
        pub fn trigger_level_2_8_c(self) -> &'a mut W {
            self.variant(RXTRIGLVLW::TRIGGER_LEVEL_2_8_C)
        }
        # [ doc = "Trigger level 3 (14 characters or 0x0E)." ]
        # [ inline ( always ) ]
        pub fn trigger_level_3_14_(self) -> &'a mut W {
            self.variant(RXTRIGLVLW::TRIGGER_LEVEL_3_14_)
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
        # [ doc = "Bit 0 - FIFO Enable." ]
        # [ inline ( always ) ]
        pub fn fifoen(&mut self) -> _FIFOENW {
            _FIFOENW { w: self }
        }
        # [ doc = "Bit 1 - RX FIFO Reset." ]
        # [ inline ( always ) ]
        pub fn rxfifores(&mut self) -> _RXFIFORESW {
            _RXFIFORESW { w: self }
        }
        # [ doc = "Bit 2 - TX FIFO Reset." ]
        # [ inline ( always ) ]
        pub fn txfifores(&mut self) -> _TXFIFORESW {
            _TXFIFORESW { w: self }
        }
        # [ doc = "Bit 3 - DMA Mode Select. When the FIFO enable (bit 0 of this register) is set, this bit selects the DMA mode. See Section 18.6.6.1." ]
        # [ inline ( always ) ]
        pub fn dmamode(&mut self) -> _DMAMODEW {
            _DMAMODEW { w: self }
        }
        # [ doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UARTn FIFO characters must be written before an interrupt or DMA request is activated." ]
        # [ inline ( always ) ]
        pub fn rxtriglvl(&mut self) -> _RXTRIGLVLW {
            _RXTRIGLVLW { w: self }
        }
    }
}
# [ doc = "Line Control Register. Contains controls for frame formatting and break generation." ]
pub struct LCR {
    register: VolatileCell<u32>,
}
# [ doc = "Line Control Register. Contains controls for frame formatting and break generation." ]
pub mod lcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::LCR {
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
    # [ doc = "Possible values of the field `WLS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum WLSR {
        # [ doc = "5-bit character length" ]
        _5_BIT_CHARACTER_LENG,
        # [ doc = "6-bit character length" ]
        _6_BIT_CHARACTER_LENG,
        # [ doc = "7-bit character length" ]
        _7_BIT_CHARACTER_LENG,
        # [ doc = "8-bit character length" ]
        _8_BIT_CHARACTER_LENG,
    }
    impl WLSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                WLSR::_5_BIT_CHARACTER_LENG => 0,
                WLSR::_6_BIT_CHARACTER_LENG => 1,
                WLSR::_7_BIT_CHARACTER_LENG => 2,
                WLSR::_8_BIT_CHARACTER_LENG => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> WLSR {
            match value {
                0 => WLSR::_5_BIT_CHARACTER_LENG,
                1 => WLSR::_6_BIT_CHARACTER_LENG,
                2 => WLSR::_7_BIT_CHARACTER_LENG,
                3 => WLSR::_8_BIT_CHARACTER_LENG,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `_5_BIT_CHARACTER_LENG`" ]
        # [ inline ( always ) ]
        pub fn is_5_bit_character_leng(&self) -> bool {
            *self == WLSR::_5_BIT_CHARACTER_LENG
        }
        # [ doc = "Checks if the value of the field is `_6_BIT_CHARACTER_LENG`" ]
        # [ inline ( always ) ]
        pub fn is_6_bit_character_leng(&self) -> bool {
            *self == WLSR::_6_BIT_CHARACTER_LENG
        }
        # [ doc = "Checks if the value of the field is `_7_BIT_CHARACTER_LENG`" ]
        # [ inline ( always ) ]
        pub fn is_7_bit_character_leng(&self) -> bool {
            *self == WLSR::_7_BIT_CHARACTER_LENG
        }
        # [ doc = "Checks if the value of the field is `_8_BIT_CHARACTER_LENG`" ]
        # [ inline ( always ) ]
        pub fn is_8_bit_character_leng(&self) -> bool {
            *self == WLSR::_8_BIT_CHARACTER_LENG
        }
    }
    # [ doc = "Possible values of the field `SBS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SBSR {
        # [ doc = "1 stop bit." ]
        _1_STOP_BIT_,
        # [ doc = "2 stop bits (1.5 if UnLCR[1:0]=00)." ]
        _2_STOP_BITS_1_5_IF_,
    }
    impl SBSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                SBSR::_1_STOP_BIT_ => false,
                SBSR::_2_STOP_BITS_1_5_IF_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SBSR {
            match value {
                false => SBSR::_1_STOP_BIT_,
                true => SBSR::_2_STOP_BITS_1_5_IF_,
            }
        }
        # [ doc = "Checks if the value of the field is `_1_STOP_BIT_`" ]
        # [ inline ( always ) ]
        pub fn is_1_stop_bit_(&self) -> bool {
            *self == SBSR::_1_STOP_BIT_
        }
        # [ doc = "Checks if the value of the field is `_2_STOP_BITS_1_5_IF_`" ]
        # [ inline ( always ) ]
        pub fn is_2_stop_bits_1_5_if_(&self) -> bool {
            *self == SBSR::_2_STOP_BITS_1_5_IF_
        }
    }
    # [ doc = "Possible values of the field `PE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PER {
        # [ doc = "Disable parity generation and checking." ]
        DISABLE_PARITY_GENER,
        # [ doc = "Enable parity generation and checking." ]
        ENABLE_PARITY_GENERA,
    }
    impl PER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                PER::DISABLE_PARITY_GENER => false,
                PER::ENABLE_PARITY_GENERA => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PER {
            match value {
                false => PER::DISABLE_PARITY_GENER,
                true => PER::ENABLE_PARITY_GENERA,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_PARITY_GENER`" ]
        # [ inline ( always ) ]
        pub fn is_disable_parity_gener(&self) -> bool {
            *self == PER::DISABLE_PARITY_GENER
        }
        # [ doc = "Checks if the value of the field is `ENABLE_PARITY_GENERA`" ]
        # [ inline ( always ) ]
        pub fn is_enable_parity_genera(&self) -> bool {
            *self == PER::ENABLE_PARITY_GENERA
        }
    }
    # [ doc = "Possible values of the field `PS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PSR {
        # [ doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd." ]
        ODD_PARITY_NUMBER_O,
        # [ doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even." ]
        EVEN_PARITY_NUMBER_,
        # [ doc = "Forced 1 stick parity." ]
        FORCED_1_STICK_PARIT,
        # [ doc = "Forced 0 stick parity." ]
        FORCED_0_STICK_PARIT,
    }
    impl PSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PSR::ODD_PARITY_NUMBER_O => 0,
                PSR::EVEN_PARITY_NUMBER_ => 1,
                PSR::FORCED_1_STICK_PARIT => 2,
                PSR::FORCED_0_STICK_PARIT => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PSR {
            match value {
                0 => PSR::ODD_PARITY_NUMBER_O,
                1 => PSR::EVEN_PARITY_NUMBER_,
                2 => PSR::FORCED_1_STICK_PARIT,
                3 => PSR::FORCED_0_STICK_PARIT,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `ODD_PARITY_NUMBER_O`" ]
        # [ inline ( always ) ]
        pub fn is_odd_parity_number_o(&self) -> bool {
            *self == PSR::ODD_PARITY_NUMBER_O
        }
        # [ doc = "Checks if the value of the field is `EVEN_PARITY_NUMBER_`" ]
        # [ inline ( always ) ]
        pub fn is_even_parity_number_(&self) -> bool {
            *self == PSR::EVEN_PARITY_NUMBER_
        }
        # [ doc = "Checks if the value of the field is `FORCED_1_STICK_PARIT`" ]
        # [ inline ( always ) ]
        pub fn is_forced_1_stick_parit(&self) -> bool {
            *self == PSR::FORCED_1_STICK_PARIT
        }
        # [ doc = "Checks if the value of the field is `FORCED_0_STICK_PARIT`" ]
        # [ inline ( always ) ]
        pub fn is_forced_0_stick_parit(&self) -> bool {
            *self == PSR::FORCED_0_STICK_PARIT
        }
    }
    # [ doc = "Possible values of the field `BC`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum BCR {
        # [ doc = "Disable break transmission." ]
        DISABLE_BREAK_TRANSM,
        # [ doc = "Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR[6] is active high." ]
        ENABLE_BREAK_TRANSMI,
    }
    impl BCR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                BCR::DISABLE_BREAK_TRANSM => false,
                BCR::ENABLE_BREAK_TRANSMI => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> BCR {
            match value {
                false => BCR::DISABLE_BREAK_TRANSM,
                true => BCR::ENABLE_BREAK_TRANSMI,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_BREAK_TRANSM`" ]
        # [ inline ( always ) ]
        pub fn is_disable_break_transm(&self) -> bool {
            *self == BCR::DISABLE_BREAK_TRANSM
        }
        # [ doc = "Checks if the value of the field is `ENABLE_BREAK_TRANSMI`" ]
        # [ inline ( always ) ]
        pub fn is_enable_break_transmi(&self) -> bool {
            *self == BCR::ENABLE_BREAK_TRANSMI
        }
    }
    # [ doc = "Possible values of the field `DLAB`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DLABR {
        # [ doc = "Disable access to Divisor Latches." ]
        DISABLE_ACCESS_TO_DI,
        # [ doc = "Enable access to Divisor Latches." ]
        ENABLE_ACCESS_TO_DIV,
    }
    impl DLABR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DLABR::DISABLE_ACCESS_TO_DI => false,
                DLABR::ENABLE_ACCESS_TO_DIV => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DLABR {
            match value {
                false => DLABR::DISABLE_ACCESS_TO_DI,
                true => DLABR::ENABLE_ACCESS_TO_DIV,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_ACCESS_TO_DI`" ]
        # [ inline ( always ) ]
        pub fn is_disable_access_to_di(&self) -> bool {
            *self == DLABR::DISABLE_ACCESS_TO_DI
        }
        # [ doc = "Checks if the value of the field is `ENABLE_ACCESS_TO_DIV`" ]
        # [ inline ( always ) ]
        pub fn is_enable_access_to_div(&self) -> bool {
            *self == DLABR::ENABLE_ACCESS_TO_DIV
        }
    }
    # [ doc = "Values that can be written to the field `WLS`" ]
    pub enum WLSW {
        # [ doc = "5-bit character length" ]
        _5_BIT_CHARACTER_LENG,
        # [ doc = "6-bit character length" ]
        _6_BIT_CHARACTER_LENG,
        # [ doc = "7-bit character length" ]
        _7_BIT_CHARACTER_LENG,
        # [ doc = "8-bit character length" ]
        _8_BIT_CHARACTER_LENG,
    }
    impl WLSW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                WLSW::_5_BIT_CHARACTER_LENG => 0,
                WLSW::_6_BIT_CHARACTER_LENG => 1,
                WLSW::_7_BIT_CHARACTER_LENG => 2,
                WLSW::_8_BIT_CHARACTER_LENG => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WLSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WLSW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: WLSW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "5-bit character length" ]
        # [ inline ( always ) ]
        pub fn _5_bit_character_leng(self) -> &'a mut W {
            self.variant(WLSW::_5_BIT_CHARACTER_LENG)
        }
        # [ doc = "6-bit character length" ]
        # [ inline ( always ) ]
        pub fn _6_bit_character_leng(self) -> &'a mut W {
            self.variant(WLSW::_6_BIT_CHARACTER_LENG)
        }
        # [ doc = "7-bit character length" ]
        # [ inline ( always ) ]
        pub fn _7_bit_character_leng(self) -> &'a mut W {
            self.variant(WLSW::_7_BIT_CHARACTER_LENG)
        }
        # [ doc = "8-bit character length" ]
        # [ inline ( always ) ]
        pub fn _8_bit_character_leng(self) -> &'a mut W {
            self.variant(WLSW::_8_BIT_CHARACTER_LENG)
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
    # [ doc = "Values that can be written to the field `SBS`" ]
    pub enum SBSW {
        # [ doc = "1 stop bit." ]
        _1_STOP_BIT_,
        # [ doc = "2 stop bits (1.5 if UnLCR[1:0]=00)." ]
        _2_STOP_BITS_1_5_IF_,
    }
    impl SBSW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SBSW::_1_STOP_BIT_ => false,
                SBSW::_2_STOP_BITS_1_5_IF_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SBSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SBSW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SBSW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "1 stop bit." ]
        # [ inline ( always ) ]
        pub fn _1_stop_bit_(self) -> &'a mut W {
            self.variant(SBSW::_1_STOP_BIT_)
        }
        # [ doc = "2 stop bits (1.5 if UnLCR[1:0]=00)." ]
        # [ inline ( always ) ]
        pub fn _2_stop_bits_1_5_if_(self) -> &'a mut W {
            self.variant(SBSW::_2_STOP_BITS_1_5_IF_)
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
    # [ doc = "Values that can be written to the field `PE`" ]
    pub enum PEW {
        # [ doc = "Disable parity generation and checking." ]
        DISABLE_PARITY_GENER,
        # [ doc = "Enable parity generation and checking." ]
        ENABLE_PARITY_GENERA,
    }
    impl PEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PEW::DISABLE_PARITY_GENER => false,
                PEW::ENABLE_PARITY_GENERA => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable parity generation and checking." ]
        # [ inline ( always ) ]
        pub fn disable_parity_gener(self) -> &'a mut W {
            self.variant(PEW::DISABLE_PARITY_GENER)
        }
        # [ doc = "Enable parity generation and checking." ]
        # [ inline ( always ) ]
        pub fn enable_parity_genera(self) -> &'a mut W {
            self.variant(PEW::ENABLE_PARITY_GENERA)
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
    # [ doc = "Values that can be written to the field `PS`" ]
    pub enum PSW {
        # [ doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd." ]
        ODD_PARITY_NUMBER_O,
        # [ doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even." ]
        EVEN_PARITY_NUMBER_,
        # [ doc = "Forced 1 stick parity." ]
        FORCED_1_STICK_PARIT,
        # [ doc = "Forced 0 stick parity." ]
        FORCED_0_STICK_PARIT,
    }
    impl PSW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PSW::ODD_PARITY_NUMBER_O => 0,
                PSW::EVEN_PARITY_NUMBER_ => 1,
                PSW::FORCED_1_STICK_PARIT => 2,
                PSW::FORCED_0_STICK_PARIT => 3,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PSW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PSW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        # [ doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd." ]
        # [ inline ( always ) ]
        pub fn odd_parity_number_o(self) -> &'a mut W {
            self.variant(PSW::ODD_PARITY_NUMBER_O)
        }
        # [ doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even." ]
        # [ inline ( always ) ]
        pub fn even_parity_number_(self) -> &'a mut W {
            self.variant(PSW::EVEN_PARITY_NUMBER_)
        }
        # [ doc = "Forced 1 stick parity." ]
        # [ inline ( always ) ]
        pub fn forced_1_stick_parit(self) -> &'a mut W {
            self.variant(PSW::FORCED_1_STICK_PARIT)
        }
        # [ doc = "Forced 0 stick parity." ]
        # [ inline ( always ) ]
        pub fn forced_0_stick_parit(self) -> &'a mut W {
            self.variant(PSW::FORCED_0_STICK_PARIT)
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
    # [ doc = "Values that can be written to the field `BC`" ]
    pub enum BCW {
        # [ doc = "Disable break transmission." ]
        DISABLE_BREAK_TRANSM,
        # [ doc = "Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR[6] is active high." ]
        ENABLE_BREAK_TRANSMI,
    }
    impl BCW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                BCW::DISABLE_BREAK_TRANSM => false,
                BCW::ENABLE_BREAK_TRANSMI => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BCW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BCW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable break transmission." ]
        # [ inline ( always ) ]
        pub fn disable_break_transm(self) -> &'a mut W {
            self.variant(BCW::DISABLE_BREAK_TRANSM)
        }
        # [ doc = "Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR[6] is active high." ]
        # [ inline ( always ) ]
        pub fn enable_break_transmi(self) -> &'a mut W {
            self.variant(BCW::ENABLE_BREAK_TRANSMI)
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
    # [ doc = "Values that can be written to the field `DLAB`" ]
    pub enum DLABW {
        # [ doc = "Disable access to Divisor Latches." ]
        DISABLE_ACCESS_TO_DI,
        # [ doc = "Enable access to Divisor Latches." ]
        ENABLE_ACCESS_TO_DIV,
    }
    impl DLABW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                DLABW::DISABLE_ACCESS_TO_DI => false,
                DLABW::ENABLE_ACCESS_TO_DIV => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DLABW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DLABW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: DLABW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable access to Divisor Latches." ]
        # [ inline ( always ) ]
        pub fn disable_access_to_di(self) -> &'a mut W {
            self.variant(DLABW::DISABLE_ACCESS_TO_DI)
        }
        # [ doc = "Enable access to Divisor Latches." ]
        # [ inline ( always ) ]
        pub fn enable_access_to_div(self) -> &'a mut W {
            self.variant(DLABW::ENABLE_ACCESS_TO_DIV)
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
        # [ doc = "Bits 0:1 - Word Length Select." ]
        # [ inline ( always ) ]
        pub fn wls(&self) -> WLSR {
            WLSR::_from({
                            const MASK: u8 = 3;
                            const OFFSET: u8 = 0;
                            ((self.bits >> OFFSET) & MASK as u32) as u8
                        })
        }
        # [ doc = "Bit 2 - Stop Bit Select" ]
        # [ inline ( always ) ]
        pub fn sbs(&self) -> SBSR {
            SBSR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 2;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 3 - Parity Enable." ]
        # [ inline ( always ) ]
        pub fn pe(&self) -> PER {
            PER::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 3;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bits 4:5 - Parity Select" ]
        # [ inline ( always ) ]
        pub fn ps(&self) -> PSR {
            PSR::_from({
                           const MASK: u8 = 3;
                           const OFFSET: u8 = 4;
                           ((self.bits >> OFFSET) & MASK as u32) as u8
                       })
        }
        # [ doc = "Bit 6 - Break Control" ]
        # [ inline ( always ) ]
        pub fn bc(&self) -> BCR {
            BCR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 6;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 7 - Divisor Latch Access Bit" ]
        # [ inline ( always ) ]
        pub fn dlab(&self) -> DLABR {
            DLABR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 7;
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
        # [ doc = "Bits 0:1 - Word Length Select." ]
        # [ inline ( always ) ]
        pub fn wls(&mut self) -> _WLSW {
            _WLSW { w: self }
        }
        # [ doc = "Bit 2 - Stop Bit Select" ]
        # [ inline ( always ) ]
        pub fn sbs(&mut self) -> _SBSW {
            _SBSW { w: self }
        }
        # [ doc = "Bit 3 - Parity Enable." ]
        # [ inline ( always ) ]
        pub fn pe(&mut self) -> _PEW {
            _PEW { w: self }
        }
        # [ doc = "Bits 4:5 - Parity Select" ]
        # [ inline ( always ) ]
        pub fn ps(&mut self) -> _PSW {
            _PSW { w: self }
        }
        # [ doc = "Bit 6 - Break Control" ]
        # [ inline ( always ) ]
        pub fn bc(&mut self) -> _BCW {
            _BCW { w: self }
        }
        # [ doc = "Bit 7 - Divisor Latch Access Bit" ]
        # [ inline ( always ) ]
        pub fn dlab(&mut self) -> _DLABW {
            _DLABW { w: self }
        }
    }
}
# [ doc = "Line Status Register. Contains flags for transmit and receive status, including line errors." ]
pub struct LSR {
    register: VolatileCell<u32>,
}
# [ doc = "Line Status Register. Contains flags for transmit and receive status, including line errors." ]
pub mod lsr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::LSR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = "Possible values of the field `RDR`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RDRR {
        # [ doc = "The UARTn receiver FIFO is empty." ]
        EMPTY,
        # [ doc = "The UARTn receiver FIFO is not empty." ]
        NOTEMPTY,
    }
    impl RDRR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RDRR::EMPTY => false,
                RDRR::NOTEMPTY => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RDRR {
            match value {
                false => RDRR::EMPTY,
                true => RDRR::NOTEMPTY,
            }
        }
        # [ doc = "Checks if the value of the field is `EMPTY`" ]
        # [ inline ( always ) ]
        pub fn is_empty(&self) -> bool {
            *self == RDRR::EMPTY
        }
        # [ doc = "Checks if the value of the field is `NOTEMPTY`" ]
        # [ inline ( always ) ]
        pub fn is_notempty(&self) -> bool {
            *self == RDRR::NOTEMPTY
        }
    }
    # [ doc = "Possible values of the field `OE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OER {
        # [ doc = "Overrun error status is inactive." ]
        INACTIVE,
        # [ doc = "Overrun error status is active." ]
        ACTIVE,
    }
    impl OER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                OER::INACTIVE => false,
                OER::ACTIVE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> OER {
            match value {
                false => OER::INACTIVE,
                true => OER::ACTIVE,
            }
        }
        # [ doc = "Checks if the value of the field is `INACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_inactive(&self) -> bool {
            *self == OER::INACTIVE
        }
        # [ doc = "Checks if the value of the field is `ACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_active(&self) -> bool {
            *self == OER::ACTIVE
        }
    }
    # [ doc = "Possible values of the field `PE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PER {
        # [ doc = "Parity error status is inactive." ]
        INACTIVE,
        # [ doc = "Parity error status is active." ]
        ACTIVE,
    }
    impl PER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                PER::INACTIVE => false,
                PER::ACTIVE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PER {
            match value {
                false => PER::INACTIVE,
                true => PER::ACTIVE,
            }
        }
        # [ doc = "Checks if the value of the field is `INACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_inactive(&self) -> bool {
            *self == PER::INACTIVE
        }
        # [ doc = "Checks if the value of the field is `ACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_active(&self) -> bool {
            *self == PER::ACTIVE
        }
    }
    # [ doc = "Possible values of the field `FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum FER {
        # [ doc = "Framing error status is inactive." ]
        INACTIVE,
        # [ doc = "Framing error status is active." ]
        ACTIVE,
    }
    impl FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                FER::INACTIVE => false,
                FER::ACTIVE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> FER {
            match value {
                false => FER::INACTIVE,
                true => FER::ACTIVE,
            }
        }
        # [ doc = "Checks if the value of the field is `INACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_inactive(&self) -> bool {
            *self == FER::INACTIVE
        }
        # [ doc = "Checks if the value of the field is `ACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_active(&self) -> bool {
            *self == FER::ACTIVE
        }
    }
    # [ doc = "Possible values of the field `BI`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum BIR {
        # [ doc = "Break interrupt status is inactive." ]
        INACTIVE,
        # [ doc = "Break interrupt status is active." ]
        ACTIVE,
    }
    impl BIR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                BIR::INACTIVE => false,
                BIR::ACTIVE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> BIR {
            match value {
                false => BIR::INACTIVE,
                true => BIR::ACTIVE,
            }
        }
        # [ doc = "Checks if the value of the field is `INACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_inactive(&self) -> bool {
            *self == BIR::INACTIVE
        }
        # [ doc = "Checks if the value of the field is `ACTIVE`" ]
        # [ inline ( always ) ]
        pub fn is_active(&self) -> bool {
            *self == BIR::ACTIVE
        }
    }
    # [ doc = "Possible values of the field `THRE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum THRER {
        # [ doc = "UnTHR contains valid data." ]
        VALIDDATA,
        # [ doc = "UnTHR is empty." ]
        EMPTY,
    }
    impl THRER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                THRER::VALIDDATA => false,
                THRER::EMPTY => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> THRER {
            match value {
                false => THRER::VALIDDATA,
                true => THRER::EMPTY,
            }
        }
        # [ doc = "Checks if the value of the field is `VALIDDATA`" ]
        # [ inline ( always ) ]
        pub fn is_validdata(&self) -> bool {
            *self == THRER::VALIDDATA
        }
        # [ doc = "Checks if the value of the field is `EMPTY`" ]
        # [ inline ( always ) ]
        pub fn is_empty(&self) -> bool {
            *self == THRER::EMPTY
        }
    }
    # [ doc = "Possible values of the field `TEMT`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TEMTR {
        # [ doc = "UnTHR and/or the UnTSR contains valid data." ]
        VALIDDATA,
        # [ doc = "UnTHR and the UnTSR are empty." ]
        EMPTY,
    }
    impl TEMTR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TEMTR::VALIDDATA => false,
                TEMTR::EMPTY => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TEMTR {
            match value {
                false => TEMTR::VALIDDATA,
                true => TEMTR::EMPTY,
            }
        }
        # [ doc = "Checks if the value of the field is `VALIDDATA`" ]
        # [ inline ( always ) ]
        pub fn is_validdata(&self) -> bool {
            *self == TEMTR::VALIDDATA
        }
        # [ doc = "Checks if the value of the field is `EMPTY`" ]
        # [ inline ( always ) ]
        pub fn is_empty(&self) -> bool {
            *self == TEMTR::EMPTY
        }
    }
    # [ doc = "Possible values of the field `RXFE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RXFER {
        # [ doc = "UnRBR contains no UARTn RX errors or UnFCR[0]=0." ]
        NOERROR,
        # [ doc = "UARTn RBR contains at least one UARTn RX error." ]
        ERRORS,
    }
    impl RXFER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RXFER::NOERROR => false,
                RXFER::ERRORS => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RXFER {
            match value {
                false => RXFER::NOERROR,
                true => RXFER::ERRORS,
            }
        }
        # [ doc = "Checks if the value of the field is `NOERROR`" ]
        # [ inline ( always ) ]
        pub fn is_noerror(&self) -> bool {
            *self == RXFER::NOERROR
        }
        # [ doc = "Checks if the value of the field is `ERRORS`" ]
        # [ inline ( always ) ]
        pub fn is_errors(&self) -> bool {
            *self == RXFER::ERRORS
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Receiver Data Ready. UnLSR[0] is set when the UnRBR holds an unread character and is cleared when the UARTn RBR FIFO is empty." ]
        # [ inline ( always ) ]
        pub fn rdr(&self) -> RDRR {
            RDRR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 0;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An UnLSR read clears UnLSR[1]. UnLSR[1] is set when UARTn RSR has a new character assembled and the UARTn RBR FIFO is full. In this case, the UARTn RBR FIFO will not be overwritten and the character in the UARTn RSR will be lost." ]
        # [ inline ( always ) ]
        pub fn oe(&self) -> OER {
            OER::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 1;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An UnLSR read clears UnLSR[2]. Time of parity error detection is dependent on UnFCR[0]. Note: A parity error is associated with the character at the top of the UARTn RBR FIFO." ]
        # [ inline ( always ) ]
        pub fn pe(&self) -> PER {
            PER::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 2;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An UnLSR read clears UnLSR[3]. The time of the framing error detection is dependent on UnFCR[0]. Upon detection of a framing error, the Rx will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UARTn RBR FIFO." ]
        # [ inline ( always ) ]
        pub fn fe(&self) -> FER {
            FER::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 3;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 4 - Break Interrupt. When RXDn is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXDn goes to marking state (all ones). An UnLSR read clears this status bit. The time of break detection is dependent on UnFCR[0]. Note: The break interrupt is associated with the character at the top of the UARTn RBR FIFO." ]
        # [ inline ( always ) ]
        pub fn bi(&self) -> BIR {
            BIR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 4;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UARTn THR and is cleared on a UnTHR write." ]
        # [ inline ( always ) ]
        pub fn thre(&self) -> THRER {
            THRER::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 5;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 6 - Transmitter Empty. TEMT is set when both UnTHR and UnTSR are empty; TEMT is cleared when either the UnTSR or the UnTHR contain valid data." ]
        # [ inline ( always ) ]
        pub fn temt(&self) -> TEMTR {
            TEMTR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 6;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 7 - Error in RX FIFO . UnLSR[7] is set when a character with a Rx error such as framing error, parity error or break interrupt, is loaded into the UnRBR. This bit is cleared when the UnLSR register is read and there are no subsequent errors in the UARTn FIFO." ]
        # [ inline ( always ) ]
        pub fn rxfe(&self) -> RXFER {
            RXFER::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 7;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
    }
}
# [ doc = "Scratch Pad Register. 8-bit temporary storage for software." ]
pub struct SCR {
    register: VolatileCell<u32>,
}
# [ doc = "Scratch Pad Register. 8-bit temporary storage for software." ]
pub mod scr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::SCR {
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
    pub struct PADR {
        bits: u8,
    }
    impl PADR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PADW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PADW<'a> {
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
        # [ doc = "Bits 0:7 - A readable, writable byte." ]
        # [ inline ( always ) ]
        pub fn pad(&self) -> PADR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PADR { bits }
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
        # [ doc = "Bits 0:7 - A readable, writable byte." ]
        # [ inline ( always ) ]
        pub fn pad(&mut self) -> _PADW {
            _PADW { w: self }
        }
    }
}
# [ doc = "Auto-baud Control Register. Contains controls for the auto-baud feature." ]
pub struct ACR {
    register: VolatileCell<u32>,
}
# [ doc = "Auto-baud Control Register. Contains controls for the auto-baud feature." ]
pub mod acr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ACR {
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
    # [ doc = "Possible values of the field `START`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum STARTR {
        # [ doc = "Auto-baud stop (auto-baud is not running)." ]
        AUTO_BAUD_STOP_AUTO,
        # [ doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion." ]
        AUTO_BAUD_START_AUT,
    }
    impl STARTR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                STARTR::AUTO_BAUD_STOP_AUTO => false,
                STARTR::AUTO_BAUD_START_AUT => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> STARTR {
            match value {
                false => STARTR::AUTO_BAUD_STOP_AUTO,
                true => STARTR::AUTO_BAUD_START_AUT,
            }
        }
        # [ doc = "Checks if the value of the field is `AUTO_BAUD_STOP_AUTO`" ]
        # [ inline ( always ) ]
        pub fn is_auto_baud_stop_auto(&self) -> bool {
            *self == STARTR::AUTO_BAUD_STOP_AUTO
        }
        # [ doc = "Checks if the value of the field is `AUTO_BAUD_START_AUT`" ]
        # [ inline ( always ) ]
        pub fn is_auto_baud_start_aut(&self) -> bool {
            *self == STARTR::AUTO_BAUD_START_AUT
        }
    }
    # [ doc = "Possible values of the field `MODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MODER {
        # [ doc = "Mode 0." ]
        MODE_0_,
        # [ doc = "Mode 1." ]
        MODE_1_,
    }
    impl MODER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                MODER::MODE_0_ => false,
                MODER::MODE_1_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MODER {
            match value {
                false => MODER::MODE_0_,
                true => MODER::MODE_1_,
            }
        }
        # [ doc = "Checks if the value of the field is `MODE_0_`" ]
        # [ inline ( always ) ]
        pub fn is_mode_0_(&self) -> bool {
            *self == MODER::MODE_0_
        }
        # [ doc = "Checks if the value of the field is `MODE_1_`" ]
        # [ inline ( always ) ]
        pub fn is_mode_1_(&self) -> bool {
            *self == MODER::MODE_1_
        }
    }
    # [ doc = "Possible values of the field `AUTORESTART`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum AUTORESTARTR {
        # [ doc = "No restart." ]
        NO_RESTART_,
        # [ doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)" ]
        RESTART_IN_CASE_OF_T,
    }
    impl AUTORESTARTR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                AUTORESTARTR::NO_RESTART_ => false,
                AUTORESTARTR::RESTART_IN_CASE_OF_T => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> AUTORESTARTR {
            match value {
                false => AUTORESTARTR::NO_RESTART_,
                true => AUTORESTARTR::RESTART_IN_CASE_OF_T,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_RESTART_`" ]
        # [ inline ( always ) ]
        pub fn is_no_restart_(&self) -> bool {
            *self == AUTORESTARTR::NO_RESTART_
        }
        # [ doc = "Checks if the value of the field is `RESTART_IN_CASE_OF_T`" ]
        # [ inline ( always ) ]
        pub fn is_restart_in_case_of_t(&self) -> bool {
            *self == AUTORESTARTR::RESTART_IN_CASE_OF_T
        }
    }
    # [ doc = "Possible values of the field `ABEOINTCLR`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ABEOINTCLRR {
        # [ doc = "No impact." ]
        NO_IMPACT_,
        # [ doc = "Clear the corresponding interrupt in the IIR." ]
        CLEAR_THE_CORRESPOND,
    }
    impl ABEOINTCLRR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ABEOINTCLRR::NO_IMPACT_ => false,
                ABEOINTCLRR::CLEAR_THE_CORRESPOND => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABEOINTCLRR {
            match value {
                false => ABEOINTCLRR::NO_IMPACT_,
                true => ABEOINTCLRR::CLEAR_THE_CORRESPOND,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_IMPACT_`" ]
        # [ inline ( always ) ]
        pub fn is_no_impact_(&self) -> bool {
            *self == ABEOINTCLRR::NO_IMPACT_
        }
        # [ doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`" ]
        # [ inline ( always ) ]
        pub fn is_clear_the_correspond(&self) -> bool {
            *self == ABEOINTCLRR::CLEAR_THE_CORRESPOND
        }
    }
    # [ doc = "Possible values of the field `ABTOINTCLR`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ABTOINTCLRR {
        # [ doc = "No impact." ]
        NO_IMPACT_,
        # [ doc = "Clear the corresponding interrupt in the IIR." ]
        CLEAR_THE_CORRESPOND,
    }
    impl ABTOINTCLRR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ABTOINTCLRR::NO_IMPACT_ => false,
                ABTOINTCLRR::CLEAR_THE_CORRESPOND => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABTOINTCLRR {
            match value {
                false => ABTOINTCLRR::NO_IMPACT_,
                true => ABTOINTCLRR::CLEAR_THE_CORRESPOND,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_IMPACT_`" ]
        # [ inline ( always ) ]
        pub fn is_no_impact_(&self) -> bool {
            *self == ABTOINTCLRR::NO_IMPACT_
        }
        # [ doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`" ]
        # [ inline ( always ) ]
        pub fn is_clear_the_correspond(&self) -> bool {
            *self == ABTOINTCLRR::CLEAR_THE_CORRESPOND
        }
    }
    # [ doc = "Values that can be written to the field `START`" ]
    pub enum STARTW {
        # [ doc = "Auto-baud stop (auto-baud is not running)." ]
        AUTO_BAUD_STOP_AUTO,
        # [ doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion." ]
        AUTO_BAUD_START_AUT,
    }
    impl STARTW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                STARTW::AUTO_BAUD_STOP_AUTO => false,
                STARTW::AUTO_BAUD_START_AUT => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _STARTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STARTW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: STARTW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Auto-baud stop (auto-baud is not running)." ]
        # [ inline ( always ) ]
        pub fn auto_baud_stop_auto(self) -> &'a mut W {
            self.variant(STARTW::AUTO_BAUD_STOP_AUTO)
        }
        # [ doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion." ]
        # [ inline ( always ) ]
        pub fn auto_baud_start_aut(self) -> &'a mut W {
            self.variant(STARTW::AUTO_BAUD_START_AUT)
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
    # [ doc = "Values that can be written to the field `MODE`" ]
    pub enum MODEW {
        # [ doc = "Mode 0." ]
        MODE_0_,
        # [ doc = "Mode 1." ]
        MODE_1_,
    }
    impl MODEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MODEW::MODE_0_ => false,
                MODEW::MODE_1_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MODEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Mode 0." ]
        # [ inline ( always ) ]
        pub fn mode_0_(self) -> &'a mut W {
            self.variant(MODEW::MODE_0_)
        }
        # [ doc = "Mode 1." ]
        # [ inline ( always ) ]
        pub fn mode_1_(self) -> &'a mut W {
            self.variant(MODEW::MODE_1_)
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
    # [ doc = "Values that can be written to the field `AUTORESTART`" ]
    pub enum AUTORESTARTW {
        # [ doc = "No restart." ]
        NO_RESTART_,
        # [ doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)" ]
        RESTART_IN_CASE_OF_T,
    }
    impl AUTORESTARTW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                AUTORESTARTW::NO_RESTART_ => false,
                AUTORESTARTW::RESTART_IN_CASE_OF_T => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _AUTORESTARTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AUTORESTARTW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: AUTORESTARTW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "No restart." ]
        # [ inline ( always ) ]
        pub fn no_restart_(self) -> &'a mut W {
            self.variant(AUTORESTARTW::NO_RESTART_)
        }
        # [ doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)" ]
        # [ inline ( always ) ]
        pub fn restart_in_case_of_t(self) -> &'a mut W {
            self.variant(AUTORESTARTW::RESTART_IN_CASE_OF_T)
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
    # [ doc = "Values that can be written to the field `ABEOINTCLR`" ]
    pub enum ABEOINTCLRW {
        # [ doc = "No impact." ]
        NO_IMPACT_,
        # [ doc = "Clear the corresponding interrupt in the IIR." ]
        CLEAR_THE_CORRESPOND,
    }
    impl ABEOINTCLRW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ABEOINTCLRW::NO_IMPACT_ => false,
                ABEOINTCLRW::CLEAR_THE_CORRESPOND => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ABEOINTCLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABEOINTCLRW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ABEOINTCLRW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "No impact." ]
        # [ inline ( always ) ]
        pub fn no_impact_(self) -> &'a mut W {
            self.variant(ABEOINTCLRW::NO_IMPACT_)
        }
        # [ doc = "Clear the corresponding interrupt in the IIR." ]
        # [ inline ( always ) ]
        pub fn clear_the_correspond(self) -> &'a mut W {
            self.variant(ABEOINTCLRW::CLEAR_THE_CORRESPOND)
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
    # [ doc = "Values that can be written to the field `ABTOINTCLR`" ]
    pub enum ABTOINTCLRW {
        # [ doc = "No impact." ]
        NO_IMPACT_,
        # [ doc = "Clear the corresponding interrupt in the IIR." ]
        CLEAR_THE_CORRESPOND,
    }
    impl ABTOINTCLRW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ABTOINTCLRW::NO_IMPACT_ => false,
                ABTOINTCLRW::CLEAR_THE_CORRESPOND => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ABTOINTCLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABTOINTCLRW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ABTOINTCLRW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "No impact." ]
        # [ inline ( always ) ]
        pub fn no_impact_(self) -> &'a mut W {
            self.variant(ABTOINTCLRW::NO_IMPACT_)
        }
        # [ doc = "Clear the corresponding interrupt in the IIR." ]
        # [ inline ( always ) ]
        pub fn clear_the_correspond(self) -> &'a mut W {
            self.variant(ABTOINTCLRW::CLEAR_THE_CORRESPOND)
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
        # [ doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion." ]
        # [ inline ( always ) ]
        pub fn start(&self) -> STARTR {
            STARTR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 1 - Auto-baud mode select bit." ]
        # [ inline ( always ) ]
        pub fn mode(&self) -> MODER {
            MODER::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 1;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 2 - Restart bit." ]
        # [ inline ( always ) ]
        pub fn autorestart(&self) -> AUTORESTARTR {
            AUTORESTARTR::_from({
                                    const MASK: bool = true;
                                    const OFFSET: u8 = 2;
                                    ((self.bits >> OFFSET) & MASK as u32) != 0
                                })
        }
        # [ doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact." ]
        # [ inline ( always ) ]
        pub fn abeointclr(&self) -> ABEOINTCLRR {
            ABEOINTCLRR::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 8;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact." ]
        # [ inline ( always ) ]
        pub fn abtointclr(&self) -> ABTOINTCLRR {
            ABTOINTCLRR::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 9;
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
        # [ doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion." ]
        # [ inline ( always ) ]
        pub fn start(&mut self) -> _STARTW {
            _STARTW { w: self }
        }
        # [ doc = "Bit 1 - Auto-baud mode select bit." ]
        # [ inline ( always ) ]
        pub fn mode(&mut self) -> _MODEW {
            _MODEW { w: self }
        }
        # [ doc = "Bit 2 - Restart bit." ]
        # [ inline ( always ) ]
        pub fn autorestart(&mut self) -> _AUTORESTARTW {
            _AUTORESTARTW { w: self }
        }
        # [ doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact." ]
        # [ inline ( always ) ]
        pub fn abeointclr(&mut self) -> _ABEOINTCLRW {
            _ABEOINTCLRW { w: self }
        }
        # [ doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact." ]
        # [ inline ( always ) ]
        pub fn abtointclr(&mut self) -> _ABTOINTCLRW {
            _ABTOINTCLRW { w: self }
        }
    }
}
# [ doc = "Fractional Divider Register. Generates a clock input for the baud rate divider." ]
pub struct FDR {
    register: VolatileCell<u32>,
}
# [ doc = "Fractional Divider Register. Generates a clock input for the baud rate divider." ]
pub mod fdr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::FDR {
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
    pub struct DIVADDVALR {
        bits: u8,
    }
    impl DIVADDVALR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MULVALR {
        bits: u8,
    }
    impl MULVALR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DIVADDVALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIVADDVALW<'a> {
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
    pub struct _MULVALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MULVALW<'a> {
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:3 - Baud-rate generation pre-scaler divisor value. If this field is 0, fractional baud-rate generator will not impact the UARTn baudrate." ]
        # [ inline ( always ) ]
        pub fn divaddval(&self) -> DIVADDVALR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DIVADDVALR { bits }
        }
        # [ doc = "Bits 4:7 - Baud-rate pre-scaler multiplier value. This field must be greater or equal 1 for UARTn to operate properly, regardless of whether the fractional baud-rate generator is used or not." ]
        # [ inline ( always ) ]
        pub fn mulval(&self) -> MULVALR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MULVALR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 16 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:3 - Baud-rate generation pre-scaler divisor value. If this field is 0, fractional baud-rate generator will not impact the UARTn baudrate." ]
        # [ inline ( always ) ]
        pub fn divaddval(&mut self) -> _DIVADDVALW {
            _DIVADDVALW { w: self }
        }
        # [ doc = "Bits 4:7 - Baud-rate pre-scaler multiplier value. This field must be greater or equal 1 for UARTn to operate properly, regardless of whether the fractional baud-rate generator is used or not." ]
        # [ inline ( always ) ]
        pub fn mulval(&mut self) -> _MULVALW {
            _MULVALW { w: self }
        }
    }
}
# [ doc = "Transmit Enable Register. Turns off UART transmitter for use with software flow control." ]
pub struct TER {
    register: VolatileCell<u32>,
}
# [ doc = "Transmit Enable Register. Turns off UART transmitter for use with software flow control." ]
pub mod ter {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TER {
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
    pub struct TXENR {
        bits: bool,
    }
    impl TXENR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _TXENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXENW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 7 - When this bit is 1, as it is after a Reset, data written to the THR is output on the TXD pin as soon as any preceding data has been sent. If this bit is cleared to 0 while a character is being sent, the transmission of that character is completed, but no further characters are sent until this bit is set again. In other words, a 0 in this bit blocks the transfer of characters from the THR or TX FIFO into the transmit shift register. Software implementing software-handshaking can clear this bit when it receives an XOFF character (DC3). Software can set this bit again when it receives an XON (DC1) character." ]
        # [ inline ( always ) ]
        pub fn txen(&self) -> TXENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TXENR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 128 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 7 - When this bit is 1, as it is after a Reset, data written to the THR is output on the TXD pin as soon as any preceding data has been sent. If this bit is cleared to 0 while a character is being sent, the transmission of that character is completed, but no further characters are sent until this bit is set again. In other words, a 0 in this bit blocks the transfer of characters from the THR or TX FIFO into the transmit shift register. Software implementing software-handshaking can clear this bit when it receives an XOFF character (DC3). Software can set this bit again when it receives an XON (DC1) character." ]
        # [ inline ( always ) ]
        pub fn txen(&mut self) -> _TXENW {
            _TXENW { w: self }
        }
    }
}
# [ doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes." ]
pub struct RS485CTRL {
    register: VolatileCell<u32>,
}
# [ doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes." ]
pub mod rs485ctrl {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RS485CTRL {
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
    # [ doc = "Possible values of the field `NMMEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum NMMENR {
        # [ doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled." ]
        DISABLED,
        # [ doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte has the parity bit = 1, generating a received data interrupt. See Section 18.6.16 RS-485/EIA-485 modes of operation." ]
        ENABLED,
    }
    impl NMMENR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                NMMENR::DISABLED => false,
                NMMENR::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> NMMENR {
            match value {
                false => NMMENR::DISABLED,
                true => NMMENR::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == NMMENR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == NMMENR::ENABLED
        }
    }
    # [ doc = "Possible values of the field `RXDIS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RXDISR {
        # [ doc = "The receiver is enabled." ]
        ENABLED,
        # [ doc = "The receiver is disabled." ]
        DISABLED,
    }
    impl RXDISR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RXDISR::ENABLED => false,
                RXDISR::DISABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RXDISR {
            match value {
                false => RXDISR::ENABLED,
                true => RXDISR::DISABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == RXDISR::ENABLED
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == RXDISR::DISABLED
        }
    }
    # [ doc = "Possible values of the field `AADEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum AADENR {
        # [ doc = "Auto Address Detect (AAD) is disabled." ]
        DISABLED,
        # [ doc = "Auto Address Detect (AAD) is enabled." ]
        ENABLED,
    }
    impl AADENR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                AADENR::DISABLED => false,
                AADENR::ENABLED => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> AADENR {
            match value {
                false => AADENR::DISABLED,
                true => AADENR::ENABLED,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED`" ]
        # [ inline ( always ) ]
        pub fn is_disabled(&self) -> bool {
            *self == AADENR::DISABLED
        }
        # [ doc = "Checks if the value of the field is `ENABLED`" ]
        # [ inline ( always ) ]
        pub fn is_enabled(&self) -> bool {
            *self == AADENR::ENABLED
        }
    }
    # [ doc = "Possible values of the field `DCTRL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DCTRLR {
        # [ doc = "Disable Auto Direction Control." ]
        DISABLE_AUTO_DIRECTI,
        # [ doc = "Enable Auto Direction Control." ]
        ENABLE_AUTO_DIRECTIO,
    }
    impl DCTRLR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DCTRLR::DISABLE_AUTO_DIRECTI => false,
                DCTRLR::ENABLE_AUTO_DIRECTIO => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DCTRLR {
            match value {
                false => DCTRLR::DISABLE_AUTO_DIRECTI,
                true => DCTRLR::ENABLE_AUTO_DIRECTIO,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_AUTO_DIRECTI`" ]
        # [ inline ( always ) ]
        pub fn is_disable_auto_directi(&self) -> bool {
            *self == DCTRLR::DISABLE_AUTO_DIRECTI
        }
        # [ doc = "Checks if the value of the field is `ENABLE_AUTO_DIRECTIO`" ]
        # [ inline ( always ) ]
        pub fn is_enable_auto_directio(&self) -> bool {
            *self == DCTRLR::ENABLE_AUTO_DIRECTIO
        }
    }
    # [ doc = "Possible values of the field `OINV`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OINVR {
        # [ doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted." ]
        DIRLOW,
        # [ doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted." ]
        DIRHIGH,
    }
    impl OINVR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                OINVR::DIRLOW => false,
                OINVR::DIRHIGH => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> OINVR {
            match value {
                false => OINVR::DIRLOW,
                true => OINVR::DIRHIGH,
            }
        }
        # [ doc = "Checks if the value of the field is `DIRLOW`" ]
        # [ inline ( always ) ]
        pub fn is_dirlow(&self) -> bool {
            *self == OINVR::DIRLOW
        }
        # [ doc = "Checks if the value of the field is `DIRHIGH`" ]
        # [ inline ( always ) ]
        pub fn is_dirhigh(&self) -> bool {
            *self == OINVR::DIRHIGH
        }
    }
    # [ doc = "Values that can be written to the field `NMMEN`" ]
    pub enum NMMENW {
        # [ doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled." ]
        DISABLED,
        # [ doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte has the parity bit = 1, generating a received data interrupt. See Section 18.6.16 RS-485/EIA-485 modes of operation." ]
        ENABLED,
    }
    impl NMMENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                NMMENW::DISABLED => false,
                NMMENW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _NMMENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _NMMENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: NMMENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(NMMENW::DISABLED)
        }
        # [ doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte has the parity bit = 1, generating a received data interrupt. See Section 18.6.16 RS-485/EIA-485 modes of operation." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(NMMENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `RXDIS`" ]
    pub enum RXDISW {
        # [ doc = "The receiver is enabled." ]
        ENABLED,
        # [ doc = "The receiver is disabled." ]
        DISABLED,
    }
    impl RXDISW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                RXDISW::ENABLED => false,
                RXDISW::DISABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RXDISW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXDISW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RXDISW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The receiver is enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(RXDISW::ENABLED)
        }
        # [ doc = "The receiver is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(RXDISW::DISABLED)
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
    # [ doc = "Values that can be written to the field `AADEN`" ]
    pub enum AADENW {
        # [ doc = "Auto Address Detect (AAD) is disabled." ]
        DISABLED,
        # [ doc = "Auto Address Detect (AAD) is enabled." ]
        ENABLED,
    }
    impl AADENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                AADENW::DISABLED => false,
                AADENW::ENABLED => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _AADENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AADENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: AADENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Auto Address Detect (AAD) is disabled." ]
        # [ inline ( always ) ]
        pub fn disabled(self) -> &'a mut W {
            self.variant(AADENW::DISABLED)
        }
        # [ doc = "Auto Address Detect (AAD) is enabled." ]
        # [ inline ( always ) ]
        pub fn enabled(self) -> &'a mut W {
            self.variant(AADENW::ENABLED)
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
    # [ doc = "Values that can be written to the field `DCTRL`" ]
    pub enum DCTRLW {
        # [ doc = "Disable Auto Direction Control." ]
        DISABLE_AUTO_DIRECTI,
        # [ doc = "Enable Auto Direction Control." ]
        ENABLE_AUTO_DIRECTIO,
    }
    impl DCTRLW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                DCTRLW::DISABLE_AUTO_DIRECTI => false,
                DCTRLW::ENABLE_AUTO_DIRECTIO => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DCTRLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DCTRLW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: DCTRLW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable Auto Direction Control." ]
        # [ inline ( always ) ]
        pub fn disable_auto_directi(self) -> &'a mut W {
            self.variant(DCTRLW::DISABLE_AUTO_DIRECTI)
        }
        # [ doc = "Enable Auto Direction Control." ]
        # [ inline ( always ) ]
        pub fn enable_auto_directio(self) -> &'a mut W {
            self.variant(DCTRLW::ENABLE_AUTO_DIRECTIO)
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
    # [ doc = "Values that can be written to the field `OINV`" ]
    pub enum OINVW {
        # [ doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted." ]
        DIRLOW,
        # [ doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted." ]
        DIRHIGH,
    }
    impl OINVW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                OINVW::DIRLOW => false,
                OINVW::DIRHIGH => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OINVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OINVW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: OINVW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted." ]
        # [ inline ( always ) ]
        pub fn dirlow(self) -> &'a mut W {
            self.variant(OINVW::DIRLOW)
        }
        # [ doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted." ]
        # [ inline ( always ) ]
        pub fn dirhigh(self) -> &'a mut W {
            self.variant(OINVW::DIRHIGH)
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
        # [ doc = "Bit 0 - NMM enable." ]
        # [ inline ( always ) ]
        pub fn nmmen(&self) -> NMMENR {
            NMMENR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 1 - Receiver enable." ]
        # [ inline ( always ) ]
        pub fn rxdis(&self) -> RXDISR {
            RXDISR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 1;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 2 - AAD enable." ]
        # [ inline ( always ) ]
        pub fn aaden(&self) -> AADENR {
            AADENR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 2;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 4 - Direction control enable." ]
        # [ inline ( always ) ]
        pub fn dctrl(&self) -> DCTRLR {
            DCTRLR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 4;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin." ]
        # [ inline ( always ) ]
        pub fn oinv(&self) -> OINVR {
            OINVR::_from({
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
        # [ doc = "Bit 0 - NMM enable." ]
        # [ inline ( always ) ]
        pub fn nmmen(&mut self) -> _NMMENW {
            _NMMENW { w: self }
        }
        # [ doc = "Bit 1 - Receiver enable." ]
        # [ inline ( always ) ]
        pub fn rxdis(&mut self) -> _RXDISW {
            _RXDISW { w: self }
        }
        # [ doc = "Bit 2 - AAD enable." ]
        # [ inline ( always ) ]
        pub fn aaden(&mut self) -> _AADENW {
            _AADENW { w: self }
        }
        # [ doc = "Bit 4 - Direction control enable." ]
        # [ inline ( always ) ]
        pub fn dctrl(&mut self) -> _DCTRLW {
            _DCTRLW { w: self }
        }
        # [ doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin." ]
        # [ inline ( always ) ]
        pub fn oinv(&mut self) -> _OINVW {
            _OINVW { w: self }
        }
    }
}
# [ doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode." ]
pub struct RS485ADRMATCH {
    register: VolatileCell<u32>,
}
# [ doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode." ]
pub mod rs485adrmatch {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RS485ADRMATCH {
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
    pub struct ADRMATCHR {
        bits: u8,
    }
    impl ADRMATCHR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADRMATCHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADRMATCHW<'a> {
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
        # [ doc = "Bits 0:7 - Contains the address match value." ]
        # [ inline ( always ) ]
        pub fn adrmatch(&self) -> ADRMATCHR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADRMATCHR { bits }
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
        # [ doc = "Bits 0:7 - Contains the address match value." ]
        # [ inline ( always ) ]
        pub fn adrmatch(&mut self) -> _ADRMATCHW {
            _ADRMATCHW { w: self }
        }
    }
}
# [ doc = "RS-485/EIA-485 direction control delay." ]
pub struct RS485DLY {
    register: VolatileCell<u32>,
}
# [ doc = "RS-485/EIA-485 direction control delay." ]
pub mod rs485dly {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RS485DLY {
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
    pub struct DLYR {
        bits: u8,
    }
    impl DLYR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DLYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DLYW<'a> {
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
        # [ doc = "Bits 0:7 - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter." ]
        # [ inline ( always ) ]
        pub fn dly(&self) -> DLYR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DLYR { bits }
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
        # [ doc = "Bits 0:7 - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter." ]
        # [ inline ( always ) ]
        pub fn dly(&mut self) -> _DLYW {
            _DLYW { w: self }
        }
    }
}
# [ doc = "UART0/2/3" ]
pub struct UART0 {
    register_block: RegisterBlock,
}
impl Deref for UART0 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "UART2" ]
pub const UART2: Peripheral<UART2> = unsafe { Peripheral::new(1074364416) };
# [ doc = r" Register block" ]
pub struct UART2 {
    register_block: RegisterBlock,
}
impl Deref for UART2 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "UART3" ]
pub const UART3: Peripheral<UART3> = unsafe { Peripheral::new(1074380800) };
# [ doc = r" Register block" ]
pub struct UART3 {
    register_block: RegisterBlock,
}
impl Deref for UART3 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
