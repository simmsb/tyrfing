#[doc = "Register `SCTRLB` reader"]
pub type R = crate::R<SCTRLB_SPEC>;
#[doc = "Register `SCTRLB` writer"]
pub type W = crate::W<SCTRLB_SPEC>;
#[doc = "Field `SCMD` reader - Command"]
pub type SCMD_R = crate::FieldReader<SCMD_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCMD_A {
    #[doc = "0: No Action"]
    NOACT = 0,
    #[doc = "2: Used To Complete a Transaction"]
    COMPTRANS = 2,
    #[doc = "3: Used in Response to Address/Data Interrupt"]
    RESPONSE = 3,
}
impl From<SCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: SCMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCMD_A {
    type Ux = u8;
}
impl SCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SCMD_A> {
        match self.bits {
            0 => Some(SCMD_A::NOACT),
            2 => Some(SCMD_A::COMPTRANS),
            3 => Some(SCMD_A::RESPONSE),
            _ => None,
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == SCMD_A::NOACT
    }
    #[doc = "Used To Complete a Transaction"]
    #[inline(always)]
    pub fn is_comptrans(&self) -> bool {
        *self == SCMD_A::COMPTRANS
    }
    #[doc = "Used in Response to Address/Data Interrupt"]
    #[inline(always)]
    pub fn is_response(&self) -> bool {
        *self == SCMD_A::RESPONSE
    }
}
#[doc = "Field `SCMD` writer - Command"]
pub type SCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SCMD_A>;
impl<'a, REG> SCMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(SCMD_A::NOACT)
    }
    #[doc = "Used To Complete a Transaction"]
    #[inline(always)]
    pub fn comptrans(self) -> &'a mut crate::W<REG> {
        self.variant(SCMD_A::COMPTRANS)
    }
    #[doc = "Used in Response to Address/Data Interrupt"]
    #[inline(always)]
    pub fn response(self) -> &'a mut crate::W<REG> {
        self.variant(SCMD_A::RESPONSE)
    }
}
#[doc = "Field `ACKACT` reader - Acknowledge Action"]
pub type ACKACT_R = crate::BitReader<ACKACT_A>;
#[doc = "Acknowledge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKACT_A {
    #[doc = "0: Send ACK"]
    ACK = 0,
    #[doc = "1: Send NACK"]
    NACK = 1,
}
impl From<ACKACT_A> for bool {
    #[inline(always)]
    fn from(variant: ACKACT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACKACT_A {
        match self.bits {
            false => ACKACT_A::ACK,
            true => ACKACT_A::NACK,
        }
    }
    #[doc = "Send ACK"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ACKACT_A::ACK
    }
    #[doc = "Send NACK"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == ACKACT_A::NACK
    }
}
#[doc = "Field `ACKACT` writer - Acknowledge Action"]
pub type ACKACT_W<'a, REG> = crate::BitWriter<'a, REG, ACKACT_A>;
impl<'a, REG> ACKACT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Send ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(ACKACT_A::ACK)
    }
    #[doc = "Send NACK"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(ACKACT_A::NACK)
    }
}
impl R {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    pub fn scmd(&self) -> SCMD_R {
        SCMD_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&self) -> ACKACT_R {
        ACKACT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn scmd(&mut self) -> SCMD_W<SCTRLB_SPEC> {
        SCMD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Acknowledge Action"]
    #[inline(always)]
    #[must_use]
    pub fn ackact(&mut self) -> ACKACT_W<SCTRLB_SPEC> {
        ACKACT_W::new(self, 2)
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
#[doc = "Slave Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCTRLB_SPEC;
impl crate::RegisterSpec for SCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sctrlb::R`](R) reader structure"]
impl crate::Readable for SCTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sctrlb::W`](W) writer structure"]
impl crate::Writable for SCTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTRLB to value 0"]
impl crate::Resettable for SCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
