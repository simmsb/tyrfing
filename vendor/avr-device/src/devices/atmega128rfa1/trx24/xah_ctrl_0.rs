#[doc = "Register `XAH_CTRL_0` reader"]
pub type R = crate::R<XAH_CTRL_0_SPEC>;
#[doc = "Register `XAH_CTRL_0` writer"]
pub type W = crate::W<XAH_CTRL_0_SPEC>;
#[doc = "Field `SLOTTED_OPERATION` reader - Set Slotted Acknowledgment"]
pub type SLOTTED_OPERATION_R = crate::BitReader<SLOTTED_OPERATION_A>;
#[doc = "Set Slotted Acknowledgment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLOTTED_OPERATION_A {
    #[doc = "0: The radio transceiver operates in unslotted mode. An acknowledgment frame is automatically sent if requested."]
    SLOTTED_OP_DIS = 0,
    #[doc = "1: The transmission of an acknowledgment frame has to be controlled by the microcontroller."]
    SLOTTED_OP_EN = 1,
}
impl From<SLOTTED_OPERATION_A> for bool {
    #[inline(always)]
    fn from(variant: SLOTTED_OPERATION_A) -> Self {
        variant as u8 != 0
    }
}
impl SLOTTED_OPERATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLOTTED_OPERATION_A {
        match self.bits {
            false => SLOTTED_OPERATION_A::SLOTTED_OP_DIS,
            true => SLOTTED_OPERATION_A::SLOTTED_OP_EN,
        }
    }
    #[doc = "The radio transceiver operates in unslotted mode. An acknowledgment frame is automatically sent if requested."]
    #[inline(always)]
    pub fn is_slotted_op_dis(&self) -> bool {
        *self == SLOTTED_OPERATION_A::SLOTTED_OP_DIS
    }
    #[doc = "The transmission of an acknowledgment frame has to be controlled by the microcontroller."]
    #[inline(always)]
    pub fn is_slotted_op_en(&self) -> bool {
        *self == SLOTTED_OPERATION_A::SLOTTED_OP_EN
    }
}
#[doc = "Field `SLOTTED_OPERATION` writer - Set Slotted Acknowledgment"]
pub type SLOTTED_OPERATION_W<'a, REG> = crate::BitWriter<'a, REG, SLOTTED_OPERATION_A>;
impl<'a, REG> SLOTTED_OPERATION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The radio transceiver operates in unslotted mode. An acknowledgment frame is automatically sent if requested."]
    #[inline(always)]
    pub fn slotted_op_dis(self) -> &'a mut crate::W<REG> {
        self.variant(SLOTTED_OPERATION_A::SLOTTED_OP_DIS)
    }
    #[doc = "The transmission of an acknowledgment frame has to be controlled by the microcontroller."]
    #[inline(always)]
    pub fn slotted_op_en(self) -> &'a mut crate::W<REG> {
        self.variant(SLOTTED_OPERATION_A::SLOTTED_OP_EN)
    }
}
#[doc = "Field `MAX_CSMA_RETRIES` reader - Maximum Number of CSMA-CA Procedure Repetition Attempts"]
pub type MAX_CSMA_RETRIES_R = crate::FieldReader<MAX_CSMA_RETRIES_A>;
#[doc = "Maximum Number of CSMA-CA Procedure Repetition Attempts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAX_CSMA_RETRIES_A {
    #[doc = "0: No repetition of CSMA-CA procedure"]
    NO_REPETITION_OF_CSMA_CA_PROCEDURE = 0,
    #[doc = "1: One repetition of CSMA-CA procedure"]
    ONE_REPETITION_OF_CSMA_CA_PROCEDURE = 1,
    #[doc = "5: Five repetitions (highest IEEE 802.15.4 compliant value)"]
    FIVE_REPETITIONS_HIGHEST_IEEE_802_15_4_COMPLIANT_VALUE = 5,
    #[doc = "7: Immediate frame re-transmission without performing CSMA-CA"]
    IMMEDIATE_FRAME_RE_TRANSMISSION_WITHOUT_PERFORMING_CSMA_CA = 7,
}
impl From<MAX_CSMA_RETRIES_A> for u8 {
    #[inline(always)]
    fn from(variant: MAX_CSMA_RETRIES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAX_CSMA_RETRIES_A {
    type Ux = u8;
}
impl MAX_CSMA_RETRIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAX_CSMA_RETRIES_A> {
        match self.bits {
            0 => Some(MAX_CSMA_RETRIES_A::NO_REPETITION_OF_CSMA_CA_PROCEDURE),
            1 => Some(MAX_CSMA_RETRIES_A::ONE_REPETITION_OF_CSMA_CA_PROCEDURE),
            5 => Some(MAX_CSMA_RETRIES_A::FIVE_REPETITIONS_HIGHEST_IEEE_802_15_4_COMPLIANT_VALUE),
            7 => {
                Some(MAX_CSMA_RETRIES_A::IMMEDIATE_FRAME_RE_TRANSMISSION_WITHOUT_PERFORMING_CSMA_CA)
            }
            _ => None,
        }
    }
    #[doc = "No repetition of CSMA-CA procedure"]
    #[inline(always)]
    pub fn is_no_repetition_of_csma_ca_procedure(&self) -> bool {
        *self == MAX_CSMA_RETRIES_A::NO_REPETITION_OF_CSMA_CA_PROCEDURE
    }
    #[doc = "One repetition of CSMA-CA procedure"]
    #[inline(always)]
    pub fn is_one_repetition_of_csma_ca_procedure(&self) -> bool {
        *self == MAX_CSMA_RETRIES_A::ONE_REPETITION_OF_CSMA_CA_PROCEDURE
    }
    #[doc = "Five repetitions (highest IEEE 802.15.4 compliant value)"]
    #[inline(always)]
    pub fn is_five_repetitions_highest_ieee_802_15_4_compliant_value(&self) -> bool {
        *self == MAX_CSMA_RETRIES_A::FIVE_REPETITIONS_HIGHEST_IEEE_802_15_4_COMPLIANT_VALUE
    }
    #[doc = "Immediate frame re-transmission without performing CSMA-CA"]
    #[inline(always)]
    pub fn is_immediate_frame_re_transmission_without_performing_csma_ca(&self) -> bool {
        *self == MAX_CSMA_RETRIES_A::IMMEDIATE_FRAME_RE_TRANSMISSION_WITHOUT_PERFORMING_CSMA_CA
    }
}
#[doc = "Field `MAX_CSMA_RETRIES` writer - Maximum Number of CSMA-CA Procedure Repetition Attempts"]
pub type MAX_CSMA_RETRIES_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MAX_CSMA_RETRIES_A>;
impl<'a, REG> MAX_CSMA_RETRIES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No repetition of CSMA-CA procedure"]
    #[inline(always)]
    pub fn no_repetition_of_csma_ca_procedure(self) -> &'a mut crate::W<REG> {
        self.variant(MAX_CSMA_RETRIES_A::NO_REPETITION_OF_CSMA_CA_PROCEDURE)
    }
    #[doc = "One repetition of CSMA-CA procedure"]
    #[inline(always)]
    pub fn one_repetition_of_csma_ca_procedure(self) -> &'a mut crate::W<REG> {
        self.variant(MAX_CSMA_RETRIES_A::ONE_REPETITION_OF_CSMA_CA_PROCEDURE)
    }
    #[doc = "Five repetitions (highest IEEE 802.15.4 compliant value)"]
    #[inline(always)]
    pub fn five_repetitions_highest_ieee_802_15_4_compliant_value(self) -> &'a mut crate::W<REG> {
        self.variant(MAX_CSMA_RETRIES_A::FIVE_REPETITIONS_HIGHEST_IEEE_802_15_4_COMPLIANT_VALUE)
    }
    #[doc = "Immediate frame re-transmission without performing CSMA-CA"]
    #[inline(always)]
    pub fn immediate_frame_re_transmission_without_performing_csma_ca(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(MAX_CSMA_RETRIES_A::IMMEDIATE_FRAME_RE_TRANSMISSION_WITHOUT_PERFORMING_CSMA_CA)
    }
}
#[doc = "Field `MAX_FRAME_RETRIES` reader - Maximum Number of Frame Re-transmission Attempts"]
pub type MAX_FRAME_RETRIES_R = crate::FieldReader<MAX_FRAME_RETRIES_A>;
#[doc = "Maximum Number of Frame Re-transmission Attempts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAX_FRAME_RETRIES_A {
    #[doc = "0: Retransmission of frame is not attempted."]
    RETRANSMISSION_OF_FRAME_IS_NOT_ATTEMPTED = 0,
    #[doc = "1: Retransmission of frame is attempted once."]
    RETRANSMISSION_OF_FRAME_IS_ATTEMPTED_ONCE = 1,
    #[doc = "15: Retransmission of frame is attempted 15 times."]
    RETRANSMISSION_OF_FRAME_IS_ATTEMPTED_15_TIMES = 15,
}
impl From<MAX_FRAME_RETRIES_A> for u8 {
    #[inline(always)]
    fn from(variant: MAX_FRAME_RETRIES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAX_FRAME_RETRIES_A {
    type Ux = u8;
}
impl MAX_FRAME_RETRIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAX_FRAME_RETRIES_A> {
        match self.bits {
            0 => Some(MAX_FRAME_RETRIES_A::RETRANSMISSION_OF_FRAME_IS_NOT_ATTEMPTED),
            1 => Some(MAX_FRAME_RETRIES_A::RETRANSMISSION_OF_FRAME_IS_ATTEMPTED_ONCE),
            15 => Some(MAX_FRAME_RETRIES_A::RETRANSMISSION_OF_FRAME_IS_ATTEMPTED_15_TIMES),
            _ => None,
        }
    }
    #[doc = "Retransmission of frame is not attempted."]
    #[inline(always)]
    pub fn is_retransmission_of_frame_is_not_attempted(&self) -> bool {
        *self == MAX_FRAME_RETRIES_A::RETRANSMISSION_OF_FRAME_IS_NOT_ATTEMPTED
    }
    #[doc = "Retransmission of frame is attempted once."]
    #[inline(always)]
    pub fn is_retransmission_of_frame_is_attempted_once(&self) -> bool {
        *self == MAX_FRAME_RETRIES_A::RETRANSMISSION_OF_FRAME_IS_ATTEMPTED_ONCE
    }
    #[doc = "Retransmission of frame is attempted 15 times."]
    #[inline(always)]
    pub fn is_retransmission_of_frame_is_attempted_15_times(&self) -> bool {
        *self == MAX_FRAME_RETRIES_A::RETRANSMISSION_OF_FRAME_IS_ATTEMPTED_15_TIMES
    }
}
#[doc = "Field `MAX_FRAME_RETRIES` writer - Maximum Number of Frame Re-transmission Attempts"]
pub type MAX_FRAME_RETRIES_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MAX_FRAME_RETRIES_A>;
impl<'a, REG> MAX_FRAME_RETRIES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Retransmission of frame is not attempted."]
    #[inline(always)]
    pub fn retransmission_of_frame_is_not_attempted(self) -> &'a mut crate::W<REG> {
        self.variant(MAX_FRAME_RETRIES_A::RETRANSMISSION_OF_FRAME_IS_NOT_ATTEMPTED)
    }
    #[doc = "Retransmission of frame is attempted once."]
    #[inline(always)]
    pub fn retransmission_of_frame_is_attempted_once(self) -> &'a mut crate::W<REG> {
        self.variant(MAX_FRAME_RETRIES_A::RETRANSMISSION_OF_FRAME_IS_ATTEMPTED_ONCE)
    }
    #[doc = "Retransmission of frame is attempted 15 times."]
    #[inline(always)]
    pub fn retransmission_of_frame_is_attempted_15_times(self) -> &'a mut crate::W<REG> {
        self.variant(MAX_FRAME_RETRIES_A::RETRANSMISSION_OF_FRAME_IS_ATTEMPTED_15_TIMES)
    }
}
impl R {
    #[doc = "Bit 0 - Set Slotted Acknowledgment"]
    #[inline(always)]
    pub fn slotted_operation(&self) -> SLOTTED_OPERATION_R {
        SLOTTED_OPERATION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Maximum Number of CSMA-CA Procedure Repetition Attempts"]
    #[inline(always)]
    pub fn max_csma_retries(&self) -> MAX_CSMA_RETRIES_R {
        MAX_CSMA_RETRIES_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:7 - Maximum Number of Frame Re-transmission Attempts"]
    #[inline(always)]
    pub fn max_frame_retries(&self) -> MAX_FRAME_RETRIES_R {
        MAX_FRAME_RETRIES_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - Set Slotted Acknowledgment"]
    #[inline(always)]
    #[must_use]
    pub fn slotted_operation(&mut self) -> SLOTTED_OPERATION_W<XAH_CTRL_0_SPEC> {
        SLOTTED_OPERATION_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Maximum Number of CSMA-CA Procedure Repetition Attempts"]
    #[inline(always)]
    #[must_use]
    pub fn max_csma_retries(&mut self) -> MAX_CSMA_RETRIES_W<XAH_CTRL_0_SPEC> {
        MAX_CSMA_RETRIES_W::new(self, 1)
    }
    #[doc = "Bits 4:7 - Maximum Number of Frame Re-transmission Attempts"]
    #[inline(always)]
    #[must_use]
    pub fn max_frame_retries(&mut self) -> MAX_FRAME_RETRIES_W<XAH_CTRL_0_SPEC> {
        MAX_FRAME_RETRIES_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transceiver Extended Operating Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xah_ctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xah_ctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XAH_CTRL_0_SPEC;
impl crate::RegisterSpec for XAH_CTRL_0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xah_ctrl_0::R`](R) reader structure"]
impl crate::Readable for XAH_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xah_ctrl_0::W`](W) writer structure"]
impl crate::Writable for XAH_CTRL_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XAH_CTRL_0 to value 0"]
impl crate::Resettable for XAH_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
