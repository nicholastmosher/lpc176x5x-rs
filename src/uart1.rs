# ! [ doc = "UART1" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "UART1" ]
pub const UART1: Peripheral<UART1> = unsafe { Peripheral::new(1073807360) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - DLAB =0 Receiver Buffer Register. Contains the next received character to be read." ]
    pub rbr: RBR,
    # [ doc = "0x04 - DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider." ]
    pub dlm: DLM,
    # [ doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending." ]
    pub iir: IIR,
    # [ doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation." ]
    pub lcr: LCR,
    # [ doc = "0x10 - Modem Control Register. Contains controls for flow control handshaking and loopback mode." ]
    pub mcr: MCR,
    # [ doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors." ]
    pub lsr: LSR,
    # [ doc = "0x18 - Modem Status Register. Contains handshake signal status flags." ]
    pub msr: MSR,
    # [ doc = "0x1c - Scratch Pad Register. 8-bit temporary storage for software." ]
    pub scr: SCR,
    # [ doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature." ]
    pub acr: ACR,
    _reserved0: [u8; 4usize],
    # [ doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider." ]
    pub fdr: FDR,
    _reserved1: [u8; 4usize],
    # [ doc = "0x30 - Transmit Enable Register. Turns off UART transmitter for use with software flow control." ]
    pub ter: TER,
    _reserved2: [u8; 24usize],
    # [ doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes." ]
    pub rs485ctrl: RS485CTRL,
    # [ doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode." ]
    pub rs485adrmatch: RS485ADRMATCH,
    # [ doc = "0x54 - RS-485/EIA-485 direction control delay." ]
    pub rs485dly: RS485DLY,
}
# [ doc = "DLAB =0 Receiver Buffer Register. Contains the next received character to be read." ]
pub struct RBR {
    register: VolatileCell<u32>,
}
# [ doc = "DLAB =0 Receiver Buffer Register. Contains the next received character to be read." ]
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
        # [ doc = "Bits 0:7 - The UART1 Receiver Buffer Register contains the oldest received byte in the UART1 RX FIFO." ]
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
# [ doc = "DLAB =0. Transmit Holding Register. The next character to be transmitted is written here." ]
pub struct THR {
    register: VolatileCell<u32>,
}
# [ doc = "DLAB =0. Transmit Holding Register. The next character to be transmitted is written here." ]
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
        # [ doc = "Bits 0:7 - Writing to the UART1 Transmit Holding Register causes the data to be stored in the UART1 transmit FIFO. The byte will be sent when it reaches the bottom of the FIFO and the transmitter is available." ]
        # [ inline ( always ) ]
        pub fn thr(&mut self) -> _THRW {
            _THRW { w: self }
        }
    }
}
# [ doc = "DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider." ]
pub struct DLL {
    register: VolatileCell<u32>,
}
# [ doc = "DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider." ]
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
        # [ doc = "Bits 0:7 - The UART1 Divisor Latch LSB Register, along with the U1DLM register, determines the baud rate of the UART1." ]
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
        # [ doc = "Bits 0:7 - The UART1 Divisor Latch LSB Register, along with the U1DLM register, determines the baud rate of the UART1." ]
        # [ inline ( always ) ]
        pub fn dllsb(&mut self) -> _DLLSBW {
            _DLLSBW { w: self }
        }
    }
}
# [ doc = "DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider." ]
pub struct DLM {
    register: VolatileCell<u32>,
}
# [ doc = "DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider." ]
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
        # [ doc = "Bits 0:7 - The UART1 Divisor Latch MSB Register, along with the U1DLL register, determines the baud rate of the UART1." ]
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
        # [ doc = "Bits 0:7 - The UART1 Divisor Latch MSB Register, along with the U1DLL register, determines the baud rate of the UART1." ]
        # [ inline ( always ) ]
        pub fn dlmsb(&mut self) -> _DLMSBW {
            _DLMSBW { w: self }
        }
    }
}
# [ doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts." ]
pub struct IER {
    register: VolatileCell<u32>,
}
# [ doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts." ]
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
    # [ doc = "Possible values of the field `MSIE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum MSIER {
        # [ doc = "Disable the modem interrupt." ]
        DISABLE_THE_MODEM_IN,
        # [ doc = "Enable the modem interrupt." ]
        ENABLE_THE_MODEM_INT,
    }
    impl MSIER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                MSIER::DISABLE_THE_MODEM_IN => false,
                MSIER::ENABLE_THE_MODEM_INT => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> MSIER {
            match value {
                false => MSIER::DISABLE_THE_MODEM_IN,
                true => MSIER::ENABLE_THE_MODEM_INT,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_THE_MODEM_IN`" ]
        # [ inline ( always ) ]
        pub fn is_disable_the_modem_in(&self) -> bool {
            *self == MSIER::DISABLE_THE_MODEM_IN
        }
        # [ doc = "Checks if the value of the field is `ENABLE_THE_MODEM_INT`" ]
        # [ inline ( always ) ]
        pub fn is_enable_the_modem_int(&self) -> bool {
            *self == MSIER::ENABLE_THE_MODEM_INT
        }
    }
    # [ doc = "Possible values of the field `CTSIE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CTSIER {
        # [ doc = "Disable the CTS interrupt." ]
        DISABLE_THE_CTS_INTE,
        # [ doc = "Enable the CTS interrupt." ]
        ENABLE_THE_CTS_INTER,
    }
    impl CTSIER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CTSIER::DISABLE_THE_CTS_INTE => false,
                CTSIER::ENABLE_THE_CTS_INTER => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CTSIER {
            match value {
                false => CTSIER::DISABLE_THE_CTS_INTE,
                true => CTSIER::ENABLE_THE_CTS_INTER,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_THE_CTS_INTE`" ]
        # [ inline ( always ) ]
        pub fn is_disable_the_cts_inte(&self) -> bool {
            *self == CTSIER::DISABLE_THE_CTS_INTE
        }
        # [ doc = "Checks if the value of the field is `ENABLE_THE_CTS_INTER`" ]
        # [ inline ( always ) ]
        pub fn is_enable_the_cts_inter(&self) -> bool {
            *self == CTSIER::ENABLE_THE_CTS_INTER
        }
    }
    # [ doc = "Possible values of the field `ABEOIE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ABEOIER {
        # [ doc = "Disable end of auto-baud Interrupt." ]
        DISABLE_END_OF_AUTO_,
        # [ doc = "Enable end of auto-baud Interrupt." ]
        ENABLE_END_OF_AUTO_B,
    }
    impl ABEOIER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ABEOIER::DISABLE_END_OF_AUTO_ => false,
                ABEOIER::ENABLE_END_OF_AUTO_B => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABEOIER {
            match value {
                false => ABEOIER::DISABLE_END_OF_AUTO_,
                true => ABEOIER::ENABLE_END_OF_AUTO_B,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_END_OF_AUTO_`" ]
        # [ inline ( always ) ]
        pub fn is_disable_end_of_auto_(&self) -> bool {
            *self == ABEOIER::DISABLE_END_OF_AUTO_
        }
        # [ doc = "Checks if the value of the field is `ENABLE_END_OF_AUTO_B`" ]
        # [ inline ( always ) ]
        pub fn is_enable_end_of_auto_b(&self) -> bool {
            *self == ABEOIER::ENABLE_END_OF_AUTO_B
        }
    }
    # [ doc = "Possible values of the field `ABTOIE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ABTOIER {
        # [ doc = "Disable auto-baud time-out Interrupt." ]
        DISABLE_AUTO_BAUD_TI,
        # [ doc = "Enable auto-baud time-out Interrupt." ]
        ENABLE_AUTO_BAUD_TIM,
    }
    impl ABTOIER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ABTOIER::DISABLE_AUTO_BAUD_TI => false,
                ABTOIER::ENABLE_AUTO_BAUD_TIM => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABTOIER {
            match value {
                false => ABTOIER::DISABLE_AUTO_BAUD_TI,
                true => ABTOIER::ENABLE_AUTO_BAUD_TIM,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_AUTO_BAUD_TI`" ]
        # [ inline ( always ) ]
        pub fn is_disable_auto_baud_ti(&self) -> bool {
            *self == ABTOIER::DISABLE_AUTO_BAUD_TI
        }
        # [ doc = "Checks if the value of the field is `ENABLE_AUTO_BAUD_TIM`" ]
        # [ inline ( always ) ]
        pub fn is_enable_auto_baud_tim(&self) -> bool {
            *self == ABTOIER::ENABLE_AUTO_BAUD_TIM
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
    # [ doc = "Values that can be written to the field `MSIE`" ]
    pub enum MSIEW {
        # [ doc = "Disable the modem interrupt." ]
        DISABLE_THE_MODEM_IN,
        # [ doc = "Enable the modem interrupt." ]
        ENABLE_THE_MODEM_INT,
    }
    impl MSIEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                MSIEW::DISABLE_THE_MODEM_IN => false,
                MSIEW::ENABLE_THE_MODEM_INT => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MSIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MSIEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: MSIEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable the modem interrupt." ]
        # [ inline ( always ) ]
        pub fn disable_the_modem_in(self) -> &'a mut W {
            self.variant(MSIEW::DISABLE_THE_MODEM_IN)
        }
        # [ doc = "Enable the modem interrupt." ]
        # [ inline ( always ) ]
        pub fn enable_the_modem_int(self) -> &'a mut W {
            self.variant(MSIEW::ENABLE_THE_MODEM_INT)
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
    # [ doc = "Values that can be written to the field `CTSIE`" ]
    pub enum CTSIEW {
        # [ doc = "Disable the CTS interrupt." ]
        DISABLE_THE_CTS_INTE,
        # [ doc = "Enable the CTS interrupt." ]
        ENABLE_THE_CTS_INTER,
    }
    impl CTSIEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CTSIEW::DISABLE_THE_CTS_INTE => false,
                CTSIEW::ENABLE_THE_CTS_INTER => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CTSIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTSIEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CTSIEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable the CTS interrupt." ]
        # [ inline ( always ) ]
        pub fn disable_the_cts_inte(self) -> &'a mut W {
            self.variant(CTSIEW::DISABLE_THE_CTS_INTE)
        }
        # [ doc = "Enable the CTS interrupt." ]
        # [ inline ( always ) ]
        pub fn enable_the_cts_inter(self) -> &'a mut W {
            self.variant(CTSIEW::ENABLE_THE_CTS_INTER)
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
    # [ doc = "Values that can be written to the field `ABEOIE`" ]
    pub enum ABEOIEW {
        # [ doc = "Disable end of auto-baud Interrupt." ]
        DISABLE_END_OF_AUTO_,
        # [ doc = "Enable end of auto-baud Interrupt." ]
        ENABLE_END_OF_AUTO_B,
    }
    impl ABEOIEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ABEOIEW::DISABLE_END_OF_AUTO_ => false,
                ABEOIEW::ENABLE_END_OF_AUTO_B => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ABEOIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABEOIEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ABEOIEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable end of auto-baud Interrupt." ]
        # [ inline ( always ) ]
        pub fn disable_end_of_auto_(self) -> &'a mut W {
            self.variant(ABEOIEW::DISABLE_END_OF_AUTO_)
        }
        # [ doc = "Enable end of auto-baud Interrupt." ]
        # [ inline ( always ) ]
        pub fn enable_end_of_auto_b(self) -> &'a mut W {
            self.variant(ABEOIEW::ENABLE_END_OF_AUTO_B)
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
    # [ doc = "Values that can be written to the field `ABTOIE`" ]
    pub enum ABTOIEW {
        # [ doc = "Disable auto-baud time-out Interrupt." ]
        DISABLE_AUTO_BAUD_TI,
        # [ doc = "Enable auto-baud time-out Interrupt." ]
        ENABLE_AUTO_BAUD_TIM,
    }
    impl ABTOIEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ABTOIEW::DISABLE_AUTO_BAUD_TI => false,
                ABTOIEW::ENABLE_AUTO_BAUD_TIM => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ABTOIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABTOIEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ABTOIEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable auto-baud time-out Interrupt." ]
        # [ inline ( always ) ]
        pub fn disable_auto_baud_ti(self) -> &'a mut W {
            self.variant(ABTOIEW::DISABLE_AUTO_BAUD_TI)
        }
        # [ doc = "Enable auto-baud time-out Interrupt." ]
        # [ inline ( always ) ]
        pub fn enable_auto_baud_tim(self) -> &'a mut W {
            self.variant(ABTOIEW::ENABLE_AUTO_BAUD_TIM)
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
        # [ doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt." ]
        # [ inline ( always ) ]
        pub fn rbrie(&self) -> RBRIER {
            RBRIER::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR[5]." ]
        # [ inline ( always ) ]
        pub fn threie(&self) -> THREIER {
            THREIER::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 1;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR[4:1]." ]
        # [ inline ( always ) ]
        pub fn rxie(&self) -> RXIER {
            RXIER::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 2;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR[3:0]." ]
        # [ inline ( always ) ]
        pub fn msie(&self) -> MSIER {
            MSIER::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 3;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER[3]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER[3] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER[3] and IER[7] bits are set." ]
        # [ inline ( always ) ]
        pub fn ctsie(&self) -> CTSIER {
            CTSIER::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 7;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 8 - Enables the end of auto-baud interrupt." ]
        # [ inline ( always ) ]
        pub fn abeoie(&self) -> ABEOIER {
            ABEOIER::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 8;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 9 - Enables the auto-baud time-out interrupt." ]
        # [ inline ( always ) ]
        pub fn abtoie(&self) -> ABTOIER {
            ABTOIER::_from({
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
        # [ doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt." ]
        # [ inline ( always ) ]
        pub fn rbrie(&mut self) -> _RBRIEW {
            _RBRIEW { w: self }
        }
        # [ doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR[5]." ]
        # [ inline ( always ) ]
        pub fn threie(&mut self) -> _THREIEW {
            _THREIEW { w: self }
        }
        # [ doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR[4:1]." ]
        # [ inline ( always ) ]
        pub fn rxie(&mut self) -> _RXIEW {
            _RXIEW { w: self }
        }
        # [ doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR[3:0]." ]
        # [ inline ( always ) ]
        pub fn msie(&mut self) -> _MSIEW {
            _MSIEW { w: self }
        }
        # [ doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER[3]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER[3] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER[3] and IER[7] bits are set." ]
        # [ inline ( always ) ]
        pub fn ctsie(&mut self) -> _CTSIEW {
            _CTSIEW { w: self }
        }
        # [ doc = "Bit 8 - Enables the end of auto-baud interrupt." ]
        # [ inline ( always ) ]
        pub fn abeoie(&mut self) -> _ABEOIEW {
            _ABEOIEW { w: self }
        }
        # [ doc = "Bit 9 - Enables the auto-baud time-out interrupt." ]
        # [ inline ( always ) ]
        pub fn abtoie(&mut self) -> _ABTOIEW {
            _ABTOIEW { w: self }
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
        RLS,
        # [ doc = "2a - Receive Data Available (RDA)." ]
        RDA,
        # [ doc = "2b - Character Time-out Indicator (CTI)." ]
        CTI,
        # [ doc = "3   - THRE Interrupt." ]
        THRE,
        # [ doc = "4   - Modem Interrupt." ]
        MODEM,
        # [ doc = r" Reserved" ]
        _Reserved(u8),
    }
    impl INTIDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                INTIDR::RLS => 3,
                INTIDR::RDA => 2,
                INTIDR::CTI => 6,
                INTIDR::THRE => 1,
                INTIDR::MODEM => 0,
                INTIDR::_Reserved(bits) => bits,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> INTIDR {
            match value {
                3 => INTIDR::RLS,
                2 => INTIDR::RDA,
                6 => INTIDR::CTI,
                1 => INTIDR::THRE,
                0 => INTIDR::MODEM,
                i => INTIDR::_Reserved(i),
            }
        }
        # [ doc = "Checks if the value of the field is `RLS`" ]
        # [ inline ( always ) ]
        pub fn is_rls(&self) -> bool {
            *self == INTIDR::RLS
        }
        # [ doc = "Checks if the value of the field is `RDA`" ]
        # [ inline ( always ) ]
        pub fn is_rda(&self) -> bool {
            *self == INTIDR::RDA
        }
        # [ doc = "Checks if the value of the field is `CTI`" ]
        # [ inline ( always ) ]
        pub fn is_cti(&self) -> bool {
            *self == INTIDR::CTI
        }
        # [ doc = "Checks if the value of the field is `THRE`" ]
        # [ inline ( always ) ]
        pub fn is_thre(&self) -> bool {
            *self == INTIDR::THRE
        }
        # [ doc = "Checks if the value of the field is `MODEM`" ]
        # [ inline ( always ) ]
        pub fn is_modem(&self) -> bool {
            *self == INTIDR::MODEM
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
        # [ doc = "Bit 0 - Interrupt status. Note that IIR[0] is active low. The pending interrupt can be determined by evaluating IIR[3:1]." ]
        # [ inline ( always ) ]
        pub fn intstatus(&self) -> INTSTATUSR {
            INTSTATUSR::_from({
                                  const MASK: bool = true;
                                  const OFFSET: u8 = 0;
                                  ((self.bits >> OFFSET) & MASK as u32) != 0
                              })
        }
        # [ doc = "Bits 1:3 - Interrupt identification. IER[3:1] identifies an interrupt corresponding to the UART1 Rx or TX FIFO. All other combinations of IER[3:1] not listed below are reserved (100,101,111)." ]
        # [ inline ( always ) ]
        pub fn intid(&self) -> INTIDR {
            INTIDR::_from({
                              const MASK: u8 = 7;
                              const OFFSET: u8 = 1;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bits 6:7 - Copies of FCR[0]." ]
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
# [ doc = "FIFO Control Register. Controls UART1 FIFO usage and modes." ]
pub struct FCR {
    register: VolatileCell<u32>,
}
# [ doc = "FIFO Control Register. Controls UART1 FIFO usage and modes." ]
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
        # [ doc = "Must not be used in the application." ]
        MUST_NOT_BE_USED_IN_,
        # [ doc = "Active high enable for both UART1 Rx and TX FIFOs and FCR[7:1] access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs." ]
        ACTIVE_HIGH_ENABLE_F,
    }
    impl FIFOENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                FIFOENW::MUST_NOT_BE_USED_IN_ => false,
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
        # [ doc = "Must not be used in the application." ]
        # [ inline ( always ) ]
        pub fn must_not_be_used_in_(self) -> &'a mut W {
            self.variant(FIFOENW::MUST_NOT_BE_USED_IN_)
        }
        # [ doc = "Active high enable for both UART1 Rx and TX FIFOs and FCR[7:1] access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs." ]
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
        # [ doc = "No impact on either of UART1 FIFOs." ]
        NO_IMPACT_ON_EITHER_,
        # [ doc = "Writing a logic 1 to FCR[1] will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing." ]
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
        # [ doc = "No impact on either of UART1 FIFOs." ]
        # [ inline ( always ) ]
        pub fn no_impact_on_either_(self) -> &'a mut W {
            self.variant(RXFIFORESW::NO_IMPACT_ON_EITHER_)
        }
        # [ doc = "Writing a logic 1 to FCR[1] will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing." ]
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
        # [ doc = "No impact on either of UART1 FIFOs." ]
        NO_IMPACT_ON_EITHER_,
        # [ doc = "Writing a logic 1 to FCR[2] will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing." ]
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
        # [ doc = "No impact on either of UART1 FIFOs." ]
        # [ inline ( always ) ]
        pub fn no_impact_on_either_(self) -> &'a mut W {
            self.variant(TXFIFORESW::NO_IMPACT_ON_EITHER_)
        }
        # [ doc = "Writing a logic 1 to FCR[2] will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing." ]
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
        # [ doc = "Bit 0 - FIFO enable." ]
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
        # [ doc = "Bit 3 - DMA Mode Select. When the FIFO enable bit (bit 0 of this register) is set, this bit selects the DMA mode. See Section 36.6.6.1." ]
        # [ inline ( always ) ]
        pub fn dmamode(&mut self) -> _DMAMODEW {
            _DMAMODEW { w: self }
        }
        # [ doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UART1 FIFO characters must be written before an interrupt is activated." ]
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
        # [ doc = "5-bit character length." ]
        _5_BIT_CHARACTER_LENG,
        # [ doc = "6-bit character length." ]
        _6_BIT_CHARACTER_LENG,
        # [ doc = "7-bit character length." ]
        _7_BIT_CHARACTER_LENG,
        # [ doc = "8-bit character length." ]
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
        # [ doc = "2 stop bits (1.5 if LCR[1:0]=00)." ]
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
        FORCED1STICK_PAR,
        # [ doc = "Forced 0 stick parity." ]
        FORCED0STICK_PAR,
    }
    impl PSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                PSR::ODD_PARITY_NUMBER_O => 0,
                PSR::EVEN_PARITY_NUMBER_ => 1,
                PSR::FORCED1STICK_PAR => 2,
                PSR::FORCED0STICK_PAR => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> PSR {
            match value {
                0 => PSR::ODD_PARITY_NUMBER_O,
                1 => PSR::EVEN_PARITY_NUMBER_,
                2 => PSR::FORCED1STICK_PAR,
                3 => PSR::FORCED0STICK_PAR,
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
        # [ doc = "Checks if the value of the field is `FORCED1STICK_PAR`" ]
        # [ inline ( always ) ]
        pub fn is_forced1stick_par(&self) -> bool {
            *self == PSR::FORCED1STICK_PAR
        }
        # [ doc = "Checks if the value of the field is `FORCED0STICK_PAR`" ]
        # [ inline ( always ) ]
        pub fn is_forced0stick_par(&self) -> bool {
            *self == PSR::FORCED0STICK_PAR
        }
    }
    # [ doc = "Possible values of the field `BC`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum BCR {
        # [ doc = "Disable break transmission." ]
        DISABLE_BREAK_TRANSM,
        # [ doc = "Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR[6] is active high." ]
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
        # [ doc = "5-bit character length." ]
        _5_BIT_CHARACTER_LENG,
        # [ doc = "6-bit character length." ]
        _6_BIT_CHARACTER_LENG,
        # [ doc = "7-bit character length." ]
        _7_BIT_CHARACTER_LENG,
        # [ doc = "8-bit character length." ]
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
        # [ doc = "5-bit character length." ]
        # [ inline ( always ) ]
        pub fn _5_bit_character_leng(self) -> &'a mut W {
            self.variant(WLSW::_5_BIT_CHARACTER_LENG)
        }
        # [ doc = "6-bit character length." ]
        # [ inline ( always ) ]
        pub fn _6_bit_character_leng(self) -> &'a mut W {
            self.variant(WLSW::_6_BIT_CHARACTER_LENG)
        }
        # [ doc = "7-bit character length." ]
        # [ inline ( always ) ]
        pub fn _7_bit_character_leng(self) -> &'a mut W {
            self.variant(WLSW::_7_BIT_CHARACTER_LENG)
        }
        # [ doc = "8-bit character length." ]
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
        # [ doc = "2 stop bits (1.5 if LCR[1:0]=00)." ]
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
        # [ doc = "2 stop bits (1.5 if LCR[1:0]=00)." ]
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
        FORCED1STICK_PAR,
        # [ doc = "Forced 0 stick parity." ]
        FORCED0STICK_PAR,
    }
    impl PSW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PSW::ODD_PARITY_NUMBER_O => 0,
                PSW::EVEN_PARITY_NUMBER_ => 1,
                PSW::FORCED1STICK_PAR => 2,
                PSW::FORCED0STICK_PAR => 3,
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
        pub fn forced1stick_par(self) -> &'a mut W {
            self.variant(PSW::FORCED1STICK_PAR)
        }
        # [ doc = "Forced 0 stick parity." ]
        # [ inline ( always ) ]
        pub fn forced0stick_par(self) -> &'a mut W {
            self.variant(PSW::FORCED0STICK_PAR)
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
        # [ doc = "Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR[6] is active high." ]
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
        # [ doc = "Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR[6] is active high." ]
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
        # [ doc = "Bit 2 - Stop Bit Select." ]
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
        # [ doc = "Bits 4:5 - Parity Select." ]
        # [ inline ( always ) ]
        pub fn ps(&self) -> PSR {
            PSR::_from({
                           const MASK: u8 = 3;
                           const OFFSET: u8 = 4;
                           ((self.bits >> OFFSET) & MASK as u32) as u8
                       })
        }
        # [ doc = "Bit 6 - Break Control." ]
        # [ inline ( always ) ]
        pub fn bc(&self) -> BCR {
            BCR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 6;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 7 - Divisor Latch Access Bit (DLAB)" ]
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
        # [ doc = "Bit 2 - Stop Bit Select." ]
        # [ inline ( always ) ]
        pub fn sbs(&mut self) -> _SBSW {
            _SBSW { w: self }
        }
        # [ doc = "Bit 3 - Parity Enable." ]
        # [ inline ( always ) ]
        pub fn pe(&mut self) -> _PEW {
            _PEW { w: self }
        }
        # [ doc = "Bits 4:5 - Parity Select." ]
        # [ inline ( always ) ]
        pub fn ps(&mut self) -> _PSW {
            _PSW { w: self }
        }
        # [ doc = "Bit 6 - Break Control." ]
        # [ inline ( always ) ]
        pub fn bc(&mut self) -> _BCW {
            _BCW { w: self }
        }
        # [ doc = "Bit 7 - Divisor Latch Access Bit (DLAB)" ]
        # [ inline ( always ) ]
        pub fn dlab(&mut self) -> _DLABW {
            _DLABW { w: self }
        }
    }
}
# [ doc = "Modem Control Register. Contains controls for flow control handshaking and loopback mode." ]
pub struct MCR {
    register: VolatileCell<u32>,
}
# [ doc = "Modem Control Register. Contains controls for flow control handshaking and loopback mode." ]
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
    # [ doc = r" Value of the field" ]
    pub struct DTRCTRLR {
        bits: bool,
    }
    impl DTRCTRLR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RTSCTRLR {
        bits: bool,
    }
    impl RTSCTRLR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Possible values of the field `LMS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum LMSR {
        # [ doc = "Disable modem loopback mode." ]
        DISABLE_MODEM_LOOPBA,
        # [ doc = "Enable modem loopback mode." ]
        ENABLE_MODEM_LOOPBAC,
    }
    impl LMSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                LMSR::DISABLE_MODEM_LOOPBA => false,
                LMSR::ENABLE_MODEM_LOOPBAC => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> LMSR {
            match value {
                false => LMSR::DISABLE_MODEM_LOOPBA,
                true => LMSR::ENABLE_MODEM_LOOPBAC,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_MODEM_LOOPBA`" ]
        # [ inline ( always ) ]
        pub fn is_disable_modem_loopba(&self) -> bool {
            *self == LMSR::DISABLE_MODEM_LOOPBA
        }
        # [ doc = "Checks if the value of the field is `ENABLE_MODEM_LOOPBAC`" ]
        # [ inline ( always ) ]
        pub fn is_enable_modem_loopbac(&self) -> bool {
            *self == LMSR::ENABLE_MODEM_LOOPBAC
        }
    }
    # [ doc = "Possible values of the field `RTSEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RTSENR {
        # [ doc = "Disable auto-rts flow control." ]
        DISABLE_AUTO_RTS_FLO,
        # [ doc = "Enable auto-rts flow control." ]
        ENABLE_AUTO_RTS_FLOW,
    }
    impl RTSENR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RTSENR::DISABLE_AUTO_RTS_FLO => false,
                RTSENR::ENABLE_AUTO_RTS_FLOW => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RTSENR {
            match value {
                false => RTSENR::DISABLE_AUTO_RTS_FLO,
                true => RTSENR::ENABLE_AUTO_RTS_FLOW,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_AUTO_RTS_FLO`" ]
        # [ inline ( always ) ]
        pub fn is_disable_auto_rts_flo(&self) -> bool {
            *self == RTSENR::DISABLE_AUTO_RTS_FLO
        }
        # [ doc = "Checks if the value of the field is `ENABLE_AUTO_RTS_FLOW`" ]
        # [ inline ( always ) ]
        pub fn is_enable_auto_rts_flow(&self) -> bool {
            *self == RTSENR::ENABLE_AUTO_RTS_FLOW
        }
    }
    # [ doc = "Possible values of the field `CTSEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CTSENR {
        # [ doc = "Disable auto-cts flow control." ]
        DISABLE_AUTO_CTS_FLO,
        # [ doc = "Enable auto-cts flow control." ]
        ENABLE_AUTO_CTS_FLOW,
    }
    impl CTSENR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CTSENR::DISABLE_AUTO_CTS_FLO => false,
                CTSENR::ENABLE_AUTO_CTS_FLOW => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CTSENR {
            match value {
                false => CTSENR::DISABLE_AUTO_CTS_FLO,
                true => CTSENR::ENABLE_AUTO_CTS_FLOW,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE_AUTO_CTS_FLO`" ]
        # [ inline ( always ) ]
        pub fn is_disable_auto_cts_flo(&self) -> bool {
            *self == CTSENR::DISABLE_AUTO_CTS_FLO
        }
        # [ doc = "Checks if the value of the field is `ENABLE_AUTO_CTS_FLOW`" ]
        # [ inline ( always ) ]
        pub fn is_enable_auto_cts_flow(&self) -> bool {
            *self == CTSENR::ENABLE_AUTO_CTS_FLOW
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DTRCTRLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTRCTRLW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RTSCTRLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RTSCTRLW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    # [ doc = "Values that can be written to the field `LMS`" ]
    pub enum LMSW {
        # [ doc = "Disable modem loopback mode." ]
        DISABLE_MODEM_LOOPBA,
        # [ doc = "Enable modem loopback mode." ]
        ENABLE_MODEM_LOOPBAC,
    }
    impl LMSW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                LMSW::DISABLE_MODEM_LOOPBA => false,
                LMSW::ENABLE_MODEM_LOOPBAC => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LMSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LMSW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: LMSW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable modem loopback mode." ]
        # [ inline ( always ) ]
        pub fn disable_modem_loopba(self) -> &'a mut W {
            self.variant(LMSW::DISABLE_MODEM_LOOPBA)
        }
        # [ doc = "Enable modem loopback mode." ]
        # [ inline ( always ) ]
        pub fn enable_modem_loopbac(self) -> &'a mut W {
            self.variant(LMSW::ENABLE_MODEM_LOOPBAC)
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
    # [ doc = "Values that can be written to the field `RTSEN`" ]
    pub enum RTSENW {
        # [ doc = "Disable auto-rts flow control." ]
        DISABLE_AUTO_RTS_FLO,
        # [ doc = "Enable auto-rts flow control." ]
        ENABLE_AUTO_RTS_FLOW,
    }
    impl RTSENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                RTSENW::DISABLE_AUTO_RTS_FLO => false,
                RTSENW::ENABLE_AUTO_RTS_FLOW => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RTSENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RTSENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RTSENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable auto-rts flow control." ]
        # [ inline ( always ) ]
        pub fn disable_auto_rts_flo(self) -> &'a mut W {
            self.variant(RTSENW::DISABLE_AUTO_RTS_FLO)
        }
        # [ doc = "Enable auto-rts flow control." ]
        # [ inline ( always ) ]
        pub fn enable_auto_rts_flow(self) -> &'a mut W {
            self.variant(RTSENW::ENABLE_AUTO_RTS_FLOW)
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
    # [ doc = "Values that can be written to the field `CTSEN`" ]
    pub enum CTSENW {
        # [ doc = "Disable auto-cts flow control." ]
        DISABLE_AUTO_CTS_FLO,
        # [ doc = "Enable auto-cts flow control." ]
        ENABLE_AUTO_CTS_FLOW,
    }
    impl CTSENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CTSENW::DISABLE_AUTO_CTS_FLO => false,
                CTSENW::ENABLE_AUTO_CTS_FLOW => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CTSENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CTSENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CTSENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disable auto-cts flow control." ]
        # [ inline ( always ) ]
        pub fn disable_auto_cts_flo(self) -> &'a mut W {
            self.variant(CTSENW::DISABLE_AUTO_CTS_FLO)
        }
        # [ doc = "Enable auto-cts flow control." ]
        # [ inline ( always ) ]
        pub fn enable_auto_cts_flow(self) -> &'a mut W {
            self.variant(CTSENW::ENABLE_AUTO_CTS_FLOW)
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
        # [ doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active." ]
        # [ inline ( always ) ]
        pub fn dtrctrl(&self) -> DTRCTRLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTRCTRLR { bits }
        }
        # [ doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active." ]
        # [ inline ( always ) ]
        pub fn rtsctrl(&self) -> RTSCTRLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RTSCTRLR { bits }
        }
        # [ doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR." ]
        # [ inline ( always ) ]
        pub fn lms(&self) -> LMSR {
            LMSR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 4;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 6 - RTS enable." ]
        # [ inline ( always ) ]
        pub fn rtsen(&self) -> RTSENR {
            RTSENR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 6;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 7 - CTS enable." ]
        # [ inline ( always ) ]
        pub fn ctsen(&self) -> CTSENR {
            CTSENR::_from({
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
        # [ doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active." ]
        # [ inline ( always ) ]
        pub fn dtrctrl(&mut self) -> _DTRCTRLW {
            _DTRCTRLW { w: self }
        }
        # [ doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active." ]
        # [ inline ( always ) ]
        pub fn rtsctrl(&mut self) -> _RTSCTRLW {
            _RTSCTRLW { w: self }
        }
        # [ doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR." ]
        # [ inline ( always ) ]
        pub fn lms(&mut self) -> _LMSW {
            _LMSW { w: self }
        }
        # [ doc = "Bit 6 - RTS enable." ]
        # [ inline ( always ) ]
        pub fn rtsen(&mut self) -> _RTSENW {
            _RTSENW { w: self }
        }
        # [ doc = "Bit 7 - CTS enable." ]
        # [ inline ( always ) ]
        pub fn ctsen(&mut self) -> _CTSENW {
            _CTSENW { w: self }
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
        # [ doc = "The UART1 receiver FIFO is empty." ]
        EMPTY,
        # [ doc = "The UART1 receiver FIFO is not empty." ]
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
        # [ doc = "THR contains valid data." ]
        VALID,
        # [ doc = "THR is empty." ]
        THR_IS_EMPTY_,
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
                THRER::VALID => false,
                THRER::THR_IS_EMPTY_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> THRER {
            match value {
                false => THRER::VALID,
                true => THRER::THR_IS_EMPTY_,
            }
        }
        # [ doc = "Checks if the value of the field is `VALID`" ]
        # [ inline ( always ) ]
        pub fn is_valid(&self) -> bool {
            *self == THRER::VALID
        }
        # [ doc = "Checks if the value of the field is `THR_IS_EMPTY_`" ]
        # [ inline ( always ) ]
        pub fn is_thr_is_empty_(&self) -> bool {
            *self == THRER::THR_IS_EMPTY_
        }
    }
    # [ doc = "Possible values of the field `TEMT`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TEMTR {
        # [ doc = "THR and/or the TSR contains valid data." ]
        VALID,
        # [ doc = "THR and the TSR are empty." ]
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
                TEMTR::VALID => false,
                TEMTR::EMPTY => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TEMTR {
            match value {
                false => TEMTR::VALID,
                true => TEMTR::EMPTY,
            }
        }
        # [ doc = "Checks if the value of the field is `VALID`" ]
        # [ inline ( always ) ]
        pub fn is_valid(&self) -> bool {
            *self == TEMTR::VALID
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
        # [ doc = "RBR contains no UART1 RX errors or FCR[0]=0." ]
        NOERROR,
        # [ doc = "UART1 RBR contains at least one UART1 RX error." ]
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
        # [ doc = "Bit 0 - Receiver Data Ready. LSR[0] is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty." ]
        # [ inline ( always ) ]
        pub fn rdr(&self) -> RDRR {
            RDRR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 0;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR[1]. LSR[1] is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost." ]
        # [ inline ( always ) ]
        pub fn oe(&self) -> OER {
            OER::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 1;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR[2]. Time of parity error detection is dependent on FCR[0]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO." ]
        # [ inline ( always ) ]
        pub fn pe(&self) -> PER {
            PER::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 2;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR[3]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO." ]
        # [ inline ( always ) ]
        pub fn fe(&self) -> FER {
            FER::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 3;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 4 - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR[0]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO." ]
        # [ inline ( always ) ]
        pub fn bi(&self) -> BIR {
            BIR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 4;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write." ]
        # [ inline ( always ) ]
        pub fn thre(&self) -> THRER {
            THRER::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 5;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 6 - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data." ]
        # [ inline ( always ) ]
        pub fn temt(&self) -> TEMTR {
            TEMTR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 6;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 7 - Error in RX FIFO. LSR[7] is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO." ]
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
# [ doc = "Modem Status Register. Contains handshake signal status flags." ]
pub struct MSR {
    register: VolatileCell<u32>,
}
# [ doc = "Modem Status Register. Contains handshake signal status flags." ]
pub mod msr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::MSR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = "Possible values of the field `DCTS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DCTSR {
        # [ doc = "No change detected on modem input, CTS." ]
        NO_CHANGE_DETECTED_O,
        # [ doc = "State change detected on modem input, CTS." ]
        STATE_CHANGE_DETECTE,
    }
    impl DCTSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DCTSR::NO_CHANGE_DETECTED_O => false,
                DCTSR::STATE_CHANGE_DETECTE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DCTSR {
            match value {
                false => DCTSR::NO_CHANGE_DETECTED_O,
                true => DCTSR::STATE_CHANGE_DETECTE,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`" ]
        # [ inline ( always ) ]
        pub fn is_no_change_detected_o(&self) -> bool {
            *self == DCTSR::NO_CHANGE_DETECTED_O
        }
        # [ doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`" ]
        # [ inline ( always ) ]
        pub fn is_state_change_detecte(&self) -> bool {
            *self == DCTSR::STATE_CHANGE_DETECTE
        }
    }
    # [ doc = "Possible values of the field `DDSR`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DDSRR {
        # [ doc = "No change detected on modem input, DSR." ]
        NO_CHANGE_DETECTED_O,
        # [ doc = "State change detected on modem input, DSR." ]
        STATE_CHANGE_DETECTE,
    }
    impl DDSRR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DDSRR::NO_CHANGE_DETECTED_O => false,
                DDSRR::STATE_CHANGE_DETECTE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DDSRR {
            match value {
                false => DDSRR::NO_CHANGE_DETECTED_O,
                true => DDSRR::STATE_CHANGE_DETECTE,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`" ]
        # [ inline ( always ) ]
        pub fn is_no_change_detected_o(&self) -> bool {
            *self == DDSRR::NO_CHANGE_DETECTED_O
        }
        # [ doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`" ]
        # [ inline ( always ) ]
        pub fn is_state_change_detecte(&self) -> bool {
            *self == DDSRR::STATE_CHANGE_DETECTE
        }
    }
    # [ doc = "Possible values of the field `TERI`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TERIR {
        # [ doc = "No change detected on modem input, RI." ]
        NO_CHANGE_DETECTED_O,
        # [ doc = "Low-to-high transition detected on RI." ]
        LOW_TO_HIGH_TRANSITI,
    }
    impl TERIR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TERIR::NO_CHANGE_DETECTED_O => false,
                TERIR::LOW_TO_HIGH_TRANSITI => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TERIR {
            match value {
                false => TERIR::NO_CHANGE_DETECTED_O,
                true => TERIR::LOW_TO_HIGH_TRANSITI,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`" ]
        # [ inline ( always ) ]
        pub fn is_no_change_detected_o(&self) -> bool {
            *self == TERIR::NO_CHANGE_DETECTED_O
        }
        # [ doc = "Checks if the value of the field is `LOW_TO_HIGH_TRANSITI`" ]
        # [ inline ( always ) ]
        pub fn is_low_to_high_transiti(&self) -> bool {
            *self == TERIR::LOW_TO_HIGH_TRANSITI
        }
    }
    # [ doc = "Possible values of the field `DDCD`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DDCDR {
        # [ doc = "No change detected on modem input, DCD." ]
        NO_CHANGE_DETECTED_O,
        # [ doc = "State change detected on modem input, DCD." ]
        STATE_CHANGE_DETECTE,
    }
    impl DDCDR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DDCDR::NO_CHANGE_DETECTED_O => false,
                DDCDR::STATE_CHANGE_DETECTE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DDCDR {
            match value {
                false => DDCDR::NO_CHANGE_DETECTED_O,
                true => DDCDR::STATE_CHANGE_DETECTE,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`" ]
        # [ inline ( always ) ]
        pub fn is_no_change_detected_o(&self) -> bool {
            *self == DDCDR::NO_CHANGE_DETECTED_O
        }
        # [ doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`" ]
        # [ inline ( always ) ]
        pub fn is_state_change_detecte(&self) -> bool {
            *self == DDCDR::STATE_CHANGE_DETECTE
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CTSR {
        bits: bool,
    }
    impl CTSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DSRR {
        bits: bool,
    }
    impl DSRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RIR {
        bits: bool,
    }
    impl RIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DCDR {
        bits: bool,
    }
    impl DCDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read." ]
        # [ inline ( always ) ]
        pub fn dcts(&self) -> DCTSR {
            DCTSR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 0;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 1 - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read." ]
        # [ inline ( always ) ]
        pub fn ddsr(&self) -> DDSRR {
            DDSRR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 1;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 2 - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read." ]
        # [ inline ( always ) ]
        pub fn teri(&self) -> TERIR {
            TERIR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 2;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 3 - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read." ]
        # [ inline ( always ) ]
        pub fn ddcd(&self) -> DDCDR {
            DDCDR::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 3;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 4 - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR[1] in modem loopback mode." ]
        # [ inline ( always ) ]
        pub fn cts(&self) -> CTSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTSR { bits }
        }
        # [ doc = "Bit 5 - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR[0] in modem loopback mode." ]
        # [ inline ( always ) ]
        pub fn dsr(&self) -> DSRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DSRR { bits }
        }
        # [ doc = "Bit 6 - Ring Indicator State. Complement of input RI. This bit is connected to MCR[2] in modem loopback mode." ]
        # [ inline ( always ) ]
        pub fn ri(&self) -> RIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RIR { bits }
        }
        # [ doc = "Bit 7 - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR[3] in modem loopback mode." ]
        # [ inline ( always ) ]
        pub fn dcd(&self) -> DCDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DCDR { bits }
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
        STOP,
        # [ doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion." ]
        START,
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
                STARTR::STOP => false,
                STARTR::START => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> STARTR {
            match value {
                false => STARTR::STOP,
                true => STARTR::START,
            }
        }
        # [ doc = "Checks if the value of the field is `STOP`" ]
        # [ inline ( always ) ]
        pub fn is_stop(&self) -> bool {
            *self == STARTR::STOP
        }
        # [ doc = "Checks if the value of the field is `START`" ]
        # [ inline ( always ) ]
        pub fn is_start(&self) -> bool {
            *self == STARTR::START
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
        # [ doc = "No restart" ]
        NO_RESTART,
        # [ doc = "Restart in case of time-out (counter restarts at next UART1 Rx falling edge)" ]
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
                AUTORESTARTR::NO_RESTART => false,
                AUTORESTARTR::RESTART_IN_CASE_OF_T => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> AUTORESTARTR {
            match value {
                false => AUTORESTARTR::NO_RESTART,
                true => AUTORESTARTR::RESTART_IN_CASE_OF_T,
            }
        }
        # [ doc = "Checks if the value of the field is `NO_RESTART`" ]
        # [ inline ( always ) ]
        pub fn is_no_restart(&self) -> bool {
            *self == AUTORESTARTR::NO_RESTART
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
        # [ doc = "Writing a 0 has no impact." ]
        WRITING_A_0_HAS_NO_I,
        # [ doc = "Writing a 1 will clear the corresponding interrupt in the IIR." ]
        WRITING_A_1_WILL_CLE,
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
                ABEOINTCLRR::WRITING_A_0_HAS_NO_I => false,
                ABEOINTCLRR::WRITING_A_1_WILL_CLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABEOINTCLRR {
            match value {
                false => ABEOINTCLRR::WRITING_A_0_HAS_NO_I,
                true => ABEOINTCLRR::WRITING_A_1_WILL_CLE,
            }
        }
        # [ doc = "Checks if the value of the field is `WRITING_A_0_HAS_NO_I`" ]
        # [ inline ( always ) ]
        pub fn is_writing_a_0_has_no_i(&self) -> bool {
            *self == ABEOINTCLRR::WRITING_A_0_HAS_NO_I
        }
        # [ doc = "Checks if the value of the field is `WRITING_A_1_WILL_CLE`" ]
        # [ inline ( always ) ]
        pub fn is_writing_a_1_will_cle(&self) -> bool {
            *self == ABEOINTCLRR::WRITING_A_1_WILL_CLE
        }
    }
    # [ doc = "Possible values of the field `ABTOINTCLR`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ABTOINTCLRR {
        # [ doc = "Writing a 0 has no impact." ]
        WRITING_A_0_HAS_NO_I,
        # [ doc = "Writing a 1 will clear the corresponding interrupt in the IIR." ]
        WRITING_A_1_WILL_CLE,
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
                ABTOINTCLRR::WRITING_A_0_HAS_NO_I => false,
                ABTOINTCLRR::WRITING_A_1_WILL_CLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABTOINTCLRR {
            match value {
                false => ABTOINTCLRR::WRITING_A_0_HAS_NO_I,
                true => ABTOINTCLRR::WRITING_A_1_WILL_CLE,
            }
        }
        # [ doc = "Checks if the value of the field is `WRITING_A_0_HAS_NO_I`" ]
        # [ inline ( always ) ]
        pub fn is_writing_a_0_has_no_i(&self) -> bool {
            *self == ABTOINTCLRR::WRITING_A_0_HAS_NO_I
        }
        # [ doc = "Checks if the value of the field is `WRITING_A_1_WILL_CLE`" ]
        # [ inline ( always ) ]
        pub fn is_writing_a_1_will_cle(&self) -> bool {
            *self == ABTOINTCLRR::WRITING_A_1_WILL_CLE
        }
    }
    # [ doc = "Values that can be written to the field `START`" ]
    pub enum STARTW {
        # [ doc = "Auto-baud stop (auto-baud is not running)." ]
        STOP,
        # [ doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion." ]
        START,
    }
    impl STARTW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                STARTW::STOP => false,
                STARTW::START => true,
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
        pub fn stop(self) -> &'a mut W {
            self.variant(STARTW::STOP)
        }
        # [ doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion." ]
        # [ inline ( always ) ]
        pub fn start(self) -> &'a mut W {
            self.variant(STARTW::START)
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
        # [ doc = "No restart" ]
        NO_RESTART,
        # [ doc = "Restart in case of time-out (counter restarts at next UART1 Rx falling edge)" ]
        RESTART_IN_CASE_OF_T,
    }
    impl AUTORESTARTW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                AUTORESTARTW::NO_RESTART => false,
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
        # [ doc = "No restart" ]
        # [ inline ( always ) ]
        pub fn no_restart(self) -> &'a mut W {
            self.variant(AUTORESTARTW::NO_RESTART)
        }
        # [ doc = "Restart in case of time-out (counter restarts at next UART1 Rx falling edge)" ]
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
        # [ doc = "Writing a 0 has no impact." ]
        WRITING_A_0_HAS_NO_I,
        # [ doc = "Writing a 1 will clear the corresponding interrupt in the IIR." ]
        WRITING_A_1_WILL_CLE,
    }
    impl ABEOINTCLRW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ABEOINTCLRW::WRITING_A_0_HAS_NO_I => false,
                ABEOINTCLRW::WRITING_A_1_WILL_CLE => true,
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
        # [ doc = "Writing a 0 has no impact." ]
        # [ inline ( always ) ]
        pub fn writing_a_0_has_no_i(self) -> &'a mut W {
            self.variant(ABEOINTCLRW::WRITING_A_0_HAS_NO_I)
        }
        # [ doc = "Writing a 1 will clear the corresponding interrupt in the IIR." ]
        # [ inline ( always ) ]
        pub fn writing_a_1_will_cle(self) -> &'a mut W {
            self.variant(ABEOINTCLRW::WRITING_A_1_WILL_CLE)
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
        # [ doc = "Writing a 0 has no impact." ]
        WRITING_A_0_HAS_NO_I,
        # [ doc = "Writing a 1 will clear the corresponding interrupt in the IIR." ]
        WRITING_A_1_WILL_CLE,
    }
    impl ABTOINTCLRW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ABTOINTCLRW::WRITING_A_0_HAS_NO_I => false,
                ABTOINTCLRW::WRITING_A_1_WILL_CLE => true,
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
        # [ doc = "Writing a 0 has no impact." ]
        # [ inline ( always ) ]
        pub fn writing_a_0_has_no_i(self) -> &'a mut W {
            self.variant(ABTOINTCLRW::WRITING_A_0_HAS_NO_I)
        }
        # [ doc = "Writing a 1 will clear the corresponding interrupt in the IIR." ]
        # [ inline ( always ) ]
        pub fn writing_a_1_will_cle(self) -> &'a mut W {
            self.variant(ABTOINTCLRW::WRITING_A_1_WILL_CLE)
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
        # [ doc = "Bit 0 - Auto-baud start bit. This bit is automatically cleared after auto-baud completion." ]
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
        # [ doc = "Bit 2 - Auto-baud restart bit." ]
        # [ inline ( always ) ]
        pub fn autorestart(&self) -> AUTORESTARTR {
            AUTORESTARTR::_from({
                                    const MASK: bool = true;
                                    const OFFSET: u8 = 2;
                                    ((self.bits >> OFFSET) & MASK as u32) != 0
                                })
        }
        # [ doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only)." ]
        # [ inline ( always ) ]
        pub fn abeointclr(&self) -> ABEOINTCLRR {
            ABEOINTCLRR::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 8;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only)." ]
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
        # [ doc = "Bit 0 - Auto-baud start bit. This bit is automatically cleared after auto-baud completion." ]
        # [ inline ( always ) ]
        pub fn start(&mut self) -> _STARTW {
            _STARTW { w: self }
        }
        # [ doc = "Bit 1 - Auto-baud mode select bit." ]
        # [ inline ( always ) ]
        pub fn mode(&mut self) -> _MODEW {
            _MODEW { w: self }
        }
        # [ doc = "Bit 2 - Auto-baud restart bit." ]
        # [ inline ( always ) ]
        pub fn autorestart(&mut self) -> _AUTORESTARTW {
            _AUTORESTARTW { w: self }
        }
        # [ doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only)." ]
        # [ inline ( always ) ]
        pub fn abeointclr(&mut self) -> _ABEOINTCLRW {
            _ABEOINTCLRW { w: self }
        }
        # [ doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only)." ]
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
        # [ doc = "Bits 0:3 - Baud rate generation pre-scaler divisor value. If this field is 0, fractional baud rate generator will not impact the UART1 baud rate." ]
        # [ inline ( always ) ]
        pub fn divaddval(&self) -> DIVADDVALR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DIVADDVALR { bits }
        }
        # [ doc = "Bits 4:7 - Baud rate pre-scaler multiplier value. This field must be greater or equal 1 for UART1 to operate properly, regardless of whether the fractional baud rate generator is used or not." ]
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
        # [ doc = "Bits 0:3 - Baud rate generation pre-scaler divisor value. If this field is 0, fractional baud rate generator will not impact the UART1 baud rate." ]
        # [ inline ( always ) ]
        pub fn divaddval(&mut self) -> _DIVADDVALW {
            _DIVADDVALW { w: self }
        }
        # [ doc = "Bits 4:7 - Baud rate pre-scaler multiplier value. This field must be greater or equal 1 for UART1 to operate properly, regardless of whether the fractional baud rate generator is used or not." ]
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
        # [ doc = "Bit 7 - When this bit is 1, as it is after a Reset, data written to the THR is output on the TXD pin as soon as any preceding data has been sent. If this bit cleared to 0 while a character is being sent, the transmission of that character is completed, but no further characters are sent until this bit is set again. In other words, a 0 in this bit blocks the transfer of characters from the THR or TX FIFO into the transmit shift register. Software can clear this bit when it detects that the a hardware-handshaking TX-permit signal (CTS) has gone false, or with software handshaking, when it receives an XOFF character (DC3). Software can set this bit again when it detects that the TX-permit signal has gone true, or when it receives an XON (DC1) character." ]
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
        # [ doc = "Bit 7 - When this bit is 1, as it is after a Reset, data written to the THR is output on the TXD pin as soon as any preceding data has been sent. If this bit cleared to 0 while a character is being sent, the transmission of that character is completed, but no further characters are sent until this bit is set again. In other words, a 0 in this bit blocks the transfer of characters from the THR or TX FIFO into the transmit shift register. Software can clear this bit when it detects that the a hardware-handshaking TX-permit signal (CTS) has gone false, or with software handshaking, when it receives an XOFF character (DC3). Software can set this bit again when it detects that the TX-permit signal has gone true, or when it receives an XON (DC1) character." ]
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
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt." ]
        ENABLED_IN_THIS_MOD,
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
                NMMENR::DISABLED_ => false,
                NMMENR::ENABLED_IN_THIS_MOD => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> NMMENR {
            match value {
                false => NMMENR::DISABLED_,
                true => NMMENR::ENABLED_IN_THIS_MOD,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == NMMENR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `ENABLED_IN_THIS_MOD`" ]
        # [ inline ( always ) ]
        pub fn is_enabled_in_this_mod(&self) -> bool {
            *self == NMMENR::ENABLED_IN_THIS_MOD
        }
    }
    # [ doc = "Possible values of the field `RXDIS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RXDISR {
        # [ doc = "Enabled." ]
        ENABLED_,
        # [ doc = "Disabled." ]
        DISABLED_,
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
                RXDISR::ENABLED_ => false,
                RXDISR::DISABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RXDISR {
            match value {
                false => RXDISR::ENABLED_,
                true => RXDISR::DISABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_enabled_(&self) -> bool {
            *self == RXDISR::ENABLED_
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == RXDISR::DISABLED_
        }
    }
    # [ doc = "Possible values of the field `AADEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum AADENR {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Enabled." ]
        ENABLED_,
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
                AADENR::DISABLED_ => false,
                AADENR::ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> AADENR {
            match value {
                false => AADENR::DISABLED_,
                true => AADENR::ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_(&self) -> bool {
            *self == AADENR::DISABLED_
        }
        # [ doc = "Checks if the value of the field is `ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_enabled_(&self) -> bool {
            *self == AADENR::ENABLED_
        }
    }
    # [ doc = "Possible values of the field `SEL`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SELR {
        # [ doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control." ]
        RTS_IF_DIRECTION_CO,
        # [ doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control." ]
        DTR_IF_DIRECTION_CO,
    }
    impl SELR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                SELR::RTS_IF_DIRECTION_CO => false,
                SELR::DTR_IF_DIRECTION_CO => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SELR {
            match value {
                false => SELR::RTS_IF_DIRECTION_CO,
                true => SELR::DTR_IF_DIRECTION_CO,
            }
        }
        # [ doc = "Checks if the value of the field is `RTS_IF_DIRECTION_CO`" ]
        # [ inline ( always ) ]
        pub fn is_rts_if_direction_co(&self) -> bool {
            *self == SELR::RTS_IF_DIRECTION_CO
        }
        # [ doc = "Checks if the value of the field is `DTR_IF_DIRECTION_CO`" ]
        # [ inline ( always ) ]
        pub fn is_dtr_if_direction_co(&self) -> bool {
            *self == SELR::DTR_IF_DIRECTION_CO
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
        # [ doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted." ]
        LOW_THE_DIRECTION_C,
        # [ doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted." ]
        HIGH_THE_DIRECTION_,
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
                OINVR::LOW_THE_DIRECTION_C => false,
                OINVR::HIGH_THE_DIRECTION_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> OINVR {
            match value {
                false => OINVR::LOW_THE_DIRECTION_C,
                true => OINVR::HIGH_THE_DIRECTION_,
            }
        }
        # [ doc = "Checks if the value of the field is `LOW_THE_DIRECTION_C`" ]
        # [ inline ( always ) ]
        pub fn is_low_the_direction_c(&self) -> bool {
            *self == OINVR::LOW_THE_DIRECTION_C
        }
        # [ doc = "Checks if the value of the field is `HIGH_THE_DIRECTION_`" ]
        # [ inline ( always ) ]
        pub fn is_high_the_direction_(&self) -> bool {
            *self == OINVR::HIGH_THE_DIRECTION_
        }
    }
    # [ doc = "Values that can be written to the field `NMMEN`" ]
    pub enum NMMENW {
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt." ]
        ENABLED_IN_THIS_MOD,
    }
    impl NMMENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                NMMENW::DISABLED_ => false,
                NMMENW::ENABLED_IN_THIS_MOD => true,
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
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(NMMENW::DISABLED_)
        }
        # [ doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enabled_in_this_mod(self) -> &'a mut W {
            self.variant(NMMENW::ENABLED_IN_THIS_MOD)
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
        # [ doc = "Enabled." ]
        ENABLED_,
        # [ doc = "Disabled." ]
        DISABLED_,
    }
    impl RXDISW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                RXDISW::ENABLED_ => false,
                RXDISW::DISABLED_ => true,
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
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled_(self) -> &'a mut W {
            self.variant(RXDISW::ENABLED_)
        }
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(RXDISW::DISABLED_)
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
        # [ doc = "Disabled." ]
        DISABLED_,
        # [ doc = "Enabled." ]
        ENABLED_,
    }
    impl AADENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                AADENW::DISABLED_ => false,
                AADENW::ENABLED_ => true,
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
        # [ doc = "Disabled." ]
        # [ inline ( always ) ]
        pub fn disabled_(self) -> &'a mut W {
            self.variant(AADENW::DISABLED_)
        }
        # [ doc = "Enabled." ]
        # [ inline ( always ) ]
        pub fn enabled_(self) -> &'a mut W {
            self.variant(AADENW::ENABLED_)
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
    # [ doc = "Values that can be written to the field `SEL`" ]
    pub enum SELW {
        # [ doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control." ]
        RTS_IF_DIRECTION_CO,
        # [ doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control." ]
        DTR_IF_DIRECTION_CO,
    }
    impl SELW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SELW::RTS_IF_DIRECTION_CO => false,
                SELW::DTR_IF_DIRECTION_CO => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SELW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SELW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control." ]
        # [ inline ( always ) ]
        pub fn rts_if_direction_co(self) -> &'a mut W {
            self.variant(SELW::RTS_IF_DIRECTION_CO)
        }
        # [ doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control." ]
        # [ inline ( always ) ]
        pub fn dtr_if_direction_co(self) -> &'a mut W {
            self.variant(SELW::DTR_IF_DIRECTION_CO)
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
        # [ doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted." ]
        LOW_THE_DIRECTION_C,
        # [ doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted." ]
        HIGH_THE_DIRECTION_,
    }
    impl OINVW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                OINVW::LOW_THE_DIRECTION_C => false,
                OINVW::HIGH_THE_DIRECTION_ => true,
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
        # [ doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted." ]
        # [ inline ( always ) ]
        pub fn low_the_direction_c(self) -> &'a mut W {
            self.variant(OINVW::LOW_THE_DIRECTION_C)
        }
        # [ doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted." ]
        # [ inline ( always ) ]
        pub fn high_the_direction_(self) -> &'a mut W {
            self.variant(OINVW::HIGH_THE_DIRECTION_)
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
        # [ doc = "Bit 0 - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select." ]
        # [ inline ( always ) ]
        pub fn nmmen(&self) -> NMMENR {
            NMMENR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 1 - Receive enable." ]
        # [ inline ( always ) ]
        pub fn rxdis(&self) -> RXDISR {
            RXDISR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 1;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 2 - Auto Address Detect (AAD) enable." ]
        # [ inline ( always ) ]
        pub fn aaden(&self) -> AADENR {
            AADENR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 2;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 3 - Direction control." ]
        # [ inline ( always ) ]
        pub fn sel(&self) -> SELR {
            SELR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 3;
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
        # [ doc = "Bit 5 - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin." ]
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
        # [ doc = "Bit 0 - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select." ]
        # [ inline ( always ) ]
        pub fn nmmen(&mut self) -> _NMMENW {
            _NMMENW { w: self }
        }
        # [ doc = "Bit 1 - Receive enable." ]
        # [ inline ( always ) ]
        pub fn rxdis(&mut self) -> _RXDISW {
            _RXDISW { w: self }
        }
        # [ doc = "Bit 2 - Auto Address Detect (AAD) enable." ]
        # [ inline ( always ) ]
        pub fn aaden(&mut self) -> _AADENW {
            _AADENW { w: self }
        }
        # [ doc = "Bit 3 - Direction control." ]
        # [ inline ( always ) ]
        pub fn sel(&mut self) -> _SELW {
            _SELW { w: self }
        }
        # [ doc = "Bit 4 - Direction control enable." ]
        # [ inline ( always ) ]
        pub fn dctrl(&mut self) -> _DCTRLW {
            _DCTRLW { w: self }
        }
        # [ doc = "Bit 5 - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin." ]
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
        # [ doc = "Bits 0:7 - Contains the direction control (RTS or DTR) delay value. This register works in conjunction with an 8-bit counter." ]
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
        # [ doc = "Bits 0:7 - Contains the direction control (RTS or DTR) delay value. This register works in conjunction with an 8-bit counter." ]
        # [ inline ( always ) ]
        pub fn dly(&mut self) -> _DLYW {
            _DLYW { w: self }
        }
    }
}
# [ doc = "UART1" ]
pub struct UART1 {
    register_block: RegisterBlock,
}
impl Deref for UART1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
