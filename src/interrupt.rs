# ! [ doc = r" Interrupts" ]
use cortex_m::ctxt::Context;
use cortex_m::exception;
use cortex_m::interrupt::Nr;
# [ doc = "0 - WDT" ]
pub struct WDT {
    _0: (),
}
unsafe impl Context for WDT {}
unsafe impl Nr for WDT {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        0
    }
}
impl !Send for WDT {}
# [ doc = "1 - TIMER0" ]
pub struct TIMER0 {
    _0: (),
}
unsafe impl Context for TIMER0 {}
unsafe impl Nr for TIMER0 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        1
    }
}
impl !Send for TIMER0 {}
# [ doc = "2 - TIMER1" ]
pub struct TIMER1 {
    _0: (),
}
unsafe impl Context for TIMER1 {}
unsafe impl Nr for TIMER1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        2
    }
}
impl !Send for TIMER1 {}
# [ doc = "3 - TIMER2" ]
pub struct TIMER2 {
    _0: (),
}
unsafe impl Context for TIMER2 {}
unsafe impl Nr for TIMER2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        3
    }
}
impl !Send for TIMER2 {}
# [ doc = "4 - TIMER3" ]
pub struct TIMER3 {
    _0: (),
}
unsafe impl Context for TIMER3 {}
unsafe impl Nr for TIMER3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        4
    }
}
impl !Send for TIMER3 {}
# [ doc = "5 - UART0" ]
pub struct UART0 {
    _0: (),
}
unsafe impl Context for UART0 {}
unsafe impl Nr for UART0 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        5
    }
}
impl !Send for UART0 {}
# [ doc = "6 - UART1" ]
pub struct UART1 {
    _0: (),
}
unsafe impl Context for UART1 {}
unsafe impl Nr for UART1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        6
    }
}
impl !Send for UART1 {}
# [ doc = "7 - UART2" ]
pub struct UART2 {
    _0: (),
}
unsafe impl Context for UART2 {}
unsafe impl Nr for UART2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        7
    }
}
impl !Send for UART2 {}
# [ doc = "8 - UART3" ]
pub struct UART3 {
    _0: (),
}
unsafe impl Context for UART3 {}
unsafe impl Nr for UART3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        8
    }
}
impl !Send for UART3 {}
# [ doc = "9 - PWM1" ]
pub struct PWM1 {
    _0: (),
}
unsafe impl Context for PWM1 {}
unsafe impl Nr for PWM1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        9
    }
}
impl !Send for PWM1 {}
# [ doc = "10 - I2C0" ]
pub struct I2C0 {
    _0: (),
}
unsafe impl Context for I2C0 {}
unsafe impl Nr for I2C0 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        10
    }
}
impl !Send for I2C0 {}
# [ doc = "11 - I2C1" ]
pub struct I2C1 {
    _0: (),
}
unsafe impl Context for I2C1 {}
unsafe impl Nr for I2C1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        11
    }
}
impl !Send for I2C1 {}
# [ doc = "12 - I2C2" ]
pub struct I2C2 {
    _0: (),
}
unsafe impl Context for I2C2 {}
unsafe impl Nr for I2C2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        12
    }
}
impl !Send for I2C2 {}
# [ doc = "13 - SPI" ]
pub struct SPI {
    _0: (),
}
unsafe impl Context for SPI {}
unsafe impl Nr for SPI {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        13
    }
}
impl !Send for SPI {}
# [ doc = "14 - SSP0" ]
pub struct SSP0 {
    _0: (),
}
unsafe impl Context for SSP0 {}
unsafe impl Nr for SSP0 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        14
    }
}
impl !Send for SSP0 {}
# [ doc = "15 - SSP1" ]
pub struct SSP1 {
    _0: (),
}
unsafe impl Context for SSP1 {}
unsafe impl Nr for SSP1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        15
    }
}
impl !Send for SSP1 {}
# [ doc = "16 - PLL0" ]
pub struct PLL0 {
    _0: (),
}
unsafe impl Context for PLL0 {}
unsafe impl Nr for PLL0 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        16
    }
}
impl !Send for PLL0 {}
# [ doc = "17 - RTC" ]
pub struct RTC {
    _0: (),
}
unsafe impl Context for RTC {}
unsafe impl Nr for RTC {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        17
    }
}
impl !Send for RTC {}
# [ doc = "18 - EINT0" ]
pub struct EINT0 {
    _0: (),
}
unsafe impl Context for EINT0 {}
unsafe impl Nr for EINT0 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        18
    }
}
impl !Send for EINT0 {}
# [ doc = "19 - EINT1" ]
pub struct EINT1 {
    _0: (),
}
unsafe impl Context for EINT1 {}
unsafe impl Nr for EINT1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        19
    }
}
impl !Send for EINT1 {}
# [ doc = "20 - EINT2" ]
pub struct EINT2 {
    _0: (),
}
unsafe impl Context for EINT2 {}
unsafe impl Nr for EINT2 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        20
    }
}
impl !Send for EINT2 {}
# [ doc = "21 - EINT3" ]
pub struct EINT3 {
    _0: (),
}
unsafe impl Context for EINT3 {}
unsafe impl Nr for EINT3 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        21
    }
}
impl !Send for EINT3 {}
# [ doc = "22 - ADC" ]
pub struct ADC {
    _0: (),
}
unsafe impl Context for ADC {}
unsafe impl Nr for ADC {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        22
    }
}
impl !Send for ADC {}
# [ doc = "23 - BOD" ]
pub struct BOD {
    _0: (),
}
unsafe impl Context for BOD {}
unsafe impl Nr for BOD {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        23
    }
}
impl !Send for BOD {}
# [ doc = "24 - USB" ]
pub struct USB {
    _0: (),
}
unsafe impl Context for USB {}
unsafe impl Nr for USB {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        24
    }
}
impl !Send for USB {}
# [ doc = "25 - CAN" ]
pub struct CAN {
    _0: (),
}
unsafe impl Context for CAN {}
unsafe impl Nr for CAN {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        25
    }
}
impl !Send for CAN {}
# [ doc = "26 - DMA" ]
pub struct DMA {
    _0: (),
}
unsafe impl Context for DMA {}
unsafe impl Nr for DMA {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        26
    }
}
impl !Send for DMA {}
# [ doc = "27 - I2S" ]
pub struct I2S {
    _0: (),
}
unsafe impl Context for I2S {}
unsafe impl Nr for I2S {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        27
    }
}
impl !Send for I2S {}
# [ doc = "28 - ENET" ]
pub struct ENET {
    _0: (),
}
unsafe impl Context for ENET {}
unsafe impl Nr for ENET {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        28
    }
}
impl !Send for ENET {}
# [ doc = "29 - RIT" ]
pub struct RIT {
    _0: (),
}
unsafe impl Context for RIT {}
unsafe impl Nr for RIT {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        29
    }
}
impl !Send for RIT {}
# [ doc = "30 - MCPWM" ]
pub struct MCPWM {
    _0: (),
}
unsafe impl Context for MCPWM {}
unsafe impl Nr for MCPWM {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        30
    }
}
impl !Send for MCPWM {}
# [ doc = "31 - QEI" ]
pub struct QEI {
    _0: (),
}
unsafe impl Context for QEI {}
unsafe impl Nr for QEI {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        31
    }
}
impl !Send for QEI {}
# [ doc = "32 - PLL1" ]
pub struct PLL1 {
    _0: (),
}
unsafe impl Context for PLL1 {}
unsafe impl Nr for PLL1 {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        32
    }
}
impl !Send for PLL1 {}
# [ doc = "33 - USBActivity" ]
pub struct USBACTIVITY {
    _0: (),
}
unsafe impl Context for USBACTIVITY {}
unsafe impl Nr for USBACTIVITY {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        33
    }
}
impl !Send for USBACTIVITY {}
# [ doc = "34 - CANActivity" ]
pub struct CANACTIVITY {
    _0: (),
}
unsafe impl Context for CANACTIVITY {}
unsafe impl Nr for CANACTIVITY {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        34
    }
}
impl !Send for CANACTIVITY {}
# [ doc = r" Interrupt handlers" ]
# [ allow ( non_snake_case ) ]
# [ repr ( C ) ]
pub struct Handlers {
    # [ doc = "0 - WDT" ]
    pub WDT: extern "C" fn(WDT),
    # [ doc = "1 - TIMER0" ]
    pub TIMER0: extern "C" fn(TIMER0),
    # [ doc = "2 - TIMER1" ]
    pub TIMER1: extern "C" fn(TIMER1),
    # [ doc = "3 - TIMER2" ]
    pub TIMER2: extern "C" fn(TIMER2),
    # [ doc = "4 - TIMER3" ]
    pub TIMER3: extern "C" fn(TIMER3),
    # [ doc = "5 - UART0" ]
    pub UART0: extern "C" fn(UART0),
    # [ doc = "6 - UART1" ]
    pub UART1: extern "C" fn(UART1),
    # [ doc = "7 - UART2" ]
    pub UART2: extern "C" fn(UART2),
    # [ doc = "8 - UART3" ]
    pub UART3: extern "C" fn(UART3),
    # [ doc = "9 - PWM1" ]
    pub PWM1: extern "C" fn(PWM1),
    # [ doc = "10 - I2C0" ]
    pub I2C0: extern "C" fn(I2C0),
    # [ doc = "11 - I2C1" ]
    pub I2C1: extern "C" fn(I2C1),
    # [ doc = "12 - I2C2" ]
    pub I2C2: extern "C" fn(I2C2),
    # [ doc = "13 - SPI" ]
    pub SPI: extern "C" fn(SPI),
    # [ doc = "14 - SSP0" ]
    pub SSP0: extern "C" fn(SSP0),
    # [ doc = "15 - SSP1" ]
    pub SSP1: extern "C" fn(SSP1),
    # [ doc = "16 - PLL0" ]
    pub PLL0: extern "C" fn(PLL0),
    # [ doc = "17 - RTC" ]
    pub RTC: extern "C" fn(RTC),
    # [ doc = "18 - EINT0" ]
    pub EINT0: extern "C" fn(EINT0),
    # [ doc = "19 - EINT1" ]
    pub EINT1: extern "C" fn(EINT1),
    # [ doc = "20 - EINT2" ]
    pub EINT2: extern "C" fn(EINT2),
    # [ doc = "21 - EINT3" ]
    pub EINT3: extern "C" fn(EINT3),
    # [ doc = "22 - ADC" ]
    pub ADC: extern "C" fn(ADC),
    # [ doc = "23 - BOD" ]
    pub BOD: extern "C" fn(BOD),
    # [ doc = "24 - USB" ]
    pub USB: extern "C" fn(USB),
    # [ doc = "25 - CAN" ]
    pub CAN: extern "C" fn(CAN),
    # [ doc = "26 - DMA" ]
    pub DMA: extern "C" fn(DMA),
    # [ doc = "27 - I2S" ]
    pub I2S: extern "C" fn(I2S),
    # [ doc = "28 - ENET" ]
    pub ENET: extern "C" fn(ENET),
    # [ doc = "29 - RIT" ]
    pub RIT: extern "C" fn(RIT),
    # [ doc = "30 - MCPWM" ]
    pub MCPWM: extern "C" fn(MCPWM),
    # [ doc = "31 - QEI" ]
    pub QEI: extern "C" fn(QEI),
    # [ doc = "32 - PLL1" ]
    pub PLL1: extern "C" fn(PLL1),
    # [ doc = "33 - USBActivity" ]
    pub USBACTIVITY: extern "C" fn(USBACTIVITY),
    # [ doc = "34 - CANActivity" ]
    pub CANACTIVITY: extern "C" fn(CANACTIVITY),
}
# [ doc = r" Default interrupt handlers" ]
pub const DEFAULT_HANDLERS: Handlers = Handlers {
    WDT: exception::default_handler,
    TIMER0: exception::default_handler,
    TIMER1: exception::default_handler,
    TIMER2: exception::default_handler,
    TIMER3: exception::default_handler,
    UART0: exception::default_handler,
    UART1: exception::default_handler,
    UART2: exception::default_handler,
    UART3: exception::default_handler,
    PWM1: exception::default_handler,
    I2C0: exception::default_handler,
    I2C1: exception::default_handler,
    I2C2: exception::default_handler,
    SPI: exception::default_handler,
    SSP0: exception::default_handler,
    SSP1: exception::default_handler,
    PLL0: exception::default_handler,
    RTC: exception::default_handler,
    EINT0: exception::default_handler,
    EINT1: exception::default_handler,
    EINT2: exception::default_handler,
    EINT3: exception::default_handler,
    ADC: exception::default_handler,
    BOD: exception::default_handler,
    USB: exception::default_handler,
    CAN: exception::default_handler,
    DMA: exception::default_handler,
    I2S: exception::default_handler,
    ENET: exception::default_handler,
    RIT: exception::default_handler,
    MCPWM: exception::default_handler,
    QEI: exception::default_handler,
    PLL1: exception::default_handler,
    USBACTIVITY: exception::default_handler,
    CANACTIVITY: exception::default_handler,
};
# [ doc = r" Enumeration of all the interrupts" ]
pub enum Interrupt {
    # [ doc = "0 - WDT" ]
    WDT,
    # [ doc = "1 - TIMER0" ]
    TIMER0,
    # [ doc = "2 - TIMER1" ]
    TIMER1,
    # [ doc = "3 - TIMER2" ]
    TIMER2,
    # [ doc = "4 - TIMER3" ]
    TIMER3,
    # [ doc = "5 - UART0" ]
    UART0,
    # [ doc = "6 - UART1" ]
    UART1,
    # [ doc = "7 - UART2" ]
    UART2,
    # [ doc = "8 - UART3" ]
    UART3,
    # [ doc = "9 - PWM1" ]
    PWM1,
    # [ doc = "10 - I2C0" ]
    I2C0,
    # [ doc = "11 - I2C1" ]
    I2C1,
    # [ doc = "12 - I2C2" ]
    I2C2,
    # [ doc = "13 - SPI" ]
    SPI,
    # [ doc = "14 - SSP0" ]
    SSP0,
    # [ doc = "15 - SSP1" ]
    SSP1,
    # [ doc = "16 - PLL0" ]
    PLL0,
    # [ doc = "17 - RTC" ]
    RTC,
    # [ doc = "18 - EINT0" ]
    EINT0,
    # [ doc = "19 - EINT1" ]
    EINT1,
    # [ doc = "20 - EINT2" ]
    EINT2,
    # [ doc = "21 - EINT3" ]
    EINT3,
    # [ doc = "22 - ADC" ]
    ADC,
    # [ doc = "23 - BOD" ]
    BOD,
    # [ doc = "24 - USB" ]
    USB,
    # [ doc = "25 - CAN" ]
    CAN,
    # [ doc = "26 - DMA" ]
    DMA,
    # [ doc = "27 - I2S" ]
    I2S,
    # [ doc = "28 - ENET" ]
    ENET,
    # [ doc = "29 - RIT" ]
    RIT,
    # [ doc = "30 - MCPWM" ]
    MCPWM,
    # [ doc = "31 - QEI" ]
    QEI,
    # [ doc = "32 - PLL1" ]
    PLL1,
    # [ doc = "33 - USBActivity" ]
    USBACTIVITY,
    # [ doc = "34 - CANActivity" ]
    CANACTIVITY,
}
unsafe impl Nr for Interrupt {
    # [ inline ( always ) ]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WDT => 0,
            Interrupt::TIMER0 => 1,
            Interrupt::TIMER1 => 2,
            Interrupt::TIMER2 => 3,
            Interrupt::TIMER3 => 4,
            Interrupt::UART0 => 5,
            Interrupt::UART1 => 6,
            Interrupt::UART2 => 7,
            Interrupt::UART3 => 8,
            Interrupt::PWM1 => 9,
            Interrupt::I2C0 => 10,
            Interrupt::I2C1 => 11,
            Interrupt::I2C2 => 12,
            Interrupt::SPI => 13,
            Interrupt::SSP0 => 14,
            Interrupt::SSP1 => 15,
            Interrupt::PLL0 => 16,
            Interrupt::RTC => 17,
            Interrupt::EINT0 => 18,
            Interrupt::EINT1 => 19,
            Interrupt::EINT2 => 20,
            Interrupt::EINT3 => 21,
            Interrupt::ADC => 22,
            Interrupt::BOD => 23,
            Interrupt::USB => 24,
            Interrupt::CAN => 25,
            Interrupt::DMA => 26,
            Interrupt::I2S => 27,
            Interrupt::ENET => 28,
            Interrupt::RIT => 29,
            Interrupt::MCPWM => 30,
            Interrupt::QEI => 31,
            Interrupt::PLL1 => 32,
            Interrupt::USBACTIVITY => 33,
            Interrupt::CANACTIVITY => 34,
        }
    }
}
