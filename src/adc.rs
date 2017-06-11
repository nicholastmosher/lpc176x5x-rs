# ! [ doc = "Analog-to-Digital Converter (ADC)" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "Analog-to-Digital Converter (ADC)" ]
pub const ADC: Peripheral<ADC> = unsafe { Peripheral::new(1073954816) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur." ]
    pub adcr: ADCR,
    # [ doc = "0x04 - A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion." ]
    pub gdr: GDR,
    _reserved0: [u8; 4usize],
    # [ doc = "0x0c - A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt." ]
    pub inten: INTEN,
    # [ doc = "0x10 - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
    pub dr0: DR,
    # [ doc = "0x14 - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
    pub dr1: DR,
    # [ doc = "0x18 - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
    pub dr2: DR,
    # [ doc = "0x1c - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
    pub dr3: DR,
    # [ doc = "0x20 - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
    pub dr4: DR,
    # [ doc = "0x24 - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
    pub dr5: DR,
    # [ doc = "0x28 - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
    pub dr6: DR,
    # [ doc = "0x2c - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
    pub dr7: DR,
    # [ doc = "0x30 - A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag." ]
    pub stat: STAT,
    # [ doc = "0x34 - ADC trim register." ]
    pub trm: TRM,
}
# [ doc = "A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur." ]
pub struct ADCR {
    register: VolatileCell<u32>,
}
# [ doc = "A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur." ]
pub mod adcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::ADCR {
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
    pub struct SELR {
        bits: u8,
    }
    impl SELR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CLKDIVR {
        bits: u8,
    }
    impl CLKDIVR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Possible values of the field `BURST`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum BURSTR {
        # [ doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start." ]
        BURST,
        # [ doc = "Conversions are software controlled and require 31 clocks." ]
        SW,
    }
    impl BURSTR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                BURSTR::BURST => true,
                BURSTR::SW => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> BURSTR {
            match value {
                true => BURSTR::BURST,
                false => BURSTR::SW,
            }
        }
        # [ doc = "Checks if the value of the field is `BURST`" ]
        # [ inline ( always ) ]
        pub fn is_burst(&self) -> bool {
            *self == BURSTR::BURST
        }
        # [ doc = "Checks if the value of the field is `SW`" ]
        # [ inline ( always ) ]
        pub fn is_sw(&self) -> bool {
            *self == BURSTR::SW
        }
    }
    # [ doc = "Possible values of the field `PDN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PDNR {
        # [ doc = "The A/D converter is operational." ]
        POWERED,
        # [ doc = "The A/D converter is in power-down mode." ]
        POWERDOWN,
    }
    impl PDNR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                PDNR::POWERED => true,
                PDNR::POWERDOWN => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> PDNR {
            match value {
                true => PDNR::POWERED,
                false => PDNR::POWERDOWN,
            }
        }
        # [ doc = "Checks if the value of the field is `POWERED`" ]
        # [ inline ( always ) ]
        pub fn is_powered(&self) -> bool {
            *self == PDNR::POWERED
        }
        # [ doc = "Checks if the value of the field is `POWERDOWN`" ]
        # [ inline ( always ) ]
        pub fn is_powerdown(&self) -> bool {
            *self == PDNR::POWERDOWN
        }
    }
    # [ doc = "Possible values of the field `START`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum STARTR {
        # [ doc = "No start (this value should be used when clearing PDN to 0)." ]
        NO_START_THIS_VALUE,
        # [ doc = "Start conversion now." ]
        START_CONVERSION_NOW,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on the P2[10] pin." ]
        P2_10,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on the P1[27] pin." ]
        P1_27,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin." ]
        MAT0_1,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin." ]
        MAT0_3,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin." ]
        MAT1_0,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin." ]
        MAT1_1,
    }
    impl STARTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                STARTR::NO_START_THIS_VALUE => 0,
                STARTR::START_CONVERSION_NOW => 1,
                STARTR::P2_10 => 2,
                STARTR::P1_27 => 3,
                STARTR::MAT0_1 => 4,
                STARTR::MAT0_3 => 5,
                STARTR::MAT1_0 => 6,
                STARTR::MAT1_1 => 7,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> STARTR {
            match value {
                0 => STARTR::NO_START_THIS_VALUE,
                1 => STARTR::START_CONVERSION_NOW,
                2 => STARTR::P2_10,
                3 => STARTR::P1_27,
                4 => STARTR::MAT0_1,
                5 => STARTR::MAT0_3,
                6 => STARTR::MAT1_0,
                7 => STARTR::MAT1_1,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `NO_START_THIS_VALUE`" ]
        # [ inline ( always ) ]
        pub fn is_no_start_this_value(&self) -> bool {
            *self == STARTR::NO_START_THIS_VALUE
        }
        # [ doc = "Checks if the value of the field is `START_CONVERSION_NOW`" ]
        # [ inline ( always ) ]
        pub fn is_start_conversion_now(&self) -> bool {
            *self == STARTR::START_CONVERSION_NOW
        }
        # [ doc = "Checks if the value of the field is `P2_10`" ]
        # [ inline ( always ) ]
        pub fn is_p2_10(&self) -> bool {
            *self == STARTR::P2_10
        }
        # [ doc = "Checks if the value of the field is `P1_27`" ]
        # [ inline ( always ) ]
        pub fn is_p1_27(&self) -> bool {
            *self == STARTR::P1_27
        }
        # [ doc = "Checks if the value of the field is `MAT0_1`" ]
        # [ inline ( always ) ]
        pub fn is_mat0_1(&self) -> bool {
            *self == STARTR::MAT0_1
        }
        # [ doc = "Checks if the value of the field is `MAT0_3`" ]
        # [ inline ( always ) ]
        pub fn is_mat0_3(&self) -> bool {
            *self == STARTR::MAT0_3
        }
        # [ doc = "Checks if the value of the field is `MAT1_0`" ]
        # [ inline ( always ) ]
        pub fn is_mat1_0(&self) -> bool {
            *self == STARTR::MAT1_0
        }
        # [ doc = "Checks if the value of the field is `MAT1_1`" ]
        # [ inline ( always ) ]
        pub fn is_mat1_1(&self) -> bool {
            *self == STARTR::MAT1_1
        }
    }
    # [ doc = "Possible values of the field `EDGE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EDGER {
        # [ doc = "Start conversion on a falling edge on the selected CAP/MAT signal." ]
        FALLLING,
        # [ doc = "Start conversion on a rising edge on the selected CAP/MAT signal." ]
        RISING,
    }
    impl EDGER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                EDGER::FALLLING => true,
                EDGER::RISING => false,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> EDGER {
            match value {
                true => EDGER::FALLLING,
                false => EDGER::RISING,
            }
        }
        # [ doc = "Checks if the value of the field is `FALLLING`" ]
        # [ inline ( always ) ]
        pub fn is_fallling(&self) -> bool {
            *self == EDGER::FALLLING
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == EDGER::RISING
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SELW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _CLKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKDIVW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `BURST`" ]
    pub enum BURSTW {
        # [ doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start." ]
        BURST,
        # [ doc = "Conversions are software controlled and require 31 clocks." ]
        SW,
    }
    impl BURSTW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                BURSTW::BURST => true,
                BURSTW::SW => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BURSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BURSTW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: BURSTW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start." ]
        # [ inline ( always ) ]
        pub fn burst(self) -> &'a mut W {
            self.variant(BURSTW::BURST)
        }
        # [ doc = "Conversions are software controlled and require 31 clocks." ]
        # [ inline ( always ) ]
        pub fn sw(self) -> &'a mut W {
            self.variant(BURSTW::SW)
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
    # [ doc = "Values that can be written to the field `PDN`" ]
    pub enum PDNW {
        # [ doc = "The A/D converter is operational." ]
        POWERED,
        # [ doc = "The A/D converter is in power-down mode." ]
        POWERDOWN,
    }
    impl PDNW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                PDNW::POWERED => true,
                PDNW::POWERDOWN => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PDNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PDNW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: PDNW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The A/D converter is operational." ]
        # [ inline ( always ) ]
        pub fn powered(self) -> &'a mut W {
            self.variant(PDNW::POWERED)
        }
        # [ doc = "The A/D converter is in power-down mode." ]
        # [ inline ( always ) ]
        pub fn powerdown(self) -> &'a mut W {
            self.variant(PDNW::POWERDOWN)
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
    # [ doc = "Values that can be written to the field `START`" ]
    pub enum STARTW {
        # [ doc = "No start (this value should be used when clearing PDN to 0)." ]
        NO_START_THIS_VALUE,
        # [ doc = "Start conversion now." ]
        START_CONVERSION_NOW,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on the P2[10] pin." ]
        P2_10,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on the P1[27] pin." ]
        P1_27,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin." ]
        MAT0_1,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin." ]
        MAT0_3,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin." ]
        MAT1_0,
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin." ]
        MAT1_1,
    }
    impl STARTW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                STARTW::NO_START_THIS_VALUE => 0,
                STARTW::START_CONVERSION_NOW => 1,
                STARTW::P2_10 => 2,
                STARTW::P1_27 => 3,
                STARTW::MAT0_1 => 4,
                STARTW::MAT0_3 => 5,
                STARTW::MAT1_0 => 6,
                STARTW::MAT1_1 => 7,
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
                self.bits(variant._bits())
            }
        }
        # [ doc = "No start (this value should be used when clearing PDN to 0)." ]
        # [ inline ( always ) ]
        pub fn no_start_this_value(self) -> &'a mut W {
            self.variant(STARTW::NO_START_THIS_VALUE)
        }
        # [ doc = "Start conversion now." ]
        # [ inline ( always ) ]
        pub fn start_conversion_now(self) -> &'a mut W {
            self.variant(STARTW::START_CONVERSION_NOW)
        }
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on the P2[10] pin." ]
        # [ inline ( always ) ]
        pub fn p2_10(self) -> &'a mut W {
            self.variant(STARTW::P2_10)
        }
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on the P1[27] pin." ]
        # [ inline ( always ) ]
        pub fn p1_27(self) -> &'a mut W {
            self.variant(STARTW::P1_27)
        }
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin." ]
        # [ inline ( always ) ]
        pub fn mat0_1(self) -> &'a mut W {
            self.variant(STARTW::MAT0_1)
        }
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin." ]
        # [ inline ( always ) ]
        pub fn mat0_3(self) -> &'a mut W {
            self.variant(STARTW::MAT0_3)
        }
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin." ]
        # [ inline ( always ) ]
        pub fn mat1_0(self) -> &'a mut W {
            self.variant(STARTW::MAT1_0)
        }
        # [ doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin." ]
        # [ inline ( always ) ]
        pub fn mat1_1(self) -> &'a mut W {
            self.variant(STARTW::MAT1_1)
        }
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `EDGE`" ]
    pub enum EDGEW {
        # [ doc = "Start conversion on a falling edge on the selected CAP/MAT signal." ]
        FALLLING,
        # [ doc = "Start conversion on a rising edge on the selected CAP/MAT signal." ]
        RISING,
    }
    impl EDGEW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                EDGEW::FALLLING => true,
                EDGEW::RISING => false,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EDGEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EDGEW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: EDGEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Start conversion on a falling edge on the selected CAP/MAT signal." ]
        # [ inline ( always ) ]
        pub fn fallling(self) -> &'a mut W {
            self.variant(EDGEW::FALLLING)
        }
        # [ doc = "Start conversion on a rising edge on the selected CAP/MAT signal." ]
        # [ inline ( always ) ]
        pub fn rising(self) -> &'a mut W {
            self.variant(EDGEW::RISING)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:7 - Selects which of the AD0[7:0] pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0[0], and bit 7 selects pin AD0[7]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01." ]
        # [ inline ( always ) ]
        pub fn sel(&self) -> SELR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SELR { bits }
        }
        # [ doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable." ]
        # [ inline ( always ) ]
        pub fn clkdiv(&self) -> CLKDIVR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CLKDIVR { bits }
        }
        # [ doc = "Bit 16 - Burst mode" ]
        # [ inline ( always ) ]
        pub fn burst(&self) -> BURSTR {
            BURSTR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 16;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 21 - Power down mode" ]
        # [ inline ( always ) ]
        pub fn pdn(&self) -> PDNR {
            PDNR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 21;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:" ]
        # [ inline ( always ) ]
        pub fn start(&self) -> STARTR {
            STARTR::_from({
                              const MASK: u8 = 7;
                              const OFFSET: u8 = 24;
                              ((self.bits >> OFFSET) & MASK as u32) as u8
                          })
        }
        # [ doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:" ]
        # [ inline ( always ) ]
        pub fn edge(&self) -> EDGER {
            EDGER::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 27;
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
        # [ doc = "Bits 0:7 - Selects which of the AD0[7:0] pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0[0], and bit 7 selects pin AD0[7]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01." ]
        # [ inline ( always ) ]
        pub fn sel(&mut self) -> _SELW {
            _SELW { w: self }
        }
        # [ doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable." ]
        # [ inline ( always ) ]
        pub fn clkdiv(&mut self) -> _CLKDIVW {
            _CLKDIVW { w: self }
        }
        # [ doc = "Bit 16 - Burst mode" ]
        # [ inline ( always ) ]
        pub fn burst(&mut self) -> _BURSTW {
            _BURSTW { w: self }
        }
        # [ doc = "Bit 21 - Power down mode" ]
        # [ inline ( always ) ]
        pub fn pdn(&mut self) -> _PDNW {
            _PDNW { w: self }
        }
        # [ doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:" ]
        # [ inline ( always ) ]
        pub fn start(&mut self) -> _STARTW {
            _STARTW { w: self }
        }
        # [ doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:" ]
        # [ inline ( always ) ]
        pub fn edge(&mut self) -> _EDGEW {
            _EDGEW { w: self }
        }
    }
}
# [ doc = "A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion." ]
pub struct GDR {
    register: VolatileCell<u32>,
}
# [ doc = "A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion." ]
pub mod gdr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::GDR {
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
    pub struct RESULTR {
        bits: u16,
    }
    impl RESULTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CHNR {
        bits: u8,
    }
    impl CHNR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUNR {
        bits: bool,
    }
    impl OVERRUNR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONER {
        bits: bool,
    }
    impl DONER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _RESULTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESULTW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CHNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CHNW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OVERRUNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVERRUNW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DONEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DONEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0[n] pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP." ]
        # [ inline ( always ) ]
        pub fn result(&self) -> RESULTR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            RESULTR { bits }
        }
        # [ doc = "Bits 24:26 - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)." ]
        # [ inline ( always ) ]
        pub fn chn(&self) -> CHNR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CHNR { bits }
        }
        # [ doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register." ]
        # [ inline ( always ) ]
        pub fn overrun(&self) -> OVERRUNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUNR { bits }
        }
        # [ doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started." ]
        # [ inline ( always ) ]
        pub fn done(&self) -> DONER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONER { bits }
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
        # [ doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0[n] pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP." ]
        # [ inline ( always ) ]
        pub fn result(&mut self) -> _RESULTW {
            _RESULTW { w: self }
        }
        # [ doc = "Bits 24:26 - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)." ]
        # [ inline ( always ) ]
        pub fn chn(&mut self) -> _CHNW {
            _CHNW { w: self }
        }
        # [ doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register." ]
        # [ inline ( always ) ]
        pub fn overrun(&mut self) -> _OVERRUNW {
            _OVERRUNW { w: self }
        }
        # [ doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started." ]
        # [ inline ( always ) ]
        pub fn done(&mut self) -> _DONEW {
            _DONEW { w: self }
        }
    }
}
# [ doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt." ]
pub struct INTEN {
    register: VolatileCell<u32>,
}
# [ doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt." ]
pub mod inten {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::INTEN {
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
    # [ doc = "Possible values of the field `ADINTEN0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ADINTEN0R {
        # [ doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 0 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ADINTEN0R::DISABLE => false,
                ADINTEN0R::ENABLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ADINTEN0R {
            match value {
                false => ADINTEN0R::DISABLE,
                true => ADINTEN0R::ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == ADINTEN0R::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == ADINTEN0R::ENABLE
        }
    }
    # [ doc = "Possible values of the field `ADINTEN1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ADINTEN1R {
        # [ doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 1 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ADINTEN1R::DISABLE => false,
                ADINTEN1R::ENABLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ADINTEN1R {
            match value {
                false => ADINTEN1R::DISABLE,
                true => ADINTEN1R::ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == ADINTEN1R::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == ADINTEN1R::ENABLE
        }
    }
    # [ doc = "Possible values of the field `ADINTEN2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ADINTEN2R {
        # [ doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 2 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ADINTEN2R::DISABLE => false,
                ADINTEN2R::ENABLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ADINTEN2R {
            match value {
                false => ADINTEN2R::DISABLE,
                true => ADINTEN2R::ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == ADINTEN2R::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == ADINTEN2R::ENABLE
        }
    }
    # [ doc = "Possible values of the field `ADINTEN3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ADINTEN3R {
        # [ doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 3 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN3R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ADINTEN3R::DISABLE => false,
                ADINTEN3R::ENABLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ADINTEN3R {
            match value {
                false => ADINTEN3R::DISABLE,
                true => ADINTEN3R::ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == ADINTEN3R::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == ADINTEN3R::ENABLE
        }
    }
    # [ doc = "Possible values of the field `ADINTEN4`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ADINTEN4R {
        # [ doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 4 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN4R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ADINTEN4R::DISABLE => false,
                ADINTEN4R::ENABLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ADINTEN4R {
            match value {
                false => ADINTEN4R::DISABLE,
                true => ADINTEN4R::ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == ADINTEN4R::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == ADINTEN4R::ENABLE
        }
    }
    # [ doc = "Possible values of the field `ADINTEN5`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ADINTEN5R {
        # [ doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 5 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN5R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ADINTEN5R::DISABLE => false,
                ADINTEN5R::ENABLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ADINTEN5R {
            match value {
                false => ADINTEN5R::DISABLE,
                true => ADINTEN5R::ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == ADINTEN5R::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == ADINTEN5R::ENABLE
        }
    }
    # [ doc = "Possible values of the field `ADINTEN6`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ADINTEN6R {
        # [ doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 6 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN6R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ADINTEN6R::DISABLE => false,
                ADINTEN6R::ENABLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ADINTEN6R {
            match value {
                false => ADINTEN6R::DISABLE,
                true => ADINTEN6R::ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == ADINTEN6R::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == ADINTEN6R::ENABLE
        }
    }
    # [ doc = "Possible values of the field `ADINTEN7`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ADINTEN7R {
        # [ doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 7 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN7R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ADINTEN7R::DISABLE => false,
                ADINTEN7R::ENABLE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ADINTEN7R {
            match value {
                false => ADINTEN7R::DISABLE,
                true => ADINTEN7R::ENABLE,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLE`" ]
        # [ inline ( always ) ]
        pub fn is_disable(&self) -> bool {
            *self == ADINTEN7R::DISABLE
        }
        # [ doc = "Checks if the value of the field is `ENABLE`" ]
        # [ inline ( always ) ]
        pub fn is_enable(&self) -> bool {
            *self == ADINTEN7R::ENABLE
        }
    }
    # [ doc = "Possible values of the field `ADGINTEN`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ADGINTENR {
        # [ doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts." ]
        CHANNELS,
        # [ doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts." ]
        GLOBAL,
    }
    impl ADGINTENR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ADGINTENR::CHANNELS => false,
                ADGINTENR::GLOBAL => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ADGINTENR {
            match value {
                false => ADGINTENR::CHANNELS,
                true => ADGINTENR::GLOBAL,
            }
        }
        # [ doc = "Checks if the value of the field is `CHANNELS`" ]
        # [ inline ( always ) ]
        pub fn is_channels(&self) -> bool {
            *self == ADGINTENR::CHANNELS
        }
        # [ doc = "Checks if the value of the field is `GLOBAL`" ]
        # [ inline ( always ) ]
        pub fn is_global(&self) -> bool {
            *self == ADGINTENR::GLOBAL
        }
    }
    # [ doc = "Values that can be written to the field `ADINTEN0`" ]
    pub enum ADINTEN0W {
        # [ doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 0 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ADINTEN0W::DISABLE => false,
                ADINTEN0W::ENABLE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADINTEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADINTEN0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADINTEN0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(ADINTEN0W::DISABLE)
        }
        # [ doc = "Completion of a conversion on ADC channel 0 will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(ADINTEN0W::ENABLE)
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
    # [ doc = "Values that can be written to the field `ADINTEN1`" ]
    pub enum ADINTEN1W {
        # [ doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 1 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ADINTEN1W::DISABLE => false,
                ADINTEN1W::ENABLE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADINTEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADINTEN1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADINTEN1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(ADINTEN1W::DISABLE)
        }
        # [ doc = "Completion of a conversion on ADC channel 1 will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(ADINTEN1W::ENABLE)
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
    # [ doc = "Values that can be written to the field `ADINTEN2`" ]
    pub enum ADINTEN2W {
        # [ doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 2 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ADINTEN2W::DISABLE => false,
                ADINTEN2W::ENABLE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADINTEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADINTEN2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADINTEN2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(ADINTEN2W::DISABLE)
        }
        # [ doc = "Completion of a conversion on ADC channel 2 will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(ADINTEN2W::ENABLE)
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
    # [ doc = "Values that can be written to the field `ADINTEN3`" ]
    pub enum ADINTEN3W {
        # [ doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 3 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ADINTEN3W::DISABLE => false,
                ADINTEN3W::ENABLE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADINTEN3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADINTEN3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADINTEN3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(ADINTEN3W::DISABLE)
        }
        # [ doc = "Completion of a conversion on ADC channel 3 will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(ADINTEN3W::ENABLE)
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
    # [ doc = "Values that can be written to the field `ADINTEN4`" ]
    pub enum ADINTEN4W {
        # [ doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 4 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN4W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ADINTEN4W::DISABLE => false,
                ADINTEN4W::ENABLE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADINTEN4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADINTEN4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADINTEN4W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(ADINTEN4W::DISABLE)
        }
        # [ doc = "Completion of a conversion on ADC channel 4 will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(ADINTEN4W::ENABLE)
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
    # [ doc = "Values that can be written to the field `ADINTEN5`" ]
    pub enum ADINTEN5W {
        # [ doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 5 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN5W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ADINTEN5W::DISABLE => false,
                ADINTEN5W::ENABLE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADINTEN5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADINTEN5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADINTEN5W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(ADINTEN5W::DISABLE)
        }
        # [ doc = "Completion of a conversion on ADC channel 5 will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(ADINTEN5W::ENABLE)
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
    # [ doc = "Values that can be written to the field `ADINTEN6`" ]
    pub enum ADINTEN6W {
        # [ doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 6 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN6W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ADINTEN6W::DISABLE => false,
                ADINTEN6W::ENABLE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADINTEN6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADINTEN6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADINTEN6W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(ADINTEN6W::DISABLE)
        }
        # [ doc = "Completion of a conversion on ADC channel 6 will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(ADINTEN6W::ENABLE)
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
    # [ doc = "Values that can be written to the field `ADINTEN7`" ]
    pub enum ADINTEN7W {
        # [ doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt." ]
        DISABLE,
        # [ doc = "Completion of a conversion on ADC channel 7 will generate an interrupt." ]
        ENABLE,
    }
    impl ADINTEN7W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ADINTEN7W::DISABLE => false,
                ADINTEN7W::ENABLE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADINTEN7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADINTEN7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADINTEN7W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn disable(self) -> &'a mut W {
            self.variant(ADINTEN7W::DISABLE)
        }
        # [ doc = "Completion of a conversion on ADC channel 7 will generate an interrupt." ]
        # [ inline ( always ) ]
        pub fn enable(self) -> &'a mut W {
            self.variant(ADINTEN7W::ENABLE)
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
    # [ doc = "Values that can be written to the field `ADGINTEN`" ]
    pub enum ADGINTENW {
        # [ doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts." ]
        CHANNELS,
        # [ doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts." ]
        GLOBAL,
    }
    impl ADGINTENW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ADGINTENW::CHANNELS => false,
                ADGINTENW::GLOBAL => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADGINTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADGINTENW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ADGINTENW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts." ]
        # [ inline ( always ) ]
        pub fn channels(self) -> &'a mut W {
            self.variant(ADGINTENW::CHANNELS)
        }
        # [ doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts." ]
        # [ inline ( always ) ]
        pub fn global(self) -> &'a mut W {
            self.variant(ADGINTENW::GLOBAL)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten0(&self) -> ADINTEN0R {
            ADINTEN0R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 0;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 1 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten1(&self) -> ADINTEN1R {
            ADINTEN1R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 1;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 2 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten2(&self) -> ADINTEN2R {
            ADINTEN2R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 2;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 3 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten3(&self) -> ADINTEN3R {
            ADINTEN3R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 3;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 4 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten4(&self) -> ADINTEN4R {
            ADINTEN4R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 4;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 5 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten5(&self) -> ADINTEN5R {
            ADINTEN5R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 5;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 6 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten6(&self) -> ADINTEN6R {
            ADINTEN6R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 6;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 7 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten7(&self) -> ADINTEN7R {
            ADINTEN7R::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 7;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
        # [ doc = "Bit 8 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adginten(&self) -> ADGINTENR {
            ADGINTENR::_from({
                                 const MASK: bool = true;
                                 const OFFSET: u8 = 8;
                                 ((self.bits >> OFFSET) & MASK as u32) != 0
                             })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 256 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten0(&mut self) -> _ADINTEN0W {
            _ADINTEN0W { w: self }
        }
        # [ doc = "Bit 1 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten1(&mut self) -> _ADINTEN1W {
            _ADINTEN1W { w: self }
        }
        # [ doc = "Bit 2 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten2(&mut self) -> _ADINTEN2W {
            _ADINTEN2W { w: self }
        }
        # [ doc = "Bit 3 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten3(&mut self) -> _ADINTEN3W {
            _ADINTEN3W { w: self }
        }
        # [ doc = "Bit 4 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten4(&mut self) -> _ADINTEN4W {
            _ADINTEN4W { w: self }
        }
        # [ doc = "Bit 5 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten5(&mut self) -> _ADINTEN5W {
            _ADINTEN5W { w: self }
        }
        # [ doc = "Bit 6 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten6(&mut self) -> _ADINTEN6W {
            _ADINTEN6W { w: self }
        }
        # [ doc = "Bit 7 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adinten7(&mut self) -> _ADINTEN7W {
            _ADINTEN7W { w: self }
        }
        # [ doc = "Bit 8 - Interrupt enable" ]
        # [ inline ( always ) ]
        pub fn adginten(&mut self) -> _ADGINTENW {
            _ADGINTENW { w: self }
        }
    }
}
# [ doc = "A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
pub struct DR {
    register: VolatileCell<u32>,
}
# [ doc = "A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0." ]
pub mod dr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::DR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RESULTR {
        bits: u16,
    }
    impl RESULTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUNR {
        bits: bool,
    }
    impl OVERRUNR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONER {
        bits: bool,
    }
    impl DONER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0[n] pin, as it falls within the range of VREFP to V SS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP." ]
        # [ inline ( always ) ]
        pub fn result(&self) -> RESULTR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            RESULTR { bits }
        }
        # [ doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits.This bit is cleared by reading this register." ]
        # [ inline ( always ) ]
        pub fn overrun(&self) -> OVERRUNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUNR { bits }
        }
        # [ doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read." ]
        # [ inline ( always ) ]
        pub fn done(&self) -> DONER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONER { bits }
        }
    }
}
# [ doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag." ]
pub struct STAT {
    register: VolatileCell<u32>,
}
# [ doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag." ]
pub mod stat {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::STAT {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONE0R {
        bits: bool,
    }
    impl DONE0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONE1R {
        bits: bool,
    }
    impl DONE1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONE2R {
        bits: bool,
    }
    impl DONE2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONE3R {
        bits: bool,
    }
    impl DONE3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONE4R {
        bits: bool,
    }
    impl DONE4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONE5R {
        bits: bool,
    }
    impl DONE5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONE6R {
        bits: bool,
    }
    impl DONE6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DONE7R {
        bits: bool,
    }
    impl DONE7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUN0R {
        bits: bool,
    }
    impl OVERRUN0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUN1R {
        bits: bool,
    }
    impl OVERRUN1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUN2R {
        bits: bool,
    }
    impl OVERRUN2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUN3R {
        bits: bool,
    }
    impl OVERRUN3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUN4R {
        bits: bool,
    }
    impl OVERRUN4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUN5R {
        bits: bool,
    }
    impl OVERRUN5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUN6R {
        bits: bool,
    }
    impl OVERRUN6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct OVERRUN7R {
        bits: bool,
    }
    impl OVERRUN7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ADINTR {
        bits: bool,
    }
    impl ADINTR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - This bit mirrors the DONE status flag from the result register for A/D channel 0." ]
        # [ inline ( always ) ]
        pub fn done0(&self) -> DONE0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONE0R { bits }
        }
        # [ doc = "Bit 1 - This bit mirrors the DONE status flag from the result register for A/D channel 1." ]
        # [ inline ( always ) ]
        pub fn done1(&self) -> DONE1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONE1R { bits }
        }
        # [ doc = "Bit 2 - This bit mirrors the DONE status flag from the result register for A/D channel 2." ]
        # [ inline ( always ) ]
        pub fn done2(&self) -> DONE2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONE2R { bits }
        }
        # [ doc = "Bit 3 - This bit mirrors the DONE status flag from the result register for A/D channel 3." ]
        # [ inline ( always ) ]
        pub fn done3(&self) -> DONE3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONE3R { bits }
        }
        # [ doc = "Bit 4 - This bit mirrors the DONE status flag from the result register for A/D channel 4." ]
        # [ inline ( always ) ]
        pub fn done4(&self) -> DONE4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONE4R { bits }
        }
        # [ doc = "Bit 5 - This bit mirrors the DONE status flag from the result register for A/D channel 5." ]
        # [ inline ( always ) ]
        pub fn done5(&self) -> DONE5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONE5R { bits }
        }
        # [ doc = "Bit 6 - This bit mirrors the DONE status flag from the result register for A/D channel 6." ]
        # [ inline ( always ) ]
        pub fn done6(&self) -> DONE6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONE6R { bits }
        }
        # [ doc = "Bit 7 - This bit mirrors the DONE status flag from the result register for A/D channel 7." ]
        # [ inline ( always ) ]
        pub fn done7(&self) -> DONE7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DONE7R { bits }
        }
        # [ doc = "Bit 8 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 0." ]
        # [ inline ( always ) ]
        pub fn overrun0(&self) -> OVERRUN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUN0R { bits }
        }
        # [ doc = "Bit 9 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 1." ]
        # [ inline ( always ) ]
        pub fn overrun1(&self) -> OVERRUN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUN1R { bits }
        }
        # [ doc = "Bit 10 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 2." ]
        # [ inline ( always ) ]
        pub fn overrun2(&self) -> OVERRUN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUN2R { bits }
        }
        # [ doc = "Bit 11 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 3." ]
        # [ inline ( always ) ]
        pub fn overrun3(&self) -> OVERRUN3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUN3R { bits }
        }
        # [ doc = "Bit 12 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 4." ]
        # [ inline ( always ) ]
        pub fn overrun4(&self) -> OVERRUN4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUN4R { bits }
        }
        # [ doc = "Bit 13 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 5." ]
        # [ inline ( always ) ]
        pub fn overrun5(&self) -> OVERRUN5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUN5R { bits }
        }
        # [ doc = "Bit 14 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 6." ]
        # [ inline ( always ) ]
        pub fn overrun6(&self) -> OVERRUN6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUN6R { bits }
        }
        # [ doc = "Bit 15 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 7." ]
        # [ inline ( always ) ]
        pub fn overrun7(&self) -> OVERRUN7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVERRUN7R { bits }
        }
        # [ doc = "Bit 16 - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register." ]
        # [ inline ( always ) ]
        pub fn adint(&self) -> ADINTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADINTR { bits }
        }
    }
}
# [ doc = "ADC trim register." ]
pub struct TRM {
    register: VolatileCell<u32>,
}
# [ doc = "ADC trim register." ]
pub mod trm {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TRM {
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
    pub struct ADCOFFSR {
        bits: u8,
    }
    impl ADCOFFSR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TRIMR {
        bits: u8,
    }
    impl TRIMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ADCOFFSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADCOFFSW<'a> {
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
    pub struct _TRIMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TRIMW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
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
        # [ doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user." ]
        # [ inline ( always ) ]
        pub fn adcoffs(&self) -> ADCOFFSR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ADCOFFSR { bits }
        }
        # [ doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write." ]
        # [ inline ( always ) ]
        pub fn trim(&self) -> TRIMR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TRIMR { bits }
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
        # [ doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user." ]
        # [ inline ( always ) ]
        pub fn adcoffs(&mut self) -> _ADCOFFSW {
            _ADCOFFSW { w: self }
        }
        # [ doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write." ]
        # [ inline ( always ) ]
        pub fn trim(&mut self) -> _TRIMW {
            _TRIMW { w: self }
        }
    }
}
# [ doc = "Analog-to-Digital Converter (ADC)" ]
pub struct ADC {
    register_block: RegisterBlock,
}
impl Deref for ADC {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
