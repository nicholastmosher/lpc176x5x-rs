# ! [ doc = "Motor Control PWM" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "Motor Control PWM" ]
pub const MCPWM: Peripheral<MCPWM> = unsafe { Peripheral::new(1074495488) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - PWM Control read address" ]
    pub con: CON,
    # [ doc = "0x04 - PWM Control set address" ]
    pub con_set: CON_SET,
    # [ doc = "0x08 - PWM Control clear address" ]
    pub con_clr: CON_CLR,
    # [ doc = "0x0c - Capture Control read address" ]
    pub capcon: CAPCON,
    # [ doc = "0x10 - Capture Control set address" ]
    pub capcon_set: CAPCON_SET,
    # [ doc = "0x14 - Event Control clear address" ]
    pub capcon_clr: CAPCON_CLR,
    # [ doc = "0x18 - Timer Counter register" ]
    pub tc0: TC,
    # [ doc = "0x1c - Timer Counter register" ]
    pub tc1: TC,
    # [ doc = "0x20 - Timer Counter register" ]
    pub tc2: TC,
    # [ doc = "0x24 - Limit register" ]
    pub lim0: LIM,
    # [ doc = "0x28 - Limit register" ]
    pub lim1: LIM,
    # [ doc = "0x2c - Limit register" ]
    pub lim2: LIM,
    # [ doc = "0x30 - Match register" ]
    pub mat0: MAT,
    # [ doc = "0x34 - Match register" ]
    pub mat1: MAT,
    # [ doc = "0x38 - Match register" ]
    pub mat2: MAT,
    # [ doc = "0x3c - Dead time register" ]
    pub dt: DT,
    # [ doc = "0x40 - Communication Pattern register" ]
    pub cp: CP,
    # [ doc = "0x44 - Capture register" ]
    pub cap0: CAP,
    # [ doc = "0x48 - Capture register" ]
    pub cap1: CAP,
    # [ doc = "0x4c - Capture register" ]
    pub cap2: CAP,
    # [ doc = "0x50 - Interrupt Enable read address" ]
    pub inten: INTEN,
    # [ doc = "0x54 - Interrupt Enable set address" ]
    pub inten_set: INTEN_SET,
    # [ doc = "0x58 - Interrupt Enable clear address" ]
    pub inten_clr: INTEN_CLR,
    # [ doc = "0x5c - Count Control read address" ]
    pub cntcon: CNTCON,
    # [ doc = "0x60 - Count Control set address" ]
    pub cntcon_set: CNTCON_SET,
    # [ doc = "0x64 - Count Control clear address" ]
    pub cntcon_clr: CNTCON_CLR,
    # [ doc = "0x68 - Interrupt flags read address" ]
    pub intf: INTF,
    # [ doc = "0x6c - Interrupt flags set address" ]
    pub intf_set: INTF_SET,
    # [ doc = "0x70 - Interrupt flags clear address" ]
    pub intf_clr: INTF_CLR,
    # [ doc = "0x74 - Capture clear address" ]
    pub cap_clr: CAP_CLR,
}
# [ doc = "PWM Control read address" ]
pub struct CON {
    register: VolatileCell<u32>,
}
# [ doc = "PWM Control read address" ]
pub mod con {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::CON {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = "Possible values of the field `RUN0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RUN0R {
        # [ doc = "Stop." ]
        STOP_,
        # [ doc = "Run." ]
        RUN_,
    }
    impl RUN0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RUN0R::STOP_ => false,
                RUN0R::RUN_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RUN0R {
            match value {
                false => RUN0R::STOP_,
                true => RUN0R::RUN_,
            }
        }
        # [ doc = "Checks if the value of the field is `STOP_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_(&self) -> bool {
            *self == RUN0R::STOP_
        }
        # [ doc = "Checks if the value of the field is `RUN_`" ]
        # [ inline ( always ) ]
        pub fn is_run_(&self) -> bool {
            *self == RUN0R::RUN_
        }
    }
    # [ doc = "Possible values of the field `CENTER0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CENTER0R {
        # [ doc = "Edge-aligned." ]
        EDGE_ALIGNED_,
        # [ doc = "Center-aligned." ]
        CENTER_ALIGNED_,
    }
    impl CENTER0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CENTER0R::EDGE_ALIGNED_ => false,
                CENTER0R::CENTER_ALIGNED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CENTER0R {
            match value {
                false => CENTER0R::EDGE_ALIGNED_,
                true => CENTER0R::CENTER_ALIGNED_,
            }
        }
        # [ doc = "Checks if the value of the field is `EDGE_ALIGNED_`" ]
        # [ inline ( always ) ]
        pub fn is_edge_aligned_(&self) -> bool {
            *self == CENTER0R::EDGE_ALIGNED_
        }
        # [ doc = "Checks if the value of the field is `CENTER_ALIGNED_`" ]
        # [ inline ( always ) ]
        pub fn is_center_aligned_(&self) -> bool {
            *self == CENTER0R::CENTER_ALIGNED_
        }
    }
    # [ doc = "Possible values of the field `POLA0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum POLA0R {
        # [ doc = "Passive state is LOW, active state is HIGH." ]
        PASSIVE_STATE_IS_LOW,
        # [ doc = "Passive state is HIGH, active state is LOW." ]
        PASSIVE_STATE_IS_HIG,
    }
    impl POLA0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                POLA0R::PASSIVE_STATE_IS_LOW => false,
                POLA0R::PASSIVE_STATE_IS_HIG => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> POLA0R {
            match value {
                false => POLA0R::PASSIVE_STATE_IS_LOW,
                true => POLA0R::PASSIVE_STATE_IS_HIG,
            }
        }
        # [ doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`" ]
        # [ inline ( always ) ]
        pub fn is_passive_state_is_low(&self) -> bool {
            *self == POLA0R::PASSIVE_STATE_IS_LOW
        }
        # [ doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`" ]
        # [ inline ( always ) ]
        pub fn is_passive_state_is_hig(&self) -> bool {
            *self == POLA0R::PASSIVE_STATE_IS_HIG
        }
    }
    # [ doc = "Possible values of the field `DTE0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DTE0R {
        # [ doc = "Dead-time disabled." ]
        DEAD_TIME_DISABLED_,
        # [ doc = "Dead-time enabled." ]
        DEAD_TIME_ENABLED_,
    }
    impl DTE0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DTE0R::DEAD_TIME_DISABLED_ => false,
                DTE0R::DEAD_TIME_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DTE0R {
            match value {
                false => DTE0R::DEAD_TIME_DISABLED_,
                true => DTE0R::DEAD_TIME_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_dead_time_disabled_(&self) -> bool {
            *self == DTE0R::DEAD_TIME_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_dead_time_enabled_(&self) -> bool {
            *self == DTE0R::DEAD_TIME_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `DISUP0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DISUP0R {
        # [ doc = "Functional registers are updated from the write registers at the end of each PWM cycle." ]
        UPDATE,
        # [ doc = "Functional registers remain the same as long as the timer is running." ]
        NOUPDATE,
    }
    impl DISUP0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DISUP0R::UPDATE => false,
                DISUP0R::NOUPDATE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DISUP0R {
            match value {
                false => DISUP0R::UPDATE,
                true => DISUP0R::NOUPDATE,
            }
        }
        # [ doc = "Checks if the value of the field is `UPDATE`" ]
        # [ inline ( always ) ]
        pub fn is_update(&self) -> bool {
            *self == DISUP0R::UPDATE
        }
        # [ doc = "Checks if the value of the field is `NOUPDATE`" ]
        # [ inline ( always ) ]
        pub fn is_noupdate(&self) -> bool {
            *self == DISUP0R::NOUPDATE
        }
    }
    # [ doc = "Possible values of the field `RUN1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RUN1R {
        # [ doc = "Stop." ]
        STOP_,
        # [ doc = "Run." ]
        RUN_,
    }
    impl RUN1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RUN1R::STOP_ => false,
                RUN1R::RUN_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RUN1R {
            match value {
                false => RUN1R::STOP_,
                true => RUN1R::RUN_,
            }
        }
        # [ doc = "Checks if the value of the field is `STOP_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_(&self) -> bool {
            *self == RUN1R::STOP_
        }
        # [ doc = "Checks if the value of the field is `RUN_`" ]
        # [ inline ( always ) ]
        pub fn is_run_(&self) -> bool {
            *self == RUN1R::RUN_
        }
    }
    # [ doc = "Possible values of the field `CENTER1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CENTER1R {
        # [ doc = "Edge-aligned." ]
        EDGE_ALIGNED_,
        # [ doc = "Center-aligned." ]
        CENTER_ALIGNED_,
    }
    impl CENTER1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CENTER1R::EDGE_ALIGNED_ => false,
                CENTER1R::CENTER_ALIGNED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CENTER1R {
            match value {
                false => CENTER1R::EDGE_ALIGNED_,
                true => CENTER1R::CENTER_ALIGNED_,
            }
        }
        # [ doc = "Checks if the value of the field is `EDGE_ALIGNED_`" ]
        # [ inline ( always ) ]
        pub fn is_edge_aligned_(&self) -> bool {
            *self == CENTER1R::EDGE_ALIGNED_
        }
        # [ doc = "Checks if the value of the field is `CENTER_ALIGNED_`" ]
        # [ inline ( always ) ]
        pub fn is_center_aligned_(&self) -> bool {
            *self == CENTER1R::CENTER_ALIGNED_
        }
    }
    # [ doc = "Possible values of the field `POLA1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum POLA1R {
        # [ doc = "Passive state is LOW, active state is HIGH." ]
        PASSIVE_STATE_IS_LOW,
        # [ doc = "Passive state is HIGH, active state is LOW." ]
        PASSIVE_STATE_IS_HIG,
    }
    impl POLA1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                POLA1R::PASSIVE_STATE_IS_LOW => false,
                POLA1R::PASSIVE_STATE_IS_HIG => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> POLA1R {
            match value {
                false => POLA1R::PASSIVE_STATE_IS_LOW,
                true => POLA1R::PASSIVE_STATE_IS_HIG,
            }
        }
        # [ doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`" ]
        # [ inline ( always ) ]
        pub fn is_passive_state_is_low(&self) -> bool {
            *self == POLA1R::PASSIVE_STATE_IS_LOW
        }
        # [ doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`" ]
        # [ inline ( always ) ]
        pub fn is_passive_state_is_hig(&self) -> bool {
            *self == POLA1R::PASSIVE_STATE_IS_HIG
        }
    }
    # [ doc = "Possible values of the field `DTE1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DTE1R {
        # [ doc = "Dead-time disabled." ]
        DEAD_TIME_DISABLED_,
        # [ doc = "Dead-time enabled." ]
        DEAD_TIME_ENABLED_,
    }
    impl DTE1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DTE1R::DEAD_TIME_DISABLED_ => false,
                DTE1R::DEAD_TIME_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DTE1R {
            match value {
                false => DTE1R::DEAD_TIME_DISABLED_,
                true => DTE1R::DEAD_TIME_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_dead_time_disabled_(&self) -> bool {
            *self == DTE1R::DEAD_TIME_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_dead_time_enabled_(&self) -> bool {
            *self == DTE1R::DEAD_TIME_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `DISUP1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DISUP1R {
        # [ doc = "Functional registers are updated from the write registers at the end of each PWM cycle." ]
        UPDATE,
        # [ doc = "Functional registers remain the same as long as the timer is running." ]
        NOUPDATE,
    }
    impl DISUP1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DISUP1R::UPDATE => false,
                DISUP1R::NOUPDATE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DISUP1R {
            match value {
                false => DISUP1R::UPDATE,
                true => DISUP1R::NOUPDATE,
            }
        }
        # [ doc = "Checks if the value of the field is `UPDATE`" ]
        # [ inline ( always ) ]
        pub fn is_update(&self) -> bool {
            *self == DISUP1R::UPDATE
        }
        # [ doc = "Checks if the value of the field is `NOUPDATE`" ]
        # [ inline ( always ) ]
        pub fn is_noupdate(&self) -> bool {
            *self == DISUP1R::NOUPDATE
        }
    }
    # [ doc = "Possible values of the field `RUN2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RUN2R {
        # [ doc = "Stop." ]
        STOP_,
        # [ doc = "Run." ]
        RUN_,
    }
    impl RUN2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RUN2R::STOP_ => false,
                RUN2R::RUN_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RUN2R {
            match value {
                false => RUN2R::STOP_,
                true => RUN2R::RUN_,
            }
        }
        # [ doc = "Checks if the value of the field is `STOP_`" ]
        # [ inline ( always ) ]
        pub fn is_stop_(&self) -> bool {
            *self == RUN2R::STOP_
        }
        # [ doc = "Checks if the value of the field is `RUN_`" ]
        # [ inline ( always ) ]
        pub fn is_run_(&self) -> bool {
            *self == RUN2R::RUN_
        }
    }
    # [ doc = "Possible values of the field `CENTER2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CENTER2R {
        # [ doc = "Edge-aligned." ]
        EDGE_ALIGNED_,
        # [ doc = "Center-aligned." ]
        CENTER_ALIGNED_,
    }
    impl CENTER2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CENTER2R::EDGE_ALIGNED_ => false,
                CENTER2R::CENTER_ALIGNED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CENTER2R {
            match value {
                false => CENTER2R::EDGE_ALIGNED_,
                true => CENTER2R::CENTER_ALIGNED_,
            }
        }
        # [ doc = "Checks if the value of the field is `EDGE_ALIGNED_`" ]
        # [ inline ( always ) ]
        pub fn is_edge_aligned_(&self) -> bool {
            *self == CENTER2R::EDGE_ALIGNED_
        }
        # [ doc = "Checks if the value of the field is `CENTER_ALIGNED_`" ]
        # [ inline ( always ) ]
        pub fn is_center_aligned_(&self) -> bool {
            *self == CENTER2R::CENTER_ALIGNED_
        }
    }
    # [ doc = "Possible values of the field `POLA2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum POLA2R {
        # [ doc = "Passive state is LOW, active state is HIGH." ]
        PASSIVE_STATE_IS_LOW,
        # [ doc = "Passive state is HIGH, active state is LOW." ]
        PASSIVE_STATE_IS_HIG,
    }
    impl POLA2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                POLA2R::PASSIVE_STATE_IS_LOW => false,
                POLA2R::PASSIVE_STATE_IS_HIG => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> POLA2R {
            match value {
                false => POLA2R::PASSIVE_STATE_IS_LOW,
                true => POLA2R::PASSIVE_STATE_IS_HIG,
            }
        }
        # [ doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`" ]
        # [ inline ( always ) ]
        pub fn is_passive_state_is_low(&self) -> bool {
            *self == POLA2R::PASSIVE_STATE_IS_LOW
        }
        # [ doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`" ]
        # [ inline ( always ) ]
        pub fn is_passive_state_is_hig(&self) -> bool {
            *self == POLA2R::PASSIVE_STATE_IS_HIG
        }
    }
    # [ doc = "Possible values of the field `DTE2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DTE2R {
        # [ doc = "Dead-time disabled." ]
        DEAD_TIME_DISABLED_,
        # [ doc = "Dead-time enabled." ]
        DEAD_TIME_ENABLED_,
    }
    impl DTE2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DTE2R::DEAD_TIME_DISABLED_ => false,
                DTE2R::DEAD_TIME_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DTE2R {
            match value {
                false => DTE2R::DEAD_TIME_DISABLED_,
                true => DTE2R::DEAD_TIME_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_dead_time_disabled_(&self) -> bool {
            *self == DTE2R::DEAD_TIME_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_dead_time_enabled_(&self) -> bool {
            *self == DTE2R::DEAD_TIME_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `DISUP2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DISUP2R {
        # [ doc = "Functional registers are updated from the write registers at the end of each PWM cycle." ]
        UPDATE,
        # [ doc = "Functional registers remain the same as long as the timer is running." ]
        NOUPDATE,
    }
    impl DISUP2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DISUP2R::UPDATE => false,
                DISUP2R::NOUPDATE => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DISUP2R {
            match value {
                false => DISUP2R::UPDATE,
                true => DISUP2R::NOUPDATE,
            }
        }
        # [ doc = "Checks if the value of the field is `UPDATE`" ]
        # [ inline ( always ) ]
        pub fn is_update(&self) -> bool {
            *self == DISUP2R::UPDATE
        }
        # [ doc = "Checks if the value of the field is `NOUPDATE`" ]
        # [ inline ( always ) ]
        pub fn is_noupdate(&self) -> bool {
            *self == DISUP2R::NOUPDATE
        }
    }
    # [ doc = "Possible values of the field `INVBDC`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum INVBDCR {
        # [ doc = "The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)." ]
        OPPOSITE,
        # [ doc = "The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)" ]
        SAME,
    }
    impl INVBDCR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                INVBDCR::OPPOSITE => false,
                INVBDCR::SAME => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> INVBDCR {
            match value {
                false => INVBDCR::OPPOSITE,
                true => INVBDCR::SAME,
            }
        }
        # [ doc = "Checks if the value of the field is `OPPOSITE`" ]
        # [ inline ( always ) ]
        pub fn is_opposite(&self) -> bool {
            *self == INVBDCR::OPPOSITE
        }
        # [ doc = "Checks if the value of the field is `SAME`" ]
        # [ inline ( always ) ]
        pub fn is_same(&self) -> bool {
            *self == INVBDCR::SAME
        }
    }
    # [ doc = "Possible values of the field `ACMODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ACMODER {
        # [ doc = "3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register." ]
        _3_PHASE_AC_MODE_OFF,
        # [ doc = "3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0." ]
        _3_PHASE_AC_MODE_ON_,
    }
    impl ACMODER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ACMODER::_3_PHASE_AC_MODE_OFF => false,
                ACMODER::_3_PHASE_AC_MODE_ON_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ACMODER {
            match value {
                false => ACMODER::_3_PHASE_AC_MODE_OFF,
                true => ACMODER::_3_PHASE_AC_MODE_ON_,
            }
        }
        # [ doc = "Checks if the value of the field is `_3_PHASE_AC_MODE_OFF`" ]
        # [ inline ( always ) ]
        pub fn is_3_phase_ac_mode_off(&self) -> bool {
            *self == ACMODER::_3_PHASE_AC_MODE_OFF
        }
        # [ doc = "Checks if the value of the field is `_3_PHASE_AC_MODE_ON_`" ]
        # [ inline ( always ) ]
        pub fn is_3_phase_ac_mode_on_(&self) -> bool {
            *self == ACMODER::_3_PHASE_AC_MODE_ON_
        }
    }
    # [ doc = "Possible values of the field `DCMODE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DCMODER {
        # [ doc = "3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)" ]
        _3_PHASE_DC_MODE_OFF,
        # [ doc = "3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs." ]
        _3_PHASE_DC_MODE_ON_,
    }
    impl DCMODER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DCMODER::_3_PHASE_DC_MODE_OFF => false,
                DCMODER::_3_PHASE_DC_MODE_ON_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DCMODER {
            match value {
                false => DCMODER::_3_PHASE_DC_MODE_OFF,
                true => DCMODER::_3_PHASE_DC_MODE_ON_,
            }
        }
        # [ doc = "Checks if the value of the field is `_3_PHASE_DC_MODE_OFF`" ]
        # [ inline ( always ) ]
        pub fn is_3_phase_dc_mode_off(&self) -> bool {
            *self == DCMODER::_3_PHASE_DC_MODE_OFF
        }
        # [ doc = "Checks if the value of the field is `_3_PHASE_DC_MODE_ON_`" ]
        # [ inline ( always ) ]
        pub fn is_3_phase_dc_mode_on_(&self) -> bool {
            *self == DCMODER::_3_PHASE_DC_MODE_ON_
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Stops/starts timer channel 0." ]
        # [ inline ( always ) ]
        pub fn run0(&self) -> RUN0R {
            RUN0R::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 0;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 1 - Edge/center aligned operation for channel 0." ]
        # [ inline ( always ) ]
        pub fn center0(&self) -> CENTER0R {
            CENTER0R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 1;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 2 - Selects polarity of the MCOA0 and MCOB0 pins." ]
        # [ inline ( always ) ]
        pub fn pola0(&self) -> POLA0R {
            POLA0R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 2;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 3 - Controls the dead-time feature for channel 0." ]
        # [ inline ( always ) ]
        pub fn dte0(&self) -> DTE0R {
            DTE0R::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 3;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 4 - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)." ]
        # [ inline ( always ) ]
        pub fn disup0(&self) -> DISUP0R {
            DISUP0R::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 4;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 8 - Stops/starts timer channel 1." ]
        # [ inline ( always ) ]
        pub fn run1(&self) -> RUN1R {
            RUN1R::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 8;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 9 - Edge/center aligned operation for channel 1." ]
        # [ inline ( always ) ]
        pub fn center1(&self) -> CENTER1R {
            CENTER1R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 9;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 10 - Selects polarity of the MCOA1 and MCOB1 pins." ]
        # [ inline ( always ) ]
        pub fn pola1(&self) -> POLA1R {
            POLA1R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 10;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 11 - Controls the dead-time feature for channel 1." ]
        # [ inline ( always ) ]
        pub fn dte1(&self) -> DTE1R {
            DTE1R::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 11;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 12 - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)." ]
        # [ inline ( always ) ]
        pub fn disup1(&self) -> DISUP1R {
            DISUP1R::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 12;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 16 - Stops/starts timer channel 2." ]
        # [ inline ( always ) ]
        pub fn run2(&self) -> RUN2R {
            RUN2R::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 16;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 17 - Edge/center aligned operation for channel 2." ]
        # [ inline ( always ) ]
        pub fn center2(&self) -> CENTER2R {
            CENTER2R::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 17;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 18 - Selects polarity of the MCOA2 and MCOB2 pins." ]
        # [ inline ( always ) ]
        pub fn pola2(&self) -> POLA2R {
            POLA2R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 18;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 19 - Controls the dead-time feature for channel 1." ]
        # [ inline ( always ) ]
        pub fn dte2(&self) -> DTE2R {
            DTE2R::_from({
                             const MASK: bool = true;
                             const OFFSET: u8 = 19;
                             ((self.bits >> OFFSET) & MASK as u32) != 0
                         })
        }
        # [ doc = "Bit 20 - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)." ]
        # [ inline ( always ) ]
        pub fn disup2(&self) -> DISUP2R {
            DISUP2R::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 20;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 29 - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode." ]
        # [ inline ( always ) ]
        pub fn invbdc(&self) -> INVBDCR {
            INVBDCR::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 29;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 30 - 3-phase AC mode select (see Section 24.8.7)." ]
        # [ inline ( always ) ]
        pub fn acmode(&self) -> ACMODER {
            ACMODER::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 30;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 31 - 3-phase DC mode select (see Section 24.8.6)." ]
        # [ inline ( always ) ]
        pub fn dcmode(&self) -> DCMODER {
            DCMODER::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 31;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
    }
}
# [ doc = "PWM Control set address" ]
pub struct CON_SET {
    register: VolatileCell<u32>,
}
# [ doc = "PWM Control set address" ]
pub mod con_set {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CON_SET {
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
    pub struct _RUN0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RUN0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CENTER0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CENTER0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _POLA0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLA0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DTE0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTE0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DISUP0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DISUP0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RUN1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RUN1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CENTER1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CENTER1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _POLA1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLA1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DTE1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTE1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DISUP1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DISUP1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RUN2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RUN2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CENTER2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CENTER2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _POLA2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLA2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DTE2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTE2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DISUP2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DISUP2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _INVBDC_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVBDC_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ACMODE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACMODE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DCMODE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DCMODE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn run0_set(&mut self) -> _RUN0_SETW {
            _RUN0_SETW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn center0_set(&mut self) -> _CENTER0_SETW {
            _CENTER0_SETW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn pola0_set(&mut self) -> _POLA0_SETW {
            _POLA0_SETW { w: self }
        }
        # [ doc = "Bit 3 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn dte0_set(&mut self) -> _DTE0_SETW {
            _DTE0_SETW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn disup0_set(&mut self) -> _DISUP0_SETW {
            _DISUP0_SETW { w: self }
        }
        # [ doc = "Bit 8 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn run1_set(&mut self) -> _RUN1_SETW {
            _RUN1_SETW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn center1_set(&mut self) -> _CENTER1_SETW {
            _CENTER1_SETW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn pola1_set(&mut self) -> _POLA1_SETW {
            _POLA1_SETW { w: self }
        }
        # [ doc = "Bit 11 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn dte1_set(&mut self) -> _DTE1_SETW {
            _DTE1_SETW { w: self }
        }
        # [ doc = "Bit 12 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn disup1_set(&mut self) -> _DISUP1_SETW {
            _DISUP1_SETW { w: self }
        }
        # [ doc = "Bit 16 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn run2_set(&mut self) -> _RUN2_SETW {
            _RUN2_SETW { w: self }
        }
        # [ doc = "Bit 17 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn center2_set(&mut self) -> _CENTER2_SETW {
            _CENTER2_SETW { w: self }
        }
        # [ doc = "Bit 18 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn pola2_set(&mut self) -> _POLA2_SETW {
            _POLA2_SETW { w: self }
        }
        # [ doc = "Bit 19 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn dte2_set(&mut self) -> _DTE2_SETW {
            _DTE2_SETW { w: self }
        }
        # [ doc = "Bit 20 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn disup2_set(&mut self) -> _DISUP2_SETW {
            _DISUP2_SETW { w: self }
        }
        # [ doc = "Bit 29 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn invbdc_set(&mut self) -> _INVBDC_SETW {
            _INVBDC_SETW { w: self }
        }
        # [ doc = "Bit 30 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn acmode_set(&mut self) -> _ACMODE_SETW {
            _ACMODE_SETW { w: self }
        }
        # [ doc = "Bit 31 - Writing a one sets the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn dcmode_set(&mut self) -> _DCMODE_SETW {
            _DCMODE_SETW { w: self }
        }
    }
}
# [ doc = "PWM Control clear address" ]
pub struct CON_CLR {
    register: VolatileCell<u32>,
}
# [ doc = "PWM Control clear address" ]
pub mod con_clr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CON_CLR {
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
    pub struct _RUN0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RUN0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CENTER0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CENTER0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _POLA0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLA0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DTE0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTE0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DISUP0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DISUP0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RUN1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RUN1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CENTER1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CENTER1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _POLA1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLA1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DTE1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTE1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DISUP1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DISUP1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RUN2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RUN2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CENTER2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CENTER2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _POLA2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLA2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DTE2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTE2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DISUP2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DISUP2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _INVBDC_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVBDC_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ACMOD_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACMOD_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DCMODE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DCMODE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn run0_clr(&mut self) -> _RUN0_CLRW {
            _RUN0_CLRW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn center0_clr(&mut self) -> _CENTER0_CLRW {
            _CENTER0_CLRW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn pola0_clr(&mut self) -> _POLA0_CLRW {
            _POLA0_CLRW { w: self }
        }
        # [ doc = "Bit 3 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn dte0_clr(&mut self) -> _DTE0_CLRW {
            _DTE0_CLRW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn disup0_clr(&mut self) -> _DISUP0_CLRW {
            _DISUP0_CLRW { w: self }
        }
        # [ doc = "Bit 8 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn run1_clr(&mut self) -> _RUN1_CLRW {
            _RUN1_CLRW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn center1_clr(&mut self) -> _CENTER1_CLRW {
            _CENTER1_CLRW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn pola1_clr(&mut self) -> _POLA1_CLRW {
            _POLA1_CLRW { w: self }
        }
        # [ doc = "Bit 11 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn dte1_clr(&mut self) -> _DTE1_CLRW {
            _DTE1_CLRW { w: self }
        }
        # [ doc = "Bit 12 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn disup1_clr(&mut self) -> _DISUP1_CLRW {
            _DISUP1_CLRW { w: self }
        }
        # [ doc = "Bit 16 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn run2_clr(&mut self) -> _RUN2_CLRW {
            _RUN2_CLRW { w: self }
        }
        # [ doc = "Bit 17 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn center2_clr(&mut self) -> _CENTER2_CLRW {
            _CENTER2_CLRW { w: self }
        }
        # [ doc = "Bit 18 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn pola2_clr(&mut self) -> _POLA2_CLRW {
            _POLA2_CLRW { w: self }
        }
        # [ doc = "Bit 19 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn dte2_clr(&mut self) -> _DTE2_CLRW {
            _DTE2_CLRW { w: self }
        }
        # [ doc = "Bit 20 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn disup2_clr(&mut self) -> _DISUP2_CLRW {
            _DISUP2_CLRW { w: self }
        }
        # [ doc = "Bit 29 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn invbdc_clr(&mut self) -> _INVBDC_CLRW {
            _INVBDC_CLRW { w: self }
        }
        # [ doc = "Bit 30 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn acmod_clr(&mut self) -> _ACMOD_CLRW {
            _ACMOD_CLRW { w: self }
        }
        # [ doc = "Bit 31 - Writing a one clears the corresponding bit in the CON register." ]
        # [ inline ( always ) ]
        pub fn dcmode_clr(&mut self) -> _DCMODE_CLRW {
            _DCMODE_CLRW { w: self }
        }
    }
}
# [ doc = "Capture Control read address" ]
pub struct CAPCON {
    register: VolatileCell<u32>,
}
# [ doc = "Capture Control read address" ]
pub mod capcon {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::CAPCON {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP0MCI0_RER {
        bits: bool,
    }
    impl CAP0MCI0_RER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP0MCI0_FER {
        bits: bool,
    }
    impl CAP0MCI0_FER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP0MCI1_RER {
        bits: bool,
    }
    impl CAP0MCI1_RER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP0MCI1_FER {
        bits: bool,
    }
    impl CAP0MCI1_FER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP0MCI2_RER {
        bits: bool,
    }
    impl CAP0MCI2_RER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP0MCI2_FER {
        bits: bool,
    }
    impl CAP0MCI2_FER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP1MCI0_RER {
        bits: bool,
    }
    impl CAP1MCI0_RER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP1MCI0_FER {
        bits: bool,
    }
    impl CAP1MCI0_FER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP1MCI1_RER {
        bits: bool,
    }
    impl CAP1MCI1_RER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP1MCI1_FER {
        bits: bool,
    }
    impl CAP1MCI1_FER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP1MCI2_RER {
        bits: bool,
    }
    impl CAP1MCI2_RER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP1MCI2_FER {
        bits: bool,
    }
    impl CAP1MCI2_FER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP2MCI0_RER {
        bits: bool,
    }
    impl CAP2MCI0_RER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP2MCI0_FER {
        bits: bool,
    }
    impl CAP2MCI0_FER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP2MCI1_RER {
        bits: bool,
    }
    impl CAP2MCI1_RER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP2MCI1_FER {
        bits: bool,
    }
    impl CAP2MCI1_FER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP2MCI2_RER {
        bits: bool,
    }
    impl CAP2MCI2_RER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct CAP2MCI2_FER {
        bits: bool,
    }
    impl CAP2MCI2_FER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RT0R {
        bits: bool,
    }
    impl RT0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RT1R {
        bits: bool,
    }
    impl RT1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RT2R {
        bits: bool,
    }
    impl RT2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI0." ]
        # [ inline ( always ) ]
        pub fn cap0mci0_re(&self) -> CAP0MCI0_RER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP0MCI0_RER { bits }
        }
        # [ doc = "Bit 1 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI0." ]
        # [ inline ( always ) ]
        pub fn cap0mci0_fe(&self) -> CAP0MCI0_FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP0MCI0_FER { bits }
        }
        # [ doc = "Bit 2 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI1." ]
        # [ inline ( always ) ]
        pub fn cap0mci1_re(&self) -> CAP0MCI1_RER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP0MCI1_RER { bits }
        }
        # [ doc = "Bit 3 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI1." ]
        # [ inline ( always ) ]
        pub fn cap0mci1_fe(&self) -> CAP0MCI1_FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP0MCI1_FER { bits }
        }
        # [ doc = "Bit 4 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI2." ]
        # [ inline ( always ) ]
        pub fn cap0mci2_re(&self) -> CAP0MCI2_RER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP0MCI2_RER { bits }
        }
        # [ doc = "Bit 5 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI2." ]
        # [ inline ( always ) ]
        pub fn cap0mci2_fe(&self) -> CAP0MCI2_FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP0MCI2_FER { bits }
        }
        # [ doc = "Bit 6 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI0." ]
        # [ inline ( always ) ]
        pub fn cap1mci0_re(&self) -> CAP1MCI0_RER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP1MCI0_RER { bits }
        }
        # [ doc = "Bit 7 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI0." ]
        # [ inline ( always ) ]
        pub fn cap1mci0_fe(&self) -> CAP1MCI0_FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP1MCI0_FER { bits }
        }
        # [ doc = "Bit 8 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI1." ]
        # [ inline ( always ) ]
        pub fn cap1mci1_re(&self) -> CAP1MCI1_RER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP1MCI1_RER { bits }
        }
        # [ doc = "Bit 9 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI1." ]
        # [ inline ( always ) ]
        pub fn cap1mci1_fe(&self) -> CAP1MCI1_FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP1MCI1_FER { bits }
        }
        # [ doc = "Bit 10 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI2." ]
        # [ inline ( always ) ]
        pub fn cap1mci2_re(&self) -> CAP1MCI2_RER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP1MCI2_RER { bits }
        }
        # [ doc = "Bit 11 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI2." ]
        # [ inline ( always ) ]
        pub fn cap1mci2_fe(&self) -> CAP1MCI2_FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP1MCI2_FER { bits }
        }
        # [ doc = "Bit 12 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI0." ]
        # [ inline ( always ) ]
        pub fn cap2mci0_re(&self) -> CAP2MCI0_RER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP2MCI0_RER { bits }
        }
        # [ doc = "Bit 13 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI0." ]
        # [ inline ( always ) ]
        pub fn cap2mci0_fe(&self) -> CAP2MCI0_FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP2MCI0_FER { bits }
        }
        # [ doc = "Bit 14 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI1." ]
        # [ inline ( always ) ]
        pub fn cap2mci1_re(&self) -> CAP2MCI1_RER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP2MCI1_RER { bits }
        }
        # [ doc = "Bit 15 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI1." ]
        # [ inline ( always ) ]
        pub fn cap2mci1_fe(&self) -> CAP2MCI1_FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP2MCI1_FER { bits }
        }
        # [ doc = "Bit 16 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI2." ]
        # [ inline ( always ) ]
        pub fn cap2mci2_re(&self) -> CAP2MCI2_RER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP2MCI2_RER { bits }
        }
        # [ doc = "Bit 17 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI2." ]
        # [ inline ( always ) ]
        pub fn cap2mci2_fe(&self) -> CAP2MCI2_FER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CAP2MCI2_FER { bits }
        }
        # [ doc = "Bit 18 - If this bit is 1, TC0 is reset by a channel 0 capture event." ]
        # [ inline ( always ) ]
        pub fn rt0(&self) -> RT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RT0R { bits }
        }
        # [ doc = "Bit 19 - If this bit is 1, TC1 is reset by a channel 1 capture event." ]
        # [ inline ( always ) ]
        pub fn rt1(&self) -> RT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RT1R { bits }
        }
        # [ doc = "Bit 20 - If this bit is 1, TC2 is reset by a channel 2 capture event." ]
        # [ inline ( always ) ]
        pub fn rt2(&self) -> RT2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RT2R { bits }
        }
    }
}
# [ doc = "Capture Control set address" ]
pub struct CAPCON_SET {
    register: VolatileCell<u32>,
}
# [ doc = "Capture Control set address" ]
pub mod capcon_set {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CAPCON_SET {
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
    pub struct _CAP0MCI0_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI0_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI0_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI0_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI1_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI1_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI1_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI1_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI2_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI2_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI2_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI2_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI0_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI0_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI0_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI0_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI1_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI1_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI1_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI1_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI2_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI2_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI2_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI2_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI0_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI0_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI0_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI0_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI1_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI1_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI1_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI1_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI2_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI2_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI2_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI2_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RT0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RT0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RT1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RT1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RT2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RT2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci0_re_set(&mut self) -> _CAP0MCI0_RE_SETW {
            _CAP0MCI0_RE_SETW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci0_fe_set(&mut self) -> _CAP0MCI0_FE_SETW {
            _CAP0MCI0_FE_SETW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci1_re_set(&mut self) -> _CAP0MCI1_RE_SETW {
            _CAP0MCI1_RE_SETW { w: self }
        }
        # [ doc = "Bit 3 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci1_fe_set(&mut self) -> _CAP0MCI1_FE_SETW {
            _CAP0MCI1_FE_SETW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci2_re_set(&mut self) -> _CAP0MCI2_RE_SETW {
            _CAP0MCI2_RE_SETW { w: self }
        }
        # [ doc = "Bit 5 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci2_fe_set(&mut self) -> _CAP0MCI2_FE_SETW {
            _CAP0MCI2_FE_SETW { w: self }
        }
        # [ doc = "Bit 6 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci0_re_set(&mut self) -> _CAP1MCI0_RE_SETW {
            _CAP1MCI0_RE_SETW { w: self }
        }
        # [ doc = "Bit 7 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci0_fe_set(&mut self) -> _CAP1MCI0_FE_SETW {
            _CAP1MCI0_FE_SETW { w: self }
        }
        # [ doc = "Bit 8 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci1_re_set(&mut self) -> _CAP1MCI1_RE_SETW {
            _CAP1MCI1_RE_SETW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci1_fe_set(&mut self) -> _CAP1MCI1_FE_SETW {
            _CAP1MCI1_FE_SETW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci2_re_set(&mut self) -> _CAP1MCI2_RE_SETW {
            _CAP1MCI2_RE_SETW { w: self }
        }
        # [ doc = "Bit 11 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci2_fe_set(&mut self) -> _CAP1MCI2_FE_SETW {
            _CAP1MCI2_FE_SETW { w: self }
        }
        # [ doc = "Bit 12 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci0_re_set(&mut self) -> _CAP2MCI0_RE_SETW {
            _CAP2MCI0_RE_SETW { w: self }
        }
        # [ doc = "Bit 13 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci0_fe_set(&mut self) -> _CAP2MCI0_FE_SETW {
            _CAP2MCI0_FE_SETW { w: self }
        }
        # [ doc = "Bit 14 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci1_re_set(&mut self) -> _CAP2MCI1_RE_SETW {
            _CAP2MCI1_RE_SETW { w: self }
        }
        # [ doc = "Bit 15 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci1_fe_set(&mut self) -> _CAP2MCI1_FE_SETW {
            _CAP2MCI1_FE_SETW { w: self }
        }
        # [ doc = "Bit 16 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci2_re_set(&mut self) -> _CAP2MCI2_RE_SETW {
            _CAP2MCI2_RE_SETW { w: self }
        }
        # [ doc = "Bit 17 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci2_fe_set(&mut self) -> _CAP2MCI2_FE_SETW {
            _CAP2MCI2_FE_SETW { w: self }
        }
        # [ doc = "Bit 18 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn rt0_set(&mut self) -> _RT0_SETW {
            _RT0_SETW { w: self }
        }
        # [ doc = "Bit 19 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn rt1_set(&mut self) -> _RT1_SETW {
            _RT1_SETW { w: self }
        }
        # [ doc = "Bit 20 - Writing a one sets the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn rt2_set(&mut self) -> _RT2_SETW {
            _RT2_SETW { w: self }
        }
    }
}
# [ doc = "Event Control clear address" ]
pub struct CAPCON_CLR {
    register: VolatileCell<u32>,
}
# [ doc = "Event Control clear address" ]
pub mod capcon_clr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CAPCON_CLR {
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
    pub struct _CAP0MCI0_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI0_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI0_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI0_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI1_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI1_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI1_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI1_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI2_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI2_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP0MCI2_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP0MCI2_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI0_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI0_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI0_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI0_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI1_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI1_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI1_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI1_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI2_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI2_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP1MCI2_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP1MCI2_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI0_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI0_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI0_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI0_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI1_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI1_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI1_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI1_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI2_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI2_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP2MCI2_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP2MCI2_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RT0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RT0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RT1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RT1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RT2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RT2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci0_re_clr(&mut self) -> _CAP0MCI0_RE_CLRW {
            _CAP0MCI0_RE_CLRW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci0_fe_clr(&mut self) -> _CAP0MCI0_FE_CLRW {
            _CAP0MCI0_FE_CLRW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci1_re_clr(&mut self) -> _CAP0MCI1_RE_CLRW {
            _CAP0MCI1_RE_CLRW { w: self }
        }
        # [ doc = "Bit 3 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci1_fe_clr(&mut self) -> _CAP0MCI1_FE_CLRW {
            _CAP0MCI1_FE_CLRW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci2_re_clr(&mut self) -> _CAP0MCI2_RE_CLRW {
            _CAP0MCI2_RE_CLRW { w: self }
        }
        # [ doc = "Bit 5 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap0mci2_fe_clr(&mut self) -> _CAP0MCI2_FE_CLRW {
            _CAP0MCI2_FE_CLRW { w: self }
        }
        # [ doc = "Bit 6 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci0_re_clr(&mut self) -> _CAP1MCI0_RE_CLRW {
            _CAP1MCI0_RE_CLRW { w: self }
        }
        # [ doc = "Bit 7 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci0_fe_clr(&mut self) -> _CAP1MCI0_FE_CLRW {
            _CAP1MCI0_FE_CLRW { w: self }
        }
        # [ doc = "Bit 8 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci1_re_clr(&mut self) -> _CAP1MCI1_RE_CLRW {
            _CAP1MCI1_RE_CLRW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci1_fe_clr(&mut self) -> _CAP1MCI1_FE_CLRW {
            _CAP1MCI1_FE_CLRW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci2_re_clr(&mut self) -> _CAP1MCI2_RE_CLRW {
            _CAP1MCI2_RE_CLRW { w: self }
        }
        # [ doc = "Bit 11 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap1mci2_fe_clr(&mut self) -> _CAP1MCI2_FE_CLRW {
            _CAP1MCI2_FE_CLRW { w: self }
        }
        # [ doc = "Bit 12 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci0_re_clr(&mut self) -> _CAP2MCI0_RE_CLRW {
            _CAP2MCI0_RE_CLRW { w: self }
        }
        # [ doc = "Bit 13 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci0_fe_clr(&mut self) -> _CAP2MCI0_FE_CLRW {
            _CAP2MCI0_FE_CLRW { w: self }
        }
        # [ doc = "Bit 14 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci1_re_clr(&mut self) -> _CAP2MCI1_RE_CLRW {
            _CAP2MCI1_RE_CLRW { w: self }
        }
        # [ doc = "Bit 15 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci1_fe_clr(&mut self) -> _CAP2MCI1_FE_CLRW {
            _CAP2MCI1_FE_CLRW { w: self }
        }
        # [ doc = "Bit 16 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci2_re_clr(&mut self) -> _CAP2MCI2_RE_CLRW {
            _CAP2MCI2_RE_CLRW { w: self }
        }
        # [ doc = "Bit 17 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn cap2mci2_fe_clr(&mut self) -> _CAP2MCI2_FE_CLRW {
            _CAP2MCI2_FE_CLRW { w: self }
        }
        # [ doc = "Bit 18 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn rt0_clr(&mut self) -> _RT0_CLRW {
            _RT0_CLRW { w: self }
        }
        # [ doc = "Bit 19 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn rt1_clr(&mut self) -> _RT1_CLRW {
            _RT1_CLRW { w: self }
        }
        # [ doc = "Bit 20 - Writing a one clears the corresponding bits in the CAPCON register." ]
        # [ inline ( always ) ]
        pub fn rt2_clr(&mut self) -> _RT2_CLRW {
            _RT2_CLRW { w: self }
        }
    }
}
# [ doc = "Timer Counter register" ]
pub struct TC {
    register: VolatileCell<u32>,
}
# [ doc = "Timer Counter register" ]
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
    pub struct MCTCR {
        bits: u32,
    }
    impl MCTCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MCTCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCTCW<'a> {
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
        # [ doc = "Bits 0:31 - Timer/Counter value." ]
        # [ inline ( always ) ]
        pub fn mctc(&self) -> MCTCR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            MCTCR { bits }
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
        # [ doc = "Bits 0:31 - Timer/Counter value." ]
        # [ inline ( always ) ]
        pub fn mctc(&mut self) -> _MCTCW {
            _MCTCW { w: self }
        }
    }
}
# [ doc = "Limit register" ]
pub struct LIM {
    register: VolatileCell<u32>,
}
# [ doc = "Limit register" ]
pub mod lim {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::LIM {
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
    pub struct MCLIMR {
        bits: u32,
    }
    impl MCLIMR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MCLIMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCLIMW<'a> {
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
        # [ doc = "Bits 0:31 - Limit value." ]
        # [ inline ( always ) ]
        pub fn mclim(&self) -> MCLIMR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            MCLIMR { bits }
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
        # [ doc = "Bits 0:31 - Limit value." ]
        # [ inline ( always ) ]
        pub fn mclim(&mut self) -> _MCLIMW {
            _MCLIMW { w: self }
        }
    }
}
# [ doc = "Match register" ]
pub struct MAT {
    register: VolatileCell<u32>,
}
# [ doc = "Match register" ]
pub mod mat {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MAT {
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
    pub struct MCMATR {
        bits: u32,
    }
    impl MCMATR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MCMATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCMATW<'a> {
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
        # [ doc = "Bits 0:31 - Match value." ]
        # [ inline ( always ) ]
        pub fn mcmat(&self) -> MCMATR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            MCMATR { bits }
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
        # [ doc = "Bits 0:31 - Match value." ]
        # [ inline ( always ) ]
        pub fn mcmat(&mut self) -> _MCMATW {
            _MCMATW { w: self }
        }
    }
}
# [ doc = "Dead time register" ]
pub struct DT {
    register: VolatileCell<u32>,
}
# [ doc = "Dead time register" ]
pub mod dt {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::DT {
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
    pub struct DT0R {
        bits: u16,
    }
    impl DT0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DT1R {
        bits: u16,
    }
    impl DT1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DT2R {
        bits: u16,
    }
    impl DT2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DT0W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DT1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DT2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DT2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 1023;
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
        # [ doc = "Bits 0:9 - Dead time for channel 0.[1]" ]
        # [ inline ( always ) ]
        pub fn dt0(&self) -> DT0R {
            let bits = {
                const MASK: u16 = 1023;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DT0R { bits }
        }
        # [ doc = "Bits 10:19 - Dead time for channel 1.[2]" ]
        # [ inline ( always ) ]
        pub fn dt1(&self) -> DT1R {
            let bits = {
                const MASK: u16 = 1023;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DT1R { bits }
        }
        # [ doc = "Bits 20:29 - Dead time for channel 2.[2]" ]
        # [ inline ( always ) ]
        pub fn dt2(&self) -> DT2R {
            let bits = {
                const MASK: u16 = 1023;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DT2R { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 1073741823 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:9 - Dead time for channel 0.[1]" ]
        # [ inline ( always ) ]
        pub fn dt0(&mut self) -> _DT0W {
            _DT0W { w: self }
        }
        # [ doc = "Bits 10:19 - Dead time for channel 1.[2]" ]
        # [ inline ( always ) ]
        pub fn dt1(&mut self) -> _DT1W {
            _DT1W { w: self }
        }
        # [ doc = "Bits 20:29 - Dead time for channel 2.[2]" ]
        # [ inline ( always ) ]
        pub fn dt2(&mut self) -> _DT2W {
            _DT2W { w: self }
        }
    }
}
# [ doc = "Communication Pattern register" ]
pub struct CP {
    register: VolatileCell<u32>,
}
# [ doc = "Communication Pattern register" ]
pub mod cp {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CP {
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
    # [ doc = "Possible values of the field `CCPA0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CCPA0R {
        # [ doc = "MCOA0 passive." ]
        MCOA0_PASSIVE_,
        # [ doc = "internal MCOA0." ]
        INTERNAL_MCOA0_,
    }
    impl CCPA0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CCPA0R::MCOA0_PASSIVE_ => false,
                CCPA0R::INTERNAL_MCOA0_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CCPA0R {
            match value {
                false => CCPA0R::MCOA0_PASSIVE_,
                true => CCPA0R::INTERNAL_MCOA0_,
            }
        }
        # [ doc = "Checks if the value of the field is `MCOA0_PASSIVE_`" ]
        # [ inline ( always ) ]
        pub fn is_mcoa0_passive_(&self) -> bool {
            *self == CCPA0R::MCOA0_PASSIVE_
        }
        # [ doc = "Checks if the value of the field is `INTERNAL_MCOA0_`" ]
        # [ inline ( always ) ]
        pub fn is_internal_mcoa0_(&self) -> bool {
            *self == CCPA0R::INTERNAL_MCOA0_
        }
    }
    # [ doc = "Possible values of the field `CCPB0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CCPB0R {
        # [ doc = "MCOB0 passive." ]
        MCOB0_PASSIVE_,
        # [ doc = "MCOB0 tracks internal MCOA0." ]
        MCOB0_TRACKS_INTERNA,
    }
    impl CCPB0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CCPB0R::MCOB0_PASSIVE_ => false,
                CCPB0R::MCOB0_TRACKS_INTERNA => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CCPB0R {
            match value {
                false => CCPB0R::MCOB0_PASSIVE_,
                true => CCPB0R::MCOB0_TRACKS_INTERNA,
            }
        }
        # [ doc = "Checks if the value of the field is `MCOB0_PASSIVE_`" ]
        # [ inline ( always ) ]
        pub fn is_mcob0_passive_(&self) -> bool {
            *self == CCPB0R::MCOB0_PASSIVE_
        }
        # [ doc = "Checks if the value of the field is `MCOB0_TRACKS_INTERNA`" ]
        # [ inline ( always ) ]
        pub fn is_mcob0_tracks_interna(&self) -> bool {
            *self == CCPB0R::MCOB0_TRACKS_INTERNA
        }
    }
    # [ doc = "Possible values of the field `CCPA1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CCPA1R {
        # [ doc = "MCOA1 passive." ]
        MCOA1_PASSIVE_,
        # [ doc = "MCOA1 tracks internal MCOA0." ]
        MCOA1_TRACKS_INTERNA,
    }
    impl CCPA1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CCPA1R::MCOA1_PASSIVE_ => false,
                CCPA1R::MCOA1_TRACKS_INTERNA => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CCPA1R {
            match value {
                false => CCPA1R::MCOA1_PASSIVE_,
                true => CCPA1R::MCOA1_TRACKS_INTERNA,
            }
        }
        # [ doc = "Checks if the value of the field is `MCOA1_PASSIVE_`" ]
        # [ inline ( always ) ]
        pub fn is_mcoa1_passive_(&self) -> bool {
            *self == CCPA1R::MCOA1_PASSIVE_
        }
        # [ doc = "Checks if the value of the field is `MCOA1_TRACKS_INTERNA`" ]
        # [ inline ( always ) ]
        pub fn is_mcoa1_tracks_interna(&self) -> bool {
            *self == CCPA1R::MCOA1_TRACKS_INTERNA
        }
    }
    # [ doc = "Possible values of the field `CCPB1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CCPB1R {
        # [ doc = "MCOB1 passive." ]
        MCOB1_PASSIVE_,
        # [ doc = "MCOB1 tracks internal MCOA0." ]
        MCOB1_TRACKS_INTERNA,
    }
    impl CCPB1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CCPB1R::MCOB1_PASSIVE_ => false,
                CCPB1R::MCOB1_TRACKS_INTERNA => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CCPB1R {
            match value {
                false => CCPB1R::MCOB1_PASSIVE_,
                true => CCPB1R::MCOB1_TRACKS_INTERNA,
            }
        }
        # [ doc = "Checks if the value of the field is `MCOB1_PASSIVE_`" ]
        # [ inline ( always ) ]
        pub fn is_mcob1_passive_(&self) -> bool {
            *self == CCPB1R::MCOB1_PASSIVE_
        }
        # [ doc = "Checks if the value of the field is `MCOB1_TRACKS_INTERNA`" ]
        # [ inline ( always ) ]
        pub fn is_mcob1_tracks_interna(&self) -> bool {
            *self == CCPB1R::MCOB1_TRACKS_INTERNA
        }
    }
    # [ doc = "Possible values of the field `CCPA2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CCPA2R {
        # [ doc = "MCOA2 passive." ]
        MCOA2_PASSIVE_,
        # [ doc = "MCOA2 tracks internal MCOA0." ]
        MCOA2_TRACKS_INTERNA,
    }
    impl CCPA2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CCPA2R::MCOA2_PASSIVE_ => false,
                CCPA2R::MCOA2_TRACKS_INTERNA => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CCPA2R {
            match value {
                false => CCPA2R::MCOA2_PASSIVE_,
                true => CCPA2R::MCOA2_TRACKS_INTERNA,
            }
        }
        # [ doc = "Checks if the value of the field is `MCOA2_PASSIVE_`" ]
        # [ inline ( always ) ]
        pub fn is_mcoa2_passive_(&self) -> bool {
            *self == CCPA2R::MCOA2_PASSIVE_
        }
        # [ doc = "Checks if the value of the field is `MCOA2_TRACKS_INTERNA`" ]
        # [ inline ( always ) ]
        pub fn is_mcoa2_tracks_interna(&self) -> bool {
            *self == CCPA2R::MCOA2_TRACKS_INTERNA
        }
    }
    # [ doc = "Possible values of the field `CCPB2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CCPB2R {
        # [ doc = "MCOB2 passive." ]
        MCOB2_PASSIVE_,
        # [ doc = "MCOB2 tracks internal MCOA0." ]
        MCOB2_TRACKS_INTERNA,
    }
    impl CCPB2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CCPB2R::MCOB2_PASSIVE_ => false,
                CCPB2R::MCOB2_TRACKS_INTERNA => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CCPB2R {
            match value {
                false => CCPB2R::MCOB2_PASSIVE_,
                true => CCPB2R::MCOB2_TRACKS_INTERNA,
            }
        }
        # [ doc = "Checks if the value of the field is `MCOB2_PASSIVE_`" ]
        # [ inline ( always ) ]
        pub fn is_mcob2_passive_(&self) -> bool {
            *self == CCPB2R::MCOB2_PASSIVE_
        }
        # [ doc = "Checks if the value of the field is `MCOB2_TRACKS_INTERNA`" ]
        # [ inline ( always ) ]
        pub fn is_mcob2_tracks_interna(&self) -> bool {
            *self == CCPB2R::MCOB2_TRACKS_INTERNA
        }
    }
    # [ doc = "Values that can be written to the field `CCPA0`" ]
    pub enum CCPA0W {
        # [ doc = "MCOA0 passive." ]
        MCOA0_PASSIVE_,
        # [ doc = "internal MCOA0." ]
        INTERNAL_MCOA0_,
    }
    impl CCPA0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CCPA0W::MCOA0_PASSIVE_ => false,
                CCPA0W::INTERNAL_MCOA0_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCPA0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCPA0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CCPA0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "MCOA0 passive." ]
        # [ inline ( always ) ]
        pub fn mcoa0_passive_(self) -> &'a mut W {
            self.variant(CCPA0W::MCOA0_PASSIVE_)
        }
        # [ doc = "internal MCOA0." ]
        # [ inline ( always ) ]
        pub fn internal_mcoa0_(self) -> &'a mut W {
            self.variant(CCPA0W::INTERNAL_MCOA0_)
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
    # [ doc = "Values that can be written to the field `CCPB0`" ]
    pub enum CCPB0W {
        # [ doc = "MCOB0 passive." ]
        MCOB0_PASSIVE_,
        # [ doc = "MCOB0 tracks internal MCOA0." ]
        MCOB0_TRACKS_INTERNA,
    }
    impl CCPB0W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CCPB0W::MCOB0_PASSIVE_ => false,
                CCPB0W::MCOB0_TRACKS_INTERNA => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCPB0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCPB0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CCPB0W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "MCOB0 passive." ]
        # [ inline ( always ) ]
        pub fn mcob0_passive_(self) -> &'a mut W {
            self.variant(CCPB0W::MCOB0_PASSIVE_)
        }
        # [ doc = "MCOB0 tracks internal MCOA0." ]
        # [ inline ( always ) ]
        pub fn mcob0_tracks_interna(self) -> &'a mut W {
            self.variant(CCPB0W::MCOB0_TRACKS_INTERNA)
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
    # [ doc = "Values that can be written to the field `CCPA1`" ]
    pub enum CCPA1W {
        # [ doc = "MCOA1 passive." ]
        MCOA1_PASSIVE_,
        # [ doc = "MCOA1 tracks internal MCOA0." ]
        MCOA1_TRACKS_INTERNA,
    }
    impl CCPA1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CCPA1W::MCOA1_PASSIVE_ => false,
                CCPA1W::MCOA1_TRACKS_INTERNA => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCPA1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCPA1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CCPA1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "MCOA1 passive." ]
        # [ inline ( always ) ]
        pub fn mcoa1_passive_(self) -> &'a mut W {
            self.variant(CCPA1W::MCOA1_PASSIVE_)
        }
        # [ doc = "MCOA1 tracks internal MCOA0." ]
        # [ inline ( always ) ]
        pub fn mcoa1_tracks_interna(self) -> &'a mut W {
            self.variant(CCPA1W::MCOA1_TRACKS_INTERNA)
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
    # [ doc = "Values that can be written to the field `CCPB1`" ]
    pub enum CCPB1W {
        # [ doc = "MCOB1 passive." ]
        MCOB1_PASSIVE_,
        # [ doc = "MCOB1 tracks internal MCOA0." ]
        MCOB1_TRACKS_INTERNA,
    }
    impl CCPB1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CCPB1W::MCOB1_PASSIVE_ => false,
                CCPB1W::MCOB1_TRACKS_INTERNA => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCPB1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCPB1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CCPB1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "MCOB1 passive." ]
        # [ inline ( always ) ]
        pub fn mcob1_passive_(self) -> &'a mut W {
            self.variant(CCPB1W::MCOB1_PASSIVE_)
        }
        # [ doc = "MCOB1 tracks internal MCOA0." ]
        # [ inline ( always ) ]
        pub fn mcob1_tracks_interna(self) -> &'a mut W {
            self.variant(CCPB1W::MCOB1_TRACKS_INTERNA)
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
    # [ doc = "Values that can be written to the field `CCPA2`" ]
    pub enum CCPA2W {
        # [ doc = "MCOA2 passive." ]
        MCOA2_PASSIVE_,
        # [ doc = "MCOA2 tracks internal MCOA0." ]
        MCOA2_TRACKS_INTERNA,
    }
    impl CCPA2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CCPA2W::MCOA2_PASSIVE_ => false,
                CCPA2W::MCOA2_TRACKS_INTERNA => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCPA2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCPA2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CCPA2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "MCOA2 passive." ]
        # [ inline ( always ) ]
        pub fn mcoa2_passive_(self) -> &'a mut W {
            self.variant(CCPA2W::MCOA2_PASSIVE_)
        }
        # [ doc = "MCOA2 tracks internal MCOA0." ]
        # [ inline ( always ) ]
        pub fn mcoa2_tracks_interna(self) -> &'a mut W {
            self.variant(CCPA2W::MCOA2_TRACKS_INTERNA)
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
    # [ doc = "Values that can be written to the field `CCPB2`" ]
    pub enum CCPB2W {
        # [ doc = "MCOB2 passive." ]
        MCOB2_PASSIVE_,
        # [ doc = "MCOB2 tracks internal MCOA0." ]
        MCOB2_TRACKS_INTERNA,
    }
    impl CCPB2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CCPB2W::MCOB2_PASSIVE_ => false,
                CCPB2W::MCOB2_TRACKS_INTERNA => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CCPB2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCPB2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CCPB2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "MCOB2 passive." ]
        # [ inline ( always ) ]
        pub fn mcob2_passive_(self) -> &'a mut W {
            self.variant(CCPB2W::MCOB2_PASSIVE_)
        }
        # [ doc = "MCOB2 tracks internal MCOA0." ]
        # [ inline ( always ) ]
        pub fn mcob2_tracks_interna(self) -> &'a mut W {
            self.variant(CCPB2W::MCOB2_TRACKS_INTERNA)
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
        # [ doc = "Bit 0 - Communication pattern output A, channel 0." ]
        # [ inline ( always ) ]
        pub fn ccpa0(&self) -> CCPA0R {
            CCPA0R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 1 - Communication pattern output B, channel 0." ]
        # [ inline ( always ) ]
        pub fn ccpb0(&self) -> CCPB0R {
            CCPB0R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 1;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 2 - Communication pattern output A, channel 1." ]
        # [ inline ( always ) ]
        pub fn ccpa1(&self) -> CCPA1R {
            CCPA1R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 2;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 3 - Communication pattern output B, channel 1." ]
        # [ inline ( always ) ]
        pub fn ccpb1(&self) -> CCPB1R {
            CCPB1R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 3;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 4 - Communication pattern output A, channel 2." ]
        # [ inline ( always ) ]
        pub fn ccpa2(&self) -> CCPA2R {
            CCPA2R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 4;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 5 - Communication pattern output B, channel 2." ]
        # [ inline ( always ) ]
        pub fn ccpb2(&self) -> CCPB2R {
            CCPB2R::_from({
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
        # [ doc = "Bit 0 - Communication pattern output A, channel 0." ]
        # [ inline ( always ) ]
        pub fn ccpa0(&mut self) -> _CCPA0W {
            _CCPA0W { w: self }
        }
        # [ doc = "Bit 1 - Communication pattern output B, channel 0." ]
        # [ inline ( always ) ]
        pub fn ccpb0(&mut self) -> _CCPB0W {
            _CCPB0W { w: self }
        }
        # [ doc = "Bit 2 - Communication pattern output A, channel 1." ]
        # [ inline ( always ) ]
        pub fn ccpa1(&mut self) -> _CCPA1W {
            _CCPA1W { w: self }
        }
        # [ doc = "Bit 3 - Communication pattern output B, channel 1." ]
        # [ inline ( always ) ]
        pub fn ccpb1(&mut self) -> _CCPB1W {
            _CCPB1W { w: self }
        }
        # [ doc = "Bit 4 - Communication pattern output A, channel 2." ]
        # [ inline ( always ) ]
        pub fn ccpa2(&mut self) -> _CCPA2W {
            _CCPA2W { w: self }
        }
        # [ doc = "Bit 5 - Communication pattern output B, channel 2." ]
        # [ inline ( always ) ]
        pub fn ccpb2(&mut self) -> _CCPB2W {
            _CCPB2W { w: self }
        }
    }
}
# [ doc = "Capture register" ]
pub struct CAP {
    register: VolatileCell<u32>,
}
# [ doc = "Capture register" ]
pub mod cap {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::CAP {
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
        # [ doc = "Bits 0:31 - Current TC value at a capture event." ]
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
# [ doc = "Interrupt Enable read address" ]
pub struct INTEN {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt Enable read address" ]
pub mod inten {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::INTEN {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = "Possible values of the field `ILIM0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ILIM0R {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl ILIM0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ILIM0R::INTERRUPT_DISABLED_ => false,
                ILIM0R::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ILIM0R {
            match value {
                false => ILIM0R::INTERRUPT_DISABLED_,
                true => ILIM0R::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == ILIM0R::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == ILIM0R::INTERRUPT_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `IMAT0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum IMAT0R {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl IMAT0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                IMAT0R::INTERRUPT_DISABLED_ => false,
                IMAT0R::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> IMAT0R {
            match value {
                false => IMAT0R::INTERRUPT_DISABLED_,
                true => IMAT0R::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == IMAT0R::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == IMAT0R::INTERRUPT_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `ICAP0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ICAP0R {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl ICAP0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ICAP0R::INTERRUPT_DISABLED_ => false,
                ICAP0R::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ICAP0R {
            match value {
                false => ICAP0R::INTERRUPT_DISABLED_,
                true => ICAP0R::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == ICAP0R::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == ICAP0R::INTERRUPT_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `ILIM1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ILIM1R {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl ILIM1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ILIM1R::INTERRUPT_DISABLED_ => false,
                ILIM1R::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ILIM1R {
            match value {
                false => ILIM1R::INTERRUPT_DISABLED_,
                true => ILIM1R::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == ILIM1R::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == ILIM1R::INTERRUPT_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `IMAT1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum IMAT1R {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl IMAT1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                IMAT1R::INTERRUPT_DISABLED_ => false,
                IMAT1R::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> IMAT1R {
            match value {
                false => IMAT1R::INTERRUPT_DISABLED_,
                true => IMAT1R::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == IMAT1R::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == IMAT1R::INTERRUPT_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `ICAP1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ICAP1R {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl ICAP1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ICAP1R::INTERRUPT_DISABLED_ => false,
                ICAP1R::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ICAP1R {
            match value {
                false => ICAP1R::INTERRUPT_DISABLED_,
                true => ICAP1R::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == ICAP1R::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == ICAP1R::INTERRUPT_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `ILIM2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ILIM2R {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl ILIM2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ILIM2R::INTERRUPT_DISABLED_ => false,
                ILIM2R::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ILIM2R {
            match value {
                false => ILIM2R::INTERRUPT_DISABLED_,
                true => ILIM2R::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == ILIM2R::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == ILIM2R::INTERRUPT_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `IMAT2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum IMAT2R {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl IMAT2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                IMAT2R::INTERRUPT_DISABLED_ => false,
                IMAT2R::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> IMAT2R {
            match value {
                false => IMAT2R::INTERRUPT_DISABLED_,
                true => IMAT2R::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == IMAT2R::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == IMAT2R::INTERRUPT_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `ICAP2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ICAP2R {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl ICAP2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ICAP2R::INTERRUPT_DISABLED_ => false,
                ICAP2R::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ICAP2R {
            match value {
                false => ICAP2R::INTERRUPT_DISABLED_,
                true => ICAP2R::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == ICAP2R::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == ICAP2R::INTERRUPT_ENABLED_
        }
    }
    # [ doc = "Possible values of the field `ABORT`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ABORTR {
        # [ doc = "Interrupt disabled." ]
        INTERRUPT_DISABLED_,
        # [ doc = "Interrupt enabled." ]
        INTERRUPT_ENABLED_,
    }
    impl ABORTR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ABORTR::INTERRUPT_DISABLED_ => false,
                ABORTR::INTERRUPT_ENABLED_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABORTR {
            match value {
                false => ABORTR::INTERRUPT_DISABLED_,
                true => ABORTR::INTERRUPT_ENABLED_,
            }
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_disabled_(&self) -> bool {
            *self == ABORTR::INTERRUPT_DISABLED_
        }
        # [ doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`" ]
        # [ inline ( always ) ]
        pub fn is_interrupt_enabled_(&self) -> bool {
            *self == ABORTR::INTERRUPT_ENABLED_
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Limit interrupt for channel 0." ]
        # [ inline ( always ) ]
        pub fn ilim0(&self) -> ILIM0R {
            ILIM0R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 0;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 1 - Match interrupt for channel 0." ]
        # [ inline ( always ) ]
        pub fn imat0(&self) -> IMAT0R {
            IMAT0R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 1;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 2 - Capture interrupt for channel 0." ]
        # [ inline ( always ) ]
        pub fn icap0(&self) -> ICAP0R {
            ICAP0R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 2;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 4 - Limit interrupt for channel 1." ]
        # [ inline ( always ) ]
        pub fn ilim1(&self) -> ILIM1R {
            ILIM1R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 4;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 5 - Match interrupt for channel 1." ]
        # [ inline ( always ) ]
        pub fn imat1(&self) -> IMAT1R {
            IMAT1R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 5;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 6 - Capture interrupt for channel 1." ]
        # [ inline ( always ) ]
        pub fn icap1(&self) -> ICAP1R {
            ICAP1R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 6;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 8 - Limit interrupt for channel 2." ]
        # [ inline ( always ) ]
        pub fn ilim2(&self) -> ILIM2R {
            ILIM2R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 8;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 9 - Match interrupt for channel 2." ]
        # [ inline ( always ) ]
        pub fn imat2(&self) -> IMAT2R {
            IMAT2R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 9;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 10 - Capture interrupt for channel 2." ]
        # [ inline ( always ) ]
        pub fn icap2(&self) -> ICAP2R {
            ICAP2R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 10;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 15 - Fast abort interrupt." ]
        # [ inline ( always ) ]
        pub fn abort(&self) -> ABORTR {
            ABORTR::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 15;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
    }
}
# [ doc = "Interrupt Enable set address" ]
pub struct INTEN_SET {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt Enable set address" ]
pub mod inten_set {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::INTEN_SET {
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
    pub struct _ILIM0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ILIM1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ILIM2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ABORT_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABORT_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim0_set(&mut self) -> _ILIM0_SETW {
            _ILIM0_SETW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn imat0_set(&mut self) -> _IMAT0_SETW {
            _IMAT0_SETW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn icap0_set(&mut self) -> _ICAP0_SETW {
            _ICAP0_SETW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim1_set(&mut self) -> _ILIM1_SETW {
            _ILIM1_SETW { w: self }
        }
        # [ doc = "Bit 5 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn imat1_set(&mut self) -> _IMAT1_SETW {
            _IMAT1_SETW { w: self }
        }
        # [ doc = "Bit 6 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn icap1_set(&mut self) -> _ICAP1_SETW {
            _ICAP1_SETW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim2_set(&mut self) -> _ILIM2_SETW {
            _ILIM2_SETW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn imat2_set(&mut self) -> _IMAT2_SETW {
            _IMAT2_SETW { w: self }
        }
        # [ doc = "Bit 11 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn icap2_set(&mut self) -> _ICAP2_SETW {
            _ICAP2_SETW { w: self }
        }
        # [ doc = "Bit 15 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn abort_set(&mut self) -> _ABORT_SETW {
            _ABORT_SETW { w: self }
        }
    }
}
# [ doc = "Interrupt Enable clear address" ]
pub struct INTEN_CLR {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt Enable clear address" ]
pub mod inten_clr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::INTEN_CLR {
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
    pub struct _ILIM0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ILIM1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ILIM2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ABORT_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABORT_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim0_clr(&mut self) -> _ILIM0_CLRW {
            _ILIM0_CLRW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn imat0_clr(&mut self) -> _IMAT0_CLRW {
            _IMAT0_CLRW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn icap0_clr(&mut self) -> _ICAP0_CLRW {
            _ICAP0_CLRW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim1_clr(&mut self) -> _ILIM1_CLRW {
            _ILIM1_CLRW { w: self }
        }
        # [ doc = "Bit 5 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn imat1_clr(&mut self) -> _IMAT1_CLRW {
            _IMAT1_CLRW { w: self }
        }
        # [ doc = "Bit 6 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn icap1_clr(&mut self) -> _ICAP1_CLRW {
            _ICAP1_CLRW { w: self }
        }
        # [ doc = "Bit 8 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim2_clr(&mut self) -> _ILIM2_CLRW {
            _ILIM2_CLRW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn imat2_clr(&mut self) -> _IMAT2_CLRW {
            _IMAT2_CLRW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn icap2_clr(&mut self) -> _ICAP2_CLRW {
            _ICAP2_CLRW { w: self }
        }
        # [ doc = "Bit 15 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn abort_clr(&mut self) -> _ABORT_CLRW {
            _ABORT_CLRW { w: self }
        }
    }
}
# [ doc = "Interrupt flags read address" ]
pub struct INTF {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt flags read address" ]
pub mod intf {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::INTF {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = "Possible values of the field `ILIM0_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ILIM0_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl ILIM0_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ILIM0_FR::THIS_INTERRUPT_SOURC => false,
                ILIM0_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ILIM0_FR {
            match value {
                false => ILIM0_FR::THIS_INTERRUPT_SOURC,
                true => ILIM0_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == ILIM0_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == ILIM0_FR::IF_THE_CORRESPONDING
        }
    }
    # [ doc = "Possible values of the field `IMAT0_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum IMAT0_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl IMAT0_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                IMAT0_FR::THIS_INTERRUPT_SOURC => false,
                IMAT0_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> IMAT0_FR {
            match value {
                false => IMAT0_FR::THIS_INTERRUPT_SOURC,
                true => IMAT0_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == IMAT0_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == IMAT0_FR::IF_THE_CORRESPONDING
        }
    }
    # [ doc = "Possible values of the field `ICAP0_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ICAP0_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl ICAP0_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ICAP0_FR::THIS_INTERRUPT_SOURC => false,
                ICAP0_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ICAP0_FR {
            match value {
                false => ICAP0_FR::THIS_INTERRUPT_SOURC,
                true => ICAP0_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == ICAP0_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == ICAP0_FR::IF_THE_CORRESPONDING
        }
    }
    # [ doc = "Possible values of the field `ILIM1_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ILIM1_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl ILIM1_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ILIM1_FR::THIS_INTERRUPT_SOURC => false,
                ILIM1_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ILIM1_FR {
            match value {
                false => ILIM1_FR::THIS_INTERRUPT_SOURC,
                true => ILIM1_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == ILIM1_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == ILIM1_FR::IF_THE_CORRESPONDING
        }
    }
    # [ doc = "Possible values of the field `IMAT1_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum IMAT1_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl IMAT1_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                IMAT1_FR::THIS_INTERRUPT_SOURC => false,
                IMAT1_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> IMAT1_FR {
            match value {
                false => IMAT1_FR::THIS_INTERRUPT_SOURC,
                true => IMAT1_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == IMAT1_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == IMAT1_FR::IF_THE_CORRESPONDING
        }
    }
    # [ doc = "Possible values of the field `ICAP1_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ICAP1_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl ICAP1_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ICAP1_FR::THIS_INTERRUPT_SOURC => false,
                ICAP1_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ICAP1_FR {
            match value {
                false => ICAP1_FR::THIS_INTERRUPT_SOURC,
                true => ICAP1_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == ICAP1_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == ICAP1_FR::IF_THE_CORRESPONDING
        }
    }
    # [ doc = "Possible values of the field `ILIM2_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ILIM2_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl ILIM2_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ILIM2_FR::THIS_INTERRUPT_SOURC => false,
                ILIM2_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ILIM2_FR {
            match value {
                false => ILIM2_FR::THIS_INTERRUPT_SOURC,
                true => ILIM2_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == ILIM2_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == ILIM2_FR::IF_THE_CORRESPONDING
        }
    }
    # [ doc = "Possible values of the field `IMAT2_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum IMAT2_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl IMAT2_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                IMAT2_FR::THIS_INTERRUPT_SOURC => false,
                IMAT2_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> IMAT2_FR {
            match value {
                false => IMAT2_FR::THIS_INTERRUPT_SOURC,
                true => IMAT2_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == IMAT2_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == IMAT2_FR::IF_THE_CORRESPONDING
        }
    }
    # [ doc = "Possible values of the field `ICAP2_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ICAP2_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl ICAP2_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ICAP2_FR::THIS_INTERRUPT_SOURC => false,
                ICAP2_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ICAP2_FR {
            match value {
                false => ICAP2_FR::THIS_INTERRUPT_SOURC,
                true => ICAP2_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == ICAP2_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == ICAP2_FR::IF_THE_CORRESPONDING
        }
    }
    # [ doc = "Possible values of the field `ABORT_F`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ABORT_FR {
        # [ doc = "This interrupt source is not contributing to the MCPWM interrupt request." ]
        THIS_INTERRUPT_SOURC,
        # [ doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller." ]
        IF_THE_CORRESPONDING,
    }
    impl ABORT_FR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ABORT_FR::THIS_INTERRUPT_SOURC => false,
                ABORT_FR::IF_THE_CORRESPONDING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ABORT_FR {
            match value {
                false => ABORT_FR::THIS_INTERRUPT_SOURC,
                true => ABORT_FR::IF_THE_CORRESPONDING,
            }
        }
        # [ doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`" ]
        # [ inline ( always ) ]
        pub fn is_this_interrupt_sourc(&self) -> bool {
            *self == ABORT_FR::THIS_INTERRUPT_SOURC
        }
        # [ doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`" ]
        # [ inline ( always ) ]
        pub fn is_if_the_corresponding(&self) -> bool {
            *self == ABORT_FR::IF_THE_CORRESPONDING
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Limit interrupt flag for channel 0." ]
        # [ inline ( always ) ]
        pub fn ilim0_f(&self) -> ILIM0_FR {
            ILIM0_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 0;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 1 - Match interrupt flag for channel 0." ]
        # [ inline ( always ) ]
        pub fn imat0_f(&self) -> IMAT0_FR {
            IMAT0_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 1;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 2 - Capture interrupt flag for channel 0." ]
        # [ inline ( always ) ]
        pub fn icap0_f(&self) -> ICAP0_FR {
            ICAP0_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 2;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 4 - Limit interrupt flag for channel 1." ]
        # [ inline ( always ) ]
        pub fn ilim1_f(&self) -> ILIM1_FR {
            ILIM1_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 4;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 5 - Match interrupt flag for channel 1." ]
        # [ inline ( always ) ]
        pub fn imat1_f(&self) -> IMAT1_FR {
            IMAT1_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 5;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 6 - Capture interrupt flag for channel 1." ]
        # [ inline ( always ) ]
        pub fn icap1_f(&self) -> ICAP1_FR {
            ICAP1_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 6;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 8 - Limit interrupt flag for channel 2." ]
        # [ inline ( always ) ]
        pub fn ilim2_f(&self) -> ILIM2_FR {
            ILIM2_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 8;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 9 - Match interrupt flag for channel 2." ]
        # [ inline ( always ) ]
        pub fn imat2_f(&self) -> IMAT2_FR {
            IMAT2_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 9;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 10 - Capture interrupt flag for channel 2." ]
        # [ inline ( always ) ]
        pub fn icap2_f(&self) -> ICAP2_FR {
            ICAP2_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 10;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
        # [ doc = "Bit 15 - Fast abort interrupt flag." ]
        # [ inline ( always ) ]
        pub fn abort_f(&self) -> ABORT_FR {
            ABORT_FR::_from({
                                const MASK: bool = true;
                                const OFFSET: u8 = 15;
                                ((self.bits >> OFFSET) & MASK as u32) != 0
                            })
        }
    }
}
# [ doc = "Interrupt flags set address" ]
pub struct INTF_SET {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt flags set address" ]
pub mod intf_set {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::INTF_SET {
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
    pub struct _ILIM0_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM0_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT0_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT0_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP0_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP0_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ILIM1_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM1_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT1_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT1_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP1_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP1_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ILIM2_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM2_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT2_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT2_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP2_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP2_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ABORT_F_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABORT_F_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim0_f_set(&mut self) -> _ILIM0_F_SETW {
            _ILIM0_F_SETW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn imat0_f_set(&mut self) -> _IMAT0_F_SETW {
            _IMAT0_F_SETW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn icap0_f_set(&mut self) -> _ICAP0_F_SETW {
            _ICAP0_F_SETW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim1_f_set(&mut self) -> _ILIM1_F_SETW {
            _ILIM1_F_SETW { w: self }
        }
        # [ doc = "Bit 5 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn imat1_f_set(&mut self) -> _IMAT1_F_SETW {
            _IMAT1_F_SETW { w: self }
        }
        # [ doc = "Bit 6 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn icap1_f_set(&mut self) -> _ICAP1_F_SETW {
            _ICAP1_F_SETW { w: self }
        }
        # [ doc = "Bit 8 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim2_f_set(&mut self) -> _ILIM2_F_SETW {
            _ILIM2_F_SETW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn imat2_f_set(&mut self) -> _IMAT2_F_SETW {
            _IMAT2_F_SETW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn icap2_f_set(&mut self) -> _ICAP2_F_SETW {
            _ICAP2_F_SETW { w: self }
        }
        # [ doc = "Bit 15 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt." ]
        # [ inline ( always ) ]
        pub fn abort_f_set(&mut self) -> _ABORT_F_SETW {
            _ABORT_F_SETW { w: self }
        }
    }
}
# [ doc = "Interrupt flags clear address" ]
pub struct INTF_CLR {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt flags clear address" ]
pub mod intf_clr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::INTF_CLR {
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
    pub struct _ILIM0_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM0_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT0_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT0_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP0_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP0_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ILIM1_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM1_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT1_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT1_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP1_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP1_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ILIM2_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ILIM2_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IMAT2_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IMAT2_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ICAP2_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ICAP2_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ABORT_F_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ABORT_F_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one clears the corresponding bit in the INTF register, thus clearing the corresponding interrupt request." ]
        # [ inline ( always ) ]
        pub fn ilim0_f_clr(&mut self) -> _ILIM0_F_CLRW {
            _ILIM0_F_CLRW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn imat0_f_clr(&mut self) -> _IMAT0_F_CLRW {
            _IMAT0_F_CLRW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn icap0_f_clr(&mut self) -> _ICAP0_F_CLRW {
            _ICAP0_F_CLRW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim1_f_clr(&mut self) -> _ILIM1_F_CLRW {
            _ILIM1_F_CLRW { w: self }
        }
        # [ doc = "Bit 5 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn imat1_f_clr(&mut self) -> _IMAT1_F_CLRW {
            _IMAT1_F_CLRW { w: self }
        }
        # [ doc = "Bit 6 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn icap1_f_clr(&mut self) -> _ICAP1_F_CLRW {
            _ICAP1_F_CLRW { w: self }
        }
        # [ doc = "Bit 8 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn ilim2_f_clr(&mut self) -> _ILIM2_F_CLRW {
            _ILIM2_F_CLRW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn imat2_f_clr(&mut self) -> _IMAT2_F_CLRW {
            _IMAT2_F_CLRW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn icap2_f_clr(&mut self) -> _ICAP2_F_CLRW {
            _ICAP2_F_CLRW { w: self }
        }
        # [ doc = "Bit 15 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt." ]
        # [ inline ( always ) ]
        pub fn abort_f_clr(&mut self) -> _ABORT_F_CLRW {
            _ABORT_F_CLRW { w: self }
        }
    }
}
# [ doc = "Count Control read address" ]
pub struct CNTCON {
    register: VolatileCell<u32>,
}
# [ doc = "Count Control read address" ]
pub mod cntcon {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::CNTCON {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = "Possible values of the field `TC0MCI0_RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC0MCI0_RER {
        # [ doc = "A rising edge on MCI0 does not affect counter 0." ]
        A_RISING_EDGE_ON_MCI,
        # [ doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI0." ]
        RISING,
    }
    impl TC0MCI0_RER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC0MCI0_RER::A_RISING_EDGE_ON_MCI => false,
                TC0MCI0_RER::RISING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC0MCI0_RER {
            match value {
                false => TC0MCI0_RER::A_RISING_EDGE_ON_MCI,
                true => TC0MCI0_RER::RISING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`" ]
        # [ inline ( always ) ]
        pub fn is_a_rising_edge_on_mci(&self) -> bool {
            *self == TC0MCI0_RER::A_RISING_EDGE_ON_MCI
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == TC0MCI0_RER::RISING
        }
    }
    # [ doc = "Possible values of the field `TC0MCI0_FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC0MCI0_FER {
        # [ doc = "A falling edge on MCI0 does not affect counter 0." ]
        A_FALLING_EDGE_ON_MC,
        # [ doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI0." ]
        FALLING,
    }
    impl TC0MCI0_FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC0MCI0_FER::A_FALLING_EDGE_ON_MC => false,
                TC0MCI0_FER::FALLING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC0MCI0_FER {
            match value {
                false => TC0MCI0_FER::A_FALLING_EDGE_ON_MC,
                true => TC0MCI0_FER::FALLING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`" ]
        # [ inline ( always ) ]
        pub fn is_a_falling_edge_on_mc(&self) -> bool {
            *self == TC0MCI0_FER::A_FALLING_EDGE_ON_MC
        }
        # [ doc = "Checks if the value of the field is `FALLING`" ]
        # [ inline ( always ) ]
        pub fn is_falling(&self) -> bool {
            *self == TC0MCI0_FER::FALLING
        }
    }
    # [ doc = "Possible values of the field `TC0MCI1_RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC0MCI1_RER {
        # [ doc = "A rising edge on MCI1 does not affect counter 0." ]
        A_RISING_EDGE_ON_MCI,
        # [ doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI1." ]
        RISING,
    }
    impl TC0MCI1_RER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC0MCI1_RER::A_RISING_EDGE_ON_MCI => false,
                TC0MCI1_RER::RISING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC0MCI1_RER {
            match value {
                false => TC0MCI1_RER::A_RISING_EDGE_ON_MCI,
                true => TC0MCI1_RER::RISING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`" ]
        # [ inline ( always ) ]
        pub fn is_a_rising_edge_on_mci(&self) -> bool {
            *self == TC0MCI1_RER::A_RISING_EDGE_ON_MCI
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == TC0MCI1_RER::RISING
        }
    }
    # [ doc = "Possible values of the field `TC0MCI1_FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC0MCI1_FER {
        # [ doc = "A falling edge on MCI1 does not affect counter 0." ]
        A_FALLING_EDGE_ON_MC,
        # [ doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI1." ]
        FALLING,
    }
    impl TC0MCI1_FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC0MCI1_FER::A_FALLING_EDGE_ON_MC => false,
                TC0MCI1_FER::FALLING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC0MCI1_FER {
            match value {
                false => TC0MCI1_FER::A_FALLING_EDGE_ON_MC,
                true => TC0MCI1_FER::FALLING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`" ]
        # [ inline ( always ) ]
        pub fn is_a_falling_edge_on_mc(&self) -> bool {
            *self == TC0MCI1_FER::A_FALLING_EDGE_ON_MC
        }
        # [ doc = "Checks if the value of the field is `FALLING`" ]
        # [ inline ( always ) ]
        pub fn is_falling(&self) -> bool {
            *self == TC0MCI1_FER::FALLING
        }
    }
    # [ doc = "Possible values of the field `TC0MCI2_RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC0MCI2_RER {
        # [ doc = "A rising edge on MCI0 does not affect counter 0." ]
        A_RISING_EDGE_ON_MCI,
        # [ doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI2." ]
        RISING,
    }
    impl TC0MCI2_RER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC0MCI2_RER::A_RISING_EDGE_ON_MCI => false,
                TC0MCI2_RER::RISING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC0MCI2_RER {
            match value {
                false => TC0MCI2_RER::A_RISING_EDGE_ON_MCI,
                true => TC0MCI2_RER::RISING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`" ]
        # [ inline ( always ) ]
        pub fn is_a_rising_edge_on_mci(&self) -> bool {
            *self == TC0MCI2_RER::A_RISING_EDGE_ON_MCI
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == TC0MCI2_RER::RISING
        }
    }
    # [ doc = "Possible values of the field `TC0MCI2_FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC0MCI2_FER {
        # [ doc = "A falling edge on MCI0 does not affect counter 0." ]
        A_FALLING_EDGE_ON_MC,
        # [ doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI2." ]
        FALLLING,
    }
    impl TC0MCI2_FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC0MCI2_FER::A_FALLING_EDGE_ON_MC => false,
                TC0MCI2_FER::FALLLING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC0MCI2_FER {
            match value {
                false => TC0MCI2_FER::A_FALLING_EDGE_ON_MC,
                true => TC0MCI2_FER::FALLLING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`" ]
        # [ inline ( always ) ]
        pub fn is_a_falling_edge_on_mc(&self) -> bool {
            *self == TC0MCI2_FER::A_FALLING_EDGE_ON_MC
        }
        # [ doc = "Checks if the value of the field is `FALLLING`" ]
        # [ inline ( always ) ]
        pub fn is_fallling(&self) -> bool {
            *self == TC0MCI2_FER::FALLLING
        }
    }
    # [ doc = "Possible values of the field `TC1MCI0_RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC1MCI0_RER {
        # [ doc = "A rising edge on MCI0 does not affect counter 1." ]
        A_RISING_EDGE_ON_MCI,
        # [ doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI0." ]
        RISING,
    }
    impl TC1MCI0_RER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC1MCI0_RER::A_RISING_EDGE_ON_MCI => false,
                TC1MCI0_RER::RISING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC1MCI0_RER {
            match value {
                false => TC1MCI0_RER::A_RISING_EDGE_ON_MCI,
                true => TC1MCI0_RER::RISING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`" ]
        # [ inline ( always ) ]
        pub fn is_a_rising_edge_on_mci(&self) -> bool {
            *self == TC1MCI0_RER::A_RISING_EDGE_ON_MCI
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == TC1MCI0_RER::RISING
        }
    }
    # [ doc = "Possible values of the field `TC1MCI0_FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC1MCI0_FER {
        # [ doc = "A falling edge on MCI0 does not affect counter 1." ]
        A_FALLING_EDGE_ON_MC,
        # [ doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI0." ]
        FALLING,
    }
    impl TC1MCI0_FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC1MCI0_FER::A_FALLING_EDGE_ON_MC => false,
                TC1MCI0_FER::FALLING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC1MCI0_FER {
            match value {
                false => TC1MCI0_FER::A_FALLING_EDGE_ON_MC,
                true => TC1MCI0_FER::FALLING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`" ]
        # [ inline ( always ) ]
        pub fn is_a_falling_edge_on_mc(&self) -> bool {
            *self == TC1MCI0_FER::A_FALLING_EDGE_ON_MC
        }
        # [ doc = "Checks if the value of the field is `FALLING`" ]
        # [ inline ( always ) ]
        pub fn is_falling(&self) -> bool {
            *self == TC1MCI0_FER::FALLING
        }
    }
    # [ doc = "Possible values of the field `TC1MCI1_RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC1MCI1_RER {
        # [ doc = "A rising edge on MCI1 does not affect counter 1." ]
        A_RISING_EDGE_ON_MCI,
        # [ doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI1." ]
        RISING,
    }
    impl TC1MCI1_RER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC1MCI1_RER::A_RISING_EDGE_ON_MCI => false,
                TC1MCI1_RER::RISING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC1MCI1_RER {
            match value {
                false => TC1MCI1_RER::A_RISING_EDGE_ON_MCI,
                true => TC1MCI1_RER::RISING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`" ]
        # [ inline ( always ) ]
        pub fn is_a_rising_edge_on_mci(&self) -> bool {
            *self == TC1MCI1_RER::A_RISING_EDGE_ON_MCI
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == TC1MCI1_RER::RISING
        }
    }
    # [ doc = "Possible values of the field `TC1MCI1_FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC1MCI1_FER {
        # [ doc = "A falling edge on MCI0 does not affect counter 1." ]
        A_FALLING_EDGE_ON_MC,
        # [ doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI1." ]
        FALLING,
    }
    impl TC1MCI1_FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC1MCI1_FER::A_FALLING_EDGE_ON_MC => false,
                TC1MCI1_FER::FALLING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC1MCI1_FER {
            match value {
                false => TC1MCI1_FER::A_FALLING_EDGE_ON_MC,
                true => TC1MCI1_FER::FALLING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`" ]
        # [ inline ( always ) ]
        pub fn is_a_falling_edge_on_mc(&self) -> bool {
            *self == TC1MCI1_FER::A_FALLING_EDGE_ON_MC
        }
        # [ doc = "Checks if the value of the field is `FALLING`" ]
        # [ inline ( always ) ]
        pub fn is_falling(&self) -> bool {
            *self == TC1MCI1_FER::FALLING
        }
    }
    # [ doc = "Possible values of the field `TC1MCI2_RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC1MCI2_RER {
        # [ doc = "A rising edge on MCI2 does not affect counter 1." ]
        A_RISING_EDGE_ON_MCI,
        # [ doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI2." ]
        RISING,
    }
    impl TC1MCI2_RER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC1MCI2_RER::A_RISING_EDGE_ON_MCI => false,
                TC1MCI2_RER::RISING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC1MCI2_RER {
            match value {
                false => TC1MCI2_RER::A_RISING_EDGE_ON_MCI,
                true => TC1MCI2_RER::RISING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`" ]
        # [ inline ( always ) ]
        pub fn is_a_rising_edge_on_mci(&self) -> bool {
            *self == TC1MCI2_RER::A_RISING_EDGE_ON_MCI
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == TC1MCI2_RER::RISING
        }
    }
    # [ doc = "Possible values of the field `TC1MCI2_FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC1MCI2_FER {
        # [ doc = "A falling edge on MCI2 does not affect counter 1." ]
        A_FALLING_EDGE_ON_MC,
        # [ doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI2." ]
        FALLING,
    }
    impl TC1MCI2_FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC1MCI2_FER::A_FALLING_EDGE_ON_MC => false,
                TC1MCI2_FER::FALLING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC1MCI2_FER {
            match value {
                false => TC1MCI2_FER::A_FALLING_EDGE_ON_MC,
                true => TC1MCI2_FER::FALLING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`" ]
        # [ inline ( always ) ]
        pub fn is_a_falling_edge_on_mc(&self) -> bool {
            *self == TC1MCI2_FER::A_FALLING_EDGE_ON_MC
        }
        # [ doc = "Checks if the value of the field is `FALLING`" ]
        # [ inline ( always ) ]
        pub fn is_falling(&self) -> bool {
            *self == TC1MCI2_FER::FALLING
        }
    }
    # [ doc = "Possible values of the field `TC2MCI0_RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC2MCI0_RER {
        # [ doc = "A rising edge on MCI0 does not affect counter 2." ]
        A_RISING_EDGE_ON_MCI,
        # [ doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI0." ]
        RISING,
    }
    impl TC2MCI0_RER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC2MCI0_RER::A_RISING_EDGE_ON_MCI => false,
                TC2MCI0_RER::RISING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC2MCI0_RER {
            match value {
                false => TC2MCI0_RER::A_RISING_EDGE_ON_MCI,
                true => TC2MCI0_RER::RISING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`" ]
        # [ inline ( always ) ]
        pub fn is_a_rising_edge_on_mci(&self) -> bool {
            *self == TC2MCI0_RER::A_RISING_EDGE_ON_MCI
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == TC2MCI0_RER::RISING
        }
    }
    # [ doc = "Possible values of the field `TC2MCI0_FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC2MCI0_FER {
        # [ doc = "A falling edge on MCI0 does not affect counter 2." ]
        A_FALLING_EDGE_ON_MC,
        # [ doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI0." ]
        FALLING,
    }
    impl TC2MCI0_FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC2MCI0_FER::A_FALLING_EDGE_ON_MC => false,
                TC2MCI0_FER::FALLING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC2MCI0_FER {
            match value {
                false => TC2MCI0_FER::A_FALLING_EDGE_ON_MC,
                true => TC2MCI0_FER::FALLING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`" ]
        # [ inline ( always ) ]
        pub fn is_a_falling_edge_on_mc(&self) -> bool {
            *self == TC2MCI0_FER::A_FALLING_EDGE_ON_MC
        }
        # [ doc = "Checks if the value of the field is `FALLING`" ]
        # [ inline ( always ) ]
        pub fn is_falling(&self) -> bool {
            *self == TC2MCI0_FER::FALLING
        }
    }
    # [ doc = "Possible values of the field `TC2MCI1_RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC2MCI1_RER {
        # [ doc = "A rising edge on MCI1 does not affect counter 2." ]
        A_RISING_EDGE_ON_MCI,
        # [ doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI1." ]
        RISING,
    }
    impl TC2MCI1_RER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC2MCI1_RER::A_RISING_EDGE_ON_MCI => false,
                TC2MCI1_RER::RISING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC2MCI1_RER {
            match value {
                false => TC2MCI1_RER::A_RISING_EDGE_ON_MCI,
                true => TC2MCI1_RER::RISING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`" ]
        # [ inline ( always ) ]
        pub fn is_a_rising_edge_on_mci(&self) -> bool {
            *self == TC2MCI1_RER::A_RISING_EDGE_ON_MCI
        }
        # [ doc = "Checks if the value of the field is `RISING`" ]
        # [ inline ( always ) ]
        pub fn is_rising(&self) -> bool {
            *self == TC2MCI1_RER::RISING
        }
    }
    # [ doc = "Possible values of the field `TC2MCI1_FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC2MCI1_FER {
        # [ doc = "A falling edge on MCI1 does not affect counter 2." ]
        A_FALLING_EDGE_ON_MC,
        # [ doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI1." ]
        FALLING,
    }
    impl TC2MCI1_FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC2MCI1_FER::A_FALLING_EDGE_ON_MC => false,
                TC2MCI1_FER::FALLING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC2MCI1_FER {
            match value {
                false => TC2MCI1_FER::A_FALLING_EDGE_ON_MC,
                true => TC2MCI1_FER::FALLING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`" ]
        # [ inline ( always ) ]
        pub fn is_a_falling_edge_on_mc(&self) -> bool {
            *self == TC2MCI1_FER::A_FALLING_EDGE_ON_MC
        }
        # [ doc = "Checks if the value of the field is `FALLING`" ]
        # [ inline ( always ) ]
        pub fn is_falling(&self) -> bool {
            *self == TC2MCI1_FER::FALLING
        }
    }
    # [ doc = "Possible values of the field `TC2MCI2_RE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC2MCI2_RER {
        # [ doc = "A rising edge on MCI2 does not affect counter 2." ]
        A_RISING_EDGE_ON_MCI,
        # [ doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI2." ]
        RISIING,
    }
    impl TC2MCI2_RER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC2MCI2_RER::A_RISING_EDGE_ON_MCI => false,
                TC2MCI2_RER::RISIING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC2MCI2_RER {
            match value {
                false => TC2MCI2_RER::A_RISING_EDGE_ON_MCI,
                true => TC2MCI2_RER::RISIING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`" ]
        # [ inline ( always ) ]
        pub fn is_a_rising_edge_on_mci(&self) -> bool {
            *self == TC2MCI2_RER::A_RISING_EDGE_ON_MCI
        }
        # [ doc = "Checks if the value of the field is `RISIING`" ]
        # [ inline ( always ) ]
        pub fn is_risiing(&self) -> bool {
            *self == TC2MCI2_RER::RISIING
        }
    }
    # [ doc = "Possible values of the field `TC2MCI2_FE`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TC2MCI2_FER {
        # [ doc = "A falling edge on MCI2 does not affect counter 2." ]
        A_FALLING_EDGE_ON_MC,
        # [ doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI2." ]
        FALLING,
    }
    impl TC2MCI2_FER {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TC2MCI2_FER::A_FALLING_EDGE_ON_MC => false,
                TC2MCI2_FER::FALLING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TC2MCI2_FER {
            match value {
                false => TC2MCI2_FER::A_FALLING_EDGE_ON_MC,
                true => TC2MCI2_FER::FALLING,
            }
        }
        # [ doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`" ]
        # [ inline ( always ) ]
        pub fn is_a_falling_edge_on_mc(&self) -> bool {
            *self == TC2MCI2_FER::A_FALLING_EDGE_ON_MC
        }
        # [ doc = "Checks if the value of the field is `FALLING`" ]
        # [ inline ( always ) ]
        pub fn is_falling(&self) -> bool {
            *self == TC2MCI2_FER::FALLING
        }
    }
    # [ doc = "Possible values of the field `CNTR0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CNTR0R {
        # [ doc = "Channel 0 is in timer mode." ]
        CHANNEL_0_IS_IN_TIME,
        # [ doc = "Channel 0 is in counter mode." ]
        CHANNEL_0_IS_IN_COUN,
    }
    impl CNTR0R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CNTR0R::CHANNEL_0_IS_IN_TIME => false,
                CNTR0R::CHANNEL_0_IS_IN_COUN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CNTR0R {
            match value {
                false => CNTR0R::CHANNEL_0_IS_IN_TIME,
                true => CNTR0R::CHANNEL_0_IS_IN_COUN,
            }
        }
        # [ doc = "Checks if the value of the field is `CHANNEL_0_IS_IN_TIME`" ]
        # [ inline ( always ) ]
        pub fn is_channel_0_is_in_time(&self) -> bool {
            *self == CNTR0R::CHANNEL_0_IS_IN_TIME
        }
        # [ doc = "Checks if the value of the field is `CHANNEL_0_IS_IN_COUN`" ]
        # [ inline ( always ) ]
        pub fn is_channel_0_is_in_coun(&self) -> bool {
            *self == CNTR0R::CHANNEL_0_IS_IN_COUN
        }
    }
    # [ doc = "Possible values of the field `CNTR1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CNTR1R {
        # [ doc = "Channel 1 is in timer mode." ]
        CHANNEL_1_IS_IN_TIME,
        # [ doc = "Channel 1 is in counter mode." ]
        CHANNEL_1_IS_IN_COUN,
    }
    impl CNTR1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CNTR1R::CHANNEL_1_IS_IN_TIME => false,
                CNTR1R::CHANNEL_1_IS_IN_COUN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CNTR1R {
            match value {
                false => CNTR1R::CHANNEL_1_IS_IN_TIME,
                true => CNTR1R::CHANNEL_1_IS_IN_COUN,
            }
        }
        # [ doc = "Checks if the value of the field is `CHANNEL_1_IS_IN_TIME`" ]
        # [ inline ( always ) ]
        pub fn is_channel_1_is_in_time(&self) -> bool {
            *self == CNTR1R::CHANNEL_1_IS_IN_TIME
        }
        # [ doc = "Checks if the value of the field is `CHANNEL_1_IS_IN_COUN`" ]
        # [ inline ( always ) ]
        pub fn is_channel_1_is_in_coun(&self) -> bool {
            *self == CNTR1R::CHANNEL_1_IS_IN_COUN
        }
    }
    # [ doc = "Possible values of the field `CNTR2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CNTR2R {
        # [ doc = "Channel 2 is in timer mode." ]
        CHANNEL_2_IS_IN_TIME,
        # [ doc = "Channel 2 is in counter mode." ]
        CHANNEL_2_IS_IN_COUN,
    }
    impl CNTR2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                CNTR2R::CHANNEL_2_IS_IN_TIME => false,
                CNTR2R::CHANNEL_2_IS_IN_COUN => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> CNTR2R {
            match value {
                false => CNTR2R::CHANNEL_2_IS_IN_TIME,
                true => CNTR2R::CHANNEL_2_IS_IN_COUN,
            }
        }
        # [ doc = "Checks if the value of the field is `CHANNEL_2_IS_IN_TIME`" ]
        # [ inline ( always ) ]
        pub fn is_channel_2_is_in_time(&self) -> bool {
            *self == CNTR2R::CHANNEL_2_IS_IN_TIME
        }
        # [ doc = "Checks if the value of the field is `CHANNEL_2_IS_IN_COUN`" ]
        # [ inline ( always ) ]
        pub fn is_channel_2_is_in_coun(&self) -> bool {
            *self == CNTR2R::CHANNEL_2_IS_IN_COUN
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bit 0 - Counter 0 rising edge mode, channel 0." ]
        # [ inline ( always ) ]
        pub fn tc0mci0_re(&self) -> TC0MCI0_RER {
            TC0MCI0_RER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 0;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 1 - Counter 0 falling edge mode, channel 0." ]
        # [ inline ( always ) ]
        pub fn tc0mci0_fe(&self) -> TC0MCI0_FER {
            TC0MCI0_FER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 1;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 2 - Counter 0 rising edge mode, channel 1." ]
        # [ inline ( always ) ]
        pub fn tc0mci1_re(&self) -> TC0MCI1_RER {
            TC0MCI1_RER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 2;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 3 - Counter 0 falling edge mode, channel 1." ]
        # [ inline ( always ) ]
        pub fn tc0mci1_fe(&self) -> TC0MCI1_FER {
            TC0MCI1_FER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 3;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 4 - Counter 0 rising edge mode, channel 2." ]
        # [ inline ( always ) ]
        pub fn tc0mci2_re(&self) -> TC0MCI2_RER {
            TC0MCI2_RER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 4;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 5 - Counter 0 falling edge mode, channel 2." ]
        # [ inline ( always ) ]
        pub fn tc0mci2_fe(&self) -> TC0MCI2_FER {
            TC0MCI2_FER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 5;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 6 - Counter 1 rising edge mode, channel 0." ]
        # [ inline ( always ) ]
        pub fn tc1mci0_re(&self) -> TC1MCI0_RER {
            TC1MCI0_RER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 6;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 7 - Counter 1 falling edge mode, channel 0." ]
        # [ inline ( always ) ]
        pub fn tc1mci0_fe(&self) -> TC1MCI0_FER {
            TC1MCI0_FER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 7;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 8 - Counter 1 rising edge mode, channel 1." ]
        # [ inline ( always ) ]
        pub fn tc1mci1_re(&self) -> TC1MCI1_RER {
            TC1MCI1_RER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 8;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 9 - Counter 1 falling edge mode, channel 1." ]
        # [ inline ( always ) ]
        pub fn tc1mci1_fe(&self) -> TC1MCI1_FER {
            TC1MCI1_FER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 9;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 10 - Counter 1 rising edge mode, channel 2." ]
        # [ inline ( always ) ]
        pub fn tc1mci2_re(&self) -> TC1MCI2_RER {
            TC1MCI2_RER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 10;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 11 - Counter 1 falling edge mode, channel 2." ]
        # [ inline ( always ) ]
        pub fn tc1mci2_fe(&self) -> TC1MCI2_FER {
            TC1MCI2_FER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 11;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 12 - Counter 2 rising edge mode, channel 0." ]
        # [ inline ( always ) ]
        pub fn tc2mci0_re(&self) -> TC2MCI0_RER {
            TC2MCI0_RER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 12;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 13 - Counter 2 falling edge mode, channel 0." ]
        # [ inline ( always ) ]
        pub fn tc2mci0_fe(&self) -> TC2MCI0_FER {
            TC2MCI0_FER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 13;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 14 - Counter 2 rising edge mode, channel 1." ]
        # [ inline ( always ) ]
        pub fn tc2mci1_re(&self) -> TC2MCI1_RER {
            TC2MCI1_RER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 14;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 15 - Counter 2 falling edge mode, channel 1." ]
        # [ inline ( always ) ]
        pub fn tc2mci1_fe(&self) -> TC2MCI1_FER {
            TC2MCI1_FER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 15;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 16 - Counter 2 rising edge mode, channel 2." ]
        # [ inline ( always ) ]
        pub fn tc2mci2_re(&self) -> TC2MCI2_RER {
            TC2MCI2_RER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 16;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 17 - Counter 2 falling edge mode, channel 2." ]
        # [ inline ( always ) ]
        pub fn tc2mci2_fe(&self) -> TC2MCI2_FER {
            TC2MCI2_FER::_from({
                                   const MASK: bool = true;
                                   const OFFSET: u8 = 17;
                                   ((self.bits >> OFFSET) & MASK as u32) != 0
                               })
        }
        # [ doc = "Bit 29 - Channel 0 counter/timer mode." ]
        # [ inline ( always ) ]
        pub fn cntr0(&self) -> CNTR0R {
            CNTR0R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 29;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 30 - Channel 1 counter/timer mode." ]
        # [ inline ( always ) ]
        pub fn cntr1(&self) -> CNTR1R {
            CNTR1R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 30;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 31 - Channel 2 counter/timer mode." ]
        # [ inline ( always ) ]
        pub fn cntr2(&self) -> CNTR2R {
            CNTR2R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 31;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
    }
}
# [ doc = "Count Control set address" ]
pub struct CNTCON_SET {
    register: VolatileCell<u32>,
}
# [ doc = "Count Control set address" ]
pub mod cntcon_set {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CNTCON_SET {
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
    pub struct _TC0MCI0_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI0_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI0_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI0_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI1_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI1_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI1_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI1_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI2_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI2_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI2_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI2_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI0_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI0_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI0_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI0_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI1_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI1_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI1_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI1_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI2_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI2_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI2_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI2_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI0_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI0_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI0_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI0_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI1_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI1_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI1_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI1_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI2_RE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI2_RE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI2_FE_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI2_FE_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CNTR0_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTR0_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CNTR1_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTR1_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CNTR2_SETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTR2_SETW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci0_re_set(&mut self) -> _TC0MCI0_RE_SETW {
            _TC0MCI0_RE_SETW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci0_fe_set(&mut self) -> _TC0MCI0_FE_SETW {
            _TC0MCI0_FE_SETW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci1_re_set(&mut self) -> _TC0MCI1_RE_SETW {
            _TC0MCI1_RE_SETW { w: self }
        }
        # [ doc = "Bit 3 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci1_fe_set(&mut self) -> _TC0MCI1_FE_SETW {
            _TC0MCI1_FE_SETW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci2_re_set(&mut self) -> _TC0MCI2_RE_SETW {
            _TC0MCI2_RE_SETW { w: self }
        }
        # [ doc = "Bit 5 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci2_fe_set(&mut self) -> _TC0MCI2_FE_SETW {
            _TC0MCI2_FE_SETW { w: self }
        }
        # [ doc = "Bit 6 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci0_re_set(&mut self) -> _TC1MCI0_RE_SETW {
            _TC1MCI0_RE_SETW { w: self }
        }
        # [ doc = "Bit 7 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci0_fe_set(&mut self) -> _TC1MCI0_FE_SETW {
            _TC1MCI0_FE_SETW { w: self }
        }
        # [ doc = "Bit 8 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci1_re_set(&mut self) -> _TC1MCI1_RE_SETW {
            _TC1MCI1_RE_SETW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci1_fe_set(&mut self) -> _TC1MCI1_FE_SETW {
            _TC1MCI1_FE_SETW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci2_re_set(&mut self) -> _TC1MCI2_RE_SETW {
            _TC1MCI2_RE_SETW { w: self }
        }
        # [ doc = "Bit 11 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci2_fe_set(&mut self) -> _TC1MCI2_FE_SETW {
            _TC1MCI2_FE_SETW { w: self }
        }
        # [ doc = "Bit 12 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci0_re_set(&mut self) -> _TC2MCI0_RE_SETW {
            _TC2MCI0_RE_SETW { w: self }
        }
        # [ doc = "Bit 13 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci0_fe_set(&mut self) -> _TC2MCI0_FE_SETW {
            _TC2MCI0_FE_SETW { w: self }
        }
        # [ doc = "Bit 14 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci1_re_set(&mut self) -> _TC2MCI1_RE_SETW {
            _TC2MCI1_RE_SETW { w: self }
        }
        # [ doc = "Bit 15 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci1_fe_set(&mut self) -> _TC2MCI1_FE_SETW {
            _TC2MCI1_FE_SETW { w: self }
        }
        # [ doc = "Bit 16 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci2_re_set(&mut self) -> _TC2MCI2_RE_SETW {
            _TC2MCI2_RE_SETW { w: self }
        }
        # [ doc = "Bit 17 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci2_fe_set(&mut self) -> _TC2MCI2_FE_SETW {
            _TC2MCI2_FE_SETW { w: self }
        }
        # [ doc = "Bit 29 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn cntr0_set(&mut self) -> _CNTR0_SETW {
            _CNTR0_SETW { w: self }
        }
        # [ doc = "Bit 30 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn cntr1_set(&mut self) -> _CNTR1_SETW {
            _CNTR1_SETW { w: self }
        }
        # [ doc = "Bit 31 - Writing a one sets the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn cntr2_set(&mut self) -> _CNTR2_SETW {
            _CNTR2_SETW { w: self }
        }
    }
}
# [ doc = "Count Control clear address" ]
pub struct CNTCON_CLR {
    register: VolatileCell<u32>,
}
# [ doc = "Count Control clear address" ]
pub mod cntcon_clr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CNTCON_CLR {
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
    pub struct _TC0MCI0_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI0_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI0_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI0_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI1_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI1_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI1_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI1_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI2_REW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI2_REW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC0MCI2_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC0MCI2_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI0_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI0_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI0_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI0_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI1_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI1_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI1_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI1_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI2_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI2_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC1MCI2_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC1MCI2_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI0_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI0_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI0_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI0_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI1_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI1_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI1_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI1_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI2_RE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI2_RE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC2MCI2_FE_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC2MCI2_FE_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CNTR0_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTR0_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CNTR1_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTR1_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CNTR2_CLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTR2_CLRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci0_re_clr(&mut self) -> _TC0MCI0_RE_CLRW {
            _TC0MCI0_RE_CLRW { w: self }
        }
        # [ doc = "Bit 1 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci0_fe_clr(&mut self) -> _TC0MCI0_FE_CLRW {
            _TC0MCI0_FE_CLRW { w: self }
        }
        # [ doc = "Bit 2 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci1_re_clr(&mut self) -> _TC0MCI1_RE_CLRW {
            _TC0MCI1_RE_CLRW { w: self }
        }
        # [ doc = "Bit 3 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci1_fe_clr(&mut self) -> _TC0MCI1_FE_CLRW {
            _TC0MCI1_FE_CLRW { w: self }
        }
        # [ doc = "Bit 4 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci2_re(&mut self) -> _TC0MCI2_REW {
            _TC0MCI2_REW { w: self }
        }
        # [ doc = "Bit 5 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc0mci2_fe_clr(&mut self) -> _TC0MCI2_FE_CLRW {
            _TC0MCI2_FE_CLRW { w: self }
        }
        # [ doc = "Bit 6 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci0_re_clr(&mut self) -> _TC1MCI0_RE_CLRW {
            _TC1MCI0_RE_CLRW { w: self }
        }
        # [ doc = "Bit 7 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci0_fe_clr(&mut self) -> _TC1MCI0_FE_CLRW {
            _TC1MCI0_FE_CLRW { w: self }
        }
        # [ doc = "Bit 8 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci1_re_clr(&mut self) -> _TC1MCI1_RE_CLRW {
            _TC1MCI1_RE_CLRW { w: self }
        }
        # [ doc = "Bit 9 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci1_fe_clr(&mut self) -> _TC1MCI1_FE_CLRW {
            _TC1MCI1_FE_CLRW { w: self }
        }
        # [ doc = "Bit 10 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci2_re_clr(&mut self) -> _TC1MCI2_RE_CLRW {
            _TC1MCI2_RE_CLRW { w: self }
        }
        # [ doc = "Bit 11 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc1mci2_fe_clr(&mut self) -> _TC1MCI2_FE_CLRW {
            _TC1MCI2_FE_CLRW { w: self }
        }
        # [ doc = "Bit 12 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci0_re_clr(&mut self) -> _TC2MCI0_RE_CLRW {
            _TC2MCI0_RE_CLRW { w: self }
        }
        # [ doc = "Bit 13 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci0_fe_clr(&mut self) -> _TC2MCI0_FE_CLRW {
            _TC2MCI0_FE_CLRW { w: self }
        }
        # [ doc = "Bit 14 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci1_re_clr(&mut self) -> _TC2MCI1_RE_CLRW {
            _TC2MCI1_RE_CLRW { w: self }
        }
        # [ doc = "Bit 15 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci1_fe_clr(&mut self) -> _TC2MCI1_FE_CLRW {
            _TC2MCI1_FE_CLRW { w: self }
        }
        # [ doc = "Bit 16 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci2_re_clr(&mut self) -> _TC2MCI2_RE_CLRW {
            _TC2MCI2_RE_CLRW { w: self }
        }
        # [ doc = "Bit 17 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn tc2mci2_fe_clr(&mut self) -> _TC2MCI2_FE_CLRW {
            _TC2MCI2_FE_CLRW { w: self }
        }
        # [ doc = "Bit 29 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn cntr0_clr(&mut self) -> _CNTR0_CLRW {
            _CNTR0_CLRW { w: self }
        }
        # [ doc = "Bit 30 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn cntr1_clr(&mut self) -> _CNTR1_CLRW {
            _CNTR1_CLRW { w: self }
        }
        # [ doc = "Bit 31 - Writing a one clears the corresponding bit in the CNTCON register." ]
        # [ inline ( always ) ]
        pub fn cntr2_clr(&mut self) -> _CNTR2_CLRW {
            _CNTR2_CLRW { w: self }
        }
    }
}
# [ doc = "Capture clear address" ]
pub struct CAP_CLR {
    register: VolatileCell<u32>,
}
# [ doc = "Capture clear address" ]
pub mod cap_clr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CAP_CLR {
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
    pub struct _CAP_CLR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP_CLR0W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP_CLR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP_CLR1W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CAP_CLR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAP_CLR2W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Writing a 1 to this bit clears the CAP0 register." ]
        # [ inline ( always ) ]
        pub fn cap_clr0(&mut self) -> _CAP_CLR0W {
            _CAP_CLR0W { w: self }
        }
        # [ doc = "Bit 1 - Writing a 1 to this bit clears the CAP1 register." ]
        # [ inline ( always ) ]
        pub fn cap_clr1(&mut self) -> _CAP_CLR1W {
            _CAP_CLR1W { w: self }
        }
        # [ doc = "Bit 2 - Writing a 1 to this bit clears the CAP2 register." ]
        # [ inline ( always ) ]
        pub fn cap_clr2(&mut self) -> _CAP_CLR2W {
            _CAP_CLR2W { w: self }
        }
    }
}
# [ doc = "Motor Control PWM" ]
pub struct MCPWM {
    register_block: RegisterBlock,
}
impl Deref for MCPWM {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
