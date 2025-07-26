#[doc = "Register `MCTRLB` reader"]
pub type R = crate::R<MCTRLB_SPEC>;
#[doc = "Register `MCTRLB` writer"]
pub type W = crate::W<MCTRLB_SPEC>;
#[doc = "Field `MCMD` reader - Command"]
pub type MCMD_R = crate::FieldReader<MCMD_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCMD_A {
    #[doc = "0: No Action"]
    NOACT = 0,
    #[doc = "1: Issue Repeated Start Condition"]
    REPSTART = 1,
    #[doc = "2: Receive or Transmit Data, depending on DIR"]
    RECVTRANS = 2,
    #[doc = "3: Issue Stop Condition"]
    STOP = 3,
}
impl From<MCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: MCMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCMD_A {
    type Ux = u8;
}
impl MCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCMD_A {
        match self.bits {
            0 => MCMD_A::NOACT,
            1 => MCMD_A::REPSTART,
            2 => MCMD_A::RECVTRANS,
            3 => MCMD_A::STOP,
            _ => unreachable!(),
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == MCMD_A::NOACT
    }
    #[doc = "Issue Repeated Start Condition"]
    #[inline(always)]
    pub fn is_repstart(&self) -> bool {
        *self == MCMD_A::REPSTART
    }
    #[doc = "Receive or Transmit Data, depending on DIR"]
    #[inline(always)]
    pub fn is_recvtrans(&self) -> bool {
        *self == MCMD_A::RECVTRANS
    }
    #[doc = "Issue Stop Condition"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MCMD_A::STOP
    }
}
#[doc = "Field `MCMD` writer - Command"]
pub type MCMD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MCMD_A>;
impl<'a, REG> MCMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(MCMD_A::NOACT)
    }
    #[doc = "Issue Repeated Start Condition"]
    #[inline(always)]
    pub fn repstart(self) -> &'a mut crate::W<REG> {
        self.variant(MCMD_A::REPSTART)
    }
    #[doc = "Receive or Transmit Data, depending on DIR"]
    #[inline(always)]
    pub fn recvtrans(self) -> &'a mut crate::W<REG> {
        self.variant(MCMD_A::RECVTRANS)
    }
    #[doc = "Issue Stop Condition"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(MCMD_A::STOP)
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
#[doc = "Field `FLUSH` reader - Flush"]
pub type FLUSH_R = crate::BitReader;
#[doc = "Field `FLUSH` writer - Flush"]
pub type FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    pub fn mcmd(&self) -> MCMD_R {
        MCMD_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&self) -> ACKACT_R {
        ACKACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flush"]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn mcmd(&mut self) -> MCMD_W<MCTRLB_SPEC> {
        MCMD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Acknowledge Action"]
    #[inline(always)]
    #[must_use]
    pub fn ackact(&mut self) -> ACKACT_W<MCTRLB_SPEC> {
        ACKACT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Flush"]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<MCTRLB_SPEC> {
        FLUSH_W::new(self, 3)
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
#[doc = "Master Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCTRLB_SPEC;
impl crate::RegisterSpec for MCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mctrlb::R`](R) reader structure"]
impl crate::Readable for MCTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mctrlb::W`](W) writer structure"]
impl crate::Writable for MCTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTRLB to value 0"]
impl crate::Resettable for MCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
