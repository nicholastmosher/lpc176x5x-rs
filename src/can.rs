# ! [ doc = "CAN1 controller" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "CAN1 controller" ]
pub const CAN1: Peripheral<CAN1> = unsafe { Peripheral::new(1074020352) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - Controls the operating mode of the CAN Controller." ]
    pub mod_: MOD,
    # [ doc = "0x04 - Command bits that affect the state of the CAN Controller" ]
    pub cmr: CMR,
    # [ doc = "0x08 - Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1." ]
    pub gsr: GSR,
    # [ doc = "0x0c - Interrupt status, Arbitration Lost Capture, Error Code Capture" ]
    pub icr: ICR,
    # [ doc = "0x10 - Interrupt Enable" ]
    pub ier: IER,
    # [ doc = "0x14 - Bus Timing. Can only be written when RM in CANMOD is 1." ]
    pub btr: BTR,
    # [ doc = "0x18 - Error Warning Limit. Can only be written when RM in CANMOD is 1." ]
    pub ewl: EWL,
    # [ doc = "0x1c - Status Register" ]
    pub sr: SR,
    # [ doc = "0x20 - Receive frame status. Can only be written when RM in CANMOD is 1." ]
    pub rfs: RFS,
    # [ doc = "0x24 - Received Identifier. Can only be written when RM in CANMOD is 1." ]
    pub rid: RID,
    # [ doc = "0x28 - Received data bytes 1-4. Can only be written when RM in CANMOD is 1." ]
    pub rda: RDA,
    # [ doc = "0x2c - Received data bytes 5-8. Can only be written when RM in CANMOD is 1." ]
    pub rdb: RDB,
    # [ doc = "0x30 - Transmit frame info (Tx Buffer )" ]
    pub tfi1: TFI,
    # [ doc = "0x34 - Transmit Identifier (Tx Buffer)" ]
    pub tid1: TID,
    # [ doc = "0x38 - Transmit data bytes 1-4 (Tx Buffer)" ]
    pub tda1: TDA,
    # [ doc = "0x3c - Transmit data bytes 5-8 (Tx Buffer )" ]
    pub tdb1: TDB,
    # [ doc = "0x40 - Transmit frame info (Tx Buffer )" ]
    pub tfi2: TFI,
    # [ doc = "0x44 - Transmit Identifier (Tx Buffer)" ]
    pub tid2: TID,
    # [ doc = "0x48 - Transmit data bytes 1-4 (Tx Buffer)" ]
    pub tda2: TDA,
    # [ doc = "0x4c - Transmit data bytes 5-8 (Tx Buffer )" ]
    pub tdb2: TDB,
    # [ doc = "0x50 - Transmit frame info (Tx Buffer )" ]
    pub tfi3: TFI,
    # [ doc = "0x54 - Transmit Identifier (Tx Buffer)" ]
    pub tid3: TID,
    # [ doc = "0x58 - Transmit data bytes 1-4 (Tx Buffer)" ]
    pub tda3: TDA,
    # [ doc = "0x5c - Transmit data bytes 5-8 (Tx Buffer )" ]
    pub tdb3: TDB,
}
# [ doc = "Controls the operating mode of the CAN Controller." ]
pub struct MOD {
    register: VolatileCell<u32>,
}
# [ doc = "Controls the operating mode of the CAN Controller." ]
pub mod mod_ {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MOD {
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
    # [ doc = "Possible values of the field `RM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RMR {
        # [ doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written." ]
        NORMAL_THE_CAN_CONTR,
        # [ doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted." ]
        RESET_CAN_OPERATION,
    }
    impl RMR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RMR::NORMAL_THE_CAN_CONTR => false,
                RMR::RESET_CAN_OPERATION => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RMR {
            match value {
                false => RMR::NORMAL_THE_CAN_CONTR,
                true => RMR::RESET_CAN_OPERATION,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL_THE_CAN_CONTR`" ]
        # [ inline ( always ) ]
        pub fn is_normal_the_can_contr(&self) -> bool {
            *self == RMR::NORMAL_THE_CAN_CONTR
        }
        # [ doc = "Checks if the value of the field is `RESET_CAN_OPERATION`" ]
        # [ inline ( always ) ]
        pub fn is_reset_can_operation(&self) -> bool {
            *self == RMR::RESET_CAN_OPERATION
        }
    }
    # [ doc = "Possible values of the field `LOM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum LOMR {
        # [ doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value." ]
        NORMAL_THE_CAN_CONT,
        # [ doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging." ]
        LISTEN_ONLY_THE_CON,
    }
    impl LOMR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                LOMR::NORMAL_THE_CAN_CONT => false,
                LOMR::LISTEN_ONLY_THE_CON => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> LOMR {
            match value {
                false => LOMR::NORMAL_THE_CAN_CONT,
                true => LOMR::LISTEN_ONLY_THE_CON,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL_THE_CAN_CONT`" ]
        # [ inline ( always ) ]
        pub fn is_normal_the_can_cont(&self) -> bool {
            *self == LOMR::NORMAL_THE_CAN_CONT
        }
        # [ doc = "Checks if the value of the field is `LISTEN_ONLY_THE_CON`" ]
        # [ inline ( always ) ]
        pub fn is_listen_only_the_con(&self) -> bool {
            *self == LOMR::LISTEN_ONLY_THE_CON
        }
    }
    # [ doc = "Possible values of the field `STM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum STMR {
        # [ doc = "Normal. A transmitted message must be acknowledged to be considered successful." ]
        NORMAL_A_TRANSMITTE,
        # [ doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR." ]
        SELF_TEST_THE_CONTR,
    }
    impl STMR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                STMR::NORMAL_A_TRANSMITTE => false,
                STMR::SELF_TEST_THE_CONTR => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> STMR {
            match value {
                false => STMR::NORMAL_A_TRANSMITTE,
                true => STMR::SELF_TEST_THE_CONTR,
            }
        }
        # [ doc = "Checks if the value of the field is `NORMAL_A_TRANSMITTE`" ]
        # [ inline ( always ) ]
        pub fn is_normal_a_transmitte(&self) -> bool {
            *self == STMR::NORMAL_A_TRANSMITTE
        }
        # [ doc = "Checks if the value of the field is `SELF_TEST_THE_CONTR`" ]
        # [ inline ( always ) ]
        pub fn is_self_test_the_contr(&self) -> bool {
            *self == STMR::SELF_TEST_THE_CONTR
        }
    }
    # [ doc = "Possible values of the field `TPM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TPMR {
        # [ doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier." ]
        CAN_ID_THE_TRANSMIT,
        # [ doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer." ]
        LOCAL_PRIORITY_THE_,
    }
    impl TPMR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TPMR::CAN_ID_THE_TRANSMIT => false,
                TPMR::LOCAL_PRIORITY_THE_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TPMR {
            match value {
                false => TPMR::CAN_ID_THE_TRANSMIT,
                true => TPMR::LOCAL_PRIORITY_THE_,
            }
        }
        # [ doc = "Checks if the value of the field is `CAN_ID_THE_TRANSMIT`" ]
        # [ inline ( always ) ]
        pub fn is_can_id_the_transmit(&self) -> bool {
            *self == TPMR::CAN_ID_THE_TRANSMIT
        }
        # [ doc = "Checks if the value of the field is `LOCAL_PRIORITY_THE_`" ]
        # [ inline ( always ) ]
        pub fn is_local_priority_the_(&self) -> bool {
            *self == TPMR::LOCAL_PRIORITY_THE_
        }
    }
    # [ doc = "Possible values of the field `SM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SMR {
        # [ doc = "Wake-up. Normal operation." ]
        WAKE_UP_NORMAL_OPER,
        # [ doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565." ]
        SLEEP_THE_CAN_CONTR,
    }
    impl SMR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                SMR::WAKE_UP_NORMAL_OPER => false,
                SMR::SLEEP_THE_CAN_CONTR => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SMR {
            match value {
                false => SMR::WAKE_UP_NORMAL_OPER,
                true => SMR::SLEEP_THE_CAN_CONTR,
            }
        }
        # [ doc = "Checks if the value of the field is `WAKE_UP_NORMAL_OPER`" ]
        # [ inline ( always ) ]
        pub fn is_wake_up_normal_oper(&self) -> bool {
            *self == SMR::WAKE_UP_NORMAL_OPER
        }
        # [ doc = "Checks if the value of the field is `SLEEP_THE_CAN_CONTR`" ]
        # [ inline ( always ) ]
        pub fn is_sleep_the_can_contr(&self) -> bool {
            *self == SMR::SLEEP_THE_CAN_CONTR
        }
    }
    # [ doc = "Possible values of the field `RPM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RPMR {
        # [ doc = "Low active. RD input is active Low (dominant bit = 0)." ]
        LOW_ACTIVE_RD_INPUT,
        # [ doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity." ]
        HIGH_ACTIVE_RD_INPU,
    }
    impl RPMR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RPMR::LOW_ACTIVE_RD_INPUT => false,
                RPMR::HIGH_ACTIVE_RD_INPU => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RPMR {
            match value {
                false => RPMR::LOW_ACTIVE_RD_INPUT,
                true => RPMR::HIGH_ACTIVE_RD_INPU,
            }
        }
        # [ doc = "Checks if the value of the field is `LOW_ACTIVE_RD_INPUT`" ]
        # [ inline ( always ) ]
        pub fn is_low_active_rd_input(&self) -> bool {
            *self == RPMR::LOW_ACTIVE_RD_INPUT
        }
        # [ doc = "Checks if the value of the field is `HIGH_ACTIVE_RD_INPU`" ]
        # [ inline ( always ) ]
        pub fn is_high_active_rd_inpu(&self) -> bool {
            *self == RPMR::HIGH_ACTIVE_RD_INPU
        }
    }
    # [ doc = "Possible values of the field `TM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TMR {
        # [ doc = "Disabled. Normal operation." ]
        DISABLED_NORMAL_OPE,
        # [ doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock." ]
        ENABLED_THE_TD_PIN_,
    }
    impl TMR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TMR::DISABLED_NORMAL_OPE => false,
                TMR::ENABLED_THE_TD_PIN_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TMR {
            match value {
                false => TMR::DISABLED_NORMAL_OPE,
                true => TMR::ENABLED_THE_TD_PIN_,
            }
        }
        # [ doc = "Checks if the value of the field is `DISABLED_NORMAL_OPE`" ]
        # [ inline ( always ) ]
        pub fn is_disabled_normal_ope(&self) -> bool {
            *self == TMR::DISABLED_NORMAL_OPE
        }
        # [ doc = "Checks if the value of the field is `ENABLED_THE_TD_PIN_`" ]
        # [ inline ( always ) ]
        pub fn is_enabled_the_td_pin_(&self) -> bool {
            *self == TMR::ENABLED_THE_TD_PIN_
        }
    }
    # [ doc = "Values that can be written to the field `RM`" ]
    pub enum RMW {
        # [ doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written." ]
        NORMAL_THE_CAN_CONTR,
        # [ doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted." ]
        RESET_CAN_OPERATION,
    }
    impl RMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                RMW::NORMAL_THE_CAN_CONTR => false,
                RMW::RESET_CAN_OPERATION => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written." ]
        # [ inline ( always ) ]
        pub fn normal_the_can_contr(self) -> &'a mut W {
            self.variant(RMW::NORMAL_THE_CAN_CONTR)
        }
        # [ doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted." ]
        # [ inline ( always ) ]
        pub fn reset_can_operation(self) -> &'a mut W {
            self.variant(RMW::RESET_CAN_OPERATION)
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
    # [ doc = "Values that can be written to the field `LOM`" ]
    pub enum LOMW {
        # [ doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value." ]
        NORMAL_THE_CAN_CONT,
        # [ doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging." ]
        LISTEN_ONLY_THE_CON,
    }
    impl LOMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                LOMW::NORMAL_THE_CAN_CONT => false,
                LOMW::LISTEN_ONLY_THE_CON => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LOMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LOMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: LOMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value." ]
        # [ inline ( always ) ]
        pub fn normal_the_can_cont(self) -> &'a mut W {
            self.variant(LOMW::NORMAL_THE_CAN_CONT)
        }
        # [ doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging." ]
        # [ inline ( always ) ]
        pub fn listen_only_the_con(self) -> &'a mut W {
            self.variant(LOMW::LISTEN_ONLY_THE_CON)
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
    # [ doc = "Values that can be written to the field `STM`" ]
    pub enum STMW {
        # [ doc = "Normal. A transmitted message must be acknowledged to be considered successful." ]
        NORMAL_A_TRANSMITTE,
        # [ doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR." ]
        SELF_TEST_THE_CONTR,
    }
    impl STMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                STMW::NORMAL_A_TRANSMITTE => false,
                STMW::SELF_TEST_THE_CONTR => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _STMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: STMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Normal. A transmitted message must be acknowledged to be considered successful." ]
        # [ inline ( always ) ]
        pub fn normal_a_transmitte(self) -> &'a mut W {
            self.variant(STMW::NORMAL_A_TRANSMITTE)
        }
        # [ doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR." ]
        # [ inline ( always ) ]
        pub fn self_test_the_contr(self) -> &'a mut W {
            self.variant(STMW::SELF_TEST_THE_CONTR)
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
    # [ doc = "Values that can be written to the field `TPM`" ]
    pub enum TPMW {
        # [ doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier." ]
        CAN_ID_THE_TRANSMIT,
        # [ doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer." ]
        LOCAL_PRIORITY_THE_,
    }
    impl TPMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                TPMW::CAN_ID_THE_TRANSMIT => false,
                TPMW::LOCAL_PRIORITY_THE_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TPMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TPMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TPMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier." ]
        # [ inline ( always ) ]
        pub fn can_id_the_transmit(self) -> &'a mut W {
            self.variant(TPMW::CAN_ID_THE_TRANSMIT)
        }
        # [ doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer." ]
        # [ inline ( always ) ]
        pub fn local_priority_the_(self) -> &'a mut W {
            self.variant(TPMW::LOCAL_PRIORITY_THE_)
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
    # [ doc = "Values that can be written to the field `SM`" ]
    pub enum SMW {
        # [ doc = "Wake-up. Normal operation." ]
        WAKE_UP_NORMAL_OPER,
        # [ doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565." ]
        SLEEP_THE_CAN_CONTR,
    }
    impl SMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SMW::WAKE_UP_NORMAL_OPER => false,
                SMW::SLEEP_THE_CAN_CONTR => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Wake-up. Normal operation." ]
        # [ inline ( always ) ]
        pub fn wake_up_normal_oper(self) -> &'a mut W {
            self.variant(SMW::WAKE_UP_NORMAL_OPER)
        }
        # [ doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565." ]
        # [ inline ( always ) ]
        pub fn sleep_the_can_contr(self) -> &'a mut W {
            self.variant(SMW::SLEEP_THE_CAN_CONTR)
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
    # [ doc = "Values that can be written to the field `RPM`" ]
    pub enum RPMW {
        # [ doc = "Low active. RD input is active Low (dominant bit = 0)." ]
        LOW_ACTIVE_RD_INPUT,
        # [ doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity." ]
        HIGH_ACTIVE_RD_INPU,
    }
    impl RPMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                RPMW::LOW_ACTIVE_RD_INPUT => false,
                RPMW::HIGH_ACTIVE_RD_INPU => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RPMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RPMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RPMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Low active. RD input is active Low (dominant bit = 0)." ]
        # [ inline ( always ) ]
        pub fn low_active_rd_input(self) -> &'a mut W {
            self.variant(RPMW::LOW_ACTIVE_RD_INPUT)
        }
        # [ doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity." ]
        # [ inline ( always ) ]
        pub fn high_active_rd_inpu(self) -> &'a mut W {
            self.variant(RPMW::HIGH_ACTIVE_RD_INPU)
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
    # [ doc = "Values that can be written to the field `TM`" ]
    pub enum TMW {
        # [ doc = "Disabled. Normal operation." ]
        DISABLED_NORMAL_OPE,
        # [ doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock." ]
        ENABLED_THE_TD_PIN_,
    }
    impl TMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                TMW::DISABLED_NORMAL_OPE => false,
                TMW::ENABLED_THE_TD_PIN_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Disabled. Normal operation." ]
        # [ inline ( always ) ]
        pub fn disabled_normal_ope(self) -> &'a mut W {
            self.variant(TMW::DISABLED_NORMAL_OPE)
        }
        # [ doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock." ]
        # [ inline ( always ) ]
        pub fn enabled_the_td_pin_(self) -> &'a mut W {
            self.variant(TMW::ENABLED_THE_TD_PIN_)
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
        # [ doc = "Bit 0 - Reset Mode." ]
        # [ inline ( always ) ]
        pub fn rm(&self) -> RMR {
            RMR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 0;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 1 - Listen Only Mode." ]
        # [ inline ( always ) ]
        pub fn lom(&self) -> LOMR {
            LOMR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 1;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 2 - Self Test Mode." ]
        # [ inline ( always ) ]
        pub fn stm(&self) -> STMR {
            STMR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 2;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 3 - Transmit Priority Mode." ]
        # [ inline ( always ) ]
        pub fn tpm(&self) -> TPMR {
            TPMR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 3;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 4 - Sleep Mode." ]
        # [ inline ( always ) ]
        pub fn sm(&self) -> SMR {
            SMR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 4;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 5 - Receive Polarity Mode." ]
        # [ inline ( always ) ]
        pub fn rpm(&self) -> RPMR {
            RPMR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 5;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 7 - Test Mode." ]
        # [ inline ( always ) ]
        pub fn tm(&self) -> TMR {
            TMR::_from({
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
        # [ doc = "Bit 0 - Reset Mode." ]
        # [ inline ( always ) ]
        pub fn rm(&mut self) -> _RMW {
            _RMW { w: self }
        }
        # [ doc = "Bit 1 - Listen Only Mode." ]
        # [ inline ( always ) ]
        pub fn lom(&mut self) -> _LOMW {
            _LOMW { w: self }
        }
        # [ doc = "Bit 2 - Self Test Mode." ]
        # [ inline ( always ) ]
        pub fn stm(&mut self) -> _STMW {
            _STMW { w: self }
        }
        # [ doc = "Bit 3 - Transmit Priority Mode." ]
        # [ inline ( always ) ]
        pub fn tpm(&mut self) -> _TPMW {
            _TPMW { w: self }
        }
        # [ doc = "Bit 4 - Sleep Mode." ]
        # [ inline ( always ) ]
        pub fn sm(&mut self) -> _SMW {
            _SMW { w: self }
        }
        # [ doc = "Bit 5 - Receive Polarity Mode." ]
        # [ inline ( always ) ]
        pub fn rpm(&mut self) -> _RPMW {
            _RPMW { w: self }
        }
        # [ doc = "Bit 7 - Test Mode." ]
        # [ inline ( always ) ]
        pub fn tm(&mut self) -> _TMW {
            _TMW { w: self }
        }
    }
}
# [ doc = "Command bits that affect the state of the CAN Controller" ]
pub struct CMR {
    register: VolatileCell<u32>,
}
# [ doc = "Command bits that affect the state of the CAN Controller" ]
pub mod cmr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::CMR {
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
    # [ doc = "Values that can be written to the field `TR`" ]
    pub enum TRW {
        # [ doc = "Absent.No transmission request." ]
        ABSENT_NO_TRANSMISSI,
        # [ doc = "Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))" ]
        PRESENT_THE_MESSAGE,
    }
    impl TRW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                TRW::ABSENT_NO_TRANSMISSI => false,
                TRW::PRESENT_THE_MESSAGE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TRW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: TRW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Absent.No transmission request." ]
        # [ inline ( always ) ]
        pub fn absent_no_transmissi(self) -> &'a mut W {
            self.variant(TRW::ABSENT_NO_TRANSMISSI)
        }
        # [ doc = "Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))" ]
        # [ inline ( always ) ]
        pub fn present_the_message(self) -> &'a mut W {
            self.variant(TRW::PRESENT_THE_MESSAGE)
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
    # [ doc = "Values that can be written to the field `AT`" ]
    pub enum ATW {
        # [ doc = "No action. Do not abort the transmission." ]
        NO_ACTION_DO_NOT_AB,
        # [ doc = "Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled." ]
        PRESENT_IF_NOT_ALRE,
    }
    impl ATW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                ATW::NO_ACTION_DO_NOT_AB => false,
                ATW::PRESENT_IF_NOT_ALRE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ATW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: ATW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "No action. Do not abort the transmission." ]
        # [ inline ( always ) ]
        pub fn no_action_do_not_ab(self) -> &'a mut W {
            self.variant(ATW::NO_ACTION_DO_NOT_AB)
        }
        # [ doc = "Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled." ]
        # [ inline ( always ) ]
        pub fn present_if_not_alre(self) -> &'a mut W {
            self.variant(ATW::PRESENT_IF_NOT_ALRE)
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
    # [ doc = "Values that can be written to the field `RRB`" ]
    pub enum RRBW {
        # [ doc = "No action. Do not release the receive buffer." ]
        NO_ACTION_DO_NOT_RE,
        # [ doc = "Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)." ]
        RELEASED_THE_INFORM,
    }
    impl RRBW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                RRBW::NO_ACTION_DO_NOT_RE => false,
                RRBW::RELEASED_THE_INFORM => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RRBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RRBW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: RRBW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "No action. Do not release the receive buffer." ]
        # [ inline ( always ) ]
        pub fn no_action_do_not_re(self) -> &'a mut W {
            self.variant(RRBW::NO_ACTION_DO_NOT_RE)
        }
        # [ doc = "Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)." ]
        # [ inline ( always ) ]
        pub fn released_the_inform(self) -> &'a mut W {
            self.variant(RRBW::RELEASED_THE_INFORM)
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
    # [ doc = "Values that can be written to the field `CDO`" ]
    pub enum CDOW {
        # [ doc = "No action. Do not clear the data overrun bit." ]
        NO_ACTION_DO_NOT_CL,
        # [ doc = "Clear. The Data Overrun bit in Status Register(s) is cleared." ]
        CLEAR_THE_DATA_OVER,
    }
    impl CDOW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                CDOW::NO_ACTION_DO_NOT_CL => false,
                CDOW::CLEAR_THE_DATA_OVER => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CDOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CDOW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: CDOW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "No action. Do not clear the data overrun bit." ]
        # [ inline ( always ) ]
        pub fn no_action_do_not_cl(self) -> &'a mut W {
            self.variant(CDOW::NO_ACTION_DO_NOT_CL)
        }
        # [ doc = "Clear. The Data Overrun bit in Status Register(s) is cleared." ]
        # [ inline ( always ) ]
        pub fn clear_the_data_over(self) -> &'a mut W {
            self.variant(CDOW::CLEAR_THE_DATA_OVER)
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
    # [ doc = "Values that can be written to the field `SRR`" ]
    pub enum SRRW {
        # [ doc = "Absent. No self reception request." ]
        ABSENT_NO_SELF_RECE,
        # [ doc = "Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter." ]
        PRESENT_THE_MESSAGE,
    }
    impl SRRW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SRRW::ABSENT_NO_SELF_RECE => false,
                SRRW::PRESENT_THE_MESSAGE => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SRRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SRRW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SRRW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Absent. No self reception request." ]
        # [ inline ( always ) ]
        pub fn absent_no_self_rece(self) -> &'a mut W {
            self.variant(SRRW::ABSENT_NO_SELF_RECE)
        }
        # [ doc = "Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter." ]
        # [ inline ( always ) ]
        pub fn present_the_message(self) -> &'a mut W {
            self.variant(SRRW::PRESENT_THE_MESSAGE)
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
    # [ doc = "Values that can be written to the field `STB1`" ]
    pub enum STB1W {
        # [ doc = "Not selected. Tx Buffer 1 is not selected for transmission." ]
        NOT_SELECTED_TX_BUF,
        # [ doc = "Selected. Tx Buffer 1 is selected for transmission." ]
        SELECTED_TX_BUFFER_,
    }
    impl STB1W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                STB1W::NOT_SELECTED_TX_BUF => false,
                STB1W::SELECTED_TX_BUFFER_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _STB1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _STB1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: STB1W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Not selected. Tx Buffer 1 is not selected for transmission." ]
        # [ inline ( always ) ]
        pub fn not_selected_tx_buf(self) -> &'a mut W {
            self.variant(STB1W::NOT_SELECTED_TX_BUF)
        }
        # [ doc = "Selected. Tx Buffer 1 is selected for transmission." ]
        # [ inline ( always ) ]
        pub fn selected_tx_buffer_(self) -> &'a mut W {
            self.variant(STB1W::SELECTED_TX_BUFFER_)
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
    # [ doc = "Values that can be written to the field `STB2`" ]
    pub enum STB2W {
        # [ doc = "Not selected. Tx Buffer 2 is not selected for transmission." ]
        NOT_SELECTED_TX_BUF,
        # [ doc = "Selected. Tx Buffer 2 is selected for transmission." ]
        SELECTED_TX_BUFFER_,
    }
    impl STB2W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                STB2W::NOT_SELECTED_TX_BUF => false,
                STB2W::SELECTED_TX_BUFFER_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _STB2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _STB2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: STB2W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Not selected. Tx Buffer 2 is not selected for transmission." ]
        # [ inline ( always ) ]
        pub fn not_selected_tx_buf(self) -> &'a mut W {
            self.variant(STB2W::NOT_SELECTED_TX_BUF)
        }
        # [ doc = "Selected. Tx Buffer 2 is selected for transmission." ]
        # [ inline ( always ) ]
        pub fn selected_tx_buffer_(self) -> &'a mut W {
            self.variant(STB2W::SELECTED_TX_BUFFER_)
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
    # [ doc = "Values that can be written to the field `STB3`" ]
    pub enum STB3W {
        # [ doc = "Not selected. Tx Buffer 3 is not selected for transmission." ]
        NOT_SELECTED_TX_BUF,
        # [ doc = "Selected. Tx Buffer 3 is selected for transmission." ]
        SELECTED_TX_BUFFER_,
    }
    impl STB3W {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                STB3W::NOT_SELECTED_TX_BUF => false,
                STB3W::SELECTED_TX_BUFFER_ => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _STB3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _STB3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: STB3W) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "Not selected. Tx Buffer 3 is not selected for transmission." ]
        # [ inline ( always ) ]
        pub fn not_selected_tx_buf(self) -> &'a mut W {
            self.variant(STB3W::NOT_SELECTED_TX_BUF)
        }
        # [ doc = "Selected. Tx Buffer 3 is selected for transmission." ]
        # [ inline ( always ) ]
        pub fn selected_tx_buffer_(self) -> &'a mut W {
            self.variant(STB3W::SELECTED_TX_BUFFER_)
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
        # [ doc = "Bit 0 - Transmission Request." ]
        # [ inline ( always ) ]
        pub fn tr(&mut self) -> _TRW {
            _TRW { w: self }
        }
        # [ doc = "Bit 1 - Abort Transmission." ]
        # [ inline ( always ) ]
        pub fn at(&mut self) -> _ATW {
            _ATW { w: self }
        }
        # [ doc = "Bit 2 - Release Receive Buffer." ]
        # [ inline ( always ) ]
        pub fn rrb(&mut self) -> _RRBW {
            _RRBW { w: self }
        }
        # [ doc = "Bit 3 - Clear Data Overrun." ]
        # [ inline ( always ) ]
        pub fn cdo(&mut self) -> _CDOW {
            _CDOW { w: self }
        }
        # [ doc = "Bit 4 - Self Reception Request." ]
        # [ inline ( always ) ]
        pub fn srr(&mut self) -> _SRRW {
            _SRRW { w: self }
        }
        # [ doc = "Bit 5 - Select Tx Buffer 1." ]
        # [ inline ( always ) ]
        pub fn stb1(&mut self) -> _STB1W {
            _STB1W { w: self }
        }
        # [ doc = "Bit 6 - Select Tx Buffer 2." ]
        # [ inline ( always ) ]
        pub fn stb2(&mut self) -> _STB2W {
            _STB2W { w: self }
        }
        # [ doc = "Bit 7 - Select Tx Buffer 3." ]
        # [ inline ( always ) ]
        pub fn stb3(&mut self) -> _STB3W {
            _STB3W { w: self }
        }
    }
}
# [ doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1." ]
pub struct GSR {
    register: VolatileCell<u32>,
}
# [ doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1." ]
pub mod gsr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::GSR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = "Possible values of the field `RBS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RBSR {
        # [ doc = "Empty. No message is available." ]
        EMPTY_NO_MESSAGE_IS,
        # [ doc = "Full. At least one complete message is received by the Double Receive Buffer and available in the CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers. This bit is cleared by the Release Receive Buffer command in CANxCMR, if no subsequent received message is available." ]
        FULL_AT_LEAST_ONE_C,
    }
    impl RBSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RBSR::EMPTY_NO_MESSAGE_IS => false,
                RBSR::FULL_AT_LEAST_ONE_C => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RBSR {
            match value {
                false => RBSR::EMPTY_NO_MESSAGE_IS,
                true => RBSR::FULL_AT_LEAST_ONE_C,
            }
        }
        # [ doc = "Checks if the value of the field is `EMPTY_NO_MESSAGE_IS`" ]
        # [ inline ( always ) ]
        pub fn is_empty_no_message_is(&self) -> bool {
            *self == RBSR::EMPTY_NO_MESSAGE_IS
        }
        # [ doc = "Checks if the value of the field is `FULL_AT_LEAST_ONE_C`" ]
        # [ inline ( always ) ]
        pub fn is_full_at_least_one_c(&self) -> bool {
            *self == RBSR::FULL_AT_LEAST_ONE_C
        }
    }
    # [ doc = "Possible values of the field `DOS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum DOSR {
        # [ doc = "Absent. No data overrun has occurred since the last Clear Data Overrun command was given/written to CANxCMR (or since Reset)." ]
        ABSENT_NO_DATA_OVER,
        # [ doc = "Overrun. A message was lost because the preceding message to this CAN controller was not read and released quickly enough (there was not enough space for a new message in the Double Receive Buffer)." ]
        OVERRUN_A_MESSAGE_W,
    }
    impl DOSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                DOSR::ABSENT_NO_DATA_OVER => false,
                DOSR::OVERRUN_A_MESSAGE_W => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> DOSR {
            match value {
                false => DOSR::ABSENT_NO_DATA_OVER,
                true => DOSR::OVERRUN_A_MESSAGE_W,
            }
        }
        # [ doc = "Checks if the value of the field is `ABSENT_NO_DATA_OVER`" ]
        # [ inline ( always ) ]
        pub fn is_absent_no_data_over(&self) -> bool {
            *self == DOSR::ABSENT_NO_DATA_OVER
        }
        # [ doc = "Checks if the value of the field is `OVERRUN_A_MESSAGE_W`" ]
        # [ inline ( always ) ]
        pub fn is_overrun_a_message_w(&self) -> bool {
            *self == DOSR::OVERRUN_A_MESSAGE_W
        }
    }
    # [ doc = "Possible values of the field `TBS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TBSR {
        # [ doc = "Locked. At least one of the Transmit Buffers is not available for the CPU, i.e. at least one previously queued message for this CAN controller has not yet been sent, and therefore software should not write to the CANxTFI, CANxTID, CANxTDA, nor CANxTDB registers of that (those) Tx buffer(s)." ]
        LOCKED_AT_LEAST_ONE,
        # [ doc = "Released. All three Transmit Buffers are available for the CPU. No transmit message is pending for this CAN controller (in any of the 3 Tx buffers), and software may write to any of the CANxTFI, CANxTID, CANxTDA, and CANxTDB registers." ]
        RELEASED_ALL_THREE_,
    }
    impl TBSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TBSR::LOCKED_AT_LEAST_ONE => false,
                TBSR::RELEASED_ALL_THREE_ => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TBSR {
            match value {
                false => TBSR::LOCKED_AT_LEAST_ONE,
                true => TBSR::RELEASED_ALL_THREE_,
            }
        }
        # [ doc = "Checks if the value of the field is `LOCKED_AT_LEAST_ONE`" ]
        # [ inline ( always ) ]
        pub fn is_locked_at_least_one(&self) -> bool {
            *self == TBSR::LOCKED_AT_LEAST_ONE
        }
        # [ doc = "Checks if the value of the field is `RELEASED_ALL_THREE_`" ]
        # [ inline ( always ) ]
        pub fn is_released_all_three_(&self) -> bool {
            *self == TBSR::RELEASED_ALL_THREE_
        }
    }
    # [ doc = "Possible values of the field `TCS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TCSR {
        # [ doc = "Incomplete. At least one requested transmission has not been successfully completed yet." ]
        INCOMPLETE_AT_LEAST,
        # [ doc = "Complete. All requested transmission(s) has (have) been successfully completed." ]
        COMPLETE_ALL_REQUES,
    }
    impl TCSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TCSR::INCOMPLETE_AT_LEAST => false,
                TCSR::COMPLETE_ALL_REQUES => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TCSR {
            match value {
                false => TCSR::INCOMPLETE_AT_LEAST,
                true => TCSR::COMPLETE_ALL_REQUES,
            }
        }
        # [ doc = "Checks if the value of the field is `INCOMPLETE_AT_LEAST`" ]
        # [ inline ( always ) ]
        pub fn is_incomplete_at_least(&self) -> bool {
            *self == TCSR::INCOMPLETE_AT_LEAST
        }
        # [ doc = "Checks if the value of the field is `COMPLETE_ALL_REQUES`" ]
        # [ inline ( always ) ]
        pub fn is_complete_all_reques(&self) -> bool {
            *self == TCSR::COMPLETE_ALL_REQUES
        }
    }
    # [ doc = "Possible values of the field `RS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RSR {
        # [ doc = "Idle. The CAN controller is idle." ]
        IDLE_THE_CAN_CONTRO,
        # [ doc = "Receive. The CAN controller is receiving a message." ]
        RECEIVE_THE_CAN_CON,
    }
    impl RSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                RSR::IDLE_THE_CAN_CONTRO => false,
                RSR::RECEIVE_THE_CAN_CON => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> RSR {
            match value {
                false => RSR::IDLE_THE_CAN_CONTRO,
                true => RSR::RECEIVE_THE_CAN_CON,
            }
        }
        # [ doc = "Checks if the value of the field is `IDLE_THE_CAN_CONTRO`" ]
        # [ inline ( always ) ]
        pub fn is_idle_the_can_contro(&self) -> bool {
            *self == RSR::IDLE_THE_CAN_CONTRO
        }
        # [ doc = "Checks if the value of the field is `RECEIVE_THE_CAN_CON`" ]
        # [ inline ( always ) ]
        pub fn is_receive_the_can_con(&self) -> bool {
            *self == RSR::RECEIVE_THE_CAN_CON
        }
    }
    # [ doc = "Possible values of the field `TS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TSR {
        # [ doc = "Idle. The CAN controller is idle." ]
        IDLE_THE_CAN_CONTRO,
        # [ doc = "Transmit. The CAN controller is sending a message." ]
        TRANSMIT_THE_CAN_CO,
    }
    impl TSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TSR::IDLE_THE_CAN_CONTRO => false,
                TSR::TRANSMIT_THE_CAN_CO => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TSR {
            match value {
                false => TSR::IDLE_THE_CAN_CONTRO,
                true => TSR::TRANSMIT_THE_CAN_CO,
            }
        }
        # [ doc = "Checks if the value of the field is `IDLE_THE_CAN_CONTRO`" ]
        # [ inline ( always ) ]
        pub fn is_idle_the_can_contro(&self) -> bool {
            *self == TSR::IDLE_THE_CAN_CONTRO
        }
        # [ doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`" ]
        # [ inline ( always ) ]
        pub fn is_transmit_the_can_co(&self) -> bool {
            *self == TSR::TRANSMIT_THE_CAN_CO
        }
    }
    # [ doc = "Possible values of the field `ES`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ESR {
        # [ doc = "OK. Both error counters are below the Error Warning Limit." ]
        OK_BOTH_ERROR_COUNT,
        # [ doc = "Error. One or both of the Transmit and Receive Error Counters has reached the limit set in the Error Warning Limit register." ]
        ERROR_ONE_OR_BOTH_O,
    }
    impl ESR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ESR::OK_BOTH_ERROR_COUNT => false,
                ESR::ERROR_ONE_OR_BOTH_O => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ESR {
            match value {
                false => ESR::OK_BOTH_ERROR_COUNT,
                true => ESR::ERROR_ONE_OR_BOTH_O,
            }
        }
        # [ doc = "Checks if the value of the field is `OK_BOTH_ERROR_COUNT`" ]
        # [ inline ( always ) ]
        pub fn is_ok_both_error_count(&self) -> bool {
            *self == ESR::OK_BOTH_ERROR_COUNT
        }
        # [ doc = "Checks if the value of the field is `ERROR_ONE_OR_BOTH_O`" ]
        # [ inline ( always ) ]
        pub fn is_error_one_or_both_o(&self) -> bool {
            *self == ESR::ERROR_ONE_OR_BOTH_O
        }
    }
    # [ doc = "Possible values of the field `BS`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum BSR {
        # [ doc = "Bus-on. The CAN Controller is involved in bus activities" ]
        BUS_ON_THE_CAN_CONT,
        # [ doc = "Bus-off. The CAN controller is currently not involved/prohibited from bus activity because the Transmit Error Counter reached its limiting value of 255." ]
        BUS_OFF_THE_CAN_CON,
    }
    impl BSR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                BSR::BUS_ON_THE_CAN_CONT => false,
                BSR::BUS_OFF_THE_CAN_CON => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> BSR {
            match value {
                false => BSR::BUS_ON_THE_CAN_CONT,
                true => BSR::BUS_OFF_THE_CAN_CON,
            }
        }
        # [ doc = "Checks if the value of the field is `BUS_ON_THE_CAN_CONT`" ]
        # [ inline ( always ) ]
        pub fn is_bus_on_the_can_cont(&self) -> bool {
            *self == BSR::BUS_ON_THE_CAN_CONT
        }
        # [ doc = "Checks if the value of the field is `BUS_OFF_THE_CAN_CON`" ]
        # [ inline ( always ) ]
        pub fn is_bus_off_the_can_con(&self) -> bool {
            *self == BSR::BUS_OFF_THE_CAN_CON
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RXERRR {
        bits: u8,
    }
    impl RXERRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TXERRR {
        bits: u8,
    }
    impl TXERRR {
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
        # [ doc = "Bit 0 - Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared." ]
        # [ inline ( always ) ]
        pub fn rbs(&self) -> RBSR {
            RBSR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 0;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 1 - Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled." ]
        # [ inline ( always ) ]
        pub fn dos(&self) -> DOSR {
            DOSR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 1;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 2 - Transmit Buffer Status." ]
        # [ inline ( always ) ]
        pub fn tbs(&self) -> TBSR {
            TBSR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 2;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 3 - Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully." ]
        # [ inline ( always ) ]
        pub fn tcs(&self) -> TCSR {
            TCSR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 3;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
        # [ doc = "Bit 4 - Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits." ]
        # [ inline ( always ) ]
        pub fn rs(&self) -> RSR {
            RSR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 4;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 5 - Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits." ]
        # [ inline ( always ) ]
        pub fn ts(&self) -> TSR {
            TSR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 5;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 6 - Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018)." ]
        # [ inline ( always ) ]
        pub fn es(&self) -> ESR {
            ESR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 6;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bit 7 - Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery." ]
        # [ inline ( always ) ]
        pub fn bs(&self) -> BSR {
            BSR::_from({
                           const MASK: bool = true;
                           const OFFSET: u8 = 7;
                           ((self.bits >> OFFSET) & MASK as u32) != 0
                       })
        }
        # [ doc = "Bits 16:23 - The current value of the Rx Error Counter (an 8-bit value)." ]
        # [ inline ( always ) ]
        pub fn rxerr(&self) -> RXERRR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            RXERRR { bits }
        }
        # [ doc = "Bits 24:31 - The current value of the Tx Error Counter (an 8-bit value)." ]
        # [ inline ( always ) ]
        pub fn txerr(&self) -> TXERRR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TXERRR { bits }
        }
    }
}
# [ doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture" ]
pub struct ICR {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture" ]
pub mod icr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::ICR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
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
    pub struct TI1R {
        bits: bool,
    }
    impl TI1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EIR {
        bits: bool,
    }
    impl EIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DOIR {
        bits: bool,
    }
    impl DOIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WUIR {
        bits: bool,
    }
    impl WUIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EPIR {
        bits: bool,
    }
    impl EPIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ALIR {
        bits: bool,
    }
    impl ALIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BEIR {
        bits: bool,
    }
    impl BEIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDIR {
        bits: bool,
    }
    impl IDIR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TI2R {
        bits: bool,
    }
    impl TI2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TI3R {
        bits: bool,
    }
    impl TI3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ERRBIT4_0R {
        bits: u8,
    }
    impl ERRBIT4_0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Possible values of the field `ERRDIR`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ERRDIRR {
        # [ doc = "Error occurred during transmitting." ]
        ERROR_OCCURRED_DURING_TRANSMITTING,
        # [ doc = "Error occurred during receiving." ]
        ERROR_OCCURRED_DURING_RECEIVING,
    }
    impl ERRDIRR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                ERRDIRR::ERROR_OCCURRED_DURING_TRANSMITTING => false,
                ERRDIRR::ERROR_OCCURRED_DURING_RECEIVING => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> ERRDIRR {
            match value {
                false => ERRDIRR::ERROR_OCCURRED_DURING_TRANSMITTING,
                true => ERRDIRR::ERROR_OCCURRED_DURING_RECEIVING,
            }
        }
        # [ doc = "Checks if the value of the field is `ERROR_OCCURRED_DURING_TRANSMITTING`" ]
        # [ inline ( always ) ]
        pub fn is_error_occurred_during_transmitting(&self) -> bool {
            *self == ERRDIRR::ERROR_OCCURRED_DURING_TRANSMITTING
        }
        # [ doc = "Checks if the value of the field is `ERROR_OCCURRED_DURING_RECEIVING`" ]
        # [ inline ( always ) ]
        pub fn is_error_occurred_during_receiving(&self) -> bool {
            *self == ERRDIRR::ERROR_OCCURRED_DURING_RECEIVING
        }
    }
    # [ doc = "Possible values of the field `ERRC1_0`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ERRC1_0R {
        # [ doc = "Bit error" ]
        BIT_ERROR,
        # [ doc = "Form error" ]
        FORM_ERROR,
        # [ doc = "Stuff error" ]
        STUFF_ERROR,
        # [ doc = "Other error" ]
        OTHER_ERROR,
    }
    impl ERRC1_0R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            match *self {
                ERRC1_0R::BIT_ERROR => 0,
                ERRC1_0R::FORM_ERROR => 1,
                ERRC1_0R::STUFF_ERROR => 2,
                ERRC1_0R::OTHER_ERROR => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: u8) -> ERRC1_0R {
            match value {
                0 => ERRC1_0R::BIT_ERROR,
                1 => ERRC1_0R::FORM_ERROR,
                2 => ERRC1_0R::STUFF_ERROR,
                3 => ERRC1_0R::OTHER_ERROR,
                _ => unreachable!(),
            }
        }
        # [ doc = "Checks if the value of the field is `BIT_ERROR`" ]
        # [ inline ( always ) ]
        pub fn is_bit_error(&self) -> bool {
            *self == ERRC1_0R::BIT_ERROR
        }
        # [ doc = "Checks if the value of the field is `FORM_ERROR`" ]
        # [ inline ( always ) ]
        pub fn is_form_error(&self) -> bool {
            *self == ERRC1_0R::FORM_ERROR
        }
        # [ doc = "Checks if the value of the field is `STUFF_ERROR`" ]
        # [ inline ( always ) ]
        pub fn is_stuff_error(&self) -> bool {
            *self == ERRC1_0R::STUFF_ERROR
        }
        # [ doc = "Checks if the value of the field is `OTHER_ERROR`" ]
        # [ inline ( always ) ]
        pub fn is_other_error(&self) -> bool {
            *self == ERRC1_0R::OTHER_ERROR
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ALCBITR {
        bits: u8,
    }
    impl ALCBITR {
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
        # [ doc = "Bit 0 - Receive Interrupt. This bit is set whenever the RBS bit in CANxSR and the RIE bit in CANxIER are both 1, indicating that a new message was received and stored in the Receive Buffer. The Receive Interrupt Bit is not cleared upon a read access to the Interrupt Register. Giving the Command Release Receive Buffer will clear RI temporarily. If there is another message available within the Receive Buffer after the release command, RI is set again. Otherwise RI remains cleared." ]
        # [ inline ( always ) ]
        pub fn ri(&self) -> RIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RIR { bits }
        }
        # [ doc = "Bit 1 - Transmit Interrupt 1. This bit is set when the TBS1 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB1 was successfully transmitted or aborted), indicating that Transmit buffer 1 is available, and the TIE1 bit in CANxIER is 1." ]
        # [ inline ( always ) ]
        pub fn ti1(&self) -> TI1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TI1R { bits }
        }
        # [ doc = "Bit 2 - Error Warning Interrupt. This bit is set on every change (set or clear) of either the Error Status or Bus Status bit in CANxSR and the EIE bit bit is set within the Interrupt Enable Register at the time of the change." ]
        # [ inline ( always ) ]
        pub fn ei(&self) -> EIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EIR { bits }
        }
        # [ doc = "Bit 3 - Data Overrun Interrupt. This bit is set when the DOS bit in CANxSR goes from 0 to 1 and the DOIE bit in CANxIER is 1." ]
        # [ inline ( always ) ]
        pub fn doi(&self) -> DOIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DOIR { bits }
        }
        # [ doc = "Bit 4 - Wake-Up Interrupt. This bit is set if the CAN controller is sleeping and bus activity is detected and the WUIE bit in CANxIER is 1. A Wake-Up Interrupt is also generated if the CPU tries to set the Sleep bit while the CAN controller is involved in bus activities or a CAN Interrupt is pending. The WUI flag can also get asserted when the according enable bit WUIE is not set. In this case a Wake-Up Interrupt does not get asserted." ]
        # [ inline ( always ) ]
        pub fn wui(&self) -> WUIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WUIR { bits }
        }
        # [ doc = "Bit 5 - Error Passive Interrupt. This bit is set if the EPIE bit in CANxIER is 1, and the CAN controller switches between Error Passive and Error Active mode in either direction. This is the case when the CAN Controller has reached the Error Passive Status (at least one error counter exceeds the CAN protocol defined level of 127) or if the CAN Controller is in Error Passive Status and enters the Error Active Status again." ]
        # [ inline ( always ) ]
        pub fn epi(&self) -> EPIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EPIR { bits }
        }
        # [ doc = "Bit 6 - Arbitration Lost Interrupt. This bit is set if the ALIE bit in CANxIER is 1, and the CAN controller loses arbitration while attempting to transmit. In this case the CAN node becomes a receiver." ]
        # [ inline ( always ) ]
        pub fn ali(&self) -> ALIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ALIR { bits }
        }
        # [ doc = "Bit 7 - Bus Error Interrupt -- this bit is set if the BEIE bit in CANxIER is 1, and the CAN controller detects an error on the bus." ]
        # [ inline ( always ) ]
        pub fn bei(&self) -> BEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BEIR { bits }
        }
        # [ doc = "Bit 8 - ID Ready Interrupt -- this bit is set if the IDIE bit in CANxIER is 1, and a CAN Identifier has been received (a message was successfully transmitted or aborted). This bit is set whenever a message was successfully transmitted or aborted and the IDIE bit is set in the IER register." ]
        # [ inline ( always ) ]
        pub fn idi(&self) -> IDIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDIR { bits }
        }
        # [ doc = "Bit 9 - Transmit Interrupt 2. This bit is set when the TBS2 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB2 was successfully transmitted or aborted), indicating that Transmit buffer 2 is available, and the TIE2 bit in CANxIER is 1." ]
        # [ inline ( always ) ]
        pub fn ti2(&self) -> TI2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TI2R { bits }
        }
        # [ doc = "Bit 10 - Transmit Interrupt 3. This bit is set when the TBS3 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB3 was successfully transmitted or aborted), indicating that Transmit buffer 3 is available, and the TIE3 bit in CANxIER is 1." ]
        # [ inline ( always ) ]
        pub fn ti3(&self) -> TI3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TI3R { bits }
        }
        # [ doc = "Bits 16:20 - Error Code Capture: when the CAN controller detects a bus error, the location of the error within the frame is captured in this field. The value reflects an internal state variable, and as a result is not very linear: 00011 = Start of Frame 00010 = ID28 ... ID21 00110 = ID20 ... ID18 00100 = SRTR Bit 00101 = IDE bit 00111 = ID17 ... 13 01111 = ID12 ... ID5 01110 = ID4 ... ID0 01100 = RTR Bit 01101 = Reserved Bit 1 01001 = Reserved Bit 0 01011 = Data Length Code 01010 = Data Field 01000 = CRC Sequence 11000 = CRC Delimiter 11001 = Acknowledge Slot 11011 = Acknowledge Delimiter 11010 = End of Frame 10010 = Intermission Whenever a bus error occurs, the corresponding bus error interrupt is forced, if enabled. At the same time, the current position of the Bit Stream Processor is captured into the Error Code Capture Register. The content within this register is fixed until the user software has read out its content once. From now on, the capture mechanism is activated again, i.e. reading the CANxICR enables another Bus Error Interrupt." ]
        # [ inline ( always ) ]
        pub fn errbit4_0(&self) -> ERRBIT4_0R {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ERRBIT4_0R { bits }
        }
        # [ doc = "Bit 21 - When the CAN controller detects a bus error, the direction of the current bit is captured in this bit." ]
        # [ inline ( always ) ]
        pub fn errdir(&self) -> ERRDIRR {
            ERRDIRR::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 21;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bits 22:23 - When the CAN controller detects a bus error, the type of error is captured in this field:" ]
        # [ inline ( always ) ]
        pub fn errc1_0(&self) -> ERRC1_0R {
            ERRC1_0R::_from({
                                const MASK: u8 = 3;
                                const OFFSET: u8 = 22;
                                ((self.bits >> OFFSET) & MASK as u32) as u8
                            })
        }
        # [ doc = "Bits 24:31 - Each time arbitration is lost while trying to send on the CAN, the bit number within the frame is captured into this field. After the content of ALCBIT is read, the ALI bit is cleared and a new Arbitration Lost interrupt can occur. 00 = arbitration lost in the first bit (MS) of identifier ... 11 = arbitration lost in SRTS bit (RTR bit for standard frame messages) 12 = arbitration lost in IDE bit 13 = arbitration lost in 12th bit of identifier (extended frame only) ... 30 = arbitration lost in last bit of identifier (extended frame only) 31 = arbitration lost in RTR bit (extended frame only) On arbitration lost, the corresponding arbitration lost interrupt is forced, if enabled. At that time, the current bit position of the Bit Stream Processor is captured into the Arbitration Lost Capture Register. The content within this register is fixed until the user application has read out its contents once. From now on, the capture mechanism is activated again." ]
        # [ inline ( always ) ]
        pub fn alcbit(&self) -> ALCBITR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            ALCBITR { bits }
        }
    }
}
# [ doc = "Interrupt Enable" ]
pub struct IER {
    register: VolatileCell<u32>,
}
# [ doc = "Interrupt Enable" ]
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
    # [ doc = r" Value of the field" ]
    pub struct RIER {
        bits: bool,
    }
    impl RIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIE1R {
        bits: bool,
    }
    impl TIE1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EIER {
        bits: bool,
    }
    impl EIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DOIER {
        bits: bool,
    }
    impl DOIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct WUIER {
        bits: bool,
    }
    impl WUIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct EPIER {
        bits: bool,
    }
    impl EPIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ALIER {
        bits: bool,
    }
    impl ALIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BEIER {
        bits: bool,
    }
    impl BEIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct IDIER {
        bits: bool,
    }
    impl IDIER {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIE2R {
        bits: bool,
    }
    impl TIE2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TIE3R {
        bits: bool,
    }
    impl TIE3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _RIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIE1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIE1W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DOIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DOIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WUIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WUIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EPIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EPIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ALIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ALIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BEIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BEIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IDIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDIEW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIE2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIE2W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TIE3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TIE3W<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bit 0 - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn rie(&self) -> RIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RIER { bits }
        }
        # [ doc = "Bit 1 - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn tie1(&self) -> TIE1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIE1R { bits }
        }
        # [ doc = "Bit 2 - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn eie(&self) -> EIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EIER { bits }
        }
        # [ doc = "Bit 3 - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn doie(&self) -> DOIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DOIER { bits }
        }
        # [ doc = "Bit 4 - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested." ]
        # [ inline ( always ) ]
        pub fn wuie(&self) -> WUIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WUIER { bits }
        }
        # [ doc = "Bit 5 - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested." ]
        # [ inline ( always ) ]
        pub fn epie(&self) -> EPIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EPIER { bits }
        }
        # [ doc = "Bit 6 - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested." ]
        # [ inline ( always ) ]
        pub fn alie(&self) -> ALIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ALIER { bits }
        }
        # [ doc = "Bit 7 - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn beie(&self) -> BEIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BEIER { bits }
        }
        # [ doc = "Bit 8 - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn idie(&self) -> IDIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDIER { bits }
        }
        # [ doc = "Bit 9 - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn tie2(&self) -> TIE2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIE2R { bits }
        }
        # [ doc = "Bit 10 - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn tie3(&self) -> TIE3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TIE3R { bits }
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
        # [ doc = "Bit 0 - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn rie(&mut self) -> _RIEW {
            _RIEW { w: self }
        }
        # [ doc = "Bit 1 - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn tie1(&mut self) -> _TIE1W {
            _TIE1W { w: self }
        }
        # [ doc = "Bit 2 - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn eie(&mut self) -> _EIEW {
            _EIEW { w: self }
        }
        # [ doc = "Bit 3 - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn doie(&mut self) -> _DOIEW {
            _DOIEW { w: self }
        }
        # [ doc = "Bit 4 - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested." ]
        # [ inline ( always ) ]
        pub fn wuie(&mut self) -> _WUIEW {
            _WUIEW { w: self }
        }
        # [ doc = "Bit 5 - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested." ]
        # [ inline ( always ) ]
        pub fn epie(&mut self) -> _EPIEW {
            _EPIEW { w: self }
        }
        # [ doc = "Bit 6 - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested." ]
        # [ inline ( always ) ]
        pub fn alie(&mut self) -> _ALIEW {
            _ALIEW { w: self }
        }
        # [ doc = "Bit 7 - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn beie(&mut self) -> _BEIEW {
            _BEIEW { w: self }
        }
        # [ doc = "Bit 8 - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn idie(&mut self) -> _IDIEW {
            _IDIEW { w: self }
        }
        # [ doc = "Bit 9 - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn tie2(&mut self) -> _TIE2W {
            _TIE2W { w: self }
        }
        # [ doc = "Bit 10 - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt." ]
        # [ inline ( always ) ]
        pub fn tie3(&mut self) -> _TIE3W {
            _TIE3W { w: self }
        }
    }
}
# [ doc = "Bus Timing. Can only be written when RM in CANMOD is 1." ]
pub struct BTR {
    register: VolatileCell<u32>,
}
# [ doc = "Bus Timing. Can only be written when RM in CANMOD is 1." ]
pub mod btr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::BTR {
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
    pub struct BRPR {
        bits: u16,
    }
    impl BRPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct SJWR {
        bits: u8,
    }
    impl SJWR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TESG1R {
        bits: u8,
    }
    impl TESG1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct TESG2R {
        bits: u8,
    }
    impl TESG2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Possible values of the field `SAM`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum SAMR {
        # [ doc = "The bus is sampled once (recommended for high speed buses)" ]
        THE_BUS_IS_SAMPLED_O,
        # [ doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)" ]
        THE_BUS_IS_SAMPLED_3,
    }
    impl SAMR {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                SAMR::THE_BUS_IS_SAMPLED_O => false,
                SAMR::THE_BUS_IS_SAMPLED_3 => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> SAMR {
            match value {
                false => SAMR::THE_BUS_IS_SAMPLED_O,
                true => SAMR::THE_BUS_IS_SAMPLED_3,
            }
        }
        # [ doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_O`" ]
        # [ inline ( always ) ]
        pub fn is_the_bus_is_sampled_o(&self) -> bool {
            *self == SAMR::THE_BUS_IS_SAMPLED_O
        }
        # [ doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_3`" ]
        # [ inline ( always ) ]
        pub fn is_the_bus_is_sampled_3(&self) -> bool {
            *self == SAMR::THE_BUS_IS_SAMPLED_3
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BRPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BRPW<'a> {
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
    pub struct _SJWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SJWW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TESG1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TESG1W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TESG2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TESG2W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = "Values that can be written to the field `SAM`" ]
    pub enum SAMW {
        # [ doc = "The bus is sampled once (recommended for high speed buses)" ]
        THE_BUS_IS_SAMPLED_O,
        # [ doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)" ]
        THE_BUS_IS_SAMPLED_3,
    }
    impl SAMW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> bool {
            match *self {
                SAMW::THE_BUS_IS_SAMPLED_O => false,
                SAMW::THE_BUS_IS_SAMPLED_3 => true,
            }
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SAMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SAMW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        # [ inline ( always ) ]
        pub fn variant(self, variant: SAMW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        # [ doc = "The bus is sampled once (recommended for high speed buses)" ]
        # [ inline ( always ) ]
        pub fn the_bus_is_sampled_o(self) -> &'a mut W {
            self.variant(SAMW::THE_BUS_IS_SAMPLED_O)
        }
        # [ doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)" ]
        # [ inline ( always ) ]
        pub fn the_bus_is_sampled_3(self) -> &'a mut W {
            self.variant(SAMW::THE_BUS_IS_SAMPLED_3)
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
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock." ]
        # [ inline ( always ) ]
        pub fn brp(&self) -> BRPR {
            let bits = {
                const MASK: u16 = 1023;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            BRPR { bits }
        }
        # [ doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks." ]
        # [ inline ( always ) ]
        pub fn sjw(&self) -> SJWR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SJWR { bits }
        }
        # [ doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks." ]
        # [ inline ( always ) ]
        pub fn tesg1(&self) -> TESG1R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TESG1R { bits }
        }
        # [ doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks." ]
        # [ inline ( always ) ]
        pub fn tesg2(&self) -> TESG2R {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            TESG2R { bits }
        }
        # [ doc = "Bit 23 - Sampling" ]
        # [ inline ( always ) ]
        pub fn sam(&self) -> SAMR {
            SAMR::_from({
                            const MASK: bool = true;
                            const OFFSET: u8 = 23;
                            ((self.bits >> OFFSET) & MASK as u32) != 0
                        })
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 1835008 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock." ]
        # [ inline ( always ) ]
        pub fn brp(&mut self) -> _BRPW {
            _BRPW { w: self }
        }
        # [ doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks." ]
        # [ inline ( always ) ]
        pub fn sjw(&mut self) -> _SJWW {
            _SJWW { w: self }
        }
        # [ doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks." ]
        # [ inline ( always ) ]
        pub fn tesg1(&mut self) -> _TESG1W {
            _TESG1W { w: self }
        }
        # [ doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks." ]
        # [ inline ( always ) ]
        pub fn tesg2(&mut self) -> _TESG2W {
            _TESG2W { w: self }
        }
        # [ doc = "Bit 23 - Sampling" ]
        # [ inline ( always ) ]
        pub fn sam(&mut self) -> _SAMW {
            _SAMW { w: self }
        }
    }
}
# [ doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1." ]
pub struct EWL {
    register: VolatileCell<u32>,
}
# [ doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1." ]
pub mod ewl {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::EWL {
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
    pub struct EWLR {
        bits: u8,
    }
    impl EWLR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EWLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EWLW<'a> {
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
        # [ doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set." ]
        # [ inline ( always ) ]
        pub fn ewl(&self) -> EWLR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EWLR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 96 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set." ]
        # [ inline ( always ) ]
        pub fn ewl(&mut self) -> _EWLW {
            _EWLW { w: self }
        }
    }
}
# [ doc = "Status Register" ]
pub struct SR {
    register: VolatileCell<u32>,
}
# [ doc = "Status Register" ]
pub mod sr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::SR {
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RBS_1R {
        bits: bool,
    }
    impl RBS_1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DOS_1R {
        bits: bool,
    }
    impl DOS_1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Possible values of the field `TBS1_1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TBS1_1R {
        # [ doc = "Locked. Software cannot access the Tx Buffer 1 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process." ]
        LOCKED_SOFTWARE_CAN,
        # [ doc = "Released. Software may write a message into the Transmit Buffer 1 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers." ]
        RELEASED_SOFTWARE_M,
    }
    impl TBS1_1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TBS1_1R::LOCKED_SOFTWARE_CAN => false,
                TBS1_1R::RELEASED_SOFTWARE_M => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TBS1_1R {
            match value {
                false => TBS1_1R::LOCKED_SOFTWARE_CAN,
                true => TBS1_1R::RELEASED_SOFTWARE_M,
            }
        }
        # [ doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`" ]
        # [ inline ( always ) ]
        pub fn is_locked_software_can(&self) -> bool {
            *self == TBS1_1R::LOCKED_SOFTWARE_CAN
        }
        # [ doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`" ]
        # [ inline ( always ) ]
        pub fn is_released_software_m(&self) -> bool {
            *self == TBS1_1R::RELEASED_SOFTWARE_M
        }
    }
    # [ doc = "Possible values of the field `TCS1_1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TCS1_1R {
        # [ doc = "Incomplete. The previously requested transmission for Tx Buffer 1 is not complete." ]
        INCOMPLETE_THE_PREV,
        # [ doc = "Complete. The previously requested transmission for Tx Buffer 1 has been successfully completed." ]
        COMPLETE_THE_PREVIO,
    }
    impl TCS1_1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TCS1_1R::INCOMPLETE_THE_PREV => false,
                TCS1_1R::COMPLETE_THE_PREVIO => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TCS1_1R {
            match value {
                false => TCS1_1R::INCOMPLETE_THE_PREV,
                true => TCS1_1R::COMPLETE_THE_PREVIO,
            }
        }
        # [ doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`" ]
        # [ inline ( always ) ]
        pub fn is_incomplete_the_prev(&self) -> bool {
            *self == TCS1_1R::INCOMPLETE_THE_PREV
        }
        # [ doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`" ]
        # [ inline ( always ) ]
        pub fn is_complete_the_previo(&self) -> bool {
            *self == TCS1_1R::COMPLETE_THE_PREVIO
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RS_1R {
        bits: bool,
    }
    impl RS_1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Possible values of the field `TS1_1`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TS1_1R {
        # [ doc = "Idle. There is no transmission from Tx Buffer 1." ]
        IDLE_THERE_IS_NO_TR,
        # [ doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 1." ]
        TRANSMIT_THE_CAN_CO,
    }
    impl TS1_1R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TS1_1R::IDLE_THERE_IS_NO_TR => false,
                TS1_1R::TRANSMIT_THE_CAN_CO => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TS1_1R {
            match value {
                false => TS1_1R::IDLE_THERE_IS_NO_TR,
                true => TS1_1R::TRANSMIT_THE_CAN_CO,
            }
        }
        # [ doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`" ]
        # [ inline ( always ) ]
        pub fn is_idle_there_is_no_tr(&self) -> bool {
            *self == TS1_1R::IDLE_THERE_IS_NO_TR
        }
        # [ doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`" ]
        # [ inline ( always ) ]
        pub fn is_transmit_the_can_co(&self) -> bool {
            *self == TS1_1R::TRANSMIT_THE_CAN_CO
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ES_1R {
        bits: bool,
    }
    impl ES_1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BS_1R {
        bits: bool,
    }
    impl BS_1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RBS_2R {
        bits: bool,
    }
    impl RBS_2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DOS_2R {
        bits: bool,
    }
    impl DOS_2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Possible values of the field `TBS2_2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TBS2_2R {
        # [ doc = "Locked. Software cannot access the Tx Buffer 2 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process." ]
        LOCKED_SOFTWARE_CAN,
        # [ doc = "Released. Software may write a message into the Transmit Buffer 2 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers." ]
        RELEASED_SOFTWARE_M,
    }
    impl TBS2_2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TBS2_2R::LOCKED_SOFTWARE_CAN => false,
                TBS2_2R::RELEASED_SOFTWARE_M => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TBS2_2R {
            match value {
                false => TBS2_2R::LOCKED_SOFTWARE_CAN,
                true => TBS2_2R::RELEASED_SOFTWARE_M,
            }
        }
        # [ doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`" ]
        # [ inline ( always ) ]
        pub fn is_locked_software_can(&self) -> bool {
            *self == TBS2_2R::LOCKED_SOFTWARE_CAN
        }
        # [ doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`" ]
        # [ inline ( always ) ]
        pub fn is_released_software_m(&self) -> bool {
            *self == TBS2_2R::RELEASED_SOFTWARE_M
        }
    }
    # [ doc = "Possible values of the field `TCS2_2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TCS2_2R {
        # [ doc = "Incomplete. The previously requested transmission for Tx Buffer 2 is not complete." ]
        INCOMPLETE_THE_PREV,
        # [ doc = "Complete. The previously requested transmission for Tx Buffer 2 has been successfully completed." ]
        COMPLETE_THE_PREVIO,
    }
    impl TCS2_2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TCS2_2R::INCOMPLETE_THE_PREV => false,
                TCS2_2R::COMPLETE_THE_PREVIO => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TCS2_2R {
            match value {
                false => TCS2_2R::INCOMPLETE_THE_PREV,
                true => TCS2_2R::COMPLETE_THE_PREVIO,
            }
        }
        # [ doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`" ]
        # [ inline ( always ) ]
        pub fn is_incomplete_the_prev(&self) -> bool {
            *self == TCS2_2R::INCOMPLETE_THE_PREV
        }
        # [ doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`" ]
        # [ inline ( always ) ]
        pub fn is_complete_the_previo(&self) -> bool {
            *self == TCS2_2R::COMPLETE_THE_PREVIO
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RS_2R {
        bits: bool,
    }
    impl RS_2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Possible values of the field `TS2_2`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TS2_2R {
        # [ doc = "Idle. There is no transmission from Tx Buffer 2." ]
        IDLE_THERE_IS_NO_TR,
        # [ doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 2." ]
        TRANSMIT_THE_CAN_CO,
    }
    impl TS2_2R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TS2_2R::IDLE_THERE_IS_NO_TR => false,
                TS2_2R::TRANSMIT_THE_CAN_CO => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TS2_2R {
            match value {
                false => TS2_2R::IDLE_THERE_IS_NO_TR,
                true => TS2_2R::TRANSMIT_THE_CAN_CO,
            }
        }
        # [ doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`" ]
        # [ inline ( always ) ]
        pub fn is_idle_there_is_no_tr(&self) -> bool {
            *self == TS2_2R::IDLE_THERE_IS_NO_TR
        }
        # [ doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`" ]
        # [ inline ( always ) ]
        pub fn is_transmit_the_can_co(&self) -> bool {
            *self == TS2_2R::TRANSMIT_THE_CAN_CO
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ES_2R {
        bits: bool,
    }
    impl ES_2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BS_2R {
        bits: bool,
    }
    impl BS_2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RBS_3R {
        bits: bool,
    }
    impl RBS_3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DOS_3R {
        bits: bool,
    }
    impl DOS_3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Possible values of the field `TBS3_3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TBS3_3R {
        # [ doc = "Locked. Software cannot access the Tx Buffer 3 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process." ]
        LOCKED_SOFTWARE_CAN,
        # [ doc = "Released. Software may write a message into the Transmit Buffer 3 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers." ]
        RELEASED_SOFTWARE_M,
    }
    impl TBS3_3R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TBS3_3R::LOCKED_SOFTWARE_CAN => false,
                TBS3_3R::RELEASED_SOFTWARE_M => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TBS3_3R {
            match value {
                false => TBS3_3R::LOCKED_SOFTWARE_CAN,
                true => TBS3_3R::RELEASED_SOFTWARE_M,
            }
        }
        # [ doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`" ]
        # [ inline ( always ) ]
        pub fn is_locked_software_can(&self) -> bool {
            *self == TBS3_3R::LOCKED_SOFTWARE_CAN
        }
        # [ doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`" ]
        # [ inline ( always ) ]
        pub fn is_released_software_m(&self) -> bool {
            *self == TBS3_3R::RELEASED_SOFTWARE_M
        }
    }
    # [ doc = "Possible values of the field `TCS3_3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TCS3_3R {
        # [ doc = "Incomplete. The previously requested transmission for Tx Buffer 3 is not complete." ]
        INCOMPLETE_THE_PREV,
        # [ doc = "Complete. The previously requested transmission for Tx Buffer 3 has been successfully completed." ]
        COMPLETE_THE_PREVIO,
    }
    impl TCS3_3R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TCS3_3R::INCOMPLETE_THE_PREV => false,
                TCS3_3R::COMPLETE_THE_PREVIO => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TCS3_3R {
            match value {
                false => TCS3_3R::INCOMPLETE_THE_PREV,
                true => TCS3_3R::COMPLETE_THE_PREVIO,
            }
        }
        # [ doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`" ]
        # [ inline ( always ) ]
        pub fn is_incomplete_the_prev(&self) -> bool {
            *self == TCS3_3R::INCOMPLETE_THE_PREV
        }
        # [ doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`" ]
        # [ inline ( always ) ]
        pub fn is_complete_the_previo(&self) -> bool {
            *self == TCS3_3R::COMPLETE_THE_PREVIO
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RS_3R {
        bits: bool,
    }
    impl RS_3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = "Possible values of the field `TS3_3`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TS3_3R {
        # [ doc = "Idle. There is no transmission from Tx Buffer 3." ]
        IDLE_THERE_IS_NO_TR,
        # [ doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 3." ]
        TRANSMIT_THE_CAN_CO,
    }
    impl TS3_3R {
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
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
                TS3_3R::IDLE_THERE_IS_NO_TR => false,
                TS3_3R::TRANSMIT_THE_CAN_CO => true,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(value: bool) -> TS3_3R {
            match value {
                false => TS3_3R::IDLE_THERE_IS_NO_TR,
                true => TS3_3R::TRANSMIT_THE_CAN_CO,
            }
        }
        # [ doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`" ]
        # [ inline ( always ) ]
        pub fn is_idle_there_is_no_tr(&self) -> bool {
            *self == TS3_3R::IDLE_THERE_IS_NO_TR
        }
        # [ doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`" ]
        # [ inline ( always ) ]
        pub fn is_transmit_the_can_co(&self) -> bool {
            *self == TS3_3R::TRANSMIT_THE_CAN_CO
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct ES_3R {
        bits: bool,
    }
    impl ES_3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BS_3R {
        bits: bool,
    }
    impl BS_3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
        # [ doc = "Bit 0 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn rbs_1(&self) -> RBS_1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RBS_1R { bits }
        }
        # [ doc = "Bit 1 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn dos_1(&self) -> DOS_1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DOS_1R { bits }
        }
        # [ doc = "Bit 2 - Transmit Buffer Status 1." ]
        # [ inline ( always ) ]
        pub fn tbs1_1(&self) -> TBS1_1R {
            TBS1_1R::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 2;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 3 - Transmission Complete Status." ]
        # [ inline ( always ) ]
        pub fn tcs1_1(&self) -> TCS1_1R {
            TCS1_1R::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 3;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 4 - Receive Status. This bit is identical to the RS bit in the GSR." ]
        # [ inline ( always ) ]
        pub fn rs_1(&self) -> RS_1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RS_1R { bits }
        }
        # [ doc = "Bit 5 - Transmit Status 1." ]
        # [ inline ( always ) ]
        pub fn ts1_1(&self) -> TS1_1R {
            TS1_1R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 5;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 6 - Error Status. This bit is identical to the ES bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn es_1(&self) -> ES_1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ES_1R { bits }
        }
        # [ doc = "Bit 7 - Bus Status. This bit is identical to the BS bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn bs_1(&self) -> BS_1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BS_1R { bits }
        }
        # [ doc = "Bit 8 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn rbs_2(&self) -> RBS_2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RBS_2R { bits }
        }
        # [ doc = "Bit 9 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn dos_2(&self) -> DOS_2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DOS_2R { bits }
        }
        # [ doc = "Bit 10 - Transmit Buffer Status 2." ]
        # [ inline ( always ) ]
        pub fn tbs2_2(&self) -> TBS2_2R {
            TBS2_2R::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 10;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 11 - Transmission Complete Status." ]
        # [ inline ( always ) ]
        pub fn tcs2_2(&self) -> TCS2_2R {
            TCS2_2R::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 11;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 12 - Receive Status. This bit is identical to the RS bit in the GSR." ]
        # [ inline ( always ) ]
        pub fn rs_2(&self) -> RS_2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RS_2R { bits }
        }
        # [ doc = "Bit 13 - Transmit Status 2." ]
        # [ inline ( always ) ]
        pub fn ts2_2(&self) -> TS2_2R {
            TS2_2R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 13;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 14 - Error Status. This bit is identical to the ES bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn es_2(&self) -> ES_2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ES_2R { bits }
        }
        # [ doc = "Bit 15 - Bus Status. This bit is identical to the BS bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn bs_2(&self) -> BS_2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BS_2R { bits }
        }
        # [ doc = "Bit 16 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn rbs_3(&self) -> RBS_3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RBS_3R { bits }
        }
        # [ doc = "Bit 17 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn dos_3(&self) -> DOS_3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DOS_3R { bits }
        }
        # [ doc = "Bit 18 - Transmit Buffer Status 3." ]
        # [ inline ( always ) ]
        pub fn tbs3_3(&self) -> TBS3_3R {
            TBS3_3R::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 18;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 19 - Transmission Complete Status." ]
        # [ inline ( always ) ]
        pub fn tcs3_3(&self) -> TCS3_3R {
            TCS3_3R::_from({
                               const MASK: bool = true;
                               const OFFSET: u8 = 19;
                               ((self.bits >> OFFSET) & MASK as u32) != 0
                           })
        }
        # [ doc = "Bit 20 - Receive Status. This bit is identical to the RS bit in the GSR." ]
        # [ inline ( always ) ]
        pub fn rs_3(&self) -> RS_3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RS_3R { bits }
        }
        # [ doc = "Bit 21 - Transmit Status 3." ]
        # [ inline ( always ) ]
        pub fn ts3_3(&self) -> TS3_3R {
            TS3_3R::_from({
                              const MASK: bool = true;
                              const OFFSET: u8 = 21;
                              ((self.bits >> OFFSET) & MASK as u32) != 0
                          })
        }
        # [ doc = "Bit 22 - Error Status. This bit is identical to the ES bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn es_3(&self) -> ES_3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ES_3R { bits }
        }
        # [ doc = "Bit 23 - Bus Status. This bit is identical to the BS bit in the CANxGSR." ]
        # [ inline ( always ) ]
        pub fn bs_3(&self) -> BS_3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BS_3R { bits }
        }
    }
}
# [ doc = "Receive frame status. Can only be written when RM in CANMOD is 1." ]
pub struct RFS {
    register: VolatileCell<u32>,
}
# [ doc = "Receive frame status. Can only be written when RM in CANMOD is 1." ]
pub mod rfs {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RFS {
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
    pub struct IDINDEXR {
        bits: u16,
    }
    impl IDINDEXR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct BPR {
        bits: bool,
    }
    impl BPR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DLCR {
        bits: u8,
    }
    impl DLCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RTRR {
        bits: bool,
    }
    impl RTRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FFR {
        bits: bool,
    }
    impl FFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _IDINDEXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDINDEXW<'a> {
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
    pub struct _BPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BPW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DLCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DLCW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RTRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RTRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _FFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bits 0:9 - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values." ]
        # [ inline ( always ) ]
        pub fn idindex(&self) -> IDINDEXR {
            let bits = {
                const MASK: u16 = 1023;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            IDINDEXR { bits }
        }
        # [ doc = "Bit 10 - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless." ]
        # [ inline ( always ) ]
        pub fn bp(&self) -> BPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BPR { bits }
        }
        # [ doc = "Bits 16:19 - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding." ]
        # [ inline ( always ) ]
        pub fn dlc(&self) -> DLCR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DLCR { bits }
        }
        # [ doc = "Bit 30 - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier." ]
        # [ inline ( always ) ]
        pub fn rtr(&self) -> RTRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RTRR { bits }
        }
        # [ doc = "Bit 31 - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below." ]
        # [ inline ( always ) ]
        pub fn ff(&self) -> FFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FFR { bits }
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
        # [ doc = "Bits 0:9 - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values." ]
        # [ inline ( always ) ]
        pub fn idindex(&mut self) -> _IDINDEXW {
            _IDINDEXW { w: self }
        }
        # [ doc = "Bit 10 - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless." ]
        # [ inline ( always ) ]
        pub fn bp(&mut self) -> _BPW {
            _BPW { w: self }
        }
        # [ doc = "Bits 16:19 - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding." ]
        # [ inline ( always ) ]
        pub fn dlc(&mut self) -> _DLCW {
            _DLCW { w: self }
        }
        # [ doc = "Bit 30 - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier." ]
        # [ inline ( always ) ]
        pub fn rtr(&mut self) -> _RTRW {
            _RTRW { w: self }
        }
        # [ doc = "Bit 31 - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below." ]
        # [ inline ( always ) ]
        pub fn ff(&mut self) -> _FFW {
            _FFW { w: self }
        }
    }
}
# [ doc = "Received Identifier. Can only be written when RM in CANMOD is 1." ]
pub struct RID {
    register: VolatileCell<u32>,
}
# [ doc = "Received Identifier. Can only be written when RM in CANMOD is 1." ]
pub mod rid {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RID {
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
    pub struct IDR {
        bits: u16,
    }
    impl IDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 2047;
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
        # [ doc = "Bits 0:10 - The 11-bit Identifier field of the current received message. In CAN 2.0A, these bits are called ID10-0, while in CAN 2.0B they're called ID29-18." ]
        # [ inline ( always ) ]
        pub fn id(&self) -> IDR {
            let bits = {
                const MASK: u16 = 2047;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            IDR { bits }
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
        # [ doc = "Bits 0:10 - The 11-bit Identifier field of the current received message. In CAN 2.0A, these bits are called ID10-0, while in CAN 2.0B they're called ID29-18." ]
        # [ inline ( always ) ]
        pub fn id(&mut self) -> _IDW {
            _IDW { w: self }
        }
    }
}
# [ doc = "Received data bytes 1-4. Can only be written when RM in CANMOD is 1." ]
pub struct RDA {
    register: VolatileCell<u32>,
}
# [ doc = "Received data bytes 1-4. Can only be written when RM in CANMOD is 1." ]
pub mod rda {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RDA {
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
    pub struct DATA1R {
        bits: u8,
    }
    impl DATA1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA2R {
        bits: u8,
    }
    impl DATA2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA3R {
        bits: u8,
    }
    impl DATA3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA4R {
        bits: u8,
    }
    impl DATA4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATA1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA1W<'a> {
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
    pub struct _DATA2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA2W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DATA3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA3W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DATA4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA4W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
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
        # [ doc = "Bits 0:7 - Data 1. If the DLC field in CANRFS >= 0001, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data1(&self) -> DATA1R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA1R { bits }
        }
        # [ doc = "Bits 8:15 - Data 2. If the DLC field in CANRFS >= 0010, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data2(&self) -> DATA2R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA2R { bits }
        }
        # [ doc = "Bits 16:23 - Data 3. If the DLC field in CANRFS >= 0011, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data3(&self) -> DATA3R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA3R { bits }
        }
        # [ doc = "Bits 24:31 - Data 4. If the DLC field in CANRFS >= 0100, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data4(&self) -> DATA4R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA4R { bits }
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
        # [ doc = "Bits 0:7 - Data 1. If the DLC field in CANRFS >= 0001, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data1(&mut self) -> _DATA1W {
            _DATA1W { w: self }
        }
        # [ doc = "Bits 8:15 - Data 2. If the DLC field in CANRFS >= 0010, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data2(&mut self) -> _DATA2W {
            _DATA2W { w: self }
        }
        # [ doc = "Bits 16:23 - Data 3. If the DLC field in CANRFS >= 0011, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data3(&mut self) -> _DATA3W {
            _DATA3W { w: self }
        }
        # [ doc = "Bits 24:31 - Data 4. If the DLC field in CANRFS >= 0100, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data4(&mut self) -> _DATA4W {
            _DATA4W { w: self }
        }
    }
}
# [ doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1." ]
pub struct RDB {
    register: VolatileCell<u32>,
}
# [ doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1." ]
pub mod rdb {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::RDB {
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
    pub struct DATA5R {
        bits: u8,
    }
    impl DATA5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA6R {
        bits: u8,
    }
    impl DATA6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA7R {
        bits: u8,
    }
    impl DATA7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA8R {
        bits: u8,
    }
    impl DATA8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATA5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA5W<'a> {
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
    pub struct _DATA6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA6W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DATA7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA7W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DATA8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA8W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
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
        # [ doc = "Bits 0:7 - Data 5. If the DLC field in CANRFS >= 0101, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data5(&self) -> DATA5R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA5R { bits }
        }
        # [ doc = "Bits 8:15 - Data 6. If the DLC field in CANRFS >= 0110, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data6(&self) -> DATA6R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA6R { bits }
        }
        # [ doc = "Bits 16:23 - Data 7. If the DLC field in CANRFS >= 0111, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data7(&self) -> DATA7R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA7R { bits }
        }
        # [ doc = "Bits 24:31 - Data 8. If the DLC field in CANRFS >= 1000, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data8(&self) -> DATA8R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA8R { bits }
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
        # [ doc = "Bits 0:7 - Data 5. If the DLC field in CANRFS >= 0101, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data5(&mut self) -> _DATA5W {
            _DATA5W { w: self }
        }
        # [ doc = "Bits 8:15 - Data 6. If the DLC field in CANRFS >= 0110, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data6(&mut self) -> _DATA6W {
            _DATA6W { w: self }
        }
        # [ doc = "Bits 16:23 - Data 7. If the DLC field in CANRFS >= 0111, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data7(&mut self) -> _DATA7W {
            _DATA7W { w: self }
        }
        # [ doc = "Bits 24:31 - Data 8. If the DLC field in CANRFS >= 1000, this contains the first Data byte of the current received message." ]
        # [ inline ( always ) ]
        pub fn data8(&mut self) -> _DATA8W {
            _DATA8W { w: self }
        }
    }
}
# [ doc = "Transmit frame info (Tx Buffer )" ]
pub struct TFI {
    register: VolatileCell<u32>,
}
# [ doc = "Transmit frame info (Tx Buffer )" ]
pub mod tfi {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TFI {
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
    pub struct PRIOR {
        bits: u8,
    }
    impl PRIOR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DLCR {
        bits: u8,
    }
    impl DLCR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct RTRR {
        bits: bool,
    }
    impl RTRR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
        pub fn is_clear(&self) -> bool {
            !self.bit()
        }
        # [ doc = r" Returns `true` if the bit is set (1)" ]
        # [ inline ( always ) ]
        pub fn is_set(&self) -> bool {
            self.bit()
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct FFR {
        bits: bool,
    }
    impl FFR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bit(&self) -> bool {
            self.bits
        }
        # [ doc = r" Returns `true` if the bit is clear (0)" ]
        # [ inline ( always ) ]
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
    pub struct _PRIOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PRIOW<'a> {
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
    pub struct _DLCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DLCW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RTRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RTRW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _FFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FFW<'a> {
        # [ doc = r" Sets the field bit" ]
        pub fn set(self) -> &'a mut W {
            self.bit(true)
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
        # [ doc = "Bits 0:7 - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first." ]
        # [ inline ( always ) ]
        pub fn prio(&self) -> PRIOR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PRIOR { bits }
        }
        # [ doc = "Bits 16:19 - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes" ]
        # [ inline ( always ) ]
        pub fn dlc(&self) -> DLCR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DLCR { bits }
        }
        # [ doc = "Bit 30 - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes." ]
        # [ inline ( always ) ]
        pub fn rtr(&self) -> RTRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RTRR { bits }
        }
        # [ doc = "Bit 31 - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)." ]
        # [ inline ( always ) ]
        pub fn ff(&self) -> FFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FFR { bits }
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
        # [ doc = "Bits 0:7 - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first." ]
        # [ inline ( always ) ]
        pub fn prio(&mut self) -> _PRIOW {
            _PRIOW { w: self }
        }
        # [ doc = "Bits 16:19 - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes" ]
        # [ inline ( always ) ]
        pub fn dlc(&mut self) -> _DLCW {
            _DLCW { w: self }
        }
        # [ doc = "Bit 30 - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes." ]
        # [ inline ( always ) ]
        pub fn rtr(&mut self) -> _RTRW {
            _RTRW { w: self }
        }
        # [ doc = "Bit 31 - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)." ]
        # [ inline ( always ) ]
        pub fn ff(&mut self) -> _FFW {
            _FFW { w: self }
        }
    }
}
# [ doc = "Transmit Identifier (Tx Buffer)" ]
pub struct TID {
    register: VolatileCell<u32>,
}
# [ doc = "Transmit Identifier (Tx Buffer)" ]
pub mod tid {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TID {
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
    pub struct IDR {
        bits: u16,
    }
    impl IDR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 2047;
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
        # [ doc = "Bits 0:10 - The 11-bit Identifier to be sent in the next transmit message." ]
        # [ inline ( always ) ]
        pub fn id(&self) -> IDR {
            let bits = {
                const MASK: u16 = 2047;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            IDR { bits }
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
        # [ doc = "Bits 0:10 - The 11-bit Identifier to be sent in the next transmit message." ]
        # [ inline ( always ) ]
        pub fn id(&mut self) -> _IDW {
            _IDW { w: self }
        }
    }
}
# [ doc = "Transmit data bytes 1-4 (Tx Buffer)" ]
pub struct TDA {
    register: VolatileCell<u32>,
}
# [ doc = "Transmit data bytes 1-4 (Tx Buffer)" ]
pub mod tda {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TDA {
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
    pub struct DATA1R {
        bits: u8,
    }
    impl DATA1R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA2R {
        bits: u8,
    }
    impl DATA2R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA3R {
        bits: u8,
    }
    impl DATA3R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA4R {
        bits: u8,
    }
    impl DATA4R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATA1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA1W<'a> {
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
    pub struct _DATA2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA2W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DATA3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA3W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DATA4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA4W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
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
        # [ doc = "Bits 0:7 - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data1(&self) -> DATA1R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA1R { bits }
        }
        # [ doc = "Bits 8:15 - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data2(&self) -> DATA2R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA2R { bits }
        }
        # [ doc = "Bits 16:23 - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data3(&self) -> DATA3R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA3R { bits }
        }
        # [ doc = "Bits 24:31 - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data4(&self) -> DATA4R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA4R { bits }
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
        # [ doc = "Bits 0:7 - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data1(&mut self) -> _DATA1W {
            _DATA1W { w: self }
        }
        # [ doc = "Bits 8:15 - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data2(&mut self) -> _DATA2W {
            _DATA2W { w: self }
        }
        # [ doc = "Bits 16:23 - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data3(&mut self) -> _DATA3W {
            _DATA3W { w: self }
        }
        # [ doc = "Bits 24:31 - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data4(&mut self) -> _DATA4W {
            _DATA4W { w: self }
        }
    }
}
# [ doc = "Transmit data bytes 5-8 (Tx Buffer )" ]
pub struct TDB {
    register: VolatileCell<u32>,
}
# [ doc = "Transmit data bytes 5-8 (Tx Buffer )" ]
pub mod tdb {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::TDB {
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
    pub struct DATA5R {
        bits: u8,
    }
    impl DATA5R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA6R {
        bits: u8,
    }
    impl DATA6R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA7R {
        bits: u8,
    }
    impl DATA7R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct DATA8R {
        bits: u8,
    }
    impl DATA8R {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DATA5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA5W<'a> {
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
    pub struct _DATA6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA6W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DATA7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA7W<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _DATA8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATA8W<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
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
        # [ doc = "Bits 0:7 - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data5(&self) -> DATA5R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA5R { bits }
        }
        # [ doc = "Bits 8:15 - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data6(&self) -> DATA6R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA6R { bits }
        }
        # [ doc = "Bits 16:23 - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data7(&self) -> DATA7R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA7R { bits }
        }
        # [ doc = "Bits 24:31 - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data8(&self) -> DATA8R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DATA8R { bits }
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
        # [ doc = "Bits 0:7 - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data5(&mut self) -> _DATA5W {
            _DATA5W { w: self }
        }
        # [ doc = "Bits 8:15 - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data6(&mut self) -> _DATA6W {
            _DATA6W { w: self }
        }
        # [ doc = "Bits 16:23 - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data7(&mut self) -> _DATA7W {
            _DATA7W { w: self }
        }
        # [ doc = "Bits 24:31 - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message." ]
        # [ inline ( always ) ]
        pub fn data8(&mut self) -> _DATA8W {
            _DATA8W { w: self }
        }
    }
}
# [ doc = "CAN1 controller" ]
pub struct CAN1 {
    register_block: RegisterBlock,
}
impl Deref for CAN1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
# [ doc = "CAN2" ]
pub const CAN2: Peripheral<CAN2> = unsafe { Peripheral::new(1074036736) };
# [ doc = r" Register block" ]
pub struct CAN2 {
    register_block: RegisterBlock,
}
impl Deref for CAN2 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
