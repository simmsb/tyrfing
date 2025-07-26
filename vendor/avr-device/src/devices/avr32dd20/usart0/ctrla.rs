#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `RS485` reader - RS485 Mode internal transmitter"]
pub type RS485_R = crate::BitReader<RS485_A>;
#[doc = "RS485 Mode internal transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RS485_A {
    #[doc = "0: RS485 Mode disabled"]
    DISABLE = 0,
    #[doc = "1: RS485 Mode enabled"]
    ENABLE = 1,
}
impl From<RS485_A> for bool {
    #[inline(always)]
    fn from(variant: RS485_A) -> Self {
        variant as u8 != 0
    }
}
impl RS485_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RS485_A {
        match self.bits {
            false => RS485_A::DISABLE,
            true => RS485_A::ENABLE,
        }
    }
    #[doc = "RS485 Mode disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RS485_A::DISABLE
    }
    #[doc = "RS485 Mode enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RS485_A::ENABLE
    }
}
#[doc = "Field `RS485` writer - RS485 Mode internal transmitter"]
pub type RS485_W<'a, REG> = crate::BitWriter<'a, REG, RS485_A>;
impl<'a, REG> RS485_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RS485 Mode disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RS485_A::DISABLE)
    }
    #[doc = "RS485 Mode enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RS485_A::ENABLE)
    }
}
#[doc = "Field `ABEIE` reader - Auto-baud Error Interrupt Enable"]
pub type ABEIE_R = crate::BitReader;
#[doc = "Field `ABEIE` writer - Auto-baud Error Interrupt Enable"]
pub type ABEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBME` reader - Loop-back Mode Enable"]
pub type LBME_R = crate::BitReader;
#[doc = "Field `LBME` writer - Loop-back Mode Enable"]
pub type LBME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSIE` reader - Receiver Start Frame Interrupt Enable"]
pub type RXSIE_R = crate::BitReader;
#[doc = "Field `RXSIE` writer - Receiver Start Frame Interrupt Enable"]
pub type RXSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREIE` reader - Data Register Empty Interrupt Enable"]
pub type DREIE_R = crate::BitReader;
#[doc = "Field `DREIE` writer - Data Register Empty Interrupt Enable"]
pub type DREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIE` reader - Transmit Complete Interrupt Enable"]
pub type TXCIE_R = crate::BitReader;
#[doc = "Field `TXCIE` writer - Transmit Complete Interrupt Enable"]
pub type TXCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCIE` reader - Receive Complete Interrupt Enable"]
pub type RXCIE_R = crate::BitReader;
#[doc = "Field `RXCIE` writer - Receive Complete Interrupt Enable"]
pub type RXCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RS485 Mode internal transmitter"]
    #[inline(always)]
    pub fn rs485(&self) -> RS485_R {
        RS485_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Auto-baud Error Interrupt Enable"]
    #[inline(always)]
    pub fn abeie(&self) -> ABEIE_R {
        ABEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Loop-back Mode Enable"]
    #[inline(always)]
    pub fn lbme(&self) -> LBME_R {
        LBME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Start Frame Interrupt Enable"]
    #[inline(always)]
    pub fn rxsie(&self) -> RXSIE_R {
        RXSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dreie(&self) -> DREIE_R {
        DREIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie(&self) -> RXCIE_R {
        RXCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RS485 Mode internal transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rs485(&mut self) -> RS485_W<CTRLA_SPEC> {
        RS485_W::new(self, 0)
    }
    #[doc = "Bit 2 - Auto-baud Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn abeie(&mut self) -> ABEIE_W<CTRLA_SPEC> {
        ABEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Loop-back Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbme(&mut self) -> LBME_W<CTRLA_SPEC> {
        LBME_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Start Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsie(&mut self) -> RXSIE_W<CTRLA_SPEC> {
        RXSIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dreie(&mut self) -> DREIE_W<CTRLA_SPEC> {
        DREIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie(&mut self) -> TXCIE_W<CTRLA_SPEC> {
        TXCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie(&mut self) -> RXCIE_W<CTRLA_SPEC> {
        RXCIE_W::new(self, 7)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
