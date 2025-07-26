#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `ENRDY` reader - Enable ready"]
pub type ENRDY_R = crate::BitReader;
#[doc = "Field `CMDRDY` reader - Command ready"]
pub type CMDRDY_R = crate::BitReader;
#[doc = "Field `PWMACTA` reader - PWM activity on A"]
pub type PWMACTA_R = crate::BitReader;
#[doc = "Field `PWMACTA` writer - PWM activity on A"]
pub type PWMACTA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMACTB` reader - PWM activity on B"]
pub type PWMACTB_R = crate::BitReader;
#[doc = "Field `PWMACTB` writer - PWM activity on B"]
pub type PWMACTB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ready"]
    #[inline(always)]
    pub fn enrdy(&self) -> ENRDY_R {
        ENRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command ready"]
    #[inline(always)]
    pub fn cmdrdy(&self) -> CMDRDY_R {
        CMDRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM activity on A"]
    #[inline(always)]
    pub fn pwmacta(&self) -> PWMACTA_R {
        PWMACTA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM activity on B"]
    #[inline(always)]
    pub fn pwmactb(&self) -> PWMACTB_R {
        PWMACTB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - PWM activity on A"]
    #[inline(always)]
    #[must_use]
    pub fn pwmacta(&mut self) -> PWMACTA_W<STATUS_SPEC> {
        PWMACTA_W::new(self, 6)
    }
    #[doc = "Bit 7 - PWM activity on B"]
    #[inline(always)]
    #[must_use]
    pub fn pwmactb(&mut self) -> PWMACTB_W<STATUS_SPEC> {
        PWMACTB_W::new(self, 7)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
