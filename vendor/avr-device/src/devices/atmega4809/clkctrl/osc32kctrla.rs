#[doc = "Register `OSC32KCTRLA` reader"]
pub type R = crate::R<OSC32KCTRLA_SPEC>;
#[doc = "Register `OSC32KCTRLA` writer"]
pub type W = crate::W<OSC32KCTRLA_SPEC>;
#[doc = "Field `RUNSTDBY` reader - Run standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run standby"]
pub type RUNSTDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Run standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Run standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<OSC32KCTRLA_SPEC> {
        RUNSTDBY_W::new(self, 1)
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
#[doc = "OSC32K Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc32kctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc32kctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSC32KCTRLA_SPEC;
impl crate::RegisterSpec for OSC32KCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osc32kctrla::R`](R) reader structure"]
impl crate::Readable for OSC32KCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osc32kctrla::W`](W) writer structure"]
impl crate::Writable for OSC32KCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC32KCTRLA to value 0"]
impl crate::Resettable for OSC32KCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
