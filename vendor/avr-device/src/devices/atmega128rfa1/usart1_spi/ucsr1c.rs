#[doc = "Register `UCSR1C` reader"]
pub type R = crate::R<UCSR1C_SPEC>;
#[doc = "Register `UCSR1C` writer"]
pub type W = crate::W<UCSR1C_SPEC>;
#[doc = "Field `UCPOL1` reader - Clock Polarity"]
pub type UCPOL1_R = crate::BitReader;
#[doc = "Field `UCPOL1` writer - Clock Polarity"]
pub type UCPOL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPHA1` reader - Clock Phase"]
pub type UCPHA1_R = crate::BitReader;
#[doc = "Field `UCPHA1` writer - Clock Phase"]
pub type UCPHA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDORD1` reader - Data Order"]
pub type UDORD1_R = crate::BitReader;
#[doc = "Field `UDORD1` writer - Data Order"]
pub type UDORD1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn ucpol1(&self) -> UCPOL1_R {
        UCPOL1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ucpha1(&self) -> UCPHA1_R {
        UCPHA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Order"]
    #[inline(always)]
    pub fn udord1(&self) -> UDORD1_R {
        UDORD1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucpol1(&mut self) -> UCPOL1_W<UCSR1C_SPEC> {
        UCPOL1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ucpha1(&mut self) -> UCPHA1_W<UCSR1C_SPEC> {
        UCPHA1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn udord1(&mut self) -> UDORD1_W<UCSR1C_SPEC> {
        UDORD1_W::new(self, 2)
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
#[doc = "USART1 MSPIM Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR1C_SPEC;
impl crate::RegisterSpec for UCSR1C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr1c::R`](R) reader structure"]
impl crate::Readable for UCSR1C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr1c::W`](W) writer structure"]
impl crate::Writable for UCSR1C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR1C to value 0"]
impl crate::Resettable for UCSR1C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
