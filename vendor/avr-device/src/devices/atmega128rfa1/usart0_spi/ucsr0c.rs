#[doc = "Register `UCSR0C` reader"]
pub type R = crate::R<UCSR0C_SPEC>;
#[doc = "Register `UCSR0C` writer"]
pub type W = crate::W<UCSR0C_SPEC>;
#[doc = "Field `UCPOL0` reader - Clock Polarity"]
pub type UCPOL0_R = crate::BitReader;
#[doc = "Field `UCPOL0` writer - Clock Polarity"]
pub type UCPOL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPHA0` reader - Clock Phase"]
pub type UCPHA0_R = crate::BitReader;
#[doc = "Field `UCPHA0` writer - Clock Phase"]
pub type UCPHA0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDORD0` reader - Data Order"]
pub type UDORD0_R = crate::BitReader;
#[doc = "Field `UDORD0` writer - Data Order"]
pub type UDORD0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn ucpol0(&self) -> UCPOL0_R {
        UCPOL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ucpha0(&self) -> UCPHA0_R {
        UCPHA0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Order"]
    #[inline(always)]
    pub fn udord0(&self) -> UDORD0_R {
        UDORD0_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucpol0(&mut self) -> UCPOL0_W<UCSR0C_SPEC> {
        UCPOL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ucpha0(&mut self) -> UCPHA0_W<UCSR0C_SPEC> {
        UCPHA0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn udord0(&mut self) -> UDORD0_W<UCSR0C_SPEC> {
        UDORD0_W::new(self, 2)
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
#[doc = "USART0 MSPIM Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR0C_SPEC;
impl crate::RegisterSpec for UCSR0C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr0c::R`](R) reader structure"]
impl crate::Readable for UCSR0C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr0c::W`](W) writer structure"]
impl crate::Writable for UCSR0C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR0C to value 0"]
impl crate::Resettable for UCSR0C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
