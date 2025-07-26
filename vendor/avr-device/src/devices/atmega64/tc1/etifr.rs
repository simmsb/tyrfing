#[doc = "Register `ETIFR` reader"]
pub type R = crate::R<ETIFR_SPEC>;
#[doc = "Register `ETIFR` writer"]
pub type W = crate::W<ETIFR_SPEC>;
#[doc = "Field `OCF1C` reader - Timer/Counter 1, Output Compare C Match Flag"]
pub type OCF1C_R = crate::BitReader;
#[doc = "Field `OCF1C` writer - Timer/Counter 1, Output Compare C Match Flag"]
pub type OCF1C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter 1, Output Compare C Match Flag"]
    #[inline(always)]
    pub fn ocf1c(&self) -> OCF1C_R {
        OCF1C_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter 1, Output Compare C Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1c(&mut self) -> OCF1C_W<ETIFR_SPEC> {
        OCF1C_W::new(self, 0)
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
#[doc = "Extended Timer/Counter Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETIFR_SPEC;
impl crate::RegisterSpec for ETIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`etifr::R`](R) reader structure"]
impl crate::Readable for ETIFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etifr::W`](W) writer structure"]
impl crate::Writable for ETIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETIFR to value 0"]
impl crate::Resettable for ETIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
